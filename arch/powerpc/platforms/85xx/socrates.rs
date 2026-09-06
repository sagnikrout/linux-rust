//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/socrates.c
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
// Copyright (c) 2008 Emcraft Systems
// Sergei Poselenov <sposelenov@emcraft.com>
//
// Based on MPC8560 ADS and arch/ppc tqm85xx ports
//
// Maintained by Kumar Gala (see MAINTAINERS for contact information)
//
// Copyright 2008 Freescale Semiconductor Inc.
//
// Copyright (c) 2005-2006 DENX Software Engineering
// Stefan Roese <sr@denx.de>
//
// Based on original work by
// Kumar Gala <kumar.gala@freescale.com>
// Copyright 2004 Freescale Semiconductor Inc.
//

#[no_mangle]
unsafe extern "C" fn socrates_pic_init() -> void __init {
    static void __init socrates_pic_init(void)
    {
    struct device_node *np;
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "abb,socrates-fpga-pic");
    if (!np) {
    printk(KERN_ERR "Could not find socrates-fpga-pic node\n");
    return;
    }
    socrates_fpga_pic_init(np);
    of_node_put(np);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn socrates_setup_arch() -> void __init {
    static void __init socrates_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("socrates_setup_arch()", 0);
    fsl_pci_assign_primary();
    }
    machine_arch_initcall(socrates, mpc85xx_common_publish_devices);
    define_machine(socrates) {
    .name			= "Socrates",
    .compatible		= "abb,socrates",
    .setup_arch		= socrates_setup_arch,
    .init_IRQ		= socrates_pic_init,
    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
