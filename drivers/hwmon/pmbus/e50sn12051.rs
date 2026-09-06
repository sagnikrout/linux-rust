//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/e50sn12051.c
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
// Hardware monitoring driver for E50SN12051
//

    static struct pmbus_driver_info e50sn12051_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP,
    };
    static const struct i2c_device_id e50sn12051_id[] = { { "e50sn12051", 0 }, {} };
    MODULE_DEVICE_TABLE(i2c, e50sn12051_id);
    static const struct of_device_id e50sn12051_of_match[] = {
    { .compatible = "delta,e50sn12051" },
    {},
    };
    MODULE_DEVICE_TABLE(of, e50sn12051_of_match);
#[no_mangle]
unsafe extern "C" fn e50sn12051_probe(client: *mut i2c_client) -> c_int {
    static int e50sn12051_probe(struct i2c_client *client)
    {
    return pmbus_do_probe(client, &e50sn12051_info);
    }
    static struct i2c_driver e50sn12051_driver = {
    .driver = {
    .name = "e50sn12051",
    .of_match_table = e50sn12051_of_match,
    },
    .probe = e50sn12051_probe,
    .id_table = e50sn12051_id,
    };
    module_i2c_driver(e50sn12051_driver);
    MODULE_AUTHOR("Kevin Chang <kevin.chang2@amd.com>");
    MODULE_DESCRIPTION("PMBus driver for E50SN12051");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
