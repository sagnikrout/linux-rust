//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/treeboot-bamboo.c
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
// Copyright IBM Corporation, 2007
// Josh Boyer <jwboyer@linux.vnet.ibm.com>
//
// Based on ebony wrapper:
// Copyright 2007 David Gibson, IBM Corporation.
//

    BSS_STACK(4096);
pub const PIBS_MAC0: c_uint = 0xfffc0400;
pub const PIBS_MAC1: c_uint = 0xfffc0500;
    char pibs_mac0[6];
    char pibs_mac1[6];
#[no_mangle]
unsafe extern "C" fn read_pibs_mac() {
    static void read_pibs_mac(void)
    {
    unsigned long long mac64;
    mac64 = strtoull((char *)PIBS_MAC0, 0, 16);
    memcpy(&pibs_mac0, (char *)&mac64+2, 6);
    mac64 = strtoull((char *)PIBS_MAC1, 0, 16);
    memcpy(&pibs_mac1, (char *)&mac64+2, 6);
    }
#[no_mangle]
pub unsafe extern "C" fn platform_init() {
    void platform_init(void)
    {
    let mut end_of_ram: c_ulong = 0x8000000;
    let mut avail_ram: c_ulong = end_of_ram - (unsigned long)_end;
    simple_alloc_init(_end, avail_ram, 32, 64);
    read_pibs_mac();
    bamboo_init((u8 *)&pibs_mac0, (u8 *)&pibs_mac1);
    }
