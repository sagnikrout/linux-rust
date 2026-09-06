//! Automatically rewritten from C to Rust
//! Source: lib/crc/crc4.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only
//
// crc4.c - simple crc-4 calculations.
//

    static const uint8_t crc4_tab[] = {
    0x0, 0x7, 0xe, 0x9, 0xb, 0xc, 0x5, 0x2,
    0x1, 0x6, 0xf, 0x8, 0xa, 0xd, 0x4, 0x3,
    };
//
// crc4 - calculate the 4-bit crc of a value.
// @c:    starting crc4
// @x:    value to checksum
// @bits: number of bits in @x to checksum
//
// Returns the crc4 value of @x, using polynomial 0b10111.
//
// The @x value is treated as left-aligned, and bits above @bits are ignored
// in the crc calculations.
//
#[no_mangle]
pub unsafe extern "C" fn crc4(c: u8, x: u64, bits: c_int) -> u8 {
    uint8_t crc4(uint8_t c, uint64_t x, int bits)
    {
    int i;
// mask off anything above the top bit
    x &= (1ull << bits) - 1;
// Align to 4-bits
    bits = (bits + 3) & ~0x3;
// Calculate crc4 over four-bit nibbles, starting at the MSbit
    for (i = bits - 4; i >= 0; i -= 4)
    c = crc4_tab[c ^ ((x >> i) & 0xf)];
    return c;
    }
    EXPORT_SYMBOL_GPL(crc4);
    MODULE_DESCRIPTION("CRC4 calculations");
    MODULE_LICENSE("GPL");
