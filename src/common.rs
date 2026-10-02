#[inline]
pub const fn nibble_to_hex(nibble: u8, alpha_offset: u8) -> u8 {
    nibble + b'0' + if nibble > 9 { alpha_offset } else { 0 }
}
