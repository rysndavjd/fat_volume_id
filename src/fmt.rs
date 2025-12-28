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

use core::str::FromStr;

use crate::{
    Error, VolumeId32,
    std::{fmt, str},
};

#[cfg(feature = "std")]
use crate::std::string::{String, ToString};

impl fmt::Debug for VolumeId32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::LowerHex::fmt(&self, f);
    }
}

impl fmt::Display for VolumeId32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::LowerHex::fmt(&self, f);
    }
}

#[cfg(feature = "std")]
impl From<VolumeId32> for String {
    fn from(volumeid32: VolumeId32) -> Self {
        volumeid32.to_string()
    }
}

impl fmt::LowerHex for VolumeId32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        return Ok(());
    }
}

impl fmt::UpperHex for VolumeId32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        return Ok(());
    }
}

impl FromStr for VolumeId32 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Self::try_parse(s);
    }
}
