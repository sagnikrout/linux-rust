//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/max1668.c
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
// Copyright (c) 2011 David George <david.george@ska.ac.za>
//
// based on adm1021.c
// some credit to Christoph Scheurer, but largely a rewrite
//

// Addresses to scan
    static const unsigned short max1668_addr_list[] = {
    0x18, 0x19, 0x1a, 0x29, 0x2a, 0x2b, 0x4c, 0x4d, 0x4e, I2C_CLIENT_END };
// max1668 registers

pub const MAX1668_REG_STAT1: c_uint = 0x05;
pub const MAX1668_REG_STAT2: c_uint = 0x06;
pub const MAX1668_REG_MAN_ID: c_uint = 0xfe;
pub const MAX1668_REG_DEV_ID: c_uint = 0xff;
// limits
// high limits

// read low limits

// manufacturer and device ID Constants
pub const MAN_ID_MAXIM: c_uint = 0x4d;
pub const DEV_ID_MAX1668: c_uint = 0x3;
pub const DEV_ID_MAX1805: c_uint = 0x5;
pub const DEV_ID_MAX1989: c_uint = 0xb;
// read only mode module parameter
    static bool read_only;
    module_param(read_only, bool, 0);
    MODULE_PARM_DESC(read_only, "Don't set any values, read only mode");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max1668_data {
    pub regmap: *mut regmap,
    pub channels: c_int,
}

    static int max1668_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct max1668_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    u32 regs[2] = { MAX1668_REG_STAT1, MAX1668_REG_TEMP(channel) };
    u8 regvals[2];
    u32 regval;
    int ret;
    switch (attr) {
    case hwmon_temp_input:
    ret = regmap_read(regmap, MAX1668_REG_TEMP(channel), &regval);
    if (ret)
    return ret;
// val = sign_extend32(regval, 7) * 1000;
    break;
    case hwmon_temp_min:
    ret = regmap_read(regmap, MAX1668_REG_LIML(channel), &regval);
    if (ret)
    return ret;
// val = sign_extend32(regval, 7) * 1000;
    break;
    case hwmon_temp_max:
    ret = regmap_read(regmap, MAX1668_REG_LIMH(channel), &regval);
    if (ret)
    return ret;
// val = sign_extend32(regval, 7) * 1000;
    break;
    case hwmon_temp_min_alarm:
    ret = regmap_read(regmap,
    channel ? MAX1668_REG_STAT2 : MAX1668_REG_STAT1,
    &regval);
    if (ret)
    return ret;
    if (channel)
// val = !!(regval & BIT(9 - channel * 2));
    else
// val = !!(regval & BIT(5));
    break;
    case hwmon_temp_max_alarm:
    ret = regmap_read(regmap,
    channel ? MAX1668_REG_STAT2 : MAX1668_REG_STAT1,
    &regval);
    if (ret)
    return ret;
    if (channel)
// val = !!(regval & BIT(8 - channel * 2));
    else
// val = !!(regval & BIT(6));
    break;
    case hwmon_temp_fault:
    ret = regmap_multi_reg_read(regmap, regs, regvals, 2);
    if (ret)
    return ret;
// val = !!((regvals[0] & BIT(4)) && regvals[1] == 127);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int max1668_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct max1668_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    val = clamp_val(val / 1000, -128, 127);
    switch (attr) {
    case hwmon_temp_min:
    return regmap_write(regmap, MAX1668_REG_LIML(channel), val);
    case hwmon_temp_max:
    return regmap_write(regmap, MAX1668_REG_LIMH(channel), val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t max1668_is_visible(const void *_data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct max1668_data *data = _data;
    if (channel >= data.channels)
    return 0;
    switch (attr) {
    case hwmon_temp_min:
    case hwmon_temp_max:
    return read_only ? 0444 : 0644;
    case hwmon_temp_input:
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    return 0444;
    case hwmon_temp_fault:
    if (channel)
    return 0444;
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct hwmon_channel_info * const max1668_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM |
    HWMON_T_FAULT,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM |
    HWMON_T_FAULT,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM |
    HWMON_T_FAULT,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM |
    HWMON_T_FAULT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops max1668_hwmon_ops = {
    .is_visible = max1668_is_visible,
    .read = max1668_read,
    .write = max1668_write,
    };
    static const struct hwmon_chip_info max1668_chip_info = {
    .ops = &max1668_hwmon_ops,
    .info = max1668_info,
    };
// Return 0 if detection is successful, -ENODEV otherwise
    static int max1668_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    const char *type_name;
    int man_id, dev_id;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
// Check for unsupported part
    man_id = i2c_smbus_read_byte_data(client, MAX1668_REG_MAN_ID);
    if (man_id != MAN_ID_MAXIM)
    return -ENODEV;
    dev_id = i2c_smbus_read_byte_data(client, MAX1668_REG_DEV_ID);
    if (dev_id < 0)
    return -ENODEV;
    type_name = core::ptr::null_mut();
    if (dev_id == DEV_ID_MAX1668)
    type_name = "max1668";
#[no_mangle]
pub unsafe extern "C" fn if(DEV_ID_MAX1805: dev_id ==) -> else {
    else if (dev_id == DEV_ID_MAX1805)
    type_name = "max1805";
#[no_mangle]
pub unsafe extern "C" fn if(DEV_ID_MAX1989: dev_id ==) -> else {
    else if (dev_id == DEV_ID_MAX1989)
    type_name = "max1989";
    if (!type_name)
    return -ENODEV;
    strscpy(info.type, type_name, I2C_NAME_SIZE);
    return 0;
    }
// regmap
#[no_mangle]
unsafe extern "C" fn max1668_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int max1668_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(context, reg);
    if (ret < 0)
    return ret;
// val = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max1668_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int max1668_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    return i2c_smbus_write_byte_data(context, reg + 11, val);
    }
#[no_mangle]
unsafe extern "C" fn max1668_regmap_is_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool max1668_regmap_is_volatile(struct device *dev, unsigned int reg)
    {
    return reg <= MAX1668_REG_STAT2;
    }
#[no_mangle]
unsafe extern "C" fn max1668_regmap_is_writeable(dev: *mut device, reg: c_uint) -> bool {
    static bool max1668_regmap_is_writeable(struct device *dev, unsigned int reg)
    {
    return reg > MAX1668_REG_STAT2 && reg <= MAX1668_REG_LIML(4);
    }
    static const struct regmap_config max1668_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .cache_type = REGCACHE_MAPLE,
    .volatile_reg = max1668_regmap_is_volatile,
    .writeable_reg = max1668_regmap_is_writeable,
    };
    static const struct regmap_bus max1668_regmap_bus = {
    .reg_write = max1668_reg_write,
    .reg_read = max1668_reg_read,
    };
#[no_mangle]
unsafe extern "C" fn max1668_probe(client: *mut i2c_client) -> c_int {
    static int max1668_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct max1668_data *data;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    data = devm_kzalloc(dev, sizeof(struct max1668_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = devm_regmap_init(dev, &max1668_regmap_bus, client,
    &max1668_regmap_config);
    if (IS_ERR(data.regmap))
    return PTR_ERR(data.regmap);
    data.channels = (uintptr_t)i2c_get_match_data(client);
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name, data,
    &max1668_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id max1668_id[] = {
    { .name = "max1668", .driver_data = 5 },
    { .name = "max1805", .driver_data = 3 },
    { .name = "max1989", .driver_data = 5 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max1668_id);
// This is the driver that will be inserted
    static struct i2c_driver max1668_driver = {
    .class = I2C_CLASS_HWMON,
    .driver = {
    .name	= "max1668",
    },
    .probe = max1668_probe,
    .id_table = max1668_id,
    .detect	= max1668_detect,
    .address_list = max1668_addr_list,
    };
    module_i2c_driver(max1668_driver);
    MODULE_AUTHOR("David George <david.george@ska.ac.za>");
    MODULE_DESCRIPTION("MAX1668 remote temperature sensor driver");
    MODULE_LICENSE("GPL");
