//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-74xx-mmio.c
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
// 74xx MMIO GPIO driver
//
// Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmio_74xx_gpio_priv {
    pub gen_gc: gpio_generic_chip,
    pub flags: c_uint,
}

    static const struct of_device_id mmio_74xx_gpio_ids[] = {
    {
    .compatible	= "ti,741g125",
    .data		= (const void *)(MMIO_74XX_DIR_IN | 1),
    },
    {
    .compatible	= "ti,742g125",
    .data		= (const void *)(MMIO_74XX_DIR_IN | 2),
    },
    {
    .compatible	= "ti,74125",
    .data		= (const void *)(MMIO_74XX_DIR_IN | 4),
    },
    {
    .compatible	= "ti,74365",
    .data		= (const void *)(MMIO_74XX_DIR_IN | 6),
    },
    {
    .compatible	= "ti,74244",
    .data		= (const void *)(MMIO_74XX_DIR_IN | 8),
    },
    {
    .compatible	= "ti,741624",
    .data		= (const void *)(MMIO_74XX_DIR_IN | 16),
    },
    {
    .compatible	= "ti,741g74",
    .data		= (const void *)(MMIO_74XX_DIR_OUT | 1),
    },
    {
    .compatible	= "ti,7474",
    .data		= (const void *)(MMIO_74XX_DIR_OUT | 2),
    },
    {
    .compatible	= "ti,74175",
    .data		= (const void *)(MMIO_74XX_DIR_OUT | 4),
    },
    {
    .compatible	= "ti,74174",
    .data		= (const void *)(MMIO_74XX_DIR_OUT | 6),
    },
    {
    .compatible	= "ti,74273",
    .data		= (const void *)(MMIO_74XX_DIR_OUT | 8),
    },
    {
    .compatible	= "ti,7416374",
    .data		= (const void *)(MMIO_74XX_DIR_OUT | 16),
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, mmio_74xx_gpio_ids);
#[no_mangle]
unsafe extern "C" fn mmio_74xx_get_direction(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int mmio_74xx_get_direction(struct gpio_chip *gc, unsigned offset)
    {
    struct mmio_74xx_gpio_priv *priv = gpiochip_get_data(gc);
    if (priv.flags & MMIO_74XX_DIR_OUT)
    return GPIO_LINE_DIRECTION_OUT;
    return  GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn mmio_74xx_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int mmio_74xx_dir_in(struct gpio_chip *gc, unsigned int gpio)
    {
    struct mmio_74xx_gpio_priv *priv = gpiochip_get_data(gc);
    if (priv.flags & MMIO_74XX_DIR_IN)
    return 0;
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn mmio_74xx_dir_out(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    static int mmio_74xx_dir_out(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct mmio_74xx_gpio_priv *priv = gpiochip_get_data(gc);
    if (priv.flags & MMIO_74XX_DIR_OUT)
    return gpio_generic_chip_set(&priv.gen_gc, gpio, val);
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn mmio_74xx_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int mmio_74xx_gpio_probe(struct platform_device *pdev)
    {
    let mut config: gpio_generic_chip_config = { };
    struct mmio_74xx_gpio_priv *priv;
    void __iomem *dat;
    int err;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.flags = (uintptr_t)device_get_match_data(&pdev.dev);
    dat = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dat))
    return PTR_ERR(dat);
    config.dev = &pdev.dev;
    config.sz = DIV_ROUND_UP(MMIO_74XX_BIT_CNT(priv.flags), 8);
    config.dat = dat;
    err = gpio_generic_chip_init(&priv.gen_gc, &config);
    if (err)
    return err;
    priv.gen_gc.gc.direction_input = mmio_74xx_dir_in;
    priv.gen_gc.gc.direction_output = mmio_74xx_dir_out;
    priv.gen_gc.gc.get_direction = mmio_74xx_get_direction;
    priv.gen_gc.gc.ngpio = MMIO_74XX_BIT_CNT(priv.flags);
    priv.gen_gc.gc.owner = THIS_MODULE;
    return devm_gpiochip_add_data(&pdev.dev, &priv.gen_gc.gc, priv);
    }
    static struct platform_driver mmio_74xx_gpio_driver = {
    .driver	= {
    .name		= "74xx-mmio-gpio",
    .of_match_table	= mmio_74xx_gpio_ids,
    },
    .probe	= mmio_74xx_gpio_probe,
    };
    module_platform_driver(mmio_74xx_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alexander Shiyan <shc_work@mail.ru>");
    MODULE_DESCRIPTION("74xx MMIO GPIO driver");
