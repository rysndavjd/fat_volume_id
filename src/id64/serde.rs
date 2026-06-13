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
    id64::{Error, VolumeId64, fmt::SimpleId64},
    std::{fmt, marker::PhantomData},
};
use serde_core::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Error as _},
};

fn de_error<E: de::Error>(e: Error) -> E {
    E::custom(format_args!("VolumeId64 parsing failed: {}", e))
}

impl Serialize for VolumeId64 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(self.simple().encode_lower(&mut [0u8; SimpleId64::LENGTH]))
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

impl Serialize for SimpleId64 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.encode_lower(&mut [0u8; SimpleId64::LENGTH]))
    }
}

struct ReadableVisitor<T> {
    expecting: &'static str,
    _marker: PhantomData<T>,
}

impl<'vi, T: DeserializeId64> de::Visitor<'vi> for ReadableVisitor<T> {
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
            match seq.next_element()? { Some(e) => e, None => return Err(A::Error::invalid_length(4, &self)) },
            match seq.next_element()? { Some(e) => e, None => return Err(A::Error::invalid_length(5, &self)) },
            match seq.next_element()? { Some(e) => e, None => return Err(A::Error::invalid_length(6, &self)) },
            match seq.next_element()? { Some(e) => e, None => return Err(A::Error::invalid_length(7, &self)) },
        ];

        T::from_bytes(bytes).map_err(de_error)
    }
}

struct BytesVisitor<T> {
    _marker: PhantomData<T>,
}

impl<'vi, T: DeserializeId64> de::Visitor<'vi> for BytesVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a 4 byte array")
    }

    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<T, E> {
        T::from_slice(value).map_err(de_error)
    }
}

trait DeserializeId64 {
    fn from_str(formatted: &str) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_slice(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_bytes(bytes: [u8; 8]) -> Result<Self, Error>
    where
        Self: Sized;
}

impl DeserializeId64 for VolumeId64 {
    fn from_str(formatted: &str) -> Result<Self, Error> {
        formatted.parse()
    }

    fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        VolumeId64::from_slice(bytes)
    }

    fn from_bytes(bytes: [u8; 8]) -> Result<Self, Error> {
        Ok(VolumeId64::from_bytes(bytes))
    }
}

impl<'de> Deserialize<'de> for VolumeId64 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(ReadableVisitor {
                expecting: "a formatted volumeid64 string",
                _marker: PhantomData::<VolumeId64>,
            })
        } else {
            deserializer.deserialize_bytes(BytesVisitor {
                _marker: PhantomData::<VolumeId64>,
            })
        }
    }
}

impl DeserializeId64 for SimpleId64 {
    fn from_str(formatted: &str) -> Result<Self, Error> {
        formatted.parse()
    }

    fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        Ok(VolumeId64::from_slice(bytes)?.into())
    }

    fn from_bytes(bytes: [u8; 8]) -> Result<Self, Error> {
        Ok(VolumeId64::from_bytes(bytes).into())
    }
}

impl<'de> Deserialize<'de> for SimpleId64 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(ReadableVisitor {
            expecting: "a VolumeId64 string in the simple format",
            _marker: PhantomData::<SimpleId64>,
        })
    }
}

/// Serialize a [`VolumeId64`] as a `[u8; 8]`.
///
/// [`VolumeId64`]: crate::id64::VolumeId64
pub mod compact {
    /// Serialize from a [`VolumeId64`] as a `[u8; 8]`
    ///
    /// [`VolumeId64`]: crate::id64::VolumeId64
    pub fn serialize<S>(u: &crate::id64::VolumeId64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serde_core::Serialize::serialize(u.as_bytes(), serializer)
    }

    /// Deserialize a `[u8; 8]` as a [`VolumeId64`]
    ///
    /// [`VolumeId64`]: crate::id64::VolumeId64
    pub fn deserialize<'de, D>(deserializer: D) -> Result<crate::id64::VolumeId64, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        let bytes: [u8; 8] = serde_core::Deserialize::deserialize(deserializer)?;

        Ok(crate::id64::VolumeId64::from_bytes(bytes))
    }

    #[cfg(test)]
    mod tests {
        use serde_derive::*;
        use serde_test::Configure;

        #[test]
        fn test_serialize_compact() {
            #[derive(Serialize, Debug, Deserialize, PartialEq)]
            struct Container {
                #[serde(with = "crate::id64::serde::compact")]
                u: crate::id64::VolumeId64,
            }

            let bytes: &[u8; 8] = b"F9168C5E";
            let container = Container {
                u: crate::id64::VolumeId64::from_slice(bytes).unwrap(),
            };

            serde_test::assert_tokens(
                &container.compact(),
                &[
                    serde_test::Token::Struct {
                        name: "Container",
                        len: 1,
                    },
                    serde_test::Token::Str("u"),
                    serde_test::Token::Tuple { len: 8 },
                    serde_test::Token::U8(bytes[0]),
                    serde_test::Token::U8(bytes[1]),
                    serde_test::Token::U8(bytes[2]),
                    serde_test::Token::U8(bytes[3]),
                    serde_test::Token::U8(bytes[4]),
                    serde_test::Token::U8(bytes[5]),
                    serde_test::Token::U8(bytes[6]),
                    serde_test::Token::U8(bytes[7]),
                    serde_test::Token::TupleEnd,
                    serde_test::Token::StructEnd,
                ],
            )
        }
    }
}

/// Serialize a [`VolumeId64`] as [`SimpleId64`].
///
/// [`VolumeId64`]: crate::id64::VolumeId64
/// [`SimpleId64`]: crate::id64::fmt::SimpleId64
///
/// ## Examples
///
/// Serialize and deserialize using the simple format, failing to deserialize
/// any other format:
///
/// ```
/// #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
/// struct StructA {
///     #[serde(with = "fat_volume_id::id64::serde::simple")]
///     id: fat_volume_id::id64::VolumeId64,
/// }
/// ```
///
/// Serialize using the simple format, but deserialize any format:
///
/// ```
/// #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
/// struct StructB {
///     #[serde(serialize_with = "fat_volume_id::id64::serde::simple::serialize")]
///     id: fat_volume_id::id64::VolumeId64,
/// }
/// ```
pub mod simple {
    use super::*;

    /// Serialize a [`VolumeId64`] as a simple string.
    ///
    /// [`VolumeId64`]: crate::id64::VolumeId64
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(serde_derive::Serialize)]
    /// struct Struct {
    ///     #[serde(serialize_with = "fat_volume_id::id64::serde::simple::serialize")]
    ///     id: fat_volume_id::id64::VolumeId64,
    /// }
    ///
    /// ```
    pub fn serialize<S>(u: &VolumeId64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serde_core::Serialize::serialize(u.as_simple(), serializer)
    }

    /// Deserialize a simple-formatted string as a [`VolumeId64`].
    ///
    /// [`VolumeId64`]: crate::id64::VolumeId64
    pub fn deserialize<'de, D>(deserializer: D) -> Result<VolumeId64, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        Ok(SimpleId64::deserialize(deserializer)?.into())
    }

    #[cfg(test)]
    mod tests {
        use crate::id64::VolumeId64;
        use serde_test::Token;

        const SIMPLE_STR: &str = "cc0e01bd0e01a196";

        #[test]
        fn test_serialize_as_simple() {
            #[derive(serde_derive::Serialize)]
            struct Struct(#[serde(with = "super")] VolumeId64);

            let u = Struct(VolumeId64::parse(SIMPLE_STR).unwrap());
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
            struct Struct(#[serde(with = "super")] VolumeId64);
            let s = Struct(SIMPLE_STR.parse().unwrap());
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
    }
}

#[cfg(test)]
mod serde_tests {
    use super::*;

    use serde_test::{Compact, Configure, Readable, Token};

    #[test]
    fn test_serialize_readable_string() {
        let str = "cc0e01bd0e01a196";
        let v = VolumeId64::parse(str).unwrap();
        serde_test::assert_tokens(&v.readable(), &[Token::Str(str)]);
    }

    #[test]
    fn test_deserialize_readable_compact() {
        let bytes = b"cc0e01bd";
        let v = VolumeId64::from_slice(bytes).unwrap();

        serde_test::assert_de_tokens(
            &v.readable(),
            &[
                serde_test::Token::Tuple { len: 8 },
                serde_test::Token::U8(bytes[0]),
                serde_test::Token::U8(bytes[1]),
                serde_test::Token::U8(bytes[2]),
                serde_test::Token::U8(bytes[3]),
                serde_test::Token::U8(bytes[4]),
                serde_test::Token::U8(bytes[5]),
                serde_test::Token::U8(bytes[6]),
                serde_test::Token::U8(bytes[7]),
                serde_test::Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_deserialize_readable_bytes() {
        let bytes = b"F9168C5E";
        let v = VolumeId64::from_slice(bytes).unwrap();

        serde_test::assert_de_tokens(&v.readable(), &[serde_test::Token::Bytes(bytes)]);
    }

    #[test]
    fn test_serialize_simple() {
        let str = "cc0e01bd0e01a196";
        let v = VolumeId64::parse(str).unwrap();
        serde_test::assert_ser_tokens(&v.simple(), &[Token::Str(str)]);
        serde_test::assert_de_tokens(&v.simple(), &[Token::Str(str)]);
    }

    #[test]
    fn test_serialize_non_human_readable() {
        let bytes = b"cc0e01bd";
        let v = VolumeId64::from_slice(bytes).unwrap();
        serde_test::assert_tokens(
            &v.compact(),
            &[serde_test::Token::Bytes(&[
                99, 99, 48, 101, 48, 49, 98, 100,
            ])],
        );
    }

    #[test]
    fn test_de_failure() {
        serde_test::assert_de_tokens_error::<Readable<VolumeId64>>(
            &[Token::Str("hello_world")],
            "VolumeId64 parsing failed: invalid character: expected [0-9a-fA-F], found `h` at 1",
        );

        serde_test::assert_de_tokens_error::<Compact<VolumeId64>>(
            &[Token::Bytes(b"hello_world")],
            "VolumeId64 parsing failed: invalid byte length, found 11",
        );
    }
}
