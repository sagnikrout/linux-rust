//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/menf21bmc.c
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
// MEN 14F021P00 Board Management Controller (BMC) MFD Core Driver.
//
// Copyright (C) 2014 MEN Mikro Elektronik Nuernberg GmbH
//

pub const BMC_CMD_WDT_EXIT_PROD: c_uint = 0x18;
pub const BMC_CMD_WDT_PROD_STAT: c_uint = 0x19;
pub const BMC_CMD_REV_MAJOR: c_uint = 0x80;
pub const BMC_CMD_REV_MINOR: c_uint = 0x81;
pub const BMC_CMD_REV_MAIN: c_uint = 0x82;
    static struct mfd_cell menf21bmc_cell[] = {
    { .name = "menf21bmc_wdt", },
    { .name = "menf21bmc_led", },
    { .name = "menf21bmc_hwmon", }
    };
#[no_mangle]
unsafe extern "C" fn menf21bmc_wdt_exit_prod_mode(client: *mut i2c_client) -> c_int {
    static int menf21bmc_wdt_exit_prod_mode(struct i2c_client *client)
    {
    int val, ret;
    val = i2c_smbus_read_byte_data(client, BMC_CMD_WDT_PROD_STAT);
    if (val < 0)
    return val;
//
// Production mode should be not active after delivery of the Board.
// To be sure we check it, inform the user and exit the mode
// if active.
//
    if (val == 0x00) {
    dev_info(&client.dev,
    "BMC in production mode. Exit production mode\n");
    ret = i2c_smbus_write_byte(client, BMC_CMD_WDT_EXIT_PROD);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static int
    menf21bmc_probe(struct i2c_client *client)
    {
    int rev_major, rev_minor, rev_main;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_WORD_DATA |
    I2C_FUNC_SMBUS_BYTE))
    return -ENODEV;
    rev_major = i2c_smbus_read_word_data(client, BMC_CMD_REV_MAJOR);
    if (rev_major < 0) {
    dev_err(&client.dev, "failed to get BMC major revision\n");
    return rev_major;
    }
    rev_minor = i2c_smbus_read_word_data(client, BMC_CMD_REV_MINOR);
    if (rev_minor < 0) {
    dev_err(&client.dev, "failed to get BMC minor revision\n");
    return rev_minor;
    }
    rev_main = i2c_smbus_read_word_data(client, BMC_CMD_REV_MAIN);
    if (rev_main < 0) {
    dev_err(&client.dev, "failed to get BMC main revision\n");
    return rev_main;
    }
    dev_info(&client.dev, "FW Revision: %02d.%02d.%02d\n",
    rev_major, rev_minor, rev_main);
//
// We have to exit the Production Mode of the BMC to activate the
// Watchdog functionality and the BIOS life sign monitoring.
//
    ret = menf21bmc_wdt_exit_prod_mode(client);
    if (ret < 0) {
    dev_err(&client.dev, "failed to leave production mode\n");
    return ret;
    }
    ret = devm_mfd_add_devices(&client.dev, 0, menf21bmc_cell,
    ARRAY_SIZE(menf21bmc_cell), core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret < 0) {
    dev_err(&client.dev, "failed to add BMC sub-devices\n");
    return ret;
    }
    return 0;
    }
    static const struct i2c_device_id menf21bmc_id_table[] = {
    { "menf21bmc" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, menf21bmc_id_table);
    static struct i2c_driver menf21bmc_driver = {
    .driver.name	= "menf21bmc",
    .id_table	= menf21bmc_id_table,
    .probe		= menf21bmc_probe,
    };
    module_i2c_driver(menf21bmc_driver);
    MODULE_DESCRIPTION("MEN 14F021P00 BMC mfd core driver");
    MODULE_AUTHOR("Andreas Werner <andreas.werner@men.de>");
    MODULE_LICENSE("GPL v2");
