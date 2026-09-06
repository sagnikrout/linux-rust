//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-keystone.c
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
// Texas Instruments Keystone IRQ controller IP driver
//
// Copyright (C) 2014 Texas Instruments, Inc.
// Author: Sajesh Kumar Saran <sajesh@ti.com>
// Grygorii Strashko <grygorii.strashko@ti.com>
//

// The source ID bits start from 4 to 31 (total 28 bits)
pub const BIT_OFS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keystone_irq_device {
    pub dev: *mut device,
    pub chip: irq_chip,
    pub mask: u32,
    pub irq: c_int,
    pub irqd: *mut irq_domain,
    pub devctrl_regs: *mut regmap,
    pub devctrl_offset: u32,
    pub wa_lock: raw_spinlock_t,
}

#[no_mangle]
pub unsafe extern "C" fn keystone_irq_readl(kirq: *mut keystone_irq_device) -> u32 {
    static inline u32 keystone_irq_readl(struct keystone_irq_device *kirq)
    {
    int ret;
    let mut val: u32 = 0;
    ret = regmap_read(kirq.devctrl_regs, kirq.devctrl_offset, &val);
    if (ret < 0)
    dev_dbg(kirq.dev, "irq read failed ret(%d)\n", ret);
    return val;
    }
    static inline void
    keystone_irq_writel(struct keystone_irq_device *kirq, u32 value)
    {
    int ret;
    ret = regmap_write(kirq.devctrl_regs, kirq.devctrl_offset, value);
    if (ret < 0)
    dev_dbg(kirq.dev, "irq write failed ret(%d)\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn keystone_irq_setmask(d: *mut irq_data) {
    static void keystone_irq_setmask(struct irq_data *d)
    {
    struct keystone_irq_device *kirq = irq_data_get_irq_chip_data(d);
    kirq.mask |= BIT(d.hwirq);
    dev_dbg(kirq.dev, "mask %lu [%x]\n", d.hwirq, kirq.mask);
    }
#[no_mangle]
unsafe extern "C" fn keystone_irq_unmask(d: *mut irq_data) {
    static void keystone_irq_unmask(struct irq_data *d)
    {
    struct keystone_irq_device *kirq = irq_data_get_irq_chip_data(d);
    kirq.mask &= ~BIT(d.hwirq);
    dev_dbg(kirq.dev, "unmask %lu [%x]\n", d.hwirq, kirq.mask);
    }
#[no_mangle]
unsafe extern "C" fn keystone_irq_ack(d: *mut irq_data) {
    static void keystone_irq_ack(struct irq_data *d)
    {
// nothing to do here
    }
#[no_mangle]
unsafe extern "C" fn keystone_irq_handler(irq: c_int, keystone_irq: *mut c_void) -> irqreturn_t {
    static irqreturn_t keystone_irq_handler(int irq, void *keystone_irq)
    {
    struct keystone_irq_device *kirq = keystone_irq;
    unsigned long wa_lock_flags;
    unsigned long pending;
    int src, err;
    dev_dbg(kirq.dev, "start irq %d\n", irq);
    pending = keystone_irq_readl(kirq);
    keystone_irq_writel(kirq, pending);
    dev_dbg(kirq.dev, "pending 0x%lx, mask 0x%x\n", pending, kirq.mask);
    pending = (pending >> BIT_OFS) & ~kirq.mask;
    dev_dbg(kirq.dev, "pending after mask 0x%lx\n", pending);
    for (src = 0; src < KEYSTONE_N_IRQ; src++) {
    if (BIT(src) & pending) {
    raw_spin_lock_irqsave(&kirq.wa_lock, wa_lock_flags);
    err = generic_handle_domain_irq(kirq.irqd, src);
    raw_spin_unlock_irqrestore(&kirq.wa_lock,
    wa_lock_flags);
    if (err)
    dev_warn_ratelimited(kirq.dev, "spurious irq detected hwirq %d\n",
    src);
    }
    }
    dev_dbg(kirq.dev, "end irq %d\n", irq);
    return IRQ_HANDLED;
    }
    static int keystone_irq_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {
    struct keystone_irq_device *kirq = h.host_data;
    irq_set_chip_data(virq, kirq);
    irq_set_chip_and_handler(virq, &kirq.chip, handle_level_irq);
    irq_set_probe(virq);
    return 0;
    }
    static const struct irq_domain_ops keystone_irq_ops = {
    .map	= keystone_irq_map,
    .xlate	= irq_domain_xlate_onecell,
    };
#[no_mangle]
unsafe extern "C" fn keystone_irq_probe(pdev: *mut platform_device) -> c_int {
    static int keystone_irq_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct keystone_irq_device *kirq;
    int ret;
    if (np == core::ptr::null_mut())
    return -EINVAL;
    kirq = devm_kzalloc(dev, sizeof(*kirq), GFP_KERNEL);
    if (!kirq)
    return -ENOMEM;
    kirq.devctrl_regs = syscon_regmap_lookup_by_phandle_args(np, "ti,syscon-dev",
    1, &kirq.devctrl_offset);
    if (IS_ERR(kirq.devctrl_regs))
    return PTR_ERR(kirq.devctrl_regs);
    kirq.irq = platform_get_irq(pdev, 0);
    if (kirq.irq < 0)
    return kirq.irq;
    kirq.dev = dev;
    kirq.mask = ~0x0;
    kirq.chip.name		= "keystone-irq";
    kirq.chip.irq_ack	= keystone_irq_ack;
    kirq.chip.irq_mask	= keystone_irq_setmask;
    kirq.chip.irq_unmask	= keystone_irq_unmask;
    kirq.irqd = irq_domain_create_linear(dev_fwnode(dev), KEYSTONE_N_IRQ, &keystone_irq_ops,
    kirq);
    if (!kirq.irqd) {
    dev_err(dev, "IRQ domain registration failed\n");
    return -ENODEV;
    }
    raw_spin_lock_init(&kirq.wa_lock);
    platform_set_drvdata(pdev, kirq);
    ret = request_irq(kirq.irq, keystone_irq_handler,
    0, dev_name(dev), kirq);
    if (ret) {
    irq_domain_remove(kirq.irqd);
    return ret;
    }
// clear all source bits
    keystone_irq_writel(kirq, ~0x0);
    dev_info(dev, "irqchip registered, nr_irqs %u\n", KEYSTONE_N_IRQ);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keystone_irq_remove(pdev: *mut platform_device) {
    static void keystone_irq_remove(struct platform_device *pdev)
    {
    struct keystone_irq_device *kirq = platform_get_drvdata(pdev);
    int hwirq;
    free_irq(kirq.irq, kirq);
    for (hwirq = 0; hwirq < KEYSTONE_N_IRQ; hwirq++)
    irq_dispose_mapping(irq_find_mapping(kirq.irqd, hwirq));
    irq_domain_remove(kirq.irqd);
    }
    static const struct of_device_id keystone_irq_dt_ids[] = {
    { .compatible = "ti,keystone-irq", },
    {},
    };
    MODULE_DEVICE_TABLE(of, keystone_irq_dt_ids);
    static struct platform_driver keystone_irq_device_driver = {
    .probe		= keystone_irq_probe,
    .remove		= keystone_irq_remove,
    .driver		= {
    .name	= "keystone_irq",
    .of_match_table	= of_match_ptr(keystone_irq_dt_ids),
    }
    };
    module_platform_driver(keystone_irq_device_driver);
    MODULE_AUTHOR("Texas Instruments");
    MODULE_AUTHOR("Sajesh Kumar Saran");
    MODULE_AUTHOR("Grygorii Strashko");
    MODULE_DESCRIPTION("Keystone IRQ chip");
    MODULE_LICENSE("GPL v2");
