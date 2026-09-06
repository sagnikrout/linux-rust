//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/d1u74t.c
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
// Copyright 2026 Nexthop Systems.
//

    static const struct i2c_device_id d1u74t_id[] = {
    { "d1u74t" },
    {},
    };
    MODULE_DEVICE_TABLE(i2c, d1u74t_id);
    static struct pmbus_driver_info d1u74t_info = {
    .pages = 1,
// PSU uses default linear data format.
    .func[0] = PMBUS_HAVE_PIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_TEMP |
    PMBUS_HAVE_TEMP2 | PMBUS_HAVE_TEMP3 |
    PMBUS_HAVE_STATUS_TEMP | PMBUS_HAVE_FAN12 |
    PMBUS_HAVE_STATUS_FAN12,
    };
#[no_mangle]
unsafe extern "C" fn d1u74t_probe(client: *mut i2c_client) -> c_int {
    static int d1u74t_probe(struct i2c_client *client)
    {
    char buf[I2C_SMBUS_BLOCK_MAX + 2] = { 0 };
    struct device *dev = &client.dev;
    int rc;
    rc = i2c_smbus_read_block_data(client, PMBUS_MFR_ID, buf);
    if (rc < 0)
    return dev_err_probe(dev, rc, "Failed to read PMBUS_MFR_ID\n");
    if (rc != 9 || strncmp(buf, "Murata-PS", 9)) {
    buf[rc] = '\0';
    return dev_err_probe(dev, -ENODEV,
    "Unsupported Manufacturer ID '%s'\n",
    buf);
    }
    rc = i2c_smbus_read_block_data(client, PMBUS_MFR_MODEL, buf);
    if (rc < 0)
    return dev_err_probe(dev, rc,
    "Failed to read PMBUS_MFR_MODEL\n");
    if (rc < 8 || strncmp(buf, "D1U74T-W", 8)) {
    buf[rc] = '\0';
    return dev_err_probe(dev, -ENODEV, "Model '%s' not supported\n",
    buf);
    }
    rc = pmbus_do_probe(client, &d1u74t_info);
    if (rc)
    return dev_err_probe(dev, rc, "Failed to probe\n");
    return 0;
    }
    static const struct of_device_id d1u74t_of_match[] = {
    {
    .compatible = "murata,d1u74t",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, d1u74t_of_match);
    static struct i2c_driver d1u74t_driver = {
    .driver = {
    .name = "d1u74t",
    .of_match_table = d1u74t_of_match,
    },
    .probe = d1u74t_probe,
    .id_table = d1u74t_id,
    };
    module_i2c_driver(d1u74t_driver);
    MODULE_AUTHOR("Abdurrahman Hussain");
    MODULE_DESCRIPTION("PMBus driver for Murata D1U74T-W power supplies");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
