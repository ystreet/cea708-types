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

use muldiv::MulDiv;

mod packet;
mod parser;
pub mod tables;
mod writer;

/// A CEA-608 compatibility byte pair
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cea608 {
    Field1(u8, u8),
    Field2(u8, u8),
}

/// A framerate.  Framerates larger than 60fps are not well supported.
#[derive(Debug, Copy, Clone)]
pub struct Framerate {
    numer: u32,
    denom: u32,
}

impl Framerate {
    /// Create a new [`Framerate`]
    pub const fn new(numer: u32, denom: u32) -> Self {
        Self { numer, denom }
    }

    /// The numerator of this [`Framerate`] fraction
    pub fn numer(&self) -> u32 {
        self.numer
    }

    /// The denominator of this [`Framerate`] fraction
    pub fn denom(&self) -> u32 {
        self.denom
    }

    fn cea608_pairs_per_frame(&self) -> usize {
        // CEA-608 has a max bitrate of 960 bits/s for a single field
        // TODO: handle alternating counts for 24fps
        60.mul_div_round(self.denom, self.numer).unwrap() as usize
    }

    fn max_cc_count(&self) -> usize {
        // CEA-708 has a max bitrate of 9_600 bits/s
        600.mul_div_round(self.denom, self.numer).unwrap() as usize
    }
}

pub use packet::{DTVCCPacket, Service};
pub use parser::{CCDataParser, ParserError};
pub use writer::{CCDataWriter, WriterError};

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::*;

    #[test]
    fn framerate_cea608_pairs_per_frame() {
        test_init_log();
        assert_eq!(Framerate::new(60, 1).cea608_pairs_per_frame(), 1);
        assert_eq!(Framerate::new(30, 1).cea608_pairs_per_frame(), 2);
    }

    #[test]
    fn framerate_max_cc_count() {
        test_init_log();
        assert_eq!(Framerate::new(60, 1).max_cc_count(), 10);
        assert_eq!(Framerate::new(30, 1).max_cc_count(), 20);
    }

    #[test]
    fn framerate_new() {
        test_init_log();
        let fps = Framerate::new(30, 8);
        assert_eq!(fps.numer(), 30);
        assert_eq!(fps.denom(), 8);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::OnceLock;

    static TRACING: OnceLock<()> = OnceLock::new();

    pub fn test_init_log() {
        TRACING.get_or_init(|| {
            env_logger::init();
        });
    }
}
