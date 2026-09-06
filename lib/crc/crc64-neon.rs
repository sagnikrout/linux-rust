//! Automatically rewritten from C to Rust
//! Source: lib/crc/crc64-neon.c
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
// Accelerated CRC64 (NVMe) using ARM NEON C intrinsics
//

    u64 crc64_nvme_neon(u64 crc, const u8 *p, size_t len);
// x^191 mod G, x^127 mod G
    static const u64 fold_consts_val[2] = { 0xeadc41fd2ba3d420ULL,
    0x21e9761e252621acULL };
// floor(x^127 / G), (G - x^64) / x
    static const u64 bconsts_val[2] = { 0x27ecfa329aef9f77ULL,
    0x34d926535897936aULL };
#[no_mangle]
pub unsafe extern "C" fn crc64_nvme_neon(crc: u64, p: *const u8, len: usize) -> u64 {
    u64 crc64_nvme_neon(u64 crc, const u8 *p, size_t len)
    {
    let mut fold_consts: uint64x2_t = vld1q_u64(fold_consts_val);
    let mut v0: uint64x2_t = { crc, 0 };
    let mut zero: uint64x2_t = { };
    for (;;) {
    v0 ^= vreinterpretq_u64_u8(vld1q_u8(p));
    p += 16;
    len -= 16;
    if (len < 16)
    break;
    v0 = pmull64(fold_consts, v0) ^ pmull64_high(fold_consts, v0);
    }
// Multiply the 128-bit value by x^64 and reduce it back to 128 bits.
    v0 = vextq_u64(v0, zero, 1) ^ pmull64_hi_lo(fold_consts, v0);
// Final Barrett reduction
    let mut bconsts: uint64x2_t = vld1q_u64(bconsts_val);
    let mut final: uint64x2_t = pmull64(bconsts, v0);
    v0 ^= vextq_u64(zero, final, 1) ^ pmull64_hi_lo(bconsts, final);
    return vgetq_lane_u64(v0, 1);
    }
