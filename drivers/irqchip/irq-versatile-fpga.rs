//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-versatile-fpga.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Support for Versatile FPGA-based IRQ controllers
//

pub const IRQ_STATUS: c_uint = 0x00;
pub const IRQ_RAW_STATUS: c_uint = 0x04;
pub const IRQ_ENABLE_SET: c_uint = 0x08;
pub const IRQ_ENABLE_CLEAR: c_uint = 0x0c;
pub const INT_SOFT_SET: c_uint = 0x10;
pub const INT_SOFT_CLEAR: c_uint = 0x14;
pub const FIQ_STATUS: c_uint = 0x20;
pub const FIQ_RAW_STATUS: c_uint = 0x24;
pub const FIQ_ENABLE: c_uint = 0x28;
pub const FIQ_ENABLE_SET: c_uint = 0x28;
pub const FIQ_ENABLE_CLEAR: c_uint = 0x2C;
pub const PIC_ENABLES: c_uint = 0x20	/* set interrupt pass through bits */;
//
// struct fpga_irq_data - irq data container for the FPGA IRQ controller
// @base: memory offset in virtual memory
// @domain: IRQ domain for this instance
// @valid: mask for valid IRQs on this controller
// @used_irqs: number of active IRQs on this controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_irq_data {
    pub base: *mut void __iomem,
    pub valid: u32,
    pub domain: *mut irq_domain,
    pub used_irqs: u8,
}

// we cannot allocate memory when the controllers are initially registered
    static struct fpga_irq_data fpga_irq_devices[CONFIG_VERSATILE_FPGA_IRQ_NR];
    static int fpga_irq_id;
#[no_mangle]
unsafe extern "C" fn fpga_irq_mask(d: *mut irq_data) {
    static void fpga_irq_mask(struct irq_data *d)
    {
    struct fpga_irq_data *f = irq_data_get_irq_chip_data(d);
    let mut mask: u32 = 1 << d.hwirq;
    writel(mask, f.base + IRQ_ENABLE_CLEAR);
    }
#[no_mangle]
unsafe extern "C" fn fpga_irq_unmask(d: *mut irq_data) {
    static void fpga_irq_unmask(struct irq_data *d)
    {
    struct fpga_irq_data *f = irq_data_get_irq_chip_data(d);
    let mut mask: u32 = 1 << d.hwirq;
    writel(mask, f.base + IRQ_ENABLE_SET);
    }
#[no_mangle]
unsafe extern "C" fn fpga_irq_print_chip(d: *mut irq_data, p: *mut seq_file) {
    static void fpga_irq_print_chip(struct irq_data *d, struct seq_file *p)
    {
    struct fpga_irq_data *f = irq_data_get_irq_chip_data(d);
    seq_puts(p, irq_domain_get_of_node(f.domain).name);
    }
    static const struct irq_chip fpga_chip = {
    .irq_ack	= fpga_irq_mask,
    .irq_mask	= fpga_irq_mask,
    .irq_unmask	= fpga_irq_unmask,
    .irq_print_chip	= fpga_irq_print_chip,
    };
#[no_mangle]
unsafe extern "C" fn fpga_irq_handle(desc: *mut irq_desc) {
    static void fpga_irq_handle(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    struct fpga_irq_data *f = irq_desc_get_handler_data(desc);
    u32 status;
    chained_irq_enter(chip, desc);
    status = readl(f.base + IRQ_STATUS);
    if (status == 0) {
    handle_bad_irq(desc);
    goto out;
    }
    do {
    let mut irq: c_uint = ffs(status) - 1;
    status &= ~(1 << irq);
    generic_handle_domain_irq(f.domain, irq);
    } while (status);
    out:
    chained_irq_exit(chip, desc);
    }
//
// Handle each interrupt in a single FPGA IRQ controller.  Returns non-zero
// if we've handled at least one interrupt.  This does a single read of the
// status register and handles all interrupts in order from LSB first.
//
#[no_mangle]
unsafe extern "C" fn handle_one_fpga(f: *mut fpga_irq_data, regs: *mut pt_regs) -> c_int {
    static int handle_one_fpga(struct fpga_irq_data *f, struct pt_regs *regs)
    {
    let mut handled: c_int = 0;
    int irq;
    u32 status;
    while ((status  = readl(f.base + IRQ_STATUS))) {
    irq = ffs(status) - 1;
    generic_handle_domain_irq(f.domain, irq);
    handled = 1;
    }
    return handled;
    }
//
// Keep iterating over all registered FPGA IRQ controllers until there are
// no pending interrupts.
//
#[no_mangle]
unsafe extern "C" fn fpga_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry fpga_handle_irq(struct pt_regs *regs)
    {
    int i, handled;
    do {
    for (i = 0, handled = 0; i < fpga_irq_id; ++i)
    handled |= handle_one_fpga(&fpga_irq_devices[i], regs);
    } while (handled);
    }
    static int fpga_irqdomain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    struct fpga_irq_data *f = d.host_data;
// Skip invalid IRQs, only register handlers for the real ones
    if (!(f.valid & BIT(hwirq)))
    return -EPERM;
    irq_set_chip_data(irq, f);
    irq_set_chip_and_handler(irq, &fpga_chip, handle_level_irq);
    irq_set_probe(irq);
    return 0;
    }
    static const struct irq_domain_ops fpga_irqdomain_ops = {
    .map = fpga_irqdomain_map,
    .xlate = irq_domain_xlate_onetwocell,
    };
    static void __init fpga_irq_init(void __iomem *base, int parent_irq,
    u32 valid, struct device_node *node)
    {
    struct fpga_irq_data *f;
    int i;
    if (fpga_irq_id >= ARRAY_SIZE(fpga_irq_devices)) {
    pr_err("%s: too few FPGA IRQ controllers, increase CONFIG_VERSATILE_FPGA_IRQ_NR\n", __func__);
    return;
    }
    f = &fpga_irq_devices[fpga_irq_id];
    f.base = base;
    f.valid = valid;
    if (parent_irq != -1) {
    irq_set_chained_handler_and_data(parent_irq, fpga_irq_handle,
    f);
    }
    f.domain = irq_domain_create_linear(of_fwnode_handle(node), fls(valid),
    &fpga_irqdomain_ops, f);
// This will allocate all valid descriptors in the linear case
    for (i = 0; i < fls(valid); i++)
    if (valid & BIT(i)) {
// Is this still required?
    irq_create_mapping(f.domain, i);
    f.used_irqs++;
    }
    pr_info("FPGA IRQ chip %d \"%s\" @ %p, %u irqs",
    fpga_irq_id, node.name, base, f.used_irqs);
    if (parent_irq != -1)
    pr_cont(", parent IRQ: %d\n", parent_irq);
    else
    pr_cont("\n");
    fpga_irq_id++;
    }

    static int __init fpga_irq_of_init(struct device_node *node,
    struct device_node *parent)
    {
    void __iomem *base;
    u32 clear_mask;
    u32 valid_mask;
    int parent_irq;
    if (WARN_ON(!node))
    return -ENODEV;
    base = of_iomap(node, 0);
    WARN(!base, "unable to map fpga irq registers\n");
    if (of_property_read_u32(node, "clear-mask", &clear_mask))
    clear_mask = 0;
    if (of_property_read_u32(node, "valid-mask", &valid_mask))
    valid_mask = 0;
    writel(clear_mask, base + IRQ_ENABLE_CLEAR);
    writel(clear_mask, base + FIQ_ENABLE_CLEAR);
// Some chips are cascaded from a parent IRQ
    parent_irq = irq_of_parse_and_map(node, 0);
    if (!parent_irq) {
    set_handle_irq(fpga_handle_irq);
    parent_irq = -1;
    }
    fpga_irq_init(base, parent_irq, valid_mask, node);
//
// On Versatile AB/PB, some secondary interrupts have a direct
// pass-thru to the primary controller for IRQs 20 and 22-31 which need
// to be enabled. See section 3.10 of the Versatile AB user guide.
//
    if (of_device_is_compatible(node, "arm,versatile-sic"))
    writel(0xffd00000, base + PIC_ENABLES);
    return 0;
    }
    IRQCHIP_DECLARE(arm_fpga, "arm,versatile-fpga-irq", fpga_irq_of_init);
    IRQCHIP_DECLARE(arm_fpga_sic, "arm,versatile-sic", fpga_irq_of_init);
