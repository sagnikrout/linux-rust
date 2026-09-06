//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-davinci-cp-intc.c
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
// Author: Steve Chen <schen@mvista.com>
// Copyright (C) 2008-2009, MontaVista Software, Inc. <source@mvista.com>
// Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
// Copyright (C) 2019, Texas Instruments
//
// TI Common Platform Interrupt Controller (cp_intc) driver

pub const DAVINCI_CP_INTC_CTRL: c_uint = 0x04;
pub const DAVINCI_CP_INTC_HOST_CTRL: c_uint = 0x0c;
pub const DAVINCI_CP_INTC_GLOBAL_ENABLE: c_uint = 0x10;
pub const DAVINCI_CP_INTC_SYS_STAT_IDX_CLR: c_uint = 0x24;
pub const DAVINCI_CP_INTC_SYS_ENABLE_IDX_SET: c_uint = 0x28;
pub const DAVINCI_CP_INTC_SYS_ENABLE_IDX_CLR: c_uint = 0x2c;
pub const DAVINCI_CP_INTC_HOST_ENABLE_IDX_SET: c_uint = 0x34;
pub const DAVINCI_CP_INTC_HOST_ENABLE_IDX_CLR: c_uint = 0x38;
pub const DAVINCI_CP_INTC_PRIO_IDX: c_uint = 0x80;

    static void __iomem *davinci_cp_intc_base;
    static struct irq_domain *davinci_cp_intc_irq_domain;
#[no_mangle]
pub unsafe extern "C" fn davinci_cp_intc_read(offset: c_uint) -> c_uint {
    static inline unsigned int davinci_cp_intc_read(unsigned int offset)
    {
    return readl_relaxed(davinci_cp_intc_base + offset);
    }
    static inline void davinci_cp_intc_write(unsigned long value,
    unsigned int offset)
    {
    writel_relaxed(value, davinci_cp_intc_base + offset);
    }
#[no_mangle]
unsafe extern "C" fn davinci_cp_intc_ack_irq(d: *mut irq_data) {
    static void davinci_cp_intc_ack_irq(struct irq_data *d)
    {
    davinci_cp_intc_write(d.hwirq, DAVINCI_CP_INTC_SYS_STAT_IDX_CLR);
    }
#[no_mangle]
unsafe extern "C" fn davinci_cp_intc_mask_irq(d: *mut irq_data) {
    static void davinci_cp_intc_mask_irq(struct irq_data *d)
    {
// XXX don't know why we need to disable nIRQ here...
    davinci_cp_intc_write(1, DAVINCI_CP_INTC_HOST_ENABLE_IDX_CLR);
    davinci_cp_intc_write(d.hwirq, DAVINCI_CP_INTC_SYS_ENABLE_IDX_CLR);
    davinci_cp_intc_write(1, DAVINCI_CP_INTC_HOST_ENABLE_IDX_SET);
    }
#[no_mangle]
unsafe extern "C" fn davinci_cp_intc_unmask_irq(d: *mut irq_data) {
    static void davinci_cp_intc_unmask_irq(struct irq_data *d)
    {
    davinci_cp_intc_write(d.hwirq, DAVINCI_CP_INTC_SYS_ENABLE_IDX_SET);
    }
    static int davinci_cp_intc_set_irq_type(struct irq_data *d,
    unsigned int flow_type)
    {
    unsigned int reg, mask, polarity, type;
    reg = BIT_WORD(d.hwirq);
    mask = BIT_MASK(d.hwirq);
    polarity = davinci_cp_intc_read(DAVINCI_CP_INTC_SYS_POLARITY(reg));
    type = davinci_cp_intc_read(DAVINCI_CP_INTC_SYS_TYPE(reg));
    switch (flow_type) {
    case IRQ_TYPE_EDGE_RISING:
    polarity |= mask;
    type |= mask;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    polarity &= ~mask;
    type |= mask;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    polarity |= mask;
    type &= ~mask;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    polarity &= ~mask;
    type &= ~mask;
    break;
    default:
    return -EINVAL;
    }
    davinci_cp_intc_write(polarity, DAVINCI_CP_INTC_SYS_POLARITY(reg));
    davinci_cp_intc_write(type, DAVINCI_CP_INTC_SYS_TYPE(reg));
    return 0;
    }
    static struct irq_chip davinci_cp_intc_irq_chip = {
    .name		= "cp_intc",
    .irq_ack	= davinci_cp_intc_ack_irq,
    .irq_mask	= davinci_cp_intc_mask_irq,
    .irq_unmask	= davinci_cp_intc_unmask_irq,
    .irq_set_type	= davinci_cp_intc_set_irq_type,
    .flags		= IRQCHIP_SKIP_SET_WAKE,
    };
#[no_mangle]
unsafe extern "C" fn davinci_cp_intc_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry davinci_cp_intc_handle_irq(struct pt_regs *regs)
    {
    int gpir, irqnr, none;
//
// The interrupt number is in first ten bits. The NONE field set to 1
// indicates a spurious irq.
//
    gpir = davinci_cp_intc_read(DAVINCI_CP_INTC_PRIO_IDX);
    irqnr = gpir & DAVINCI_CP_INTC_PRI_INDX_MASK;
    none = gpir & DAVINCI_CP_INTC_GPIR_NONE;
    if (unlikely(none)) {
    pr_err_once("%s: spurious irq!\n", __func__);
    return;
    }
    generic_handle_domain_irq(davinci_cp_intc_irq_domain, irqnr);
    }
    static int davinci_cp_intc_host_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {
    pr_debug("cp_intc_host_map(%d, 0x%lx)\n", virq, hw);
    irq_set_chip(virq, &davinci_cp_intc_irq_chip);
    irq_set_probe(virq);
    irq_set_handler(virq, handle_edge_irq);
    return 0;
    }
    static const struct irq_domain_ops davinci_cp_intc_irq_domain_ops = {
    .map = davinci_cp_intc_host_map,
    .xlate = irq_domain_xlate_onetwocell,
    };
    static int __init davinci_cp_intc_do_init(struct resource *res, unsigned int num_irqs,
    struct device_node *node)
    {
    let mut num_regs: c_uint = BITS_TO_LONGS(num_irqs);
    int offset, irq_base;
    void __iomem *req;
    req = request_mem_region(res.start, resource_size(res), "davinci-cp-intc");
    if (!req) {
    pr_err("%s: register range busy\n", __func__);
    return -EBUSY;
    }
    davinci_cp_intc_base = ioremap(res.start, resource_size(res));
    if (!davinci_cp_intc_base) {
    pr_err("%s: unable to ioremap register range\n", __func__);
    return -EINVAL;
    }
    davinci_cp_intc_write(0, DAVINCI_CP_INTC_GLOBAL_ENABLE);
// Disable all host interrupts
    davinci_cp_intc_write(0, DAVINCI_CP_INTC_HOST_ENABLE(0));
// Disable system interrupts
    for (offset = 0; offset < num_regs; offset++)
    davinci_cp_intc_write(~0, DAVINCI_CP_INTC_SYS_ENABLE_CLR(offset));
// Set to normal mode, no nesting, no priority hold
    davinci_cp_intc_write(0, DAVINCI_CP_INTC_CTRL);
    davinci_cp_intc_write(0, DAVINCI_CP_INTC_HOST_CTRL);
// Clear system interrupt status
    for (offset = 0; offset < num_regs; offset++)
    davinci_cp_intc_write(~0, DAVINCI_CP_INTC_SYS_STAT_CLR(offset));
// Enable nIRQ (what about nFIQ?)
    davinci_cp_intc_write(1, DAVINCI_CP_INTC_HOST_ENABLE_IDX_SET);
// 4 channels per register
    num_regs = (num_irqs + 3) >> 2;
// Default all priorities to channel 7.
    for (offset = 0; offset < num_regs; offset++)
    davinci_cp_intc_write(0x07070707, DAVINCI_CP_INTC_CHAN_MAP(offset));
    irq_base = irq_alloc_descs(-1, 0, num_irqs, 0);
    if (irq_base < 0) {
    pr_err("%s: unable to allocate interrupt descriptors: %d\n", __func__, irq_base);
    return irq_base;
    }
    davinci_cp_intc_irq_domain = irq_domain_create_legacy(of_fwnode_handle(node), num_irqs,
    irq_base, 0,
    &davinci_cp_intc_irq_domain_ops,
    core::ptr::null_mut());
    if (!davinci_cp_intc_irq_domain) {
    pr_err("%s: unable to create an interrupt domain\n", __func__);
    return -EINVAL;
    }
    set_handle_irq(davinci_cp_intc_handle_irq);
// Enable global interrupt
    davinci_cp_intc_write(1, DAVINCI_CP_INTC_GLOBAL_ENABLE);
    return 0;
    }
    static int __init davinci_cp_intc_of_init(struct device_node *node,
    struct device_node *parent)
    {
    unsigned int num_irqs;
    struct resource res;
    int ret;
    ret = of_address_to_resource(node, 0, &res);
    if (ret) {
    pr_err("%s: unable to get the register range from device-tree\n", __func__);
    return ret;
    }
    ret = of_property_read_u32(node, "ti,intc-size", &num_irqs);
    if (ret) {
    pr_err("%s: unable to read the 'ti,intc-size' property\n", __func__);
    return ret;
    }
    return davinci_cp_intc_do_init(&res, num_irqs, node);
    }
    IRQCHIP_DECLARE(cp_intc, "ti,cp-intc", davinci_cp_intc_of_init);
