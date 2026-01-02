// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
// Copyright 2025 rysndavjd.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    Error, VolumeId32,
    std::{
        fmt,
        hash::{Hash, Hasher},
        mem::transmute,
    },
};

#[cfg(feature = "std")]
use crate::std::string::{String, ToString};

impl fmt::Debug for VolumeId32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::LowerHex::fmt(&self, f);
    }
}

impl fmt::Display for VolumeId32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::LowerHex::fmt(&self, f);
    }
}

#[cfg(feature = "std")]
impl From<VolumeId32> for String {
    fn from(volumeid32: VolumeId32) -> Self {
        volumeid32.to_string()
    }
}

impl fmt::LowerHex for VolumeId32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        return Ok(());
    }
}

impl fmt::UpperHex for VolumeId32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        return Ok(());
    }
}

impl Hash for VolumeId32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0);
    }
}

/// Format a [`VolumeId32`] as a simple string, like
/// `6ddcf6da`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::IntoBytes,
        zerocopy::FromBytes,
        zerocopy::KnownLayout,
        zerocopy::Immutable,
        zerocopy::Unaligned
    )
)]
#[repr(transparent)]
pub struct SimpleId32(VolumeId32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::IntoBytes,
        zerocopy::FromBytes,
        zerocopy::KnownLayout,
        zerocopy::Immutable,
        zerocopy::Unaligned
    )
)]
#[repr(transparent)]
pub struct HyphenatedId32(VolumeId32);

impl VolumeId32 {
    #[inline]
    pub const fn simple(self) -> SimpleId32 {
        SimpleId32(self)
    }

    #[inline]
    pub fn as_simple(&self) -> &SimpleId32 {
        unsafe { transmute(self) }
    }

    #[inline]
    pub const fn hyphenated(self) -> HyphenatedId32 {
        HyphenatedId32(self)
    }

    #[inline]
    pub fn as_hyphenated(&self) -> &HyphenatedId32 {
        unsafe { transmute(self) }
    }
}

const UPPER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];
const LOWER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

#[inline]
const fn format_simpleid32(src: &[u8; 4], upper: bool) -> [u8; SimpleId32::LENGTH] {
    let lut = if upper { &UPPER } else { &LOWER };
    let mut dst = [0; SimpleId32::LENGTH];
    let mut i = 0;
    while i < 16 {
        let x = src[i];
        dst[i * 2] = lut[(x >> 4) as usize];
        dst[i * 2 + 1] = lut[(x & 0x0f) as usize];
        i += 1;
    }
    dst
}

#[inline]
const fn format_hyphenatedid32(src: &[u8; 4], upper: bool) -> [u8; HyphenatedId32::LENGTH] {
    let lut = if upper { &UPPER } else { &LOWER };
    let groups = [(0, 4), (5, 8)];
    let mut dst = [0; HyphenatedId32::LENGTH];

    let mut group_idx = 0;
    let mut i = 0;
    while group_idx < 2 {
        let (start, end) = groups[group_idx];
        let mut j = start;
        while j < end {
            let x = src[i];
            i += 1;

            dst[j] = lut[(x >> 4) as usize];
            dst[j + 1] = lut[(x & 0x0f) as usize];
            j += 2;
        }
        if group_idx < 1 {
            dst[end] = b'-';
        }
        group_idx += 1;
    }
    dst
}

impl SimpleId32 {
    pub const LENGTH: usize = 8;

    pub const fn from_volumeid32(uuid: VolumeId32) -> Self {
        SimpleId32(uuid)
    }

    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode_simpleid32(self.0.as_bytes(), buffer, false)
    }

    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode_simpleid32(self.0.as_bytes(), buffer, true)
    }

    #[inline]
    fn _encode_simpleid32<'b>(src: &[u8; 4], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
        assert!(
            buffer.len() >= Self::LENGTH,
            "Buffer too small to encode a SimpleId32"
        );

        let buf: &mut [u8; Self::LENGTH] = buffer[..Self::LENGTH].as_mut_array().unwrap();
        *buf = format_simpleid32(src, upper);

        // SAFETY: The encoded buffer is ASCII encoded
        unsafe { str::from_utf8_unchecked_mut(buf) }
    }

    pub const fn as_volumeid32(&self) -> &VolumeId32 {
        &self.0
    }

    pub const fn into_volumeid32(self) -> VolumeId32 {
        self.0
    }
}

impl HyphenatedId32 {
    pub const LENGTH: usize = 9;

    pub const fn from_volumeid32(uuid: VolumeId32) -> Self {
        HyphenatedId32(uuid)
    }

    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode_hyphenatedid32(self.0.as_bytes(), buffer, false)
    }

    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode_hyphenatedid32(self.0.as_bytes(), buffer, true)
    }

    #[inline]
    fn _encode_hyphenatedid32<'b>(src: &[u8; 4], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
        assert!(
            buffer.len() >= Self::LENGTH,
            "Buffer too small to encode a SimpleId32"
        );

        let buf: &mut [u8; Self::LENGTH] = buffer[..Self::LENGTH].as_mut_array().unwrap();
        *buf = format_hyphenatedid32(src, upper);

        // SAFETY: The encoded buffer is ASCII encoded
        unsafe { str::from_utf8_unchecked_mut(buf) }
    }

    pub const fn as_volumeid32(&self) -> &VolumeId32 {
        &self.0
    }

    pub const fn into_volumeid32(self) -> VolumeId32 {
        self.0
    }
}
