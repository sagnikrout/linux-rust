//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/mp5023.c
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
// Driver for MPS MP5023 Hot-Swap Controller
//

    static struct pmbus_driver_info mp5023_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = direct,
    .format[PSC_VOLTAGE_OUT] = direct,
    .format[PSC_CURRENT_OUT] = direct,
    .format[PSC_POWER] = direct,
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_VOLTAGE_IN] = 32,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = 0,
    .m[PSC_VOLTAGE_OUT] = 32,
    .b[PSC_VOLTAGE_OUT] = 0,
    .R[PSC_VOLTAGE_OUT] = 0,
    .m[PSC_CURRENT_OUT] = 16,
    .b[PSC_CURRENT_OUT] = 0,
    .R[PSC_CURRENT_OUT] = 0,
    .m[PSC_POWER] = 1,
    .b[PSC_POWER] = 0,
    .R[PSC_POWER] = 0,
    .m[PSC_TEMPERATURE] = 2,
    .b[PSC_TEMPERATURE] = 0,
    .R[PSC_TEMPERATURE] = 0,
    .func[0] =
    PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_PIN |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_IOUT |
    PMBUS_HAVE_STATUS_INPUT | PMBUS_HAVE_STATUS_TEMP,
    };
#[no_mangle]
unsafe extern "C" fn mp5023_probe(client: *mut i2c_client) -> c_int {
    static int mp5023_probe(struct i2c_client *client)
    {
    return pmbus_do_probe(client, &mp5023_info);
    }
    static const struct of_device_id __maybe_unused mp5023_of_match[] = {
    { .compatible = "mps,mp5023", },
    {}
    };
    MODULE_DEVICE_TABLE(of, mp5023_of_match);
    static struct i2c_driver mp5023_driver = {
    .driver = {
    .name = "mp5023",
    .of_match_table = of_match_ptr(mp5023_of_match),
    },
    .probe = mp5023_probe,
    };
    module_i2c_driver(mp5023_driver);
    MODULE_AUTHOR("Howard Chiu <howard.chiu@quantatw.com>");
    MODULE_DESCRIPTION("PMBus driver for MPS MP5023 HSC");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
