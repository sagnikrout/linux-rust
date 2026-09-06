//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/max31730.c
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
// Driver for MAX31730 3-Channel Remote Temperature Sensor
//
// Copyright (c) 2019 Guenter Roeck <linux@roeck-us.net>
//

// Addresses scanned
    static const unsigned short normal_i2c[] = { 0x1c, 0x1d, 0x1e, 0x1f, 0x4c,
    0x4d, 0x4e, 0x4f, I2C_CLIENT_END };
// The MAX31730 registers
pub const MAX31730_REG_TEMP: c_uint = 0x00;
pub const MAX31730_REG_CONF: c_uint = 0x13;

pub const MAX31730_REG_TEMP_OFFSET: c_uint = 0x16;
pub const MAX31730_TEMP_OFFSET_BASELINE: c_uint = 0x77;
pub const MAX31730_REG_OFFSET_ENABLE: c_uint = 0x17;
pub const MAX31730_REG_TEMP_MAX: c_uint = 0x20;
pub const MAX31730_REG_TEMP_MIN: c_uint = 0x30;
pub const MAX31730_REG_STATUS_HIGH: c_uint = 0x32;
pub const MAX31730_REG_STATUS_LOW: c_uint = 0x33;
pub const MAX31730_REG_CHANNEL_ENABLE: c_uint = 0x35;
pub const MAX31730_REG_TEMP_FAULT: c_uint = 0x36;
pub const MAX31730_REG_MFG_ID: c_uint = 0x50;
pub const MAX31730_MFG_ID: c_uint = 0x4d;
pub const MAX31730_REG_MFG_REV: c_uint = 0x51;
pub const MAX31730_MFG_REV: c_uint = 0x01;

pub const MAX31730_TEMP_MAX: c_int = 127937;
// Each client has this additional data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max31730_data {
    pub client: *mut i2c_client,
    pub orig_conf: u8,
    pub current_conf: u8,
    pub offset_enable: u8,
    pub channel_enable: u8,
}

// -----------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn max31730_reg_to_mc(temp: i16) -> c_long {
    static inline long max31730_reg_to_mc(s16 temp)
    {
    return DIV_ROUND_CLOSEST((temp >> 4) * 1000, 16);
    }
    static int max31730_write_config(struct max31730_data *data, u8 set_mask,
    u8 clr_mask)
    {
    u8 value;
    clr_mask |= MAX31730_EXTRANGE;
    value = data.current_conf & ~clr_mask;
    value |= set_mask;
    if (data.current_conf != value) {
    s32 err;
    err = i2c_smbus_write_byte_data(data.client, MAX31730_REG_CONF,
    value);
    if (err)
    return err;
    data.current_conf = value;
    }
    return 0;
    }
    static int max31730_set_enable(struct i2c_client *client, int reg,
    u8 *confdata, int channel, bool enable)
    {
    let mut regval: u8 = *confdata;
    int err;
    if (enable)
    regval |= BIT(channel);
    else
    regval &= ~BIT(channel);
    if (regval != *confdata) {
    err = i2c_smbus_write_byte_data(client, reg, regval);
    if (err)
    return err;
// confdata = regval;
    }
    return 0;
    }
    static int max31730_set_offset_enable(struct max31730_data *data, int channel,
    bool enable)
    {
    return max31730_set_enable(data.client, MAX31730_REG_OFFSET_ENABLE,
    &data.offset_enable, channel, enable);
    }
    static int max31730_set_channel_enable(struct max31730_data *data, int channel,
    bool enable)
    {
    return max31730_set_enable(data.client, MAX31730_REG_CHANNEL_ENABLE,
    &data.channel_enable, channel, enable);
    }
    static int max31730_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct max31730_data *data = dev_get_drvdata(dev);
    int regval, reg, offset;
    if (type != hwmon_temp)
    return -EINVAL;
    switch (attr) {
    case hwmon_temp_input:
    if (!(data.channel_enable & BIT(channel)))
    return -ENODATA;
    reg = MAX31730_REG_TEMP + (channel * 2);
    break;
    case hwmon_temp_max:
    reg = MAX31730_REG_TEMP_MAX + (channel * 2);
    break;
    case hwmon_temp_min:
    reg = MAX31730_REG_TEMP_MIN;
    break;
    case hwmon_temp_enable:
// val = !!(data->channel_enable & BIT(channel));
    return 0;
    case hwmon_temp_offset:
    if (!channel)
    return -EINVAL;
    if (!(data.offset_enable & BIT(channel))) {
// val = 0;
    return 0;
    }
    offset = i2c_smbus_read_byte_data(data.client,
    MAX31730_REG_TEMP_OFFSET);
    if (offset < 0)
    return offset;
// val = (offset - MAX31730_TEMP_OFFSET_BASELINE) * 125;
    return 0;
    case hwmon_temp_fault:
    regval = i2c_smbus_read_byte_data(data.client,
    MAX31730_REG_TEMP_FAULT);
    if (regval < 0)
    return regval;
// val = !!(regval & BIT(channel));
    return 0;
    case hwmon_temp_min_alarm:
    regval = i2c_smbus_read_byte_data(data.client,
    MAX31730_REG_STATUS_LOW);
    if (regval < 0)
    return regval;
// val = !!(regval & BIT(channel));
    return 0;
    case hwmon_temp_max_alarm:
    regval = i2c_smbus_read_byte_data(data.client,
    MAX31730_REG_STATUS_HIGH);
    if (regval < 0)
    return regval;
// val = !!(regval & BIT(channel));
    return 0;
    default:
    return -EINVAL;
    }
    regval = i2c_smbus_read_word_swapped(data.client, reg);
    if (regval < 0)
    return regval;
// val = max31730_reg_to_mc(regval);
    return 0;
    }
    static int max31730_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct max31730_data *data = dev_get_drvdata(dev);
    int reg, err;
    if (type != hwmon_temp)
    return -EINVAL;
    switch (attr) {
    case hwmon_temp_max:
    reg = MAX31730_REG_TEMP_MAX + channel * 2;
    break;
    case hwmon_temp_min:
    reg = MAX31730_REG_TEMP_MIN;
    break;
    case hwmon_temp_enable:
    if (val != 0 && val != 1)
    return -EINVAL;
    return max31730_set_channel_enable(data, channel, val);
    case hwmon_temp_offset:
    val = clamp_val(val, -14875, 17000) + 14875;
    val = DIV_ROUND_CLOSEST(val, 125);
    err = max31730_set_offset_enable(data, channel,
    val != MAX31730_TEMP_OFFSET_BASELINE);
    if (err)
    return err;
    return i2c_smbus_write_byte_data(data.client,
    MAX31730_REG_TEMP_OFFSET, val);
    default:
    return -EINVAL;
    }
    val = clamp_val(val, MAX31730_TEMP_MIN, MAX31730_TEMP_MAX);
    val = DIV_ROUND_CLOSEST(val << 4, 1000) << 4;
    return i2c_smbus_write_word_swapped(data.client, reg, (u16)val);
    }
    static umode_t max31730_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    case hwmon_temp_fault:
    return 0444;
    case hwmon_temp_min:
    return channel ? 0444 : 0644;
    case hwmon_temp_offset:
    case hwmon_temp_enable:
    case hwmon_temp_max:
    return 0644;
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct hwmon_channel_info * const max31730_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_ENABLE |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_OFFSET | HWMON_T_ENABLE |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM |
    HWMON_T_FAULT,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_OFFSET | HWMON_T_ENABLE |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM |
    HWMON_T_FAULT,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_OFFSET | HWMON_T_ENABLE |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM |
    HWMON_T_FAULT
    ),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops max31730_hwmon_ops = {
    .is_visible = max31730_is_visible,
    .read = max31730_read,
    .write = max31730_write,
    };
    static const struct hwmon_chip_info max31730_chip_info = {
    .ops = &max31730_hwmon_ops,
    .info = max31730_info,
    };
#[no_mangle]
unsafe extern "C" fn max31730_remove(data: *mut c_void) {
    static void max31730_remove(void *data)
    {
    struct max31730_data *max31730 = data;
    struct i2c_client *client = max31730.client;
    i2c_smbus_write_byte_data(client, MAX31730_REG_CONF,
    max31730.orig_conf);
    }
    static int
    max31730_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct max31730_data *data;
    int status, err;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA))
    return -EIO;
    data = devm_kzalloc(dev, sizeof(struct max31730_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
// Cache original configuration and enable status
    status = i2c_smbus_read_byte_data(client, MAX31730_REG_CHANNEL_ENABLE);
    if (status < 0)
    return status;
    data.channel_enable = status;
    status = i2c_smbus_read_byte_data(client, MAX31730_REG_OFFSET_ENABLE);
    if (status < 0)
    return status;
    data.offset_enable = status;
    status = i2c_smbus_read_byte_data(client, MAX31730_REG_CONF);
    if (status < 0)
    return status;
    data.orig_conf = status;
    data.current_conf = status;
    err = max31730_write_config(data,
    data.channel_enable ? 0 : MAX31730_STOP,
    data.channel_enable ? MAX31730_STOP : 0);
    if (err)
    return err;
    dev_set_drvdata(dev, data);
    err = devm_add_action_or_reset(dev, max31730_remove, data);
    if (err)
    return err;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data,
    &max31730_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id max31730_ids[] = {
    { .name = "max31730" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max31730_ids);
    static const struct of_device_id __maybe_unused max31730_of_match[] = {
    {
    .compatible = "maxim,max31730",
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, max31730_of_match);
    static bool max31730_check_reg_temp(struct i2c_client *client,
    int reg)
    {
    int regval;
    regval = i2c_smbus_read_byte_data(client, reg + 1);
    return regval < 0 || (regval & 0x0f);
    }
// Return 0 if detection is successful, -ENODEV otherwise
    static int max31730_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    int regval;
    int i;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_WORD_DATA))
    return -ENODEV;
    regval = i2c_smbus_read_byte_data(client, MAX31730_REG_MFG_ID);
    if (regval != MAX31730_MFG_ID)
    return -ENODEV;
    regval = i2c_smbus_read_byte_data(client, MAX31730_REG_MFG_REV);
    if (regval != MAX31730_MFG_REV)
    return -ENODEV;
// lower 4 bit of temperature and limit registers must be 0
    if (max31730_check_reg_temp(client, MAX31730_REG_TEMP_MIN))
    return -ENODEV;
    for (i = 0; i < 4; i++) {
    if (max31730_check_reg_temp(client, MAX31730_REG_TEMP + i * 2))
    return -ENODEV;
    if (max31730_check_reg_temp(client,
    MAX31730_REG_TEMP_MAX + i * 2))
    return -ENODEV;
    }
    strscpy(info.type, "max31730", I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max31730_suspend(dev: *mut device) -> c_int {
    static int max31730_suspend(struct device *dev)
    {
    struct max31730_data *data = dev_get_drvdata(dev);
    return max31730_write_config(data, MAX31730_STOP, 0);
    }
#[no_mangle]
unsafe extern "C" fn max31730_resume(dev: *mut device) -> c_int {
    static int max31730_resume(struct device *dev)
    {
    struct max31730_data *data = dev_get_drvdata(dev);
    return max31730_write_config(data, 0, MAX31730_STOP);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(max31730_pm_ops, max31730_suspend, max31730_resume);
    static struct i2c_driver max31730_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "max31730",
    .of_match_table = of_match_ptr(max31730_of_match),
    .pm	= pm_sleep_ptr(&max31730_pm_ops),
    },
    .probe		= max31730_probe,
    .id_table	= max31730_ids,
    .detect		= max31730_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(max31730_driver);
    MODULE_AUTHOR("Guenter Roeck <linux@roeck-us.net>");
    MODULE_DESCRIPTION("MAX31730 driver");
    MODULE_LICENSE("GPL");
