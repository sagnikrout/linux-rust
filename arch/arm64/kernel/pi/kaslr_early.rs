//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/pi/kaslr_early.c
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
// Copyright 2022 Google LLC
// Author: Ard Biesheuvel <ardb@google.com>
// NOTE: code in this file runs *very* early, and is not permitted to use
// global variables or anything that relies on absolute addressing.

#[no_mangle]
unsafe extern "C" fn get_kaslr_seed(fdt: *mut c_void, node: c_int) -> u64 __init {
    static u64 __init get_kaslr_seed(void *fdt, int node)
    {
    static char const seed_str[] __initconst = "kaslr-seed";
    fdt64_t *prop;
    u64 ret;
    int len;
    if (node < 0)
    return 0;
    prop = fdt_getprop_w(fdt, node, seed_str, &len);
    if (!prop || len != sizeof(u64))
    return 0;
    ret = fdt64_to_cpu(*prop);
// prop = 0;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kaslr_early_init(fdt: *mut c_void, chosen: c_int) -> u64 __init {
    u64 __init kaslr_early_init(void *fdt, int chosen)
    {
    u64 seed, range;
    if (kaslr_disabled_cmdline())
    return 0;
    seed = get_kaslr_seed(fdt, chosen);
    if (!seed) {
    if (!__early_cpu_has_rndr() ||
    !__arm64_rndr((unsigned long *)&seed))
    return 0;
    }
//
// OK, so we are proceeding with KASLR enabled. Calculate a suitable
// kernel image offset from the seed. Let's place the kernel in the
// 'middle' half of the VMALLOC area, and stay clear of the lower and
// upper quarters to avoid colliding with other allocations.
//
    range = (VMALLOC_END - KIMAGE_VADDR) / 2;
    return range / 2 + (((__uint128_t)range * seed) >> 64);
    }
