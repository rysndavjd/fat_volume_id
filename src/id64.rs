mod error;
pub mod fmt;
mod parser;
use crate::id64::error::{Error, ErrorKind};

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

    /// Creates a VolumeId64 using the supplied bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8
    /// ];
    ///
    /// let volumeid64 = VolumeId64::from_slice(&bytes)
    ///     .expect("Slice should be 8 bytes long");
    ///
    /// assert_eq!(
    ///     "a1a2a3a4a5a6a7a8",
    ///     volumeid64.to_string(),
    /// );
    /// ```
    pub fn from_slice(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 8 {
            return Err(Error(ErrorKind::ParseByteLength { len: b.len() }));
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(b);

        return Ok(VolumeId64::from_bytes(bytes));
    }

    /// Creates a VolumeId64 using the supplied bytes in big-endian.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8
    /// ];
    ///
    /// let volumeid64 = VolumeId64::from_slice_be(&bytes)
    ///     .expect("Slice should be 8 bytes long");
    ///
    /// assert_eq!(
    ///     "a8a7a6a5a4a3a2a1",
    ///     volumeid64.to_string(),
    /// );
    /// ```
    pub fn from_slice_be(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 8 {
            return Err(Error(ErrorKind::ParseByteLength { len: b.len() }));
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(b);

        return Ok(VolumeId64::from_bytes_be(bytes));
    }

    /// Creates a VolumeId64 from a 64bit value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let v = 0xa1a2a3a4a5a6a7a8;
    ///
    /// let volumeid64 = VolumeId64::from_u64(v);
    ///
    /// assert_eq!(
    ///     "a8a7a6a5a4a3a2a1",
    ///     volumeid64.simple().to_string(),
    /// );
    /// ```
    pub fn from_u64(v: u64) -> Self {
        VolumeId64::from_bytes(v.to_le_bytes())
    }

    /// Creates a VolumeId64 from a 64bit value in big-endian order.
    ///
    /// This is based on the endianness of the VolumeId64, rather than the target
    /// environment so bytes will be flipped on both big and little endian
    /// machines.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let v = 0xa1a2a3a4a5a6a7a8;
    ///
    /// let volumeid64 = VolumeId64::from_u64_be(v);
    ///
    /// assert_eq!(
    ///     "a1a2a3a4a5a6a7a8",
    ///     volumeid64.simple().to_string(),
    /// );
    /// ```
    pub fn from_u64_be(v: u64) -> Self {
        VolumeId64::from_bytes(v.to_be_bytes())
    }

    /// Returns a 64bit value containing the value.
    ///
    /// The bytes in the VolumeId64 will be packed directly into a `u64`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let volumeid64 = VolumeId64::parse("a1a2a3a4a5a6a7a8")
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     volumeid64.as_u64(),
    ///     0xa8a7a6a5a4a3a2a1,
    /// );
    /// ```
    pub const fn as_u64(&self) -> u64 {
        u64::from_le_bytes(*self.as_bytes())
    }

    /// Returns a 64bit big-endian value containing the value.
    ///
    /// The bytes in the `u64` will be flipped to convert into little-endian
    /// order. This is based on the endianness of the VolumeId64, rather than the
    /// target environment so bytes will be flipped on both big and little
    /// endian machines.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let volumeid64 = VolumeId64::parse("a1a2a3a4a5a6a7a8")
    /// .unwrap();
    ///
    /// assert_eq!(
    ///     volumeid64.as_u64_be(),
    ///     0xa1a2a3a4a5a6a7a8,
    /// );
    /// ```
    pub const fn as_u64_be(self) -> u64 {
        u64::from_be_bytes(*self.as_bytes())
    }

    /// Creates a VolumeId64 from two 32bit values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let hi = 0xa1a2a3a4;
    /// let lo = 0xa5a6a7a8;
    ///
    /// let volumeid64 = VolumeId64::from_u32_pair(hi, lo);
    ///
    /// assert_eq!(
    ///     "a8a7a6a5a4a3a2a1",
    ///     volumeid64.simple().to_string(),
    /// );
    /// ```
    pub fn from_u32_pair(high_bits: u32, low_bits: u32) -> Self {
        VolumeId64::from_u64(((high_bits as u64) << 32) | low_bits as u64)
    }

    /// Creates a VolumeId64 from two 32bit values in big endian order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let hi = 0xa1a2a3a4;
    /// let lo = 0xa5a6a7a8;
    ///
    /// let volumeid64 = VolumeId64::from_u32_pair_be(hi, lo);
    ///
    /// assert_eq!(
    ///     "a1a2a3a4a5a6a7a8",
    ///     volumeid64.simple().to_string(),
    /// );
    /// ```
    pub fn from_u32_pair_be(high_bits: u32, low_bits: u32) -> Self {
        VolumeId64::from_u64_be(((high_bits as u64) << 32) | low_bits as u64)
    }

    /// Returns an array of bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    /// let volumeid64 = VolumeId64::from_bytes([0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8]);
    ///
    /// assert_eq!(
    ///     volumeid64.as_bytes(),
    ///     &[0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8],
    /// );
    /// ```
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }

    /// Consumes self and returns the underlying byte value of the VolumeId64.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    ///
    /// let volumeid64 = VolumeId64::parse("a1a2a3a4a5a6a7a8")
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     volumeid64.into_bytes(),
    ///     [0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8]
    /// );
    /// ```
    #[inline]
    pub const fn into_bytes(self) -> [u8; 8] {
        self.0
    }

    /// Returns the bytes of the VolumeId64 in big-endian order.
    ///
    /// The bytes will be flipped to convert into little-endian order. This is
    /// based on the endianness of the VolumeId64, rather than the target environment
    /// so bytes will be flipped on both big and little endian machines.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fat_volume_id::VolumeId64;
    ///
    /// let volumeid64 = VolumeId64::parse("a1a2a3a4a5a6a7a8")
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     volumeid64.to_bytes_be(),
    ///     [0xa8, 0xa7, 0xa6, 0xa5, 0xa4, 0xa3, 0xa2, 0xa1]
    /// );
    /// ```
    #[inline]
    pub const fn to_bytes_be(self) -> [u8; 8] {
        [
            self.0[7], self.0[6], self.0[5], self.0[4], self.0[3], self.0[2], self.0[1], self.0[0],
        ]
    }

    /// Tests if the VolumeId64 is nil (all zeros).
    pub const fn is_nil(&self) -> bool {
        self.as_u64() == u64::MIN
    }

    /// Tests if the VolumeId64 is max (all ones).
    pub const fn is_max(&self) -> bool {
        self.as_u64() == u64::MAX
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

#[cfg(feature = "alloc")]
impl TryFrom<alloc::vec::Vec<u8>> for VolumeId64 {
    type Error = Error;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        VolumeId64::from_slice(&value)
    }
}
