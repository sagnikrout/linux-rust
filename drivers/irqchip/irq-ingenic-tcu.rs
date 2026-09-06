//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ingenic-tcu.c
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
// JZ47xx SoCs TCU IRQ driver
// Copyright (C) 2019 Paul Cercueil <paul@crapouillou.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_tcu {
    pub map: *mut regmap,
    pub clk: *mut clk,
    pub domain: *mut irq_domain,
    pub nb_parent_irqs: c_uint,
    pub parent_irqs: [u32; 3],
}

#[no_mangle]
unsafe extern "C" fn ingenic_tcu_intc_cascade(desc: *mut irq_desc) {
    static void ingenic_tcu_intc_cascade(struct irq_desc *desc)
    {
    struct irq_chip *irq_chip = irq_data_get_irq_chip(&desc.irq_data);
    struct irq_domain *domain = irq_desc_get_handler_data(desc);
    struct irq_chip_generic *gc = irq_get_domain_generic_chip(domain, 0);
    struct regmap *map = gc.private;
    uint32_t irq_reg, irq_mask;
    unsigned long bits;
    unsigned int i;
    regmap_read(map, TCU_REG_TFR, &irq_reg);
    regmap_read(map, TCU_REG_TMR, &irq_mask);
    chained_irq_enter(irq_chip, desc);
    irq_reg &= ~irq_mask;
    bits = irq_reg;
    for_each_set_bit(i, &bits, 32)
    generic_handle_domain_irq(domain, i);
    chained_irq_exit(irq_chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_tcu_gc_unmask_enable_reg(d: *mut irq_data) {
    static void ingenic_tcu_gc_unmask_enable_reg(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct irq_chip_type *ct = irq_data_get_chip_type(d);
    struct regmap *map = gc.private;
    let mut mask: u32 = d.mask;
    guard(raw_spinlock)(&gc.lock);
    regmap_write(map, ct.regs.ack, mask);
    regmap_write(map, ct.regs.enable, mask);
// ct->mask_cache |= mask;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_tcu_gc_mask_disable_reg(d: *mut irq_data) {
    static void ingenic_tcu_gc_mask_disable_reg(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct irq_chip_type *ct = irq_data_get_chip_type(d);
    struct regmap *map = gc.private;
    let mut mask: u32 = d.mask;
    guard(raw_spinlock)(&gc.lock);
    regmap_write(map, ct.regs.disable, mask);
// ct->mask_cache &= ~mask;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_tcu_gc_mask_disable_reg_and_ack(d: *mut irq_data) {
    static void ingenic_tcu_gc_mask_disable_reg_and_ack(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct irq_chip_type *ct = irq_data_get_chip_type(d);
    struct regmap *map = gc.private;
    let mut mask: u32 = d.mask;
    guard(raw_spinlock)(&gc.lock);
    regmap_write(map, ct.regs.ack, mask);
    regmap_write(map, ct.regs.disable, mask);
    }
    static int __init ingenic_tcu_irq_init(struct device_node *np,
    struct device_node *parent)
    {
    struct irq_chip_generic *gc;
    struct irq_chip_type *ct;
    struct ingenic_tcu *tcu;
    struct regmap *map;
    unsigned int i;
    int ret, irqs;
    map = device_node_to_regmap(np);
    if (IS_ERR(map))
    return PTR_ERR(map);
    tcu = kzalloc_obj(*tcu);
    if (!tcu)
    return -ENOMEM;
    tcu.map = map;
    irqs = of_property_count_elems_of_size(np, "interrupts", sizeof(u32));
    if (irqs < 0 || irqs > ARRAY_SIZE(tcu.parent_irqs)) {
    pr_crit("%s: Invalid 'interrupts' property\n", __func__);
    ret = -EINVAL;
    goto err_free_tcu;
    }
    tcu.nb_parent_irqs = irqs;
    tcu.domain = irq_domain_create_linear(of_fwnode_handle(np), 32, &irq_generic_chip_ops,
    core::ptr::null_mut());
    if (!tcu.domain) {
    ret = -ENOMEM;
    goto err_free_tcu;
    }
    ret = irq_alloc_domain_generic_chips(tcu.domain, 32, 1, "TCU",
    handle_level_irq, 0,
    IRQ_NOPROBE | IRQ_LEVEL, 0);
    if (ret) {
    pr_crit("%s: Invalid 'interrupts' property\n", __func__);
    goto out_domain_remove;
    }
    gc = irq_get_domain_generic_chip(tcu.domain, 0);
    ct = gc.chip_types;
    gc.wake_enabled = IRQ_MSK(32);
    gc.private = tcu.map;
    ct.regs.disable = TCU_REG_TMSR;
    ct.regs.enable = TCU_REG_TMCR;
    ct.regs.ack = TCU_REG_TFCR;
    ct.chip.irq_unmask = ingenic_tcu_gc_unmask_enable_reg;
    ct.chip.irq_mask = ingenic_tcu_gc_mask_disable_reg;
    ct.chip.irq_mask_ack = ingenic_tcu_gc_mask_disable_reg_and_ack;
    ct.chip.flags = IRQCHIP_MASK_ON_SUSPEND | IRQCHIP_SKIP_SET_WAKE;
// Mask all IRQs by default
    regmap_write(tcu.map, TCU_REG_TMSR, IRQ_MSK(32));
//
// On JZ4740, timer 0 and timer 1 have their own interrupt line;
// timers 2-7 share one interrupt.
// On SoCs >= JZ4770, timer 5 has its own interrupt line;
// timers 0-4 and 6-7 share one single interrupt.
//
// To keep things simple, we just register the same handler to
// all parent interrupts. The handler will properly detect which
// channel fired the interrupt.
//
    for (i = 0; i < irqs; i++) {
    tcu.parent_irqs[i] = irq_of_parse_and_map(np, i);
    if (!tcu.parent_irqs[i]) {
    ret = -EINVAL;
    goto out_unmap_irqs;
    }
    irq_set_chained_handler_and_data(tcu.parent_irqs[i],
    ingenic_tcu_intc_cascade,
    tcu.domain);
    }
    return 0;
    out_unmap_irqs:
    for (; i > 0; i--)
    irq_dispose_mapping(tcu.parent_irqs[i - 1]);
    out_domain_remove:
    irq_domain_remove(tcu.domain);
    err_free_tcu:
    kfree(tcu);
    return ret;
    }
    IRQCHIP_DECLARE(jz4740_tcu_irq, "ingenic,jz4740-tcu", ingenic_tcu_irq_init);
    IRQCHIP_DECLARE(jz4725b_tcu_irq, "ingenic,jz4725b-tcu", ingenic_tcu_irq_init);
    IRQCHIP_DECLARE(jz4760_tcu_irq, "ingenic,jz4760-tcu", ingenic_tcu_irq_init);
    IRQCHIP_DECLARE(jz4770_tcu_irq, "ingenic,jz4770-tcu", ingenic_tcu_irq_init);
    IRQCHIP_DECLARE(x1000_tcu_irq, "ingenic,x1000-tcu", ingenic_tcu_irq_init);
