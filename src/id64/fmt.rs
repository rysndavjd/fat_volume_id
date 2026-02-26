// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
// Copyright 2025-2026 rysndavjd.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    common::{LOWER, UPPER},
    id64::VolumeId64,
    std::{borrow::Borrow, fmt, hash::Hash, mem::transmute},
};

#[cfg(feature = "alloc")]
use crate::alloc::string::{String, ToString};

impl fmt::Debug for VolumeId64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::LowerHex::fmt(&self, f);
    }
}

impl fmt::Display for VolumeId64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::LowerHex::fmt(&self, f);
    }
}

#[cfg(feature = "alloc")]
impl From<VolumeId64> for String {
    fn from(volumeid64: VolumeId64) -> Self {
        volumeid64.to_string()
    }
}

impl fmt::LowerHex for VolumeId64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        return Ok(());
    }
}

impl fmt::UpperHex for VolumeId64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        return Ok(());
    }
}

/// Format a [`VolumeId64`] as a simple string, like
/// `6ddcf6dafdc2fd38`.
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
pub struct SimpleId64(VolumeId64);

impl VolumeId64 {
    /// Get a [`SimpleId64`] formatter.
    #[inline]
    pub const fn simple(self) -> SimpleId64 {
        SimpleId64(self)
    }

    /// Get a borrowed [`SimpleId64`] formatter.
    #[inline]
    pub fn as_simple(&self) -> &SimpleId64 {
        unsafe { transmute(self) }
    }
}

#[inline]
const fn format_simpleid64(src: &[u8; 8], upper: bool) -> [u8; SimpleId64::LENGTH] {
    let lut = if upper { &UPPER } else { &LOWER };
    let mut dst = [0; SimpleId64::LENGTH];
    let mut i = 0;
    while i < (SimpleId64::LENGTH / 2) {
        let x = src[i];
        dst[i * 2] = lut[(x >> 4) as usize];
        dst[i * 2 + 1] = lut[(x & 0x0f) as usize];
        i += 1;
    }
    dst
}

impl SimpleId64 {
        /// The length of a simple [`VolumeId64`] string.
    ///
    /// [`VolumeId64`]: ../struct.VolumeId64.html
    pub const LENGTH: usize = 16;

    /// Creates a [`SimpleId64`] from a [`VolumeId64`].
    ///     
    /// [`VolumeId64`]: ../struct.VolumeId64.html
    /// [`SimpleId64`]: struct.SimpleId64.html
    pub const fn from_volumeid64(volumeid64: VolumeId64) -> Self {
        SimpleId64(volumeid64)
    }

    /// Writes the [`VolumeId64`] as a lower-case simple string to `buffer`,
    /// and returns the subslice of the buffer that contains the encoded VolumeId64.
    ///
    /// This is slightly more efficient than using the formatting
    /// infrastructure as it avoids virtual calls, and may avoid
    /// double buffering.
    ///
    /// [`VolumeId64`]: ../struct.VolumeId64.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`].
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode(self.0.as_bytes(), buffer, false)
    }

    /// Writes the [`VolumeId64`] as an upper-case simple string to `buffer`,
    /// and returns the subslice of the buffer that contains the encoded VolumeId64.
    ///
    /// [`VolumeId64`]: ../struct.VolumeId64.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`].
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode(self.0.as_bytes(), buffer, true)
    }

    #[inline]
    fn _encode<'b>(src: &[u8; 8], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
        assert!(
            buffer.len() >= Self::LENGTH,
            "Buffer too small to encode a SimpleId64"
        );

        let buf: &mut [u8; Self::LENGTH] = (&mut buffer[..Self::LENGTH]).try_into().unwrap();
        *buf = format_simpleid64(src, upper);

        // SAFETY: The encoded buffer is ASCII encoded
        unsafe { str::from_utf8_unchecked_mut(buf) }
    }

    /// Get a reference to the underlying [`VolumeId64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fat_volume_id::VolumeId64;
    ///
    /// let simple = VolumeId64::nil().simple();
    /// assert_eq!(*simple.as_volumeid64(), VolumeId64::nil());
    /// ```
    pub const fn as_volumeid64(&self) -> &VolumeId64 {
        &self.0
    }

    /// Consumes the [`SimpleId64`], returning the underlying [`VolumeId64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fat_volume_id::VolumeId64;
    ///
    /// let simple = VolumeId64::nil().simple();
    /// assert_eq!(simple.into_volumeid64(), VolumeId64::nil());
    /// ```
    pub const fn into_volumeid64(self) -> VolumeId64 {
        self.0
    }
}

impl fmt::Display for SimpleId64 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self, f)
    }
}

impl fmt::LowerHex for SimpleId64 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.encode_lower(&mut [0; Self::LENGTH]))
    }
}

impl fmt::UpperHex for SimpleId64 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.encode_upper(&mut [0; Self::LENGTH]))
    }
}

impl From<VolumeId64> for SimpleId64 {
    #[inline]
    fn from(f: VolumeId64) -> Self {
        SimpleId64(f)
    }
}

impl From<SimpleId64> for VolumeId64 {
    #[inline]
    fn from(f: SimpleId64) -> Self {
        f.into_volumeid64()
    }
}

impl AsRef<VolumeId64> for SimpleId64 {
    #[inline]
    fn as_ref(&self) -> &VolumeId64 {
        &self.0
    }
}

impl Borrow<VolumeId64> for SimpleId64 {
    #[inline]
    fn borrow(&self) -> &VolumeId64 {
        &self.0
    }
}

impl<'a> From<&'a VolumeId64> for SimpleId64 {
    #[inline]
    fn from(f: &'a VolumeId64) -> Self {
        f.simple()
    }
}

impl<'a> From<&'a SimpleId64> for VolumeId64 {
    #[inline]
    fn from(f: &'a SimpleId64) -> Self {
        f.0
    }
}
