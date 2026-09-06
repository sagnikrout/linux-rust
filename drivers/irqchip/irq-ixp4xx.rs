//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ixp4xx.c
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
// irqchip for the IXP4xx interrupt controller
// Copyright (C) 2019 Linus Walleij <linus.walleij@linaro.org>
//
// Based on arch/arm/mach-ixp4xx/common.c
// Copyright 2002 (C) Intel Corporation
// Copyright 2003-2004 (C) MontaVista, Software, Inc.
// Copyright (C) Deepak Saxena <dsaxena@plexity.net>
//

pub const IXP4XX_ICPR: c_uint = 0x00 /* Interrupt Status */;
pub const IXP4XX_ICMR: c_uint = 0x04 /* Interrupt Enable */;
pub const IXP4XX_ICLR: c_uint = 0x08 /* Interrupt IRQ/FIQ Select */;
pub const IXP4XX_ICIP: c_uint = 0x0C /* IRQ Status */;
pub const IXP4XX_ICFP: c_uint = 0x10 /* FIQ Status */;
pub const IXP4XX_ICHR: c_uint = 0x14 /* Interrupt Priority */;
pub const IXP4XX_ICIH: c_uint = 0x18 /* IRQ Highest Pri Int */;
pub const IXP4XX_ICFH: c_uint = 0x1C /* FIQ Highest Pri Int */;
// IXP43x and IXP46x-only
pub const IXP4XX_ICPR2: c_uint = 0x20 /* Interrupt Status 2 */;
pub const IXP4XX_ICMR2: c_uint = 0x24 /* Interrupt Enable 2 */;
pub const IXP4XX_ICLR2: c_uint = 0x28 /* Interrupt IRQ/FIQ Select 2 */;
pub const IXP4XX_ICIP2: c_uint = 0x2C /* IRQ Status */;
pub const IXP4XX_ICFP2: c_uint = 0x30 /* FIQ Status */;
pub const IXP4XX_ICEEN: c_uint = 0x34 /* Error High Pri Enable */;
//
// struct ixp4xx_irq - state container for the Faraday IRQ controller
// @irqbase: IRQ controller memory base in virtual memory
// @is_356: if this is an IXP43x, IXP45x or IX46x SoC (with 64 IRQs)
// @irqchip: irqchip for this instance
// @domain: IRQ domain for this instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixp4xx_irq {
    pub irqbase: *mut void __iomem,
    pub is_356: bool,
    pub irqchip: irq_chip,
    pub domain: *mut irq_domain,
}

// Local static state container
    static struct ixp4xx_irq ixirq;
// GPIO Clocks
pub const IXP4XX_GPIO_CLK_0: c_int = 14;
pub const IXP4XX_GPIO_CLK_1: c_int = 15;
#[no_mangle]
unsafe extern "C" fn ixp4xx_set_irq_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int ixp4xx_set_irq_type(struct irq_data *d, unsigned int type)
    {
// All are level active high (asserted) here
    if (type != IRQ_TYPE_LEVEL_HIGH)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_irq_mask(d: *mut irq_data) {
    static void ixp4xx_irq_mask(struct irq_data *d)
    {
    struct ixp4xx_irq *ixi = irq_data_get_irq_chip_data(d);
    u32 val;
    if (ixi.is_356 && d.hwirq >= 32) {
    val = __raw_readl(ixi.irqbase + IXP4XX_ICMR2);
    val &= ~BIT(d.hwirq - 32);
    __raw_writel(val, ixi.irqbase + IXP4XX_ICMR2);
    } else {
    val = __raw_readl(ixi.irqbase + IXP4XX_ICMR);
    val &= ~BIT(d.hwirq);
    __raw_writel(val, ixi.irqbase + IXP4XX_ICMR);
    }
    }
//
// Level triggered interrupts on GPIO lines can only be cleared when the
// interrupt condition disappears.
//
#[no_mangle]
unsafe extern "C" fn ixp4xx_irq_unmask(d: *mut irq_data) {
    static void ixp4xx_irq_unmask(struct irq_data *d)
    {
    struct ixp4xx_irq *ixi = irq_data_get_irq_chip_data(d);
    u32 val;
    if (ixi.is_356 && d.hwirq >= 32) {
    val = __raw_readl(ixi.irqbase + IXP4XX_ICMR2);
    val |= BIT(d.hwirq - 32);
    __raw_writel(val, ixi.irqbase + IXP4XX_ICMR2);
    } else {
    val = __raw_readl(ixi.irqbase + IXP4XX_ICMR);
    val |= BIT(d.hwirq);
    __raw_writel(val, ixi.irqbase + IXP4XX_ICMR);
    }
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry ixp4xx_handle_irq(struct pt_regs *regs)
    {
    struct ixp4xx_irq *ixi = &ixirq;
    unsigned long status;
    int i;
    status = __raw_readl(ixi.irqbase + IXP4XX_ICIP);
    for_each_set_bit(i, &status, 32)
    generic_handle_domain_irq(ixi.domain, i);
//
// IXP465/IXP435 has an upper IRQ status register
//
    if (ixi.is_356) {
    status = __raw_readl(ixi.irqbase + IXP4XX_ICIP2);
    for_each_set_bit(i, &status, 32)
    generic_handle_domain_irq(ixi.domain, i + 32);
    }
    }
    static int ixp4xx_irq_domain_translate(struct irq_domain *domain,
    struct irq_fwspec *fwspec,
    unsigned long *hwirq,
    unsigned int *type)
    {
// We support standard DT translation
    if (is_of_node(fwspec.fwnode) && fwspec.param_count == 2) {
// hwirq = fwspec->param[0];
// type = fwspec->param[1];
    return 0;
    }
    if (is_fwnode_irqchip(fwspec.fwnode)) {
    if (fwspec.param_count != 2)
    return -EINVAL;
// hwirq = fwspec->param[0];
// type = fwspec->param[1];
    WARN_ON(*type == IRQ_TYPE_NONE);
    return 0;
    }
    return -EINVAL;
    }
    static int ixp4xx_irq_domain_alloc(struct irq_domain *d,
    unsigned int irq, unsigned int nr_irqs,
    void *data)
    {
    struct ixp4xx_irq *ixi = d.host_data;
    irq_hw_number_t hwirq;
    let mut type: c_uint = IRQ_TYPE_NONE;
    struct irq_fwspec *fwspec = data;
    int ret;
    int i;
    ret = ixp4xx_irq_domain_translate(d, fwspec, &hwirq, &type);
    if (ret)
    return ret;
    for (i = 0; i < nr_irqs; i++) {
//
// TODO: after converting IXP4xx to only device tree, set
// handle_bad_irq as default handler and assume all consumers
// call .set_type() as this is provided in the second cell in
// the device tree phandle.
//
    irq_domain_set_info(d,
    irq + i,
    hwirq + i,
    &ixi.irqchip,
    ixi,
    handle_level_irq,
    core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_probe(irq + i);
    }
    return 0;
    }
//
// This needs to be a hierarchical irqdomain to work well with the
// GPIO irqchip (which is lower in the hierarchy)
//
    static const struct irq_domain_ops ixp4xx_irqdomain_ops = {
    .translate = ixp4xx_irq_domain_translate,
    .alloc = ixp4xx_irq_domain_alloc,
    .free = irq_domain_free_irqs_common,
    };
//
// ixp4x_irq_setup() - Common setup code for the IXP4xx interrupt controller
// @ixi: State container
// @irqbase: Virtual memory base for the interrupt controller
// @fwnode: Corresponding fwnode abstraction for this controller
// @is_356: if this is an IXP43x, IXP45x or IXP46x SoC variant
//
    static int __init ixp4xx_irq_setup(struct ixp4xx_irq *ixi,
    void __iomem *irqbase,
    struct fwnode_handle *fwnode,
    bool is_356)
    {
    int nr_irqs;
    ixi.irqbase = irqbase;
    ixi.is_356 = is_356;
// Route all sources to IRQ instead of FIQ
    __raw_writel(0x0, ixi.irqbase + IXP4XX_ICLR);
// Disable all interrupts
    __raw_writel(0x0, ixi.irqbase + IXP4XX_ICMR);
    if (is_356) {
// Route upper 32 sources to IRQ instead of FIQ
    __raw_writel(0x0, ixi.irqbase + IXP4XX_ICLR2);
// Disable upper 32 interrupts
    __raw_writel(0x0, ixi.irqbase + IXP4XX_ICMR2);
    nr_irqs = 64;
    } else {
    nr_irqs = 32;
    }
    ixi.irqchip.name = "IXP4xx";
    ixi.irqchip.irq_mask = ixp4xx_irq_mask;
    ixi.irqchip.irq_unmask	= ixp4xx_irq_unmask;
    ixi.irqchip.irq_set_type = ixp4xx_set_irq_type;
    ixi.domain = irq_domain_create_linear(fwnode, nr_irqs,
    &ixp4xx_irqdomain_ops,
    ixi);
    if (!ixi.domain) {
    pr_crit("IXP4XX: can not add primary irqdomain\n");
    return -ENODEV;
    }
    set_handle_irq(ixp4xx_handle_irq);
    return 0;
    }
    static int __init ixp4xx_of_init_irq(struct device_node *np,
    struct device_node *parent)
    {
    struct ixp4xx_irq *ixi = &ixirq;
    void __iomem *base;
    struct fwnode_handle *fwnode;
    bool is_356;
    int ret;
    base = of_iomap(np, 0);
    if (!base) {
    pr_crit("IXP4XX: could not ioremap interrupt controller\n");
    return -ENODEV;
    }
    fwnode = of_fwnode_handle(np);
// These chip variants have 64 interrupts
    is_356 = of_device_is_compatible(np, "intel,ixp43x-interrupt") ||
    of_device_is_compatible(np, "intel,ixp45x-interrupt") ||
    of_device_is_compatible(np, "intel,ixp46x-interrupt");
    ret = ixp4xx_irq_setup(ixi, base, fwnode, is_356);
    if (ret)
    pr_crit("IXP4XX: failed to set up irqchip\n");
    return ret;
    }
    IRQCHIP_DECLARE(ixp42x, "intel,ixp42x-interrupt",
    ixp4xx_of_init_irq);
    IRQCHIP_DECLARE(ixp43x, "intel,ixp43x-interrupt",
    ixp4xx_of_init_irq);
    IRQCHIP_DECLARE(ixp45x, "intel,ixp45x-interrupt",
    ixp4xx_of_init_irq);
    IRQCHIP_DECLARE(ixp46x, "intel,ixp46x-interrupt",
    ixp4xx_of_init_irq);
