//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ingenic.c
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
// Copyright (C) 2009-2010, Lars-Peter Clausen <lars@metafoo.de>
// Ingenic XBurst platform IRQ support
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_intc_data {
    pub base: *mut void __iomem,
    pub domain: *mut irq_domain,
    pub num_chips: unsigned,
}

pub const JZ_REG_INTC_STATUS: c_uint = 0x00;
pub const JZ_REG_INTC_MASK: c_uint = 0x04;
pub const JZ_REG_INTC_SET_MASK: c_uint = 0x08;
pub const JZ_REG_INTC_CLEAR_MASK: c_uint = 0x0c;
pub const JZ_REG_INTC_PENDING: c_uint = 0x10;
pub const CHIP_SIZE: c_uint = 0x20;
#[no_mangle]
unsafe extern "C" fn intc_cascade(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t intc_cascade(int irq, void *data)
    {
    struct ingenic_intc_data *intc = irq_get_handler_data(irq);
    struct irq_domain *domain = intc.domain;
    struct irq_chip_generic *gc;
    uint32_t pending;
    unsigned i;
    for (i = 0; i < intc.num_chips; i++) {
    gc = irq_get_domain_generic_chip(domain, i * 32);
    pending = irq_reg_readl(gc, JZ_REG_INTC_PENDING);
    if (!pending)
    continue;
    while (pending) {
    let mut bit: c_int = __fls(pending);
    generic_handle_domain_irq(domain, bit + (i * 32));
    pending &= ~BIT(bit);
    }
    }
    return IRQ_HANDLED;
    }
    static int __init ingenic_intc_of_init(struct device_node *node,
    unsigned num_chips)
    {
    struct ingenic_intc_data *intc;
    struct irq_chip_generic *gc;
    struct irq_chip_type *ct;
    struct irq_domain *domain;
    int parent_irq, err = 0;
    unsigned i;
    intc = kzalloc_obj(*intc);
    if (!intc) {
    err = -ENOMEM;
    goto out_err;
    }
    parent_irq = irq_of_parse_and_map(node, 0);
    if (!parent_irq) {
    err = -EINVAL;
    goto out_free;
    }
    err = irq_set_handler_data(parent_irq, intc);
    if (err)
    goto out_unmap_irq;
    intc.num_chips = num_chips;
    intc.base = of_iomap(node, 0);
    if (!intc.base) {
    err = -ENODEV;
    goto out_unmap_irq;
    }
    domain = irq_domain_create_linear(of_fwnode_handle(node), num_chips * 32,
    &irq_generic_chip_ops, core::ptr::null_mut());
    if (!domain) {
    err = -ENOMEM;
    goto out_unmap_base;
    }
    intc.domain = domain;
    err = irq_alloc_domain_generic_chips(domain, 32, 1, "INTC",
    handle_level_irq, 0,
    IRQ_NOPROBE | IRQ_LEVEL, 0);
    if (err)
    goto out_domain_remove;
    for (i = 0; i < num_chips; i++) {
    gc = irq_get_domain_generic_chip(domain, i * 32);
    gc.wake_enabled = IRQ_MSK(32);
    gc.reg_base = intc.base + (i * CHIP_SIZE);
    ct = gc.chip_types;
    ct.regs.enable = JZ_REG_INTC_CLEAR_MASK;
    ct.regs.disable = JZ_REG_INTC_SET_MASK;
    ct.chip.irq_unmask = irq_gc_unmask_enable_reg;
    ct.chip.irq_mask = irq_gc_mask_disable_reg;
    ct.chip.irq_mask_ack = irq_gc_mask_disable_reg;
    ct.chip.irq_set_wake = irq_gc_set_wake;
    ct.chip.flags = IRQCHIP_MASK_ON_SUSPEND;
// Mask all irqs
    irq_reg_writel(gc, IRQ_MSK(32), JZ_REG_INTC_SET_MASK);
    }
    if (request_irq(parent_irq, intc_cascade, IRQF_NO_SUSPEND,
    "SoC intc cascade interrupt", core::ptr::null_mut()))
    pr_err("Failed to register SoC intc cascade interrupt\n");
    return 0;
    out_domain_remove:
    irq_domain_remove(domain);
    out_unmap_base:
    iounmap(intc.base);
    out_unmap_irq:
    irq_dispose_mapping(parent_irq);
    out_free:
    kfree(intc);
    out_err:
    return err;
    }
    static int __init intc_1chip_of_init(struct device_node *node,
    struct device_node *parent)
    {
    return ingenic_intc_of_init(node, 1);
    }
    IRQCHIP_DECLARE(jz4740_intc, "ingenic,jz4740-intc", intc_1chip_of_init);
    IRQCHIP_DECLARE(jz4725b_intc, "ingenic,jz4725b-intc", intc_1chip_of_init);
    static int __init intc_2chip_of_init(struct device_node *node,
    struct device_node *parent)
    {
    return ingenic_intc_of_init(node, 2);
    }
    IRQCHIP_DECLARE(jz4760_intc, "ingenic,jz4760-intc", intc_2chip_of_init);
    IRQCHIP_DECLARE(jz4770_intc, "ingenic,jz4770-intc", intc_2chip_of_init);
    IRQCHIP_DECLARE(jz4775_intc, "ingenic,jz4775-intc", intc_2chip_of_init);
    IRQCHIP_DECLARE(jz4780_intc, "ingenic,jz4780-intc", intc_2chip_of_init);
