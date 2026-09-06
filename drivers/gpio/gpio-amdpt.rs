//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-amdpt.c
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
// AMD Promontory GPIO driver
//
// Copyright (C) 2015 ASMedia Technology Inc.
// Author: YD Tseng <yd_tseng@asmedia.com.tw>
//

pub const PT_TOTAL_GPIO: c_int = 8;
pub const PT_TOTAL_GPIO_EX: c_int = 24;
// PCI-E MMIO register offsets
pub const PT_DIRECTION_REG: c_uint = 0x00;
pub const PT_INPUTDATA_REG: c_uint = 0x04;
pub const PT_OUTPUTDATA_REG: c_uint = 0x08;
pub const PT_CLOCKRATE_REG: c_uint = 0x0C;
pub const PT_SYNC_REG: c_uint = 0x28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_gpio_chip {
    pub chip: gpio_generic_chip,
    pub reg_base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn pt_gpio_request(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int pt_gpio_request(struct gpio_chip *gc, unsigned offset)
    {
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    struct pt_gpio_chip *pt_gpio = gpiochip_get_data(gc);
    u32 using_pins;
    dev_dbg(gc.parent, "pt_gpio_request offset=%x\n", offset);
    guard(gpio_generic_lock_irqsave)(gen_gc);
    using_pins = readl(pt_gpio.reg_base + PT_SYNC_REG);
    if (using_pins & BIT(offset)) {
    dev_warn(gc.parent, "PT GPIO pin %x reconfigured\n",
    offset);
    return -EINVAL;
    }
    writel(using_pins | BIT(offset), pt_gpio.reg_base + PT_SYNC_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pt_gpio_free(gc: *mut gpio_chip, offset: unsigned) {
    static void pt_gpio_free(struct gpio_chip *gc, unsigned offset)
    {
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    struct pt_gpio_chip *pt_gpio = gpiochip_get_data(gc);
    u32 using_pins;
    guard(gpio_generic_lock_irqsave)(gen_gc);
    using_pins = readl(pt_gpio.reg_base + PT_SYNC_REG);
    using_pins &= ~BIT(offset);
    writel(using_pins, pt_gpio.reg_base + PT_SYNC_REG);
    dev_dbg(gc.parent, "pt_gpio_free offset=%x\n", offset);
    }
#[no_mangle]
unsafe extern "C" fn pt_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int pt_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct pt_gpio_chip *pt_gpio;
    let mut ret: c_int = 0;
    if (!ACPI_COMPANION(dev)) {
    dev_err(dev, "PT GPIO device node not found\n");
    return -ENODEV;
    }
    pt_gpio = devm_kzalloc(dev, sizeof(struct pt_gpio_chip), GFP_KERNEL);
    if (!pt_gpio)
    return -ENOMEM;
    pt_gpio.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pt_gpio.reg_base)) {
    dev_err(dev, "Failed to map MMIO resource for PT GPIO.\n");
    return PTR_ERR(pt_gpio.reg_base);
    }
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = pt_gpio.reg_base + PT_INPUTDATA_REG,
    .set = pt_gpio.reg_base + PT_OUTPUTDATA_REG,
    .dirout = pt_gpio.reg_base + PT_DIRECTION_REG,
    .flags = GPIO_GENERIC_READ_OUTPUT_REG_SET,
    };
    ret = gpio_generic_chip_init(&pt_gpio.chip, &config);
    if (ret) {
    dev_err(dev, "failed to initialize the generic GPIO chip\n");
    return ret;
    }
    pt_gpio.chip.gc.owner = THIS_MODULE;
    pt_gpio.chip.gc.request = pt_gpio_request;
    pt_gpio.chip.gc.free = pt_gpio_free;
    pt_gpio.chip.gc.ngpio = (uintptr_t)device_get_match_data(dev);
    ret = devm_gpiochip_add_data(dev, &pt_gpio.chip.gc, pt_gpio);
    if (ret) {
    dev_err(dev, "Failed to register GPIO lib\n");
    return ret;
    }
    platform_set_drvdata(pdev, pt_gpio);
// initialize register setting
    writel(0, pt_gpio.reg_base + PT_SYNC_REG);
    writel(0, pt_gpio.reg_base + PT_CLOCKRATE_REG);
    dev_dbg(dev, "PT GPIO driver loaded\n");
    return ret;
    }
    static const struct acpi_device_id pt_gpio_acpi_match[] = {
    { "AMDF030", PT_TOTAL_GPIO },
    { "AMDIF030", PT_TOTAL_GPIO },
    { "AMDIF031", PT_TOTAL_GPIO_EX },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, pt_gpio_acpi_match);
    static struct platform_driver pt_gpio_driver = {
    .driver = {
    .name = "pt-gpio",
    .acpi_match_table = ACPI_PTR(pt_gpio_acpi_match),
    },
    .probe = pt_gpio_probe,
    };
    module_platform_driver(pt_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("YD Tseng <yd_tseng@asmedia.com.tw>");
    MODULE_DESCRIPTION("AMD Promontory GPIO Driver");
