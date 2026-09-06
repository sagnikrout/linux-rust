//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-xgene-sb.c
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
// AppliedMicro X-Gene SoC GPIO-Standby Driver
//
// Copyright (c) 2014, Applied Micro Circuits Corporation
// Author:	Tin Huynh <tnhuynh@apm.com>.
// Y Vo <yvo@apm.com>.
// Quan Nguyen <qnguyen@apm.com>.
//

pub const XGENE_DFLT_MAX_NGPIO: c_int = 22;
pub const XGENE_DFLT_MAX_NIRQ: c_int = 6;
pub const XGENE_DFLT_IRQ_START_PIN: c_int = 8;

pub const MPA_GPIO_INT_LVL: c_uint = 0x0290;
pub const MPA_GPIO_OE_ADDR: c_uint = 0x029c;
pub const MPA_GPIO_OUT_ADDR: c_uint = 0x02a0;
pub const MPA_GPIO_IN_ADDR: c_uint = 0x02a4;
pub const MPA_GPIO_SEL_LO: c_uint = 0x0294;
pub const GPIO_INT_LEVEL_H: c_uint = 0x000001;
pub const GPIO_INT_LEVEL_L: c_uint = 0x000000;
//
// struct xgene_gpio_sb - GPIO-Standby private data structure.
// @chip:			Generic GPIO chip data
// @regs:			GPIO register base offset
// @irq_domain:			GPIO interrupt domain
// @irq_start:			GPIO pin that start support interrupt
// @nirq:			Number of GPIO pins that supports interrupt
// @parent_irq_base:		Start parent HWIRQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_gpio_sb {
    pub chip: gpio_generic_chip,
    pub regs: *mut void __iomem,
    pub irq_domain: *mut irq_domain,
    pub irq_start: u16,
    pub nirq: u16,
    pub parent_irq_base: u16,
}

    static void xgene_gpio_set_bit(struct gpio_chip *gc,
    void __iomem *reg, u32 gpio, int val)
    {
    struct gpio_generic_chip *chip = to_gpio_generic_chip(gc);
    u32 data;
    data = gpio_generic_read_reg(chip, reg);
    if (val)
    data |= GPIO_MASK(gpio);
    else
    data &= ~GPIO_MASK(gpio);
    gpio_generic_write_reg(chip, reg, data);
    }
#[no_mangle]
unsafe extern "C" fn xgene_gpio_sb_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int xgene_gpio_sb_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct xgene_gpio_sb *priv = irq_data_get_irq_chip_data(d);
    let mut gpio: c_int = HWIRQ_TO_GPIO(priv, d.hwirq);
    let mut lvl_type: c_int = GPIO_INT_LEVEL_H;
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_EDGE_RISING:
    case IRQ_TYPE_LEVEL_HIGH:
    lvl_type = GPIO_INT_LEVEL_H;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    case IRQ_TYPE_LEVEL_LOW:
    lvl_type = GPIO_INT_LEVEL_L;
    break;
    default:
    break;
    }
    xgene_gpio_set_bit(&priv.chip.gc, priv.regs + MPA_GPIO_SEL_LO,
    gpio * 2, 1);
    xgene_gpio_set_bit(&priv.chip.gc, priv.regs + MPA_GPIO_INT_LVL,
    d.hwirq, lvl_type);
// Propagate IRQ type setting to parent
    if (type & IRQ_TYPE_EDGE_BOTH)
    return irq_chip_set_type_parent(d, IRQ_TYPE_EDGE_RISING);
    else
    return irq_chip_set_type_parent(d, IRQ_TYPE_LEVEL_HIGH);
    }
#[no_mangle]
unsafe extern "C" fn xgene_gpio_sb_irq_mask(d: *mut irq_data) {
    static void xgene_gpio_sb_irq_mask(struct irq_data *d)
    {
    struct xgene_gpio_sb *priv = irq_data_get_irq_chip_data(d);
    irq_chip_mask_parent(d);
    gpiochip_disable_irq(&priv.chip.gc, d.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn xgene_gpio_sb_irq_unmask(d: *mut irq_data) {
    static void xgene_gpio_sb_irq_unmask(struct irq_data *d)
    {
    struct xgene_gpio_sb *priv = irq_data_get_irq_chip_data(d);
    gpiochip_enable_irq(&priv.chip.gc, d.hwirq);
    irq_chip_unmask_parent(d);
    }
    static const struct irq_chip xgene_gpio_sb_irq_chip = {
    .name           = "sbgpio",
    .irq_eoi	= irq_chip_eoi_parent,
    .irq_mask       = xgene_gpio_sb_irq_mask,
    .irq_unmask     = xgene_gpio_sb_irq_unmask,
    .irq_set_type   = xgene_gpio_sb_irq_set_type,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn xgene_gpio_sb_to_irq(gc: *mut gpio_chip, gpio: u32) -> c_int {
    static int xgene_gpio_sb_to_irq(struct gpio_chip *gc, u32 gpio)
    {
    struct xgene_gpio_sb *priv = gpiochip_get_data(gc);
    struct irq_fwspec fwspec;
    if ((gpio < priv.irq_start) ||
    (gpio > HWIRQ_TO_GPIO(priv, priv.nirq)))
    return -ENXIO;
    fwspec.fwnode = gc.parent.fwnode;
    fwspec.param_count = 2;
    fwspec.param[0] = GPIO_TO_HWIRQ(priv, gpio);
    fwspec.param[1] = IRQ_TYPE_EDGE_RISING;
    return irq_create_fwspec_mapping(&fwspec);
    }
    static int xgene_gpio_sb_domain_activate(struct irq_domain *d,
    struct irq_data *irq_data,
    bool reserve)
    {
    struct xgene_gpio_sb *priv = d.host_data;
    let mut gpio: u32 = HWIRQ_TO_GPIO(priv, irq_data.hwirq);
    int ret;
    ret = gpiochip_lock_as_irq(&priv.chip.gc, gpio);
    if (ret) {
    dev_err(priv.chip.gc.parent,
    "Unable to configure XGene GPIO standby pin %d as IRQ\n",
    gpio);
    return ret;
    }
    xgene_gpio_set_bit(&priv.chip.gc, priv.regs + MPA_GPIO_SEL_LO,
    gpio * 2, 1);
    return 0;
    }
    static void xgene_gpio_sb_domain_deactivate(struct irq_domain *d,
    struct irq_data *irq_data)
    {
    struct xgene_gpio_sb *priv = d.host_data;
    let mut gpio: u32 = HWIRQ_TO_GPIO(priv, irq_data.hwirq);
    gpiochip_unlock_as_irq(&priv.chip.gc, gpio);
    xgene_gpio_set_bit(&priv.chip.gc, priv.regs + MPA_GPIO_SEL_LO,
    gpio * 2, 0);
    }
    static int xgene_gpio_sb_domain_translate(struct irq_domain *d,
    struct irq_fwspec *fwspec,
    unsigned long *hwirq,
    unsigned int *type)
    {
    struct xgene_gpio_sb *priv = d.host_data;
    if ((fwspec.param_count != 2) ||
    (fwspec.param[0] >= priv.nirq))
    return -EINVAL;
// hwirq = fwspec->param[0];
// type = fwspec->param[1];
    return 0;
    }
    static int xgene_gpio_sb_domain_alloc(struct irq_domain *domain,
    unsigned int virq,
    unsigned int nr_irqs, void *data)
    {
    struct irq_fwspec *fwspec = data;
    struct irq_fwspec parent_fwspec;
    struct xgene_gpio_sb *priv = domain.host_data;
    irq_hw_number_t hwirq;
    unsigned int i;
    hwirq = fwspec.param[0];
    for (i = 0; i < nr_irqs; i++)
    irq_domain_set_hwirq_and_chip(domain, virq + i, hwirq + i,
    &xgene_gpio_sb_irq_chip, priv);
    parent_fwspec.fwnode = domain.parent.fwnode;
    if (is_of_node(parent_fwspec.fwnode)) {
    parent_fwspec.param_count = 3;
    parent_fwspec.param[0] = 0;/* SPI */
// Skip SGIs and PPIs
    parent_fwspec.param[1] = hwirq + priv.parent_irq_base - 32;
    parent_fwspec.param[2] = fwspec.param[1];
    } else if (is_fwnode_irqchip(parent_fwspec.fwnode)) {
    parent_fwspec.param_count = 2;
    parent_fwspec.param[0] = hwirq + priv.parent_irq_base;
    parent_fwspec.param[1] = fwspec.param[1];
    } else
    return -EINVAL;
    return irq_domain_alloc_irqs_parent(domain, virq, nr_irqs,
    &parent_fwspec);
    }
    static const struct irq_domain_ops xgene_gpio_sb_domain_ops = {
    .translate      = xgene_gpio_sb_domain_translate,
    .alloc          = xgene_gpio_sb_domain_alloc,
    .free           = irq_domain_free_irqs_common,
    .activate	= xgene_gpio_sb_domain_activate,
    .deactivate	= xgene_gpio_sb_domain_deactivate,
    };
#[no_mangle]
unsafe extern "C" fn xgene_gpio_sb_probe(pdev: *mut platform_device) -> c_int {
    static int xgene_gpio_sb_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct xgene_gpio_sb *priv;
    int ret;
    void __iomem *regs;
    struct irq_domain *parent_domain = core::ptr::null_mut();
    u32 val32;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    priv.regs = regs;
    ret = platform_get_irq(pdev, 0);
    if (ret > 0) {
    priv.parent_irq_base = irq_get_irq_data(ret).hwirq;
    parent_domain = irq_get_irq_data(ret).domain;
    }
    if (!parent_domain) {
    dev_err(&pdev.dev, "unable to obtain parent domain\n");
    return -ENODEV;
    }
    config = (struct gpio_generic_chip_config) {
    .dev = &pdev.dev,
    .sz = 4,
    .dat = regs + MPA_GPIO_IN_ADDR,
    .set = regs + MPA_GPIO_OUT_ADDR,
    .dirout = regs + MPA_GPIO_OE_ADDR,
    };
    ret = gpio_generic_chip_init(&priv.chip, &config);
    if (ret)
    return ret;
    priv.chip.gc.to_irq = xgene_gpio_sb_to_irq;
// Retrieve start irq pin, use default if property not found
    priv.irq_start = XGENE_DFLT_IRQ_START_PIN;
    if (!device_property_read_u32(&pdev.dev, "apm,irq-start", &val32))
    priv.irq_start = val32;
// Retrieve number irqs, use default if property not found
    priv.nirq = XGENE_DFLT_MAX_NIRQ;
    if (!device_property_read_u32(&pdev.dev, "apm,nr-irqs", &val32))
    priv.nirq = val32;
// Retrieve number gpio, use default if property not found
    priv.chip.gc.ngpio = XGENE_DFLT_MAX_NGPIO;
    if (!device_property_read_u32(&pdev.dev, "apm,nr-gpios", &val32))
    priv.chip.gc.ngpio = val32;
    dev_info(&pdev.dev, "Support %d gpios, %d irqs start from pin %d\n",
    priv.chip.gc.ngpio, priv.nirq, priv.irq_start);
    platform_set_drvdata(pdev, priv);
    priv.irq_domain = irq_domain_create_hierarchy(parent_domain,
    0, priv.nirq, pdev.dev.fwnode,
    &xgene_gpio_sb_domain_ops, priv);
    if (!priv.irq_domain)
    return -ENODEV;
    priv.chip.gc.irq.domain = priv.irq_domain;
    ret = devm_gpiochip_add_data(&pdev.dev, &priv.chip.gc, priv);
    if (ret) {
    dev_err(&pdev.dev,
    "failed to register X-Gene GPIO Standby driver\n");
    irq_domain_remove(priv.irq_domain);
    return ret;
    }
    dev_info(&pdev.dev, "X-Gene GPIO Standby driver registered\n");
// Register interrupt handlers for GPIO signaled ACPI Events
    acpi_gpiochip_request_interrupts(&priv.chip.gc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xgene_gpio_sb_remove(pdev: *mut platform_device) {
    static void xgene_gpio_sb_remove(struct platform_device *pdev)
    {
    struct xgene_gpio_sb *priv = platform_get_drvdata(pdev);
    acpi_gpiochip_free_interrupts(&priv.chip.gc);
    irq_domain_remove(priv.irq_domain);
    }
    static const struct of_device_id xgene_gpio_sb_of_match[] = {
    { .compatible = "apm,xgene-gpio-sb" },
    {}
    };
    MODULE_DEVICE_TABLE(of, xgene_gpio_sb_of_match);
    static const struct acpi_device_id xgene_gpio_sb_acpi_match[] = {
    { "APMC0D15" },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, xgene_gpio_sb_acpi_match);
    static struct platform_driver xgene_gpio_sb_driver = {
    .driver = {
    .name = "xgene-gpio-sb",
    .of_match_table = xgene_gpio_sb_of_match,
    .acpi_match_table = xgene_gpio_sb_acpi_match,
    },
    .probe = xgene_gpio_sb_probe,
    .remove = xgene_gpio_sb_remove,
    };
    module_platform_driver(xgene_gpio_sb_driver);
    MODULE_AUTHOR("AppliedMicro");
    MODULE_DESCRIPTION("APM X-Gene GPIO Standby driver");
    MODULE_LICENSE("GPL");
