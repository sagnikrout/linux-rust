//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/twr_p102x.c
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
// Copyright 2010-2011, 2013 Freescale Semiconductor, Inc.
//
// Author: Michael Johnston <michael.johnston@freescale.com>
//
// Description:
// TWR-P102x Board Setup
//

#[no_mangle]
unsafe extern "C" fn twr_p1025_pic_init() -> void __init {
    static void __init twr_p1025_pic_init(void)
    {
    struct mpic *mpic;
    mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN |
    MPIC_SINGLE_DEST_CPU,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn twr_p1025_setup_arch() -> void __init {
    static void __init twr_p1025_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("twr_p1025_setup_arch()", 0);
    mpc85xx_smp_init();
    fsl_pci_assign_primary();

    mpc85xx_qe_par_io_init();

    if (machine_is(twr_p1025)) {
    struct ccsr_guts __iomem *guts;
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,p1021-guts");
    if (np) {
    guts = of_iomap(np, 0);
    if (!guts)
    pr_err("twr_p1025: could not map global utilities register\n");
    else {
// P1025 has pins muxed for QE and other functions. To
// enable QE UEC mode, we need to set bit QE0 for UCC1
// in Eth mode, QE0 and QE3 for UCC5 in Eth mode, QE9
// and QE12 for QE MII management signals in PMUXCR
// register.
// Set QE mux bits in PMUXCR
    setbits32(&guts.pmuxcr, MPC85xx_PMUXCR_QE(0) |
    MPC85xx_PMUXCR_QE(3) |
    MPC85xx_PMUXCR_QE(9) |
    MPC85xx_PMUXCR_QE(12));
    iounmap(guts);

// On P1025TWR board, the UCC7 acted as UART port.
// However, The UCC7's CTS pin is low level in default,
// it will impact the transmission in full duplex
// communication. So disable the Flow control pin PA18.
// The UCC7 UART just can use RXD and TXD pins.
//
    par_io_config_pin(0, 18, 0, 0, 0, 0);

// Drive PB29 to CPLD low - CPLD will then change
// muxing from LBC to QE
    par_io_config_pin(1, 29, 1, 0, 0, 0);
    par_io_data_set(1, 29, 0);
    }
    of_node_put(np);
    }
    }

    pr_info("TWR-P1025 board from Freescale Semiconductor\n");
    }
    machine_arch_initcall(twr_p1025, mpc85xx_common_publish_devices);
    define_machine(twr_p1025) {
    .name			= "TWR-P1025",
    .compatible		= "fsl,TWR-P1025",
    .setup_arch		= twr_p1025_setup_arch,
    .init_IRQ		= twr_p1025_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
