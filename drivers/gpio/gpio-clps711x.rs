//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-clps711x.c
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
// CLPS711X GPIO driver
//
// Copyright (C) 2012,2013 Alexander Shiyan <shc_work@mail.ru>
//

#[no_mangle]
unsafe extern "C" fn clps711x_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int clps711x_gpio_probe(struct platform_device *pdev)
    {
    let mut config: gpio_generic_chip_config = { };
    struct device_node *np = pdev.dev.of_node;
    struct gpio_generic_chip *gen_gc;
    void __iomem *dat, *dir;
    int err, id;
    if (!np)
    return -ENODEV;
    id = of_alias_get_id(np, "gpio");
    if ((id < 0) || (id > 4))
    return -ENODEV;
    gen_gc = devm_kzalloc(&pdev.dev, sizeof(*gen_gc), GFP_KERNEL);
    if (!gen_gc)
    return -ENOMEM;
    dat = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dat))
    return PTR_ERR(dat);
    dir = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(dir))
    return PTR_ERR(dir);
    config.dev = &pdev.dev;
    config.sz = 1;
    config.dat = dat;
    switch (id) {
    case 3:
// PORTD is inverted logic for direction register
    config.dirin = dir;
    break;
    default:
    config.dirout = dir;
    break;
    }
    err = gpio_generic_chip_init(gen_gc, &config);
    if (err)
    return err;
    switch (id) {
    case 4:
// PORTE is 3 lines only
    gen_gc.gc.ngpio = 3;
    break;
    default:
    break;
    }
    gen_gc.gc.base = -1;
    gen_gc.gc.owner = THIS_MODULE;
    return devm_gpiochip_add_data(&pdev.dev, &gen_gc.gc, core::ptr::null_mut());
    }
    static const struct of_device_id clps711x_gpio_ids[] = {
    { .compatible = "cirrus,ep7209-gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, clps711x_gpio_ids);
    static struct platform_driver clps711x_gpio_driver = {
    .driver	= {
    .name		= "clps711x-gpio",
    .of_match_table	= clps711x_gpio_ids,
    },
    .probe	= clps711x_gpio_probe,
    };
    module_platform_driver(clps711x_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alexander Shiyan <shc_work@mail.ru>");
    MODULE_DESCRIPTION("CLPS711X GPIO driver");
    MODULE_ALIAS("platform:clps711x-gpio");
