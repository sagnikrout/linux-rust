//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/86xx/gef_sbc610.c
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
// GE SBC610 board support
//
// Author: Martyn Welch <martyn.welch@ge.com>
//
// Copyright 2008 GE Intelligent Platforms Embedded Systems, Inc.
//
// Based on: mpc86xx_hpcn.c (MPC86xx HPCN board specific routines)
// Copyright 2006 Freescale Semiconductor Inc.
//
// NEC fixup adapted from arch/mips/pci/fixup-lm2e.c
//

    void __iomem *sbc610_regs;
#[no_mangle]
unsafe extern "C" fn gef_sbc610_init_irq() -> void __init {
    static void __init gef_sbc610_init_irq(void)
    {
    struct device_node *cascade_node = core::ptr::null_mut();
    mpc86xx_init_irq();
//
// There is a simple interrupt handler in the main FPGA, this needs
// to be cascaded into the MPIC
//
    cascade_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "gef,fpga-pic");
    if (!cascade_node) {
    printk(KERN_WARNING "SBC610: No FPGA PIC\n");
    return;
    }
    gef_pic_init(cascade_node);
    of_node_put(cascade_node);
    }
#[no_mangle]
unsafe extern "C" fn gef_sbc610_setup_arch() -> void __init {
    static void __init gef_sbc610_setup_arch(void)
    {
    struct device_node *regs;
    printk(KERN_INFO "GE Intelligent Platforms SBC610 6U VPX SBC\n");

    mpc86xx_smp_init();

    fsl_pci_assign_primary();
// Remap basic board registers
    regs = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "gef,fpga-regs");
    if (regs) {
    sbc610_regs = of_iomap(regs, 0);
    if (sbc610_regs == core::ptr::null_mut())
    printk(KERN_WARNING "Unable to map board registers\n");
    of_node_put(regs);
    }

    mmio_nvram_init();

    }
// Return the PCB revision
#[no_mangle]
unsafe extern "C" fn gef_sbc610_get_pcb_rev() -> c_uint {
    static unsigned int gef_sbc610_get_pcb_rev(void)
    {
    unsigned int reg;
    reg = ioread32(sbc610_regs);
    return (reg >> 8) & 0xff;
    }
// Return the board (software) revision
#[no_mangle]
unsafe extern "C" fn gef_sbc610_get_board_rev() -> c_uint {
    static unsigned int gef_sbc610_get_board_rev(void)
    {
    unsigned int reg;
    reg = ioread32(sbc610_regs);
    return (reg >> 16) & 0xff;
    }
// Return the FPGA revision
#[no_mangle]
unsafe extern "C" fn gef_sbc610_get_fpga_rev() -> c_uint {
    static unsigned int gef_sbc610_get_fpga_rev(void)
    {
    unsigned int reg;
    reg = ioread32(sbc610_regs);
    return (reg >> 24) & 0xf;
    }
#[no_mangle]
unsafe extern "C" fn gef_sbc610_show_cpuinfo(m: *mut seq_file) {
    static void gef_sbc610_show_cpuinfo(struct seq_file *m)
    {
    let mut svid: c_uint = mfspr(SPRN_SVR);
    seq_printf(m, "Vendor\t\t: GE Intelligent Platforms\n");
    seq_printf(m, "Revision\t: %u%c\n", gef_sbc610_get_pcb_rev(),
    ('A' + gef_sbc610_get_board_rev() - 1));
    seq_printf(m, "FPGA Revision\t: %u\n", gef_sbc610_get_fpga_rev());
    seq_printf(m, "SVR\t\t: 0x%x\n", svid);
    }
#[no_mangle]
unsafe extern "C" fn gef_sbc610_nec_fixup(pdev: *mut pci_dev) {
    static void gef_sbc610_nec_fixup(struct pci_dev *pdev)
    {
    unsigned int val;
// Do not do the fixup on other platforms!
    if (!machine_is(gef_sbc610))
    return;
    printk(KERN_INFO "Running NEC uPD720101 Fixup\n");
// Ensure ports 1, 2, 3, 4 & 5 are enabled
    pci_read_config_dword(pdev, 0xe0, &val);
    pci_write_config_dword(pdev, 0xe0, (val & ~7) | 0x5);
// System clock is 48-MHz Oscillator and EHCI Enabled.
    pci_write_config_dword(pdev, 0xe4, 1 << 5);
    }
    DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_NEC, PCI_DEVICE_ID_NEC_USB,
    gef_sbc610_nec_fixup);
    machine_arch_initcall(gef_sbc610, mpc86xx_common_publish_devices);
    define_machine(gef_sbc610) {
    .name			= "GE SBC610",
    .compatible		= "gef,sbc610",
    .setup_arch		= gef_sbc610_setup_arch,
    .init_IRQ		= gef_sbc610_init_irq,
    .show_cpuinfo		= gef_sbc610_show_cpuinfo,
    .get_irq		= mpic_get_irq,
    .time_init		= mpc86xx_time_init,
    .progress		= udbg_progress,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,

    };
