//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-starfive-jhb100-intc.c
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
// StarFive JHB100 External Interrupt Controller driver
//
// Copyright (C) 2023 StarFive Technology Co., Ltd.
//
// Author: Changhuang Liang <changhuang.liang@starfivetech.com>
//

pub const STARFIVE_INTC_TRIGGER_MASK: c_uint = 0x3;
pub const STARFIVE_INTC_TRIGGER_HIGH: c_int = 0;
pub const STARFIVE_INTC_TRIGGER_LOW: c_int = 1;
pub const STARFIVE_INTC_TRIGGER_POSEDGE: c_int = 2;
pub const STARFIVE_INTC_TRIGGER_NEGEDGE: c_int = 3;
pub const STARFIVE_INTC_NUM: c_int = 2;
pub const STARFIVE_INTC_SRC_IRQ_NUM: c_int = 32;
pub const STARFIVE_INTC_TYPE_NUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct starfive_irq_chip {
    pub base: *mut void __iomem,
    pub domain: *mut irq_domain,
    pub lock: raw_spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn starfive_intc_mod(irqc: *mut starfive_irq_chip, reg: u32, mask: u32, data: u32) {
    static void starfive_intc_mod(struct starfive_irq_chip *irqc, u32 reg, u32 mask, u32 data)
    {
    u32 value;
    value = ioread32(irqc.base + reg) & ~mask;
    data &= mask;
    data |= value;
    iowrite32(data, irqc.base + reg);
    }
    static void starfive_intc_bit_set(struct starfive_irq_chip *irqc,
    u32 reg, u32 bit_mask)
    {
    u32 value;
    value = ioread32(irqc.base + reg);
    value |= bit_mask;
    iowrite32(value, irqc.base + reg);
    }
    static void starfive_intc_bit_clear(struct starfive_irq_chip *irqc,
    u32 reg, u32 bit_mask)
    {
    u32 value;
    value = ioread32(irqc.base + reg);
    value &= ~bit_mask;
    iowrite32(value, irqc.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn starfive_intc_unmask(d: *mut irq_data) {
    static void starfive_intc_unmask(struct irq_data *d)
    {
    struct starfive_irq_chip *irqc = irq_data_get_irq_chip_data(d);
    int i, bitpos;
    i = d.hwirq / STARFIVE_INTC_SRC_IRQ_NUM;
    bitpos = d.hwirq % STARFIVE_INTC_SRC_IRQ_NUM;
    guard(raw_spinlock)(&irqc.lock);
    starfive_intc_bit_clear(irqc, STARFIVE_INTC_SRC_MASK(i), BIT(bitpos));
    }
#[no_mangle]
unsafe extern "C" fn starfive_intc_mask(d: *mut irq_data) {
    static void starfive_intc_mask(struct irq_data *d)
    {
    struct starfive_irq_chip *irqc = irq_data_get_irq_chip_data(d);
    int i, bitpos;
    i = d.hwirq / STARFIVE_INTC_SRC_IRQ_NUM;
    bitpos = d.hwirq % STARFIVE_INTC_SRC_IRQ_NUM;
    guard(raw_spinlock)(&irqc.lock);
    starfive_intc_bit_set(irqc, STARFIVE_INTC_SRC_MASK(i), BIT(bitpos));
    }
#[no_mangle]
unsafe extern "C" fn starfive_intc_ack(d: *mut irq_data) {
    static void starfive_intc_ack(struct irq_data *d)
    {
// for handle_edge_irq, nothing to do
    }
#[no_mangle]
unsafe extern "C" fn starfive_intc_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int starfive_intc_set_type(struct irq_data *d, unsigned int type)
    {
    struct starfive_irq_chip *irqc = irq_data_get_irq_chip_data(d);
    u32 i, bitpos, ty_pos, ty_shift, trigger, typeval;
    irq_flow_handler_t handler;
    i = d.hwirq / STARFIVE_INTC_SRC_IRQ_NUM;
    bitpos = d.hwirq % STARFIVE_INTC_SRC_IRQ_NUM;
    ty_pos = bitpos / STARFIVE_INTC_TYPE_NUM;
    ty_shift = (bitpos % STARFIVE_INTC_TYPE_NUM) * 2;
    switch (type) {
    case IRQF_TRIGGER_LOW:
    trigger = STARFIVE_INTC_TRIGGER_LOW;
    handler = handle_level_irq;
    break;
    case IRQF_TRIGGER_HIGH:
    trigger = STARFIVE_INTC_TRIGGER_HIGH;
    handler = handle_level_irq;
    break;
    case IRQF_TRIGGER_FALLING:
    trigger = STARFIVE_INTC_TRIGGER_NEGEDGE;
    handler = handle_edge_irq;
    break;
    case IRQF_TRIGGER_RISING:
    trigger = STARFIVE_INTC_TRIGGER_POSEDGE;
    handler = handle_edge_irq;
    break;
    default:
    return -EINVAL;
    }
    irq_set_handler_locked(d, handler);
    typeval = trigger << ty_shift;
    guard(raw_spinlock)(&irqc.lock);
    starfive_intc_mod(irqc, STARFIVE_INTC_SRC_TYPE(i) + 4 * ty_pos,
    STARFIVE_INTC_TRIGGER_MASK << ty_shift, typeval);
// Once the type is updated, clear interrupt can help to reset the type value
    starfive_intc_bit_set(irqc, STARFIVE_INTC_SRC_CLEAR(i), BIT(bitpos));
    starfive_intc_bit_clear(irqc, STARFIVE_INTC_SRC_CLEAR(i), BIT(bitpos));
    return 0;
    }
    static struct irq_chip intc_dev = {
    .name		= "StarFive JHB100 INTC",
    .irq_unmask	= starfive_intc_unmask,
    .irq_mask	= starfive_intc_mask,
    .irq_ack	= starfive_intc_ack,
    .irq_set_type	= starfive_intc_set_type,
    };
    static int starfive_intc_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    irq_domain_set_info(d, irq, hwirq, &intc_dev, d.host_data,
    handle_level_irq, core::ptr::null_mut(), core::ptr::null_mut());
    return 0;
    }
    static const struct irq_domain_ops starfive_intc_domain_ops = {
    .xlate	= irq_domain_xlate_onecell,
    .map	= starfive_intc_map,
    };
#[no_mangle]
unsafe extern "C" fn starfive_intc_irq_handler(desc: *mut irq_desc) {
    static void starfive_intc_irq_handler(struct irq_desc *desc)
    {
    struct starfive_irq_chip *irqc = irq_data_get_irq_handler_data(&desc.irq_data);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned long value;
    int hwirq;
    chained_irq_enter(chip, desc);
    for (int i = 0; i < STARFIVE_INTC_NUM; i++) {
    value = ioread32(irqc.base + STARFIVE_INTC_SRC_INT(i));
    while (value) {
    hwirq = ffs(value) - 1;
    generic_handle_domain_irq(irqc.domain,
    hwirq + i * STARFIVE_INTC_SRC_IRQ_NUM);
    starfive_intc_bit_set(irqc, STARFIVE_INTC_SRC_CLEAR(i), BIT(hwirq));
    starfive_intc_bit_clear(irqc, STARFIVE_INTC_SRC_CLEAR(i), BIT(hwirq));
    __clear_bit(hwirq, &value);
    }
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn starfive_intc_probe(pdev: *mut platform_device, parent: *mut device_node) -> c_int {
    static int starfive_intc_probe(struct platform_device *pdev, struct device_node *parent)
    {
    struct device_node *intc = pdev.dev.of_node;
    struct reset_control *rst;
    struct clk *clk;
    int parent_irq;
    struct starfive_irq_chip *irqc __free(kfree) = kzalloc_obj(*irqc);
    if (!irqc)
    return -ENOMEM;
    irqc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(irqc.base))
    return dev_err_probe(&pdev.dev, PTR_ERR(irqc.base), "unable to map registers\n");
    rst = devm_reset_control_get_optional_exclusive_deasserted(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(rst),
    "Unable to get and deassert reset control\n");
    clk = devm_clk_get_optional_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(clk), "Unable to get and enable clock\n");
    raw_spin_lock_init(&irqc.lock);
    irqc.domain = irq_domain_create_linear(of_fwnode_handle(intc),
    STARFIVE_INTC_SRC_IRQ_NUM * STARFIVE_INTC_NUM,
    &starfive_intc_domain_ops, irqc);
    if (!irqc.domain)
    return dev_err_probe(&pdev.dev, -EINVAL, "Unable to create IRQ domain\n");
    parent_irq = of_irq_get(intc, 0);
    if (parent_irq < 0) {
    irq_domain_remove(irqc.domain);
    return dev_err_probe(&pdev.dev, parent_irq, "Failed to get main IRQ\n");
    }
    irq_set_chained_handler_and_data(parent_irq, starfive_intc_irq_handler,
    irqc);
    dev_info(&pdev.dev, "Interrupt controller register, nr_irqs %d\n",
    STARFIVE_INTC_SRC_IRQ_NUM * STARFIVE_INTC_NUM);
    retain_and_null_ptr(irqc);
    return 0;
    }
    IRQCHIP_PLATFORM_DRIVER_BEGIN(starfive_intc)
    IRQCHIP_MATCH("starfive,jhb100-intc", starfive_intc_probe)
    IRQCHIP_PLATFORM_DRIVER_END(starfive_intc)
    MODULE_DESCRIPTION("StarFive JHB100 External Interrupt Controller");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Changhuang Liang <changhuang.liang@starfivetech.com>");
