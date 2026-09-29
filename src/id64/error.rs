/// A general error that can occur when working with VolumeId64s.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error(pub(crate) ErrorKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ErrorKind {
    /// Invalid character in the [`VolumeId64`] string.
    ///
    /// [`VolumeId64`]: ../struct.VolumeId64.html
    ParseChar {
        character: char,
        /// 0 based index
        index: usize,
    },
    /// A simple [`VolumeId64`] didn't contain 16 characters.
    ///
    /// [`VolumeId64`]: ../struct.VolumeId64.html
    ParseSimpleLength { len: usize },
    /// A byte array didn't contain 8 bytes
    ParseByteLength { len: usize },
    /// The input was not a valid UTF8 string
    ParseInvalidUTF8,
}

/// A string that is guaranteed to fail to parse to a [`VolumeId64`].
///
/// This type acts as a lightweight error indicator, suggesting
/// that the string cannot be parsed but offering no error
/// details. To get details, use [`InvalidVolumeId64::into_err`].
///
/// [`VolumeId64`]: ../struct.VolumeId64.html
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InvalidVolumeId64<'a>(pub(crate) &'a [u8]);

impl<'a> InvalidVolumeId64<'a> {
    /// Converts the lightweight error type into detailed diagnostics.
    pub fn into_err(self) -> Error {
        // Check whether or not the input was ever actually a valid UTF8 string
        let input_str = match crate::std::str::from_utf8(self.0) {
            Ok(s) => s,
            Err(_) => return Error(ErrorKind::ParseInvalidUTF8),
        };

        for (index, character) in input_str.char_indices() {
            if !character.is_ascii_hexdigit() {
                // Non-hex char
                return Error(ErrorKind::ParseChar { character, index });
            }
        }

        // This means that we tried and failed to parse a simpleid64.
        // Since we verified that all the characters are valid, this means
        // that it MUST have an invalid length.
        Error(ErrorKind::ParseSimpleLength {
            len: input_str.len(),
        })
    }
}

impl crate::std::fmt::Display for Error {
    fn fmt(&self, f: &mut crate::std::fmt::Formatter) -> crate::std::fmt::Result {
        match self.0 {
            ErrorKind::ParseChar {
                character, index, ..
            } => {
                write!(
                    f,
                    "invalid character: expected [0-9a-fA-F], found `{}` at {}",
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
        }
    }
}

impl crate::std::error::Error for Error {}
