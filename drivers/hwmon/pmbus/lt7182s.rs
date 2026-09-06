//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/lt7182s.c
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
// Hardware monitoring driver for Analog Devices LT7182S
//
// Copyright (c) 2022 Guenter Roeck
//

pub const LT7182S_NUM_PAGES: c_int = 2;
pub const MFR_READ_EXTVCC: c_uint = 0xcd;
pub const MFR_READ_ITH: c_uint = 0xce;
pub const MFR_CONFIG_ALL_LT7182S: c_uint = 0xd1;
pub const MFR_IOUT_PEAK: c_uint = 0xd7;
pub const MFR_ADC_CONTROL_LT7182S: c_uint = 0xd8;

pub const MFR_VOUT_PEAK: c_uint = 0xdd;
pub const MFR_VIN_PEAK: c_uint = 0xde;
pub const MFR_TEMPERATURE_1_PEAK: c_uint = 0xdf;
pub const MFR_CLEAR_PEAKS: c_uint = 0xe3;

#[no_mangle]
unsafe extern "C" fn lt7182s_read_word_data(client: *mut i2c_client, page: c_int, phase: c_int, reg: c_int) -> c_int {
    static int lt7182s_read_word_data(struct i2c_client *client, int page, int phase, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VMON:
    if (page == 0 || page == 1)
    ret = pmbus_read_word_data(client, page, phase, MFR_READ_ITH);
    else
    ret = pmbus_read_word_data(client, 0, phase, MFR_READ_EXTVCC);
    break;
    case PMBUS_VIRT_READ_IOUT_MAX:
    ret = pmbus_read_word_data(client, page, phase, MFR_IOUT_PEAK);
    break;
    case PMBUS_VIRT_READ_VOUT_MAX:
    ret = pmbus_read_word_data(client, page, phase, MFR_VOUT_PEAK);
    break;
    case PMBUS_VIRT_READ_VIN_MAX:
    ret = pmbus_read_word_data(client, page, phase, MFR_VIN_PEAK);
    break;
    case PMBUS_VIRT_READ_TEMP_MAX:
    ret = pmbus_read_word_data(client, page, phase, MFR_TEMPERATURE_1_PEAK);
    break;
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    ret = (page == 0) ? 0 : -ENODATA;
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lt7182s_write_word_data(client: *mut i2c_client, page: c_int, reg: c_int, word: u16) -> c_int {
    static int lt7182s_write_word_data(struct i2c_client *client, int page, int reg, u16 word)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    ret = pmbus_write_byte(client, 0, MFR_CLEAR_PEAKS);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static struct pmbus_driver_info lt7182s_info = {
    .pages = LT7182S_NUM_PAGES,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .format[PSC_POWER] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_POUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_STATUS_INPUT | PMBUS_HAVE_STATUS_TEMP,
    .func[1] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_POUT |
    PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_STATUS_INPUT,
    .read_word_data = lt7182s_read_word_data,
    .write_word_data = lt7182s_write_word_data,
    };
#[no_mangle]
unsafe extern "C" fn lt7182s_probe(client: *mut i2c_client) -> c_int {
    static int lt7182s_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct pmbus_driver_info *info;
    u8 buf[I2C_SMBUS_BLOCK_MAX + 1];
    int ret;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_BYTE_DATA |
    I2C_FUNC_SMBUS_READ_WORD_DATA |
    I2C_FUNC_SMBUS_READ_BLOCK_DATA))
    return -ENODEV;
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_ID, buf);
    if (ret < 0) {
    dev_err(dev, "Failed to read PMBUS_MFR_ID\n");
    return ret;
    }
    if (ret != 3 || strncmp(buf, "ADI", 3)) {
    buf[ret] = '\0';
    dev_err(dev, "Manufacturer '%s' not supported\n", buf);
    return -ENODEV;
    }
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_MODEL, buf);
    if (ret < 0) {
    dev_err(dev, "Failed to read PMBUS_MFR_MODEL\n");
    return ret;
    }
    if (ret != 7 || strncmp(buf, "LT7182S", 7)) {
    buf[ret] = '\0';
    dev_err(dev, "Model '%s' not supported\n", buf);
    return -ENODEV;
    }
    info = devm_kmemdup(dev, &lt7182s_info,
    sizeof(struct pmbus_driver_info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
// Set data format to IEEE754 if configured
    ret = i2c_smbus_read_word_data(client, MFR_CONFIG_ALL_LT7182S);
    if (ret < 0)
    return ret;
    if (ret & MFR_CONFIG_IEEE) {
    info.format[PSC_VOLTAGE_IN] = ieee754;
    info.format[PSC_VOLTAGE_OUT] = ieee754;
    info.format[PSC_CURRENT_IN] = ieee754;
    info.format[PSC_CURRENT_OUT] = ieee754;
    info.format[PSC_TEMPERATURE] = ieee754;
    info.format[PSC_POWER] = ieee754;
    }
// Enable VMON output if configured
    ret = i2c_smbus_read_byte_data(client, MFR_ADC_CONTROL_LT7182S);
    if (ret < 0)
    return ret;
    if (ret & MFR_DEBUG_TELEMETRY) {
    info.pages = 3;
    info.func[0] |= PMBUS_HAVE_VMON;
    info.func[1] |= PMBUS_HAVE_VMON;
    info.func[2] = PMBUS_HAVE_VMON;
    }
    return pmbus_do_probe(client, info);
    }
    static const struct i2c_device_id lt7182s_id[] = {
    { .name = "lt7182s" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lt7182s_id);
    static const struct of_device_id __maybe_unused lt7182s_of_match[] = {
    { .compatible = "adi,lt7182s" },
    {}
    };
    static struct i2c_driver lt7182s_driver = {
    .driver = {
    .name = "lt7182s",
    .of_match_table = of_match_ptr(lt7182s_of_match),
    },
    .probe = lt7182s_probe,
    .id_table = lt7182s_id,
    };
    module_i2c_driver(lt7182s_driver);
    MODULE_AUTHOR("Guenter Roeck <linux@roeck-us.net>");
    MODULE_DESCRIPTION("PMBus driver for Analog Devices LT7182S");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
