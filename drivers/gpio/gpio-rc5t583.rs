//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-rc5t583.c
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
// GPIO driver for RICOH583 power management chip.
//
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
// Author: Laxman dewangan <ldewangan@nvidia.com>
//
// Based on code
// Copyright (C) 2011 RICOH COMPANY,LTD
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc5t583_gpio {
    pub gpio_chip: gpio_chip,
    pub rc5t583: *mut rc5t583,
}

#[no_mangle]
unsafe extern "C" fn rc5t583_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int rc5t583_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct rc5t583_gpio *rc5t583_gpio = gpiochip_get_data(gc);
    struct device *parent = rc5t583_gpio.rc5t583.dev;
    let mut val: u8 = 0;
    int ret;
    ret = rc5t583_read(parent, RC5T583_GPIO_MON_IOIN, &val);
    if (ret < 0)
    return ret;
    return !!(val & BIT(offset));
    }
#[no_mangle]
unsafe extern "C" fn rc5t583_gpio_set(gc: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int rc5t583_gpio_set(struct gpio_chip *gc, unsigned int offset, int val)
    {
    struct rc5t583_gpio *rc5t583_gpio = gpiochip_get_data(gc);
    struct device *parent = rc5t583_gpio.rc5t583.dev;
    int ret;
    if (val)
    ret = rc5t583_set_bits(parent, RC5T583_GPIO_IOOUT,
    BIT(offset));
    else
    ret = rc5t583_clear_bits(parent, RC5T583_GPIO_IOOUT,
    BIT(offset));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rc5t583_gpio_dir_input(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int rc5t583_gpio_dir_input(struct gpio_chip *gc, unsigned int offset)
    {
    struct rc5t583_gpio *rc5t583_gpio = gpiochip_get_data(gc);
    struct device *parent = rc5t583_gpio.rc5t583.dev;
    int ret;
    ret = rc5t583_clear_bits(parent, RC5T583_GPIO_IOSEL, BIT(offset));
    if (ret < 0)
    return ret;
// Set pin to gpio mode
    return rc5t583_clear_bits(parent, RC5T583_GPIO_PGSEL, BIT(offset));
    }
    static int rc5t583_gpio_dir_output(struct gpio_chip *gc, unsigned offset,
    int value)
    {
    struct rc5t583_gpio *rc5t583_gpio = gpiochip_get_data(gc);
    struct device *parent = rc5t583_gpio.rc5t583.dev;
    int ret;
    ret = rc5t583_gpio_set(gc, offset, value);
    if (ret)
    return ret;
    ret = rc5t583_set_bits(parent, RC5T583_GPIO_IOSEL, BIT(offset));
    if (ret < 0)
    return ret;
// Set pin to gpio mode
    return rc5t583_clear_bits(parent, RC5T583_GPIO_PGSEL, BIT(offset));
    }
#[no_mangle]
unsafe extern "C" fn rc5t583_gpio_to_irq(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int rc5t583_gpio_to_irq(struct gpio_chip *gc, unsigned offset)
    {
    struct rc5t583_gpio *rc5t583_gpio = gpiochip_get_data(gc);
    if (offset < RC5T583_MAX_GPIO)
    return rc5t583_gpio.rc5t583.irq_base +
    RC5T583_IRQ_GPIO0 + offset;
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn rc5t583_gpio_free(gc: *mut gpio_chip, offset: unsigned) {
    static void rc5t583_gpio_free(struct gpio_chip *gc, unsigned offset)
    {
    struct rc5t583_gpio *rc5t583_gpio = gpiochip_get_data(gc);
    struct device *parent = rc5t583_gpio.rc5t583.dev;
    rc5t583_set_bits(parent, RC5T583_GPIO_PGSEL, BIT(offset));
    }
#[no_mangle]
unsafe extern "C" fn rc5t583_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int rc5t583_gpio_probe(struct platform_device *pdev)
    {
    struct rc5t583 *rc5t583 = dev_get_drvdata(pdev.dev.parent);
    struct rc5t583_platform_data *pdata = dev_get_platdata(rc5t583.dev);
    struct rc5t583_gpio *rc5t583_gpio;
    rc5t583_gpio = devm_kzalloc(&pdev.dev, sizeof(*rc5t583_gpio),
    GFP_KERNEL);
    if (!rc5t583_gpio)
    return -ENOMEM;
    rc5t583_gpio.gpio_chip.label = "gpio-rc5t583",
    rc5t583_gpio.gpio_chip.owner = THIS_MODULE,
    rc5t583_gpio.gpio_chip.free = rc5t583_gpio_free,
    rc5t583_gpio.gpio_chip.direction_input = rc5t583_gpio_dir_input,
    rc5t583_gpio.gpio_chip.direction_output = rc5t583_gpio_dir_output,
    rc5t583_gpio.gpio_chip.set = rc5t583_gpio_set,
    rc5t583_gpio.gpio_chip.get = rc5t583_gpio_get,
    rc5t583_gpio.gpio_chip.to_irq = rc5t583_gpio_to_irq,
    rc5t583_gpio.gpio_chip.ngpio = RC5T583_MAX_GPIO,
    rc5t583_gpio.gpio_chip.can_sleep = true,
    rc5t583_gpio.gpio_chip.parent = &pdev.dev;
    rc5t583_gpio.gpio_chip.base = -1;
    rc5t583_gpio.rc5t583 = rc5t583;
    if (pdata && pdata.gpio_base)
    rc5t583_gpio.gpio_chip.base = pdata.gpio_base;
    return devm_gpiochip_add_data(&pdev.dev, &rc5t583_gpio.gpio_chip,
    rc5t583_gpio);
    }
    static struct platform_driver rc5t583_gpio_driver = {
    .driver = {
    .name    = "rc5t583-gpio",
    },
    .probe		= rc5t583_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn rc5t583_gpio_init() -> int __init {
    static int __init rc5t583_gpio_init(void)
    {
    return platform_driver_register(&rc5t583_gpio_driver);
    }
    subsys_initcall(rc5t583_gpio_init);
