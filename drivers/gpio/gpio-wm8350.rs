//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-wm8350.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// gpiolib support for Wolfson WM835x PMICs
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_gpio_data {
    pub wm8350: *mut wm8350,
    pub gpio_chip: gpio_chip,
}

#[no_mangle]
unsafe extern "C" fn wm8350_gpio_direction_in(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int wm8350_gpio_direction_in(struct gpio_chip *chip, unsigned offset)
    {
    struct wm8350_gpio_data *wm8350_gpio = gpiochip_get_data(chip);
    struct wm8350 *wm8350 = wm8350_gpio.wm8350;
    return wm8350_set_bits(wm8350, WM8350_GPIO_CONFIGURATION_I_O,
    1 << offset);
    }
#[no_mangle]
unsafe extern "C" fn wm8350_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int wm8350_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct wm8350_gpio_data *wm8350_gpio = gpiochip_get_data(chip);
    struct wm8350 *wm8350 = wm8350_gpio.wm8350;
    int ret;
    ret = wm8350_reg_read(wm8350, WM8350_GPIO_LEVEL);
    if (ret < 0)
    return ret;
    if (ret & (1 << offset))
    return 1;
    else
    return 0;
    }
    static int wm8350_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct wm8350_gpio_data *wm8350_gpio = gpiochip_get_data(chip);
    struct wm8350 *wm8350 = wm8350_gpio.wm8350;
    if (value)
    return wm8350_set_bits(wm8350, WM8350_GPIO_LEVEL, 1 << offset);
    return wm8350_clear_bits(wm8350, WM8350_GPIO_LEVEL, 1 << offset);
    }
    static int wm8350_gpio_direction_out(struct gpio_chip *chip,
    unsigned offset, int value)
    {
    struct wm8350_gpio_data *wm8350_gpio = gpiochip_get_data(chip);
    struct wm8350 *wm8350 = wm8350_gpio.wm8350;
    int ret;
    ret = wm8350_clear_bits(wm8350, WM8350_GPIO_CONFIGURATION_I_O,
    1 << offset);
    if (ret < 0)
    return ret;
// Don't have an atomic direction/value setup
    return wm8350_gpio_set(chip, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn wm8350_gpio_to_irq(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int wm8350_gpio_to_irq(struct gpio_chip *chip, unsigned offset)
    {
    struct wm8350_gpio_data *wm8350_gpio = gpiochip_get_data(chip);
    struct wm8350 *wm8350 = wm8350_gpio.wm8350;
    if (!wm8350.irq_base)
    return -EINVAL;
    return wm8350.irq_base + WM8350_IRQ_GPIO(offset);
    }
    static const struct gpio_chip template_chip = {
    .label			= "wm8350",
    .owner			= THIS_MODULE,
    .direction_input	= wm8350_gpio_direction_in,
    .get			= wm8350_gpio_get,
    .direction_output	= wm8350_gpio_direction_out,
    .set			= wm8350_gpio_set,
    .to_irq			= wm8350_gpio_to_irq,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn wm8350_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int wm8350_gpio_probe(struct platform_device *pdev)
    {
    struct wm8350 *wm8350 = dev_get_drvdata(pdev.dev.parent);
    struct wm8350_platform_data *pdata = dev_get_platdata(wm8350.dev);
    struct wm8350_gpio_data *wm8350_gpio;
    wm8350_gpio = devm_kzalloc(&pdev.dev, sizeof(*wm8350_gpio),
    GFP_KERNEL);
    if (wm8350_gpio == core::ptr::null_mut())
    return -ENOMEM;
    wm8350_gpio.wm8350 = wm8350;
    wm8350_gpio.gpio_chip = template_chip;
    wm8350_gpio.gpio_chip.ngpio = 13;
    wm8350_gpio.gpio_chip.parent = &pdev.dev;
    if (pdata && pdata.gpio_base)
    wm8350_gpio.gpio_chip.base = pdata.gpio_base;
    else
    wm8350_gpio.gpio_chip.base = -1;
    return devm_gpiochip_add_data(&pdev.dev, &wm8350_gpio.gpio_chip, wm8350_gpio);
    }
    static struct platform_driver wm8350_gpio_driver = {
    .driver.name	= "wm8350-gpio",
    .probe		= wm8350_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn wm8350_gpio_init() -> int __init {
    static int __init wm8350_gpio_init(void)
    {
    return platform_driver_register(&wm8350_gpio_driver);
    }
    subsys_initcall(wm8350_gpio_init);
#[no_mangle]
unsafe extern "C" fn wm8350_gpio_exit() -> void __exit {
    static void __exit wm8350_gpio_exit(void)
    {
    platform_driver_unregister(&wm8350_gpio_driver);
    }
    module_exit(wm8350_gpio_exit);
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_DESCRIPTION("GPIO interface for WM8350 PMICs");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:wm8350-gpio");
