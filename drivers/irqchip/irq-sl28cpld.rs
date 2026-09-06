//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-sl28cpld.c
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
// sl28cpld interrupt controller driver
//
// Copyright 2020 Kontron Europe GmbH
//

pub const INTC_IE: c_uint = 0x00;
pub const INTC_IP: c_uint = 0x01;
    static const struct regmap_irq sl28cpld_irqs[] = {
    REGMAP_IRQ_REG_LINE(0, 8),
    REGMAP_IRQ_REG_LINE(1, 8),
    REGMAP_IRQ_REG_LINE(2, 8),
    REGMAP_IRQ_REG_LINE(3, 8),
    REGMAP_IRQ_REG_LINE(4, 8),
    REGMAP_IRQ_REG_LINE(5, 8),
    REGMAP_IRQ_REG_LINE(6, 8),
    REGMAP_IRQ_REG_LINE(7, 8),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl28cpld_intc {
    pub regmap: *mut regmap,
    pub chip: regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,
}

#[no_mangle]
unsafe extern "C" fn sl28cpld_intc_probe(pdev: *mut platform_device) -> c_int {
    static int sl28cpld_intc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sl28cpld_intc *irqchip;
    int irq;
    u32 base;
    int ret;
    if (!dev.parent)
    return -ENODEV;
    irqchip = devm_kzalloc(dev, sizeof(*irqchip), GFP_KERNEL);
    if (!irqchip)
    return -ENOMEM;
    irqchip.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!irqchip.regmap)
    return -ENODEV;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = device_property_read_u32(&pdev.dev, "reg", &base);
    if (ret)
    return -EINVAL;
    irqchip.chip.name = "sl28cpld-intc";
    irqchip.chip.irqs = sl28cpld_irqs;
    irqchip.chip.num_irqs = ARRAY_SIZE(sl28cpld_irqs);
    irqchip.chip.num_regs = 1;
    irqchip.chip.status_base = base + INTC_IP;
    irqchip.chip.unmask_base = base + INTC_IE;
    irqchip.chip.ack_base = base + INTC_IP;
    return devm_regmap_add_irq_chip_fwnode(dev, dev_fwnode(dev),
    irqchip.regmap, irq,
    IRQF_SHARED | IRQF_ONESHOT, 0,
    &irqchip.chip,
    &irqchip.irq_data);
    }
    static const struct of_device_id sl28cpld_intc_of_match[] = {
    { .compatible = "kontron,sl28cpld-intc" },
    {}
    };
    MODULE_DEVICE_TABLE(of, sl28cpld_intc_of_match);
    static struct platform_driver sl28cpld_intc_driver = {
    .probe = sl28cpld_intc_probe,
    .driver = {
    .name = "sl28cpld-intc",
    .of_match_table = sl28cpld_intc_of_match,
    }
    };
    module_platform_driver(sl28cpld_intc_driver);
    MODULE_DESCRIPTION("sl28cpld Interrupt Controller Driver");
    MODULE_AUTHOR("Michael Walle <michael@walle.cc>");
