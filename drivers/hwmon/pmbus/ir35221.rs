//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/ir35221.c
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
// Hardware monitoring driver for IR35221
//
// Copyright (C) IBM Corporation 2017.
//

pub const IR35221_MFR_VIN_PEAK: c_uint = 0xc5;
pub const IR35221_MFR_VOUT_PEAK: c_uint = 0xc6;
pub const IR35221_MFR_IOUT_PEAK: c_uint = 0xc7;
pub const IR35221_MFR_TEMP_PEAK: c_uint = 0xc8;
pub const IR35221_MFR_VIN_VALLEY: c_uint = 0xc9;
pub const IR35221_MFR_VOUT_VALLEY: c_uint = 0xca;
pub const IR35221_MFR_IOUT_VALLEY: c_uint = 0xcb;
pub const IR35221_MFR_TEMP_VALLEY: c_uint = 0xcc;
    static int ir35221_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VIN_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    IR35221_MFR_VIN_PEAK);
    break;
    case PMBUS_VIRT_READ_VOUT_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    IR35221_MFR_VOUT_PEAK);
    break;
    case PMBUS_VIRT_READ_IOUT_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    IR35221_MFR_IOUT_PEAK);
    break;
    case PMBUS_VIRT_READ_TEMP_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    IR35221_MFR_TEMP_PEAK);
    break;
    case PMBUS_VIRT_READ_VIN_MIN:
    ret = pmbus_read_word_data(client, page, phase,
    IR35221_MFR_VIN_VALLEY);
    break;
    case PMBUS_VIRT_READ_VOUT_MIN:
    ret = pmbus_read_word_data(client, page, phase,
    IR35221_MFR_VOUT_VALLEY);
    break;
    case PMBUS_VIRT_READ_IOUT_MIN:
    ret = pmbus_read_word_data(client, page, phase,
    IR35221_MFR_IOUT_VALLEY);
    break;
    case PMBUS_VIRT_READ_TEMP_MIN:
    ret = pmbus_read_word_data(client, page, phase,
    IR35221_MFR_TEMP_VALLEY);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ir35221_probe(client: *mut i2c_client) -> c_int {
    static int ir35221_probe(struct i2c_client *client)
    {
    struct pmbus_driver_info *info;
    u8 buf[I2C_SMBUS_BLOCK_MAX];
    int ret;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_BYTE_DATA
    | I2C_FUNC_SMBUS_READ_WORD_DATA
    | I2C_FUNC_SMBUS_READ_BLOCK_DATA))
    return -ENODEV;
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_ID, buf);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to read PMBUS_MFR_ID\n");
    return ret;
    }
    if (ret != 2 || strncmp(buf, "RI", strlen("RI"))) {
    dev_err(&client.dev, "MFR_ID unrecognised\n");
    return -ENODEV;
    }
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_MODEL, buf);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to read PMBUS_MFR_MODEL\n");
    return ret;
    }
    if (ret != 2 || !(buf[0] == 0x6c && buf[1] == 0x00)) {
    dev_err(&client.dev, "MFR_MODEL unrecognised\n");
    return -ENODEV;
    }
    info = devm_kzalloc(&client.dev, sizeof(struct pmbus_driver_info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.read_word_data = ir35221_read_word_data;
    info.pages = 2;
    info.format[PSC_VOLTAGE_IN] = linear;
    info.format[PSC_VOLTAGE_OUT] = linear;
    info.format[PSC_CURRENT_IN] = linear;
    info.format[PSC_CURRENT_OUT] = linear;
    info.format[PSC_POWER] = linear;
    info.format[PSC_TEMPERATURE] = linear;
    info.func[0] = PMBUS_HAVE_VIN
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_IIN
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_PIN
    | PMBUS_HAVE_POUT | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_STATUS_INPUT | PMBUS_HAVE_STATUS_TEMP;
    info.func[1] = info.func[0];
    return pmbus_do_probe(client, info);
    }
    static const struct i2c_device_id ir35221_id[] = {
    { .name = "ir35221" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ir35221_id);
    static struct i2c_driver ir35221_driver = {
    .driver = {
    .name	= "ir35221",
    },
    .probe		= ir35221_probe,
    .id_table	= ir35221_id,
    };
    module_i2c_driver(ir35221_driver);
    MODULE_AUTHOR("Samuel Mendoza-Jonas <sam@mendozajonas.com");
    MODULE_DESCRIPTION("PMBus driver for IR35221");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
