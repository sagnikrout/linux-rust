//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-zynqmp-modepin.c
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
// Driver for the ps-mode pin configuration.
//
// Copyright (c) 2021 Xilinx, Inc.
//

// 4-bit boot mode pins
pub const MODE_PINS: c_int = 4;
//
// modepin_gpio_get_value - Get the state of the specified pin of GPIO device
// @chip:	gpio_chip instance to be worked on
// @pin:	gpio pin number within the device
//
// This function reads the state of the specified pin of the GPIO device.
//
// Return: 0 if the pin is low, 1 if pin is high, -EINVAL wrong pin configured
// or error value.
//
#[no_mangle]
unsafe extern "C" fn modepin_gpio_get_value(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    static int modepin_gpio_get_value(struct gpio_chip *chip, unsigned int pin)
    {
    let mut regval: u32 = 0;
    int ret;
    ret = zynqmp_pm_bootmode_read(&regval);
    if (ret)
    return ret;
// When [0:3] corresponding bit is set, then read output bit [8:11],
// if the bit is clear then read input bit [4:7] for status or value.
//
    if (regval & BIT(pin))
    return !!(regval & BIT(pin + 8));
    else
    return !!(regval & BIT(pin + 4));
    }
//
// modepin_gpio_set_value - Modify the state of the pin with specified value
// @chip:	gpio_chip instance to be worked on
// @pin:	gpio pin number within the device
// @state:	value used to modify the state of the specified pin
//
// This function reads the state of the specified pin of the GPIO device, mask
// with the capture state of GPIO pin, and update pin of GPIO device.
//
// Return:	None.
//
    static int modepin_gpio_set_value(struct gpio_chip *chip, unsigned int pin,
    int state)
    {
    let mut bootpin_val: u32 = 0;
    int ret;
    zynqmp_pm_bootmode_read(&bootpin_val);
// Configure pin as an output by set bit [0:3]
    bootpin_val |= BIT(pin);
    if (state)
    bootpin_val |= BIT(pin + 8);
    else
    bootpin_val &= ~BIT(pin + 8);
// Configure bootpin value
    ret = zynqmp_pm_bootmode_write(bootpin_val);
    if (ret)
    pr_err("modepin: set value error %d for pin %d\n", ret, pin);
    return ret;
    }
//
// modepin_gpio_dir_in - Set the direction of the specified GPIO pin as input
// @chip:	gpio_chip instance to be worked on
// @pin:	gpio pin number within the device
//
// Return: 0 always
//
#[no_mangle]
unsafe extern "C" fn modepin_gpio_dir_in(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    static int modepin_gpio_dir_in(struct gpio_chip *chip, unsigned int pin)
    {
    return 0;
    }
//
// modepin_gpio_dir_out - Set the direction of the specified GPIO pin as output
// @chip:	gpio_chip instance to be worked on
// @pin:	gpio pin number within the device
// @state:	value to be written to specified pin
//
// Return: 0 always
//
    static int modepin_gpio_dir_out(struct gpio_chip *chip, unsigned int pin,
    int state)
    {
    return modepin_gpio_set_value(chip, pin, state);
    }
//
// modepin_gpio_probe - Initialization method for modepin_gpio
// @pdev:		platform device instance
//
// Return: 0 on success, negative error otherwise.
//
#[no_mangle]
unsafe extern "C" fn modepin_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int modepin_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_chip *chip;
    int status;
    chip = devm_kzalloc(&pdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    platform_set_drvdata(pdev, chip);
// configure the gpio chip
    chip.base = -1;
    chip.ngpio = MODE_PINS;
    chip.owner = THIS_MODULE;
    chip.parent = &pdev.dev;
    chip.get = modepin_gpio_get_value;
    chip.set = modepin_gpio_set_value;
    chip.direction_input = modepin_gpio_dir_in;
    chip.direction_output = modepin_gpio_dir_out;
    chip.label = dev_name(&pdev.dev);
// modepin gpio registration
    status = devm_gpiochip_add_data(&pdev.dev, chip, chip);
    if (status)
    return dev_err_probe(&pdev.dev, status,
    "Failed to add GPIO chip\n");
    return status;
    }
    static const struct of_device_id modepin_platform_id[] = {
    { .compatible = "xlnx,zynqmp-gpio-modepin", },
    { }
    };
    MODULE_DEVICE_TABLE(of, modepin_platform_id);
    static struct platform_driver modepin_platform_driver = {
    .driver = {
    .name = "modepin-gpio",
    .of_match_table = modepin_platform_id,
    },
    .probe = modepin_gpio_probe,
    };
    module_platform_driver(modepin_platform_driver);
    MODULE_AUTHOR("Piyush Mehta <piyush.mehta@xilinx.com>");
    MODULE_DESCRIPTION("ZynqMP Boot PS_MODE Configuration");
    MODULE_LICENSE("GPL v2");
