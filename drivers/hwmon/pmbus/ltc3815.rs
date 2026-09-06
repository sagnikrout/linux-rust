//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/ltc3815.c
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
// Hardware monitoring driver for LTC3815
//
// Copyright (c) 2015 Linear Technology
// Copyright (c) 2015 Guenter Roeck
//

pub const LTC3815_MFR_IOUT_PEAK: c_uint = 0xd7;
pub const LTC3815_MFR_VOUT_PEAK: c_uint = 0xdd;
pub const LTC3815_MFR_VIN_PEAK: c_uint = 0xde;
pub const LTC3815_MFR_TEMP_PEAK: c_uint = 0xdf;
pub const LTC3815_MFR_IIN_PEAK: c_uint = 0xe1;
pub const LTC3815_MFR_SPECIAL_ID: c_uint = 0xe7;
pub const LTC3815_ID: c_uint = 0x8000;
pub const LTC3815_ID_MASK: c_uint = 0xff00;
#[no_mangle]
unsafe extern "C" fn ltc3815_read_byte_data(client: *mut i2c_client, page: c_int, reg: c_int) -> c_int {
    static int ltc3815_read_byte_data(struct i2c_client *client, int page, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_VOUT_MODE:
//
// The chip returns 0x3e, suggesting VID mode with manufacturer
// specific VID codes. Since the output voltage is reported
// with a LSB of 0.5mV, override and report direct mode with
// appropriate coefficients.
//
    ret = 0x40;
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ltc3815_write_byte(client: *mut i2c_client, page: c_int, reg: u8) -> c_int {
    static int ltc3815_write_byte(struct i2c_client *client, int page, u8 reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_CLEAR_FAULTS:
//
// LTC3815 does not support the CLEAR_FAULTS command.
// Emulate it by clearing the status register.
//
    ret = pmbus_read_word_data(client, 0, 0xff, PMBUS_STATUS_WORD);
    if (ret > 0) {
    pmbus_write_word_data(client, 0, PMBUS_STATUS_WORD,
    ret);
    ret = 0;
    }
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int ltc3815_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VIN_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    LTC3815_MFR_VIN_PEAK);
    break;
    case PMBUS_VIRT_READ_VOUT_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    LTC3815_MFR_VOUT_PEAK);
    break;
    case PMBUS_VIRT_READ_TEMP_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    LTC3815_MFR_TEMP_PEAK);
    break;
    case PMBUS_VIRT_READ_IOUT_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    LTC3815_MFR_IOUT_PEAK);
    break;
    case PMBUS_VIRT_READ_IIN_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    LTC3815_MFR_IIN_PEAK);
    break;
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    case PMBUS_VIRT_RESET_IOUT_HISTORY:
    case PMBUS_VIRT_RESET_IIN_HISTORY:
    ret = 0;
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int ltc3815_write_word_data(struct i2c_client *client, int page,
    int reg, u16 word)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_RESET_IIN_HISTORY:
    ret = pmbus_write_word_data(client, page,
    LTC3815_MFR_IIN_PEAK, 0);
    break;
    case PMBUS_VIRT_RESET_IOUT_HISTORY:
    ret = pmbus_write_word_data(client, page,
    LTC3815_MFR_IOUT_PEAK, 0);
    break;
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
    ret = pmbus_write_word_data(client, page,
    LTC3815_MFR_VOUT_PEAK, 0);
    break;
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    ret = pmbus_write_word_data(client, page,
    LTC3815_MFR_VIN_PEAK, 0);
    break;
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    ret = pmbus_write_word_data(client, page,
    LTC3815_MFR_TEMP_PEAK, 0);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static const struct i2c_device_id ltc3815_id[] = {
    { .name = "ltc3815" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc3815_id);
    static struct pmbus_driver_info ltc3815_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = direct,
    .format[PSC_VOLTAGE_OUT] = direct,
    .format[PSC_CURRENT_IN] = direct,
    .format[PSC_CURRENT_OUT] = direct,
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_VOLTAGE_IN] = 250,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = 0,
    .m[PSC_VOLTAGE_OUT] = 2,
    .b[PSC_VOLTAGE_OUT] = 0,
    .R[PSC_VOLTAGE_OUT] = 3,
    .m[PSC_CURRENT_IN] = 1,
    .b[PSC_CURRENT_IN] = 0,
    .R[PSC_CURRENT_IN] = 2,
    .m[PSC_CURRENT_OUT] = 1,
    .b[PSC_CURRENT_OUT] = 0,
    .R[PSC_CURRENT_OUT] = 2,
    .m[PSC_TEMPERATURE] = 1,
    .b[PSC_TEMPERATURE] = 0,
    .R[PSC_TEMPERATURE] = 0,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN | PMBUS_HAVE_VOUT |
    PMBUS_HAVE_IOUT | PMBUS_HAVE_TEMP,
    .read_byte_data = ltc3815_read_byte_data,
    .read_word_data = ltc3815_read_word_data,
    .write_byte = ltc3815_write_byte,
    .write_word_data = ltc3815_write_word_data,
    };
#[no_mangle]
unsafe extern "C" fn ltc3815_probe(client: *mut i2c_client) -> c_int {
    static int ltc3815_probe(struct i2c_client *client)
    {
    int chip_id;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_WORD_DATA))
    return -ENODEV;
    chip_id = i2c_smbus_read_word_data(client, LTC3815_MFR_SPECIAL_ID);
    if (chip_id < 0)
    return chip_id;
    if ((chip_id & LTC3815_ID_MASK) != LTC3815_ID)
    return -ENODEV;
    return pmbus_do_probe(client, &ltc3815_info);
    }
    static struct i2c_driver ltc3815_driver = {
    .driver = {
    .name = "ltc3815",
    },
    .probe = ltc3815_probe,
    .id_table = ltc3815_id,
    };
    module_i2c_driver(ltc3815_driver);
    MODULE_AUTHOR("Guenter Roeck");
    MODULE_DESCRIPTION("PMBus driver for LTC3815");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
