use crate::std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error(pub(crate) ErrorKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ErrorKind {
    /// Invalid character in given string.
    ParseChar { character: char, index: usize },
    /// Invalid length of given string.
    ParseSimpleLength { len: usize },
    /// Invalid length of given byte array.
    ParseByteLength { len: usize },
    /// Invalid number of groups in hyphenated variant.
    ParseGroupCount { count: usize },
    /// Invalid length of a group within a hyphenated variant.
    ParseGroupLength {
        group: usize,
        len: usize,
        index: usize,
    },
    /// The input was not a valid UTF8 string
    ParseInvalidUTF8,
    // /// Some other parsing error occurred.
    // ParseOther,
}

/// A string that is guaranteed to fail to parse to a [`VolumeId32`] or [`VolumeId64`].
///
/// This type acts as a lightweight error indicator, suggesting
/// that the string cannot be parsed but offering no error
/// details. To get details, use `InvalidVolumeId::into_err`.
///
/// [`VolumeId32`]: ../struct.VolumeId32.html
/// [`VolumeId64`]: ../struct.VolumeId64.html
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InvalidVolumeId<'a>(pub(crate) &'a [u8]);

impl<'a> InvalidVolumeId<'a> {
    /// Converts the lightweight error type into detailed diagnostics.
    pub fn into_err(self) -> Error {
        // Check whether or not the input was ever actually a valid UTF8 string
        let input_str = match std::str::from_utf8(self.0) {
            Ok(s) => s,
            Err(_) => return Error(ErrorKind::ParseInvalidUTF8),
        };

        let mut hyphen_count = 0;
        let mut group_bounds = [0; 4];

        for (index, character) in input_str.char_indices() {
            let byte = character as u8;
            if character as u32 - byte as u32 > 0 {
                // Multibyte char
                return Error(ErrorKind::ParseChar {
                    character,
                    index: index + 1,
                });
            } else if byte == b'-' {
                // While we search, also count group breaks
                if hyphen_count < 1 {
                    group_bounds[hyphen_count] = index;
                }
                hyphen_count += 1;
            } else if !byte.is_ascii_hexdigit() {
                // Non-hex char
                return Error(ErrorKind::ParseChar {
                    character: byte as char,
                    index: index + 1,
                });
            }
        }

        if hyphen_count == 0 {
            // This means that we tried and failed to parse a simple uuid.
            // Since we verified that all the characters are valid, this means
            // that it MUST have an invalid length.
            Error(ErrorKind::ParseSimpleLength {
                len: input_str.len(),
            })
        } else if hyphen_count != 1 {
            // We tried to parse a hyphenated variant, but there weren't
            // 2 groups (1 hyphen splits).
            Error(ErrorKind::ParseGroupCount {
                count: hyphen_count + 1,
            })
        } else {
            // There are 2 groups, one of them has an incorrect length
            const BLOCK_STARTS: [usize; 2] = [0, 5];
            for i in 0..1 {
                if group_bounds[i] != BLOCK_STARTS[i + 1] - 1 {
                    return Error(ErrorKind::ParseGroupLength {
                        group: i,
                        len: group_bounds[i] - BLOCK_STARTS[i],
                        index: BLOCK_STARTS[i] + 1,
                    });
                }
            }

            // // The last group must be too short/long
            Error(ErrorKind::ParseGroupLength {
                group: 1,
                len: input_str.len() - BLOCK_STARTS[1],
                index: BLOCK_STARTS[1] + 1,
            })
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ErrorKind::ParseChar {
                character, index, ..
            } => {
                write!(
                    f,
                    "invalid character: expected [0-9a-fA-F-], found `{}` at {}",
                    character, index
                )
            }
            ErrorKind::ParseSimpleLength { len } => {
                write!(
                    f,
                    "invalid length: expected length for simple format, found {}",
                    len
                )
            }
            ErrorKind::ParseByteLength { len } => {
                write!(f, "invalid byte length, found {}", len)
            }
            ErrorKind::ParseGroupCount { count } => {
                write!(f, "invalid group count: expected 2, found {}", count)
            }
            ErrorKind::ParseGroupLength { group, len, .. } => {
                let expected = [8, 4, 4, 4, 12][group];
                write!(
                    f,
                    "invalid group length in group {}: expected {}, found {}",
                    group, expected, len
                )
            }
            ErrorKind::ParseInvalidUTF8 => write!(f, "non-UTF8 input"),
            // ErrorKind::ParseOther => write!(f, "failed to parse a UUID"),
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;
    use crate::std::error;

    impl error::Error for Error {}
}
