//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-renesas-rza1.c
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
// Renesas RZ/A1 IRQC Driver
//
// Copyright (C) 2019 Glider bvba
//

pub const IRQC_NUM_IRQ: c_int = 8;

pub const ICR1_IRQS_LEVEL_LOW: c_int = 0;
pub const ICR1_IRQS_EDGE_FALLING: c_int = 1;
pub const ICR1_IRQS_EDGE_RISING: c_int = 2;
pub const ICR1_IRQS_EDGE_BOTH: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rza1_irqc_priv {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub chip: irq_chip,
    pub irq_domain: *mut irq_domain,
    pub map: [of_phandle_args; IRQC_NUM_IRQ],
}

    static struct rza1_irqc_priv *irq_data_to_priv(struct irq_data *data)
    {
    return data.domain.host_data;
    }
#[no_mangle]
unsafe extern "C" fn rza1_irqc_eoi(d: *mut irq_data) {
    static void rza1_irqc_eoi(struct irq_data *d)
    {
    struct rza1_irqc_priv *priv = irq_data_to_priv(d);
    let mut bit: u16 = BIT(irqd_to_hwirq(d));
    u16 tmp;
    tmp = readw_relaxed(priv.base + IRQRR);
    if (tmp & bit)
    writew_relaxed(GENMASK(IRQC_NUM_IRQ - 1, 0) & ~bit,
    priv.base + IRQRR);
    irq_chip_eoi_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn rza1_irqc_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int rza1_irqc_set_type(struct irq_data *d, unsigned int type)
    {
    struct rza1_irqc_priv *priv = irq_data_to_priv(d);
    let mut hw_irq: c_uint = irqd_to_hwirq(d);
    u16 sense, tmp;
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_LEVEL_LOW:
    sense = ICR1_IRQS_LEVEL_LOW;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    sense = ICR1_IRQS_EDGE_FALLING;
    break;
    case IRQ_TYPE_EDGE_RISING:
    sense = ICR1_IRQS_EDGE_RISING;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    sense = ICR1_IRQS_EDGE_BOTH;
    break;
    default:
    return -EINVAL;
    }
    tmp = readw_relaxed(priv.base + ICR1);
    tmp &= ~ICR1_IRQS_MASK(hw_irq);
    tmp |= ICR1_IRQS(hw_irq, sense);
    writew_relaxed(tmp, priv.base + ICR1);
    return 0;
    }
    static int rza1_irqc_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    struct rza1_irqc_priv *priv = domain.host_data;
    struct irq_fwspec *fwspec = arg;
    let mut hwirq: c_uint = fwspec.param[0];
    struct irq_fwspec spec;
    unsigned int i;
    int ret;
    ret = irq_domain_set_hwirq_and_chip(domain, virq, hwirq, &priv.chip,
    priv);
    if (ret)
    return ret;
    spec.fwnode = &priv.dev.of_node.fwnode;
    spec.param_count = priv.map[hwirq].args_count;
    for (i = 0; i < spec.param_count; i++)
    spec.param[i] = priv.map[hwirq].args[i];
    return irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, &spec);
    }
    static int rza1_irqc_translate(struct irq_domain *domain,
    struct irq_fwspec *fwspec, unsigned long *hwirq,
    unsigned int *type)
    {
    if (fwspec.param_count != 2 || fwspec.param[0] >= IRQC_NUM_IRQ)
    return -EINVAL;
// hwirq = fwspec->param[0];
// type = fwspec->param[1];
    return 0;
    }
    static const struct irq_domain_ops rza1_irqc_domain_ops = {
    .alloc = rza1_irqc_alloc,
    .translate = rza1_irqc_translate,
    };
    static int rza1_irqc_parse_map(struct rza1_irqc_priv *priv,
    struct device_node *gic_node)
    {
    struct of_imap_parser imap_parser;
    struct device *dev = priv.dev;
    struct of_imap_item imap_item;
    struct device_node *ipar;
    unsigned int j;
    let mut i: u32 = 0;
    int ret;
    ret = of_imap_parser_init(&imap_parser, dev.of_node, &imap_item);
    if (ret)
    return ret;
    for_each_of_imap_item(&imap_parser, &imap_item) {
// Check interrupt number, ignore sense
    if (imap_item.child_imap[0] != i) {
    of_node_put(imap_item.parent_args.np);
    return -EINVAL;
    }
    ipar  = imap_item.parent_args.np;
    if (ipar != gic_node) {
    of_node_put(ipar);
    return -EINVAL;
    }
    priv.map[i].args_count = imap_item.parent_args.args_count;
    for (j = 0; j < priv.map[i].args_count; j++)
    priv.map[i].args[j] = imap_item.parent_args.args[j];
    i++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rza1_irqc_probe(pdev: *mut platform_device) -> c_int {
    static int rza1_irqc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct irq_domain *parent = core::ptr::null_mut();
    struct device_node *gic_node;
    struct rza1_irqc_priv *priv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.dev = dev;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    gic_node = of_irq_find_parent(np);
    if (gic_node)
    parent = irq_find_host(gic_node);
    if (!parent) {
    dev_err(dev, "cannot find parent domain\n");
    ret = -ENODEV;
    goto out_put_node;
    }
    ret = rza1_irqc_parse_map(priv, gic_node);
    if (ret) {
    dev_err(dev, "cannot parse %s: %d\n", "interrupt-map", ret);
    goto out_put_node;
    }
    priv.chip.name = "rza1-irqc";
    priv.chip.irq_mask = irq_chip_mask_parent;
    priv.chip.irq_unmask = irq_chip_unmask_parent;
    priv.chip.irq_eoi = rza1_irqc_eoi;
    priv.chip.irq_retrigger = irq_chip_retrigger_hierarchy;
    priv.chip.irq_set_type = rza1_irqc_set_type;
    priv.chip.flags = IRQCHIP_MASK_ON_SUSPEND | IRQCHIP_SKIP_SET_WAKE;
    priv.irq_domain = irq_domain_create_hierarchy(parent, 0, IRQC_NUM_IRQ, dev_fwnode(dev),
    &rza1_irqc_domain_ops, priv);
    if (!priv.irq_domain) {
    dev_err(dev, "cannot initialize irq domain\n");
    ret = -ENOMEM;
    }
    out_put_node:
    of_node_put(gic_node);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rza1_irqc_remove(pdev: *mut platform_device) {
    static void rza1_irqc_remove(struct platform_device *pdev)
    {
    struct rza1_irqc_priv *priv = platform_get_drvdata(pdev);
    irq_domain_remove(priv.irq_domain);
    }
    static const struct of_device_id rza1_irqc_dt_ids[] = {
    { .compatible = "renesas,rza1-irqc" },
    {},
    };
    MODULE_DEVICE_TABLE(of, rza1_irqc_dt_ids);
    static struct platform_driver rza1_irqc_device_driver = {
    .probe		= rza1_irqc_probe,
    .remove		= rza1_irqc_remove,
    .driver		= {
    .name		= "renesas_rza1_irqc",
    .of_match_table	= rza1_irqc_dt_ids,
    }
    };
#[no_mangle]
unsafe extern "C" fn rza1_irqc_init() -> int __init {
    static int __init rza1_irqc_init(void)
    {
    return platform_driver_register(&rza1_irqc_device_driver);
    }
    postcore_initcall(rza1_irqc_init);
#[no_mangle]
unsafe extern "C" fn rza1_irqc_exit() -> void __exit {
    static void __exit rza1_irqc_exit(void)
    {
    platform_driver_unregister(&rza1_irqc_device_driver);
    }
    module_exit(rza1_irqc_exit);
    MODULE_AUTHOR("Geert Uytterhoeven <geert+renesas@glider.be>");
    MODULE_DESCRIPTION("Renesas RZ/A1 IRQC Driver");
