//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/c293pcie.c
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
// C293PCIE Board Setup
//
// Copyright 2013 Freescale Semiconductor Inc.
//

#[no_mangle]
unsafe extern "C" fn c293_pcie_pic_init() -> void __init {
    static void __init c293_pcie_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN |
    MPIC_SINGLE_DEST_CPU, 0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn c293_pcie_setup_arch() -> void __init {
    static void __init c293_pcie_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("c293_pcie_setup_arch()", 0);
    fsl_pci_assign_primary();
    printk(KERN_INFO "C293 PCIE board from Freescale Semiconductor\n");
    }
    machine_arch_initcall(c293_pcie, mpc85xx_common_publish_devices);
    define_machine(c293_pcie) {
    .name			= "C293 PCIE",
    .compatible		= "fsl,C293PCIE",
    .setup_arch		= c293_pcie_setup_arch,
    .init_IRQ		= c293_pcie_pic_init,
    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
