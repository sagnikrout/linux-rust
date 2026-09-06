//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/ebony.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2007 David Gibson, IBM Corporation.
//
// Based on earlier code:
// Copyright (C) Paul Mackerras 1997.
//
// Matt Porter <mporter@kernel.crashing.org>
// Copyright 2002-2005 MontaVista Software Inc.
//
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
// Copyright (c) 2003, 2004 Zultys Technologies
//

    static u8 *ebony_mac0, *ebony_mac1;

pub const EBONY_FPGA_FLASH_SEL: c_uint = 0x01;

#[no_mangle]
unsafe extern "C" fn ebony_flashsel_fixup() {
    static void ebony_flashsel_fixup(void)
    {
    void *devp;
    u32 reg[3] = {0x0, 0x0, 0x80000};
    u8 *fpga;
    let mut fpga_reg0: u8 = 0x0;
    devp = finddevice(EBONY_FPGA_PATH);
    if (!devp)
    fatal("Couldn't locate FPGA node %s\n\r", EBONY_FPGA_PATH);
    if (getprop(devp, "virtual-reg", &fpga, sizeof(fpga)) != sizeof(fpga))
    fatal("%s has missing or invalid virtual-reg property\n\r",
    EBONY_FPGA_PATH);
    fpga_reg0 = in_8(fpga);
    devp = finddevice(EBONY_SMALL_FLASH_PATH);
    if (!devp)
    fatal("Couldn't locate small flash node %s\n\r",
    EBONY_SMALL_FLASH_PATH);
    if (getprop(devp, "reg", reg, sizeof(reg)) != sizeof(reg))
    fatal("%s has reg property of unexpected size\n\r",
    EBONY_SMALL_FLASH_PATH);
// Invert address bit 14 (IBM-endian) if FLASH_SEL fpga bit is set
    if (fpga_reg0 & EBONY_FPGA_FLASH_SEL)
    reg[1] ^= 0x80000;
    setprop(devp, "reg", reg, sizeof(reg));
    }
#[no_mangle]
unsafe extern "C" fn ebony_fixups() {
    static void ebony_fixups(void)
    {
// FIXME: sysclk should be derived by reading the FPGA registers
    let mut sysclk: c_ulong = 33000000;
    ibm440gp_fixup_clocks(sysclk, 6 * 1843200);
    ibm4xx_sdram_fixup_memsize();
    dt_fixup_mac_address_by_alias("ethernet0", ebony_mac0);
    dt_fixup_mac_address_by_alias("ethernet1", ebony_mac1);
    ibm4xx_fixup_ebc_ranges("/plb/opb/ebc");
    ebony_flashsel_fixup();
    }
#[no_mangle]
pub unsafe extern "C" fn ebony_init(mac0: *mut c_void, mac1: *mut c_void) {
    void ebony_init(void *mac0, void *mac1)
    {
    platform_ops.fixups = ebony_fixups;
    platform_ops.exit = ibm44x_dbcr_reset;
    ebony_mac0 = mac0;
    ebony_mac1 = mac1;
    fdt_init(_dtb_start);
    serial_console_init();
    }
