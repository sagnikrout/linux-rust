//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-renesas-irqc.c
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
// Renesas IRQC Driver
//
// Copyright (C) 2013 Magnus Damm
//

pub const IRQC_REQ_STS: c_uint = 0x00	/* Interrupt Request Status Register */;
pub const IRQC_EN_STS: c_uint = 0x04	/* Interrupt Enable Status Register */;
pub const IRQC_EN_SET: c_uint = 0x08	/* Interrupt Enable Set Register */;

// SYS-CPU vs. RT-CPU
pub const DETECT_STATUS: c_uint = 0x100	/* IRQn Detect Status Register */;
pub const MONITOR: c_uint = 0x104	/* IRQn Signal Level Monitor Register */;
pub const HLVL_STS: c_uint = 0x108	/* IRQn High Level Detect Status Register */;
pub const LLVL_STS: c_uint = 0x10c	/* IRQn Low Level Detect Status Register */;
pub const S_R_EDGE_STS: c_uint = 0x110	/* IRQn Sync Rising Edge Detect Status Reg. */;
pub const S_F_EDGE_STS: c_uint = 0x114	/* IRQn Sync Falling Edge Detect Status Reg. */;
pub const A_R_EDGE_STS: c_uint = 0x118	/* IRQn Async Rising Edge Detect Status Reg. */;
pub const A_F_EDGE_STS: c_uint = 0x11c	/* IRQn Async Falling Edge Detect Status Reg. */;
pub const CHTEN_STS: c_uint = 0x120	/* Chattering Reduction Status Register */;

// IRQn Configuration Register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irqc_irq {
    pub hw_irq: c_int,
    pub requested_irq: c_int,
    pub p: *mut irqc_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irqc_priv {
    pub iomem: *mut void __iomem,
    pub cpu_int_base: *mut void __iomem,
    pub irq: [irqc_irq; IRQC_IRQ_MAX],
    pub number_of_irqs: c_uint,
    pub dev: *mut device,
    pub gc: *mut irq_chip_generic,
    pub irq_domain: *mut irq_domain,
    pub wakeup_path: core::sync::atomic::AtomicI32,
}

    static struct irqc_priv *irq_data_to_priv(struct irq_data *data)
    {
    return data.domain.host_data;
    }
#[no_mangle]
unsafe extern "C" fn irqc_dbg(i: *mut irqc_irq, str: *mut c_char) {
    static void irqc_dbg(struct irqc_irq *i, char *str)
    {
    dev_dbg(i.p.dev, "%s (%d:%d)\n", str, i.requested_irq, i.hw_irq);
    }
    static unsigned char irqc_sense[IRQ_TYPE_SENSE_MASK + 1] = {
    [IRQ_TYPE_LEVEL_LOW]	= 0x01,
    [IRQ_TYPE_LEVEL_HIGH]	= 0x02,
    [IRQ_TYPE_EDGE_FALLING]	= 0x04,	/* Synchronous */
    [IRQ_TYPE_EDGE_RISING]	= 0x08,	/* Synchronous */
    [IRQ_TYPE_EDGE_BOTH]	= 0x0c,	/* Synchronous */
    };
#[no_mangle]
unsafe extern "C" fn irqc_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int irqc_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct irqc_priv *p = irq_data_to_priv(d);
    let mut hw_irq: c_int = irqd_to_hwirq(d);
    let mut value: c_uchar = irqc_sense[type & IRQ_TYPE_SENSE_MASK];
    u32 tmp;
    irqc_dbg(&p.irq[hw_irq], "sense");
    if (!value)
    return -EINVAL;
    tmp = ioread32(p.iomem + IRQC_CONFIG(hw_irq));
    tmp &= ~0x3f;
    tmp |= value;
    iowrite32(tmp, p.iomem + IRQC_CONFIG(hw_irq));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irqc_irq_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    static int irqc_irq_set_wake(struct irq_data *d, unsigned int on)
    {
    struct irqc_priv *p = irq_data_to_priv(d);
    let mut hw_irq: c_int = irqd_to_hwirq(d);
    irq_set_irq_wake(p.irq[hw_irq].requested_irq, on);
    if (on)
    atomic_inc(&p.wakeup_path);
    else
    atomic_dec(&p.wakeup_path);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irqc_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t irqc_irq_handler(int irq, void *dev_id)
    {
    struct irqc_irq *i = dev_id;
    struct irqc_priv *p = i.p;
    let mut bit: u32 = BIT(i.hw_irq);
    irqc_dbg(i, "demux1");
    if (ioread32(p.iomem + DETECT_STATUS) & bit) {
    iowrite32(bit, p.iomem + DETECT_STATUS);
    irqc_dbg(i, "demux2");
    generic_handle_domain_irq(p.irq_domain, i.hw_irq);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn irqc_probe(pdev: *mut platform_device) -> c_int {
    static int irqc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const char *name = dev_name(dev);
    struct irqc_priv *p;
    int ret;
    int k;
    p = devm_kzalloc(dev, sizeof(*p), GFP_KERNEL);
    if (!p)
    return -ENOMEM;
    p.dev = dev;
    platform_set_drvdata(pdev, p);
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
// allow any number of IRQs between 1 and IRQC_IRQ_MAX
    for (k = 0; k < IRQC_IRQ_MAX; k++) {
    ret = platform_get_irq_optional(pdev, k);
    if (ret == -ENXIO)
    break;
    if (ret < 0)
    goto err_runtime_pm_disable;
    p.irq[k].p = p;
    p.irq[k].hw_irq = k;
    p.irq[k].requested_irq = ret;
    }
    p.number_of_irqs = k;
    if (p.number_of_irqs < 1) {
    dev_err(dev, "not enough IRQ resources\n");
    ret = -EINVAL;
    goto err_runtime_pm_disable;
    }
// ioremap IOMEM and setup read/write callbacks
    p.iomem = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(p.iomem)) {
    ret = PTR_ERR(p.iomem);
    goto err_runtime_pm_disable;
    }
    p.cpu_int_base = p.iomem + IRQC_INT_CPU_BASE(0); /* SYS-SPI */
    p.irq_domain = irq_domain_create_linear(dev_fwnode(dev), p.number_of_irqs,
    &irq_generic_chip_ops, p);
    if (!p.irq_domain) {
    ret = -ENXIO;
    dev_err(dev, "cannot initialize irq domain\n");
    goto err_runtime_pm_disable;
    }
    p.irq_domain.flags |= IRQ_DOMAIN_FLAG_DESTROY_GC;
    ret = irq_alloc_domain_generic_chips(p.irq_domain, p.number_of_irqs,
    1, "irqc", handle_level_irq,
    0, 0, IRQ_GC_INIT_NESTED_LOCK);
    if (ret) {
    dev_err(dev, "cannot allocate generic chip\n");
    goto err_remove_domain;
    }
    p.gc = irq_get_domain_generic_chip(p.irq_domain, 0);
    p.gc.reg_base = p.cpu_int_base;
    p.gc.chip_types[0].regs.enable = IRQC_EN_SET;
    p.gc.chip_types[0].regs.disable = IRQC_EN_STS;
    p.gc.chip_types[0].chip.irq_mask = irq_gc_mask_disable_reg;
    p.gc.chip_types[0].chip.irq_unmask = irq_gc_unmask_enable_reg;
    p.gc.chip_types[0].chip.irq_set_type	= irqc_irq_set_type;
    p.gc.chip_types[0].chip.irq_set_wake	= irqc_irq_set_wake;
    p.gc.chip_types[0].chip.flags	= IRQCHIP_MASK_ON_SUSPEND;
    irq_domain_set_pm_device(p.irq_domain, dev);
// request interrupts one by one
    for (k = 0; k < p.number_of_irqs; k++) {
    if (devm_request_irq(dev, p.irq[k].requested_irq,
    irqc_irq_handler, 0, name, &p.irq[k])) {
    dev_err(dev, "failed to request IRQ\n");
    ret = -ENOENT;
    goto err_remove_domain;
    }
    }
    dev_info(dev, "driving %d irqs\n", p.number_of_irqs);
    return 0;
    err_remove_domain:
    irq_domain_remove(p.irq_domain);
    err_runtime_pm_disable:
    pm_runtime_put(dev);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn irqc_remove(pdev: *mut platform_device) {
    static void irqc_remove(struct platform_device *pdev)
    {
    struct irqc_priv *p = platform_get_drvdata(pdev);
    irq_domain_remove(p.irq_domain);
    pm_runtime_put(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn irqc_suspend(dev: *mut device) -> c_int {
    static int irqc_suspend(struct device *dev)
    {
    struct irqc_priv *p = dev_get_drvdata(dev);
    if (atomic_read(&p.wakeup_path))
    device_set_wakeup_path(dev);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(irqc_pm_ops, irqc_suspend, core::ptr::null_mut());
    static const struct of_device_id irqc_dt_ids[] = {
    { .compatible = "renesas,irqc", },
    {},
    };
    MODULE_DEVICE_TABLE(of, irqc_dt_ids);
    static struct platform_driver irqc_device_driver = {
    .probe		= irqc_probe,
    .remove		= irqc_remove,
    .driver		= {
    .name		= "renesas_irqc",
    .of_match_table	= irqc_dt_ids,
    .pm		= pm_sleep_ptr(&irqc_pm_ops),
    }
    };
#[no_mangle]
unsafe extern "C" fn irqc_init() -> int __init {
    static int __init irqc_init(void)
    {
    return platform_driver_register(&irqc_device_driver);
    }
    postcore_initcall(irqc_init);
#[no_mangle]
unsafe extern "C" fn irqc_exit() -> void __exit {
    static void __exit irqc_exit(void)
    {
    platform_driver_unregister(&irqc_device_driver);
    }
    module_exit(irqc_exit);
    MODULE_AUTHOR("Magnus Damm");
    MODULE_DESCRIPTION("Renesas IRQC Driver");
