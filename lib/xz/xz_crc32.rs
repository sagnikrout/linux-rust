//! Automatically rewritten from C to Rust
//! Source: lib/xz/xz_crc32.c
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


// SPDX-License-Identifier: 0BSD
//
// CRC32 using the polynomial from IEEE-802.3
//
// Authors: Lasse Collin <lasse.collin@tukaani.org>
// Igor Pavlov <https://7-zip.org/>
//
// This is not the fastest implementation, but it is pretty compact.
// The fastest versions of xz_crc32() on modern CPUs without hardware
// accelerated CRC instruction are 3-5 times as fast as this version,
// but they are bigger and use more memory for the lookup table.
//

//
// STATIC_RW_DATA is used in the pre-boot environment on some architectures.
// See <linux/decompress/mm.h> for details.
//

    STATIC_RW_DATA uint32_t xz_crc32_table[256];
#[no_mangle]
pub unsafe extern "C" fn xz_crc32_init() {
    void xz_crc32_init(void)
    {
    let mut poly: u32 = 0xEDB88320;
    uint32_t i;
    uint32_t j;
    uint32_t r;
    for (i = 0; i < 256; ++i) {
    r = i;
    for (j = 0; j < 8; ++j)
    r = (r >> 1) ^ (poly & ~((r & 1) - 1));
    xz_crc32_table[i] = r;
    }
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn xz_crc32(buf: *const u8, size: usize, crc: u32) -> u32 {
    uint32_t xz_crc32(const uint8_t *buf, size_t size, uint32_t crc)
    {
    crc = ~crc;
    while (size != 0) {
    crc = xz_crc32_table[*buf++ ^ (crc & 0xFF)] ^ (crc >> 8);
    --size;
    }
    return ~crc;
    }
