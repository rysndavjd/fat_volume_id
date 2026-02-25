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
    common::{HEX_TABLE, SHL4_TABLE},
    id64::{
        VolumeId64,
        error::{Error, ErrorKind, InvalidVolumeId64},
        fmt::SimpleId64,
    },
    std::str::FromStr,
};

impl FromStr for VolumeId64 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s).map_err(|_| Error(ErrorKind::ParseOther))
    }
}

impl TryFrom<&'_ str> for VolumeId64 {
    type Error = Error;

    fn try_from(s: &'_ str) -> Result<Self, Self::Error> {
        Self::try_parse(s).map_err(|_| Error(ErrorKind::ParseOther))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<crate::alloc::string::String> for VolumeId64 {
    type Error = Error;

    fn try_from(s: crate::alloc::string::String) -> Result<Self, Self::Error> {
        Self::try_parse(s.as_ref()).map_err(|_| Error(ErrorKind::ParseOther))
    }
}

impl VolumeId64 {
    /// Parses a [`VolumeId64`] from a string slice of hexadecimal digits.
    /// Automatically gets additional infomation of errors if any are returned
    /// using `InvalidVolumeId64::into_err`.
    ///
    /// To parse a [`VolumeId64`] from a byte stream instead of a UTF8 string, see
    /// [`try_parse_ascii`].
    ///
    /// # Examples
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let volumeid64 = VolumeId64::parse("49aa648a49aa648a")
    ///     .expect("Failed Parsing String");
    ///
    /// assert_eq!(volumeid64.to_string(), "49aa648a49aa648a");
    /// ```
    /// [`try_parse_ascii`]: #method.try_parse_ascii
    pub fn parse(input: &str) -> Result<Self, Error> {
        Self::try_parse_ascii(input.as_bytes()).map_err(InvalidVolumeId64::into_err)
    }

    /// Parses a [`VolumeId64`] from a string slice of hexadecimal digits.
    /// Without getting additional infomation on errors instead returning
    /// `InvalidVolumeId64`.
    ///
    /// To parse a [`VolumeId64`] from a byte stream instead of a UTF8 string, see
    /// [`try_parse_ascii`].
    ///
    /// # Examples
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let volumeid64 = VolumeId64::try_parse("49aa648a49aa648a")
    ///     .expect("Failed Parsing String");
    ///
    /// assert_eq!(volumeid64.to_string(), "49aa648a49aa648a");
    /// ```
    /// [`try_parse_ascii`]: #method.try_parse_ascii
    pub const fn try_parse<'a>(input: &'a str) -> Result<Self, InvalidVolumeId64<'a>> {
        Self::try_parse_ascii(input.as_bytes())
    }

    /// Parses a [`VolumeId64`] from a string of hexadecimal digits.
    ///
    /// The input is expected to be a string of ASCII characters. This method
    /// can be more convenient than [`try_parse`] if the [`VolumeId64`] is being
    /// parsed from a byte stream instead of from a UTF8 string.
    ///
    /// # Examples
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let volumeid64 = VolumeId64::try_parse_ascii(b"49aa648a49aa648a")
    ///     .expect("Failed Parsing String");
    ///
    /// assert_eq!(volumeid64.to_string(), "49aa648a49aa648a");
    /// ```
    /// [`try_parse`]: #method.try_parse
    pub const fn try_parse_ascii<'a>(s: &'a [u8]) -> Result<Self, InvalidVolumeId64<'a>> {
        match (s.len(), s) {
            (16, s) => match parse_simpleid64(s) {
                Ok(bytes) => Ok(VolumeId64::from_bytes(bytes)),
                Err(e) => Err(e),
            },
            _ => Err(InvalidVolumeId64(s)),
        }
    }
}

#[inline]
pub(crate) const fn parse_simpleid64(s: &'_ [u8]) -> Result<[u8; 8], InvalidVolumeId64<'_>> {
    if s.len() != SimpleId64::LENGTH {
        return Err(InvalidVolumeId64(s));
    }

    let mut buf = [0u8; 8];

    let mut i = 0;

    while i < 8 {
        // Convert a two-char hex value (like `A8`)
        // into a byte (like `10101000`)
        let h1 = HEX_TABLE[s[i * 2] as usize];
        let h2 = HEX_TABLE[s[i * 2 + 1] as usize];

        // We use `0xff` as a sentinel value to indicate
        // an invalid hex character sequence (like the letter `G`)
        if h1 | h2 == 0xff {
            return Err(InvalidVolumeId64(s));
        }

        // The upper nibble needs to be shifted into position
        // to produce the final byte value
        buf[i] = SHL4_TABLE[h1 as usize] | h2;
        i += 1;
    }

    return Ok(buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id64::error::ErrorKind;

    #[test]
    fn test_parse_volumeid64_valid() {
        assert!(VolumeId64::try_parse("0000000000000000").is_ok());

        assert!(VolumeId64::try_parse("6ddcf6da6ddcf6da").is_ok());
        assert!(VolumeId64::try_parse("6DDCF6DA6DDCF6DA").is_ok());
    }

    #[test]
    fn test_parse_volumeid64_invalid() {
        assert_eq!(
            VolumeId64::parse(""),
            Err(Error(ErrorKind::ParseSimpleLength { len: 0 }))
        );

        assert_eq!(
            VolumeId64::parse("!"),
            Err(Error(ErrorKind::ParseChar {
                character: '!',
                index: 1,
            }))
        );

        assert_eq!(
            VolumeId64::parse("QABC123456789ABC"),
            Err(Error(ErrorKind::ParseChar {
                character: 'Q',
                index: 1,
            }))
        );

        assert_eq!(
            VolumeId64::parse("F9168C5X12345678"),
            Err(Error(ErrorKind::ParseChar {
                character: 'X',
                index: 8,
            }))
        );

        assert_eq!(
            VolumeId64::parse("{F9168C512345678"),
            Err(Error(ErrorKind::ParseChar {
                character: '{',
                index: 1,
            }))
        );

        assert_eq!(
            VolumeId64::parse("67e5"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 4 }))
        );

        assert_eq!(
            VolumeId64::parse("123456789ABCDFE12"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 17 }))
        );

        assert_eq!(
            VolumeId64::parse("67e55abg12abcdef"),
            Err(Error(ErrorKind::ParseChar {
                character: 'g',
                index: 8,
            }))
        );

        assert_eq!(
            VolumeId64::parse("67e5%2fb12345678"),
            Err(Error(ErrorKind::ParseChar {
                character: '%',
                index: 5,
            }))
        );

        assert_eq!(
            VolumeId64::parse("231231212212423424324323477343246663"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 36 }))
        );

        assert_eq!(
            VolumeId64::parse("67e5504410b1426f9247bb680e5fe0c"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 31 }))
        );

        assert_eq!(
            VolumeId64::parse("Abcdef1267e550Xb"),
            Err(Error(ErrorKind::ParseChar {
                character: 'X',
                index: 15,
            }))
        );

        assert_eq!(
            VolumeId64::parse("\u{bcf3c}"),
            Err(Error(ErrorKind::ParseChar {
                character: '\u{bcf3c}',
                index: 1
            }))
        );
    }
}
