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

use core::cell::UnsafeCell;

/// A wrapper type providing volatile access to a value
#[repr(transparent)]
pub struct Volatile<T> {
    value: UnsafeCell<T>,
}

impl<T> Volatile<T> {
    #[inline]
    pub const fn new(value: T) -> Volatile<T> {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn read(&self) -> T {
        // SAFETY: `Volatile` owns the data it stores.
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    #[inline]
    pub fn write(&self, value: T) {
        // SAFETY: `Volatile` owns the data it stores.
        unsafe { core::ptr::write_volatile(self.value.get(), value) };
    }
}

/// Represents a 64 bit pointer, of which its low and high bits are split in 32-bit-aligned
/// volatile memory.
// TODO: endian memes
#[repr(C)]
#[cfg(target_pointer_width = "64")]
#[allow(clippy::module_name_repetitions)]
pub struct VolatileSplitPtr<T> {
    low: Volatile<u32>,
    high: Volatile<u32>,
    marker: core::marker::PhantomData<T>,
}

impl<T> VolatileSplitPtr<T> {
    /// Gets the contained pointer
    #[inline]
    pub fn get(&self) -> *mut T {
        let low = self.low.read();
        let high = self.high.read();

        (((high as u64) << 32) | (low as u64)) as *mut T
    }

    /// Sets the contained pointer
    #[inline]
    pub fn set(&self, ptr: *mut T) {
        #![allow(clippy::cast_possible_truncation)]
        let ptr = ptr as usize;

        self.low.write(ptr as u32);
        self.high.write((ptr >> 32) as u32);
    }

    /// Sets the component parts of the contained pointer
    #[inline]
    pub fn set_parts(&self, high: u32, low: u32) {
        self.low.write(low);
        self.high.write(high);
    }
}
