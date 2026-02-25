#![no_main]
use libfuzzer_sys::fuzz_target;

use fat_volume_id::VolumeId64;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(volumeid64) = str::from_utf8(data) {
        // Ensure the parser doesn't panic
        let _ = VolumeId64::try_parse(volumeid64);
    }
});
