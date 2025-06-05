// Copyright (C) 2025 Matthew Waters <matthew@centricular.com>
//
// Licensed under the MIT license <LICENSE-MIT> or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use log::{debug, trace};

use crate::{tables, ParserError, WriterError};

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
        if service.is_empty() {
            return Err(WriterError::EmptyService);
        }
        self.services.push(service);
        Ok(())
    }

    pub(crate) fn parse_hdr_byte(byte: u8) -> (u8, usize) {
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
    /// Will return [ParserError::LengthMismatch] if the data is shorter than the length advertised in
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
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        if data.is_empty() {
            return Err(ParserError::LengthMismatch {
                expected: 1,
                actual: 0,
            });
        }
        let (seq_no, len) = Self::parse_hdr_byte(data[0]);
        trace!(
            "dtvcc seq:{seq_no} len {len} data {data_len}",
            data_len = data.len()
        );
        if (len + 1) < data.len() {
            return Err(ParserError::LengthMismatch {
                expected: len + 1,
                actual: data.len(),
            });
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

    /// The [Service]s for this [DTVCCPacket]
    pub fn services(&self) -> &[Service] {
        &self.services
    }

    pub(crate) fn cc_count(&self) -> usize {
        (self.len() + 1) / 2
    }

    fn hdr_byte(&self) -> u8 {
        debug_assert!(self.len() <= 128);
        (self.seq_no & 0x3) << 6 | (self.cc_count() & 0x3F) as u8
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

    pub(crate) fn write_as_cc_data<W: std::io::Write>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        // TODO: fail if we would overrun max size
        // TODO: handle framerate?
        if self.services.is_empty() {
            return Ok(());
        }
        let mut written = vec![];
        for service in self.services.iter() {
            service.write(&mut written)?;
            trace!("wrote service {service:?}");
        }
        w.write_all(&[0xFF, self.hdr_byte(), written[0]])?;
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

    /// Whether this [Service] block contains no [Code](tables::Code)s.
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::Service;
    /// let service = Service::new(1);
    /// assert_eq!(service.len(), 0);
    /// assert!(service.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.codes.is_empty()
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
    /// * [ParserError::LengthMismatch] if the length of the data is less than the size advertised in the
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
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        if data.is_empty() {
            return Err(ParserError::LengthMismatch {
                expected: 1,
                actual: 0,
            });
        }
        let byte = data[0];
        let mut service_no = (byte & 0xE0) >> 5;
        let block_size = (byte & 0x1F) as usize;
        let mut idx = 1;
        if service_no == 7 && block_size != 0 {
            if data.len() == 1 {
                return Err(ParserError::LengthMismatch {
                    expected: 2,
                    actual: data.len(),
                });
            }
            let byte2 = data[1];
            service_no = byte2 & 0x3F;
            idx += 1;
        }
        trace!("service no: {service_no}, block_size: {block_size}");

        if data.len() < idx + block_size {
            return Err(ParserError::LengthMismatch {
                expected: idx + block_size,
                actual: data.len(),
            });
        }

        if service_no != 0 {
            Ok(Self {
                number: service_no,
                codes: tables::Code::from_data(&data[idx..idx + block_size])?,
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
        if self.number >= 7 {
            let mut buf = [0; 2];
            buf[0] = 0xE0 | len;
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

    #[test]
    fn service_numbers() {
        test_init_log();
        for i in 1..64 {
            let mut service = Service::new(i);
            let code = tables::Code::Asterisk;
            service.push_code(&code).unwrap();
            let mut output = vec![];
            service.write(&mut output).unwrap();
            log::info!("created service {i} with data {output:x?}");
            let parsed = Service::parse(&output).unwrap();
            assert_eq!(service.number(), parsed.number());
            assert_eq!(service.codes(), &[code]);
        }
    }
}
