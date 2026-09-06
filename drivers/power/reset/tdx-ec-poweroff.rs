//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/tdx-ec-poweroff.c
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
// Toradex Embedded Controller driver
//
// Copyright (C) 2025 Toradex
//
// Author: Emanuele Ghidoli <emanuele.ghidoli@toradex.com>
//

pub const EC_CHIP_ID_REG: c_uint = 0x00;
pub const EC_CHIP_ID_SMARC_IMX95: c_uint = 0x11;
pub const EC_CHIP_ID_SMARC_IMX8MP: c_uint = 0x12;
pub const EC_VERSION_REG_MAJOR: c_uint = 0x01;
pub const EC_VERSION_REG_MINOR: c_uint = 0x02;
pub const EC_ID_VERSION_LEN: c_int = 3;
pub const EC_CMD_REG: c_uint = 0xD0;
pub const EC_CMD_POWEROFF: c_uint = 0x01;
pub const EC_CMD_RESET: c_uint = 0x02;
pub const EC_REG_MAX: c_uint = 0xD0;
pub const EC_CMD_TIMEOUT_MS: c_int = 1000;
    static const struct regmap_range volatile_ranges[] = {
    regmap_reg_range(EC_CMD_REG, EC_CMD_REG),
    };
    static const struct regmap_access_table volatile_table = {
    .yes_ranges	= volatile_ranges,
    .n_yes_ranges	= ARRAY_SIZE(volatile_ranges),
    };
    static const struct regmap_range read_ranges[] = {
    regmap_reg_range(EC_CHIP_ID_REG, EC_VERSION_REG_MINOR),
    };
    static const struct regmap_access_table read_table = {
    .yes_ranges	= read_ranges,
    .n_yes_ranges	= ARRAY_SIZE(read_ranges),
    };
    static const struct regmap_config regmap_config = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .max_register	= EC_REG_MAX,
    .cache_type	= REGCACHE_RBTREE,
    .rd_table	= &read_table,
    .volatile_table = &volatile_table,
    };
#[no_mangle]
unsafe extern "C" fn tdx_ec_cmd(regmap: *mut regmap, cmd: u8) -> c_int {
    static int tdx_ec_cmd(struct regmap *regmap, u8 cmd)
    {
    let mut err: c_int = regmap_write(regmap, EC_CMD_REG, cmd);
    if (err)
    dev_err(regmap_get_device(regmap), "Failed to send command 0x%02X: %d\n", cmd, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tdx_ec_power_off(data: *mut sys_off_data) -> c_int {
    static int tdx_ec_power_off(struct sys_off_data *data)
    {
    struct regmap *regmap = data.cb_data;
    int err;
    err = tdx_ec_cmd(regmap, EC_CMD_POWEROFF);
    if (err) {
    dev_err(data.dev, "Failed to send power off command\n");
    } else {
    mdelay(EC_CMD_TIMEOUT_MS);
    WARN_ONCE(1, "Unable to power off system\n");
    }
    return err ? NOTIFY_BAD : NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn tdx_ec_restart(data: *mut sys_off_data) -> c_int {
    static int tdx_ec_restart(struct sys_off_data *data)
    {
    struct regmap *regmap = data.cb_data;
    int err;
    err = tdx_ec_cmd(regmap, EC_CMD_RESET);
    if (err) {
    dev_err(data.dev, "Failed to send restart command\n");
    } else {
    mdelay(EC_CMD_TIMEOUT_MS);
    WARN_ONCE(1, "Unable to restart system\n");
    }
    return err ? NOTIFY_BAD : NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn tdx_ec_register_power_off_restart(dev: *mut device, regmap: *mut regmap) -> c_int {
    static int tdx_ec_register_power_off_restart(struct device *dev, struct regmap *regmap)
    {
    int err;
    err = devm_register_sys_off_handler(dev, SYS_OFF_MODE_RESTART,
    SYS_OFF_PRIO_FIRMWARE,
    tdx_ec_restart, regmap);
    if (err)
    return err;
    return devm_register_sys_off_handler(dev, SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_FIRMWARE,
    tdx_ec_power_off, regmap);
    }
#[no_mangle]
unsafe extern "C" fn tdx_ec_probe(client: *mut i2c_client) -> c_int {
    static int tdx_ec_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    u8 reg_val[EC_ID_VERSION_LEN];
    struct regmap *regmap;
    int err;
    regmap = devm_regmap_init_i2c(client, &regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    err = regmap_bulk_read(regmap, EC_CHIP_ID_REG, &reg_val, EC_ID_VERSION_LEN);
    if (err)
    return dev_err_probe(dev, err,
    "Cannot read id and version registers\n");
    dev_info(dev, "Toradex Embedded Controller id %x - Firmware %u.%u\n",
    reg_val[0], reg_val[1], reg_val[2]);
    err = tdx_ec_register_power_off_restart(dev, regmap);
    if (err)
    return dev_err_probe(dev, err,
    "Cannot register system restart handler\n");
    return 0;
    }
    static const struct of_device_id __maybe_unused of_tdx_ec_match[] = {
    { .compatible = "toradex,smarc-ec" },
    {}
    };
    MODULE_DEVICE_TABLE(of, of_tdx_ec_match);
    static struct i2c_driver tdx_ec_driver = {
    .probe			= tdx_ec_probe,
    .driver			= {
    .name		= "toradex-smarc-ec",
    .of_match_table = of_tdx_ec_match,
    },
    };
    module_i2c_driver(tdx_ec_driver);
    MODULE_AUTHOR("Emanuele Ghidoli <emanuele.ghidoli@toradex.com>");
    MODULE_DESCRIPTION("Toradex SMARC Embedded Controller driver");
    MODULE_LICENSE("GPL");
