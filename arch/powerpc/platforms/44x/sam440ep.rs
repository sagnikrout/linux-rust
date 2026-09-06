//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/sam440ep.c
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
// Sam440ep board specific routines based off bamboo.c code
// original copyrights below
//
// Wade Farnsworth <wfarnsworth@mvista.com>
// Copyright 2004 MontaVista Software Inc.
//
// Rewritten and ported to the merged powerpc tree:
// Josh Boyer <jwboyer@linux.vnet.ibm.com>
// Copyright 2007 IBM Corporation
//
// Modified from bamboo.c for sam440ep:
// Copyright 2008 Giuseppe Coviello <gicoviello@gmail.com>
//

    static const struct of_device_id sam440ep_of_bus[] __initconst = {
    { .compatible = "ibm,plb4", },
    { .compatible = "ibm,opb", },
    { .compatible = "ibm,ebc", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn sam440ep_device_probe() -> int __init {
    static int __init sam440ep_device_probe(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), sam440ep_of_bus, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(sam440ep, sam440ep_device_probe);
#[no_mangle]
unsafe extern "C" fn sam440ep_probe() -> int __init {
    static int __init sam440ep_probe(void)
    {
    pci_set_flags(PCI_REASSIGN_ALL_RSRC);
    return 1;
    }
    define_machine(sam440ep) {
    .name 			= "Sam440ep",
    .compatible		= "acube,sam440ep",
    .probe 			= sam440ep_probe,
    .progress 		= udbg_progress,
    .init_IRQ 		= uic_init_tree,
    .get_irq 		= uic_get_irq,
    .restart		= ppc4xx_reset_system,
    };
    static struct i2c_board_info sam440ep_rtc_info = {
    .type = "m41st85",
    .addr = 0x68,
    .irq = -1,
    };
#[no_mangle]
unsafe extern "C" fn sam440ep_setup_rtc() -> int __init {
    static int __init sam440ep_setup_rtc(void)
    {
    return i2c_register_board_info(0, &sam440ep_rtc_info, 1);
    }
    machine_device_initcall(sam440ep, sam440ep_setup_rtc);
