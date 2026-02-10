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

#![no_std]
#![allow(clippy::needless_return)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

mod id32;
mod id64;

#[cfg(feature = "id32")]
pub use id32::{
    VolumeId32,
    fmt::{HyphenatedId32, SimpleId32},
};
