//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/lm83.c
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
// lm83.c - Part of lm_sensors, Linux kernel modules for hardware
// monitoring
// Copyright (C) 2003-2009  Jean Delvare <jdelvare@suse.de>
//
// Heavily inspired from the lm78, lm75 and adm1021 drivers. The LM83 is
// a sensor chip made by National Semiconductor. It reports up to four
// temperatures (its own plus up to three external ones) with a 1 deg
// resolution and a 3-4 deg accuracy. Complete datasheet can be obtained
// from National's website at:
// http://www.national.com/pf/LM/LM83.html
// Since the datasheet omits to give the chip stepping code, I give it
// here: 0x03 (at register 0xff).
//
// Also supports the LM82 temp sensor, which is basically a stripped down
// model of the LM83.  Datasheet is here:
// http://www.national.com/pf/LM/LM82.html
//

//
// Addresses to scan
// Address is selected using 2 three-level pins, resulting in 9 possible
// addresses.
//
    static const unsigned short normal_i2c[] = {
    0x18, 0x19, 0x1a, 0x29, 0x2a, 0x2b, 0x4c, 0x4d, 0x4e, I2C_CLIENT_END };
    enum chips { lm83, lm82 };
//
// The LM83 registers
// Manufacturer ID is 0x01 for National Semiconductor.
//
pub const LM83_REG_R_MAN_ID: c_uint = 0xFE;
pub const LM83_REG_R_CHIP_ID: c_uint = 0xFF;
pub const LM83_REG_R_CONFIG: c_uint = 0x03;
pub const LM83_REG_W_CONFIG: c_uint = 0x09;
pub const LM83_REG_R_STATUS1: c_uint = 0x02;
pub const LM83_REG_R_STATUS2: c_uint = 0x35;
pub const LM83_REG_R_LOCAL_TEMP: c_uint = 0x00;
pub const LM83_REG_R_LOCAL_HIGH: c_uint = 0x05;
pub const LM83_REG_W_LOCAL_HIGH: c_uint = 0x0B;
pub const LM83_REG_R_REMOTE1_TEMP: c_uint = 0x30;
pub const LM83_REG_R_REMOTE1_HIGH: c_uint = 0x38;
pub const LM83_REG_W_REMOTE1_HIGH: c_uint = 0x50;
pub const LM83_REG_R_REMOTE2_TEMP: c_uint = 0x01;
pub const LM83_REG_R_REMOTE2_HIGH: c_uint = 0x07;
pub const LM83_REG_W_REMOTE2_HIGH: c_uint = 0x0D;
pub const LM83_REG_R_REMOTE3_TEMP: c_uint = 0x31;
pub const LM83_REG_R_REMOTE3_HIGH: c_uint = 0x3A;
pub const LM83_REG_W_REMOTE3_HIGH: c_uint = 0x52;
pub const LM83_REG_R_TCRIT: c_uint = 0x42;
pub const LM83_REG_W_TCRIT: c_uint = 0x5A;
    static const u8 LM83_REG_TEMP[] = {
    LM83_REG_R_LOCAL_TEMP,
    LM83_REG_R_REMOTE1_TEMP,
    LM83_REG_R_REMOTE2_TEMP,
    LM83_REG_R_REMOTE3_TEMP,
    };
    static const u8 LM83_REG_MAX[] = {
    LM83_REG_R_LOCAL_HIGH,
    LM83_REG_R_REMOTE1_HIGH,
    LM83_REG_R_REMOTE2_HIGH,
    LM83_REG_R_REMOTE3_HIGH,
    };
// alarm and fault registers and bits, indexed by channel
    static const u8 LM83_ALARM_REG[] = {
    LM83_REG_R_STATUS1, LM83_REG_R_STATUS2, LM83_REG_R_STATUS1, LM83_REG_R_STATUS2
    };
    static const u8 LM83_MAX_ALARM_BIT[] = {
    BIT(6), BIT(7), BIT(4), BIT(4)
    };
    static const u8 LM83_CRIT_ALARM_BIT[] = {
    BIT(0), BIT(0), BIT(1), BIT(1)
    };
    static const u8 LM83_FAULT_BIT[] = {
    0, BIT(5), BIT(2), BIT(2)
    };
//
// Client data (each client gets its own)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm83_data {
    pub regmap: *mut regmap,
    pub type: enum chips,
}

// regmap code
#[no_mangle]
unsafe extern "C" fn lm83_regmap_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int lm83_regmap_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct i2c_client *client = context;
    int ret;
    ret = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0)
    return ret;
// val = ret;
    return 0;
    }
//
// The regmap write function maps read register addresses to write register
// addresses. This is necessary for regmap register caching to work.
// An alternative would be to clear the regmap cache whenever a register is
// written, but that would be much more expensive.
//
#[no_mangle]
unsafe extern "C" fn lm83_regmap_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int lm83_regmap_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    struct i2c_client *client = context;
    switch (reg) {
    case LM83_REG_R_CONFIG:
    case LM83_REG_R_LOCAL_HIGH:
    case LM83_REG_R_REMOTE2_HIGH:
    reg += 0x06;
    break;
    case LM83_REG_R_REMOTE1_HIGH:
    case LM83_REG_R_REMOTE3_HIGH:
    case LM83_REG_R_TCRIT:
    reg += 0x18;
    break;
    default:
    break;
    }
    return i2c_smbus_write_byte_data(client, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn lm83_regmap_is_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool lm83_regmap_is_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case LM83_REG_R_LOCAL_TEMP:
    case LM83_REG_R_REMOTE1_TEMP:
    case LM83_REG_R_REMOTE2_TEMP:
    case LM83_REG_R_REMOTE3_TEMP:
    case LM83_REG_R_STATUS1:
    case LM83_REG_R_STATUS2:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config lm83_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .cache_type = REGCACHE_MAPLE,
    .volatile_reg = lm83_regmap_is_volatile,
    .reg_read = lm83_regmap_reg_read,
    .reg_write = lm83_regmap_reg_write,
    };
// hwmon API
#[no_mangle]
unsafe extern "C" fn lm83_temp_read(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int lm83_temp_read(struct device *dev, u32 attr, int channel, long *val)
    {
    struct lm83_data *data = dev_get_drvdata(dev);
    unsigned int regval;
    int err;
    switch (attr) {
    case hwmon_temp_input:
    err = regmap_read(data.regmap, LM83_REG_TEMP[channel], &regval);
    if (err < 0)
    return err;
// val = (s8)regval * 1000;
    break;
    case hwmon_temp_max:
    err = regmap_read(data.regmap, LM83_REG_MAX[channel], &regval);
    if (err < 0)
    return err;
// val = (s8)regval * 1000;
    break;
    case hwmon_temp_crit:
    err = regmap_read(data.regmap, LM83_REG_R_TCRIT, &regval);
    if (err < 0)
    return err;
// val = (s8)regval * 1000;
    break;
    case hwmon_temp_max_alarm:
    err = regmap_read(data.regmap, LM83_ALARM_REG[channel], &regval);
    if (err < 0)
    return err;
// val = !!(regval & LM83_MAX_ALARM_BIT[channel]);
    break;
    case hwmon_temp_crit_alarm:
    err = regmap_read(data.regmap, LM83_ALARM_REG[channel], &regval);
    if (err < 0)
    return err;
// val = !!(regval & LM83_CRIT_ALARM_BIT[channel]);
    break;
    case hwmon_temp_fault:
    err = regmap_read(data.regmap, LM83_ALARM_REG[channel], &regval);
    if (err < 0)
    return err;
// val = !!(regval & LM83_FAULT_BIT[channel]);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm83_temp_write(dev: *mut device, attr: u32, channel: c_int, val: c_long) -> c_int {
    static int lm83_temp_write(struct device *dev, u32 attr, int channel, long val)
    {
    struct lm83_data *data = dev_get_drvdata(dev);
    unsigned int regval;
    int err;
    regval = DIV_ROUND_CLOSEST(clamp_val(val, -128000, 127000), 1000);
    switch (attr) {
    case hwmon_temp_max:
    err = regmap_write(data.regmap, LM83_REG_MAX[channel], regval);
    if (err < 0)
    return err;
    break;
    case hwmon_temp_crit:
    err = regmap_write(data.regmap, LM83_REG_R_TCRIT, regval);
    if (err < 0)
    return err;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm83_chip_read(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int lm83_chip_read(struct device *dev, u32 attr, int channel, long *val)
    {
    struct lm83_data *data = dev_get_drvdata(dev);
    unsigned int regval;
    int err;
    switch (attr) {
    case hwmon_chip_alarms:
    err = regmap_read(data.regmap, LM83_REG_R_STATUS1, &regval);
    if (err < 0)
    return err;
// val = regval;
    err = regmap_read(data.regmap, LM83_REG_R_STATUS2, &regval);
    if (err < 0)
    return err;
// val |= regval << 8;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int lm83_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_chip:
    return lm83_chip_read(dev, attr, channel, val);
    case hwmon_temp:
    return lm83_temp_read(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int lm83_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_temp:
    return lm83_temp_write(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t lm83_is_visible(const void *_data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct lm83_data *data = _data;
//
// LM82 only supports a single external channel, modeled as channel 2.
//
    if (data.type == lm82 && (channel == 1 || channel == 3))
    return 0;
    switch (type) {
    case hwmon_chip:
    if (attr == hwmon_chip_alarms)
    return 0444;
    break;
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_max_alarm:
    case hwmon_temp_crit_alarm:
    return 0444;
    case hwmon_temp_fault:
    if (channel)
    return 0444;
    break;
    case hwmon_temp_max:
    return 0644;
    case hwmon_temp_crit:
    if (channel == 2)
    return 0644;
    return 0444;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct hwmon_channel_info * const lm83_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_ALARMS),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM | HWMON_T_FAULT,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM | HWMON_T_FAULT,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM | HWMON_T_FAULT
    ),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops lm83_hwmon_ops = {
    .is_visible = lm83_is_visible,
    .read = lm83_read,
    .write = lm83_write,
    };
    static const struct hwmon_chip_info lm83_chip_info = {
    .ops = &lm83_hwmon_ops,
    .info = lm83_info,
    };
// Return 0 if detection is successful, -ENODEV otherwise
    static int lm83_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    const char *name;
    u8 man_id, chip_id;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
// Detection
    if ((i2c_smbus_read_byte_data(client, LM83_REG_R_STATUS1) & 0xA8) ||
    (i2c_smbus_read_byte_data(client, LM83_REG_R_STATUS2) & 0x48) ||
    (i2c_smbus_read_byte_data(client, LM83_REG_R_CONFIG) & 0x41)) {
    dev_dbg(&adapter.dev, "LM83 detection failed at 0x%02x\n",
    client.addr);
    return -ENODEV;
    }
// Identification
    man_id = i2c_smbus_read_byte_data(client, LM83_REG_R_MAN_ID);
    if (man_id != 0x01)	/* National Semiconductor */
    return -ENODEV;
    chip_id = i2c_smbus_read_byte_data(client, LM83_REG_R_CHIP_ID);
    switch (chip_id) {
    case 0x03:
//
// According to the LM82 datasheet dated March 2013, recent
// revisions of LM82 have a die revision of 0x03. This was
// confirmed with a real chip. Further details in this revision
// of the LM82 datasheet strongly suggest that LM82 is just a
// repackaged LM83. It is therefore impossible to distinguish
// those chips from LM83, and they will be misdetected as LM83.
//
    name = "lm83";
    break;
    case 0x01:
    name = "lm82";
    break;
    default:
// identification failed
    dev_dbg(&adapter.dev,
    "Unsupported chip (man_id=0x%02X, chip_id=0x%02X)\n",
    man_id, chip_id);
    return -ENODEV;
    }
    strscpy(info.type, name, I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm83_probe(client: *mut i2c_client) -> c_int {
    static int lm83_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct lm83_data *data;
    data = devm_kzalloc(dev, sizeof(struct lm83_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = devm_regmap_init(dev, core::ptr::null_mut(), client, &lm83_regmap_config);
    if (IS_ERR(data.regmap))
    return PTR_ERR(data.regmap);
    data.type = (uintptr_t)i2c_get_match_data(client);
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data, &lm83_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
//
// Driver data (common to all clients)
//
    static const struct i2c_device_id lm83_id[] = {
    { .name = "lm83", .driver_data = lm83 },
    { .name = "lm82", .driver_data = lm82 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lm83_id);
    static struct i2c_driver lm83_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "lm83",
    },
    .probe		= lm83_probe,
    .id_table	= lm83_id,
    .detect		= lm83_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(lm83_driver);
    MODULE_AUTHOR("Jean Delvare <jdelvare@suse.de>");
    MODULE_DESCRIPTION("LM83 driver");
    MODULE_LICENSE("GPL");
