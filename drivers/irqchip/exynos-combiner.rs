//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/exynos-combiner.c
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
// Copyright (c) 2010-2011 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Combiner irqchip for EXYNOS
//

pub const COMBINER_ENABLE_SET: c_uint = 0x0;
pub const COMBINER_ENABLE_CLEAR: c_uint = 0x4;
pub const COMBINER_INT_STATUS: c_uint = 0xC;
pub const IRQ_IN_COMBINER: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct combiner_chip_data {
    pub hwirq_offset: c_uint,
    pub irq_mask: c_uint,
    pub base: *mut void __iomem,
    pub parent_irq: c_uint,

    pub pm_save: u32,

}

    static struct combiner_chip_data *combiner_data;
    static struct irq_domain *combiner_irq_domain;
    let mut max_nr: static unsigned int = 20;
    static inline void __iomem *combiner_base(struct irq_data *data)
    {
    struct combiner_chip_data *combiner_data =
    irq_data_get_irq_chip_data(data);
    return combiner_data.base;
    }
#[no_mangle]
unsafe extern "C" fn combiner_mask_irq(data: *mut irq_data) {
    static void combiner_mask_irq(struct irq_data *data)
    {
    let mut mask: u32 = 1 << (data.hwirq % 32);
    writel_relaxed(mask, combiner_base(data) + COMBINER_ENABLE_CLEAR);
    }
#[no_mangle]
unsafe extern "C" fn combiner_unmask_irq(data: *mut irq_data) {
    static void combiner_unmask_irq(struct irq_data *data)
    {
    let mut mask: u32 = 1 << (data.hwirq % 32);
    writel_relaxed(mask, combiner_base(data) + COMBINER_ENABLE_SET);
    }
#[no_mangle]
unsafe extern "C" fn combiner_handle_cascade_irq(desc: *mut irq_desc) {
    static void combiner_handle_cascade_irq(struct irq_desc *desc)
    {
    struct combiner_chip_data *chip_data = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned int combiner_irq;
    unsigned long status;
    int ret;
    chained_irq_enter(chip, desc);
    status = readl_relaxed(chip_data.base + COMBINER_INT_STATUS);
    status &= chip_data.irq_mask;
    if (status == 0)
    goto out;
    combiner_irq = chip_data.hwirq_offset + __ffs(status);
    ret = generic_handle_domain_irq(combiner_irq_domain, combiner_irq);
    if (unlikely(ret))
    handle_bad_irq(desc);
    out:
    chained_irq_exit(chip, desc);
    }

    static int combiner_set_affinity(struct irq_data *d,
    const struct cpumask *mask_val, bool force)
    {
    struct combiner_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    struct irq_chip *chip = irq_get_chip(chip_data.parent_irq);
    struct irq_data *data = irq_get_irq_data(chip_data.parent_irq);
    if (chip && chip.irq_set_affinity)
    return chip.irq_set_affinity(data, mask_val, force);
    else
    return -EINVAL;
    }

    static struct irq_chip combiner_chip = {
    .name			= "COMBINER",
    .irq_mask		= combiner_mask_irq,
    .irq_unmask		= combiner_unmask_irq,

    .irq_set_affinity	= combiner_set_affinity,

    };
    static void __init combiner_cascade_irq(struct combiner_chip_data *combiner_data,
    unsigned int irq)
    {
    irq_set_chained_handler_and_data(irq, combiner_handle_cascade_irq,
    combiner_data);
    }
    static void __init combiner_init_one(struct combiner_chip_data *combiner_data,
    unsigned int combiner_nr,
    void __iomem *base, unsigned int irq)
    {
    combiner_data.base = base;
    combiner_data.hwirq_offset = (combiner_nr & ~3) * IRQ_IN_COMBINER;
    combiner_data.irq_mask = 0xff << ((combiner_nr % 4) << 3);
    combiner_data.parent_irq = irq;
// Disable all interrupts
    writel_relaxed(combiner_data.irq_mask, base + COMBINER_ENABLE_CLEAR);
    }
    static int combiner_irq_domain_xlate(struct irq_domain *d,
    struct device_node *controller,
    const u32 *intspec, unsigned int intsize,
    unsigned long *out_hwirq,
    unsigned int *out_type)
    {
    if (irq_domain_get_of_node(d) != controller)
    return -EINVAL;
    if (intsize < 2)
    return -EINVAL;
// out_hwirq = intspec[0] * IRQ_IN_COMBINER + intspec[1];
// out_type = 0;
    return 0;
    }
    static int combiner_irq_domain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hw)
    {
    struct combiner_chip_data *combiner_data = d.host_data;
    irq_set_chip_and_handler(irq, &combiner_chip, handle_level_irq);
    irq_set_chip_data(irq, &combiner_data[hw >> 3]);
    irq_set_probe(irq);
    return 0;
    }
    static const struct irq_domain_ops combiner_irq_domain_ops = {
    .xlate	= combiner_irq_domain_xlate,
    .map	= combiner_irq_domain_map,
    };
    static void __init combiner_init(void __iomem *combiner_base,
    struct device_node *np)
    {
    int i, irq;
    unsigned int nr_irq;
    nr_irq = max_nr * IRQ_IN_COMBINER;
    combiner_data = kzalloc_objs(*combiner_data, max_nr);
    if (!combiner_data)
    return;
    combiner_irq_domain = irq_domain_create_linear(of_fwnode_handle(np), nr_irq,
    &combiner_irq_domain_ops, combiner_data);
    if (WARN_ON(!combiner_irq_domain)) {
    pr_warn("%s: irq domain init failed\n", __func__);
    return;
    }
    for (i = 0; i < max_nr; i++) {
    irq = irq_of_parse_and_map(np, i);
    combiner_init_one(&combiner_data[i], i,
    combiner_base + (i >> 2) * 0x10, irq);
    combiner_cascade_irq(&combiner_data[i], irq);
    }
    }

//
// combiner_suspend - save interrupt combiner state before suspend
// @data: syscore context
//
// Save the interrupt enable set register for all combiner groups since
// the state is lost when the system enters into a sleep state.
//
#[no_mangle]
unsafe extern "C" fn combiner_suspend(data: *mut c_void) -> c_int {
    static int combiner_suspend(void *data)
    {
    int i;
    for (i = 0; i < max_nr; i++)
    combiner_data[i].pm_save =
    readl_relaxed(combiner_data[i].base + COMBINER_ENABLE_SET);
    return 0;
    }
//
// combiner_resume - restore interrupt combiner state after resume
// @data: syscore context
//
// Restore the interrupt enable set register for all combiner groups since
// the state is lost when the system enters into a sleep state on suspend.
//
#[no_mangle]
unsafe extern "C" fn combiner_resume(data: *mut c_void) {
    static void combiner_resume(void *data)
    {
    int i;
    for (i = 0; i < max_nr; i++) {
    writel_relaxed(combiner_data[i].irq_mask,
    combiner_data[i].base + COMBINER_ENABLE_CLEAR);
    writel_relaxed(combiner_data[i].pm_save,
    combiner_data[i].base + COMBINER_ENABLE_SET);
    }
    }

    static const struct syscore_ops combiner_syscore_ops = {
    .suspend	= combiner_suspend,
    .resume		= combiner_resume,
    };
    static struct syscore combiner_syscore = {
    .ops = &combiner_syscore_ops,
    };
    static int __init combiner_of_init(struct device_node *np,
    struct device_node *parent)
    {
    void __iomem *combiner_base;
    combiner_base = of_iomap(np, 0);
    if (!combiner_base) {
    pr_err("%s: failed to map combiner registers\n", __func__);
    return -ENXIO;
    }
    if (of_property_read_u32(np, "samsung,combiner-nr", &max_nr)) {
    pr_info("%s: number of combiners not specified, "
    "setting default as %d.\n",
    __func__, max_nr);
    }
    combiner_init(combiner_base, np);
    register_syscore(&combiner_syscore);
    return 0;
    }
    IRQCHIP_DECLARE(exynos4210_combiner, "samsung,exynos4210-combiner",
    combiner_of_init);
