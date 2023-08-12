/*
 * Copyright (c) 2022 xvanc <xvancm@gmail.com>
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its contributors
 *    may be used to endorse or promote products derived from this software without
 *    specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

use core::{cmp, fmt, ops};

macro_rules! fmt_impls {
    ($name:ident: $($fmt:ident),*) => {$(
        impl fmt::$fmt for $name {
            #[inline(always)]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::$fmt::fmt(&self.get(), f)
            }
        }
    )*};
}

macro_rules! op_assign_impl {
    ($lhs:ty, $rhs:ty) => {
        impl ops::AddAssign<$rhs> for $lhs {
            #[inline(always)]
            fn add_assign(&mut self, rhs: $rhs) {
                *self = *self + rhs;
            }
        }

        impl ops::BitAndAssign<$rhs> for $lhs {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: $rhs) {
                *self = *self & rhs;
            }
        }

        impl ops::BitOrAssign<$rhs> for $lhs {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: $rhs) {
                *self = *self | rhs;
            }
        }

        impl ops::BitXorAssign<$rhs> for $lhs {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: $rhs) {
                *self = *self ^ rhs;
            }
        }

        impl ops::DivAssign<$rhs> for $lhs {
            #[inline(always)]
            fn div_assign(&mut self, rhs: $rhs) {
                *self = *self / rhs;
            }
        }

        impl ops::MulAssign<$rhs> for $lhs {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: $rhs) {
                *self = *self * rhs;
            }
        }

        impl ops::RemAssign<$rhs> for $lhs {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: $rhs) {
                *self = *self % rhs;
            }
        }

        impl ops::ShlAssign<$rhs> for $lhs {
            #[inline(always)]
            fn shl_assign(&mut self, rhs: $rhs) {
                *self = *self << rhs;
            }
        }

        impl ops::ShrAssign<$rhs> for $lhs {
            #[inline(always)]
            fn shr_assign(&mut self, rhs: $rhs) {
                *self = *self >> rhs;
            }
        }

        impl ops::SubAssign<$rhs> for $lhs {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: $rhs) {
                *self = *self - rhs;
            }
        }
    };
}

macro_rules! endian_ints {
    (
        signed;
        from: $from:ident;
        to:   $to:ident;

        $(type $name:ident = $short:ident = $type:ty;)*
    ) => {
        endian_ints! {
            from: $from;
            to:   $to;

            $(type $name = $short = $type;)*
        }

        $(
            impl ops::Neg for $name {
                type Output = Self;

                fn neg(self) -> Self {
                    Self::new(-self.get())
                }
            }
        )*
    };

    (
        from: $from:ident;
        to:   $to:ident;

        $(type $name:ident = $short:ident = $type:ty;)*
    ) => {$(

        #[repr(transparent)]
        #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
        pub struct $name($type);

        #[allow(non_camel_case_types)]
        pub type $short = $name;

        impl $name {
            pub const MIN: Self = Self::new(<$type>::MIN);
            pub const MAX: Self = Self::new(<$type>::MAX);
            pub const BITS: u32 = <$type>::BITS;

            pub const fn new(value: $type) -> Self {
                Self(value.$to())
            }

            #[inline(always)]
            pub const fn get(self) -> $type {
                <$type>::$from(self.0)
            }
        }

        fmt_impls!($name: Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex);

        impl From<$type> for $name {
            fn from(val: $type) -> Self {
                Self::new(val)
            }
        }

        impl From<$name> for $type {
            fn from(val: $name) -> $type {
                val.get()
            }
        }


        // $name op $name


        impl ops::Add for $name {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self::new(self.get() + rhs.get())
            }
        }

        impl ops::BitAnd for $name {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl ops::BitOr for $name {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl ops::BitXor for $name {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl ops::Div for $name {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                Self::new(self.get() / rhs.get())
            }
        }

        impl ops::Mul for $name {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                Self::new(self.get() * rhs.get())
            }
        }

        impl ops::Not for $name {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
                Self(!self.0)
            }
        }

        impl ops::Rem for $name {
            type Output = Self;

            #[inline(always)]
            fn rem(self, rhs: Self) -> Self {
                Self::new(self.get() % rhs.get())
            }
        }

        impl ops::Shl for $name {
            type Output = Self;

            #[inline(always)]
            fn shl(self, rhs: Self) -> Self {
                Self::new(self.get() << rhs.get())
            }
        }

        impl ops::Shr for $name {
            type Output = Self;

            #[inline(always)]
            fn shr(self, rhs: Self) -> Self {
                Self::new(self.get() >> rhs.get())
            }
        }

        impl ops::Sub for $name {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self::new(self.get() - rhs.get())
            }
        }

        op_assign_impl!($name, $name);

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> cmp::Ordering {
                self.get().cmp(&other.get())
            }
        }


        // $name op $type


        impl ops::Add<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: $type) -> Self {
                Self::new(self.get() + rhs)
            }
        }

        impl ops::BitAnd<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: $type) -> Self {
                Self::new(self.get() & rhs)
            }
        }

        impl ops::BitOr<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: $type) -> Self {
                Self::new(self.get() | rhs)
            }
        }

        impl ops::BitXor<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: $type) -> Self {
                Self::new(self.get() ^ rhs)
            }
        }

        impl ops::Div<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: $type) -> Self {
                Self::new(self.get() / rhs)
            }
        }

        impl ops::Mul<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: $type) -> Self {
                Self::new(self.get() * rhs)
            }
        }

        impl ops::Rem<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn rem(self, rhs: $type) -> Self {
                Self::new(self.get() % rhs)
            }
        }

        impl ops::Shl<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn shl(self, rhs: $type) -> Self {
                Self::new(self.get() << rhs)
            }
        }

        impl ops::Shr<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn shr(self, rhs: $type) -> Self {
                Self::new(self.get() >> rhs)
            }
        }

        impl ops::Sub<$type> for $name {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: $type) -> Self {
                Self::new(self.get() - rhs)
            }
        }

        op_assign_impl!($name, $type);

        impl PartialEq<$type> for $name {
            fn eq(&self, other: &$type) -> bool {
                self.get() == *other
            }
        }

        impl PartialOrd<$type> for $name {
            fn partial_cmp(&self, other: &$type) -> Option<cmp::Ordering> {
                Some(self.get().cmp(other))
            }
        }


        // $type op $name


        impl ops::Add<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: $name) -> Self {
                self + rhs.get()
            }
        }

        impl ops::BitAnd<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: $name) -> Self {
                self & rhs.get()
            }
        }

        impl ops::BitOr<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: $name) -> Self {
                self | rhs.get()
            }
        }

        impl ops::BitXor<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: $name) -> Self {
                self ^ rhs.get()
            }
        }

        impl ops::Div<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: $name) -> Self {
                self / rhs.get()
            }
        }

        impl ops::Mul<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: $name) -> Self {
                self * rhs.get()
            }
        }

        impl ops::Rem<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn rem(self, rhs: $name) -> Self {
                self % rhs.get()
            }
        }

        impl ops::Shl<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn shl(self, rhs: $name) -> Self {
                self << rhs.get()
            }
        }

        impl ops::Shr<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn shr(self, rhs: $name) -> Self {
                self >> rhs.get()
            }
        }

        impl ops::Sub<$name> for $type {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: $name) -> Self {
                self - rhs.get()
            }
        }

        op_assign_impl!($type, $name);

        impl PartialEq<$name> for $type {
            fn eq(&self, other: &$name) -> bool {
                *self == other.get()
            }
        }

        impl PartialOrd<$name> for $type {
            fn partial_cmp(&self, other: &$name) -> Option<cmp::Ordering> {
                Some(self.cmp(&other.get()))
            }
        }

    )*};
}

endian_ints! {
    from: from_le;
    to:   to_le;

    type LittleEndianU16   = u16_le   = u16;
    type LittleEndianU32   = u32_le   = u32;
    type LittleEndianU64   = u64_le   = u64;
    type LittleEndianUsize = usize_le = usize;
}

endian_ints! {
    signed;
    from: from_le;
    to:   to_le;

    type LittleEndianI16   = i16_le   = i16;
    type LittleEndianI32   = i32_le   = i32;
    type LittleEndianI64   = i64_le   = i64;
    type LittleEndianIsize = isize_le = isize;
}

endian_ints! {
    from: from_be;
    to:   to_be;

    type BigEndianU16   = u16_be   = u16;
    type BigEndianU32   = u32_be   = u32;
    type BigEndianU64   = u64_be   = u64;
    type BigEndianUsize = usize_be = usize;
}

endian_ints! {
    signed;
    from: from_be;
    to:   to_be;

    type BigEndianI16   = i16_be   = i16;
    type BigEndianI32   = i32_be   = i32;
    type BigEndianI64   = i64_be   = i64;
    type BigEndianIsize = isize_be = isize;
}
