#![no_main]
use libfuzzer_sys::fuzz_target;

use cea708_types::{CCDataParser, CCDataWriter, Framerate};

use once_cell::sync::Lazy;

#[macro_use]
extern crate log;

pub fn debug_init() {
    static TRACING: Lazy<()> = Lazy::new(|| {
        env_logger::init();
    });

    Lazy::force(&TRACING);
}

fuzz_target!(|data: &[u8]| {
    debug_init();
    let mut parser = CCDataParser::new();
    parser.handle_cea608();
    if let Ok(_) = parser.push(data) {
        let mut writer = CCDataWriter::default();
        while let Some(packet) = parser.pop_packet() {
            info!("parsed {packet:?}");
            writer.push_packet(packet);
        }
        if let Some(cea608) = parser.cea608() {
            info!("parsed cea608 {cea608:?}");
            for pair in cea608 {
                writer.push_cea608(*pair);
            }
        }
        let mut written = vec![];
        let framerate = Framerate::new(30, 1);
        let _ = writer.write(framerate, &mut written);
    }
});
