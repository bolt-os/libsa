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

use core::fmt;

macro_rules! endian_ints_common {
    ($name:ident, $type:ty) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd)]
        pub struct $name($type);

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <$type as fmt::Debug>::fmt(&self.get(), f)
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <$type as fmt::Display>::fmt(&self.get(), f)
            }
        }

        impl ::core::convert::From<$type> for $name {
            fn from(value: $type) -> $name {
                $name::new(value)
            }
        }

        impl ::core::convert::From<$name> for $type {
            fn from(value: $name) -> $type {
                value.get()
            }
        }
    };
}

macro_rules! little_endian_ints {
    ($(
        struct $name:ident = $type:ty;
    )*) => {$(

        endian_ints_common!($name, $type);

        impl $name {
            pub const fn new(value: $type) -> $name {
                Self(value.to_le())
            }

            pub const fn get(self) -> $type {
                <$type>::from_le(self.0)
            }

            pub fn set(&mut self, value: $type) {
                self.0 = value.to_le();
            }
        }

    )*}
}

macro_rules! big_endian_ints {
    ($(
        struct $name:ident = $type:ty;
    )*) => {$(

        endian_ints_common!($name, $type);

        impl $name {
            pub const fn new(value: $type) -> $name {
                Self(value.to_be())
            }

            pub const fn get(self) -> $type {
                <$type>::from_be(self.0)
            }

            pub fn set(&mut self, value: $type) {
                self.0 = value.to_be();
            }
        }
    )*}
}

little_endian_ints! {
    struct LittleEndianU16   = u16;
    struct LittleEndianU32   = u32;
    struct LittleEndianU64   = u64;
    struct LittleEndianUsize = usize;
    struct LittleEndianI16   = i16;
    struct LittleEndianI32   = i32;
    struct LittleEndianI64   = i64;
    struct LittleEndianIsize = isize;
}

big_endian_ints! {
    struct BigEndianU16   = u16;
    struct BigEndianU32   = u32;
    struct BigEndianU64   = u64;
    struct BigEndianUsize = usize;
    struct BigEndianI16   = i16;
    struct BigEndianI32   = i32;
    struct BigEndianI64   = i64;
    struct BigEndianIsize = isize;
}
