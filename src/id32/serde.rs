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

//! Adapters for alternative `serde` formats.

use crate::{
    id32::{
        Error, VolumeId32,
        fmt::{HyphenatedId32, SimpleId32},
    },
    std::{fmt, marker::PhantomData},
};
use serde_core::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Error as _},
};

fn de_error<E: de::Error>(e: Error) -> E {
    E::custom(format_args!("VolumeId32 parsing failed: {}", e))
}

impl Serialize for VolumeId32 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(
                self.hyphenated()
                    .encode_lower(&mut [0u8; HyphenatedId32::LENGTH]),
            )
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

impl Serialize for SimpleId32 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.encode_lower(&mut [0u8; SimpleId32::LENGTH]))
    }
}

impl Serialize for HyphenatedId32 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.encode_lower(&mut [0u8; HyphenatedId32::LENGTH]))
    }
}

struct ReadableVisitor<T> {
    expecting: &'static str,
    _marker: PhantomData<T>,
}

impl<'vi, T: DeserializeId32> de::Visitor<'vi> for ReadableVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.expecting)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<T, E> {
        T::from_str(value).map_err(de_error)
    }

    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<T, E> {
        T::from_slice(value).map_err(de_error)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<T, A::Error>
    where
        A: de::SeqAccess<'vi>,
    {
        #[rustfmt::skip]
        let bytes = [
            match seq.next_element()? { Some(e) => e, None => return Err(A::Error::invalid_length(0, &self)) },
            match seq.next_element()? { Some(e) => e, None => return Err(A::Error::invalid_length(1, &self)) },
            match seq.next_element()? { Some(e) => e, None => return Err(A::Error::invalid_length(2, &self)) },
            match seq.next_element()? { Some(e) => e, None => return Err(A::Error::invalid_length(3, &self)) },
        ];

        T::from_bytes(bytes).map_err(de_error)
    }
}

struct BytesVisitor<T> {
    _marker: PhantomData<T>,
}

impl<'vi, T: DeserializeId32> de::Visitor<'vi> for BytesVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a 4 byte array")
    }

    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<T, E> {
        T::from_slice(value).map_err(de_error)
    }
}

trait DeserializeId32 {
    fn from_str(formatted: &str) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_slice(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_bytes(bytes: [u8; 4]) -> Result<Self, Error>
    where
        Self: Sized;
}

impl DeserializeId32 for VolumeId32 {
    fn from_str(formatted: &str) -> Result<Self, Error> {
        formatted.parse()
    }

    fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        VolumeId32::from_slice(bytes)
    }

    fn from_bytes(bytes: [u8; 4]) -> Result<Self, Error> {
        Ok(VolumeId32::from_bytes(bytes))
    }
}

impl<'de> Deserialize<'de> for VolumeId32 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(ReadableVisitor {
                expecting: "a formatted volumeid32 string",
                _marker: PhantomData::<VolumeId32>,
            })
        } else {
            deserializer.deserialize_bytes(BytesVisitor {
                _marker: PhantomData::<VolumeId32>,
            })
        }
    }
}

impl DeserializeId32 for SimpleId32 {
    fn from_str(formatted: &str) -> Result<Self, Error> {
        formatted.parse()
    }

    fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        Ok(VolumeId32::from_slice(bytes)?.into())
    }

    fn from_bytes(bytes: [u8; 4]) -> Result<Self, Error> {
        Ok(VolumeId32::from_bytes(bytes).into())
    }
}

impl<'de> Deserialize<'de> for SimpleId32 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(ReadableVisitor {
            expecting: "a VolumeId32 string in the simple format",
            _marker: PhantomData::<SimpleId32>,
        })
    }
}

impl DeserializeId32 for HyphenatedId32 {
    fn from_str(formatted: &str) -> Result<Self, Error> {
        formatted.parse()
    }

    fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        Ok(VolumeId32::from_slice(bytes)?.into())
    }

    fn from_bytes(bytes: [u8; 4]) -> Result<Self, Error> {
        Ok(VolumeId32::from_bytes(bytes).into())
    }
}

impl<'de> Deserialize<'de> for HyphenatedId32 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(ReadableVisitor {
            expecting: "a VolumeId32 string in the hyphenated format",
            _marker: PhantomData::<HyphenatedId32>,
        })
    }
}

/// Serialize a [`VolumeId32`] as a `[u8; 4]`.
///
/// [`VolumeId32`]: crate::id32::VolumeId32
pub mod compact {
    /// Serialize from a [`VolumeId32`] as a `[u8; 4]`
    ///
    /// [`VolumeId32`]: crate::id32::VolumeId32
    pub fn serialize<S>(u: &crate::id32::VolumeId32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serde_core::Serialize::serialize(u.as_bytes(), serializer)
    }

    /// Deserialize a `[u8; 4]` as a [`VolumeId32`]
    ///
    /// [`VolumeId32`]: crate::id32::VolumeId32
    pub fn deserialize<'de, D>(deserializer: D) -> Result<crate::id32::VolumeId32, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        let bytes: [u8; 4] = serde_core::Deserialize::deserialize(deserializer)?;

        Ok(crate::id32::VolumeId32::from_bytes(bytes))
    }

    #[cfg(test)]
    mod tests {
        use serde_derive::*;
        use serde_test::Configure;

        #[test]
        fn test_serialize_compact() {
            #[derive(Serialize, Debug, Deserialize, PartialEq)]
            struct Container {
                #[serde(with = "crate::id32::serde::compact")]
                u: crate::id32::VolumeId32,
            }

            let bytes: &[u8; 4] = b"F916";
            let container = Container {
                u: crate::id32::VolumeId32::from_slice(bytes).unwrap(),
            };

            serde_test::assert_tokens(
                &container.compact(),
                &[
                    serde_test::Token::Struct {
                        name: "Container",
                        len: 1,
                    },
                    serde_test::Token::Str("u"),
                    serde_test::Token::Tuple { len: 4 },
                    serde_test::Token::U8(bytes[0]),
                    serde_test::Token::U8(bytes[1]),
                    serde_test::Token::U8(bytes[2]),
                    serde_test::Token::U8(bytes[3]),
                    serde_test::Token::TupleEnd,
                    serde_test::Token::StructEnd,
                ],
            )
        }
    }
}

/// Serialize a [`VolumeId32`] as [`SimpleId32`].
///
/// [`VolumeId32`]: crate::id32::VolumeId32
/// [`SimpleId32`]: crate::id32::fmt::SimpleId32
///
/// ## Examples
///
/// Serialize and deserialize using the simple format, failing to deserialize
/// any other format:
///
/// ```
/// #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
/// struct StructA {
///     #[serde(with = "fat_volume_id::id32::serde::simple")]
///     id: fat_volume_id::id32::VolumeId32,
/// }
/// ```
///
/// Serialize using the simple format, but deserialize any format:
///
/// ```
/// #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
/// struct StructB {
///     #[serde(serialize_with = "fat_volume_id::id32::serde::simple::serialize")]
///     id: fat_volume_id::id32::VolumeId32,
/// }
/// ```
pub mod simple {
    use super::*;

    /// Serialize a [`VolumeId32`] as a simple string.
    ///
    /// [`VolumeId32`]: crate::id32::VolumeId32
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(serde_derive::Serialize)]
    /// struct Struct {
    ///     #[serde(serialize_with = "fat_volume_id::id32::serde::simple::serialize")]
    ///     id: fat_volume_id::id32::VolumeId32,
    /// }
    ///
    /// ```
    pub fn serialize<S>(u: &VolumeId32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serde_core::Serialize::serialize(u.as_simple(), serializer)
    }

    /// Deserialize a simple-formatted string as a [`VolumeId32`].
    ///
    /// [`VolumeId32`]: crate::id32::VolumeId32
    pub fn deserialize<'de, D>(deserializer: D) -> Result<VolumeId32, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        Ok(SimpleId32::deserialize(deserializer)?.into())
    }

    #[cfg(test)]
    mod tests {
        use crate::id32::VolumeId32;
        use serde_test::{Readable, Token};

        const HYPHENATED_STR: &str = "f916-8c5e";
        const SIMPLE_STR: &str = "f9168c5e";

        #[test]
        fn test_serialize_as_simple() {
            #[derive(serde_derive::Serialize)]
            struct Struct(#[serde(with = "super")] VolumeId32);

            let u = Struct(VolumeId32::parse(HYPHENATED_STR).unwrap());
            serde_test::assert_ser_tokens(
                &u,
                &[
                    Token::NewtypeStruct { name: "Struct" },
                    Token::Str(SIMPLE_STR),
                ],
            );
        }

        #[test]
        fn test_de_from_simple() {
            #[derive(PartialEq, Debug, serde_derive::Deserialize)]
            struct Struct(#[serde(with = "super")] VolumeId32);
            let s = Struct(HYPHENATED_STR.parse().unwrap());
            serde_test::assert_de_tokens::<Struct>(
                &s,
                &[
                    Token::TupleStruct {
                        name: "Struct",
                        len: 1,
                    },
                    Token::BorrowedStr(SIMPLE_STR),
                    Token::TupleStructEnd,
                ],
            );
        }

        #[test]
        fn test_de_reject_hyphenated() {
            #[derive(PartialEq, Debug, serde_derive::Deserialize)]
            struct Struct(#[serde(with = "super")] VolumeId32);
            serde_test::assert_de_tokens_error::<Readable<Struct>>(
                &[
                    Token::TupleStruct {
                        name: "Struct",
                        len: 1,
                    },
                    Token::BorrowedStr(HYPHENATED_STR),
                    Token::TupleStructEnd,
                ],
                "VolumeId32 parsing failed: invalid group length in group 1: expected 4, found 4",
            );
        }
    }
}

/// Serialize a [`VolumeId32`] as [`HyphenatedId32`].
///
/// [`VolumeId32`]: crate::id32::VolumeId32
/// [`HyphenatedId32`]: crate::id32::fmt::HyphenatedId32
///
/// ## Examples
///
/// Serialize and deserialize using the hyphenated format, failing to deserialize
/// any other format:
///
/// ```
/// #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
/// struct StructA {
///     #[serde(with = "fat_volume_id::id32::serde::hyphenated")]
///     id: fat_volume_id::id32::VolumeId32,
/// }
/// ```
///
/// Serialize using the hyphenated format, but deserialize any format:
///
/// ```
/// #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
/// struct StructB {
///     #[serde(serialize_with = "fat_volume_id::id32::serde::hyphenated::serialize")]
///     id: fat_volume_id::id32::VolumeId32,
/// }
/// ```
pub mod hyphenated {
    use super::*;

    /// Serialize a [`VolumeId32`] as a hyphenated string.
    ///
    /// [`VolumeId32`]: crate::id32::VolumeId32
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(serde_derive::Serialize)]
    /// struct Struct {
    ///     #[serde(serialize_with = "fat_volume_id::id32::serde::hyphenated::serialize")]
    ///     id: fat_volume_id::id32::VolumeId32,
    /// }
    ///
    /// ```
    pub fn serialize<S>(u: &VolumeId32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serde_core::Serialize::serialize(u.as_hyphenated(), serializer)
    }

    /// Deserialize a hyphenated-formatted string as a [`VolumeId32`].
    ///
    /// [`VolumeId32`]: crate::id32::VolumeId32
    pub fn deserialize<'de, D>(deserializer: D) -> Result<VolumeId32, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        Ok(HyphenatedId32::deserialize(deserializer)?.into())
    }

    #[cfg(test)]
    mod tests {
        use crate::id32::VolumeId32;
        use serde_test::{Readable, Token};

        const HYPHENATED_STR: &str = "f916-8c5e";
        const SIMPLE_STR: &str = "f9168c5e";

        #[test]
        fn test_serialize_as_hyphenated() {
            #[derive(serde_derive::Serialize)]
            struct Struct(#[serde(with = "super")] VolumeId32);

            let u = Struct(VolumeId32::parse(HYPHENATED_STR).unwrap());
            serde_test::assert_ser_tokens(
                &u,
                &[
                    Token::NewtypeStruct { name: "Struct" },
                    Token::Str(HYPHENATED_STR),
                ],
            );
        }

        #[test]
        fn test_de_from_hyphenated() {
            #[derive(PartialEq, Debug, serde_derive::Deserialize)]
            struct Struct(#[serde(with = "super")] VolumeId32);
            let s = Struct(HYPHENATED_STR.parse().unwrap());
            serde_test::assert_de_tokens::<Struct>(
                &s,
                &[
                    Token::TupleStruct {
                        name: "Struct",
                        len: 1,
                    },
                    Token::BorrowedStr(HYPHENATED_STR),
                    Token::TupleStructEnd,
                ],
            );
        }

        #[test]
        fn test_de_reject_hyphenated() {
            #[derive(PartialEq, Debug, serde_derive::Deserialize)]
            struct Struct(#[serde(with = "super")] VolumeId32);
            serde_test::assert_de_tokens_error::<Readable<Struct>>(
                &[
                    Token::TupleStruct {
                        name: "Struct",
                        len: 1,
                    },
                    Token::BorrowedStr(SIMPLE_STR),
                    Token::TupleStructEnd,
                ],
                "VolumeId32 parsing failed: invalid length: expected length for simple format, found 8",
            );
        }
    }
}

#[cfg(test)]
mod serde_tests {
    use super::*;

    use serde_test::{Compact, Configure, Readable, Token};

    #[test]
    fn test_serialize_readable_string() {
        let str = "f916-8c5e";
        let v = VolumeId32::parse(str).unwrap();
        serde_test::assert_tokens(&v.readable(), &[Token::Str(str)]);
    }

    #[test]
    fn test_deserialize_readable_compact() {
        let bytes = b"F916";
        let v = VolumeId32::from_slice(bytes).unwrap();

        serde_test::assert_de_tokens(
            &v.readable(),
            &[
                serde_test::Token::Tuple { len: 16 },
                serde_test::Token::U8(bytes[0]),
                serde_test::Token::U8(bytes[1]),
                serde_test::Token::U8(bytes[2]),
                serde_test::Token::U8(bytes[3]),
                serde_test::Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_deserialize_readable_bytes() {
        let bytes = b"F916";
        let v = VolumeId32::from_slice(bytes).unwrap();

        serde_test::assert_de_tokens(&v.readable(), &[serde_test::Token::Bytes(bytes)]);
    }

    #[test]
    fn test_serialize_simple() {
        let str = "f9168c5e";
        let v = VolumeId32::parse(str).unwrap();
        serde_test::assert_ser_tokens(&v.simple(), &[Token::Str(str)]);
        serde_test::assert_de_tokens(&v.simple(), &[Token::Str(str)]);
    }

    #[test]
    fn test_serialize_hyphenated() {
        let str = "f916-8c5e";
        let v = VolumeId32::parse(str).unwrap();
        serde_test::assert_ser_tokens(&v.hyphenated(), &[Token::Str(str)]);
        serde_test::assert_de_tokens(&v.hyphenated(), &[Token::Str(str)]);
    }

    #[test]
    fn test_serialize_non_human_readable() {
        let bytes = b"F916";
        let v = VolumeId32::from_slice(bytes).unwrap();
        serde_test::assert_tokens(&v.compact(), &[serde_test::Token::Bytes(&[70, 57, 49, 54])]);
    }

    #[test]
    fn test_de_failure() {
        serde_test::assert_de_tokens_error::<Readable<VolumeId32>>(
            &[Token::Str("hello_world")],
            "VolumeId32 parsing failed: invalid character: expected [0-9a-fA-F], found `h` at 1",
        );

        serde_test::assert_de_tokens_error::<Compact<VolumeId32>>(
            &[Token::Bytes(b"hello_world")],
            "VolumeId32 parsing failed: invalid byte length, found 11",
        );
    }
}
