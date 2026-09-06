//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/max16064.c
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
// Hardware monitoring driver for Maxim MAX16064
//
// Copyright (c) 2011 Ericsson AB.
//

pub const MAX16064_MFR_VOUT_PEAK: c_uint = 0xd4;
pub const MAX16064_MFR_TEMPERATURE_PEAK: c_uint = 0xd6;
    static int max16064_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VOUT_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    MAX16064_MFR_VOUT_PEAK);
    break;
    case PMBUS_VIRT_READ_TEMP_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    MAX16064_MFR_TEMPERATURE_PEAK);
    break;
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    ret = 0;
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int max16064_write_word_data(struct i2c_client *client, int page,
    int reg, u16 word)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
    ret = pmbus_write_word_data(client, page,
    MAX16064_MFR_VOUT_PEAK, 0);
    break;
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    ret = pmbus_write_word_data(client, page,
    MAX16064_MFR_TEMPERATURE_PEAK,
    0xffff);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static struct pmbus_driver_info max16064_info = {
    .pages = 4,
    .format[PSC_VOLTAGE_IN] = direct,
    .format[PSC_VOLTAGE_OUT] = direct,
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_VOLTAGE_IN] = 19995,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = -1,
    .m[PSC_VOLTAGE_OUT] = 19995,
    .b[PSC_VOLTAGE_OUT] = 0,
    .R[PSC_VOLTAGE_OUT] = -1,
    .m[PSC_TEMPERATURE] = -7612,
    .b[PSC_TEMPERATURE] = 335,
    .R[PSC_TEMPERATURE] = -3,
    .func[0] = PMBUS_HAVE_VOUT | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_TEMP,
    .func[1] = PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT,
    .func[2] = PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT,
    .func[3] = PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT,
    .read_word_data = max16064_read_word_data,
    .write_word_data = max16064_write_word_data,
    };
#[no_mangle]
unsafe extern "C" fn max16064_probe(client: *mut i2c_client) -> c_int {
    static int max16064_probe(struct i2c_client *client)
    {
    return pmbus_do_probe(client, &max16064_info);
    }
    static const struct i2c_device_id max16064_id[] = {
    { .name = "max16064" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max16064_id);
// This is the driver that will be inserted
    static struct i2c_driver max16064_driver = {
    .driver = {
    .name = "max16064",
    },
    .probe = max16064_probe,
    .id_table = max16064_id,
    };
    module_i2c_driver(max16064_driver);
    MODULE_AUTHOR("Guenter Roeck");
    MODULE_DESCRIPTION("PMBus driver for Maxim MAX16064");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
