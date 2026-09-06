//! Automatically rewritten from C to Rust
//! Source: lib/crypto/nh.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2018 Google LLC
//
// Implementation of the NH almost-universal hash function, specifically the
// variant of NH used in Adiantum.  This is *not* a cryptographic hash function.
//
// Reference: section 6.3 of "Adiantum: length-preserving encryption for
// entry-level processors" (https://eprint.iacr.org/2018/720.pdf).
//

    static bool nh_arch(const u32 *key, const u8 *message, size_t message_len,
    __le64 hash[NH_NUM_PASSES])
    {
    return false;
    }

    void nh(const u32 *key, const u8 *message, size_t message_len,
    __le64 hash[NH_NUM_PASSES])
    {
    u64 sums[4] = { 0, 0, 0, 0 };
    if (nh_arch(key, message, message_len, hash))
    return;
    static_assert(NH_PAIR_STRIDE == 2);
    static_assert(NH_NUM_PASSES == 4);
    while (message_len) {
    let mut m0: u32 = get_unaligned_le32(message + 0);
    let mut m1: u32 = get_unaligned_le32(message + 4);
    let mut m2: u32 = get_unaligned_le32(message + 8);
    let mut m3: u32 = get_unaligned_le32(message + 12);
    sums[0] += (u64)(u32)(m0 + key[0]) * (u32)(m2 + key[2]);
    sums[1] += (u64)(u32)(m0 + key[4]) * (u32)(m2 + key[6]);
    sums[2] += (u64)(u32)(m0 + key[8]) * (u32)(m2 + key[10]);
    sums[3] += (u64)(u32)(m0 + key[12]) * (u32)(m2 + key[14]);
    sums[0] += (u64)(u32)(m1 + key[1]) * (u32)(m3 + key[3]);
    sums[1] += (u64)(u32)(m1 + key[5]) * (u32)(m3 + key[7]);
    sums[2] += (u64)(u32)(m1 + key[9]) * (u32)(m3 + key[11]);
    sums[3] += (u64)(u32)(m1 + key[13]) * (u32)(m3 + key[15]);
    key += NH_MESSAGE_UNIT / sizeof(key[0]);
    message += NH_MESSAGE_UNIT;
    message_len -= NH_MESSAGE_UNIT;
    }
    hash[0] = cpu_to_le64(sums[0]);
    hash[1] = cpu_to_le64(sums[1]);
    hash[2] = cpu_to_le64(sums[2]);
    hash[3] = cpu_to_le64(sums[3]);
    }
    EXPORT_SYMBOL_GPL(nh);

#[no_mangle]
unsafe extern "C" fn nh_mod_init() -> int __init {
    static int __init nh_mod_init(void)
    {
    nh_mod_init_arch();
    return 0;
    }
    subsys_initcall(nh_mod_init);
#[no_mangle]
unsafe extern "C" fn nh_mod_exit() -> void __exit {
    static void __exit nh_mod_exit(void)
    {
    }
    module_exit(nh_mod_exit);

    MODULE_DESCRIPTION("NH almost-universal hash function");
    MODULE_LICENSE("GPL");
