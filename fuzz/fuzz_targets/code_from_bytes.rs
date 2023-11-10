#![no_main]
use libfuzzer_sys::fuzz_target;

use cea708_types::tables::Code;

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
    if let Ok(code) = Code::from_data(data) {
        for c in code.iter() {
            info!("parsed {c:?}");
            let mut written = vec![];
            let _ = c.write(&mut written);
        }
    }
});
