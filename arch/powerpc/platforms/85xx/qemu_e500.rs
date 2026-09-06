//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/qemu_e500.c
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
// Paravirt target for a generic QEMU e500 machine
//
// This is intended to be a flexible device-tree-driven platform, not fixed
// to a particular piece of hardware or a particular spec of virtual hardware,
// beyond the assumption of an e500-family CPU.  Some things are still hardcoded
// here, such as MPIC, but this is a limitation of the current code rather than
// an interface contract with QEMU.
//
// Copyright 2012 Freescale Semiconductor Inc.
//

#[no_mangle]
unsafe extern "C" fn qemu_e500_pic_init() -> void __init {
    static void __init qemu_e500_pic_init(void)
    {
    struct mpic *mpic;
    unsigned int flags = MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU |
    MPIC_ENABLE_COREINT;
    mpic = mpic_alloc(core::ptr::null_mut(), 0, flags, 0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    }
#[no_mangle]
unsafe extern "C" fn qemu_e500_setup_arch() -> void __init {
    static void __init qemu_e500_setup_arch(void)
    {
    ppc_md.progress("qemu_e500_setup_arch()", 0);
    fsl_pci_assign_primary();
    swiotlb_detect_4g();
    mpc85xx_smp_init();
    }
    machine_arch_initcall(qemu_e500, mpc85xx_common_publish_devices);
    define_machine(qemu_e500) {
    .name			= "QEMU e500",
    .compatible		= "fsl,qemu-e500",
    .setup_arch		= qemu_e500_setup_arch,
    .init_IRQ		= qemu_e500_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_coreint_irq,
    .progress		= udbg_progress,
    .power_save		= e500_idle,
    };
