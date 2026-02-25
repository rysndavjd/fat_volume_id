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

//! Generate and parse 32bit and 64bit volume serial number or volume identifier.
//!
//! Example of a 32bit volume identifier:
//!
//! ```text
//! 7E5D-2CF4
//! ```
//!
//! Example of a 64bit volume identifier:
//!
//! ```text
//! CC0E01BD0E01A196
//! ```

#![no_std]
//#![deny(missing_debug_implementations, missing_docs)]
#![allow(clippy::needless_return)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

mod common;
mod id32;
mod id64;

#[cfg(feature = "id32")]
pub use id32::{
    VolumeId32,
    fmt::{HyphenatedId32, SimpleId32},
};

#[cfg(feature = "id64")]
pub use id64::{VolumeId64, fmt::SimpleId64};
