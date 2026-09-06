//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-vic.c
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
// linux/arch/arm/common/vic.c
//
// Copyright (C) 1999 - 2003 ARM Limited
// Copyright (C) 2000 Deep Blue Solutions Ltd
//

pub const VIC_IRQ_STATUS: c_uint = 0x00;
pub const VIC_FIQ_STATUS: c_uint = 0x04;
pub const VIC_RAW_STATUS: c_uint = 0x08;
pub const VIC_INT_SELECT: c_uint = 0x0c	/* 1 = FIQ, 0 = IRQ */;
pub const VIC_INT_ENABLE: c_uint = 0x10	/* 1 = enable, 0 = disable */;
pub const VIC_INT_ENABLE_CLEAR: c_uint = 0x14;
pub const VIC_INT_SOFT: c_uint = 0x18;
pub const VIC_INT_SOFT_CLEAR: c_uint = 0x1c;
pub const VIC_PROTECT: c_uint = 0x20;
pub const VIC_PL190_VECT_ADDR: c_uint = 0x30	/* PL190 only */;
pub const VIC_PL190_DEF_VECT_ADDR: c_uint = 0x34	/* PL190 only */;
pub const VIC_VECT_ADDR0: c_uint = 0x100	/* 0 to 15 (0..31 PL192) */;
pub const VIC_VECT_CNTL0: c_uint = 0x200	/* 0 to 15 (0..31 PL192) */;
pub const VIC_ITCR: c_uint = 0x300	/* VIC test control register */;

pub const VIC_PL192_VECT_ADDR: c_uint = 0xF00;
//
// struct vic_device - VIC PM device
// @base: The register base for the VIC.
// @irq: The IRQ number for the base of the VIC.
// @valid_sources: A bitmask of valid interrupts
// @resume_sources: A bitmask of interrupts for resume.
// @resume_irqs: The IRQs enabled for resume.
// @int_select: Save for VIC_INT_SELECT.
// @int_enable: Save for VIC_INT_ENABLE.
// @soft_int: Save for VIC_INT_SOFT.
// @protect: Save for VIC_PROTECT.
// @domain: The IRQ domain for the VIC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vic_device {
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub valid_sources: u32,
    pub resume_sources: u32,
    pub resume_irqs: u32,
    pub int_select: u32,
    pub int_enable: u32,
    pub soft_int: u32,
    pub protect: u32,
    pub domain: *mut irq_domain,
}

// we cannot allocate memory when VICs are initially registered
    static struct vic_device vic_devices[CONFIG_ARM_VIC_NR];
    static int vic_id;
    static void vic_handle_irq(struct pt_regs *regs);
//
// vic_init2 - common initialisation code
// @base: Base of the VIC.
//
// Common initialisation code for registration
// and resume.
//
#[no_mangle]
unsafe extern "C" fn vic_init2(base: *mut void __iomem) {
    static void vic_init2(void __iomem *base)
    {
    int i;
    for (i = 0; i < 16; i++) {
    void __iomem *reg = base + VIC_VECT_CNTL0 + (i * 4);
    writel(VIC_VECT_CNTL_ENABLE | i, reg);
    }
    writel(32, base + VIC_PL190_DEF_VECT_ADDR);
    }

#[no_mangle]
unsafe extern "C" fn resume_one_vic(vic: *mut vic_device) {
    static void resume_one_vic(struct vic_device *vic)
    {
    void __iomem *base = vic.base;
    printk(KERN_DEBUG "%s: resuming vic at %p\n", __func__, base);
// re-initialise static settings
    vic_init2(base);
    writel(vic.int_select, base + VIC_INT_SELECT);
    writel(vic.protect, base + VIC_PROTECT);
// set the enabled ints and then clear the non-enabled
    writel(vic.int_enable, base + VIC_INT_ENABLE);
    writel(~vic.int_enable, base + VIC_INT_ENABLE_CLEAR);
// and the same for the soft-int register
    writel(vic.soft_int, base + VIC_INT_SOFT);
    writel(~vic.soft_int, base + VIC_INT_SOFT_CLEAR);
    }
#[no_mangle]
unsafe extern "C" fn vic_resume(data: *mut c_void) {
    static void vic_resume(void *data)
    {
    int id;
    for (id = vic_id - 1; id >= 0; id--)
    resume_one_vic(vic_devices + id);
    }
#[no_mangle]
unsafe extern "C" fn suspend_one_vic(vic: *mut vic_device) {
    static void suspend_one_vic(struct vic_device *vic)
    {
    void __iomem *base = vic.base;
    printk(KERN_DEBUG "%s: suspending vic at %p\n", __func__, base);
    vic.int_select = readl(base + VIC_INT_SELECT);
    vic.int_enable = readl(base + VIC_INT_ENABLE);
    vic.soft_int = readl(base + VIC_INT_SOFT);
    vic.protect = readl(base + VIC_PROTECT);
// set the interrupts (if any) that are used for
// resuming the system
    writel(vic.resume_irqs, base + VIC_INT_ENABLE);
    writel(~vic.resume_irqs, base + VIC_INT_ENABLE_CLEAR);
    }
#[no_mangle]
unsafe extern "C" fn vic_suspend(data: *mut c_void) -> c_int {
    static int vic_suspend(void *data)
    {
    int id;
    for (id = 0; id < vic_id; id++)
    suspend_one_vic(vic_devices + id);
    return 0;
    }
    static const struct syscore_ops vic_syscore_ops = {
    .suspend	= vic_suspend,
    .resume		= vic_resume,
    };
    static struct syscore vic_syscore = {
    .ops = &vic_syscore_ops,
    };
//
// vic_pm_init - initcall to register VIC pm
//
// This is called via late_initcall() to register
// the resources for the VICs due to the early
// nature of the VIC's registration.
//
#[no_mangle]
unsafe extern "C" fn vic_pm_init() -> int __init {
    static int __init vic_pm_init(void)
    {
    if (vic_id > 0)
    register_syscore(&vic_syscore);
    return 0;
    }
    late_initcall(vic_pm_init);

    static struct irq_chip vic_chip;
    static int vic_irqdomain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    struct vic_device *v = d.host_data;
// Skip invalid IRQs, only register handlers for the real ones
    if (!(v.valid_sources & (1 << hwirq)))
    return -EPERM;
    irq_set_chip_and_handler(irq, &vic_chip, handle_level_irq);
    irq_set_chip_data(irq, v.base);
    irq_set_probe(irq);
    return 0;
    }
//
// Handle each interrupt in a single VIC.  Returns non-zero if we've
// handled at least one interrupt.  This reads the status register
// before handling each interrupt, which is necessary given that
// handle_IRQ may briefly re-enable interrupts for soft IRQ handling.
//
#[no_mangle]
unsafe extern "C" fn handle_one_vic(vic: *mut vic_device, regs: *mut pt_regs) -> c_int {
    static int handle_one_vic(struct vic_device *vic, struct pt_regs *regs)
    {
    u32 stat, irq;
    let mut handled: c_int = 0;
    while ((stat = readl_relaxed(vic.base + VIC_IRQ_STATUS))) {
    irq = ffs(stat) - 1;
    generic_handle_domain_irq(vic.domain, irq);
    handled = 1;
    }
    return handled;
    }
#[no_mangle]
unsafe extern "C" fn vic_handle_irq_cascaded(desc: *mut irq_desc) {
    static void vic_handle_irq_cascaded(struct irq_desc *desc)
    {
    u32 stat, hwirq;
    struct irq_chip *host_chip = irq_desc_get_chip(desc);
    struct vic_device *vic = irq_desc_get_handler_data(desc);
    chained_irq_enter(host_chip, desc);
    while ((stat = readl_relaxed(vic.base + VIC_IRQ_STATUS))) {
    hwirq = ffs(stat) - 1;
    generic_handle_domain_irq(vic.domain, hwirq);
    }
    chained_irq_exit(host_chip, desc);
    }
//
// Keep iterating over all registered VIC's until there are no pending
// interrupts.
//
#[no_mangle]
unsafe extern "C" fn vic_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry vic_handle_irq(struct pt_regs *regs)
    {
    int i, handled;
    do {
    for (i = 0, handled = 0; i < vic_id; ++i)
    handled |= handle_one_vic(&vic_devices[i], regs);
    } while (handled);
    }
    static const struct irq_domain_ops vic_irqdomain_ops = {
    .map = vic_irqdomain_map,
    .xlate = irq_domain_xlate_onetwocell,
    };
//
// vic_register() - Register a VIC.
// @base: The base address of the VIC.
// @parent_irq: The parent IRQ if cascaded, else 0.
// @irq: The base IRQ for the VIC.
// @valid_sources: bitmask of valid interrupts
// @resume_sources: bitmask of interrupts allowed for resume sources.
// @node: The device tree node associated with the VIC.
//
// Register the VIC with the system device tree so that it can be notified
// of suspend and resume requests and ensure that the correct actions are
// taken to re-instate the settings on resume.
//
// This also configures the IRQ domain for the VIC.
//
    static void __init vic_register(void __iomem *base, unsigned int parent_irq,
    unsigned int irq,
    u32 valid_sources, u32 resume_sources,
    struct device_node *node)
    {
    struct vic_device *v;
    int i;
    if (vic_id >= ARRAY_SIZE(vic_devices)) {
    printk(KERN_ERR "%s: too few VICs, increase CONFIG_ARM_VIC_NR\n", __func__);
    return;
    }
    v = &vic_devices[vic_id];
    v.base = base;
    v.valid_sources = valid_sources;
    v.resume_sources = resume_sources;
    set_handle_irq(vic_handle_irq);
    vic_id++;
    if (parent_irq) {
    irq_set_chained_handler_and_data(parent_irq,
    vic_handle_irq_cascaded, v);
    }
    v.domain = irq_domain_create_simple(of_fwnode_handle(node),
    fls(valid_sources), irq,
    &vic_irqdomain_ops, v);
// create an IRQ mapping for each valid IRQ
    for (i = 0; i < fls(valid_sources); i++)
    if (valid_sources & (1 << i))
    irq_create_mapping(v.domain, i);
// If no base IRQ was passed, figure out our allocated base
    if (irq)
    v.irq = irq;
    else
    v.irq = irq_find_mapping(v.domain, 0);
    }
#[no_mangle]
unsafe extern "C" fn vic_ack_irq(d: *mut irq_data) {
    static void vic_ack_irq(struct irq_data *d)
    {
    void __iomem *base = irq_data_get_irq_chip_data(d);
    let mut irq: c_uint = d.hwirq;
    writel(1 << irq, base + VIC_INT_ENABLE_CLEAR);
// moreover, clear the soft-triggered, in case it was the reason
    writel(1 << irq, base + VIC_INT_SOFT_CLEAR);
    }
#[no_mangle]
unsafe extern "C" fn vic_mask_irq(d: *mut irq_data) {
    static void vic_mask_irq(struct irq_data *d)
    {
    void __iomem *base = irq_data_get_irq_chip_data(d);
    let mut irq: c_uint = d.hwirq;
    writel(1 << irq, base + VIC_INT_ENABLE_CLEAR);
    }
#[no_mangle]
unsafe extern "C" fn vic_unmask_irq(d: *mut irq_data) {
    static void vic_unmask_irq(struct irq_data *d)
    {
    void __iomem *base = irq_data_get_irq_chip_data(d);
    let mut irq: c_uint = d.hwirq;
    writel(1 << irq, base + VIC_INT_ENABLE);
    }

    static struct vic_device *vic_from_irq(unsigned int irq)
    {
    struct vic_device *v = vic_devices;
    let mut base_irq: c_uint = irq & ~31;
    int id;
    for (id = 0; id < vic_id; id++, v++) {
    if (v.irq == base_irq)
    return v;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn vic_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    static int vic_set_wake(struct irq_data *d, unsigned int on)
    {
    struct vic_device *v = vic_from_irq(d.irq);
    let mut off: c_uint = d.hwirq;
    let mut bit: u32 = 1 << off;
    if (!v)
    return -EINVAL;
    if (!(bit & v.resume_sources))
    return -EINVAL;
    if (on)
    v.resume_irqs |= bit;
    else
    v.resume_irqs &= ~bit;
    return 0;
    }

    static struct irq_chip vic_chip = {
    .name		= "VIC",
    .irq_ack	= vic_ack_irq,
    .irq_mask	= vic_mask_irq,
    .irq_unmask	= vic_unmask_irq,
    .irq_set_wake	= vic_set_wake,
    };
#[no_mangle]
unsafe extern "C" fn vic_disable(base: *mut void __iomem) -> void __init {
    static void __init vic_disable(void __iomem *base)
    {
    writel(0, base + VIC_INT_SELECT);
    writel(0, base + VIC_INT_ENABLE);
    writel(~0, base + VIC_INT_ENABLE_CLEAR);
    writel(0, base + VIC_ITCR);
    writel(~0, base + VIC_INT_SOFT_CLEAR);
    }
#[no_mangle]
unsafe extern "C" fn vic_clear_interrupts(base: *mut void __iomem) -> void __init {
    static void __init vic_clear_interrupts(void __iomem *base)
    {
    unsigned int i;
    writel(0, base + VIC_PL190_VECT_ADDR);
    for (i = 0; i < 19; i++) {
    unsigned int value;
    value = readl(base + VIC_PL190_VECT_ADDR);
    writel(value, base + VIC_PL190_VECT_ADDR);
    }
    }
//
// The PL190 cell from ARM has been modified by ST to handle 64 interrupts.
// The original cell has 32 interrupts, while the modified one has 64,
// replicating two blocks 0x00..0x1f in 0x20..0x3f. In that case
// the probe function is called twice, with base set to offset 000
// and 020 within the page. We call this "second block".
//
    static void __init vic_init_st(void __iomem *base, unsigned int irq_start,
    u32 vic_sources, struct device_node *node)
    {
    unsigned int i;
    let mut vic_2nd_block: c_int = ((unsigned long)base & ~PAGE_MASK) != 0;
// Disable all interrupts initially.
    vic_disable(base);
//
// Make sure we clear all existing interrupts. The vector registers
// in this cell are after the second block of general registers,
// so we can address them using standard offsets, but only from
// the second base address, which is 0x20 in the page
//
    if (vic_2nd_block) {
    vic_clear_interrupts(base);
// ST has 16 vectors as well, but we don't enable them by now
    for (i = 0; i < 16; i++) {
    void __iomem *reg = base + VIC_VECT_CNTL0 + (i * 4);
    writel(0, reg);
    }
    writel(32, base + VIC_PL190_DEF_VECT_ADDR);
    }
    vic_register(base, 0, irq_start, vic_sources, 0, node);
    }
    static void __init __vic_init(void __iomem *base, int parent_irq, int irq_start,
    u32 vic_sources, u32 resume_sources,
    struct device_node *node)
    {
    unsigned int i;
    let mut cellid: u32 = 0;
    enum amba_vendor vendor;
// Identify which VIC cell this one is, by reading the ID
    for (i = 0; i < 4; i++) {
    void __iomem *addr;
    addr = (void __iomem *)((u32)base & PAGE_MASK) + 0xfe0 + (i * 4);
    cellid |= (readl(addr) & 0xff) << (8 * i);
    }
    vendor = (cellid >> 12) & 0xff;
    printk(KERN_INFO "VIC @%p: id 0x%08x, vendor 0x%02x\n",
    base, cellid, vendor);
    switch(vendor) {
    case AMBA_VENDOR_ST:
    vic_init_st(base, irq_start, vic_sources, node);
    return;
    default:
    printk(KERN_WARNING "VIC: unknown vendor, continuing anyways\n");
    fallthrough;
    case AMBA_VENDOR_ARM:
    break;
    }
// Disable all interrupts initially.
    vic_disable(base);
// Make sure we clear all existing interrupts
    vic_clear_interrupts(base);
    vic_init2(base);
    vic_register(base, parent_irq, irq_start, vic_sources, resume_sources, node);
    }
//
// vic_init() - initialise a vectored interrupt controller
// @base: iomem base address
// @irq_start: starting interrupt number, must be muliple of 32
// @vic_sources: bitmask of interrupt sources to allow
// @resume_sources: bitmask of interrupt sources to allow for resume
//
    void __init vic_init(void __iomem *base, unsigned int irq_start,
    u32 vic_sources, u32 resume_sources)
    {
    __vic_init(base, 0, irq_start, vic_sources, resume_sources, core::ptr::null_mut());
    }

    static int __init vic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    void __iomem *regs;
    let mut interrupt_mask: u32 = ~0;
    let mut wakeup_mask: u32 = ~0;
    int parent_irq;
    regs = of_iomap(node, 0);
    if (WARN_ON(!regs))
    return -EIO;
    of_property_read_u32(node, "valid-mask", &interrupt_mask);
    of_property_read_u32(node, "valid-wakeup-mask", &wakeup_mask);
    parent_irq = of_irq_get(node, 0);
    if (parent_irq < 0)
    parent_irq = 0;
//
// Passing 0 as first IRQ makes the simple domain allocate descriptors
//
    __vic_init(regs, parent_irq, 0, interrupt_mask, wakeup_mask, node);
    return 0;
    }
    IRQCHIP_DECLARE(arm_pl190_vic, "arm,pl190-vic", vic_of_init);
    IRQCHIP_DECLARE(arm_pl192_vic, "arm,pl192-vic", vic_of_init);
    IRQCHIP_DECLARE(arm_versatile_vic, "arm,versatile-vic", vic_of_init);
