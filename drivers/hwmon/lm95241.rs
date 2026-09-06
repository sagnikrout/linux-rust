//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/lm95241.c
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
// Copyright (C) 2008, 2010 Davide Rizzo <elpa.rizzo@gmail.com>
//
// The LM95241 is a sensor chip made by National Semiconductors.
// It reports up to three temperatures (its own plus up to two external ones).
// Complete datasheet can be obtained from National's website at:
// http://www.national.com/ds.cgi/LM/LM95241.pdf
//

    static const unsigned short normal_i2c[] = {
    0x19, 0x2a, 0x2b, I2C_CLIENT_END };
// LM95241 registers
pub const LM95241_REG_R_MAN_ID: c_uint = 0xFE;
pub const LM95241_REG_R_CHIP_ID: c_uint = 0xFF;
pub const LM95241_REG_R_STATUS: c_uint = 0x02;
pub const LM95241_REG_RW_CONFIG: c_uint = 0x03;
pub const LM95241_REG_RW_REM_FILTER: c_uint = 0x06;
pub const LM95241_REG_RW_TRUTHERM: c_uint = 0x07;
pub const LM95241_REG_W_ONE_SHOT: c_uint = 0x0F;
pub const LM95241_REG_R_LOCAL_TEMPH: c_uint = 0x10;
pub const LM95241_REG_R_REMOTE1_TEMPH: c_uint = 0x11;
pub const LM95241_REG_R_REMOTE2_TEMPH: c_uint = 0x12;
pub const LM95241_REG_R_LOCAL_TEMPL: c_uint = 0x20;
pub const LM95241_REG_R_REMOTE1_TEMPL: c_uint = 0x21;
pub const LM95241_REG_R_REMOTE2_TEMPL: c_uint = 0x22;
pub const LM95241_REG_RW_REMOTE_MODEL: c_uint = 0x30;
// LM95241 specific bitfields

pub const CFG_CR0076: c_uint = 0x00;

pub const TT1_SHIFT: c_int = 0;
pub const TT2_SHIFT: c_int = 4;
pub const TT_OFF: c_int = 0;
pub const TT_ON: c_int = 1;
pub const TT_MASK: c_int = 7;
pub const NATSEMI_MAN_ID: c_uint = 0x01;
pub const LM95231_CHIP_ID: c_uint = 0xA1;
pub const LM95241_CHIP_ID: c_uint = 0xA4;
    static const u8 lm95241_reg_address[] = {
    LM95241_REG_R_LOCAL_TEMPH,
    LM95241_REG_R_LOCAL_TEMPL,
    LM95241_REG_R_REMOTE1_TEMPH,
    LM95241_REG_R_REMOTE1_TEMPL,
    LM95241_REG_R_REMOTE2_TEMPH,
    LM95241_REG_R_REMOTE2_TEMPL
    };
// Client data (each client gets its own)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm95241_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
    pub /: *mut *mut unsigned long interval; / in milli-seconds,
    pub /: *mut *mut bool valid; / false until following fields are valid,
// registers values
    pub temp: [u8; ARRAY_SIZE(lm95241_reg_address)],
    pub trutherm: u8 status, config, model,,
}

// Conversions
#[no_mangle]
unsafe extern "C" fn temp_from_reg_signed(val_h: u8, val_l: u8) -> c_int {
    static int temp_from_reg_signed(u8 val_h, u8 val_l)
    {
    let mut val_hl: i16 = (val_h << 8) | val_l;
    return val_hl * 1000 / 256;
    }
#[no_mangle]
unsafe extern "C" fn temp_from_reg_unsigned(val_h: u8, val_l: u8) -> c_int {
    static int temp_from_reg_unsigned(u8 val_h, u8 val_l)
    {
    let mut val_hl: u16 = (val_h << 8) | val_l;
    return val_hl * 1000 / 256;
    }
    static struct lm95241_data *lm95241_update_device(struct device *dev)
    {
    struct lm95241_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    if (time_after(jiffies, data.last_updated
    + msecs_to_jiffies(data.interval)) ||
    !data.valid) {
    int i;
    dev_dbg(dev, "Updating lm95241 data.\n");
    for (i = 0; i < ARRAY_SIZE(lm95241_reg_address); i++)
    data.temp[i]
    = i2c_smbus_read_byte_data(client,
    lm95241_reg_address[i]);
    data.status = i2c_smbus_read_byte_data(client,
    LM95241_REG_R_STATUS);
    data.last_updated = jiffies;
    data.valid = true;
    }
    return data;
    }
    static int lm95241_read_chip(struct device *dev, u32 attr, int channel,
    long *val)
    {
    struct lm95241_data *data = dev_get_drvdata(dev);
    switch (attr) {
    case hwmon_chip_update_interval:
// val = data->interval;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int lm95241_read_temp(struct device *dev, u32 attr, int channel,
    long *val)
    {
    struct lm95241_data *data = lm95241_update_device(dev);
    switch (attr) {
    case hwmon_temp_input:
    if (!channel || (data.config & BIT(channel - 1)))
// val = temp_from_reg_signed(data->temp[channel * 2],
    data.temp[channel * 2 + 1]);
    else
// val = temp_from_reg_unsigned(data->temp[channel * 2],
    data.temp[channel * 2 + 1]);
    return 0;
    case hwmon_temp_min:
    if (channel == 1)
// val = (data->config & R1DF_MASK) ? -128000 : 0;
    else
// val = (data->config & R2DF_MASK) ? -128000 : 0;
    return 0;
    case hwmon_temp_max:
    if (channel == 1)
// val = (data->config & R1DF_MASK) ? 127875 : 255875;
    else
// val = (data->config & R2DF_MASK) ? 127875 : 255875;
    return 0;
    case hwmon_temp_type:
    if (channel == 1)
// val = (data->model & R1MS_MASK) ? 1 : 2;
    else
// val = (data->model & R2MS_MASK) ? 1 : 2;
    return 0;
    case hwmon_temp_fault:
    if (channel == 1)
// val = !!(data->status & R1DM);
    else
// val = !!(data->status & R2DM);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int lm95241_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_chip:
    return lm95241_read_chip(dev, attr, channel, val);
    case hwmon_temp:
    return lm95241_read_temp(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int lm95241_write_chip(struct device *dev, u32 attr, int channel,
    long val)
    {
    struct lm95241_data *data = dev_get_drvdata(dev);
    int convrate;
    u8 config;
    int ret;
    switch (attr) {
    case hwmon_chip_update_interval:
    config = data.config & ~CFG_CRMASK;
    if (val < 130) {
    convrate = 76;
    config |= CFG_CR0076;
    } else if (val < 590) {
    convrate = 182;
    config |= CFG_CR0182;
    } else if (val < 1850) {
    convrate = 1000;
    config |= CFG_CR1000;
    } else {
    convrate = 2700;
    config |= CFG_CR2700;
    }
    data.interval = convrate;
    data.config = config;
    ret = i2c_smbus_write_byte_data(data.client,
    LM95241_REG_RW_CONFIG, config);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    return ret;
    }
    static int lm95241_write_temp(struct device *dev, u32 attr, int channel,
    long val)
    {
    struct lm95241_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret;
    switch (attr) {
    case hwmon_temp_min:
    if (channel == 1) {
    if (val < 0)
    data.config |= R1DF_MASK;
    else
    data.config &= ~R1DF_MASK;
    } else {
    if (val < 0)
    data.config |= R2DF_MASK;
    else
    data.config &= ~R2DF_MASK;
    }
    data.valid = false;
    ret = i2c_smbus_write_byte_data(client, LM95241_REG_RW_CONFIG,
    data.config);
    break;
    case hwmon_temp_max:
    if (channel == 1) {
    if (val <= 127875)
    data.config |= R1DF_MASK;
    else
    data.config &= ~R1DF_MASK;
    } else {
    if (val <= 127875)
    data.config |= R2DF_MASK;
    else
    data.config &= ~R2DF_MASK;
    }
    data.valid = false;
    ret = i2c_smbus_write_byte_data(client, LM95241_REG_RW_CONFIG,
    data.config);
    break;
    case hwmon_temp_type:
    if (val != 1 && val != 2) {
    ret = -EINVAL;
    break;
    }
    if (channel == 1) {
    data.trutherm &= ~(TT_MASK << TT1_SHIFT);
    if (val == 1) {
    data.model |= R1MS_MASK;
    data.trutherm |= (TT_ON << TT1_SHIFT);
    } else {
    data.model &= ~R1MS_MASK;
    data.trutherm |= (TT_OFF << TT1_SHIFT);
    }
    } else {
    data.trutherm &= ~(TT_MASK << TT2_SHIFT);
    if (val == 1) {
    data.model |= R2MS_MASK;
    data.trutherm |= (TT_ON << TT2_SHIFT);
    } else {
    data.model &= ~R2MS_MASK;
    data.trutherm |= (TT_OFF << TT2_SHIFT);
    }
    }
    ret = i2c_smbus_write_byte_data(client,
    LM95241_REG_RW_REMOTE_MODEL,
    data.model);
    if (ret < 0)
    break;
    ret = i2c_smbus_write_byte_data(client, LM95241_REG_RW_TRUTHERM,
    data.trutherm);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    return ret;
    }
    static int lm95241_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_chip:
    return lm95241_write_chip(dev, attr, channel, val);
    case hwmon_temp:
    return lm95241_write_temp(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t lm95241_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_chip:
    switch (attr) {
    case hwmon_chip_update_interval:
    return 0644;
    }
    break;
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    return 0444;
    case hwmon_temp_fault:
    return 0444;
    case hwmon_temp_min:
    case hwmon_temp_max:
    case hwmon_temp_type:
    return 0644;
    }
    break;
    default:
    break;
    }
    return 0;
    }
// Return 0 if detection is successful, -ENODEV otherwise
    static int lm95241_detect(struct i2c_client *new_client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = new_client.adapter;
    const char *name;
    int mfg_id, chip_id;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    mfg_id = i2c_smbus_read_byte_data(new_client, LM95241_REG_R_MAN_ID);
    if (mfg_id != NATSEMI_MAN_ID)
    return -ENODEV;
    chip_id = i2c_smbus_read_byte_data(new_client, LM95241_REG_R_CHIP_ID);
    switch (chip_id) {
    case LM95231_CHIP_ID:
    name = "lm95231";
    break;
    case LM95241_CHIP_ID:
    name = "lm95241";
    break;
    default:
    return -ENODEV;
    }
// Fill the i2c board info
    strscpy(info.type, name, I2C_NAME_SIZE);
    return 0;
    }
    static void lm95241_init_client(struct i2c_client *client,
    struct lm95241_data *data)
    {
    data.interval = 1000;
    data.config = CFG_CR1000;
    data.trutherm = (TT_OFF << TT1_SHIFT) | (TT_OFF << TT2_SHIFT);
    i2c_smbus_write_byte_data(client, LM95241_REG_RW_CONFIG, data.config);
    i2c_smbus_write_byte_data(client, LM95241_REG_RW_REM_FILTER,
    R1FE_MASK | R2FE_MASK);
    i2c_smbus_write_byte_data(client, LM95241_REG_RW_TRUTHERM,
    data.trutherm);
    i2c_smbus_write_byte_data(client, LM95241_REG_RW_REMOTE_MODEL,
    data.model);
    }
    static const struct hwmon_channel_info * const lm95241_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_UPDATE_INTERVAL),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_TYPE | HWMON_T_FAULT,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_TYPE | HWMON_T_FAULT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops lm95241_hwmon_ops = {
    .is_visible = lm95241_is_visible,
    .read = lm95241_read,
    .write = lm95241_write,
    };
    static const struct hwmon_chip_info lm95241_chip_info = {
    .ops = &lm95241_hwmon_ops,
    .info = lm95241_info,
    };
#[no_mangle]
unsafe extern "C" fn lm95241_probe(client: *mut i2c_client) -> c_int {
    static int lm95241_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct lm95241_data *data;
    struct device *hwmon_dev;
    data = devm_kzalloc(dev, sizeof(struct lm95241_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
// Initialize the LM95241 chip
    lm95241_init_client(client, data);
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data,
    &lm95241_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
// Driver data (common to all clients)
    static const struct i2c_device_id lm95241_id[] = {
    { .name = "lm95231" },
    { .name = "lm95241" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lm95241_id);
    static struct i2c_driver lm95241_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= DEVNAME,
    },
    .probe		= lm95241_probe,
    .id_table	= lm95241_id,
    .detect		= lm95241_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(lm95241_driver);
    MODULE_AUTHOR("Davide Rizzo <elpa.rizzo@gmail.com>");
    MODULE_DESCRIPTION("LM95231/LM95241 sensor driver");
    MODULE_LICENSE("GPL");
