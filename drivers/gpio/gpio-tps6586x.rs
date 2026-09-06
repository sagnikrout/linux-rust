//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tps6586x.c
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
// TI TPS6586x GPIO driver
//
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
// Author: Laxman dewangan <ldewangan@nvidia.com>
//
// Based on tps6586x.c
// Copyright (c) 2010 CompuLab Ltd.
// Mike Rapoport <mike@compulab.co.il>
//

// GPIO control registers
pub const TPS6586X_GPIOSET1: c_uint = 0x5d;
pub const TPS6586X_GPIOSET2: c_uint = 0x5e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6586x_gpio {
    pub gpio_chip: gpio_chip,
    pub parent: *mut device,
}

#[no_mangle]
unsafe extern "C" fn tps6586x_gpio_get(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int tps6586x_gpio_get(struct gpio_chip *gc, unsigned offset)
    {
    struct tps6586x_gpio *tps6586x_gpio = gpiochip_get_data(gc);
    uint8_t val;
    int ret;
    ret = tps6586x_read(tps6586x_gpio.parent, TPS6586X_GPIOSET2, &val);
    if (ret)
    return ret;
    return !!(val & (1 << offset));
    }
    static int tps6586x_gpio_set(struct gpio_chip *gc, unsigned int offset,
    int value)
    {
    struct tps6586x_gpio *tps6586x_gpio = gpiochip_get_data(gc);
    return tps6586x_update(tps6586x_gpio.parent, TPS6586X_GPIOSET2,
    value << offset, 1 << offset);
    }
    static int tps6586x_gpio_output(struct gpio_chip *gc, unsigned offset,
    int value)
    {
    struct tps6586x_gpio *tps6586x_gpio = gpiochip_get_data(gc);
    uint8_t val, mask;
    int ret;
    ret = tps6586x_gpio_set(gc, offset, value);
    if (ret)
    return ret;
    val = 0x1 << (offset * 2);
    mask = 0x3 << (offset * 2);
    return tps6586x_update(tps6586x_gpio.parent, TPS6586X_GPIOSET1,
    val, mask);
    }
#[no_mangle]
unsafe extern "C" fn tps6586x_gpio_to_irq(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int tps6586x_gpio_to_irq(struct gpio_chip *gc, unsigned offset)
    {
    struct tps6586x_gpio *tps6586x_gpio = gpiochip_get_data(gc);
    return tps6586x_irq_get_virq(tps6586x_gpio.parent,
    TPS6586X_INT_PLDO_0 + offset);
    }
#[no_mangle]
unsafe extern "C" fn tps6586x_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int tps6586x_gpio_probe(struct platform_device *pdev)
    {
    struct tps6586x_platform_data *pdata;
    struct tps6586x_gpio *tps6586x_gpio;
    device_set_node(&pdev.dev, dev_fwnode(pdev.dev.parent));
    pdata = dev_get_platdata(pdev.dev.parent);
    tps6586x_gpio = devm_kzalloc(&pdev.dev,
    sizeof(*tps6586x_gpio), GFP_KERNEL);
    if (!tps6586x_gpio)
    return -ENOMEM;
    tps6586x_gpio.parent = pdev.dev.parent;
    tps6586x_gpio.gpio_chip.owner = THIS_MODULE;
    tps6586x_gpio.gpio_chip.label = pdev.name;
    tps6586x_gpio.gpio_chip.parent = &pdev.dev;
    tps6586x_gpio.gpio_chip.ngpio = 4;
    tps6586x_gpio.gpio_chip.can_sleep = true;
// FIXME: add handling of GPIOs as dedicated inputs
    tps6586x_gpio.gpio_chip.direction_output = tps6586x_gpio_output;
    tps6586x_gpio.gpio_chip.set	= tps6586x_gpio_set;
    tps6586x_gpio.gpio_chip.get	= tps6586x_gpio_get;
    tps6586x_gpio.gpio_chip.to_irq	= tps6586x_gpio_to_irq;
    if (pdata && pdata.gpio_base)
    tps6586x_gpio.gpio_chip.base = pdata.gpio_base;
    else
    tps6586x_gpio.gpio_chip.base = -1;
    return devm_gpiochip_add_data(&pdev.dev, &tps6586x_gpio.gpio_chip,
    tps6586x_gpio);
    }
    static struct platform_driver tps6586x_gpio_driver = {
    .driver.name	= "tps6586x-gpio",
    .probe		= tps6586x_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn tps6586x_gpio_init() -> int __init {
    static int __init tps6586x_gpio_init(void)
    {
    return platform_driver_register(&tps6586x_gpio_driver);
    }
    subsys_initcall(tps6586x_gpio_init);
