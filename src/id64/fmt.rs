use crate::{
    id64::VolumeId64,
    std::{borrow::Borrow, fmt, hash::Hash, mem::transmute},
};

#[cfg(feature = "alloc")]
use crate::alloc::string::{String, ToString};

impl fmt::Debug for VolumeId64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::LowerHex::fmt(&self, f);
    }
}

impl fmt::Display for VolumeId64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::LowerHex::fmt(&self, f);
    }
}

#[cfg(feature = "alloc")]
impl From<VolumeId64> for String {
    fn from(volumeid32: VolumeId64) -> Self {
        volumeid32.to_string()
    }
}

impl fmt::LowerHex for VolumeId64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        return Ok(());
    }
}

impl fmt::UpperHex for VolumeId64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        return Ok(());
    }
}

/// Format a [`VolumeId64`] as a simple string, like
/// `6ddcf6dafdc2fd38`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::IntoBytes,
        zerocopy::FromBytes,
        zerocopy::KnownLayout,
        zerocopy::Immutable,
        zerocopy::Unaligned
    )
)]
#[repr(transparent)]
pub struct SimpleId64(VolumeId64);
