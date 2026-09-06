//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/treeboot-ebony.c
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
// Old U-boot compatibility for Ebony
//
// Author: David Gibson <david@gibson.dropbear.id.au>
//
// Copyright 2007 David Gibson, IBM Corporatio.
// Based on cuboot-83xx.c, which is:
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//

    BSS_STACK(4096);
pub const OPENBIOS_MAC_BASE: c_uint = 0xfffffe0c;
pub const OPENBIOS_MAC_OFFSET: c_uint = 0xc;
#[no_mangle]
pub unsafe extern "C" fn platform_init() {
    void platform_init(void)
    {
    let mut end_of_ram: c_ulong = 0x8000000;
    let mut avail_ram: c_ulong = end_of_ram - (unsigned long)_end;
    simple_alloc_init(_end, avail_ram, 32, 64);
    ebony_init((u8 *)OPENBIOS_MAC_BASE,
    (u8 *)(OPENBIOS_MAC_BASE + OPENBIOS_MAC_OFFSET));
    }
