//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/embedded6xx/holly.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Board setup routines for the IBM 750GX/CL platform w/ TSI10x bridge
//
// Copyright 2007 IBM Corporation
//
// Stephen Winiecki <stevewin@us.ibm.com>
// Josh Boyer <jwboyer@linux.vnet.ibm.com>
//
// Based on code from mpc7448_hpc2.c
//

pub const HOLLY_PCI_CFG_PHYS: c_uint = 0x7c000000;
    static int holly_exclude_device(struct pci_controller *hose, u_char bus,
    u_char devfn)
    {
    if (bus == 0 && PCI_SLOT(devfn) == 0)
    return PCIBIOS_DEVICE_NOT_FOUND;
    else
    return PCIBIOS_SUCCESSFUL;
    }
#[no_mangle]
unsafe extern "C" fn holly_remap_bridge() -> void __init {
    static void __init holly_remap_bridge(void)
    {
    u32 lut_val, lut_addr;
    int i;
    printk(KERN_INFO "Remapping PCI bridge\n");
// Re-init the PCI bridge and LUT registers to have mappings that don't
// rely on PIBS
//
    lut_addr = 0x900;
    for (i = 0; i < 31; i++) {
    tsi108_write_reg(TSI108_PB_OFFSET + lut_addr, 0x00000201);
    lut_addr += 4;
    tsi108_write_reg(TSI108_PB_OFFSET + lut_addr, 0x0);
    lut_addr += 4;
    }
// Reserve the last LUT entry for PCI I/O space
    tsi108_write_reg(TSI108_PB_OFFSET + lut_addr, 0x00000241);
    lut_addr += 4;
    tsi108_write_reg(TSI108_PB_OFFSET + lut_addr, 0x0);
// Map PCI I/O space
    tsi108_write_reg(TSI108_PCI_PFAB_IO_UPPER, 0x0);
    tsi108_write_reg(TSI108_PCI_PFAB_IO, 0x1);
// Map PCI CFG space
    tsi108_write_reg(TSI108_PCI_PFAB_BAR0_UPPER, 0x0);
    tsi108_write_reg(TSI108_PCI_PFAB_BAR0, 0x7c000000 | 0x01);
// We don't need MEM32 and PRM remapping so disable them
    tsi108_write_reg(TSI108_PCI_PFAB_MEM32, 0x0);
    tsi108_write_reg(TSI108_PCI_PFAB_PFM3, 0x0);
    tsi108_write_reg(TSI108_PCI_PFAB_PFM4, 0x0);
// Set P2O_BAR0
    tsi108_write_reg(TSI108_PCI_P2O_BAR0_UPPER, 0x0);
    tsi108_write_reg(TSI108_PCI_P2O_BAR0, 0xc0000000);
// Init the PCI LUTs to do no remapping
    lut_addr = 0x500;
    lut_val = 0x00000002;
    for (i = 0; i < 32; i++) {
    tsi108_write_reg(TSI108_PCI_OFFSET + lut_addr, lut_val);
    lut_addr += 4;
    tsi108_write_reg(TSI108_PCI_OFFSET + lut_addr, 0x40000000);
    lut_addr += 4;
    lut_val += 0x02000000;
    }
    tsi108_write_reg(TSI108_PCI_P2O_PAGE_SIZES, 0x00007900);
// Set 64-bit PCI bus address for system memory
    tsi108_write_reg(TSI108_PCI_P2O_BAR2_UPPER, 0x0);
    tsi108_write_reg(TSI108_PCI_P2O_BAR2, 0x0);
    }
#[no_mangle]
unsafe extern "C" fn holly_init_pci() -> void __init {
    static void __init holly_init_pci(void)
    {
    struct device_node *np;
    if (ppc_md.progress)
    ppc_md.progress("holly_setup_arch():set_bridge", 0);
// setup PCI host bridge
    holly_remap_bridge();
    np = of_find_node_by_type(core::ptr::null_mut(), "pci");
    if (np)
    tsi108_setup_pci(np, HOLLY_PCI_CFG_PHYS, 1);
    of_node_put(np);
    ppc_md.pci_exclude_device = holly_exclude_device;
    if (ppc_md.progress)
    ppc_md.progress("tsi108: resources set", 0x100);
    }
#[no_mangle]
unsafe extern "C" fn holly_setup_arch() -> void __init {
    static void __init holly_setup_arch(void)
    {
    tsi108_csr_vir_base = get_vir_csrbase();
    printk(KERN_INFO "PPC750GX/CL Platform\n");
    }
//
// Interrupt setup and service.  Interrupts on the holly come
// from the four external INT pins, PCI interrupts are routed via
// PCI interrupt control registers, it generates internal IRQ23
//
// Interrupt routing on the Holly Board:
// TSI108:PB_INT[0] -> CPU0:INT#
// TSI108:PB_INT[1] -> CPU0:MCP#
// TSI108:PB_INT[2] -> N/C
// TSI108:PB_INT[3] -> N/C
//
#[no_mangle]
unsafe extern "C" fn holly_init_IRQ() -> void __init {
    static void __init holly_init_IRQ(void)
    {
    struct mpic *mpic;

    unsigned int cascade_pci_irq;
    struct device_node *tsi_pci;
    struct device_node *cascade_node = core::ptr::null_mut();

    mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN |
    MPIC_SPV_EOI | MPIC_NO_PTHROU_DIS | MPIC_REGSET_TSI108,
    24, 0,
    "Tsi108_PIC");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_assign_isu(mpic, 0, mpic.paddr + 0x100);
    mpic_init(mpic);

    tsi_pci = of_find_node_by_type(core::ptr::null_mut(), "pci");
    if (tsi_pci == core::ptr::null_mut()) {
    printk(KERN_ERR "%s: No tsi108 pci node found !\n", __func__);
    return;
    }
    cascade_node = of_find_node_by_type(core::ptr::null_mut(), "pic-router");
    if (cascade_node == core::ptr::null_mut()) {
    printk(KERN_ERR "%s: No tsi108 pci cascade node found !\n", __func__);
    return;
    }
    cascade_pci_irq = irq_of_parse_and_map(tsi_pci, 0);
    pr_debug("%s: tsi108 cascade_pci_irq = 0x%x\n", __func__, (u32) cascade_pci_irq);
    tsi108_pci_int_init(cascade_node);
    irq_set_handler_data(cascade_pci_irq, mpic);
    irq_set_chained_handler(cascade_pci_irq, tsi108_irq_cascade);
    of_node_put(tsi_pci);
    of_node_put(cascade_node);

// Configure MPIC outputs to CPU0
    tsi108_write_reg(TSI108_MPIC_OFFSET + 0x30c, 0);
    }
#[no_mangle]
unsafe extern "C" fn holly_show_cpuinfo(m: *mut seq_file) {
    static void holly_show_cpuinfo(struct seq_file *m)
    {
    seq_printf(m, "vendor\t\t: IBM\n");
    seq_printf(m, "machine\t\t: PPC750 GX/CL\n");
    }
#[no_mangle]
unsafe extern "C" fn holly_restart(cmd: *mut c_char) -> void __noreturn {
    static void __noreturn holly_restart(char *cmd)
    {
    __be32 __iomem *ocn_bar1 = core::ptr::null_mut();
    unsigned long bar;
    struct device_node *bridge = core::ptr::null_mut();
    struct resource res;
    let mut addr: phys_addr_t = 0xc0000000;
    local_irq_disable();
    bridge = of_find_node_by_type(core::ptr::null_mut(), "tsi-bridge");
    if (bridge) {
    of_address_to_resource(bridge, 0, &res);
    addr = res.start;
    of_node_put(bridge);
    }
    addr += (TSI108_PB_OFFSET + 0x414);
    ocn_bar1 = ioremap(addr, 0x4);
// Turn on the BOOT bit so the addresses are correctly
// routed to the HLP interface
    bar = ioread32be(ocn_bar1);
    bar |= 2;
    iowrite32be(bar, ocn_bar1);
    iosync();
// Set SRR0 to the reset vector and turn on MSR_IP
    mtspr(SPRN_SRR0, 0xfff00100);
    mtspr(SPRN_SRR1, MSR_IP);
// Do an rfi to jump back to firmware.  Somewhat evil,
// but it works
//
    __asm__ __volatile__("rfi" : : : "memory");
// Spin until reset happens.  Shouldn't really get here
    for (;;) ;
    }
#[no_mangle]
unsafe extern "C" fn ppc750_machine_check_exception(regs: *mut pt_regs) -> c_int {
    static int ppc750_machine_check_exception(struct pt_regs *regs)
    {
    const struct exception_table_entry *entry;
// Are we prepared to handle this fault
    if ((entry = search_exception_tables(regs.nip)) != core::ptr::null_mut()) {
    tsi108_clear_pci_cfg_error();
    regs_set_recoverable(regs);
    regs_set_return_ip(regs, extable_fixup(entry));
    return 1;
    }
    return 0;
    }
    define_machine(holly){
    .name                   	= "PPC750 GX/CL TSI",
    .compatible			= "ibm,holly",
    .setup_arch             	= holly_setup_arch,
    .discover_phbs			= holly_init_pci,
    .init_IRQ               	= holly_init_IRQ,
    .show_cpuinfo           	= holly_show_cpuinfo,
    .get_irq                	= mpic_get_irq,
    .restart                	= holly_restart,
    .machine_check_exception	= ppc750_machine_check_exception,
    .progress               	= udbg_progress,
    };
