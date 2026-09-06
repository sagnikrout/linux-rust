//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/max17616.c
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
// Hardware monitoring driver for Analog Devices MAX17616/MAX17616A
//
// Copyright (C) 2025 Analog Devices, Inc.
//

    static struct pmbus_driver_info max17616_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = direct,
    .m[PSC_VOLTAGE_IN] = 512,
    .b[PSC_VOLTAGE_IN] = -18,
    .R[PSC_VOLTAGE_IN] = -1,
    .format[PSC_VOLTAGE_OUT] = direct,
    .m[PSC_VOLTAGE_OUT] = 512,
    .b[PSC_VOLTAGE_OUT] = -18,
    .R[PSC_VOLTAGE_OUT] = -1,
    .format[PSC_CURRENT_OUT] = direct,
    .m[PSC_CURRENT_OUT] = 5845,
    .b[PSC_CURRENT_OUT] = 80,
    .R[PSC_CURRENT_OUT] = -1,
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_TEMPERATURE] = 71,
    .b[PSC_TEMPERATURE] = 19653,
    .R[PSC_TEMPERATURE] = -1,
    .func[0] =  PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_STATUS_IOUT | PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_STATUS_TEMP,
    };
#[no_mangle]
unsafe extern "C" fn max17616_probe(client: *mut i2c_client) -> c_int {
    static int max17616_probe(struct i2c_client *client)
    {
    return pmbus_do_probe(client, &max17616_info);
    }
    static const struct i2c_device_id max17616_id[] = {
    { .name = "max17616" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max17616_id);
    static const struct of_device_id max17616_of_match[] = {
    { .compatible = "adi,max17616" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max17616_of_match);
    static struct i2c_driver max17616_driver = {
    .driver = {
    .name = "max17616",
    .of_match_table = max17616_of_match,
    },
    .probe = max17616_probe,
    .id_table = max17616_id,
    };
    module_i2c_driver(max17616_driver);
    MODULE_AUTHOR("Kim Seer Paller <kimseer.paller@analog.com>");
    MODULE_DESCRIPTION("PMBus driver for Analog Devices MAX17616/MAX17616A");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
