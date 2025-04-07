// Copyright (C) 2025 Matthew Waters <matthew@centricular.com>
//
// Licensed under the MIT license <LICENSE-MIT> or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::VecDeque;
use std::time::Duration;

use muldiv::MulDiv;

use log::trace;

use crate::{Cea608, DTVCCPacket, Framerate};

/// An error enum returned when writing data fails
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum WriterError {
    /// Writing would overflow by how many bytes
    #[error("Writing the data would overflow by {0} bytes")]
    WouldOverflow(usize),
    /// It is not possible to write to this resource
    #[error("The resource is not writable")]
    ReadOnly,
}

/// A struct for writing cc_data packets
#[derive(Debug, Default)]
pub struct CCDataWriter {
    // settings
    output_cea608_padding: bool,
    output_padding: bool,
    // state
    packets: VecDeque<DTVCCPacket>,
    // part of a packet we could not fit into the previous packet
    pending_packet_data: Vec<u8>,
    cea608_1: VecDeque<(u8, u8)>,
    cea608_2: VecDeque<(u8, u8)>,
    last_cea608_was_field1: bool,
}

impl CCDataWriter {
    /// Whether to output padding CEA-608 bytes when not enough enough data has been provided
    pub fn set_output_cea608_padding(&mut self, output_cea608_padding: bool) {
        self.output_cea608_padding = output_cea608_padding;
    }

    /// Whether padding CEA-608 bytes will be used
    pub fn output_cea608_padding(&self) -> bool {
        self.output_cea608_padding
    }

    /// Whether to output padding data in the CCP bitstream when not enough data has been provided
    pub fn set_output_padding(&mut self, output_padding: bool) {
        self.output_padding = output_padding;
    }

    /// Whether padding data will be produced in the CCP
    pub fn output_padding(&self) -> bool {
        self.output_padding
    }

    /// Push a [`DTVCCPacket`] for writing
    pub fn push_packet(&mut self, packet: DTVCCPacket) {
        self.packets.push_front(packet)
    }

    /// Push a [`Cea608`] byte pair for writing
    pub fn push_cea608(&mut self, cea608: Cea608) {
        match cea608 {
            Cea608::Field1(byte0, byte1) => {
                if byte0 != 0x80 || byte1 != 0x80 {
                    self.cea608_1.push_front((byte0, byte1))
                }
            }
            Cea608::Field2(byte0, byte1) => {
                if byte0 != 0x80 || byte1 != 0x80 {
                    self.cea608_2.push_front((byte0, byte1))
                }
            }
        }
    }

    /// Clear all stored data
    pub fn flush(&mut self) {
        self.packets.clear();
        self.pending_packet_data.clear();
        self.cea608_1.clear();
        self.cea608_2.clear();
    }

    /// The amount of time that is currently stored for CEA-608 field 1 data
    pub fn buffered_cea608_field1_duration(&self) -> Duration {
        // CEA-608 has a max bitrate of 60000 * 2 / 1001 bytes/s
        Duration::from_micros(
            (self.cea608_1.len() as u64)
                .mul_div_ceil(1001 * 1_000_000, 60000)
                .unwrap(),
        )
    }

    /// The amount of time that is currently stored for CEA-608 field 2 data
    pub fn buffered_cea608_field2_duration(&self) -> Duration {
        // CEA-608 has a max bitrate of 60000 * 2 / 1001 bytes/s
        Duration::from_micros(
            (self.cea608_2.len() as u64)
                .mul_div_ceil(1001 * 1_000_000, 60000)
                .unwrap(),
        )
    }

    fn buffered_packet_bytes(&self) -> usize {
        self.pending_packet_data.len()
            + self
                .packets
                .iter()
                .map(|packet| packet.len())
                .sum::<usize>()
    }

    /// The amount of time that is currently stored for CCP data
    pub fn buffered_packet_duration(&self) -> Duration {
        // CEA-708 has a max bitrate of 9600000 / 1001 bits/s
        Duration::from_micros(
            ((self.buffered_packet_bytes() + 1) as u64 / 2)
                .mul_div_ceil(2 * 1001 * 1_000_000, 9_600_000 / 8)
                .unwrap(),
        )
    }

    /// Write the next cc_data packet taking the next relevant CEA-608 byte pairs and
    /// [`DTVCCPacket`]s.  The framerate provided determines how many bytes are written.
    pub fn write<W: std::io::Write>(
        &mut self,
        framerate: Framerate,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        let mut cea608_pair_rem = if self.output_cea608_padding {
            framerate.cea608_pairs_per_frame()
        } else {
            framerate
                .cea608_pairs_per_frame()
                .min(self.cea608_1.len().max(self.cea608_2.len() * 2))
        };

        let mut cc_count_rem = if self.output_padding {
            framerate.max_cc_count()
        } else {
            framerate.max_cc_count().min(
                cea608_pair_rem
                    + self.pending_packet_data.len() / 3
                    + self.packets.iter().map(|p| p.cc_count()).sum::<usize>(),
            )
        };
        trace!("writing with cc_count: {cc_count_rem} and {cea608_pair_rem} cea608 pairs");

        let reserved = 0x80;
        let process_cc_flag = 0x40;
        w.write_all(&[
            reserved | process_cc_flag | (cc_count_rem & 0x1f) as u8,
            0xFF,
        ])?;
        while cc_count_rem > 0 {
            if cea608_pair_rem > 0 {
                if !self.last_cea608_was_field1 {
                    trace!("attempting to write a cea608 byte pair from field 1");
                    if let Some((byte0, byte1)) = self.cea608_1.pop_back() {
                        w.write_all(&[0xFC, byte0, byte1])?;
                        cc_count_rem -= 1;
                    } else if !self.cea608_2.is_empty() {
                        // need to write valid field 0 if we are going to write field 1
                        w.write_all(&[0xFC, 0x80, 0x80])?;
                        cc_count_rem -= 1;
                    } else if self.output_cea608_padding {
                        w.write_all(&[0xF8, 0x80, 0x80])?;
                        cc_count_rem -= 1;
                    }
                    self.last_cea608_was_field1 = true;
                } else {
                    trace!("attempting to write a cea608 byte pair from field 2");
                    if let Some((byte0, byte1)) = self.cea608_2.pop_back() {
                        w.write_all(&[0xFD, byte0, byte1])?;
                        cc_count_rem -= 1;
                    } else if self.output_cea608_padding {
                        w.write_all(&[0xF9, 0x80, 0x80])?;
                        cc_count_rem -= 1;
                    }
                    self.last_cea608_was_field1 = false;
                }
                cea608_pair_rem -= 1;
            } else {
                let mut current_packet_data = &mut self.pending_packet_data;
                let mut packet_offset = 0;
                while packet_offset >= current_packet_data.len() {
                    if let Some(packet) = self.packets.pop_back() {
                        trace!("starting packet {packet:?}");
                        packet.write_as_cc_data(&mut current_packet_data)?;
                    } else {
                        trace!("no packet to write");
                        break;
                    }
                }

                trace!("cea708 pending data length {}", current_packet_data.len(),);

                while packet_offset < current_packet_data.len() && cc_count_rem > 0 {
                    assert!(current_packet_data.len() >= packet_offset + 3);
                    w.write_all(&current_packet_data[packet_offset..packet_offset + 3])?;
                    packet_offset += 3;
                    cc_count_rem -= 1;
                }

                self.pending_packet_data = current_packet_data[packet_offset..].to_vec();

                if self.packets.is_empty() && self.pending_packet_data.is_empty() {
                    // no more data to write
                    if self.output_padding {
                        trace!("writing {cc_count_rem} padding bytes");
                        while cc_count_rem > 0 {
                            w.write_all(&[0xFA, 0x00, 0x00])?;
                            cc_count_rem -= 1;
                        }
                    }
                    break;
                }
            }
        }
        Ok(())
    }
}
