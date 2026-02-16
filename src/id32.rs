mod error;
pub mod fmt;
mod parser;

use crate::id32::error::{Error, ErrorKind};

/// 32-bit Volume ID used in FAT12/16/32 and exFAT filesystems.
///
/// # Endianness
///
/// Microsoft’s FAT specification defines the FAT header as little-endian,
/// which means the volume serial number is stored in little-endian byte order.
/// This crate assumes integer inputs are already in the correct order by default,
/// regardless of the endianness of the environment. Most methods that accept integers
/// have a `_be` variant that assumes any integer values will need to have their bytes
/// flipped, regardless of the endianness of the environment.
///
/// Most users won't need to worry about endianness unless they are changing endianness
/// of Volume ID when parsing a FAT header. The important things to remember are:
///
/// - The endianness is in terms of the integer of the VolumeId32.
/// - Byte-flipping in `_be` methods applies to the integer.
/// - Endianness roundtrips, so if you create a VolumeId32 with `from_bytes_be`
///   you'll get the same values back out with `as_bytes_be`.
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
pub struct VolumeId32([u8; 4]);

impl VolumeId32 {
    /// A VolumeId32 with all zeros.
    // Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::nil();
    ///
    /// assert_eq!(
    ///     "00000000",
    ///     volumeid32.to_string(),
    /// );
    /// ```
    pub const fn nil() -> Self {
        return VolumeId32([0u8; 4]);
    }

    /// A VolumeId32 with all ones.
    // Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::max();
    ///
    /// assert_eq!(
    ///     "ffffffff",
    ///     volumeid32.to_string(),
    /// );
    /// ```
    pub const fn max() -> Self {
        return VolumeId32([0xffu8; 4]);
    }

    /// Creates a VolumeId32 using supplied bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// # use std::string::ToString;
    /// let bytes = [0xa1, 0xa2, 0xa3, 0xa4];
    ///
    /// let volumeid32 = VolumeId32::from_bytes(bytes);
    ///
    /// assert_eq!(volumeid32.to_string(), "a1a2a3a4");
    /// ```
    #[inline]
    pub const fn from_bytes(bytes: [u8; 4]) -> VolumeId32 {
        return VolumeId32(bytes);
    }

    /// Creates a VolumeId32 using supplied bytes in big-endian.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// # use std::string::ToString;
    /// let bytes = [0xa1, 0xa2, 0xa3, 0xa4];
    ///
    /// let volumeid32 = VolumeId32::from_bytes_be(bytes);
    ///
    /// assert_eq!(volumeid32.to_string(), "a4a3a2a1");
    /// ```
    #[inline]
    pub const fn from_bytes_be(b: [u8; 4]) -> VolumeId32 {
        return VolumeId32([b[3], b[2], b[1], b[0]]);
    }

    /// Creates a VolumeId32 using the supplied bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 4.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    /// ];
    ///
    /// let volumeid32 = VolumeId32::from_slice(&bytes)
    ///     .expect("Slice should be 4 bytes long");
    ///
    /// assert_eq!(
    ///     "a1a2a3a4",
    ///     volumeid32.to_string(),
    /// );
    /// ```
    pub fn from_slice(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 4 {
            return Err(Error(ErrorKind::ParseByteLength { len: b.len() }));
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(b);

        return Ok(VolumeId32::from_bytes(bytes));
    }

    /// Creates a VolumeId32 using the supplied bytes in big-endian.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 4.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    /// ];
    ///
    /// let volumeid32 = VolumeId32::from_slice_be(&bytes)
    ///     .expect("Slice should be 4 bytes long");
    ///
    /// assert_eq!(
    ///     "a4a3a2a1",
    ///     volumeid32.to_string(),
    /// );
    /// ```
    pub fn from_slice_be(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 4 {
            return Err(Error(ErrorKind::ParseByteLength { len: b.len() }));
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(b);

        return Ok(VolumeId32::from_bytes_be(bytes));
    }

    /// Creates a VolumeId32 from a 32bit value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let v = 0xa1a2a3a4;
    ///
    /// let volumeid32 = VolumeId32::from_u32(v);
    ///
    /// assert_eq!(
    ///     "a4a3-a2a1",
    ///     volumeid32.hyphenated().to_string(),
    /// );
    /// ```
    pub fn from_u32(v: u32) -> Self {
        VolumeId32::from_bytes(v.to_le_bytes())
    }

    /// Creates a VolumeId32 from a 32bit value in big-endian order.
    ///
    /// This is based on the endianness of the VolumeId32, rather than the target
    /// environment so bytes will be flipped on both big and little endian
    /// machines.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let v = 0xa1a2a3a4;
    ///
    /// let volumeid32 = VolumeId32::from_u32_be(v);
    ///
    /// assert_eq!(
    ///     "a1a2-a3a4",
    ///     volumeid32.hyphenated().to_string(),
    /// );
    /// ```
    pub fn from_u32_be(v: u32) -> Self {
        VolumeId32::from_bytes(v.to_be_bytes())
    }

    /// Returns a 32bit value containing the value.
    ///
    /// The bytes in the VolumeId32 will be packed directly into a `u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::parse("a1a2a3a4")
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     volumeid32.as_u32(),
    ///     0xa4a3a2a1,
    /// );
    /// ```
    pub const fn as_u32(&self) -> u32 {
        u32::from_le_bytes(*self.as_bytes())
    }

    /// Returns a 32bit big-endian value containing the value.
    ///
    /// The bytes in the `u32` will be flipped to convert into little-endian
    /// order. This is based on the endianness of the VolumeId32, rather than the
    /// target environment so bytes will be flipped on both big and little
    /// endian machines.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::parse("a1a2a3a4")
    /// .unwrap();
    ///
    /// assert_eq!(
    ///     volumeid32.as_u32_be(),
    ///     0xa1a2a3a4,
    /// );
    /// ```
    pub const fn as_u32_be(self) -> u32 {
        u32::from_be_bytes(*self.as_bytes())
    }

    /// Creates a VolumeId32 from two 16bit values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let hi = 0xa1a2;
    /// let lo = 0xa3a4;
    ///
    /// let volumeid32 = VolumeId32::from_u16_pair(hi, lo);
    ///
    /// assert_eq!(
    ///     "a4a3-a2a1",
    ///     volumeid32.hyphenated().to_string(),
    /// );
    /// ```
    pub fn from_u16_pair(high_bits: u16, low_bits: u16) -> Self {
        VolumeId32::from_u32(((high_bits as u32) << 16) | low_bits as u32)
    }

    /// Creates a VolumeId32 from two 16bit values in big endian order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let hi = 0xa1a2;
    /// let lo = 0xa3a4;
    ///
    /// let volumeid32 = VolumeId32::from_u16_pair_be(hi, lo);
    ///
    /// assert_eq!(
    ///     "a1a2-a3a4",
    ///     volumeid32.hyphenated().to_string(),
    /// );
    /// ```
    pub fn from_u16_pair_be(high_bits: u16, low_bits: u16) -> Self {
        VolumeId32::from_u32_be(((high_bits as u32) << 16) | low_bits as u32)
    }

    /// Returns an array of bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    /// let volumeid32 = VolumeId32::from_bytes([0xa1, 0xa2, 0xa3, 0xa4]);
    ///
    /// assert_eq!(
    ///     volumeid32.as_bytes(),
    ///     &[0xa1, 0xa2, 0xa3, 0xa4],
    /// );
    /// ```
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }

    /// Consumes self and returns the underlying byte value of the VolumeId32.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    ///
    /// let volumeid32 = VolumeId32::parse("a1a2-a3a4")
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     volumeid32.into_bytes(),
    ///     [0xa1, 0xa2, 0xa3, 0xa4]
    /// );
    /// ```
    #[inline]
    pub const fn into_bytes(self) -> [u8; 4] {
        self.0
    }

    /// Returns the bytes of the VolumeId32 in big-endian order.
    ///
    /// The bytes will be flipped to convert into little-endian order. This is
    /// based on the endianness of the VolumeId32, rather than the target environment
    /// so bytes will be flipped on both big and little endian machines.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fat_volume_id::VolumeId32;
    ///
    /// let volumeid32 = VolumeId32::parse("a1a2-a3a4")
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     volumeid32.to_bytes_be(),
    ///     [0xa4, 0xa3, 0xa2, 0xa1]
    /// );
    /// ```
    #[inline]
    pub const fn to_bytes_be(self) -> [u8; 4] {
        [self.0[3], self.0[2], self.0[1], self.0[0]]
    }

    /// Tests if the VolumeId32 is nil (all zeros).
    pub const fn is_nil(&self) -> bool {
        self.as_u32() == u32::MIN
    }

    /// Tests if the VolumeId32 is max (all ones).
    pub const fn is_max(&self) -> bool {
        self.as_u32() == u32::MAX
    }
}

impl crate::std::hash::Hash for VolumeId32 {
    fn hash<H: crate::std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.0);
    }
}

impl AsRef<VolumeId32> for VolumeId32 {
    #[inline]
    fn as_ref(&self) -> &VolumeId32 {
        self
    }
}

impl AsRef<[u8]> for VolumeId32 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl From<VolumeId32> for crate::alloc::vec::Vec<u8> {
    fn from(value: VolumeId32) -> Self {
        value.0.to_vec()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<crate::alloc::vec::Vec<u8>> for VolumeId32 {
    type Error = Error;

    fn try_from(value: crate::alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        VolumeId32::from_slice(&value)
    }
}
