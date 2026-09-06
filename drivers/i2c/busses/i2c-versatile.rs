//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-versatile.c
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
// i2c-versatile.c
//
// Copyright (C) 2006 ARM Ltd.
// written by Russell King, Deep Blue Solutions Ltd.
//

pub const I2C_CONTROL: c_uint = 0x00;
pub const I2C_CONTROLS: c_uint = 0x00;
pub const I2C_CONTROLC: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_versatile {
    pub adap: i2c_adapter,
    pub algo: i2c_algo_bit_data,
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn i2c_versatile_setsda(data: *mut c_void, state: c_int) {
    static void i2c_versatile_setsda(void *data, int state)
    {
    struct i2c_versatile *i2c = data;
    writel(SDA, i2c.base + (state ? I2C_CONTROLS : I2C_CONTROLC));
    }
#[no_mangle]
unsafe extern "C" fn i2c_versatile_setscl(data: *mut c_void, state: c_int) {
    static void i2c_versatile_setscl(void *data, int state)
    {
    struct i2c_versatile *i2c = data;
    writel(SCL, i2c.base + (state ? I2C_CONTROLS : I2C_CONTROLC));
    }
#[no_mangle]
unsafe extern "C" fn i2c_versatile_getsda(data: *mut c_void) -> c_int {
    static int i2c_versatile_getsda(void *data)
    {
    struct i2c_versatile *i2c = data;
    return !!(readl(i2c.base + I2C_CONTROL) & SDA);
    }
#[no_mangle]
unsafe extern "C" fn i2c_versatile_getscl(data: *mut c_void) -> c_int {
    static int i2c_versatile_getscl(void *data)
    {
    struct i2c_versatile *i2c = data;
    return !!(readl(i2c.base + I2C_CONTROL) & SCL);
    }
    static const struct i2c_algo_bit_data i2c_versatile_algo = {
    .setsda	= i2c_versatile_setsda,
    .setscl = i2c_versatile_setscl,
    .getsda	= i2c_versatile_getsda,
    .getscl = i2c_versatile_getscl,
    .udelay	= 30,
    .timeout = HZ,
    };
#[no_mangle]
unsafe extern "C" fn i2c_versatile_probe(dev: *mut platform_device) -> c_int {
    static int i2c_versatile_probe(struct platform_device *dev)
    {
    struct i2c_versatile *i2c;
    int ret;
    i2c = devm_kzalloc(&dev.dev, sizeof(struct i2c_versatile), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    i2c.base = devm_platform_get_and_ioremap_resource(dev, 0, core::ptr::null_mut());
    if (IS_ERR(i2c.base))
    return PTR_ERR(i2c.base);
    writel(SCL | SDA, i2c.base + I2C_CONTROLS);
    i2c.adap.owner = THIS_MODULE;
    strscpy(i2c.adap.name, "Versatile I2C adapter", sizeof(i2c.adap.name));
    i2c.adap.algo_data = &i2c.algo;
    i2c.adap.dev.parent = &dev.dev;
    i2c.adap.dev.of_node = dev.dev.of_node;
    i2c.algo = i2c_versatile_algo;
    i2c.algo.data = i2c;
    i2c.adap.nr = dev.id;
    ret = i2c_bit_add_numbered_bus(&i2c.adap);
    if (ret < 0)
    return ret;
    platform_set_drvdata(dev, i2c);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_versatile_remove(dev: *mut platform_device) {
    static void i2c_versatile_remove(struct platform_device *dev)
    {
    struct i2c_versatile *i2c = platform_get_drvdata(dev);
    i2c_del_adapter(&i2c.adap);
    }
    static const struct of_device_id i2c_versatile_match[] = {
    { .compatible = "arm,versatile-i2c", },
    {},
    };
    MODULE_DEVICE_TABLE(of, i2c_versatile_match);
    static struct platform_driver i2c_versatile_driver = {
    .probe		= i2c_versatile_probe,
    .remove		= i2c_versatile_remove,
    .driver		= {
    .name	= "versatile-i2c",
    .of_match_table = i2c_versatile_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn i2c_versatile_init() -> int __init {
    static int __init i2c_versatile_init(void)
    {
    return platform_driver_register(&i2c_versatile_driver);
    }
#[no_mangle]
unsafe extern "C" fn i2c_versatile_exit() -> void __exit {
    static void __exit i2c_versatile_exit(void)
    {
    platform_driver_unregister(&i2c_versatile_driver);
    }
    subsys_initcall(i2c_versatile_init);
    module_exit(i2c_versatile_exit);
    MODULE_DESCRIPTION("ARM Versatile I2C bus driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:versatile-i2c");
