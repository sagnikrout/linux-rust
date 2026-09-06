//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/max20860a.c
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
// Hardware monitoring driver for Analog Devices MAX20860A
//
// SPDX-FileCopyrightText: Copyright Hewlett Packard Enterprise Development LP
//

    static const struct regulator_desc max20860a_reg_desc[] = {
    PMBUS_REGULATOR_ONE("vout"),
    };

    static struct pmbus_driver_info max20860a_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT |
    PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_TEMP2 |
    PMBUS_HAVE_STATUS_TEMP | PMBUS_HAVE_STATUS_INPUT,

    .num_regulators = 1,
    .reg_desc = max20860a_reg_desc,

    };
#[no_mangle]
unsafe extern "C" fn max20860a_probe(client: *mut i2c_client) -> c_int {
    static int max20860a_probe(struct i2c_client *client)
    {
    return pmbus_do_probe(client, &max20860a_info);
    }
    static const struct i2c_device_id max20860a_id[] = {
    {"max20860a"},
    {}
    };
    MODULE_DEVICE_TABLE(i2c, max20860a_id);
    static const struct of_device_id max20860a_of_match[] = {
    { .compatible = "adi,max20860a" },
    {}
    };
    MODULE_DEVICE_TABLE(of, max20860a_of_match);
    static struct i2c_driver max20860a_driver = {
    .driver = {
    .name = "max20860a",
    .of_match_table = max20860a_of_match,
    },
    .probe = max20860a_probe,
    .id_table = max20860a_id,
    };
    module_i2c_driver(max20860a_driver);
    MODULE_AUTHOR("Syed Arif <arif.syed@hpe.com>");
    MODULE_AUTHOR("Sanman Pradhan <psanman@juniper.net>");
    MODULE_DESCRIPTION("PMBus driver for Analog Devices MAX20860A");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
