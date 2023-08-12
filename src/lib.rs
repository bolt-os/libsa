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

//! Common utilities for low-level, `#![no_std]` environments

#![warn(clippy::cargo, clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![deny(
    clippy::semicolon_if_nothing_returned,
    clippy::debug_assert_with_mut_call
)]
#![allow(
    clippy::cast_lossless,
    clippy::enum_glob_use,
    clippy::inline_always,
    clippy::items_after_statements,
    clippy::must_use_candidate,
    clippy::unreadable_literal,
    clippy::wildcard_imports
)]

#![cfg_attr(not(test), no_std)]

#[cfg(feature = "endian")]
pub mod endian;
#[cfg(feature = "volatile")]
pub mod volatile;

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_export]
macro_rules! extern_sym {
    ($sym:ident) => { $crate::extern_sym!($sym as ()) };
    (mut $sym:ident) => { $crate::extern_sym!(mut $sym as ()) };
    ($sym:ident as $t:ty) => {{
        #[allow(improper_ctypes)]
        extern "C" { static $sym: $t; }
        // SAFETY: The value is not accessed, we only take its address.
        // The `addr_of!()` macro ensures that no intermediate reference is created.
        unsafe {
            ::core::ptr::addr_of!($sym)
        }
    }};
    (mut $sym:ident as $t:ty) => {{
        #[allow(improper_ctypes)]
        extern "C" { static $sym: $t; }
        // SAFETY: The value is not accessed, we only take its address.
        // The `addr_of_mut!()` macro ensures that no intermediate reference is created.
        unsafe {
            ::core::ptr::addr_of_mut!($sym)
        }
    }};
}
