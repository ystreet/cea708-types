// Copyright (C) 2023 Matthew Waters <matthew@centricular.com>
//
// Licensed under the MIT license <LICENSE-MIT> or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # cea708-types
//!
//! Provides the necessary infrastructure to read and write [DTVCCPacket]'s containing [Service]s
//! with various [tables::Code]s
//!
//! The reference for this implementation is the [ANSI/CTA-708-E R-2018](https://shop.cta.tech/products/digital-television-dtv-closed-captioning) specification.

#[macro_use]
extern crate tracing;

pub mod tables;

/// Various possible errors when parsing data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserError {
    TooShort,
    LengthMismatch,
    IncorrectData,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{self:?}"))
    }
}

/// An error enum returned when writing data fails
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriterError {
    /// Writing would overflow by how many bytes
    WouldOverflow(usize),
    /// It is not possible to write to this resource
    ReadOnly,
}

impl From<tables::CodeError> for ParserError {
    fn from(err: tables::CodeError) -> Self {
        match err {
            tables::CodeError::TooShort => ParserError::TooShort,
            tables::CodeError::TooLong => ParserError::LengthMismatch,
        }
    }
}

#[derive(Debug, Default)]
pub struct CCDataParser {
    pending_data: Vec<u8>,
    packets: Vec<DTVCCPacket>,
    have_initial_ccp_header: bool,
    ccp_bytes_needed: usize,
}

impl CCDataParser {
    /// Create a new [CCDataParser]
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a complete `cc_data` packet into the parser for processing.
    ///
    /// Will fail with [ParserError::LengthMismatch] if the length of the data does not match the
    /// number of cc triples specified in the `cc_data` header.
    ///
    /// Ignores any CEA-608 data provided at the start of the data.  Any CEA-608 data provided
    /// after valid CEA-708 data will return [ParserError::IncorrectData].
    #[tracing::instrument(name = "CCDataParser::parse", skip(self, data))]
    pub fn push(&mut self, data: &[u8]) -> Result<(), ParserError> {
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
            return Err(ParserError::LengthMismatch);
        }

        let mut ccp_data = {
            let mut in_dtvcc = false;
            let mut ccp_data = vec![];

            // re-add first byte to pending_data
            let pending_data = {
                let mut ret = vec![];
                for (i, d) in self.pending_data.chunks(2).enumerate() {
                    if i == 0 {
                        ret.push(0xFF);
                    } else {
                        ret.push(0xFE);
                    }
                    ret.extend(d);
                    if d.len() == 1 {
                        ret.push(0x00);
                    }
                }
                ret
            };

            // find the start of ccp in data
            let ccp_offset = 2 + {
                let mut ret = None;
                for (i, triple) in data[2..].chunks_exact(3).enumerate() {
                    trace!(
                        "byte:{} triple 0x{:02x} 0x{:02x} 0x{:02x}",
                        i * 3,
                        triple[0],
                        triple[1],
                        triple[2]
                    );
                    let cc_valid = (triple[0] & 0x04) == 0x04;
                    let cc_type = triple[0] & 0x3;
                    if !in_dtvcc && cc_type == 0b00 || cc_type == 0b01 {
                        // ignore 608-in-708 data
                        continue;
                    }
                    if (cc_type & 0b10) > 0 {
                        in_dtvcc = true;
                    }
                    if !cc_valid {
                        continue;
                    }

                    if in_dtvcc && (cc_type == 0b00 || cc_type == 0b01) {
                        // invalid packet construction;
                        error!("cea608 bytes after cea708 data at byte:{}", i * 3);
                        return Err(ParserError::IncorrectData);
                    }

                    if ret.is_none() {
                        ret = Some(i * 3);
                    }
                }

                if let Some(ret) = ret {
                    ret
                } else {
                    // no data to process
                    return Ok(());
                }
            };

            let mut data_iter = pending_data.iter().chain(data[ccp_offset..].iter());
            let mut i = 0;
            loop {
                let byte0 = data_iter.next();
                let byte1 = data_iter.next();
                let byte2 = data_iter.next();
                i += 3;
                if let (Some(byte0), Some(byte1), Some(byte2)) = (byte0, byte1, byte2) {
                    let cc_valid = (byte0 & 0x04) == 0x04;
                    let cc_type = byte0 & 0x3;
                    if !in_dtvcc && cc_type == 0b00 || cc_type == 0b01 {
                        // ignore 608-in-708 data
                        continue;
                    }
                    if (cc_type & 0b10) > 0 {
                        in_dtvcc = true;
                    }
                    if !cc_valid {
                        continue;
                    }

                    if (cc_type & 0b11) == 0b11 {
                        trace!("found ccp header at index {}", i - 3);
                        self.have_initial_ccp_header = true;
                        // a header byte truncates the size of any previous packet
                        match DTVCCPacket::parse(&ccp_data) {
                            Ok(packet) => self.packets.push(packet),
                            Err(ParserError::TooShort) => (),
                            Err(ParserError::LengthMismatch) => (),
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
                } else {
                    break;
                }
            }
            ccp_data
        };

        if self.ccp_bytes_needed == 0 {
            match DTVCCPacket::parse(&ccp_data) {
                Ok(packet) => self.packets.push(packet),
                Err(ParserError::TooShort) => (),
                Err(ParserError::LengthMismatch) => (),
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
        let ret = self.packets.pop();
        trace!("popped {ret:?}");
        ret
    }
}

/// A packet in the `cc_data` bitstream
#[derive(Debug)]
pub struct DTVCCPacket {
    seq_no: u8,
    services: Vec<Service>,
}

impl DTVCCPacket {
    /// Create a new [DTVCCPacket] with the specified sequence number.
    ///
    /// # Panics
    ///
    /// * If seq_no >= 4
    pub fn new(seq_no: u8) -> Self {
        if seq_no > 3 {
            panic!("DTVCCPacket sequence numbers must be between 0 and 3 inclusive, not {seq_no}");
        }
        Self {
            seq_no,
            services: vec![],
        }
    }

    /// The sequence number of the DTVCCPacket
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::*;
    /// let packet = DTVCCPacket::new(2);
    /// assert_eq!(2, packet.sequence_no());
    /// ```
    pub fn sequence_no(&self) -> u8 {
        self.seq_no
    }

    /// The amount of free space (in bytes) that can by placed inside this [DTVCCPacket]
    pub fn free_space(&self) -> usize {
        // 128 is the max size of a DTVCCPacket, minus 1 for the header
        128 - self.len()
    }

    /// The number of bytes this [DTVCCPacket] will use when written to a byte stream.
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let mut packet = DTVCCPacket::new(2);
    /// assert_eq!(0, packet.len());
    /// let mut service = Service::new(1);
    /// service.push_code(&Code::LatinCapitalA).unwrap();
    /// packet.push_service(service);
    /// assert_eq!(3, packet.len());
    /// ```
    pub fn len(&self) -> usize {
        let services_len = self.services.iter().map(|s| s.len()).sum::<usize>();
        if services_len > 0 {
            1 + services_len
        } else {
            0
        }
    }

    /// Push a completed service block into this [DTVCCPacket]
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let mut packet = DTVCCPacket::new(2);
    /// assert_eq!(0, packet.len());
    /// let mut service = Service::new(1);
    /// service.push_code(&Code::LatinCapitalA).unwrap();
    /// packet.push_service(service);
    /// assert_eq!(3, packet.len());
    /// ```
    pub fn push_service(&mut self, service: Service) -> Result<(), WriterError> {
        // TODO: fail if we would overrun max size
        if service.len() > self.free_space() {
            return Err(WriterError::WouldOverflow(
                service.len() - self.free_space(),
            ));
        }
        self.services.push(service);
        Ok(())
    }

    fn parse_hdr_byte(byte: u8) -> (u8, usize) {
        let seq_no = (byte & 0xC0) >> 6;
        let len = byte & 0x3F;
        let len = if len == 0 {
            127usize
        } else {
            ((len as usize) * 2) - 1
        };
        (seq_no, len)
    }

    /// Parse bytes into a [DTVCCPacket]
    ///
    /// Will return [ParserError::TooShort] if the data is shorter than the length advertised in
    /// the [DTVCCPacket] header.
    ///
    /// Will return errors from [Service::parse] if parsing the contained [Service]s fails.
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let data = [0x02, 0x21, 0x41, 0x00];
    /// let packet = DTVCCPacket::parse(&data).unwrap();
    /// assert_eq!(3, packet.len());
    /// assert_eq!(0, packet.sequence_no());
    /// ```
    #[tracing::instrument(name = "DTVCCPacket::parse", err)]
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        if data.is_empty() {
            return Err(ParserError::TooShort);
        }
        let (seq_no, len) = Self::parse_hdr_byte(data[0]);
        trace!(
            "dtvcc seq:{seq_no} len {len} data {data_len}",
            data_len = data.len()
        );
        if (len + 1) < data.len() {
            return Err(ParserError::TooShort);
        }

        let mut offset = 1;
        let mut services = vec![];
        while offset < data.len() {
            let service = Service::parse(&data[offset..])?;
            trace!("parsed service {service:?}, len:{}", service.len());
            if service.len() == 0 {
                offset += 1;
                continue;
            }
            offset += service.len();
            services.push(service);
        }
        Ok(Self { seq_no, services })
    }

    /// Returns a copy of the [Service]s for this [DTVCCPacket]
    pub fn services(&self) -> &[Service] {
        &self.services
    }

    fn cc_count(&self) -> usize {
        (self.len() + 1) / 2
    }

    fn hdr_byte(&self) -> u8 {
        let packet_size_code = if self.len() == 127 {
            0
        } else {
            (self.len() + 1) / 2
        };
        (self.seq_no & 0x3) << 6 | packet_size_code as u8
    }

    /// Write the [DTVCCPacket] to a byte stream
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let mut packet = DTVCCPacket::new(2);
    /// let mut service = Service::new(1);
    /// service.push_code(&Code::LatinCapitalA).unwrap();
    /// packet.push_service(service);
    /// let mut written = vec![];
    /// packet.write(&mut written);
    /// let expected = [0x82, 0x21, 0x41, 0x00];
    /// assert_eq!(written, expected);
    /// ```
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        // TODO: fail if we would overrun max size
        w.write_all(&[self.hdr_byte()])?;
        for service in self.services.iter() {
            service.write(w)?;
        }
        if self.len() % 2 == 1 {
            w.write_all(&[0x00])?;
        }
        Ok(())
    }

    /// Write the [DTVCCPacket] bytestram encapsulated in the relevant cc_data bytes
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let mut packet = DTVCCPacket::new(2);
    /// let mut service = Service::new(1);
    /// service.push_code(&Code::LatinCapitalA).unwrap();
    /// packet.push_service(service);
    /// let mut written = vec![];
    /// packet.write_cc_data(&mut written);
    /// let expected = [0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x82, 0x21, 0xFE, 0x41, 0x00];
    /// assert_eq!(written, expected);
    /// ```
    #[tracing::instrument(name = "DTVCCPacket::write_cc_data", skip(self, w))]
    pub fn write_cc_data<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        // TODO: fail if we would overrun max size
        // TODO: handle framerate?
        if self.services.is_empty() {
            return Ok(());
        }
        let cc_count = (self.cc_count() & 0x1F) as u8;
        trace!("cc_count:{cc_count}");
        let reserved = 0x80;
        let process_cc_flag = 0x40;
        let mut written = vec![];
        for service in self.services.iter() {
            service.write(&mut written)?;
            trace!("wrote service {service:?}");
        }
        w.write_all(&[
            reserved | process_cc_flag | cc_count,
            0xFF,
            0xFF,
            self.hdr_byte(),
            written[0],
        ])?;
        for pair in written[1..].chunks(2) {
            let cc_valid = 0x04;
            let cc_type = 0b10;
            let reserved = 0xF8;
            w.write_all(&[reserved | cc_valid | cc_type])?;
            w.write_all(pair)?;
            if pair.len() == 1 {
                w.write_all(&[0x00])?;
            }
        }
        Ok(())
    }
}

/// A [Service] in a [DTVCCPacket]
///
/// As specified in CEA-708, there can be a maximum of 63 services.  Service 1 is the primary
/// caption service and Service 2 is the secondary caption service.  All other services are
/// undefined.
#[derive(Debug, Clone)]
pub struct Service {
    number: u8,
    codes: Vec<tables::Code>,
}

impl Service {
    /// Create a new [Service]
    ///
    /// # Panics
    ///
    /// * if number >= 64
    pub fn new(service_no: u8) -> Self {
        if service_no >= 64 {
            panic!("Service numbers must be between 0 and 63 inclusive, not {service_no}");
        }
        Self {
            number: service_no,
            codes: vec![],
        }
    }

    /// Returns the number of this [Service]
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let mut service = Service::new(1);
    /// assert_eq!(service.number(), 1);
    /// ```
    pub fn number(&self) -> u8 {
        self.number
    }

    fn codes_len(&self) -> usize {
        self.codes.iter().map(|c| c.byte_len()).sum()
    }

    /// The amount of free space (in bytes) that can by placed inside this [Service] block
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let service = Service::new(1);
    /// assert_eq!(service.free_space(), 31);
    /// ```
    pub fn free_space(&self) -> usize {
        // 31 is the maximum size of a service block
        31 - self.codes_len()
    }

    /// The length in bytes of this [Service] block
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let mut service = Service::new(1);
    /// assert_eq!(service.len(), 0);
    /// service.push_code(&Code::LatinCapitalA).unwrap();
    /// assert_eq!(service.len(), 2);
    /// service.push_code(&Code::LatinCapitalB).unwrap();
    /// assert_eq!(service.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        if self.number == 0 {
            return 0;
        }
        if self.codes.is_empty() {
            return 0;
        }
        let hdr_size = if self.number >= 7 { 2 } else { 1 };
        hdr_size + self.codes_len()
    }

    /// Push a [tables::Code] to the end of this [Service]
    ///
    /// # Errors
    ///
    /// * [WriterError::ReadOnly] if [Service] is number 0 (called the NULL Service)
    /// * [WriterError::WouldOverflow] if adding the [tables::Code] would cause to [Service] to overflow
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let mut service = Service::new(1);
    /// service.push_code(&Code::LatinCapitalA).unwrap();
    /// ```
    #[tracing::instrument(
        skip(self),
        fields(
            service_no = self.number
        )
    )]
    pub fn push_code(&mut self, code: &tables::Code) -> Result<(), WriterError> {
        // TODO: errors?
        if self.number == 0 {
            return Err(WriterError::ReadOnly);
        }

        if code.byte_len() > self.free_space() {
            let overflow_bytes = code.byte_len() - self.free_space();
            debug!("pushing would overflow by {overflow_bytes} bytes");
            return Err(WriterError::WouldOverflow(overflow_bytes));
        }
        trace!("pushing {code:?}");
        self.codes.push(code.clone());
        Ok(())
    }

    /// Parse a [Service] from a set of bytes
    ///
    /// # Errors
    ///
    /// * [ParserError::TooShort] if the length of the data is less than the size advertised in the
    /// header
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let bytes = [0x21, 0x41];
    /// let service = Service::parse(&bytes).unwrap();
    /// assert_eq!(service.number(), 1);
    /// assert_eq!(service.codes()[0], Code::LatinCapitalA);
    /// ```
    #[tracing::instrument(name = "Service::parse", err)]
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        let mut iter_data = data;
        if data.is_empty() {
            return Err(ParserError::TooShort);
        }
        let byte = data[0];
        iter_data = &iter_data[1..];
        let mut service_no = (byte & 0xE0) >> 5;
        let block_size = (byte & 0x1F) as usize;
        trace!("block_size: {block_size}");
        if service_no == 7 && block_size != 0 {
            if data.len() == 1 {
                return Err(ParserError::TooShort);
            }
            let byte2 = data[1];
            service_no = byte2 & 0x3F;
            iter_data = &iter_data[1..];
        }

        if service_no != 0 {
            Ok(Self {
                number: service_no,
                codes: tables::Code::from_data(&iter_data[..block_size])?,
            })
        } else {
            Ok(Self {
                number: 0,
                codes: vec![],
            })
        }
    }

    /// The ordered list of [tables::Code]s present in this [Service] block
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let mut service = Service::new(1);
    /// service.push_code(&Code::LatinCapitalA).unwrap();
    /// let codes = service.codes();
    /// assert_eq!(codes, [Code::LatinCapitalA]);
    /// ```
    pub fn codes(&self) -> &[tables::Code] {
        &self.codes
    }

    /// Write the [Service] block to a byte stream
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::{*, tables::*};
    /// let mut service = Service::new(1);
    /// service.push_code(&Code::LatinCapitalA).unwrap();
    /// let mut written = vec![];
    /// service.write(&mut written);
    /// let expected = [0x21, 0x41];
    /// assert_eq!(written, expected);
    /// ```
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        // TODO: fail if we would overrun max size
        let len = (self.codes_len() & 0x3F) as u8;
        if self.number > 7 {
            let mut buf = [0; 2];
            buf[0] = 0xC0 | len;
            buf[1] = self.number;
            w.write_all(&buf)?;
        } else {
            let byte = (self.number & 0x7) << 5 | len;
            w.write_all(&[byte])?;
        }
        for code in self.codes.iter() {
            code.write(w)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::*;

    #[test]
    fn simple_parse_dtvcc() {
        test_init_log();
        let data = [0x02, 0x01 << 5 | 0x01, 0x2A];
        let dtvcc = DTVCCPacket::parse(&data).unwrap();
        let services = dtvcc.services();
        assert_eq!(services.len(), 1);
        for service in services.iter() {
            assert_eq!(service.number, 1);
            let codes = service.codes();
            for code in codes.iter() {
                trace!("parsed {code:?}");
            }
        }
    }

    #[test]
    fn simple_write_dtvcc() {
        test_init_log();
        let mut service = Service::new(1);
        let code = tables::Code::Asterisk;
        service.push_code(&code).unwrap();
        let mut dtvcc = DTVCCPacket::new(0);
        dtvcc.push_service(service).unwrap();
        let mut written = vec![];
        dtvcc.write(&mut written).unwrap();
        let data = [0x02, 0x01 << 5 | 0x01, 0x2A, 0x00];
        assert_eq!(written, data);
    }

    #[derive(Debug)]
    struct TestCCData<'a> {
        cc_data: &'a [&'a [u8]],
        packets: &'a [(u8, &'a [(u8, &'a [tables::Code])])],
    }

    static TEST_CC_DATA: [TestCCData; 6] = [
        // simple packet with a single service and single code
        TestCCData {
            cc_data: &[&[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x21, 0xFE, 0x41, 0x00]],
            packets: &[(0, &[(1, &[tables::Code::LatinCapitalA])])],
        },
        // simple packet with a single service and two codes
        TestCCData {
            cc_data: &[&[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x22, 0xFE, 0x41, 0x42]],
            packets: &[(
                0,
                &[(
                    1,
                    &[tables::Code::LatinCapitalA, tables::Code::LatinCapitalB],
                )],
            )],
        },
        // two packets each with a single service and single code
        TestCCData {
            cc_data: &[
                &[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x21, 0xFE, 0x41, 0x00],
                &[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x42, 0x21, 0xFE, 0x42, 0x00],
            ],
            packets: &[
                (0, &[(1, &[tables::Code::LatinCapitalA])]),
                (1, &[(1, &[tables::Code::LatinCapitalB])]),
            ],
        },
        // two packets with a single service and one code split across both packets
        TestCCData {
            cc_data: &[
                &[0x80 | 0x40 | 0x01, 0xFF, 0xFF, 0x02, 0x21],
                &[0x80 | 0x40 | 0x01, 0xFF, 0xFE, 0x41, 0x00],
            ],
            packets: &[(0, &[(1, &[tables::Code::LatinCapitalA])])],
        },
        // simple packet with a single null service
        TestCCData {
            cc_data: &[&[0x80 | 0x40 | 0x01, 0xFF, 0xFF, 0x01, 0x00]],
            packets: &[(0, &[])],
        },
        // two packets with a single service and one code split across both packets with 608
        // padding data
        TestCCData {
            cc_data: &[
                &[
                    0x80 | 0x40 | 0x03,
                    0xFF,
                    0xFC,
                    0x80,
                    0x80,
                    0xFD,
                    0x80,
                    0x80,
                    0xFF,
                    0x02,
                    0x21,
                ],
                &[
                    0x80 | 0x40 | 0x03,
                    0xFF,
                    0xFC,
                    0x80,
                    0x80,
                    0xFD,
                    0x80,
                    0x80,
                    0xFE,
                    0x41,
                    0x00,
                ],
            ],
            packets: &[(0, &[(1, &[tables::Code::LatinCapitalA])])],
        },
    ];

    #[test]
    fn cc_data_parse() {
        test_init_log();
        for (i, test_data) in TEST_CC_DATA.iter().enumerate() {
            info!("parsing {i}: {test_data:?}");
            let mut parser = CCDataParser::new();
            let mut expected_iter = test_data.packets.iter();
            for data in test_data.cc_data.iter() {
                parser.push(data).unwrap();
                while let Some(packet) = parser.pop_packet() {
                    let expected = expected_iter.next().unwrap();
                    assert_eq!(expected.0, packet.sequence_no());
                    let services = packet.services();
                    let mut expected_service_iter = expected.1.iter();
                    for parsed_service in services.iter() {
                        let expected_service = expected_service_iter.next().unwrap();
                        assert_eq!(parsed_service.number(), expected_service.0);
                        assert_eq!(expected_service.1, parsed_service.codes());
                    }
                    assert!(expected_service_iter.next().is_none());
                }
            }
            assert!(parser.pop_packet().is_none());
            assert!(expected_iter.next().is_none());
        }
    }

    static WRITE_CC_DATA: [TestCCData; 3] = [
        // simple packet with a single service and single code
        TestCCData {
            cc_data: &[&[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x21, 0xFE, 0x41, 0x00]],
            packets: &[(0, &[(1, &[tables::Code::LatinCapitalA])])],
        },
        // simple packet with a single service and two codes
        TestCCData {
            cc_data: &[&[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x22, 0xFE, 0x41, 0x42]],
            packets: &[(
                0,
                &[(
                    1,
                    &[tables::Code::LatinCapitalA, tables::Code::LatinCapitalB],
                )],
            )],
        },
        // packet with a full service service
        TestCCData {
            cc_data: &[&[
                0x80 | 0x40 | 0x11,
                0xFF,
                0xFF,
                0xC0 | 0x11,
                0x20 | 0x1F,
                0xFE,
                0x41,
                0x42,
                0xFE,
                0x43,
                0x44,
                0xFE,
                0x45,
                0x46,
                0xFE,
                0x47,
                0x48,
                0xFE,
                0x49,
                0x4A,
                0xFE,
                0x4B,
                0x4C,
                0xFE,
                0x4D,
                0x4E,
                0xFE,
                0x4F,
                0x50,
                0xFE,
                0x51,
                0x52,
                0xFE,
                0x53,
                0x54,
                0xFE,
                0x55,
                0x56,
                0xFE,
                0x57,
                0x58,
                0xFE,
                0x59,
                0x5A,
                0xFE,
                0x61,
                0x62,
                0xFE,
                0x63,
                0x64,
                0xFE,
                0x65,
                0x0,
            ]],
            packets: &[(
                3,
                &[(
                    1,
                    &[
                        tables::Code::LatinCapitalA,
                        tables::Code::LatinCapitalB,
                        tables::Code::LatinCapitalC,
                        tables::Code::LatinCapitalD,
                        tables::Code::LatinCapitalE,
                        tables::Code::LatinCapitalF,
                        tables::Code::LatinCapitalG,
                        tables::Code::LatinCapitalH,
                        tables::Code::LatinCapitalI,
                        tables::Code::LatinCapitalJ,
                        tables::Code::LatinCapitalK,
                        tables::Code::LatinCapitalL,
                        tables::Code::LatinCapitalM,
                        tables::Code::LatinCapitalN,
                        tables::Code::LatinCapitalO,
                        tables::Code::LatinCapitalP,
                        tables::Code::LatinCapitalQ,
                        tables::Code::LatinCapitalR,
                        tables::Code::LatinCapitalS,
                        tables::Code::LatinCapitalT,
                        tables::Code::LatinCapitalU,
                        tables::Code::LatinCapitalV,
                        tables::Code::LatinCapitalW,
                        tables::Code::LatinCapitalX,
                        tables::Code::LatinCapitalY,
                        tables::Code::LatinCapitalZ,
                        tables::Code::LatinLowerA,
                        tables::Code::LatinLowerB,
                        tables::Code::LatinLowerC,
                        tables::Code::LatinLowerD,
                        tables::Code::LatinLowerE,
                    ],
                )],
            )],
        },
    ];

    #[test]
    fn packet_write_cc_data() {
        test_init_log();
        for cc_data in WRITE_CC_DATA.iter() {
            info!("writing {cc_data:?}");
            for ((packet_i, packet), cc_data) in cc_data.packets.iter().zip(cc_data.cc_data) {
                let mut pack = DTVCCPacket::new(*packet_i);
                for (service_no, codes) in packet.iter() {
                    let mut service = Service::new(*service_no);
                    for code in codes.iter() {
                        service.push_code(code).unwrap();
                    }
                    pack.push_service(service).unwrap();
                }
                let mut written = vec![];
                pack.write_cc_data(&mut written).unwrap();
                assert_eq!(cc_data, &written);
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use once_cell::sync::Lazy;
    use tracing_subscriber::EnvFilter;

    static TRACING: Lazy<()> = Lazy::new(|| {
        if let Ok(filter) = EnvFilter::try_from_default_env() {
            tracing_subscriber::fmt().with_env_filter(filter).init();
        }
    });

    pub fn test_init_log() {
        Lazy::force(&TRACING);
    }
}
