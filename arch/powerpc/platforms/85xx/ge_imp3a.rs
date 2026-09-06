//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/ge_imp3a.c
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
// GE IMP3A Board Setup
//
// Author Martyn Welch <martyn.welch@ge.com>
//
// Copyright 2010 GE Intelligent Platforms Embedded Systems, Inc.
//
// Based on: mpc85xx_ds.c (MPC85xx DS Board Setup)
// Copyright 2007 Freescale Semiconductor Inc.
//

    void __iomem *imp3a_regs;
#[no_mangle]
unsafe extern "C" fn ge_imp3a_pic_init() -> void __init {
    static void __init ge_imp3a_pic_init(void)
    {
    struct mpic *mpic;
    struct device_node *np;
    struct device_node *cascade_node = core::ptr::null_mut();
    if (of_machine_is_compatible("fsl,MPC8572DS-CAMP")) {
    mpic = mpic_alloc(core::ptr::null_mut(), 0,
    MPIC_NO_RESET |
    MPIC_BIG_ENDIAN |
    MPIC_SINGLE_DEST_CPU,
    0, 256, " OpenPIC  ");
    } else {
    mpic = mpic_alloc(core::ptr::null_mut(), 0,
    MPIC_BIG_ENDIAN |
    MPIC_SINGLE_DEST_CPU,
    0, 256, " OpenPIC  ");
    }
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
//
// There is a simple interrupt handler in the main FPGA, this needs
// to be cascaded into the MPIC
//
    for_each_node_by_type(np, "interrupt-controller")
    if (of_device_is_compatible(np, "gef,fpga-pic-1.00")) {
    cascade_node = np;
    break;
    }
    if (cascade_node == core::ptr::null_mut()) {
    printk(KERN_WARNING "IMP3A: No FPGA PIC\n");
    return;
    }
    gef_pic_init(cascade_node);
    of_node_put(cascade_node);
    }
#[no_mangle]
unsafe extern "C" fn ge_imp3a_pci_assign_primary() -> void __init {
    static void __init ge_imp3a_pci_assign_primary(void)
    {

    struct device_node *np;
    struct resource rsrc;
    for_each_node_by_type(np, "pci") {
    if (of_device_is_compatible(np, "fsl,mpc8540-pci") ||
    of_device_is_compatible(np, "fsl,mpc8548-pcie") ||
    of_device_is_compatible(np, "fsl,p2020-pcie")) {
    of_address_to_resource(np, 0, &rsrc);
    if ((rsrc.start & 0xfffff) == 0x9000) {
    of_node_put(fsl_pci_primary);
    fsl_pci_primary = of_node_get(np);
    }
    }
    }

    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn ge_imp3a_setup_arch() -> void __init {
    static void __init ge_imp3a_setup_arch(void)
    {
    struct device_node *regs;
    if (ppc_md.progress)
    ppc_md.progress("ge_imp3a_setup_arch()", 0);
    mpc85xx_smp_init();
    ge_imp3a_pci_assign_primary();
    swiotlb_detect_4g();
// Remap basic board registers
    regs = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ge,imp3a-fpga-regs");
    if (regs) {
    imp3a_regs = of_iomap(regs, 0);
    if (imp3a_regs == core::ptr::null_mut())
    printk(KERN_WARNING "Unable to map board registers\n");
    of_node_put(regs);
    }

    mmio_nvram_init();

    printk(KERN_INFO "GE Intelligent Platforms IMP3A 3U cPCI SBC\n");
    }
// Return the PCB revision
#[no_mangle]
unsafe extern "C" fn ge_imp3a_get_pcb_rev() -> c_uint {
    static unsigned int ge_imp3a_get_pcb_rev(void)
    {
    unsigned int reg;
    reg = ioread16(imp3a_regs);
    return (reg >> 8) & 0xff;
    }
// Return the board (software) revision
#[no_mangle]
unsafe extern "C" fn ge_imp3a_get_board_rev() -> c_uint {
    static unsigned int ge_imp3a_get_board_rev(void)
    {
    unsigned int reg;
    reg = ioread16(imp3a_regs + 0x2);
    return reg & 0xff;
    }
// Return the FPGA revision
#[no_mangle]
unsafe extern "C" fn ge_imp3a_get_fpga_rev() -> c_uint {
    static unsigned int ge_imp3a_get_fpga_rev(void)
    {
    unsigned int reg;
    reg = ioread16(imp3a_regs + 0x2);
    return (reg >> 8) & 0xff;
    }
// Return compactPCI Geographical Address
#[no_mangle]
unsafe extern "C" fn ge_imp3a_get_cpci_geo_addr() -> c_uint {
    static unsigned int ge_imp3a_get_cpci_geo_addr(void)
    {
    unsigned int reg;
    reg = ioread16(imp3a_regs + 0x6);
    return (reg & 0x0f00) >> 8;
    }
// Return compactPCI System Controller Status
#[no_mangle]
unsafe extern "C" fn ge_imp3a_get_cpci_is_syscon() -> c_uint {
    static unsigned int ge_imp3a_get_cpci_is_syscon(void)
    {
    unsigned int reg;
    reg = ioread16(imp3a_regs + 0x6);
    return reg & (1 << 12);
    }
#[no_mangle]
unsafe extern "C" fn ge_imp3a_show_cpuinfo(m: *mut seq_file) {
    static void ge_imp3a_show_cpuinfo(struct seq_file *m)
    {
    seq_printf(m, "Vendor\t\t: GE Intelligent Platforms\n");
    seq_printf(m, "Revision\t: %u%c\n", ge_imp3a_get_pcb_rev(),
    ('A' + ge_imp3a_get_board_rev() - 1));
    seq_printf(m, "FPGA Revision\t: %u\n", ge_imp3a_get_fpga_rev());
    seq_printf(m, "cPCI geo. addr\t: %u\n", ge_imp3a_get_cpci_geo_addr());
    seq_printf(m, "cPCI syscon\t: %s\n",
    ge_imp3a_get_cpci_is_syscon() ? "yes" : "no");
    }
    machine_arch_initcall(ge_imp3a, mpc85xx_common_publish_devices);
    define_machine(ge_imp3a) {
    .name			= "GE_IMP3A",
    .compatible		= "ge,IMP3A",
    .setup_arch		= ge_imp3a_setup_arch,
    .init_IRQ		= ge_imp3a_pic_init,
    .show_cpuinfo		= ge_imp3a_show_cpuinfo,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
