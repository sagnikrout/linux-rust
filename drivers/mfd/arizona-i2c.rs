//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/arizona-i2c.c
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
// Arizona-i2c.c  --  Arizona I2C bus interface
//
// Copyright 2012 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

#[no_mangle]
unsafe extern "C" fn arizona_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int arizona_i2c_probe(struct i2c_client *i2c)
    {
    struct arizona *arizona;
    const struct regmap_config *regmap_config = core::ptr::null_mut();
    unsigned long type;
    int ret;
    type = (uintptr_t)i2c_get_match_data(i2c);
    switch (type) {
    case WM5102:
    if (IS_ENABLED(CONFIG_MFD_WM5102))
    regmap_config = &wm5102_i2c_regmap;
    break;
    case WM5110:
    case WM8280:
    if (IS_ENABLED(CONFIG_MFD_WM5110))
    regmap_config = &wm5110_i2c_regmap;
    break;
    case WM8997:
    if (IS_ENABLED(CONFIG_MFD_WM8997))
    regmap_config = &wm8997_i2c_regmap;
    break;
    case WM8998:
    case WM1814:
    if (IS_ENABLED(CONFIG_MFD_WM8998))
    regmap_config = &wm8998_i2c_regmap;
    break;
    default:
    dev_err(&i2c.dev, "Unknown device type %ld\n", type);
    return -EINVAL;
    }
    if (!regmap_config) {
    dev_err(&i2c.dev,
    "No kernel support for device type %ld\n", type);
    return -EINVAL;
    }
    arizona = devm_kzalloc(&i2c.dev, sizeof(*arizona), GFP_KERNEL);
    if (arizona == core::ptr::null_mut())
    return -ENOMEM;
    arizona.regmap = devm_regmap_init_i2c(i2c, regmap_config);
    if (IS_ERR(arizona.regmap)) {
    ret = PTR_ERR(arizona.regmap);
    dev_err(&i2c.dev, "Failed to allocate register map: %d\n",
    ret);
    return ret;
    }
    arizona.type = type;
    arizona.dev = &i2c.dev;
    arizona.irq = i2c.irq;
    return arizona_dev_init(arizona);
    }
#[no_mangle]
unsafe extern "C" fn arizona_i2c_remove(i2c: *mut i2c_client) {
    static void arizona_i2c_remove(struct i2c_client *i2c)
    {
    struct arizona *arizona = dev_get_drvdata(&i2c.dev);
    arizona_dev_exit(arizona);
    }
    static const struct i2c_device_id arizona_i2c_id[] = {
    { "wm5102", WM5102 },
    { "wm5110", WM5110 },
    { "wm8280", WM8280 },
    { "wm8997", WM8997 },
    { "wm8998", WM8998 },
    { "wm1814", WM1814 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, arizona_i2c_id);

    static const struct of_device_id arizona_i2c_of_match[] = {
    { .compatible = "wlf,wm5102", .data = (void *)WM5102 },
    { .compatible = "wlf,wm5110", .data = (void *)WM5110 },
    { .compatible = "wlf,wm8280", .data = (void *)WM8280 },
    { .compatible = "wlf,wm8997", .data = (void *)WM8997 },
    { .compatible = "wlf,wm8998", .data = (void *)WM8998 },
    { .compatible = "wlf,wm1814", .data = (void *)WM1814 },
    {},
    };
    MODULE_DEVICE_TABLE(of, arizona_i2c_of_match);

    static struct i2c_driver arizona_i2c_driver = {
    .driver = {
    .name	= "arizona",
    .pm	= pm_ptr(&arizona_pm_ops),
    .of_match_table	= of_match_ptr(arizona_i2c_of_match),
    },
    .probe		= arizona_i2c_probe,
    .remove		= arizona_i2c_remove,
    .id_table	= arizona_i2c_id,
    };
    module_i2c_driver(arizona_i2c_driver);
    MODULE_SOFTDEP("pre: arizona_ldo1");
    MODULE_DESCRIPTION("Arizona I2C bus interface");
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_LICENSE("GPL");
