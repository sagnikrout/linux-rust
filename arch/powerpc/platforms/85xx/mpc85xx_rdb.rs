//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/mpc85xx_rdb.c
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
// MPC85xx RDB Board Setup
//
// Copyright 2009,2012-2013 Freescale Semiconductor Inc.
//

#[no_mangle]
unsafe extern "C" fn mpc85xx_rdb_pic_init() -> void __init {
    static void __init mpc85xx_rdb_pic_init(void)
    {
    struct mpic *mpic;
    let mut flags: c_int = MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU;
    if (of_machine_is_compatible("fsl,MPC85XXRDB-CAMP"))
    flags |= MPIC_NO_RESET;
    mpic = mpic_alloc(core::ptr::null_mut(), 0, flags, 0, 256, " OpenPIC  ");
    if (WARN_ON(!mpic))
    return;
    mpic_init(mpic);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mpc85xx_rdb_setup_arch() -> void __init {
    static void __init mpc85xx_rdb_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("mpc85xx_rdb_setup_arch()", 0);
    mpc85xx_smp_init();
    fsl_pci_assign_primary();
    mpc85xx_qe_par_io_init();

    if (machine_is(p1025_rdb)) {
    struct device_node *np;
    struct ccsr_guts __iomem *guts;
    np = of_find_node_by_name(core::ptr::null_mut(), "global-utilities");
    if (np) {
    guts = of_iomap(np, 0);
    if (!guts) {
    pr_err("mpc85xx-rdb: could not map global utilities register\n");
    } else {
// P1025 has pins muxed for QE and other functions. To
// enable QE UEC mode, we need to set bit QE0 for UCC1
// in Eth mode, QE0 and QE3 for UCC5 in Eth mode, QE9
// and QE12 for QE MII management signals in PMUXCR
// register.
//
    setbits32(&guts.pmuxcr, MPC85xx_PMUXCR_QE(0) |
    MPC85xx_PMUXCR_QE(3) |
    MPC85xx_PMUXCR_QE(9) |
    MPC85xx_PMUXCR_QE(12));
    iounmap(guts);
    }
    of_node_put(np);
    }
    }

    pr_info("MPC85xx RDB board from Freescale Semiconductor\n");
    }
    machine_arch_initcall(p1020_mbg_pc, mpc85xx_common_publish_devices);
    machine_arch_initcall(p1020_rdb, mpc85xx_common_publish_devices);
    machine_arch_initcall(p1020_rdb_pc, mpc85xx_common_publish_devices);
    machine_arch_initcall(p1020_rdb_pd, mpc85xx_common_publish_devices);
    machine_arch_initcall(p1020_utm_pc, mpc85xx_common_publish_devices);
    machine_arch_initcall(p1021_rdb_pc, mpc85xx_common_publish_devices);
    machine_arch_initcall(p1025_rdb, mpc85xx_common_publish_devices);
    machine_arch_initcall(p1024_rdb, mpc85xx_common_publish_devices);
    define_machine(p1020_rdb) {
    .name			= "P1020 RDB",
    .compatible		= "fsl,P1020RDB",
    .setup_arch		= mpc85xx_rdb_setup_arch,
    .init_IRQ		= mpc85xx_rdb_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
    define_machine(p1021_rdb_pc) {
    .name			= "P1021 RDB-PC",
    .compatible		= "fsl,P1021RDB-PC",
    .setup_arch		= mpc85xx_rdb_setup_arch,
    .init_IRQ		= mpc85xx_rdb_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
    define_machine(p1025_rdb) {
    .name			= "P1025 RDB",
    .compatible		= "fsl,P1025RDB",
    .setup_arch		= mpc85xx_rdb_setup_arch,
    .init_IRQ		= mpc85xx_rdb_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
    define_machine(p1020_mbg_pc) {
    .name			= "P1020 MBG-PC",
    .compatible		= "fsl,P1020MBG-PC",
    .setup_arch		= mpc85xx_rdb_setup_arch,
    .init_IRQ		= mpc85xx_rdb_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
    define_machine(p1020_utm_pc) {
    .name			= "P1020 UTM-PC",
    .compatible		= "fsl,P1020UTM-PC",
    .setup_arch		= mpc85xx_rdb_setup_arch,
    .init_IRQ		= mpc85xx_rdb_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
    define_machine(p1020_rdb_pc) {
    .name			= "P1020RDB-PC",
    .compatible		= "fsl,P1020RDB-PC",
    .setup_arch		= mpc85xx_rdb_setup_arch,
    .init_IRQ		= mpc85xx_rdb_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
    define_machine(p1020_rdb_pd) {
    .name			= "P1020RDB-PD",
    .compatible		= "fsl,P1020RDB-PD",
    .setup_arch		= mpc85xx_rdb_setup_arch,
    .init_IRQ		= mpc85xx_rdb_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
    define_machine(p1024_rdb) {
    .name			= "P1024 RDB",
    .compatible		= "fsl,P1024RDB",
    .setup_arch		= mpc85xx_rdb_setup_arch,
    .init_IRQ		= mpc85xx_rdb_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
