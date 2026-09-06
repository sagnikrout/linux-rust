//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/tqm85xx.c
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
unsafe extern "C" fn tqm85xx_pic_init() -> void __init {
    static void __init tqm85xx_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0,
    MPIC_BIG_ENDIAN,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    mpc85xx_cpm2_pic_init();
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn tqm85xx_setup_arch() -> void __init {
    static void __init tqm85xx_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("tqm85xx_setup_arch()", 0);

    cpm2_reset();

    fsl_pci_assign_primary();
    }
#[no_mangle]
unsafe extern "C" fn tqm85xx_show_cpuinfo(m: *mut seq_file) {
    static void tqm85xx_show_cpuinfo(struct seq_file *m)
    {
    uint pvid, svid, phid1;
    pvid = mfspr(SPRN_PVR);
    svid = mfspr(SPRN_SVR);
    seq_printf(m, "Vendor\t\t: TQ Components\n");
    seq_printf(m, "PVR\t\t: 0x%x\n", pvid);
    seq_printf(m, "SVR\t\t: 0x%x\n", svid);
// Display cpu Pll setting
    phid1 = mfspr(SPRN_HID1);
    seq_printf(m, "PLL setting\t: 0x%x\n", ((phid1 >> 24) & 0x3f));
    }
#[no_mangle]
unsafe extern "C" fn tqm85xx_ti1520_fixup(pdev: *mut pci_dev) {
    static void tqm85xx_ti1520_fixup(struct pci_dev *pdev)
    {
    unsigned int val;
// Do not do the fixup on other platforms!
    if (!machine_is(tqm85xx))
    return;
    dev_info(&pdev.dev, "Using TI 1520 fixup on TQM85xx\n");
//
// Enable P2CCLK bit in system control register
// to enable CLOCK output to power chip
//
    pci_read_config_dword(pdev, 0x80, &val);
    pci_write_config_dword(pdev, 0x80, val | (1 << 27));
    }
    DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_1520,
    tqm85xx_ti1520_fixup);
    machine_arch_initcall(tqm85xx, mpc85xx_common_publish_devices);
    static const char * const board[] __initconst = {
    "tqc,tqm8540",
    "tqc,tqm8541",
    "tqc,tqm8548",
    "tqc,tqm8555",
    "tqc,tqm8560",
    core::ptr::null_mut()
    };
    define_machine(tqm85xx) {
    .name			= "TQM85xx",
    .compatibles		= board,
    .setup_arch		= tqm85xx_setup_arch,
    .init_IRQ		= tqm85xx_pic_init,
    .show_cpuinfo		= tqm85xx_show_cpuinfo,
    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
