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
    id32::{
        VolumeId32,
        error::{Error, InvalidVolumeId32, RequestedVolumeId},
        fmt::{HyphenatedId32, SimpleId32},
    },
    std::str::FromStr,
};

#[inline]
const fn decode_hex8(hex: &[u8; 8]) -> Option<[u8; 4]> {
    let mut nibbles = [0u8; 8];
    let mut bad = 0u8;

    let mut i = 0;
    while i < 8 {
        let c = hex[i];

        // '0'..='9' map to 0..=9; 'a'..='f' and 'A'..='F' (via `| 0x20`) map to
        // 0..=5, offset by 10 to land in 10..=15.
        let digit = c.wrapping_sub(b'0');
        let alpha = (c | 0x20).wrapping_sub(b'a');

        let is_digit = digit < 10;
        let is_alpha = alpha < 6;

        nibbles[i] = if is_digit {
            digit
        } else {
            alpha.wrapping_add(10)
        };

        bad |= if is_digit | is_alpha { 0 } else { 1 };

        i += 1;
    }

    if bad != 0 {
        return None;
    }

    let mut buf = [0u8; 4];
    let mut j = 0;
    while j < 4 {
        buf[j] = (nibbles[j * 2] << 4) | nibbles[j * 2 + 1];
        j += 1;
    }

    Some(buf)
}

impl FromStr for VolumeId32 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s).map_err(|invaild| invaild.into_err())
    }
}

impl TryFrom<&'_ str> for VolumeId32 {
    type Error = Error;

    fn try_from(s: &'_ str) -> Result<Self, Self::Error> {
        Self::try_parse(s).map_err(|invaild| invaild.into_err())
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<crate::alloc::string::String> for VolumeId32 {
    type Error = Error;

    fn try_from(s: crate::alloc::string::String) -> Result<Self, Self::Error> {
        Self::try_parse(s.as_ref()).map_err(|invaild| invaild.into_err())
    }
}

impl VolumeId32 {
    /// Parses a [`VolumeId32`] from a string slice of hexadecimal digits.
    /// Automatically gets additional information of errors if any are returned
    /// using [`InvalidVolumeId32::into_err`].
    ///
    /// To parse a [`VolumeId32`] from a byte stream instead of a UTF8 string, see
    /// [`try_parse_ascii`].
    ///
    /// # Examples
    /// ```
    /// # use fat_volume_id::id32::VolumeId32;
    /// let volumeid32 = VolumeId32::parse("49aa648a")
    ///     .expect("Failed Parsing String");
    ///
    /// assert_eq!(volumeid32.to_string(), "49aa648a");
    /// ```
    /// [`try_parse_ascii`]: #method.try_parse_ascii
    pub fn parse(input: &str) -> Result<Self, Error> {
        Self::try_parse_ascii(input.as_bytes()).map_err(InvalidVolumeId32::into_err)
    }

    /// Parses a [`VolumeId32`] from a string slice of hexadecimal digits.
    /// Without getting additional information on errors instead returning
    /// [`InvalidVolumeId32`].
    ///
    /// To parse a [`VolumeId32`] from a byte stream instead of a UTF8 string, see
    /// [`try_parse_ascii`].
    ///
    /// # Examples
    /// ```
    /// # use fat_volume_id::id32::VolumeId32;
    /// let volumeid32 = VolumeId32::try_parse("49aa648a")
    ///     .expect("Failed Parsing String");
    ///
    /// assert_eq!(volumeid32.to_string(), "49aa648a");
    /// ```
    /// [`try_parse_ascii`]: #method.try_parse_ascii
    pub const fn try_parse<'a>(input: &'a str) -> Result<Self, InvalidVolumeId32<'a>> {
        Self::try_parse_ascii(input.as_bytes())
    }

    /// Parses a [`VolumeId32`] from a string of hexadecimal digits.
    ///
    /// The input is expected to be a string of ASCII characters. This method
    /// can be more convenient than [`try_parse`] if the [`VolumeId32`] is being
    /// parsed from a byte stream instead of from a UTF8 string.
    ///
    /// # Examples
    /// ```
    /// # use fat_volume_id::id32::VolumeId32;
    /// let volumeid32 = VolumeId32::try_parse_ascii(b"49aa648a")
    ///     .expect("Failed Parsing String");
    ///
    /// assert_eq!(volumeid32.to_string(), "49aa648a");
    /// ```
    /// [`try_parse`]: #method.try_parse
    pub const fn try_parse_ascii<'a>(s: &'a [u8]) -> Result<Self, InvalidVolumeId32<'a>> {
        match (s.len(), s) {
            (8, s) => match parse_simpleid32(s) {
                Ok(bytes) => Ok(VolumeId32::from_bytes(bytes)),
                Err(e) => Err(e),
            },
            (9, s) => match parse_hyphenatedid32(s) {
                Ok(bytes) => Ok(VolumeId32::from_bytes(bytes)),
                Err(e) => Err(e),
            },
            _ => Err(InvalidVolumeId32(s, RequestedVolumeId::Any)),
        }
    }
}

#[inline]
pub(crate) const fn parse_simpleid32(s: &'_ [u8]) -> Result<[u8; 4], InvalidVolumeId32<'_>> {
    let hex = if s.len() == SimpleId32::LENGTH {
        s.first_chunk::<{ SimpleId32::LENGTH }>().unwrap()
    } else {
        return Err(InvalidVolumeId32(s, RequestedVolumeId::Simple));
    };

    match decode_hex8(hex) {
        Some(buf) => Ok(buf),
        None => Err(InvalidVolumeId32(s, RequestedVolumeId::Simple)),
    }
}

#[inline]
pub(crate) const fn parse_hyphenatedid32(s: &'_ [u8]) -> Result<[u8; 4], InvalidVolumeId32<'_>> {
    if s.len() != HyphenatedId32::LENGTH {
        return Err(InvalidVolumeId32(s, RequestedVolumeId::Hyphenated));
    }

    // We look at two hex-encoded values (4 chars) at a time because
    // that's the size of the smallest group in a hyphenated VolumeId32.
    // The indexes we're interested in are:
    //
    // volumeid32 : 6ddc-f6da
    //              |   ||
    // hyphens    : |   4|
    // positions  : 0    5

    // First, ensure the hyphen appear in the right places
    match [s[4]] {
        [b'-'] => {}
        _ => return Err(InvalidVolumeId32(s, RequestedVolumeId::Hyphenated)),
    }

    // Gather the hex characters, skipping the hyphen, so they're
    // contiguous for the decoder.
    let mut hex = [0u8; 8];
    let mut i = 0;
    while i < 4 {
        hex[i] = s[i];
        i += 1;
    }
    while i < 8 {
        hex[i] = s[i + 1];
        i += 1;
    }

    match decode_hex8(&hex) {
        Some(buf) => Ok(buf),
        None => Err(InvalidVolumeId32(s, RequestedVolumeId::Hyphenated)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id32::error::ErrorKind;

    #[test]
    fn test_parse_volumeid32_valid() {
        let simple = VolumeId32::try_parse("6ddcf6da").unwrap();
        let hyphenated = VolumeId32::try_parse("6ddc-f6da").unwrap();

        assert_eq!(hyphenated, simple);

        assert!(VolumeId32::try_parse("00000000").is_ok());
        assert!(VolumeId32::try_parse("0000-0000").is_ok());

        assert!(VolumeId32::try_parse("6ddcf6da").is_ok());
        assert!(VolumeId32::try_parse("6DDCF6DA").is_ok());

        assert!(VolumeId32::try_parse("6ddc-f6da").is_ok());
        assert!(VolumeId32::try_parse("6DDC-F6DA").is_ok());
    }

    #[test]
    fn test_parse_volumeid32_invalid() {
        assert_eq!(
            VolumeId32::parse(""),
            Err(Error(ErrorKind::ParseSimpleLength { len: 0 }))
        );

        assert_eq!(
            VolumeId32::parse("!"),
            Err(Error(ErrorKind::ParseChar {
                character: '!',
                index: 0,
            }))
        );

        assert_eq!(
            VolumeId32::parse("F916-4fa"),
            Err(Error(ErrorKind::ParseChar {
                character: '-',
                index: 4
            }))
        );

        assert_eq!(
            VolumeId32::parse("QABC-1234"),
            Err(Error(ErrorKind::ParseChar {
                character: 'Q',
                index: 0,
            }))
        );

        assert_eq!(
            VolumeId32::parse("F9-16-8C5E"),
            Err(Error(ErrorKind::ParseGroupCount { count: 3 }))
        );

        assert_eq!(
            VolumeId32::parse("F9168C5X"),
            Err(Error(ErrorKind::ParseChar {
                character: 'X',
                index: 7,
            }))
        );

        assert_eq!(
            VolumeId32::parse("{F9168C5"),
            Err(Error(ErrorKind::ParseChar {
                character: '{',
                index: 0,
            }))
        );

        assert_eq!(
            VolumeId32::parse("67e5"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 4 }))
        );

        assert_eq!(
            VolumeId32::parse("123456ABC"),
            Err(Error(ErrorKind::ParseGroupCount { count: 1 }))
        );

        assert_eq!(
            VolumeId32::parse("67e55abg"),
            Err(Error(ErrorKind::ParseChar {
                character: 'g',
                index: 7,
            }))
        );

        assert_eq!(
            VolumeId32::parse("67e5%2fb"),
            Err(Error(ErrorKind::ParseChar {
                character: '%',
                index: 4,
            }))
        );

        assert_eq!(
            VolumeId32::parse("231231212212423424324323477343246663"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 36 }))
        );

        assert_eq!(
            VolumeId32::parse("67e5504410b1426f9247bb680e5fe0c"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 31 }))
        );

        assert_eq!(
            VolumeId32::parse("67e550Xb"),
            Err(Error(ErrorKind::ParseChar {
                character: 'X',
                index: 6,
            }))
        );

        assert_eq!(
            VolumeId32::parse("F916BA-CE"),
            Err(Error(ErrorKind::ParseGroupLength {
                group: 0,
                len: 6,
                index: 0,
            }))
        );

        assert_eq!(
            VolumeId32::parse("F9-16BACE"),
            Err(Error(ErrorKind::ParseGroupLength {
                group: 0,
                len: 2,
                index: 0,
            }))
        );

        assert_eq!(
            VolumeId32::parse("F916-BACEE"),
            Err(Error(ErrorKind::ParseGroupLength {
                group: 1,
                len: 5,
                index: 5,
            }))
        );

        assert_eq!(
            VolumeId32::parse("\u{bcf3c}"),
            Err(Error(ErrorKind::ParseChar {
                character: '\u{bcf3c}',
                index: 0
            }))
        );
    }
}
