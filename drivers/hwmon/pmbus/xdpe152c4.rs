//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/xdpe152c4.c
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
// Hardware monitoring driver for Infineon Multi-phase Digital VR Controllers
//
// Copyright (c) 2022 Infineon Technologies. All rights reserved.
//

pub const XDPE152_PAGE_NUM: c_int = 2;
    static struct pmbus_driver_info xdpe152_info = {
    .pages = XDPE152_PAGE_NUM,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_POWER] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_TEMP2 | PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_POUT | PMBUS_HAVE_PIN | PMBUS_HAVE_STATUS_INPUT,
    .func[1] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_POUT | PMBUS_HAVE_PIN | PMBUS_HAVE_STATUS_INPUT,
    };
#[no_mangle]
unsafe extern "C" fn xdpe152_probe(client: *mut i2c_client) -> c_int {
    static int xdpe152_probe(struct i2c_client *client)
    {
    struct pmbus_driver_info *info;
    info = devm_kmemdup(&client.dev, &xdpe152_info, sizeof(*info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    return pmbus_do_probe(client, info);
    }
    static const struct i2c_device_id xdpe152_id[] = {
    { .name = "xdpe152c4" },
    { .name = "xdpe15284" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, xdpe152_id);
    static const struct of_device_id __maybe_unused xdpe152_of_match[] = {
    {.compatible = "infineon,xdpe152c4"},
    {.compatible = "infineon,xdpe15284"},
    {}
    };
    MODULE_DEVICE_TABLE(of, xdpe152_of_match);
    static struct i2c_driver xdpe152_driver = {
    .driver = {
    .name = "xdpe152c4",
    .of_match_table = of_match_ptr(xdpe152_of_match),
    },
    .probe = xdpe152_probe,
    .id_table = xdpe152_id,
    };
    module_i2c_driver(xdpe152_driver);
    MODULE_AUTHOR("Greg Schwendimann <greg.schwendimann@infineon.com>");
    MODULE_DESCRIPTION("PMBus driver for Infineon XDPE152 family");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
