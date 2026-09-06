//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/tps546d24.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Hardware monitoring driver for TEXAS TPS546D24 buck converter
//

    static struct pmbus_driver_info tps546d24_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_VOUT
    | PMBUS_HAVE_STATUS_IOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP,
    };
#[no_mangle]
unsafe extern "C" fn tps546d24_probe(client: *mut i2c_client) -> c_int {
    static int tps546d24_probe(struct i2c_client *client)
    {
    int reg;
    reg = i2c_smbus_read_byte_data(client, PMBUS_VOUT_MODE);
    if (reg < 0)
    return reg;
    if (reg & 0x80) {
    int err;
    err = i2c_smbus_write_byte_data(client, PMBUS_VOUT_MODE, reg & 0x7f);
    if (err < 0)
    return err;
    }
    return pmbus_do_probe(client, &tps546d24_info);
    }
    static const struct i2c_device_id tps546d24_id[] = {
    { .name = "tps546d24" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tps546d24_id);
    static const struct of_device_id __maybe_unused tps546d24_of_match[] = {
    {.compatible = "ti,tps546d24"},
    {}
    };
    MODULE_DEVICE_TABLE(of, tps546d24_of_match);
// This is the driver that will be inserted
    static struct i2c_driver tps546d24_driver = {
    .driver = {
    .name = "tps546d24",
    .of_match_table = of_match_ptr(tps546d24_of_match),
    },
    .probe = tps546d24_probe,
    .id_table = tps546d24_id,
    };
    module_i2c_driver(tps546d24_driver);
    MODULE_AUTHOR("Duke Du <dukedu83@gmail.com>");
    MODULE_DESCRIPTION("PMBus driver for TI tps546d24");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
