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
    error::{ErrorKind, InvalidVolumeId},
    fmt::{HyphenatedId32, SimpleId32},
    std::str::FromStr,
};

#[cfg(feature = "std")]
use crate::std::string::String;

impl FromStr for VolumeId32 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s)
    }
}

impl TryFrom<&'_ str> for VolumeId32 {
    type Error = Error;

    fn try_from(s: &'_ str) -> Result<Self, Self::Error> {
        Self::try_parse(s)
    }
}

#[cfg(feature = "std")]
impl TryFrom<String> for VolumeId32 {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_parse(s.as_ref())
    }
}

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
        match (s.len(), s) {
            (8, s) => match parse_simpleid32(s) {
                Ok(bytes) => Ok(VolumeId32::from_bytes(bytes)),
                Err(_) => Err(Error(ErrorKind::ParseOther)),
            },
            (9, s) => match parse_hyphenated(s) {
                Ok(bytes) => Ok(VolumeId32::from_bytes(bytes)),
                Err(_) => Err(Error(ErrorKind::ParseOther)),
            },
            _ => Err(Error(ErrorKind::ParseOther)),
        }
    }
}

#[inline]
pub(crate) const fn parse_simpleid32(s: &'_ [u8]) -> Result<[u8; 4], InvalidVolumeId<'_>> {
    if s.len() != SimpleId32::LENGTH {
        return Err(InvalidVolumeId(s));
    }

    let mut buf = [0u8; 4];

    let mut i = 0;

    while i < 4 {
        // Convert a two-char hex value (like `A8`)
        // into a byte (like `10101000`)
        let h1 = HEX_TABLE[s[i * 2] as usize];
        let h2 = HEX_TABLE[s[i * 2 + 1] as usize];

        // We use `0xff` as a sentinel value to indicate
        // an invalid hex character sequence (like the letter `G`)
        if h1 | h2 == 0xff {
            return Err(InvalidVolumeId(s));
        }

        // The upper nibble needs to be shifted into position
        // to produce the final byte value
        buf[i] = SHL4_TABLE[h1 as usize] | h2;
        i += 1;
    }

    return Ok(buf);
}

#[inline]
pub(crate) const fn parse_hyphenated(s: &'_ [u8]) -> Result<[u8; 4], InvalidVolumeId<'_>> {
    if s.len() != HyphenatedId32::LENGTH {
        return Err(InvalidVolumeId(s));
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
        _ => return Err(InvalidVolumeId(s)),
    }

    let positions: [u8; 2] = [0, 5];

    let mut buf: [u8; 4] = [0; 4];
    let mut j = 0;

    while j < 2 {
        let i = positions[j];

        // The decoding here is the same as the simple case
        // We're just dealing with two values instead of one
        let h1 = HEX_TABLE[s[i as usize] as usize];
        let h2 = HEX_TABLE[s[(i + 1) as usize] as usize];
        let h3 = HEX_TABLE[s[(i + 2) as usize] as usize];
        let h4 = HEX_TABLE[s[(i + 3) as usize] as usize];

        buf[j * 2] = SHL4_TABLE[h1 as usize] | h2;
        buf[j * 2 + 1] = SHL4_TABLE[h3 as usize] | h4;
        j += 1;
    }

    Ok(buf)
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
            VolumeId32::try_parse(""),
            Err(Error(ErrorKind::ParseSimpleLength { len: 0 }))
        );

        assert_eq!(
            VolumeId32::try_parse("!"),
            Err(Error(ErrorKind::ParseChar {
                character: '!',
                index: 1,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E45"),
            Err(Error(ErrorKind::ParseGroupLength {
                group: 4,
                len: 13,
                index: 25,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4"),
            Err(Error(ErrorKind::ParseGroupLength {
                group: 3,
                len: 3,
                index: 20,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4"),
            Err(Error(ErrorKind::ParseChar {
                character: 'G',
                index: 21,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB2F4faaFB6BFF329BF39FA1E4"),
            Err(Error(ErrorKind::ParseGroupCount { count: 2 }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB2-4faaFB6BFF329BF39FA1E4"),
            Err(Error(ErrorKind::ParseGroupCount { count: 3 }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4"),
            Err(Error(ErrorKind::ParseGroupCount { count: 4 }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB2-4faa"),
            Err(Error(ErrorKind::ParseGroupCount { count: 3 }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB2-4faaXB6BFF329BF39FA1E4"),
            Err(Error(ErrorKind::ParseChar {
                character: 'X',
                index: 19,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("{F9168C5E-CEB2-4faa9B6BFF329BF39FA1E41"),
            Err(Error(ErrorKind::ParseChar {
                character: '{',
                index: 1,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("{F9168C5E-CEB2-4faa9B6BFF329BF39FA1E41}"),
            Err(Error(ErrorKind::ParseGroupCount { count: 3 }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB-24fa-eB6BFF32-BF39FA1E4"),
            Err(Error(ErrorKind::ParseGroupLength {
                group: 1,
                len: 3,
                index: 10,
            }))
        );

        // // (group, found, expecting)
        // //
        assert_eq!(
            VolumeId32::try_parse("01020304-1112-2122-3132-41424344"),
            Err(Error(ErrorKind::ParseGroupLength {
                group: 4,
                len: 8,
                index: 25,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("67e5504410b1426f9247bb680e5fe0c"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 31 }))
        );

        assert_eq!(
            VolumeId32::try_parse("67e5504410b1426f9247bb680e5fe0c88"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 33 }))
        );

        assert_eq!(
            VolumeId32::try_parse("67e5504410b1426f9247bb680e5fe0cg8"),
            Err(Error(ErrorKind::ParseChar {
                character: 'g',
                index: 32,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("67e5504410b1426%9247bb680e5fe0c8"),
            Err(Error(ErrorKind::ParseChar {
                character: '%',
                index: 16,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("231231212212423424324323477343246663"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 36 }))
        );

        assert_eq!(
            VolumeId32::try_parse("{00000000000000000000000000000000}"),
            Err(Error(ErrorKind::ParseGroupCount { count: 1 }))
        );

        assert_eq!(
            VolumeId32::try_parse("67e5504410b1426f9247bb680e5fe0c"),
            Err(Error(ErrorKind::ParseSimpleLength { len: 31 }))
        );

        assert_eq!(
            VolumeId32::try_parse("67e550X410b1426f9247bb680e5fe0cd"),
            Err(Error(ErrorKind::ParseChar {
                character: 'X',
                index: 7,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("67e550-4105b1426f9247bb680e5fe0c"),
            Err(Error(ErrorKind::ParseGroupCount { count: 2 }))
        );

        assert_eq!(
            VolumeId32::try_parse("F9168C5E-CEB2-4faa-B6BF1-02BF39FA1E4"),
            Err(Error(ErrorKind::ParseGroupLength {
                group: 3,
                len: 5,
                index: 20,
            }))
        );

        assert_eq!(
            VolumeId32::try_parse("\u{bcf3c}"),
            Err(Error(ErrorKind::ParseChar {
                character: '\u{bcf3c}',
                index: 1
            }))
        );
    }
}
