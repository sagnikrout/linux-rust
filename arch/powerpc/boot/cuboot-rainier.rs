//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/cuboot-rainier.c
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
// Old U-boot compatibility for Rainier
//
// Valentine Barshak <vbarshak@ru.mvista.com>
// Copyright 2007 MontaVista Software, Inc
//
// Based on Ebony code by David Gibson <david@gibson.dropbear.id.au>
// Copyright IBM Corporation, 2007
//
// Based on Bamboo code by Josh Boyer <jwboyer@linux.vnet.ibm.com>
// Copyright IBM Corporation, 2007
//

// Macro flag: #define TARGET_4xx
// Macro flag: #define TARGET_44x

    static bd_t bd;
#[no_mangle]
unsafe extern "C" fn rainier_fixups() {
    static void rainier_fixups(void)
    {
    let mut sysclk: c_ulong = 33333333;
    ibm440ep_fixup_clocks(sysclk, 11059200, 50000000);
    ibm4xx_fixup_ebc_ranges("/plb/opb/ebc");
    ibm4xx_denali_fixup_memsize();
    dt_fixup_mac_address_by_alias("ethernet0", bd.bi_enetaddr);
    dt_fixup_mac_address_by_alias("ethernet1", bd.bi_enet1addr);
    }
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5,
    unsigned long r6, unsigned long r7)
    {
    CUBOOT_INIT();
    platform_ops.fixups = rainier_fixups;
    platform_ops.exit = ibm44x_dbcr_reset;
    fdt_init(_dtb_start);
    serial_console_init();
    }
