pub const UPPER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

pub const LOWER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

// I have no idea how these macros from the UUID crate work so they are just
// copied and pasted here with some modifications.
#[macro_export]
macro_rules! impl_fmt_traits {
    ($Inner:ty, $($T:ident<$($a:lifetime),*>),+) => {$(
        impl<$($a),*> fmt::Display for $T<$($a),*> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(self, f)
            }
        }

        impl<$($a),*> fmt::LowerHex for $T<$($a),*> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.encode_lower(&mut [0; Self::LENGTH]))
            }
        }

        impl<$($a),*> fmt::UpperHex for $T<$($a),*> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.encode_upper(&mut [0; Self::LENGTH]))
            }
        }

        $crate::impl_fmt_from!($Inner, $T<$($a),*>);
    )+}
}

#[macro_export]
macro_rules! impl_fmt_from {
    ($Inner:ty, $T:ident<>) => {
        impl From<$Inner> for $T {
            #[inline]
            fn from(f: $Inner) -> Self {
                $T(f)
            }
        }

        impl From<$T> for $Inner {
            #[inline]
            fn from(f: $T) -> Self {
                f.0
            }
        }

        impl AsRef<$Inner> for $T {
            #[inline]
            fn as_ref(&self) -> &$Inner {
                &self.0
            }
        }

        impl Borrow<$Inner> for $T {
            #[inline]
            fn borrow(&self) -> &$Inner {
                &self.0
            }
        }
    };
    ($Inner:ty, $T:ident<$a:lifetime>) => {
        impl<$a> From<&$a $Inner> for $T<$a> {
            #[inline]
            fn from(f: &$a $Inner) -> Self {
                $T::from_volumeid32_ref(f)
            }
        }

        impl<$a> From<$T<$a>> for &$a $Inner {
            #[inline]
            fn from(f: $T<$a>) -> &$a $Inner {
                f.0
            }
        }

        impl<$a> AsRef<$Inner> for $T<$a> {
            #[inline]
            fn as_ref(&self) -> &$Inner {
                self.0
            }
        }

        impl<$a> Borrow<$Inner> for $T<$a> {
            #[inline]
            fn borrow(&self) -> &$Inner {
                self.0
            }
        }
    };
}
