//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-pasemi-platform.c
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
// Copyright (C) 2021 The Asahi Linux Contributors
//
// PA Semi PWRficient SMBus host driver for Apple SoCs
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasemi_platform_i2c_data {
    pub smbus: pasemi_smbus,
    pub clk_ref: *mut clk,
}

    static int
    pasemi_platform_i2c_calc_clk_div(struct pasemi_platform_i2c_data *data,
    u32 frequency)
    {
    let mut clk_rate: c_ulong = clk_get_rate(data.clk_ref);
    if (!clk_rate)
    return -EINVAL;
    data.smbus.clk_div = DIV_ROUND_UP(clk_rate, 16 * frequency);
    if (data.smbus.clk_div < 4)
    return dev_err_probe(data.smbus.dev, -EINVAL,
    "Bus frequency %d is too fast.\n",
    frequency);
    if (data.smbus.clk_div > 0xff)
    return dev_err_probe(data.smbus.dev, -EINVAL,
    "Bus frequency %d is too slow.\n",
    frequency);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pasemi_platform_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int pasemi_platform_i2c_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pasemi_platform_i2c_data *data;
    struct pasemi_smbus *smbus;
    u32 frequency;
    int error;
    int irq_num;
    data = devm_kzalloc(dev, sizeof(struct pasemi_platform_i2c_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    smbus = &data.smbus;
    smbus.dev = dev;
    smbus.ioaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(smbus.ioaddr))
    return PTR_ERR(smbus.ioaddr);
    if (of_property_read_u32(dev.of_node, "clock-frequency", &frequency))
    frequency = I2C_MAX_STANDARD_MODE_FREQ;
    data.clk_ref = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(data.clk_ref))
    return PTR_ERR(data.clk_ref);
    error = pasemi_platform_i2c_calc_clk_div(data, frequency);
    if (error)
    return error;
    smbus.adapter.dev.of_node = pdev.dev.of_node;
    error = pasemi_i2c_common_probe(smbus);
    if (error)
    return error;
    irq_num = platform_get_irq(pdev, 0);
    error = devm_request_irq(smbus.dev, irq_num, pasemi_irq_handler, 0, "pasemi_apple_i2c", (void *)smbus);
    if (!error)
    smbus.use_irq = 1;
    platform_set_drvdata(pdev, data);
    return 0;
    }
    static void pasemi_platform_i2c_remove(struct platform_device *pdev) { }
    static const struct of_device_id pasemi_platform_i2c_of_match[] = {
    { .compatible = "apple,t8103-i2c" },
    { .compatible = "apple,i2c" },
    {},
    };
    MODULE_DEVICE_TABLE(of, pasemi_platform_i2c_of_match);
    static struct platform_driver pasemi_platform_i2c_driver = {
    .driver	= {
    .name			= "i2c-apple",
    .of_match_table		= pasemi_platform_i2c_of_match,
    },
    .probe	= pasemi_platform_i2c_probe,
    .remove = pasemi_platform_i2c_remove,
    };
    module_platform_driver(pasemi_platform_i2c_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Sven Peter <sven@svenpeter.dev>");
    MODULE_DESCRIPTION("Apple/PASemi SMBus platform driver");
