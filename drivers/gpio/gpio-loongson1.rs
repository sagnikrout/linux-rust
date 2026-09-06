//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-loongson1.c
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
// GPIO Driver for Loongson 1 SoC
//
// Copyright (C) 2015-2023 Keguang Zhang <keguang.zhang@gmail.com>
//

// Loongson 1 GPIO Register Definitions
pub const GPIO_CFG: c_uint = 0x0;
pub const GPIO_DIR: c_uint = 0x10;
pub const GPIO_DATA: c_uint = 0x20;
pub const GPIO_OUTPUT: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls1x_gpio_chip {
    pub chip: gpio_generic_chip,
    pub reg_base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn ls1x_gpio_request(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int ls1x_gpio_request(struct gpio_chip *gc, unsigned int offset)
    {
    struct ls1x_gpio_chip *ls1x_gc = gpiochip_get_data(gc);
    guard(gpio_generic_lock_irqsave)(&ls1x_gc.chip);
    __raw_writel(__raw_readl(ls1x_gc.reg_base + GPIO_CFG) | BIT(offset),
    ls1x_gc.reg_base + GPIO_CFG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1x_gpio_free(gc: *mut gpio_chip, offset: c_uint) {
    static void ls1x_gpio_free(struct gpio_chip *gc, unsigned int offset)
    {
    struct ls1x_gpio_chip *ls1x_gc = gpiochip_get_data(gc);
    guard(gpio_generic_lock_irqsave)(&ls1x_gc.chip);
    __raw_writel(__raw_readl(ls1x_gc.reg_base + GPIO_CFG) & ~BIT(offset),
    ls1x_gc.reg_base + GPIO_CFG);
    }
#[no_mangle]
unsafe extern "C" fn ls1x_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int ls1x_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct ls1x_gpio_chip *ls1x_gc;
    int ret;
    ls1x_gc = devm_kzalloc(dev, sizeof(*ls1x_gc), GFP_KERNEL);
    if (!ls1x_gc)
    return -ENOMEM;
    ls1x_gc.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ls1x_gc.reg_base))
    return PTR_ERR(ls1x_gc.reg_base);
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = ls1x_gc.reg_base + GPIO_DATA,
    .set = ls1x_gc.reg_base + GPIO_OUTPUT,
    .dirin = ls1x_gc.reg_base + GPIO_DIR,
    };
    ret = gpio_generic_chip_init(&ls1x_gc.chip, &config);
    if (ret)
    goto err;
    ls1x_gc.chip.gc.owner = THIS_MODULE;
    ls1x_gc.chip.gc.request = ls1x_gpio_request;
    ls1x_gc.chip.gc.free = ls1x_gpio_free;
//
// Clear ngpio to let gpiolib get the correct number
// by reading ngpios property
//
    ls1x_gc.chip.gc.ngpio = 0;
    ret = devm_gpiochip_add_data(dev, &ls1x_gc.chip.gc, ls1x_gc);
    if (ret)
    goto err;
    platform_set_drvdata(pdev, ls1x_gc);
    dev_info(dev, "GPIO controller registered with %d pins\n",
    ls1x_gc.chip.gc.ngpio);
    return 0;
    err:
    dev_err(dev, "failed to register GPIO controller\n");
    return ret;
    }
    static const struct of_device_id ls1x_gpio_dt_ids[] = {
    { .compatible = "loongson,ls1x-gpio" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ls1x_gpio_dt_ids);
    static struct platform_driver ls1x_gpio_driver = {
    .probe	= ls1x_gpio_probe,
    .driver	= {
    .name	= "ls1x-gpio",
    .of_match_table = ls1x_gpio_dt_ids,
    },
    };
    module_platform_driver(ls1x_gpio_driver);
    MODULE_AUTHOR("Keguang Zhang <keguang.zhang@gmail.com>");
    MODULE_DESCRIPTION("Loongson1 GPIO driver");
    MODULE_LICENSE("GPL");
