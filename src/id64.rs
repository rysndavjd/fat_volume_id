mod error;
mod fmt;

/// 64-bit Volume ID used in NTFS filesystems.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
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
pub struct VolumeId64([u8; 8]);

impl VolumeId64 {
    /// A VolumeId64 with all zeros.
    // Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let volumeid64 = VolumeId64::nil();
    ///
    /// assert_eq!(
    ///     "0000000000000000",
    ///     volumeid64.to_string(),
    /// );
    /// ```
    pub const fn nil() -> Self {
        return VolumeId64([0u8; 8]);
    }

    /// A VolumeId64 with all ones.
    // Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let volumeid64 = VolumeId64::max();
    ///
    /// assert_eq!(
    ///     "ffffffffffffffff",
    ///     volumeid64.to_string(),
    /// );
    /// ```
    pub const fn max() -> Self {
        return VolumeId64([0xffu8; 8]);
    }

    /// Creates a VolumeId64 using supplied bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// # use std::string::ToString;
    /// let bytes = [0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8];
    ///
    /// let volumeid64 = VolumeId64::from_bytes(bytes);
    ///
    /// assert_eq!(volumeid64.to_string(), "a1a2a3a4a5a6a7a8");
    /// ```
    #[inline]
    pub const fn from_bytes(bytes: [u8; 8]) -> VolumeId64 {
        return VolumeId64(bytes);
    }

    /// Creates a VolumeId64 using supplied bytes in big-endian.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// # use std::string::ToString;
    /// let bytes = [0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8];
    ///
    /// let volumeid64 = VolumeId64::from_bytes_be(bytes);
    ///
    /// assert_eq!(volumeid64.to_string(), "a8a7a6a5a4a3a2a1");
    /// ```
    #[inline]
    pub const fn from_bytes_be(b: [u8; 8]) -> VolumeId64 {
        return VolumeId64([b[7], b[6], b[5], b[4], b[3], b[2], b[1], b[0]]);
    }
}

impl crate::std::hash::Hash for VolumeId64 {
    fn hash<H: crate::std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.0);
    }
}

impl AsRef<VolumeId64> for VolumeId64 {
    #[inline]
    fn as_ref(&self) -> &VolumeId64 {
        self
    }
}

impl AsRef<[u8]> for VolumeId64 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl From<VolumeId64> for alloc::vec::Vec<u8> {
    fn from(value: VolumeId64) -> Self {
        value.0.to_vec()
    }
}

// #[cfg(feature = "std")]
// impl TryFrom<std::vec::Vec<u8>> for VolumeId64 {
//     type Error = Error;

//     fn try_from(value: std::vec::Vec<u8>) -> Result<Self, Self::Error> {
//         VolumeId64::from_slice(&value)
//     }
// }
