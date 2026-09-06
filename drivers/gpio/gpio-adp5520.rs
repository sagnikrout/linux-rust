//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-adp5520.c
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
// GPIO driver for Analog Devices ADP5520 MFD PMICs
//
// Copyright 2009 Analog Devices Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5520_gpio {
    pub master: *mut device,
    pub gpio_chip: gpio_chip,
    pub lut: [c_uchar; ADP5520_MAXGPIOS],
    pub output: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn adp5520_gpio_get_value(chip: *mut gpio_chip, off: unsigned) -> c_int {
    static int adp5520_gpio_get_value(struct gpio_chip *chip, unsigned off)
    {
    struct adp5520_gpio *dev;
    uint8_t reg_val;
    dev = gpiochip_get_data(chip);
//
// There are dedicated registers for GPIO IN/OUT.
// Make sure we return the right value, even when configured as output
//
    if (test_bit(off, &dev.output))
    adp5520_read(dev.master, ADP5520_GPIO_OUT, &reg_val);
    else
    adp5520_read(dev.master, ADP5520_GPIO_IN, &reg_val);
    return !!(reg_val & dev.lut[off]);
    }
    static int adp5520_gpio_set_value(struct gpio_chip *chip,
    unsigned int off, int val)
    {
    struct adp5520_gpio *dev;
    dev = gpiochip_get_data(chip);
    if (val)
    return adp5520_set_bits(dev.master, ADP5520_GPIO_OUT,
    dev.lut[off]);
    else
    return adp5520_clr_bits(dev.master, ADP5520_GPIO_OUT,
    dev.lut[off]);
    }
#[no_mangle]
unsafe extern "C" fn adp5520_gpio_direction_input(chip: *mut gpio_chip, off: unsigned) -> c_int {
    static int adp5520_gpio_direction_input(struct gpio_chip *chip, unsigned off)
    {
    struct adp5520_gpio *dev;
    dev = gpiochip_get_data(chip);
    clear_bit(off, &dev.output);
    return adp5520_clr_bits(dev.master, ADP5520_GPIO_CFG_2,
    dev.lut[off]);
    }
    static int adp5520_gpio_direction_output(struct gpio_chip *chip,
    unsigned off, int val)
    {
    struct adp5520_gpio *dev;
    let mut ret: c_int = 0;
    dev = gpiochip_get_data(chip);
    set_bit(off, &dev.output);
    if (val)
    ret |= adp5520_set_bits(dev.master, ADP5520_GPIO_OUT,
    dev.lut[off]);
    else
    ret |= adp5520_clr_bits(dev.master, ADP5520_GPIO_OUT,
    dev.lut[off]);
    ret |= adp5520_set_bits(dev.master, ADP5520_GPIO_CFG_2,
    dev.lut[off]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp5520_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int adp5520_gpio_probe(struct platform_device *pdev)
    {
    struct adp5520_gpio_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct adp5520_gpio *dev;
    struct gpio_chip *gc;
    int ret, i, gpios;
    let mut ctl_mask: c_uchar = 0;
    if (pdata == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "missing platform data\n");
    return -ENODEV;
    }
    if (pdev.id != ID_ADP5520) {
    dev_err(&pdev.dev, "only ADP5520 supports GPIO\n");
    return -ENODEV;
    }
    dev = devm_kzalloc(&pdev.dev, sizeof(*dev), GFP_KERNEL);
    if (dev == core::ptr::null_mut())
    return -ENOMEM;
    dev.master = pdev.dev.parent;
    for (gpios = 0, i = 0; i < ADP5520_MAXGPIOS; i++)
    if (pdata.gpio_en_mask & (1 << i))
    dev.lut[gpios++] = 1 << i;
    if (gpios < 1)
    return -EINVAL;
    gc = &dev.gpio_chip;
    gc.direction_input  = adp5520_gpio_direction_input;
    gc.direction_output = adp5520_gpio_direction_output;
    gc.get = adp5520_gpio_get_value;
    gc.set = adp5520_gpio_set_value;
    gc.can_sleep = true;
    gc.base = pdata.gpio_start;
    gc.ngpio = gpios;
    gc.label = pdev.name;
    gc.owner = THIS_MODULE;
    ret = adp5520_clr_bits(dev.master, ADP5520_GPIO_CFG_1,
    pdata.gpio_en_mask);
    if (pdata.gpio_en_mask & ADP5520_GPIO_C3)
    ctl_mask |= ADP5520_C3_MODE;
    if (pdata.gpio_en_mask & ADP5520_GPIO_R3)
    ctl_mask |= ADP5520_R3_MODE;
    if (ctl_mask)
    ret = adp5520_set_bits(dev.master, ADP5520_LED_CONTROL,
    ctl_mask);
    ret |= adp5520_set_bits(dev.master, ADP5520_GPIO_PULLUP,
    pdata.gpio_pullup_mask);
    if (ret) {
    dev_err(&pdev.dev, "failed to write\n");
    return ret;
    }
    return devm_gpiochip_add_data(&pdev.dev, &dev.gpio_chip, dev);
    }
    static struct platform_driver adp5520_gpio_driver = {
    .driver	= {
    .name	= "adp5520-gpio",
    },
    .probe		= adp5520_gpio_probe,
    };
    module_platform_driver(adp5520_gpio_driver);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("GPIO ADP5520 Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:adp5520-gpio");
