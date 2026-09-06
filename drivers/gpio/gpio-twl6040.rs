//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-twl6040.c
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
// Access to GPOs on TWL6040 chip
//
// Copyright (C) 2012 Texas Instruments, Inc.
//
// Authors:
// Sergio Aguirre <saaguirre@ti.com>
// Peter Ujfalusi <peter.ujfalusi@ti.com>
//

#[no_mangle]
unsafe extern "C" fn twl6040gpo_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int twl6040gpo_get(struct gpio_chip *chip, unsigned offset)
    {
    struct twl6040 *twl6040 = gpiochip_get_data(chip);
    let mut ret: c_int = 0;
    ret = twl6040_reg_read(twl6040, TWL6040_REG_GPOCTL);
    if (ret < 0)
    return ret;
    return !!(ret & BIT(offset));
    }
#[no_mangle]
unsafe extern "C" fn twl6040gpo_get_direction(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int twl6040gpo_get_direction(struct gpio_chip *chip, unsigned offset)
    {
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int twl6040gpo_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct twl6040 *twl6040 = gpiochip_get_data(chip);
    int ret;
    u8 gpoctl;
    ret = twl6040_reg_read(twl6040, TWL6040_REG_GPOCTL);
    if (ret < 0)
    return ret;
    if (value)
    gpoctl = ret | BIT(offset);
    else
    gpoctl = ret & ~BIT(offset);
    return twl6040_reg_write(twl6040, TWL6040_REG_GPOCTL, gpoctl);
    }
    static int twl6040gpo_direction_out(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
// This only drives GPOs, and can't change direction
    return twl6040gpo_set(chip, offset, value);
    }
    static struct gpio_chip twl6040gpo_chip = {
    .label			= "twl6040",
    .owner			= THIS_MODULE,
    .get			= twl6040gpo_get,
    .direction_output	= twl6040gpo_direction_out,
    .get_direction		= twl6040gpo_get_direction,
    .set			= twl6040gpo_set,
    .can_sleep		= true,
    };
// ----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn gpo_twl6040_probe(pdev: *mut platform_device) -> c_int {
    static int gpo_twl6040_probe(struct platform_device *pdev)
    {
    struct device *twl6040_core_dev = pdev.dev.parent;
    struct twl6040 *twl6040 = dev_get_drvdata(twl6040_core_dev);
    int ret;
    device_set_node(&pdev.dev, dev_fwnode(pdev.dev.parent));
    twl6040gpo_chip.base = -1;
    if (twl6040_get_revid(twl6040) < TWL6041_REV_ES2_0)
    twl6040gpo_chip.ngpio = 3; /* twl6040 have 3 GPO */
    else
    twl6040gpo_chip.ngpio = 1; /* twl6041 have 1 GPO */
    twl6040gpo_chip.parent = &pdev.dev;
    ret = devm_gpiochip_add_data(&pdev.dev, &twl6040gpo_chip, twl6040);
    if (ret < 0) {
    dev_err(&pdev.dev, "could not register gpiochip, %d\n", ret);
    twl6040gpo_chip.ngpio = 0;
    }
    return ret;
    }
// Note:  this hardware lives inside an I2C-based multi-function device.
    MODULE_ALIAS("platform:twl6040-gpo");
    static struct platform_driver gpo_twl6040_driver = {
    .driver = {
    .name	= "twl6040-gpo",
    },
    .probe		= gpo_twl6040_probe,
    };
    module_platform_driver(gpo_twl6040_driver);
    MODULE_AUTHOR("Texas Instruments, Inc.");
    MODULE_DESCRIPTION("GPO interface for TWL6040");
    MODULE_LICENSE("GPL");
