#![no_main]
use libfuzzer_sys::fuzz_target;

use cea708_types::tables::Code;

use std::sync::OnceLock;

static TRACING: OnceLock<()> = OnceLock::new();

use log::info;

pub fn debug_init() {
    TRACING.get_or_init(|| {
        env_logger::init();
    });
}

fuzz_target!(|data: &[u8]| {
    debug_init();
    if let Ok(code) = Code::from_data(data) {
        for c in code.iter() {
            info!("parsed {c:?}");
            let mut written = vec![];
            let _ = c.write(&mut written);
        }
    }
});
