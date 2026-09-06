//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-mscc-ocelot.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Microsemi Ocelot IRQ controller driver
//
// Copyright (c) 2017 Microsemi Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chip_props {
    pub flags: u8,
    pub reg_off_sticky: u8,
    pub reg_off_ena: u8,
    pub reg_off_ena_clr: u8,
    pub reg_off_ena_set: u8,
    pub reg_off_ident: u8,
    pub reg_off_trigger: u8,
    pub reg_off_ena_irq0: u8,
    pub n_irq: u8,
}

    static struct chip_props ocelot_props = {
    .flags			= FLAGS_HAS_TRIGGER,
    .reg_off_sticky		= 0x10,
    .reg_off_ena		= 0x18,
    .reg_off_ena_clr	= 0x1c,
    .reg_off_ena_set	= 0x20,
    .reg_off_ident		= 0x38,
    .reg_off_trigger	= 0x4,
    .n_irq			= 24,
    };
    static struct chip_props serval_props = {
    .flags			= FLAGS_HAS_TRIGGER,
    .reg_off_sticky		= 0xc,
    .reg_off_ena		= 0x14,
    .reg_off_ena_clr	= 0x18,
    .reg_off_ena_set	= 0x1c,
    .reg_off_ident		= 0x20,
    .reg_off_trigger	= 0x4,
    .n_irq			= 24,
    };
    static struct chip_props luton_props = {
    .flags			= FLAGS_NEED_INIT_ENABLE,
    .reg_off_sticky		= 0,
    .reg_off_ena		= 0x4,
    .reg_off_ena_clr	= 0x8,
    .reg_off_ena_set	= 0xc,
    .reg_off_ident		= 0x18,
    .reg_off_ena_irq0	= 0x14,
    .n_irq			= 28,
    };
    static struct chip_props jaguar2_props = {
    .flags			= FLAGS_HAS_TRIGGER,
    .reg_off_sticky		= 0x10,
    .reg_off_ena		= 0x18,
    .reg_off_ena_clr	= 0x1c,
    .reg_off_ena_set	= 0x20,
    .reg_off_ident		= 0x38,
    .reg_off_trigger	= 0x4,
    .n_irq			= 29,
    };
#[no_mangle]
unsafe extern "C" fn ocelot_irq_unmask(data: *mut irq_data) {
    static void ocelot_irq_unmask(struct irq_data *data)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(data);
    struct irq_domain *d = data.domain;
    struct chip_props *p = d.host_data;
    struct irq_chip_type *ct = irq_data_get_chip_type(data);
    let mut mask: c_uint = data.mask;
    u32 val;
    guard(raw_spinlock)(&gc.lock);
//
// Clear sticky bits for edge mode interrupts.
// Serval has only one trigger register replication, but the adjacent
// register is always read as zero, so there's no need to handle this
// case separately.
//
    val = irq_reg_readl(gc, ICPU_CFG_INTR_INTR_TRIGGER(p, 0)) |
    irq_reg_readl(gc, ICPU_CFG_INTR_INTR_TRIGGER(p, 1));
    if (!(val & mask))
    irq_reg_writel(gc, mask, p.reg_off_sticky);
// ct->mask_cache &= ~mask;
    irq_reg_writel(gc, mask, p.reg_off_ena_set);
    }
#[no_mangle]
unsafe extern "C" fn ocelot_irq_handler(desc: *mut irq_desc) {
    static void ocelot_irq_handler(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    struct irq_domain *d = irq_desc_get_handler_data(desc);
    struct chip_props *p = d.host_data;
    struct irq_chip_generic *gc = irq_get_domain_generic_chip(d, 0);
    let mut reg: u32 = irq_reg_readl(gc, ICPU_CFG_INTR_DST_INTR_IDENT(p, 0));
    chained_irq_enter(chip, desc);
    while (reg) {
    let mut hwirq: u32 = __fls(reg);
    generic_handle_domain_irq(d, hwirq);
    reg &= ~(BIT(hwirq));
    }
    chained_irq_exit(chip, desc);
    }
    static int __init vcoreiii_irq_init(struct device_node *node,
    struct device_node *parent,
    struct chip_props *p)
    {
    struct irq_domain *domain;
    struct irq_chip_generic *gc;
    int parent_irq, ret;
    parent_irq = irq_of_parse_and_map(node, 0);
    if (!parent_irq)
    return -EINVAL;
    domain = irq_domain_create_linear(of_fwnode_handle(node), p.n_irq,
    &irq_generic_chip_ops, core::ptr::null_mut());
    if (!domain) {
    pr_err("%pOFn: unable to add irq domain\n", node);
    return -ENOMEM;
    }
    ret = irq_alloc_domain_generic_chips(domain, p.n_irq, 1,
    "icpu", handle_level_irq,
    0, 0, 0);
    if (ret) {
    pr_err("%pOFn: unable to alloc irq domain gc\n", node);
    goto err_domain_remove;
    }
    gc = irq_get_domain_generic_chip(domain, 0);
    gc.reg_base = of_iomap(node, 0);
    if (!gc.reg_base) {
    pr_err("%pOFn: unable to map resource\n", node);
    ret = -ENOMEM;
    goto err_gc_free;
    }
    gc.chip_types[0].chip.irq_ack = irq_gc_ack_set_bit;
    gc.chip_types[0].regs.ack = p.reg_off_sticky;
    if (p.flags & FLAGS_HAS_TRIGGER) {
    gc.chip_types[0].regs.mask = p.reg_off_ena_clr;
    gc.chip_types[0].chip.irq_unmask = ocelot_irq_unmask;
    gc.chip_types[0].chip.irq_mask = irq_gc_mask_set_bit;
    } else {
    gc.chip_types[0].regs.enable = p.reg_off_ena_set;
    gc.chip_types[0].regs.disable = p.reg_off_ena_clr;
    gc.chip_types[0].chip.irq_mask = irq_gc_mask_disable_reg;
    gc.chip_types[0].chip.irq_unmask = irq_gc_unmask_enable_reg;
    }
// Mask and ack all interrupts
    irq_reg_writel(gc, 0, p.reg_off_ena);
    irq_reg_writel(gc, 0xffffffff, p.reg_off_sticky);
// Overall init
    if (p.flags & FLAGS_NEED_INIT_ENABLE)
    irq_reg_writel(gc, BIT(0), p.reg_off_ena_irq0);
    domain.host_data = p;
    irq_set_chained_handler_and_data(parent_irq, ocelot_irq_handler,
    domain);
    return 0;
    err_gc_free:
    irq_free_generic_chip(gc);
    err_domain_remove:
    irq_domain_remove(domain);
    return ret;
    }
    static int __init ocelot_irq_init(struct device_node *node,
    struct device_node *parent)
    {
    return vcoreiii_irq_init(node, parent, &ocelot_props);
    }
    IRQCHIP_DECLARE(ocelot_icpu, "mscc,ocelot-icpu-intr", ocelot_irq_init);
    static int __init serval_irq_init(struct device_node *node,
    struct device_node *parent)
    {
    return vcoreiii_irq_init(node, parent, &serval_props);
    }
    IRQCHIP_DECLARE(serval_icpu, "mscc,serval-icpu-intr", serval_irq_init);
    static int __init luton_irq_init(struct device_node *node,
    struct device_node *parent)
    {
    return vcoreiii_irq_init(node, parent, &luton_props);
    }
    IRQCHIP_DECLARE(luton_icpu, "mscc,luton-icpu-intr", luton_irq_init);
    static int __init jaguar2_irq_init(struct device_node *node,
    struct device_node *parent)
    {
    return vcoreiii_irq_init(node, parent, &jaguar2_props);
    }
    IRQCHIP_DECLARE(jaguar2_icpu, "mscc,jaguar2-icpu-intr", jaguar2_irq_init);
