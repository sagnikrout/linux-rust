//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/jc42.c
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
// jc42.c - driver for Jedec JC42.4 compliant temperature sensors
//
// Copyright (c) 2010  Ericsson AB.
//
// Derived from lm77.c by Andras BALI <drewie@freemail.hu>.
//
// JC42.4 compliant temperature sensors are typically used on memory modules.
//

// Addresses to scan
    static const unsigned short normal_i2c[] = {
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, I2C_CLIENT_END };
// JC42 registers. All registers are 16 bit.
pub const JC42_REG_CAP: c_uint = 0x00;
pub const JC42_REG_CONFIG: c_uint = 0x01;
pub const JC42_REG_TEMP_UPPER: c_uint = 0x02;
pub const JC42_REG_TEMP_LOWER: c_uint = 0x03;
pub const JC42_REG_TEMP_CRITICAL: c_uint = 0x04;
pub const JC42_REG_TEMP: c_uint = 0x05;
pub const JC42_REG_MANID: c_uint = 0x06;
pub const JC42_REG_DEVICEID: c_uint = 0x07;
pub const JC42_REG_SMBUS: c_uint = 0x22 /* NXP and Atmel, possibly others? */;
// Status bits in temperature register

// Configuration register defines

// Capabilities

// Manufacturer IDs
pub const ADT_MANID: c_uint = 0x11d4  /* Analog Devices */;
pub const ATMEL_MANID: c_uint = 0x001f  /* Atmel */;
pub const ATMEL_MANID2: c_uint = 0x1114	/* Atmel */;
pub const MAX_MANID: c_uint = 0x004d  /* Maxim */;
pub const IDT_MANID: c_uint = 0x00b3  /* IDT */;
pub const MCP_MANID: c_uint = 0x0054  /* Microchip */;
pub const NXP_MANID: c_uint = 0x1131  /* NXP Semiconductors */;
pub const ONS_MANID: c_uint = 0x1b09  /* ON Semiconductor */;
pub const STM_MANID: c_uint = 0x104a  /* ST Microelectronics */;
pub const GT_MANID: c_uint = 0x1c68	/* Giantec */;
pub const GT_MANID2: c_uint = 0x132d	/* Giantec, 2nd mfg ID */;
pub const SI_MANID: c_uint = 0x1c85	/* Seiko Instruments */;
// SMBUS register

// Supported chips
// Analog Devices
pub const ADT7408_DEVID: c_uint = 0x0801;
pub const ADT7408_DEVID_MASK: c_uint = 0xffff;
// Atmel
pub const AT30TS00_DEVID: c_uint = 0x8201;
pub const AT30TS00_DEVID_MASK: c_uint = 0xffff;
pub const GT34TS02_DEVID: c_uint = 0x3300;
pub const GT34TS02_DEVID_MASK: c_uint = 0xff00;
pub const TS3000_DEVID: c_uint = 0x2900  /* Also matches TSE2002 */;
pub const TS3000_DEVID_MASK: c_uint = 0xff00;
pub const TS3001_DEVID: c_uint = 0x3000;
pub const TS3001_DEVID_MASK: c_uint = 0xff00;
// Maxim
pub const MAX6604_DEVID: c_uint = 0x3e00;
pub const MAX6604_DEVID_MASK: c_uint = 0xffff;
// Microchip
pub const MCP9804_DEVID: c_uint = 0x0200;
pub const MCP9804_DEVID_MASK: c_uint = 0xfffc;
pub const MCP9808_DEVID: c_uint = 0x0400;
pub const MCP9808_DEVID_MASK: c_uint = 0xfffc;
pub const MCP98242_DEVID: c_uint = 0x2000;
pub const MCP98242_DEVID_MASK: c_uint = 0xfffc;
pub const MCP98243_DEVID: c_uint = 0x2100;
pub const MCP98243_DEVID_MASK: c_uint = 0xfffc;
pub const MCP9843_DEVID: c_uint = 0x0000	/* Also matches mcp9805 */;
pub const MCP9843_DEVID_MASK: c_uint = 0xfffe;
// NXP
pub const SE97_DEVID: c_uint = 0xa200;
pub const SE97_DEVID_MASK: c_uint = 0xfffc;
pub const SE98_DEVID: c_uint = 0xa100;
pub const SE98_DEVID_MASK: c_uint = 0xfffc;
// ON Semiconductor
pub const CAT6095_DEVID: c_uint = 0x0800	/* Also matches CAT34TS02 */;
pub const CAT6095_DEVID_MASK: c_uint = 0xffe0;
pub const CAT34TS02C_DEVID: c_uint = 0x0a00;
pub const CAT34TS02C_DEVID_MASK: c_uint = 0xfff0;
// ST Microelectronics
pub const STTS424_DEVID: c_uint = 0x0101;
pub const STTS424_DEVID_MASK: c_uint = 0xffff;
pub const STTS424E_DEVID: c_uint = 0x0000;
pub const STTS424E_DEVID_MASK: c_uint = 0xfffe;
pub const STTS2002_DEVID: c_uint = 0x0300;
pub const STTS2002_DEVID_MASK: c_uint = 0xffff;
pub const STTS3000_DEVID: c_uint = 0x0200;
pub const STTS3000_DEVID_MASK: c_uint = 0xffff;
// TSE2004 compliant sensors
pub const TSE2004_DEVID: c_uint = 0x2200;
pub const TSE2004_DEVID_MASK: c_uint = 0xff00;
    static u16 jc42_hysteresis[] = { 0, 1500, 3000, 6000 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jc42_chips {
    pub manid: u16,
    pub devid: u16,
    pub devid_mask: u16,
}

    static struct jc42_chips jc42_chips[] = {
    { ADT_MANID, ADT7408_DEVID, ADT7408_DEVID_MASK },
    { ATMEL_MANID, AT30TS00_DEVID, AT30TS00_DEVID_MASK },
    { ATMEL_MANID2, TSE2004_DEVID, TSE2004_DEVID_MASK },
    { GT_MANID, TSE2004_DEVID, TSE2004_DEVID_MASK },
    { GT_MANID2, GT34TS02_DEVID, GT34TS02_DEVID_MASK },
    { IDT_MANID, TSE2004_DEVID, TSE2004_DEVID_MASK },
    { IDT_MANID, TS3000_DEVID, TS3000_DEVID_MASK },
    { IDT_MANID, TS3001_DEVID, TS3001_DEVID_MASK },
    { MAX_MANID, MAX6604_DEVID, MAX6604_DEVID_MASK },
    { MCP_MANID, MCP9804_DEVID, MCP9804_DEVID_MASK },
    { MCP_MANID, MCP9808_DEVID, MCP9808_DEVID_MASK },
    { MCP_MANID, MCP98242_DEVID, MCP98242_DEVID_MASK },
    { MCP_MANID, MCP98243_DEVID, MCP98243_DEVID_MASK },
    { MCP_MANID, TSE2004_DEVID, TSE2004_DEVID_MASK },
    { MCP_MANID, MCP9843_DEVID, MCP9843_DEVID_MASK },
    { NXP_MANID, SE97_DEVID, SE97_DEVID_MASK },
    { ONS_MANID, CAT6095_DEVID, CAT6095_DEVID_MASK },
    { ONS_MANID, CAT34TS02C_DEVID, CAT34TS02C_DEVID_MASK },
    { ONS_MANID, TSE2004_DEVID, TSE2004_DEVID_MASK },
    { ONS_MANID, TSE2004_DEVID, TSE2004_DEVID_MASK },
    { NXP_MANID, SE98_DEVID, SE98_DEVID_MASK },
    { SI_MANID,  TSE2004_DEVID, TSE2004_DEVID_MASK },
    { STM_MANID, STTS424_DEVID, STTS424_DEVID_MASK },
    { STM_MANID, STTS424E_DEVID, STTS424E_DEVID_MASK },
    { STM_MANID, STTS2002_DEVID, STTS2002_DEVID_MASK },
    { STM_MANID, TSE2004_DEVID, TSE2004_DEVID_MASK },
    { STM_MANID, STTS3000_DEVID, STTS3000_DEVID_MASK },
    };
// Each client has this additional data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jc42_data {
    pub regmap: *mut regmap,
    pub /: *mut *mut bool extended; / true if extended range supported,
    pub valid: bool,
    pub /: *mut *mut u16 orig_config; / original configuration,
    pub /: *mut *mut u16 config; / current configuration,
}

pub const JC42_TEMP_MIN: c_int = 0;
pub const JC42_TEMP_MAX: c_int = 125000;
#[no_mangle]
unsafe extern "C" fn jc42_temp_to_reg(temp: c_long, extended: bool) -> u16 {
    static u16 jc42_temp_to_reg(long temp, bool extended)
    {
    int ntemp = clamp_val(temp,
    extended ? JC42_TEMP_MIN_EXTENDED :
    JC42_TEMP_MIN, JC42_TEMP_MAX);
// convert from 0.001 to 0.0625 resolution
    return (ntemp * 2 / 125) & 0x1fff;
    }
#[no_mangle]
unsafe extern "C" fn jc42_temp_from_reg(reg: i16) -> c_int {
    static int jc42_temp_from_reg(s16 reg)
    {
    reg = sign_extend32(reg, 12);
// convert from 0.0625 to 0.001 resolution
    return reg * 125 / 2;
    }
    static int jc42_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct jc42_data *data = dev_get_drvdata(dev);
    unsigned int regval;
    int ret, temp, hyst;
    switch (attr) {
    case hwmon_temp_input:
    ret = regmap_read(data.regmap, JC42_REG_TEMP, &regval);
    if (ret)
    break;
// val = jc42_temp_from_reg(regval);
    break;
    case hwmon_temp_min:
    ret = regmap_read(data.regmap, JC42_REG_TEMP_LOWER, &regval);
    if (ret)
    break;
// val = jc42_temp_from_reg(regval);
    break;
    case hwmon_temp_max:
    ret = regmap_read(data.regmap, JC42_REG_TEMP_UPPER, &regval);
    if (ret)
    break;
// val = jc42_temp_from_reg(regval);
    break;
    case hwmon_temp_crit:
    ret = regmap_read(data.regmap, JC42_REG_TEMP_CRITICAL,
    &regval);
    if (ret)
    break;
// val = jc42_temp_from_reg(regval);
    break;
    case hwmon_temp_max_hyst:
    ret = regmap_read(data.regmap, JC42_REG_TEMP_UPPER, &regval);
    if (ret)
    break;
    temp = jc42_temp_from_reg(regval);
    hyst = jc42_hysteresis[FIELD_GET(JC42_CFG_HYST_MASK,
    data.config)];
// val = temp - hyst;
    break;
    case hwmon_temp_crit_hyst:
    ret = regmap_read(data.regmap, JC42_REG_TEMP_CRITICAL,
    &regval);
    if (ret)
    break;
    temp = jc42_temp_from_reg(regval);
    hyst = jc42_hysteresis[FIELD_GET(JC42_CFG_HYST_MASK,
    data.config)];
// val = temp - hyst;
    break;
    case hwmon_temp_min_alarm:
    ret = regmap_read(data.regmap, JC42_REG_TEMP, &regval);
    if (ret)
    break;
// val = FIELD_GET(JC42_ALARM_MIN, regval);
    break;
    case hwmon_temp_max_alarm:
    ret = regmap_read(data.regmap, JC42_REG_TEMP, &regval);
    if (ret)
    break;
// val = FIELD_GET(JC42_ALARM_MAX, regval);
    break;
    case hwmon_temp_crit_alarm:
    ret = regmap_read(data.regmap, JC42_REG_TEMP, &regval);
    if (ret)
    break;
// val = FIELD_GET(JC42_ALARM_CRIT, regval);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    return ret;
    }
    static int jc42_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct jc42_data *data = dev_get_drvdata(dev);
    unsigned int regval;
    int diff, hyst;
    int ret;
    switch (attr) {
    case hwmon_temp_min:
    ret = regmap_write(data.regmap, JC42_REG_TEMP_LOWER,
    jc42_temp_to_reg(val, data.extended));
    break;
    case hwmon_temp_max:
    ret = regmap_write(data.regmap, JC42_REG_TEMP_UPPER,
    jc42_temp_to_reg(val, data.extended));
    break;
    case hwmon_temp_crit:
    ret = regmap_write(data.regmap, JC42_REG_TEMP_CRITICAL,
    jc42_temp_to_reg(val, data.extended));
    break;
    case hwmon_temp_crit_hyst:
    ret = regmap_read(data.regmap, JC42_REG_TEMP_CRITICAL,
    &regval);
    if (ret)
    break;
//
// JC42.4 compliant chips only support four hysteresis values.
// Pick best choice and go from there.
//
    val = clamp_val(val, (data.extended ? JC42_TEMP_MIN_EXTENDED
    : JC42_TEMP_MIN) - 6000,
    JC42_TEMP_MAX);
    diff = jc42_temp_from_reg(regval) - val;
    hyst = 0;
    if (diff > 0) {
    if (diff < 2250)
    hyst = 1;	/* 1.5 degrees C */
#[no_mangle]
pub unsafe extern "C" fn if(4500: diff <) -> else {
    else if (diff < 4500)
    hyst = 2;	/* 3.0 degrees C */
    else
    hyst = 3;	/* 6.0 degrees C */
    }
    data.config = (data.config & ~JC42_CFG_HYST_MASK) |
    FIELD_PREP(JC42_CFG_HYST_MASK, hyst);
    ret = regmap_write(data.regmap, JC42_REG_CONFIG,
    data.config);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    return ret;
    }
    static umode_t jc42_is_visible(const void *_data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct jc42_data *data = _data;
    let mut config: c_uint = data.config;
    let mut mode: umode_t = 0444;
    switch (attr) {
    case hwmon_temp_min:
    case hwmon_temp_max:
    if (!(config & JC42_CFG_EVENT_LOCK))
    mode |= 0200;
    break;
    case hwmon_temp_crit:
    if (!(config & JC42_CFG_TCRIT_LOCK))
    mode |= 0200;
    break;
    case hwmon_temp_crit_hyst:
    if (!(config & (JC42_CFG_EVENT_LOCK | JC42_CFG_TCRIT_LOCK)))
    mode |= 0200;
    break;
    case hwmon_temp_input:
    case hwmon_temp_max_hyst:
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    case hwmon_temp_crit_alarm:
    break;
    default:
    mode = 0;
    break;
    }
    return mode;
    }
// Return 0 if detection is successful, -ENODEV otherwise
#[no_mangle]
unsafe extern "C" fn jc42_detect(client: *mut i2c_client, info: *mut i2c_board_info) -> c_int {
    static int jc42_detect(struct i2c_client *client, struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    int i, config, cap, manid, devid;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_WORD_DATA))
    return -ENODEV;
    cap = i2c_smbus_read_word_swapped(client, JC42_REG_CAP);
    config = i2c_smbus_read_word_swapped(client, JC42_REG_CONFIG);
    manid = i2c_smbus_read_word_swapped(client, JC42_REG_MANID);
    devid = i2c_smbus_read_word_swapped(client, JC42_REG_DEVICEID);
    if (cap < 0 || config < 0 || manid < 0 || devid < 0)
    return -ENODEV;
    if ((cap & 0xff00) || (config & 0xf820))
    return -ENODEV;
    if ((devid & TSE2004_DEVID_MASK) == TSE2004_DEVID &&
    (cap & 0x0062) != 0x0062)
    return -ENODEV;
    for (i = 0; i < ARRAY_SIZE(jc42_chips); i++) {
    struct jc42_chips *chip = &jc42_chips[i];
    if (manid == chip.manid &&
    (devid & chip.devid_mask) == chip.devid) {
    strscpy(info.type, "jc42", I2C_NAME_SIZE);
    return 0;
    }
    }
    return -ENODEV;
    }
    static const struct hwmon_channel_info * const jc42_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_REGISTER_TZ | HWMON_C_UPDATE_INTERVAL),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_CRIT | HWMON_T_MAX_HYST |
    HWMON_T_CRIT_HYST | HWMON_T_MIN_ALARM |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops jc42_hwmon_ops = {
    .is_visible = jc42_is_visible,
    .read = jc42_read,
    .write = jc42_write,
    };
    static const struct hwmon_chip_info jc42_chip_info = {
    .ops = &jc42_hwmon_ops,
    .info = jc42_info,
    };
#[no_mangle]
unsafe extern "C" fn jc42_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool jc42_readable_reg(struct device *dev, unsigned int reg)
    {
    return (reg >= JC42_REG_CAP && reg <= JC42_REG_DEVICEID) ||
    reg == JC42_REG_SMBUS;
    }
#[no_mangle]
unsafe extern "C" fn jc42_writable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool jc42_writable_reg(struct device *dev, unsigned int reg)
    {
    return (reg >= JC42_REG_CONFIG && reg <= JC42_REG_TEMP_CRITICAL) ||
    reg == JC42_REG_SMBUS;
    }
#[no_mangle]
unsafe extern "C" fn jc42_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool jc42_volatile_reg(struct device *dev, unsigned int reg)
    {
    let mut reg: return = = JC42_REG_CONFIG || reg == JC42_REG_TEMP;
    }
    static const struct regmap_config jc42_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .val_format_endian = REGMAP_ENDIAN_BIG,
    .max_register = JC42_REG_SMBUS,
    .writeable_reg = jc42_writable_reg,
    .readable_reg = jc42_readable_reg,
    .volatile_reg = jc42_volatile_reg,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn jc42_probe(client: *mut i2c_client) -> c_int {
    static int jc42_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    unsigned int config, cap;
    struct jc42_data *data;
    int ret;
    data = devm_kzalloc(dev, sizeof(struct jc42_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = devm_regmap_init_i2c(client, &jc42_regmap_config);
    if (IS_ERR(data.regmap))
    return PTR_ERR(data.regmap);
    i2c_set_clientdata(client, data);
    ret = regmap_read(data.regmap, JC42_REG_CAP, &cap);
    if (ret)
    return ret;
    data.extended = !!(cap & JC42_CAP_RANGE);
    if (device_property_read_bool(dev, "smbus-timeout-disable")) {
//
// Not all chips support this register, but from a
// quick read of various datasheets no chip appears
// incompatible with the below attempt to disable
// the timeout. And the whole thing is opt-in...
//
    ret = regmap_set_bits(data.regmap, JC42_REG_SMBUS,
    SMBUS_STMOUT);
    if (ret)
    return ret;
    }
    ret = regmap_read(data.regmap, JC42_REG_CONFIG, &config);
    if (ret)
    return ret;
    data.orig_config = config;
    if (config & JC42_CFG_SHUTDOWN) {
    config &= ~JC42_CFG_SHUTDOWN;
    regmap_write(data.regmap, JC42_REG_CONFIG, config);
    }
    data.config = config;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, "jc42",
    data, &jc42_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
#[no_mangle]
unsafe extern "C" fn jc42_remove(client: *mut i2c_client) {
    static void jc42_remove(struct i2c_client *client)
    {
    struct jc42_data *data = i2c_get_clientdata(client);
// Restore original configuration except hysteresis
    if ((data.config & ~JC42_CFG_HYST_MASK) !=
    (data.orig_config & ~JC42_CFG_HYST_MASK)) {
    int config;
    config = (data.orig_config & ~JC42_CFG_HYST_MASK)
    | (data.config & JC42_CFG_HYST_MASK);
    regmap_write(data.regmap, JC42_REG_CONFIG, config);
    }
    }

#[no_mangle]
unsafe extern "C" fn jc42_suspend(dev: *mut device) -> c_int {
    static int jc42_suspend(struct device *dev)
    {
    struct jc42_data *data = dev_get_drvdata(dev);
    data.config |= JC42_CFG_SHUTDOWN;
    regmap_write(data.regmap, JC42_REG_CONFIG, data.config);
    regcache_cache_only(data.regmap, true);
    regcache_mark_dirty(data.regmap);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jc42_resume(dev: *mut device) -> c_int {
    static int jc42_resume(struct device *dev)
    {
    struct jc42_data *data = dev_get_drvdata(dev);
    regcache_cache_only(data.regmap, false);
    data.config &= ~JC42_CFG_SHUTDOWN;
    regmap_write(data.regmap, JC42_REG_CONFIG, data.config);
// Restore cached register values to hardware
    return regcache_sync(data.regmap);
    }
    static const struct dev_pm_ops jc42_dev_pm_ops = {
    .suspend = jc42_suspend,
    .resume = jc42_resume,
    };

    static const struct i2c_device_id jc42_id[] = {
    { .name = "jc42" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, jc42_id);
    static const struct of_device_id jc42_of_ids[] = {
    { .compatible = "jedec,jc-42.4-temp", },
    { }
    };
    MODULE_DEVICE_TABLE(of, jc42_of_ids);
    static struct i2c_driver jc42_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "jc42",
    .pm = JC42_DEV_PM_OPS,
    .of_match_table = jc42_of_ids,
    },
    .probe		= jc42_probe,
    .remove		= jc42_remove,
    .id_table	= jc42_id,
    .detect		= jc42_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(jc42_driver);
    MODULE_AUTHOR("Guenter Roeck <linux@roeck-us.net>");
    MODULE_DESCRIPTION("JC42 driver");
    MODULE_LICENSE("GPL");
