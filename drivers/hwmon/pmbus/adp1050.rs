//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/adp1050.c
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
// Hardware monitoring driver for Analog Devices ADP1050
//
// Copyright (C) 2024 Analog Devices, Inc.
//

    static const struct regulator_desc adp1050_reg_desc[] = {
    PMBUS_REGULATOR_ONE("vout"),
    };

    static struct pmbus_driver_info adp1050_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_IIN | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_STATUS_TEMP,
    };
    static struct pmbus_driver_info adp1051_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_IOUT
    | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_STATUS_TEMP,
    };
    static struct pmbus_driver_info adp1055_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_IOUT
    | PMBUS_HAVE_TEMP2 | PMBUS_HAVE_TEMP3
    | PMBUS_HAVE_POUT
    | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_STATUS_TEMP,
    };
    static struct pmbus_driver_info ltp8800_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_IOUT
    | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_POUT
    | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_STATUS_TEMP,

    .num_regulators = 1,
    .reg_desc = adp1050_reg_desc,

    };
#[no_mangle]
unsafe extern "C" fn adp1050_probe(client: *mut i2c_client) -> c_int {
    static int adp1050_probe(struct i2c_client *client)
    {
    struct pmbus_driver_info *info;
    info = (struct pmbus_driver_info *)i2c_get_match_data(client);
    if (!info)
    return -ENODEV;
    return pmbus_do_probe(client, info);
    }
    static const struct i2c_device_id adp1050_id[] = {
    { .name = "adp1050", .driver_data = (kernel_ulong_t)&adp1050_info },
    { .name = "adp1051", .driver_data = (kernel_ulong_t)&adp1051_info },
    { .name = "adp1055", .driver_data = (kernel_ulong_t)&adp1055_info },
    { .name = "ltp8800", .driver_data = (kernel_ulong_t)&ltp8800_info },
    {}
    };
    MODULE_DEVICE_TABLE(i2c, adp1050_id);
    static const struct of_device_id adp1050_of_match[] = {
    { .compatible = "adi,adp1050", .data = &adp1050_info },
    { .compatible = "adi,adp1051", .data = &adp1051_info },
    { .compatible = "adi,adp1055", .data = &adp1055_info },
    { .compatible = "adi,ltp8800", .data = &ltp8800_info },
    {}
    };
    MODULE_DEVICE_TABLE(of, adp1050_of_match);
    static struct i2c_driver adp1050_driver = {
    .driver = {
    .name = "adp1050",
    .of_match_table = adp1050_of_match,
    },
    .probe = adp1050_probe,
    .id_table = adp1050_id,
    };
    module_i2c_driver(adp1050_driver);
    MODULE_AUTHOR("Radu Sabau <radu.sabau@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADP1050 HWMON PMBus Driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
