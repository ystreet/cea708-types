// Copyright (C) 2023 Matthew Waters <matthew@centricular.com>
//
// Licensed under the MIT license <LICENSE-MIT> or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cea708_types::*;

use std::env;

use once_cell::sync::Lazy;

#[macro_use]
extern crate tracing;
use tracing_subscriber::EnvFilter;

pub fn debug_init() {
    static TRACING: Lazy<()> = Lazy::new(|| {
        if let Ok(filter) = EnvFilter::try_from_default_env() {
            tracing_subscriber::fmt().with_env_filter(filter).init();
        }
    });

    Lazy::force(&TRACING);
}

fn main() -> std::process::ExitCode {
    debug_init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("708-dump filename");
        return std::process::ExitCode::from(1);
    }

    let file = std::fs::File::open(args[1].clone()).unwrap();
    let mut buf_reader = std::io::BufReader::new(file);

    let mut parser = CCDataParser::new();

    let mut i = 0;
    'l: loop {
        use std::io::{Read, Seek};
        // XXX: this has a hardcoded packet size
        let mut tmp = [0; 62];
        tmp[0] = 0x40 | 0x14;
        let buf_pos = buf_reader.stream_position().unwrap();
        let mut size = 0;
        while size < 60 {
            let n_read = buf_reader.read(&mut tmp[2 + size..]).unwrap();
            if n_read == 0 {
                break 'l;
            }
            size += n_read;
        }
        debug!("{i} read {size} bytes at {buf_pos} from {}", args[1]);

        trace!("{i} parsing {:?}", &tmp[..size]);
        if let Err(e) = parser.push(&tmp[..size + 2]) {
            eprintln!("{i} error parsing {e:?}");
        }

        while let Some(packet) = parser.pop_packet() {
            println!("{i} start DTVCCPacket:{}", packet.sequence_no());
            for service in packet.services().iter() {
                println!("{i}  start Service:{}", service.number());
                for code in service.codes() {
                    println!("{i}   {code:?}");
                }
                println!("{i}  end Service:{}", service.number());
            }
            println!("{i} end DTVCCPacket:{}", packet.sequence_no());
        }
        i += 1;
    }

    std::process::ExitCode::SUCCESS
}
