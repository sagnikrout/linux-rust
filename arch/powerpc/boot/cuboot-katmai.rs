//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/cuboot-katmai.c
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
// Old U-boot compatibility for Katmai
//
// Author: Hugh Blemings <hugh@au.ibm.com>
//
// Copyright 2007 Hugh Blemings, IBM Corporation.
// Based on cuboot-ebony.c which is:
// Copyright 2007 David Gibson, IBM Corporation.
// Based on cuboot-83xx.c, which is:
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//

// Macro flag: #define TARGET_4xx
// Macro flag: #define TARGET_44x

    static bd_t bd;
    BSS_STACK(4096);
#[no_mangle]
unsafe extern "C" fn katmai_fixups() {
    static void katmai_fixups(void)
    {
    let mut sysclk: c_ulong = 33333000;
// 440SP Clock logic is all but identical to 440GX
// so we just use that code for now at least
//
    ibm440spe_fixup_clocks(sysclk, 6 * 1843200, 0);
    ibm440spe_fixup_memsize();
    dt_fixup_mac_address(0, bd.bi_enetaddr);
    ibm4xx_fixup_ebc_ranges("/plb/opb/ebc");
    }
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5,
    unsigned long r6, unsigned long r7)
    {
    CUBOOT_INIT();
    platform_ops.fixups = katmai_fixups;
    fdt_init(_dtb_start);
    serial_console_init();
    }
