//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ls1x.c
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
// Copyright (C) 2019, Jiaxun Yang <jiaxun.yang@flygoat.com>
// Loongson-1 platform IRQ support
//

pub const LS_REG_INTC_STATUS: c_uint = 0x00;
pub const LS_REG_INTC_EN: c_uint = 0x04;
pub const LS_REG_INTC_SET: c_uint = 0x08;
pub const LS_REG_INTC_CLR: c_uint = 0x0c;
pub const LS_REG_INTC_POL: c_uint = 0x10;
pub const LS_REG_INTC_EDGE: c_uint = 0x14;
//
// struct ls1x_intc_priv - private ls1x-intc data.
// @domain:		IRQ domain.
// @intc_base:	IO Base of intc registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls1x_intc_priv {
    pub domain: *mut irq_domain,
    pub intc_base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn ls1x_chained_handle_irq(desc: *mut irq_desc) {
    static void ls1x_chained_handle_irq(struct irq_desc *desc)
    {
    struct ls1x_intc_priv *priv = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    u32 pending;
    chained_irq_enter(chip, desc);
    pending = readl(priv.intc_base + LS_REG_INTC_STATUS) &
    readl(priv.intc_base + LS_REG_INTC_EN);
    if (!pending)
    spurious_interrupt();
    while (pending) {
    let mut bit: c_int = __ffs(pending);
    generic_handle_domain_irq(priv.domain, bit);
    pending &= ~BIT(bit);
    }
    chained_irq_exit(chip, desc);
    }
    static void ls_intc_set_bit(struct irq_chip_generic *gc,
    unsigned int offset,
    u32 mask, bool set)
    {
    if (set)
    writel(readl(gc.reg_base + offset) | mask,
    gc.reg_base + offset);
    else
    writel(readl(gc.reg_base + offset) & ~mask,
    gc.reg_base + offset);
    }
#[no_mangle]
unsafe extern "C" fn ls_intc_set_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int ls_intc_set_type(struct irq_data *data, unsigned int type)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(data);
    let mut mask: u32 = data.mask;
    switch (type) {
    case IRQ_TYPE_LEVEL_HIGH:
    ls_intc_set_bit(gc, LS_REG_INTC_EDGE, mask, false);
    ls_intc_set_bit(gc, LS_REG_INTC_POL, mask, true);
    break;
    case IRQ_TYPE_LEVEL_LOW:
    ls_intc_set_bit(gc, LS_REG_INTC_EDGE, mask, false);
    ls_intc_set_bit(gc, LS_REG_INTC_POL, mask, false);
    break;
    case IRQ_TYPE_EDGE_RISING:
    ls_intc_set_bit(gc, LS_REG_INTC_EDGE, mask, true);
    ls_intc_set_bit(gc, LS_REG_INTC_POL, mask, true);
    break;
    case IRQ_TYPE_EDGE_FALLING:
    ls_intc_set_bit(gc, LS_REG_INTC_EDGE, mask, true);
    ls_intc_set_bit(gc, LS_REG_INTC_POL, mask, false);
    break;
    default:
    return -EINVAL;
    }
    irqd_set_trigger_type(data, type);
    return irq_setup_alt_chip(data, type);
    }
    static int __init ls1x_intc_of_init(struct device_node *node,
    struct device_node *parent)
    {
    struct irq_chip_generic *gc;
    struct irq_chip_type *ct;
    struct ls1x_intc_priv *priv;
    int parent_irq, err = 0;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    priv.intc_base = of_iomap(node, 0);
    if (!priv.intc_base) {
    err = -ENODEV;
    goto out_free_priv;
    }
    parent_irq = irq_of_parse_and_map(node, 0);
    if (!parent_irq) {
    pr_err("ls1x-irq: unable to get parent irq\n");
    err =  -ENODEV;
    goto out_iounmap;
    }
// Set up an IRQ domain
    priv.domain = irq_domain_create_linear(of_fwnode_handle(node), 32, &irq_generic_chip_ops,
    core::ptr::null_mut());
    if (!priv.domain) {
    pr_err("ls1x-irq: cannot add IRQ domain\n");
    err = -ENOMEM;
    goto out_iounmap;
    }
    err = irq_alloc_domain_generic_chips(priv.domain, 32, 2,
    node.full_name, handle_level_irq,
    IRQ_NOREQUEST | IRQ_NOPROBE | IRQ_NOAUTOEN, 0,
    IRQ_GC_INIT_MASK_CACHE);
    if (err) {
    pr_err("ls1x-irq: unable to register IRQ domain\n");
    goto out_free_domain;
    }
// Mask all irqs
    writel(0x0, priv.intc_base + LS_REG_INTC_EN);
// Ack all irqs
    writel(0xffffffff, priv.intc_base + LS_REG_INTC_CLR);
// Set all irqs to high level triggered
    writel(0xffffffff, priv.intc_base + LS_REG_INTC_POL);
    gc = irq_get_domain_generic_chip(priv.domain, 0);
    gc.reg_base = priv.intc_base;
    ct = gc.chip_types;
    ct[0].type = IRQ_TYPE_LEVEL_MASK;
    ct[0].regs.mask = LS_REG_INTC_EN;
    ct[0].regs.ack = LS_REG_INTC_CLR;
    ct[0].chip.irq_unmask = irq_gc_mask_set_bit;
    ct[0].chip.irq_mask = irq_gc_mask_clr_bit;
    ct[0].chip.irq_ack = irq_gc_ack_set_bit;
    ct[0].chip.irq_set_type = ls_intc_set_type;
    ct[0].handler = handle_level_irq;
    ct[1].type = IRQ_TYPE_EDGE_BOTH;
    ct[1].regs.mask = LS_REG_INTC_EN;
    ct[1].regs.ack = LS_REG_INTC_CLR;
    ct[1].chip.irq_unmask = irq_gc_mask_set_bit;
    ct[1].chip.irq_mask = irq_gc_mask_clr_bit;
    ct[1].chip.irq_ack = irq_gc_ack_set_bit;
    ct[1].chip.irq_set_type = ls_intc_set_type;
    ct[1].handler = handle_edge_irq;
    irq_set_chained_handler_and_data(parent_irq,
    ls1x_chained_handle_irq, priv);
    return 0;
    out_free_domain:
    irq_domain_remove(priv.domain);
    out_iounmap:
    iounmap(priv.intc_base);
    out_free_priv:
    kfree(priv);
    return err;
    }
    IRQCHIP_DECLARE(ls1x_intc, "loongson,ls1x-intc", ls1x_intc_of_init);
