//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-ts4800.c
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
// GPIO driver for the TS-4800 board
//
// Copyright (c) 2016 - Savoir-faire Linux
//

pub const INPUT_REG_OFFSET: c_uint = 0x00;
pub const OUTPUT_REG_OFFSET: c_uint = 0x02;
pub const DIRECTION_REG_OFFSET: c_uint = 0x04;
#[no_mangle]
unsafe extern "C" fn ts4800_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int ts4800_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct gpio_generic_chip *chip;
    void __iomem *base_addr;
    int retval;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    base_addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base_addr))
    return PTR_ERR(base_addr);
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 2,
    .dat = base_addr + INPUT_REG_OFFSET,
    .set = base_addr + OUTPUT_REG_OFFSET,
    .dirout = base_addr + DIRECTION_REG_OFFSET,
    };
    retval = gpio_generic_chip_init(chip, &config);
    if (retval)
    return dev_err_probe(dev, retval,
    "failed to initialize the generic GPIO chip\n");
    return devm_gpiochip_add_data(dev, &chip.gc, core::ptr::null_mut());
    }
    static const struct of_device_id ts4800_gpio_of_match[] = {
    { .compatible = "technologic,ts4800-gpio", },
    {},
    };
    MODULE_DEVICE_TABLE(of, ts4800_gpio_of_match);
    static struct platform_driver ts4800_gpio_driver = {
    .driver = {
    .name = "ts4800-gpio",
    .of_match_table = ts4800_gpio_of_match,
    },
    .probe = ts4800_gpio_probe,
    };
    module_platform_driver_probe(ts4800_gpio_driver, ts4800_gpio_probe);
    MODULE_AUTHOR("Julien Grossholtz <julien.grossholtz@savoirfairelinux.com>");
    MODULE_DESCRIPTION("TS4800 FPGA GPIO driver");
    MODULE_LICENSE("GPL v2");
