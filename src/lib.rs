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

#![no_std]
#![allow(clippy::needless_return)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

mod error;
mod fmt;
mod parser;

pub use error::{Error, ErrorKind};

/// 32-bit Volume ID used in FAT12/16/32 and exFAT filesystems simliar to a UUID.
/// Used for Identification of different volumes.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
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
pub struct VolumeId32([u8; 4]);

impl VolumeId32 {
    /// A VolumeId32 with all zeros.
    // Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::nil();
    ///
    /// assert_eq!(
    ///     "00000000",
    ///     volumeid32.to_string(),
    /// );
    /// ```
    pub const fn nil() -> Self {
        return VolumeId32([0u8; 4]);
    }

    /// A VolumeId32 with all ones.
    // Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::max();
    ///
    /// assert_eq!(
    ///     "ffffffff",
    ///     volumeid32.to_string(),
    /// );
    /// ```
    pub const fn max() -> Self {
        return VolumeId32([0xffu8; 4]);
    }

    /// Creates a VolumeId32 using supplied bytes exactly.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// # use std::string::ToString;
    /// let bytes = [0xa1, 0xa2, 0xa3, 0xa4];
    ///
    /// let volumeid32 = VolumeId32::from_bytes(bytes);
    ///
    /// assert_eq!(volumeid32.to_string(), "a1a2a3a4");
    /// ```
    #[inline]
    pub const fn from_bytes(bytes: [u8; 4]) -> VolumeId32 {
        return VolumeId32(bytes);
    }

    /// Creates a VolumeId32 using supplied bytes in little endian.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// # use std::string::ToString;
    /// let bytes = [0xa1, 0xa2, 0xa3, 0xa4];
    ///
    /// let volumeid32 = VolumeId32::from_bytes_le(bytes);
    ///
    /// assert_eq!(volumeid32.to_string(), "a4a3a2a1");
    /// ```
    #[inline]
    pub const fn from_bytes_le(b: [u8; 4]) -> VolumeId32 {
        return VolumeId32([b[3], b[2], b[1], b[0]]);
    }

    /// Creates a VolumeId32 using supplied bytes in big endian.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// # use std::string::ToString;
    /// let bytes = [0xa1, 0xa2, 0xa3, 0xa4];
    ///
    /// let volumeid32 = VolumeId32::from_bytes_be(bytes);
    ///
    /// assert_eq!(volumeid32.to_string(), "a1a2a3a4");
    /// ```
    #[inline]
    pub const fn from_bytes_be(b: [u8; 4]) -> VolumeId32 {
        return VolumeId32([b[0], b[1], b[2], b[3]]);
    }

    /// Creates a VolumeId32 using the supplied bytes exactly.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 4.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    /// ];
    ///
    /// let volumeid32 = VolumeId32::from_slice(&bytes)
    ///     .expect("Slice should be 4 bytes long");
    ///
    /// assert_eq!(
    ///     "a1a2a3a4",
    ///     volumeid32.to_string(),
    /// );
    /// ```
    pub fn from_slice(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 4 {
            return Err(Error(error::ErrorKind::ParseByteLength { len: b.len() }));
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(b);

        return Ok(VolumeId32::from_bytes(bytes));
    }

    /// Creates a VolumeId32 using the supplied bytes in little endian.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 4.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    /// ];
    ///
    /// let volumeid32 = VolumeId32::from_slice_le(&bytes)
    ///     .expect("Slice should be 4 bytes long");
    ///
    /// assert_eq!(
    ///     "a4a3a2a1",
    ///     volumeid32.to_string(),
    /// );
    /// ```
    pub fn from_slice_le(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 4 {
            return Err(Error(error::ErrorKind::ParseByteLength { len: b.len() }));
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(b);

        return Ok(VolumeId32::from_bytes_le(bytes));
    }

    /// Creates a VolumeId32 using the supplied bytes in big endian.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 4.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    /// ];
    ///
    /// let volumeid32 = VolumeId32::from_slice_be(&bytes)
    ///     .expect("Slice should be 4 bytes long");
    ///
    /// assert_eq!(
    ///     "a1a2a3a4",
    ///     volumeid32.to_string(),
    /// );
    /// ```
    pub fn from_slice_be(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 4 {
            return Err(Error(error::ErrorKind::ParseByteLength { len: b.len() }));
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(b);

        return Ok(VolumeId32::from_bytes_be(bytes));
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }

    /// Returns an array of bytes in little endian.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::from_bytes([0xa1, 0xa2, 0xa3, 0xa4]);
    ///
    /// assert_eq!(
    ///     volumeid32.as_bytes_le(),
    ///     [0xa4, 0xa3, 0xa2, 0xa1],
    /// );
    /// ```
    pub fn as_bytes_le(&self) -> [u8; 4] {
        return [self.0[3], self.0[2], self.0[1], self.0[0]];
    }

    /// Returns an array of bytes in big endian.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::from_bytes([0xa1, 0xa2, 0xa3, 0xa4]);
    ///
    /// assert_eq!(
    ///     volumeid32.as_bytes_be(),
    ///     [0xa1, 0xa2, 0xa3, 0xa4],
    /// );
    /// ```
    pub fn as_bytes_be(&self) -> [u8; 4] {
        return [self.0[0], self.0[1], self.0[2], self.0[3]];
    }

    #[inline]
    pub const fn into_bytes(self) -> [u8; 4] {
        self.0
    }
}

/// 64-bit Volume ID used in NTFS filesystems simliar to a UUID.
/// Used for Identification of different volumes.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
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
pub struct VolumeId64([u8; 8]);
