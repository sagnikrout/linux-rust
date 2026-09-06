//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/mvme2500.c
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
// Board setup routines for the Emerson/Artesyn MVME2500
//
// Copyright 2014 Elettra-Sincrotrone Trieste S.C.p.A.
//
// Based on earlier code by:
//
// Xianghua Xiao (x.xiao@freescale.com)
// Tom Armistead (tom.armistead@emerson.com)
// Copyright 2012 Emerson
//
// Author Alessio Igor Bogani <alessio.bogani@elettra.eu>
//

#[no_mangle]
unsafe extern "C" fn mvme2500_pic_init() -> void __init {
    static void __init mvme2500_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0,
    MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mvme2500_setup_arch() -> void __init {
    static void __init mvme2500_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("mvme2500_setup_arch()", 0);
    fsl_pci_assign_primary();
    pr_info("MVME2500 board from Artesyn\n");
    }
    machine_arch_initcall(mvme2500, mpc85xx_common_publish_devices);
    define_machine(mvme2500) {
    .name			= "MVME2500",
    .compatible		= "artesyn,MVME2500",
    .setup_arch		= mvme2500_setup_arch,
    .init_IRQ		= mvme2500_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
