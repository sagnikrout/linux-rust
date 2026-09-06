//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/pm6764tr.c
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
// Hardware monitoring driver for STMicroelectronics digital controller PM6764TR
//

pub const PM6764TR_PMBUS_READ_VOUT: c_uint = 0xD4;
#[no_mangle]
unsafe extern "C" fn pm6764tr_read_word_data(client: *mut i2c_client, page: c_int, phase: c_int, reg: c_int) -> c_int {
    static int pm6764tr_read_word_data(struct i2c_client *client, int page, int phase, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VMON:
    ret = pmbus_read_word_data(client, page, phase, PM6764TR_PMBUS_READ_VOUT);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static struct pmbus_driver_info pm6764tr_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = vid,
    .format[PSC_TEMPERATURE] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_POWER] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN |  PMBUS_HAVE_PIN |
    PMBUS_HAVE_IOUT | PMBUS_HAVE_POUT | PMBUS_HAVE_VMON |
    PMBUS_HAVE_STATUS_IOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP,
    .read_word_data = pm6764tr_read_word_data,
    };
#[no_mangle]
unsafe extern "C" fn pm6764tr_probe(client: *mut i2c_client) -> c_int {
    static int pm6764tr_probe(struct i2c_client *client)
    {
    return pmbus_do_probe(client, &pm6764tr_info);
    }
    static const struct i2c_device_id pm6764tr_id[] = {
    { .name = "pm6764tr" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pm6764tr_id);
    static const struct of_device_id __maybe_unused pm6764tr_of_match[] = {
    {.compatible = "st,pm6764tr"},
    {}
    };
// This is the driver that will be inserted
    static struct i2c_driver pm6764tr_driver = {
    .driver = {
    .name = "pm6764tr",
    .of_match_table = of_match_ptr(pm6764tr_of_match),
    },
    .probe = pm6764tr_probe,
    .id_table = pm6764tr_id,
    };
    module_i2c_driver(pm6764tr_driver);
    MODULE_AUTHOR("Charles Hsu");
    MODULE_DESCRIPTION("PMBus driver for  ST PM6764TR");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
