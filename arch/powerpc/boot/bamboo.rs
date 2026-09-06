//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/bamboo.c
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
// Clocking code based on code by:
// Stefan Roese <sr@denx.de>
//

    static u8 *bamboo_mac0, *bamboo_mac1;
#[no_mangle]
unsafe extern "C" fn bamboo_fixups() {
    static void bamboo_fixups(void)
    {
    let mut sysclk: c_ulong = 33333333;
    ibm440ep_fixup_clocks(sysclk, 11059200, 25000000);
    ibm4xx_sdram_fixup_memsize();
    ibm4xx_quiesce_eth((u32 *)0xef600e00, (u32 *)0xef600f00);
    dt_fixup_mac_address_by_alias("ethernet0", bamboo_mac0);
    dt_fixup_mac_address_by_alias("ethernet1", bamboo_mac1);
    }
#[no_mangle]
pub unsafe extern "C" fn bamboo_init(mac0: *mut c_void, mac1: *mut c_void) {
    void bamboo_init(void *mac0, void *mac1)
    {
    platform_ops.fixups = bamboo_fixups;
    platform_ops.exit = ibm44x_dbcr_reset;
    bamboo_mac0 = mac0;
    bamboo_mac1 = mac1;
    fdt_init(_dtb_start);
    serial_console_init();
    }
