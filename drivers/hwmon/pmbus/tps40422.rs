//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/tps40422.c
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
// Hardware monitoring driver for TI TPS40422
//
// Copyright (c) 2014 Nokia Solutions and Networks.
//

    static struct pmbus_driver_info tps40422_info = {
    .pages = 2,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VOUT | PMBUS_HAVE_TEMP2
    | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_TEMP
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT,
    .func[1] = PMBUS_HAVE_VOUT | PMBUS_HAVE_TEMP2
    | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_TEMP
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT,
    };
#[no_mangle]
unsafe extern "C" fn tps40422_probe(client: *mut i2c_client) -> c_int {
    static int tps40422_probe(struct i2c_client *client)
    {
    return pmbus_do_probe(client, &tps40422_info);
    }
    static const struct i2c_device_id tps40422_id[] = {
    { .name = "tps40422" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tps40422_id);
// This is the driver that will be inserted
    static struct i2c_driver tps40422_driver = {
    .driver = {
    .name = "tps40422",
    },
    .probe = tps40422_probe,
    .id_table = tps40422_id,
    };
    module_i2c_driver(tps40422_driver);
    MODULE_AUTHOR("Zhu Laiwen <richard.zhu@nsn.com>");
    MODULE_DESCRIPTION("PMBus driver for TI TPS40422");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
