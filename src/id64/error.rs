use crate::std::{error, fmt, str::from_utf8};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error(pub(crate) ErrorKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ErrorKind {
    /// Invalid character in the [`VolumeId64`] string.
    ///
    /// [`VolumeId64`]: ../struct.VolumeId64.html
    ParseChar { character: char, index: usize },
    /// A simple [`VolumeId64`] didn't contain 16 characters.
    ///
    /// [`VolumeId64`]: ../struct.VolumeId64.html
    ParseSimpleLength { len: usize },
    /// A byte array didn't contain 8 bytes
    ParseByteLength { len: usize },
    /// The input was not a valid UTF8 string
    ParseInvalidUTF8,
    /// Some other parsing error occurred.
    ParseOther,
}

/// A string that is guaranteed to fail to parse to a [`VolumeId64`].
///
/// This type acts as a lightweight error indicator, suggesting
/// that the string cannot be parsed but offering no error
/// details. To get details, use `InvalidVolumeId64::into_err`.
///
/// [`VolumeId64`]: ../struct.VolumeId64.html
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InvalidVolumeId64<'a>(pub(crate) &'a [u8]);

impl<'a> InvalidVolumeId64<'a> {
    /// Converts the lightweight error type into detailed diagnostics.
    pub fn into_err(self) -> Error {
        // Check whether or not the input was ever actually a valid UTF8 string
        let input_str = match from_utf8(self.0) {
            Ok(s) => s,
            Err(_) => return Error(ErrorKind::ParseInvalidUTF8),
        };

        for (index, character) in input_str.char_indices() {
            let byte = character as u8;
            if character as u32 - byte as u32 > 0 {
                // Multibyte char
                return Error(ErrorKind::ParseChar {
                    character,
                    index: index + 1,
                });
            } else if !byte.is_ascii_hexdigit() {
                // Non-hex char
                return Error(ErrorKind::ParseChar {
                    character: byte as char,
                    index: index + 1,
                });
            }
        }

        // This means that we tried and failed to parse a simpleid64.
        // Since we verified that all the characters are valid, this means
        // that it MUST have an invalid length.
        return Error(ErrorKind::ParseSimpleLength {
            len: input_str.len(),
        });
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
            ErrorKind::ParseInvalidUTF8 => write!(f, "non-UTF8 input"),
            ErrorKind::ParseOther => write!(f, "failed to parse a VolumeId64"),
        }
    }
}

impl error::Error for Error {}
