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

use crate::{Error, VolumeId32, error::ErrorKind};

const HEX_TABLE: &[u8; 256] = &{
    let mut buf = [0; 256];
    let mut i: u8 = 0;

    loop {
        buf[i as usize] = match i {
            b'0'..=b'9' => i - b'0',
            b'a'..=b'f' => i - b'a' + 10,
            b'A'..=b'F' => i - b'A' + 10,
            _ => 0xff,
        };

        if i == 255 {
            break buf;
        }

        i += 1
    }
};

const SHL4_TABLE: &[u8; 256] = &{
    let mut buf = [0; 256];
    let mut i: u8 = 0;

    loop {
        buf[i as usize] = i.wrapping_shl(4);

        if i == 255 {
            break buf;
        }

        i += 1;
    }
};

impl VolumeId32 {
    /// Parses a [`VolumeId32`] from a string slice of hexadecimal digits.
    ///
    /// To parse a [`VolumeId32`] from a byte stream instead of a UTF8 string, see
    /// [`VolumeId32::try_parse_ascii`].
    ///
    /// # Examples
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::try_parse("49aa648a")
    ///     .expect("Failed Parsing String");
    ///
    /// assert_eq!(volumeid32.to_string(), "49aa648a");
    /// ```
    pub const fn try_parse(input: &str) -> Result<Self, Error> {
        return Self::try_parse_ascii(input.as_bytes());
    }

    /// Parses a [`VolumeId32`] from a string of hexadecimal digits.
    ///
    /// The input is expected to be a string of ASCII characters. This method
    /// can be more convenient than [`VolumeId32::try_parse`] if the [`VolumeId32`] is being
    /// parsed from a byte stream instead of from a UTF8 string.
    ///
    /// # Examples
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::try_parse_ascii(b"49aa648a")
    ///     .expect("Failed Parsing String");
    ///
    /// assert_eq!(volumeid32.to_string(), "49aa648a");
    /// ```
    pub const fn try_parse_ascii(s: &[u8]) -> Result<Self, Error> {
        if s.len() != 8 {
            return Err(Error(ErrorKind::ParseByteLength { len: s.len() }));
        }

        let mut buf = [0u8; 4];

        let mut i = 0;

        while i < 4 {
            let h1 = HEX_TABLE[s[i * 2] as usize];
            let h2 = HEX_TABLE[s[i * 2 + 1] as usize];

            if h1 | h2 == 0xff {
                return Err(Error(ErrorKind::ParseInvalidAscii));
            }

            buf[i] = SHL4_TABLE[h1 as usize] | h2;
            i += 1;
        }

        return Ok(VolumeId32::from_bytes(buf));
    }
}
