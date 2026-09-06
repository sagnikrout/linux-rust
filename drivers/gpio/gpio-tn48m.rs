//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tn48m.c
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
// Delta TN48M CPLD GPIO driver
//
// Copyright (C) 2021 Sartura Ltd.
//
// Author: Robert Marko <robert.marko@sartura.hr>
//

    enum tn48m_gpio_type {
    TN48M_GP0 = 1,
    TN48M_GPI,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn48m_gpio_config {
    pub ngpio: c_int,
    pub ngpio_per_reg: c_int,
    pub type: enum tn48m_gpio_type,
}

    static const struct tn48m_gpio_config tn48m_gpo_config = {
    .ngpio = 4,
    .ngpio_per_reg = 4,
    .type = TN48M_GP0,
    };
    static const struct tn48m_gpio_config tn48m_gpi_config = {
    .ngpio = 4,
    .ngpio_per_reg = 4,
    .type = TN48M_GPI,
    };
#[no_mangle]
unsafe extern "C" fn tn48m_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int tn48m_gpio_probe(struct platform_device *pdev)
    {
    const struct tn48m_gpio_config *gpio_config;
    let mut config: gpio_regmap_config = {};
    struct regmap *regmap;
    u32 base;
    int ret;
    if (!pdev.dev.parent)
    return -ENODEV;
    gpio_config = device_get_match_data(&pdev.dev);
    if (!gpio_config)
    return -ENODEV;
    ret = device_property_read_u32(&pdev.dev, "reg", &base);
    if (ret)
    return ret;
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap)
    return -ENODEV;
    config.regmap = regmap;
    config.parent = &pdev.dev;
    config.ngpio = gpio_config.ngpio;
    config.ngpio_per_reg = gpio_config.ngpio_per_reg;
    switch (gpio_config.type) {
    case TN48M_GP0:
    config.reg_set_base = base;
    break;
    case TN48M_GPI:
    config.reg_dat_base = base;
    break;
    default:
    return -EINVAL;
    }
    return PTR_ERR_OR_ZERO(devm_gpio_regmap_register(&pdev.dev, &config));
    }
    static const struct of_device_id tn48m_gpio_of_match[] = {
    { .compatible = "delta,tn48m-gpo", .data = &tn48m_gpo_config },
    { .compatible = "delta,tn48m-gpi", .data = &tn48m_gpi_config },
    { }
    };
    MODULE_DEVICE_TABLE(of, tn48m_gpio_of_match);
    static struct platform_driver tn48m_gpio_driver = {
    .driver = {
    .name = "delta-tn48m-gpio",
    .of_match_table = tn48m_gpio_of_match,
    },
    .probe = tn48m_gpio_probe,
    };
    module_platform_driver(tn48m_gpio_driver);
    MODULE_AUTHOR("Robert Marko <robert.marko@sartura.hr>");
    MODULE_DESCRIPTION("Delta TN48M CPLD GPIO driver");
    MODULE_LICENSE("GPL");
