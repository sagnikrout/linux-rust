//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/cuboot-sam440ep.c
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
// Old U-boot compatibility for Sam440ep based off bamboo.c code
// original copyrights below
//
// Author: Josh Boyer <jwboyer@linux.vnet.ibm.com>
//
// Copyright 2007 IBM Corporation
//
// Based on cuboot-ebony.c
//
// Modified from cuboot-bamboo.c for sam440ep:
// Copyright 2008 Giuseppe Coviello <gicoviello@gmail.com>
//

// Macro flag: #define TARGET_4xx
// Macro flag: #define TARGET_44x

    static bd_t bd;
#[no_mangle]
unsafe extern "C" fn sam440ep_fixups() {
    static void sam440ep_fixups(void)
    {
    let mut sysclk: c_ulong = 66666666;
    ibm440ep_fixup_clocks(sysclk, 11059200, 25000000);
    ibm4xx_sdram_fixup_memsize();
    ibm4xx_quiesce_eth((u32 *)0xef600e00, (u32 *)0xef600f00);
    dt_fixup_mac_addresses(&bd.bi_enetaddr, &bd.bi_enet1addr);
    }
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5,
    unsigned long r6, unsigned long r7)
    {
    CUBOOT_INIT();
    platform_ops.fixups = sam440ep_fixups;
    platform_ops.exit = ibm44x_dbcr_reset;
    fdt_init(_dtb_start);
    serial_console_init();
    }
