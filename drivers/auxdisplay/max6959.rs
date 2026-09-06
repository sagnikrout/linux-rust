//! Automatically rewritten from C to Rust
//! Source: drivers/auxdisplay/max6959.c
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
// MAX6958/6959 7-segment LED display controller
// Datasheet:
// https://www.analog.com/media/en/technical-documentation/data-sheets/MAX6958-MAX6959.pdf
//
// Copyright (c) 2024, Intel Corporation.
// Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

// Registers
pub const REG_DECODE_MODE: c_uint = 0x01;
pub const REG_INTENSITY: c_uint = 0x02;
pub const REG_SCAN_LIMIT: c_uint = 0x03;
pub const REG_CONFIGURATION: c_uint = 0x04;

pub const REG_DIGIT0: c_uint = 0x20;
pub const REG_DIGIT1: c_uint = 0x21;
pub const REG_DIGIT2: c_uint = 0x22;
pub const REG_DIGIT3: c_uint = 0x23;
pub const REG_SEGMENTS: c_uint = 0x24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max6959_priv {
    pub linedisp: linedisp,
    pub work: delayed_work,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn max6959_disp_update(work: *mut work_struct) {
    static void max6959_disp_update(struct work_struct *work)
    {
    struct max6959_priv *priv = container_of(work, struct max6959_priv, work.work);
    struct linedisp *linedisp = &priv.linedisp;
    struct linedisp_map *map = linedisp.map;
    char *s = linedisp.buf;
    u8 buf[4];
// Map segments according to datasheet
    buf[0] = bitrev8(map_to_seg7(&map.map.seg7, *s++)) >> 1;
    buf[1] = bitrev8(map_to_seg7(&map.map.seg7, *s++)) >> 1;
    buf[2] = bitrev8(map_to_seg7(&map.map.seg7, *s++)) >> 1;
    buf[3] = bitrev8(map_to_seg7(&map.map.seg7, *s++)) >> 1;
    regmap_bulk_write(priv.regmap, REG_DIGIT(0), buf, ARRAY_SIZE(buf));
    }
#[no_mangle]
unsafe extern "C" fn max6959_linedisp_get_map_type(linedisp: *mut linedisp) -> c_int {
    static int max6959_linedisp_get_map_type(struct linedisp *linedisp)
    {
    struct max6959_priv *priv = container_of(linedisp, struct max6959_priv, linedisp);
    INIT_DELAYED_WORK(&priv.work, max6959_disp_update);
    return LINEDISP_MAP_SEG7;
    }
#[no_mangle]
unsafe extern "C" fn max6959_linedisp_update(linedisp: *mut linedisp) {
    static void max6959_linedisp_update(struct linedisp *linedisp)
    {
    struct max6959_priv *priv = container_of(linedisp, struct max6959_priv, linedisp);
    schedule_delayed_work(&priv.work, 0);
    }
    static const struct linedisp_ops max6959_linedisp_ops = {
    .get_map_type = max6959_linedisp_get_map_type,
    .update = max6959_linedisp_update,
    };
#[no_mangle]
unsafe extern "C" fn max6959_enable(priv: *mut max6959_priv, enable: bool) -> c_int {
    static int max6959_enable(struct max6959_priv *priv, bool enable)
    {
    return regmap_assign_bits(priv.regmap, REG_CONFIGURATION, REG_CONFIGURATION_S_BIT, enable);
    }
#[no_mangle]
unsafe extern "C" fn max6959_power_off(priv: *mut c_void) {
    static void max6959_power_off(void *priv)
    {
    max6959_enable(priv, false);
    }
#[no_mangle]
unsafe extern "C" fn max6959_power_on(priv: *mut max6959_priv) -> c_int {
    static int max6959_power_on(struct max6959_priv *priv)
    {
    struct device *dev = regmap_get_device(priv.regmap);
    int ret;
    ret = max6959_enable(priv, true);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, max6959_power_off, priv);
    }
    static const struct regmap_config max6959_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = REG_MAX,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn max6959_i2c_probe(client: *mut i2c_client) -> c_int {
    static int max6959_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct max6959_priv *priv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = devm_regmap_init_i2c(client, &max6959_regmap_config);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    ret = max6959_power_on(priv);
    if (ret)
    return ret;
    ret = linedisp_register(&priv.linedisp, dev, 4, &max6959_linedisp_ops);
    if (ret)
    return ret;
    i2c_set_clientdata(client, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max6959_i2c_remove(client: *mut i2c_client) {
    static void max6959_i2c_remove(struct i2c_client *client)
    {
    struct max6959_priv *priv = i2c_get_clientdata(client);
    cancel_delayed_work_sync(&priv.work);
    linedisp_unregister(&priv.linedisp);
    }
#[no_mangle]
unsafe extern "C" fn max6959_suspend(dev: *mut device) -> c_int {
    static int max6959_suspend(struct device *dev)
    {
    return max6959_enable(dev_get_drvdata(dev), false);
    }
#[no_mangle]
unsafe extern "C" fn max6959_resume(dev: *mut device) -> c_int {
    static int max6959_resume(struct device *dev)
    {
    return max6959_enable(dev_get_drvdata(dev), true);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(max6959_pm_ops, max6959_suspend, max6959_resume);
    static const struct i2c_device_id max6959_i2c_id[] = {
    { "max6959" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max6959_i2c_id);
    static const struct of_device_id max6959_of_table[] = {
    { .compatible = "maxim,max6959" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max6959_of_table);
    static struct i2c_driver max6959_i2c_driver = {
    .driver = {
    .name = "max6959",
    .pm = pm_sleep_ptr(&max6959_pm_ops),
    .of_match_table = max6959_of_table,
    },
    .probe = max6959_i2c_probe,
    .remove = max6959_i2c_remove,
    .id_table = max6959_i2c_id,
    };
    module_i2c_driver(max6959_i2c_driver);
    MODULE_DESCRIPTION("MAX6958/6959 7-segment LED controller");
    MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("LINEDISP");
