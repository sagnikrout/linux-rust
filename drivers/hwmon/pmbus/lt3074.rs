//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/lt3074.c
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
// Hardware monitoring driver for Analog Devices LT3074
//
// Copyright (C) 2025 Analog Devices, Inc.
//

pub const LT3074_MFR_READ_VBIAS: c_uint = 0xc6;
pub const LT3074_MFR_BIAS_OV_WARN_LIMIT: c_uint = 0xc7;
pub const LT3074_MFR_BIAS_UV_WARN_LIMIT: c_uint = 0xc8;
pub const LT3074_MFR_SPECIAL_ID: c_uint = 0xe7;
pub const LT3074_SPECIAL_ID_VALUE: c_uint = 0x1c1d;
    static const struct regulator_desc __maybe_unused lt3074_reg_desc[] = {
    PMBUS_REGULATOR_ONE("regulator"),
    };
    static int lt3074_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    switch (reg) {
    case PMBUS_VIRT_READ_VMON:
    return pmbus_read_word_data(client, page, phase,
    LT3074_MFR_READ_VBIAS);
    case PMBUS_VIRT_VMON_UV_WARN_LIMIT:
    return pmbus_read_word_data(client, page, phase,
    LT3074_MFR_BIAS_UV_WARN_LIMIT);
    case PMBUS_VIRT_VMON_OV_WARN_LIMIT:
    return pmbus_read_word_data(client, page, phase,
    LT3074_MFR_BIAS_OV_WARN_LIMIT);
    default:
    return -ENODATA;
    }
    }
    static int lt3074_write_word_data(struct i2c_client *client, int page,
    int reg, u16 word)
    {
    switch (reg) {
    case PMBUS_VIRT_VMON_UV_WARN_LIMIT:
    return pmbus_write_word_data(client, 0,
    LT3074_MFR_BIAS_UV_WARN_LIMIT,
    word);
    case PMBUS_VIRT_VMON_OV_WARN_LIMIT:
    return pmbus_write_word_data(client, 0,
    LT3074_MFR_BIAS_OV_WARN_LIMIT,
    word);
    default:
    return -ENODATA;
    }
    }
    static struct pmbus_driver_info lt3074_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_VMON |
    PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_STATUS_INPUT | PMBUS_HAVE_STATUS_TEMP,
    .read_word_data = lt3074_read_word_data,
    .write_word_data = lt3074_write_word_data,

    .num_regulators = 1,
    .reg_desc = lt3074_reg_desc,

    };
#[no_mangle]
unsafe extern "C" fn lt3074_probe(client: *mut i2c_client) -> c_int {
    static int lt3074_probe(struct i2c_client *client)
    {
    int ret;
    struct device *dev = &client.dev;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_WORD_DATA))
    return -ENODEV;
    ret = i2c_smbus_read_word_data(client, LT3074_MFR_SPECIAL_ID);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to read ID\n");
    if (ret != LT3074_SPECIAL_ID_VALUE)
    return dev_err_probe(dev, -ENODEV, "ID mismatch\n");
    return pmbus_do_probe(client, &lt3074_info);
    }
    static const struct i2c_device_id lt3074_id[] = {
    { .name = "lt3074" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lt3074_id);
    static const struct of_device_id __maybe_unused lt3074_of_match[] = {
    { .compatible = "adi,lt3074" },
    {}
    };
    MODULE_DEVICE_TABLE(of, lt3074_of_match);
    static struct i2c_driver lt3074_driver = {
    .driver = {
    .name = "lt3074",
    .of_match_table = of_match_ptr(lt3074_of_match),
    },
    .probe = lt3074_probe,
    .id_table = lt3074_id,
    };
    module_i2c_driver(lt3074_driver);
    MODULE_AUTHOR("Cedric Encarnacion <cedricjustine.encarnacion@analog.com>");
    MODULE_DESCRIPTION("PMBus driver for Analog Devices LT3074");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
