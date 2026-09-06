//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/86xx/mvme7100.c
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
// Board setup routines for the Emerson/Artesyn MVME7100
//
// Copyright 2016 Elettra-Sincrotrone Trieste S.C.p.A.
//
// Author: Alessio Igor Bogani <alessio.bogani@elettra.eu>
//
// Based on earlier code by:
//
// Ajit Prem <ajit.prem@emerson.com>
// Copyright 2008 Emerson
//
// USB host fixup is borrowed by:
//
// Martyn Welch <martyn.welch@ge.com>
// Copyright 2008 GE Intelligent Platforms Embedded Systems, Inc.
//

pub const MVME7100_INTERRUPT_REG_2_OFFSET: c_uint = 0x05;
pub const MVME7100_DS1375_MASK: c_uint = 0x40;
pub const MVME7100_MAX6649_MASK: c_uint = 0x20;
pub const MVME7100_ABORT_MASK: c_uint = 0x10;
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mvme7100_setup_arch() -> void __init {
    static void __init mvme7100_setup_arch(void)
    {
    struct device_node *bcsr_node;
    void __iomem *mvme7100_regs = core::ptr::null_mut();
    u8 reg;
    if (ppc_md.progress)
    ppc_md.progress("mvme7100_setup_arch()", 0);

    mpc86xx_smp_init();

    fsl_pci_assign_primary();
// Remap BCSR registers
    bcsr_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(),
    "artesyn,mvme7100-bcsr");
    if (bcsr_node) {
    mvme7100_regs = of_iomap(bcsr_node, 0);
    of_node_put(bcsr_node);
    }
    if (mvme7100_regs) {
// Disable ds1375, max6649, and abort interrupts
    reg = readb(mvme7100_regs + MVME7100_INTERRUPT_REG_2_OFFSET);
    reg |= MVME7100_DS1375_MASK | MVME7100_MAX6649_MASK
    | MVME7100_ABORT_MASK;
    writeb(reg, mvme7100_regs + MVME7100_INTERRUPT_REG_2_OFFSET);
    } else
    pr_warn("Unable to map board registers\n");
    pr_info("MVME7100 board from Artesyn\n");
    }
//
// Called very early, device-tree isn't unflattened
//
#[no_mangle]
unsafe extern "C" fn mvme7100_probe() -> int __init {
    static int __init mvme7100_probe(void)
    {
    let mut root: c_ulong = of_get_flat_dt_root();
    return of_flat_dt_is_compatible(root, "artesyn,MVME7100");
    }
#[no_mangle]
unsafe extern "C" fn mvme7100_usb_host_fixup(pdev: *mut pci_dev) {
    static void mvme7100_usb_host_fixup(struct pci_dev *pdev)
    {
    unsigned int val;
    if (!machine_is(mvme7100))
    return;
// Ensure only ports 1 & 2 are enabled
    pci_read_config_dword(pdev, 0xe0, &val);
    pci_write_config_dword(pdev, 0xe0, (val & ~7) | 0x2);
// System clock is 48-MHz Oscillator and EHCI Enabled.
    pci_write_config_dword(pdev, 0xe4, 1 << 5);
    }
    DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_NEC, PCI_DEVICE_ID_NEC_USB,
    mvme7100_usb_host_fixup);
    machine_arch_initcall(mvme7100, mpc86xx_common_publish_devices);
    define_machine(mvme7100) {
    .name			= "MVME7100",
    .probe			= mvme7100_probe,
    .setup_arch		= mvme7100_setup_arch,
    .init_IRQ		= mpc86xx_init_irq,
    .get_irq		= mpic_get_irq,
    .time_init		= mpc86xx_time_init,
    .progress		= udbg_progress,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,

    };
