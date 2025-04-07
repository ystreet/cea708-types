// Copyright (C) 2025 Matthew Waters <matthew@centricular.com>
//
// Licensed under the MIT license <LICENSE-MIT> or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::VecDeque;

use log::{trace, warn};

use crate::{tables, Cea608, DTVCCPacket};

/// Various possible errors when parsing data
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ParserError {
    /// Length of data does not match length advertised
    #[error("The length of the data ({actual}) does not match the advertised expected ({expected}) length")]
    LengthMismatch {
        /// The expected size
        expected: usize,
        /// The actual size
        actual: usize,
    },
    /// CEA-608 comaptibility bytes encountered after CEA-708
    #[error("CEA-608 compatibility bytes were found after CEA-708 bytes at position {byte_pos}")]
    Cea608AfterCea708 {
        /// Position of the offending bytes
        byte_pos: usize,
    },
}

impl From<tables::CodeError> for ParserError {
    fn from(err: tables::CodeError) -> Self {
        match err {
            tables::CodeError::LengthMismatch { expected, actual } => {
                ParserError::LengthMismatch { expected, actual }
            }
        }
    }
}

/// Parses a byte stream of `cc_data` bytes into indivdual [`DTVCCPacket`]s.
#[derive(Debug, Default)]
pub struct CCDataParser {
    pending_data: Vec<u8>,
    packets: VecDeque<DTVCCPacket>,
    cea608: Option<Vec<Cea608>>,
    have_initial_ccp_header: bool,
    ccp_bytes_needed: usize,
}

impl CCDataParser {
    /// Create a new [CCDataParser]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_cea608(&mut self) {
        self.cea608 = Some(vec![]);
    }

    /// Push a complete `cc_data` packet into the parser for processing.
    ///
    /// Will fail with [ParserError::LengthMismatch] if the length of the data does not match the
    /// number of cc triples specified in the `cc_data` header.
    ///
    /// Any CEA-608 data provided after valid CEA-708 data will return
    /// [ParserError::Cea608AfterCea708].
    pub fn push(&mut self, data: &[u8]) -> Result<(), ParserError> {
        trace!("parsing {data:?}");
        if let Some(ref mut cea608) = self.cea608 {
            cea608.clear();
        }

        if data.len() < 5 {
            // enough for 2 byte header plus 1 byte triple
            return Ok(());
        }
        let process_cc_data_flag = data[0] & 0x40 > 0;
        if !process_cc_data_flag {
            return Ok(());
        }

        let cc_count = data[0] & 0x1F;
        if cc_count == 0 {
            return Ok(());
        }
        trace!("cc_count: {cc_count}, len = {}", data.len());
        if (cc_count * 3 + 2) as usize != data.len() {
            return Err(ParserError::LengthMismatch {
                expected: (cc_count * 3 + 1) as usize,
                actual: data.len(),
            });
        }

        let mut ccp_data = vec![];
        let mut in_dtvcc = false;

        // re-add first byte to pending_data
        let mut pending_data = vec![];
        for (i, d) in self.pending_data.chunks(2).enumerate() {
            if i == 0 {
                pending_data.push(0xFF);
            } else {
                pending_data.push(0xFE);
            }
            pending_data.extend(d);
            if d.len() == 1 {
                pending_data.push(0x00);
            }
        }

        // find the start of ccp in data
        let ccp_offset;
        {
            let mut ret = None;
            for (i, triple) in data[2..].chunks_exact(3).enumerate() {
                let cc_valid = (triple[0] & 0x04) == 0x04;
                let cc_type = triple[0] & 0x3;
                trace!(
                    "input byte:{} triple 0x{:02x} 0x{:02x} 0x{:02x}. valid: {cc_valid}, type: {cc_type}",
                    i * 3,
                    triple[0],
                    triple[1],
                    triple[2]
                );
                if (cc_type & 0b10) > 0 {
                    in_dtvcc = true;
                }
                if !cc_valid {
                    continue;
                }
                if !in_dtvcc && (cc_type == 0b00 || cc_type == 0b01) {
                    trace!(
                        "have cea608 bytes type {cc_type} 0x{:02x} 0x{:02x}",
                        triple[1],
                        triple[2]
                    );
                    if let Some(ref mut cea608) = self.cea608 {
                        let pair = match cc_type {
                            0b00 => Cea608::Field1(triple[1], triple[2]),
                            0b01 => Cea608::Field2(triple[1], triple[2]),
                            _ => unreachable!(),
                        };
                        cea608.push(pair);
                    }
                    continue;
                }

                if in_dtvcc && (cc_type == 0b00 || cc_type == 0b01) {
                    // invalid packet construction;
                    warn!("cea608 bytes after cea708 data at byte:{}", i * 3);
                    return Err(ParserError::Cea608AfterCea708 { byte_pos: i * 3 });
                }

                if ret.is_none() {
                    ret = Some(i * 3);
                }
            }

            if let Some(ret) = ret {
                ccp_offset = 2 + ret
            } else {
                // no data to process
                return Ok(());
            }
        }
        trace!("ccp offset in input data is at index {ccp_offset}");

        let mut data_iter = pending_data.iter().chain(data[ccp_offset..].iter());
        let mut i = 0;
        in_dtvcc = false;
        loop {
            let byte0 = data_iter.next();
            let byte1 = data_iter.next();
            let byte2 = data_iter.next();
            let (Some(byte0), Some(byte1), Some(byte2)) = (byte0, byte1, byte2) else {
                break;
            };
            let cc_valid = (byte0 & 0x04) == 0x04;
            let cc_type = byte0 & 0x3;
            trace!(
                "pending byte:{i} triple 0x{byte0:02x} 0x{byte1:02x} 0x{byte2:02x}. valid: {cc_valid}, type: {cc_type}",
            );
            i += 3;
            if (cc_type & 0b10) > 0 {
                in_dtvcc = true;
            }
            if !cc_valid {
                continue;
            }
            if !in_dtvcc && (cc_type == 0b00 || cc_type == 0b01) {
                // 608-in-708 data should not be hit as we skip over it
                unreachable!();
            }

            if (cc_type & 0b11) == 0b11 {
                trace!("found ccp header at index {}", i - 3);
                self.have_initial_ccp_header = true;
                // a header byte truncates the size of any previous packet
                match DTVCCPacket::parse(&ccp_data) {
                    Ok(packet) => self.packets.push_front(packet),
                    Err(ParserError::LengthMismatch { .. }) => (),
                    Err(e) => {
                        eprintln!("{e:?}");
                        unreachable!()
                    }
                }
                in_dtvcc = false;
                ccp_data = vec![];
                let (_seq_no, packet_len) = DTVCCPacket::parse_hdr_byte(*byte1);
                trace!("waiting for {} dtvcc bytes", packet_len + 1);
                self.ccp_bytes_needed = packet_len + 1;
            }

            if self.have_initial_ccp_header {
                trace!("pushing 0x{:02x?}{:02x?}", byte1, byte2);
                if self.ccp_bytes_needed > 0 {
                    ccp_data.push(*byte1);
                    self.ccp_bytes_needed -= 1;
                }
                if self.ccp_bytes_needed > 0 {
                    ccp_data.push(*byte2);
                    self.ccp_bytes_needed -= 1;
                }
            }
        }

        if self.ccp_bytes_needed == 0 {
            match DTVCCPacket::parse(&ccp_data) {
                Ok(packet) => self.packets.push_front(packet),
                Err(ParserError::LengthMismatch { .. }) => (),
                _ => unreachable!(),
            }
            ccp_data = vec![];
        }

        self.pending_data = ccp_data;

        Ok(())
    }

    /// Clear any internal buffers
    pub fn flush(&mut self) {
        *self = Self::default();
    }

    /// Pop a valid [DTVCCPacket] or None if no packet could be parsed
    pub fn pop_packet(&mut self) -> Option<DTVCCPacket> {
        let ret = self.packets.pop_back();
        trace!("popped {ret:?}");
        ret
    }

    /// Any [`Cea608`] bytes in the last parsed `cc_data`
    pub fn cea608(&mut self) -> Option<&[Cea608]> {
        self.cea608.as_deref()
    }
}
