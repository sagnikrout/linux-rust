//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/mpc85xx_ds.c
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
// MPC85xx DS Board Setup
//
// Author Xianghua Xiao (x.xiao@freescale.com)
// Roy Zang <tie-fei.zang@freescale.com>
// - Add PCI/PCI Exprees support
// Copyright 2007 Freescale Semiconductor Inc.
//

#[no_mangle]
unsafe extern "C" fn mpc85xx_ds_pic_init() -> void __init {
    static void __init mpc85xx_ds_pic_init(void)
    {
    struct mpic *mpic;
    let mut flags: c_int = MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU;
    if (of_machine_is_compatible("fsl,MPC8572DS-CAMP"))
    flags |= MPIC_NO_RESET;
    mpic = mpic_alloc(core::ptr::null_mut(), 0, flags, 0, 256, " OpenPIC  ");
    if (WARN_ON(!mpic))
    return;
    mpic_init(mpic);
    mpc85xx_8259_init();
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mpc85xx_ds_setup_arch() -> void __init {
    static void __init mpc85xx_ds_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("mpc85xx_ds_setup_arch()", 0);
    swiotlb_detect_4g();
    fsl_pci_assign_primary();
    uli_init();
    mpc85xx_smp_init();
    pr_info("MPC85xx DS board from Freescale Semiconductor\n");
    }
    machine_arch_initcall(mpc8544_ds, mpc85xx_common_publish_devices);
    machine_arch_initcall(mpc8572_ds, mpc85xx_common_publish_devices);
    define_machine(mpc8544_ds) {
    .name			= "MPC8544 DS",
    .compatible		= "MPC8544DS",
    .setup_arch		= mpc85xx_ds_setup_arch,
    .init_IRQ		= mpc85xx_ds_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
    define_machine(mpc8572_ds) {
    .name			= "MPC8572 DS",
    .compatible		= "fsl,MPC8572DS",
    .setup_arch		= mpc85xx_ds_setup_arch,
    .init_IRQ		= mpc85xx_ds_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
