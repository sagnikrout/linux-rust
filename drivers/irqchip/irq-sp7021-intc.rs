//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-sp7021-intc.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) Sunplus Technology Co., Ltd.
// All rights reserved.
//

pub const SP_INTC_HWIRQ_MIN: c_int = 0;
pub const SP_INTC_HWIRQ_MAX: c_int = 223;

// REG_GROUP_0 regs

// REG_GROUP_1 regs

//
// When GPIO_INT0~7 set to edge trigger, doesn't work properly.
// WORKAROUND: change it to level trigger, and toggle the polarity
// at ACK/Handler to make the HW work.
//
pub const GPIO_INT0_HWIRQ: c_int = 120;
pub const GPIO_INT7_HWIRQ: c_int = 127;

    ({								\
    u32 i = irq;						\
    (i >= GPIO_INT0_HWIRQ) && (i <= GPIO_INT7_HWIRQ);	\
    })
// index of states
    enum {
    _IS_EDGE = 0,
    _IS_LOW,
    _IS_ACTIVE
    };

    static struct sp_intctl {
//
// REG_GROUP_0: include type/polarity/priority/mask regs.
// REG_GROUP_1: include clear/masked_ext0/masked_ext1/group regs.
//
    void __iomem *g0; // REG_GROUP_0 base
    void __iomem *g1; // REG_GROUP_1 base
    struct irq_domain *domain;
    raw_spinlock_t lock;
//
// store GPIO_INT states
// each interrupt has 3 states: is_edge, is_low, is_active
//
    DECLARE_BITMAP(states, (GPIO_INT7_HWIRQ - GPIO_INT0_HWIRQ + 1) * 3);
    } sp_intc;
    static struct irq_chip sp_intc_chip;
#[no_mangle]
unsafe extern "C" fn sp_intc_assign_bit(hwirq: u32, base: *mut void __iomem, value: bool) {
    static void sp_intc_assign_bit(u32 hwirq, void __iomem *base, bool value)
    {
    u32 offset, mask;
    unsigned long flags;
    void __iomem *reg;
    offset = (hwirq / 32) * 4;
    reg = base + offset;
    raw_spin_lock_irqsave(&sp_intc.lock, flags);
    mask = readl_relaxed(reg);
    if (value)
    mask |= BIT(hwirq % 32);
    else
    mask &= ~BIT(hwirq % 32);
    writel_relaxed(mask, reg);
    raw_spin_unlock_irqrestore(&sp_intc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sp_intc_ack_irq(d: *mut irq_data) {
    static void sp_intc_ack_irq(struct irq_data *d)
    {
    let mut hwirq: u32 = d.hwirq;
    if (unlikely(IS_GPIO_INT(hwirq) && TEST_STATE(hwirq, _IS_EDGE))) { // WORKAROUND
    sp_intc_assign_bit(hwirq, REG_INTR_POLARITY, !TEST_STATE(hwirq, _IS_LOW));
    ASSIGN_STATE(hwirq, _IS_ACTIVE, true);
    }
    sp_intc_assign_bit(hwirq, REG_INTR_CLEAR, 1);
    }
#[no_mangle]
unsafe extern "C" fn sp_intc_mask_irq(d: *mut irq_data) {
    static void sp_intc_mask_irq(struct irq_data *d)
    {
    sp_intc_assign_bit(d.hwirq, REG_INTR_MASK, 0);
    }
#[no_mangle]
unsafe extern "C" fn sp_intc_unmask_irq(d: *mut irq_data) {
    static void sp_intc_unmask_irq(struct irq_data *d)
    {
    sp_intc_assign_bit(d.hwirq, REG_INTR_MASK, 1);
    }
#[no_mangle]
unsafe extern "C" fn sp_intc_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int sp_intc_set_type(struct irq_data *d, unsigned int type)
    {
    let mut hwirq: u32 = d.hwirq;
    let mut is_edge: bool = !(type & IRQ_TYPE_LEVEL_MASK);
    let mut is_low: bool = (type == IRQ_TYPE_LEVEL_LOW || type == IRQ_TYPE_EDGE_FALLING);
    irq_set_handler_locked(d, is_edge ? handle_edge_irq : handle_level_irq);
    if (unlikely(IS_GPIO_INT(hwirq) && is_edge)) { // WORKAROUND
// store states
    ASSIGN_STATE(hwirq, _IS_EDGE, is_edge);
    ASSIGN_STATE(hwirq, _IS_LOW, is_low);
    ASSIGN_STATE(hwirq, _IS_ACTIVE, false);
// change to level
    is_edge = false;
    }
    sp_intc_assign_bit(hwirq, REG_INTR_TYPE, is_edge);
    sp_intc_assign_bit(hwirq, REG_INTR_POLARITY, is_low);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_intc_get_ext_irq(ext_num: c_int) -> c_int {
    static int sp_intc_get_ext_irq(int ext_num)
    {
    void __iomem *base = ext_num ? REG_MASKED_EXT1 : REG_MASKED_EXT0;
    let mut shift: u32 = ext_num ? GROUP_SHIFT_EXT1 : GROUP_SHIFT_EXT0;
    u32 groups;
    u32 pending_group;
    u32 group;
    u32 pending_irq;
    groups = readl_relaxed(REG_INTR_GROUP);
    pending_group = (groups >> shift) & GROUP_MASK;
    if (!pending_group)
    return -1;
    group = fls(pending_group) - 1;
    pending_irq = readl_relaxed(base + group * 4);
    if (!pending_irq)
    return -1;
    return (group * 32) + fls(pending_irq) - 1;
    }
#[no_mangle]
unsafe extern "C" fn sp_intc_handle_ext_cascaded(desc: *mut irq_desc) {
    static void sp_intc_handle_ext_cascaded(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    let mut ext_num: c_int = (uintptr_t)irq_desc_get_handler_data(desc);
    int hwirq;
    chained_irq_enter(chip, desc);
    while ((hwirq = sp_intc_get_ext_irq(ext_num)) >= 0) {
    if (unlikely(IS_GPIO_INT(hwirq) && TEST_STATE(hwirq, _IS_ACTIVE))) { // WORKAROUND
    ASSIGN_STATE(hwirq, _IS_ACTIVE, false);
    sp_intc_assign_bit(hwirq, REG_INTR_POLARITY, TEST_STATE(hwirq, _IS_LOW));
    } else {
    generic_handle_domain_irq(sp_intc.domain, hwirq);
    }
    }
    chained_irq_exit(chip, desc);
    }
    static struct irq_chip sp_intc_chip = {
    .name = "sp_intc",
    .irq_ack = sp_intc_ack_irq,
    .irq_mask = sp_intc_mask_irq,
    .irq_unmask = sp_intc_unmask_irq,
    .irq_set_type = sp_intc_set_type,
    };
    static int sp_intc_irq_domain_map(struct irq_domain *domain,
    unsigned int irq, irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &sp_intc_chip, handle_level_irq);
    irq_set_chip_data(irq, &sp_intc_chip);
    irq_set_noprobe(irq);
    return 0;
    }
    static const struct irq_domain_ops sp_intc_dm_ops = {
    .xlate = irq_domain_xlate_twocell,
    .map = sp_intc_irq_domain_map,
    };
#[no_mangle]
unsafe extern "C" fn sp_intc_irq_map(node: *mut device_node, i: c_int) -> c_int {
    static int sp_intc_irq_map(struct device_node *node, int i)
    {
    unsigned int irq;
    irq = irq_of_parse_and_map(node, i);
    if (!irq)
    return -ENOENT;
    irq_set_chained_handler_and_data(irq, sp_intc_handle_ext_cascaded, (void *)(uintptr_t)i);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_intc_init_dt(node: *mut device_node, parent: *mut device_node) -> int __init {
    static int __init sp_intc_init_dt(struct device_node *node, struct device_node *parent)
    {
    int i, ret;
    sp_intc.g0 = of_iomap(node, 0);
    if (!sp_intc.g0)
    return -ENXIO;
    sp_intc.g1 = of_iomap(node, 1);
    if (!sp_intc.g1) {
    ret = -ENXIO;
    goto out_unmap0;
    }
    ret = sp_intc_irq_map(node, 0); // EXT_INT0
    if (ret)
    goto out_unmap1;
    ret = sp_intc_irq_map(node, 1); // EXT_INT1
    if (ret)
    goto out_unmap1;
// initial regs
    for (i = 0; i < SP_INTC_NR_GROUPS; i++) {
// all mask
    writel_relaxed(0, REG_INTR_MASK + i * 4);
// all edge
    writel_relaxed(~0, REG_INTR_TYPE + i * 4);
// all high-active
    writel_relaxed(0, REG_INTR_POLARITY + i * 4);
// all EXT_INT0
    writel_relaxed(~0, REG_INTR_PRIORITY + i * 4);
// all clear
    writel_relaxed(~0, REG_INTR_CLEAR + i * 4);
    }
    sp_intc.domain = irq_domain_create_linear(of_fwnode_handle(node), SP_INTC_NR_IRQS,
    &sp_intc_dm_ops, &sp_intc);
    if (!sp_intc.domain) {
    ret = -ENOMEM;
    goto out_unmap1;
    }
    raw_spin_lock_init(&sp_intc.lock);
    return 0;
    out_unmap1:
    iounmap(sp_intc.g1);
    out_unmap0:
    iounmap(sp_intc.g0);
    return ret;
    }
    IRQCHIP_DECLARE(sp_intc, "sunplus,sp7021-intc", sp_intc_init_dt);
