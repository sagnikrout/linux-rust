//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/tsi108_pci.c
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
// Common routines for Tundra Semiconductor TSI108 host bridge.
//
// 2004-2005 (c) Tundra Semiconductor Corp.
// Author: Alex Bounine (alexandreb@tundra.com)
// Author: Roy Zang (tie-fei.zang@freescale.com)
// Add pci interrupt router host
//

// Macro flag: #define DBG(x...)

    ((((bus)<<16) | ((devfunc)<<8) | (offset & 0xfc)) + tsi108_pci_cfg_base)
    u32 tsi108_pci_cfg_base;
    static u32 tsi108_pci_cfg_phys;
    u32 tsi108_csr_vir_base;
    static struct irq_domain *pci_irq_host;
    extern u32 get_vir_csrbase(void);
    extern u32 tsi108_read_reg(u32 reg_offset);
    extern void tsi108_write_reg(u32 reg_offset, u32 val);
    int
    tsi108_direct_write_config(struct pci_bus *bus, unsigned int devfunc,
    int offset, int len, u32 val)
    {
    volatile unsigned char *cfg_addr;
    struct pci_controller *hose = pci_bus_to_host(bus);
    if (ppc_md.pci_exclude_device)
    if (ppc_md.pci_exclude_device(hose, bus.number, devfunc))
    return PCIBIOS_DEVICE_NOT_FOUND;
    cfg_addr = (unsigned char *)(tsi_mk_config_addr(bus.number,
    devfunc, offset) |
    (offset & 0x03));

    printk("PCI CFG write : ");
    printk("%d:0x%x:0x%x ", bus.number, devfunc, offset);
    printk("%d ADDR=0x%08x ", len, (uint) cfg_addr);
    printk("data = 0x%08x\n", val);

    switch (len) {
    case 1:
    out_8((u8 *) cfg_addr, val);
    break;
    case 2:
    out_le16((u16 *) cfg_addr, val);
    break;
    default:
    out_le32((u32 *) cfg_addr, val);
    break;
    }
    return PCIBIOS_SUCCESSFUL;
    }
#[no_mangle]
pub unsafe extern "C" fn tsi108_clear_pci_error(pci_cfg_base: u32) {
    void tsi108_clear_pci_error(u32 pci_cfg_base)
    {
    u32 err_stat, err_addr, pci_stat;
//
// Quietly clear PB and PCI error flags set as result
// of PCI/X configuration read requests.
//
// Read PB Error Log Registers
    err_stat = tsi108_read_reg(TSI108_PB_OFFSET + TSI108_PB_ERRCS);
    err_addr = tsi108_read_reg(TSI108_PB_OFFSET + TSI108_PB_AERR);
    if (err_stat & TSI108_PB_ERRCS_ES) {
// Clear error flag
    tsi108_write_reg(TSI108_PB_OFFSET + TSI108_PB_ERRCS,
    TSI108_PB_ERRCS_ES);
// Clear read error reported in PB_ISR
    tsi108_write_reg(TSI108_PB_OFFSET + TSI108_PB_ISR,
    TSI108_PB_ISR_PBS_RD_ERR);
// Clear PCI/X bus cfg errors if applicable
    if ((err_addr & 0xFF000000) == pci_cfg_base) {
    pci_stat =
    tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_CSR);
    tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_CSR,
    pci_stat);
    }
    }
    return;
    }

    __asm__ __volatile__(				\
    "	"op" %0,0,%1\n"		\
    "1:	eieio\n"			\
    "2:\n"					\
    ".section .fixup,\"ax\"\n"		\
    "3:	li %0,-1\n"			\
    "	b 2b\n"				\
    ".previous\n"				\
    EX_TABLE(1b, 3b)			\
    : "=r"(x) : "r"(addr))
    int
    tsi108_direct_read_config(struct pci_bus *bus, unsigned int devfn, int offset,
    int len, u32 * val)
    {
    volatile unsigned char *cfg_addr;
    struct pci_controller *hose = pci_bus_to_host(bus);
    u32 temp;
    if (ppc_md.pci_exclude_device)
    if (ppc_md.pci_exclude_device(hose, bus.number, devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
    cfg_addr = (unsigned char *)(tsi_mk_config_addr(bus.number,
    devfn,
    offset) | (offset &
    0x03));
    switch (len) {
    case 1:
    __tsi108_read_pci_config(temp, cfg_addr, "lbzx");
    break;
    case 2:
    __tsi108_read_pci_config(temp, cfg_addr, "lhbrx");
    break;
    default:
    __tsi108_read_pci_config(temp, cfg_addr, "lwbrx");
    break;
    }
// val = temp;

    if ((0xFFFFFFFF != temp) && (0xFFFF != temp) && (0xFF != temp)) {
    printk("PCI CFG read : ");
    printk("%d:0x%x:0x%x ", bus.number, devfn, offset);
    printk("%d ADDR=0x%08x ", len, (uint) cfg_addr);
    printk("data = 0x%x\n", *val);
    }

    return PCIBIOS_SUCCESSFUL;
    }
#[no_mangle]
pub unsafe extern "C" fn tsi108_clear_pci_cfg_error() {
    void tsi108_clear_pci_cfg_error(void)
    {
    tsi108_clear_pci_error(tsi108_pci_cfg_phys);
    }
    static struct pci_ops tsi108_direct_pci_ops = {
    .read = tsi108_direct_read_config,
    .write = tsi108_direct_write_config,
    };
#[no_mangle]
pub unsafe extern "C" fn tsi108_setup_pci(dev: *mut device_node, cfg_phys: u32, primary: c_int) -> int __init {
    int __init tsi108_setup_pci(struct device_node *dev, u32 cfg_phys, int primary)
    {
    int len;
    struct pci_controller *hose;
    struct resource rsrc;
    const int *bus_range;
    let mut has_address: c_int = 0;
// PCI Config mapping
    tsi108_pci_cfg_base = (u32)ioremap(cfg_phys, TSI108_PCI_CFG_SIZE);
    tsi108_pci_cfg_phys = cfg_phys;
    DBG("TSI_PCI: %s tsi108_pci_cfg_base=0x%x\n", __func__,
    tsi108_pci_cfg_base);
// Fetch host bridge registers address
    has_address = (of_address_to_resource(dev, 0, &rsrc) == 0);
// Get bus range if any
    bus_range = of_get_property(dev, "bus-range", &len);
    if (bus_range == core::ptr::null_mut() || len < 2 * sizeof(int)) {
    printk(KERN_WARNING "Can't get bus-range for %pOF, assume"
    " bus 0\n", dev);
    }
    hose = pcibios_alloc_controller(dev);
    if (!hose) {
    printk("PCI Host bridge init failed\n");
    return -ENOMEM;
    }
    hose.first_busno = bus_range ? bus_range[0] : 0;
    hose.last_busno = bus_range ? bus_range[1] : 0xff;
    (hose).ops = &tsi108_direct_pci_ops;
    pr_info("Found tsi108 PCI host bridge at 0x%pa. Firmware bus number: %d.%d\n",
    &rsrc.start, hose.first_busno, hose.last_busno);
// Interpret the "ranges" property
// This also maps the I/O region and sets isa_io/mem_base
    pci_process_bridge_OF_ranges(hose, dev, primary);
    return 0;
    }
//
// Low level utility functions
//
#[no_mangle]
unsafe extern "C" fn tsi108_pci_int_mask(irq: u_int) {
    static void tsi108_pci_int_mask(u_int irq)
    {
    u_int irp_cfg;
    let mut int_line: c_int = (irq - IRQ_PCI_INTAD_BASE);
    irp_cfg = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL);
    mb();
    irp_cfg |= (1 << int_line);	/* INTx_DIR = output */
    irp_cfg &= ~(3 << (8 + (int_line * 2)));	/* INTx_TYPE = unused */
    tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL, irp_cfg);
    mb();
    irp_cfg = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL);
    }
#[no_mangle]
unsafe extern "C" fn tsi108_pci_int_unmask(irq: u_int) {
    static void tsi108_pci_int_unmask(u_int irq)
    {
    u_int irp_cfg;
    let mut int_line: c_int = (irq - IRQ_PCI_INTAD_BASE);
    irp_cfg = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL);
    mb();
    irp_cfg &= ~(1 << int_line);
    irp_cfg |= (3 << (8 + (int_line * 2)));
    tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL, irp_cfg);
    mb();
    }
#[no_mangle]
unsafe extern "C" fn init_pci_source() -> void __init {
    static void __init init_pci_source(void)
    {
    tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL,
    0x0000ff00);
    tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_ENABLE,
    TSI108_PCI_IRP_ENABLE_P_INT);
    mb();
    }
#[no_mangle]
pub unsafe extern "C" fn get_pci_source() -> c_uint {
    static inline unsigned int get_pci_source(void)
    {
    let mut temp: u_int = 0;
    let mut irq: c_int = -1;
    int i;
    u_int pci_irp_stat;
    let mut mask: static int = 0;
// Read PCI/X block interrupt status register
    pci_irp_stat = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_STAT);
    mb();
    if (pci_irp_stat & TSI108_PCI_IRP_STAT_P_INT) {
// Process Interrupt from PCI bus INTA# - INTD# lines
    temp =
    tsi108_read_reg(TSI108_PCI_OFFSET +
    TSI108_PCI_IRP_INTAD) & 0xf;
    mb();
    for (i = 0; i < 4; i++, mask++) {
    if (temp & (1 << mask % 4)) {
    irq = IRQ_PCI_INTA + mask % 4;
    mask++;
    break;
    }
    }
// Disable interrupts from PCI block
    temp = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_ENABLE);
    tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_ENABLE,
    temp & ~TSI108_PCI_IRP_ENABLE_P_INT);
    mb();
    (void)tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_ENABLE);
    mb();
    }

    else {
    printk("TSI108_PIC: error in TSI108_PCI_IRP_STAT\n");
    pci_irp_stat =
    tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_STAT);
    temp =
    tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_INTAD);
    mb();
    printk(">> stat=0x%08x intad=0x%08x ", pci_irp_stat, temp);
    temp =
    tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL);
    mb();
    printk("cfg_ctl=0x%08x ", temp);
    temp =
    tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_ENABLE);
    mb();
    printk("irp_enable=0x%08x\n", temp);
    }

    return irq;
    }
//
// Linux descriptor level callbacks
//
#[no_mangle]
unsafe extern "C" fn tsi108_pci_irq_unmask(d: *mut irq_data) {
    static void tsi108_pci_irq_unmask(struct irq_data *d)
    {
    tsi108_pci_int_unmask(d.irq);
// Enable interrupts from PCI block
    tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_ENABLE,
    tsi108_read_reg(TSI108_PCI_OFFSET +
    TSI108_PCI_IRP_ENABLE) |
    TSI108_PCI_IRP_ENABLE_P_INT);
    mb();
    }
#[no_mangle]
unsafe extern "C" fn tsi108_pci_irq_mask(d: *mut irq_data) {
    static void tsi108_pci_irq_mask(struct irq_data *d)
    {
    tsi108_pci_int_mask(d.irq);
    }
#[no_mangle]
unsafe extern "C" fn tsi108_pci_irq_ack(d: *mut irq_data) {
    static void tsi108_pci_irq_ack(struct irq_data *d)
    {
    tsi108_pci_int_mask(d.irq);
    }
//
// Interrupt controller descriptor for cascaded PCI interrupt controller.
//
    static struct irq_chip tsi108_pci_irq = {
    .name = "tsi108_PCI_int",
    .irq_mask = tsi108_pci_irq_mask,
    .irq_ack = tsi108_pci_irq_ack,
    .irq_unmask = tsi108_pci_irq_unmask,
    };
    static int pci_irq_host_xlate(struct irq_domain *h, struct device_node *ct,
    const u32 *intspec, unsigned int intsize,
    irq_hw_number_t *out_hwirq, unsigned int *out_flags)
    {
// out_hwirq = intspec[0];
// out_flags = IRQ_TYPE_LEVEL_HIGH;
    return 0;
    }
    static int pci_irq_host_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {	unsigned int irq;
    DBG("%s(%d, 0x%lx)\n", __func__, virq, hw);
    if ((virq >= 1) && (virq <= 4)){
    irq = virq + IRQ_PCI_INTAD_BASE - 1;
    irq_set_status_flags(irq, IRQ_LEVEL);
    irq_set_chip(irq, &tsi108_pci_irq);
    }
    return 0;
    }
    static const struct irq_domain_ops pci_irq_domain_ops = {
    .map = pci_irq_host_map,
    .xlate = pci_irq_host_xlate,
    };
//
// Exported functions
//
// The Tsi108 PCI interrupts initialization routine.
//
// The INTA# - INTD# interrupts on the PCI bus are reported by the PCI block
// to the MPIC using single interrupt source (IRQ_TSI108_PCI). Therefore the
// PCI block has to be treated as a cascaded interrupt controller connected
// to the MPIC.
//
#[no_mangle]
pub unsafe extern "C" fn tsi108_pci_int_init(node: *mut device_node) -> void __init {
    void __init tsi108_pci_int_init(struct device_node *node)
    {
    DBG("Tsi108_pci_int_init: initializing PCI interrupts\n");
    pci_irq_host = irq_domain_create_legacy(of_fwnode_handle(node), NR_IRQS_LEGACY, 0, 0,
    &pci_irq_domain_ops, core::ptr::null_mut());
    if (pci_irq_host == core::ptr::null_mut()) {
    printk(KERN_ERR "pci_irq_host: failed to allocate irq domain!\n");
    return;
    }
    init_pci_source();
    }
#[no_mangle]
pub unsafe extern "C" fn tsi108_irq_cascade(desc: *mut irq_desc) {
    void tsi108_irq_cascade(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    let mut cascade_irq: c_uint = get_pci_source();
    if (cascade_irq)
    generic_handle_irq(cascade_irq);
    chip.irq_eoi(&desc.irq_data);
    }
