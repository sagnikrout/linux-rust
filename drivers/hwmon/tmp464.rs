//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/tmp464.c
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
// Driver for the Texas Instruments TMP464 SMBus temperature sensor IC.
// Supported models: TMP464, TMP468
// Copyright (C) 2022 Agathe Porte <agathe.porte@nokia.com>
// Preliminary support by:
// Lionel Pouliquen <lionel.lp.pouliquen@nokia.com>
//

// Addresses to scan
    static const unsigned short normal_i2c[] = { 0x48, 0x49, 0x4a, 0x4b, I2C_CLIENT_END };

pub const MAX_CHANNELS: c_int = 9;

    static const u8 TMP464_THERM_LIMIT[MAX_CHANNELS] = {
    0x39, 0x42, 0x4A, 0x52, 0x5A, 0x62, 0x6a, 0x72, 0x7a };
    static const u8 TMP464_THERM2_LIMIT[MAX_CHANNELS] = {
    0x3A, 0x43, 0x4B, 0x53, 0x5B, 0x63, 0x6b, 0x73, 0x7b };
pub const TMP464_THERM_STATUS_REG: c_uint = 0x21;
pub const TMP464_THERM2_STATUS_REG: c_uint = 0x22;
pub const TMP464_REMOTE_OPEN_REG: c_uint = 0x23;
pub const TMP464_CONFIG_REG: c_uint = 0x30;
pub const TMP464_TEMP_HYST_REG: c_uint = 0x38;
pub const TMP464_LOCK_REG: c_uint = 0xc4;
// Identification
pub const TMP464_MANUFACTURER_ID_REG: c_uint = 0xFE;
pub const TMP464_DEVICE_ID_REG: c_uint = 0xFF;
// Flags

pub const TMP464_CONFIG_RANGE: c_uint = 0x04;

pub const TMP464_CONFIG_CONVERSION_RATE_B0: c_int = 2;
pub const TMP464_CONFIG_CONVERSION_RATE_B2: c_int = 4;

    TMP464_CONFIG_CONVERSION_RATE_B0)
pub const TMP464_UNLOCK_VAL: c_uint = 0xeb19;
pub const TMP464_LOCK_VAL: c_uint = 0x5ca6;
pub const TMP464_LOCKED: c_uint = 0x8000;
// Manufacturer / Device ID's
pub const TMP464_MANUFACTURER_ID: c_uint = 0x5449;
pub const TMP464_DEVICE_ID: c_uint = 0x1468;
pub const TMP468_DEVICE_ID: c_uint = 0x0468;
    static const struct i2c_device_id tmp464_id[] = {
    { .name = "tmp464", .driver_data = TMP464_NUM_CHANNELS },
    { .name = "tmp468", .driver_data = TMP468_NUM_CHANNELS },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tmp464_id);
    static const struct of_device_id __maybe_unused tmp464_of_match[] = {
    {
    .compatible = "ti,tmp464",
    .data = (void *)TMP464_NUM_CHANNELS
    },
    {
    .compatible = "ti,tmp468",
    .data = (void *)TMP468_NUM_CHANNELS
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, tmp464_of_match);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmp464_channel {
    pub label: *const c_char,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmp464_data {
    pub regmap: *mut regmap,
    pub channels: c_int,
    pub config_orig: i16,
    pub open_reg: u16,
    pub last_updated: c_ulong,
    pub valid: bool,
    pub update_interval: c_int,
    pub channel: [tmp464_channel; MAX_CHANNELS],
}

#[no_mangle]
unsafe extern "C" fn temp_from_reg(reg: i16) -> c_int {
    static int temp_from_reg(s16 reg)
    {
    return DIV_ROUND_CLOSEST((reg >> 3) * 625, 10);
    }
#[no_mangle]
unsafe extern "C" fn temp_to_limit_reg(temp: c_long) -> i16 {
    static s16 temp_to_limit_reg(long temp)
    {
    return DIV_ROUND_CLOSEST(temp, 500) << 6;
    }
#[no_mangle]
unsafe extern "C" fn temp_to_offset_reg(temp: c_long) -> i16 {
    static s16 temp_to_offset_reg(long temp)
    {
    return DIV_ROUND_CLOSEST(temp * 10, 625) << 3;
    }
#[no_mangle]
unsafe extern "C" fn tmp464_enable_channels(data: *mut tmp464_data) -> c_int {
    static int tmp464_enable_channels(struct tmp464_data *data)
    {
    struct regmap *regmap = data.regmap;
    let mut enable: u16 = 0;
    int i;
    for (i = 0; i < data.channels; i++)
    if (data.channel[i].enabled)
    enable |= TMP464_CONFIG_REG_REN(i);
    return regmap_update_bits(regmap, TMP464_CONFIG_REG, TMP464_CONFIG_REG_REN_MASK, enable);
    }
#[no_mangle]
unsafe extern "C" fn tmp464_chip_read(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int tmp464_chip_read(struct device *dev, u32 attr, int channel, long *val)
    {
    struct tmp464_data *data = dev_get_drvdata(dev);
    switch (attr) {
    case hwmon_chip_update_interval:
// val = data->update_interval;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn tmp464_temp_read(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int tmp464_temp_read(struct device *dev, u32 attr, int channel, long *val)
    {
    struct tmp464_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    unsigned int regs[2];
    unsigned int regval;
    u16 regvals[2];
    let mut err: c_int = 0;
    switch (attr) {
    case hwmon_temp_max_alarm:
    err = regmap_read(regmap, TMP464_THERM_STATUS_REG, &regval);
    if (err < 0)
    break;
// val = !!(regval & BIT(channel + 7));
    break;
    case hwmon_temp_crit_alarm:
    err = regmap_read(regmap, TMP464_THERM2_STATUS_REG, &regval);
    if (err < 0)
    break;
// val = !!(regval & BIT(channel + 7));
    break;
    case hwmon_temp_fault:
//
// The chip clears TMP464_REMOTE_OPEN_REG after it is read
// and only updates it after the next measurement cycle is
// complete. That means we have to cache the value internally
// for one measurement cycle and report the cached value.
//
    if (!data.valid || time_after(jiffies, data.last_updated +
    msecs_to_jiffies(data.update_interval))) {
    err = regmap_read(regmap, TMP464_REMOTE_OPEN_REG, &regval);
    if (err < 0)
    break;
    data.open_reg = regval;
    data.last_updated = jiffies;
    data.valid = true;
    }
// val = !!(data->open_reg & BIT(channel + 7));
    break;
    case hwmon_temp_max_hyst:
    regs[0] = TMP464_THERM_LIMIT[channel];
    regs[1] = TMP464_TEMP_HYST_REG;
    err = regmap_multi_reg_read(regmap, regs, regvals, 2);
    if (err < 0)
    break;
// val = temp_from_reg(regvals[0] - regvals[1]);
    break;
    case hwmon_temp_max:
    err = regmap_read(regmap, TMP464_THERM_LIMIT[channel], &regval);
    if (err < 0)
    break;
// val = temp_from_reg(regval);
    break;
    case hwmon_temp_crit_hyst:
    regs[0] = TMP464_THERM2_LIMIT[channel];
    regs[1] = TMP464_TEMP_HYST_REG;
    err = regmap_multi_reg_read(regmap, regs, regvals, 2);
    if (err < 0)
    break;
// val = temp_from_reg(regvals[0] - regvals[1]);
    break;
    case hwmon_temp_crit:
    err = regmap_read(regmap, TMP464_THERM2_LIMIT[channel], &regval);
    if (err < 0)
    break;
// val = temp_from_reg(regval);
    break;
    case hwmon_temp_offset:
    err = regmap_read(regmap, TMP464_TEMP_OFFSET_REG(channel), &regval);
    if (err < 0)
    break;
// val = temp_from_reg(regval);
    break;
    case hwmon_temp_input:
    if (!data.channel[channel].enabled) {
    err = -ENODATA;
    break;
    }
    err = regmap_read(regmap, TMP464_TEMP_REG(channel), &regval);
    if (err < 0)
    break;
// val = temp_from_reg(regval);
    break;
    case hwmon_temp_enable:
// val = data->channel[channel].enabled;
    break;
    default:
    err = -EOPNOTSUPP;
    break;
    }
    return err;
    }
    static int tmp464_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_chip:
    return tmp464_chip_read(dev, attr, channel, val);
    case hwmon_temp:
    return tmp464_temp_read(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int tmp464_read_string(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    struct tmp464_data *data = dev_get_drvdata(dev);
// str = data->channel[channel].label;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tmp464_set_convrate(data: *mut tmp464_data, interval: c_long) -> c_int {
    static int tmp464_set_convrate(struct tmp464_data *data, long interval)
    {
    int rate;
//
// For valid rates, interval in milli-seconds can be calculated as
// interval = 125 << (7 - rate);
// or
// interval = (1 << (7 - rate)) * 125;
// The rate is therefore
// rate = 7 - __fls(interval / 125);
// and the rounded rate is
// rate = 7 - __fls(interval * 4 / (125 * 3));
// Use clamp_val() to avoid overflows, and to ensure valid input
// for __fls.
//
    interval = clamp_val(interval, 125, 16000);
    rate = 7 - __fls(interval * 4 / (125 * 3));
    data.update_interval = 125 << (7 - rate);
    return regmap_update_bits(data.regmap, TMP464_CONFIG_REG,
    TMP464_CONFIG_CONVERSION_RATE_MASK,
    rate << TMP464_CONFIG_CONVERSION_RATE_B0);
    }
#[no_mangle]
unsafe extern "C" fn tmp464_chip_write(data: *mut tmp464_data, attr: u32, channel: c_int, val: c_long) -> c_int {
    static int tmp464_chip_write(struct tmp464_data *data, u32 attr, int channel, long val)
    {
    switch (attr) {
    case hwmon_chip_update_interval:
    return tmp464_set_convrate(data, val);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn tmp464_temp_write(data: *mut tmp464_data, attr: u32, channel: c_int, val: c_long) -> c_int {
    static int tmp464_temp_write(struct tmp464_data *data, u32 attr, int channel, long val)
    {
    struct regmap *regmap = data.regmap;
    unsigned int regval;
    let mut err: c_int = 0;
    switch (attr) {
    case hwmon_temp_max_hyst:
    err = regmap_read(regmap, TMP464_THERM_LIMIT[0], &regval);
    if (err < 0)
    break;
    val = clamp_val(val, -256000, 256000);	/* prevent overflow/underflow */
    val = clamp_val(temp_from_reg(regval) - val, 0, 255000);
    err = regmap_write(regmap, TMP464_TEMP_HYST_REG,
    DIV_ROUND_CLOSEST(val, 1000) << 7);
    break;
    case hwmon_temp_max:
    val = temp_to_limit_reg(clamp_val(val, -255000, 255500));
    err = regmap_write(regmap, TMP464_THERM_LIMIT[channel], val);
    break;
    case hwmon_temp_crit:
    val = temp_to_limit_reg(clamp_val(val, -255000, 255500));
    err = regmap_write(regmap, TMP464_THERM2_LIMIT[channel], val);
    break;
    case hwmon_temp_offset:
    val = temp_to_offset_reg(clamp_val(val, -128000, 127937));
    err = regmap_write(regmap, TMP464_TEMP_OFFSET_REG(channel), val);
    break;
    case hwmon_temp_enable:
    data.channel[channel].enabled = !!val;
    err = tmp464_enable_channels(data);
    break;
    default:
    err = -EOPNOTSUPP;
    break;
    }
    return err;
    }
    static int tmp464_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct tmp464_data *data = dev_get_drvdata(dev);
    int err;
    switch (type) {
    case hwmon_chip:
    err = tmp464_chip_write(data, attr, channel, val);
    break;
    case hwmon_temp:
    err = tmp464_temp_write(data, attr, channel, val);
    break;
    default:
    err = -EOPNOTSUPP;
    break;
    }
    return err;
    }
    static umode_t tmp464_is_visible(const void *_data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct tmp464_data *data = _data;
    if (channel >= data.channels)
    return 0;
    if (type == hwmon_chip) {
    if (attr == hwmon_chip_update_interval)
    return 0644;
    return 0;
    }
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_max_alarm:
    case hwmon_temp_crit_alarm:
    case hwmon_temp_crit_hyst:
    return 0444;
    case hwmon_temp_enable:
    case hwmon_temp_max:
    case hwmon_temp_crit:
    return 0644;
    case hwmon_temp_max_hyst:
    if (!channel)
    return 0644;
    return 0444;
    case hwmon_temp_label:
    if (data.channel[channel].label)
    return 0444;
    return 0;
    case hwmon_temp_fault:
    if (channel)
    return 0444;
    return 0;
    case hwmon_temp_offset:
    if (channel)
    return 0644;
    return 0;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn tmp464_restore_lock(regmap: *mut c_void) {
    static void tmp464_restore_lock(void *regmap)
    {
    regmap_write(regmap, TMP464_LOCK_REG, TMP464_LOCK_VAL);
    }
#[no_mangle]
unsafe extern "C" fn tmp464_restore_config(_data: *mut c_void) {
    static void tmp464_restore_config(void *_data)
    {
    struct tmp464_data *data = _data;
    regmap_write(data.regmap, TMP464_CONFIG_REG, data.config_orig);
    }
#[no_mangle]
unsafe extern "C" fn tmp464_init_client(dev: *mut device, data: *mut tmp464_data) -> c_int {
    static int tmp464_init_client(struct device *dev, struct tmp464_data *data)
    {
    struct regmap *regmap = data.regmap;
    unsigned int regval;
    int err;
    err = regmap_read(regmap, TMP464_LOCK_REG, &regval);
    if (err)
    return err;
    if (regval == TMP464_LOCKED) {
// Explicitly unlock chip if it is locked
    err = regmap_write(regmap, TMP464_LOCK_REG, TMP464_UNLOCK_VAL);
    if (err)
    return err;
// and lock it again when unloading the driver
    err = devm_add_action_or_reset(dev, tmp464_restore_lock, regmap);
    if (err)
    return err;
    }
    err = regmap_read(regmap, TMP464_CONFIG_REG, &regval);
    if (err)
    return err;
    data.config_orig = regval;
    err = devm_add_action_or_reset(dev, tmp464_restore_config, data);
    if (err)
    return err;
// Default to 500 ms update interval
    err = regmap_update_bits(regmap, TMP464_CONFIG_REG,
    TMP464_CONFIG_CONVERSION_RATE_MASK | TMP464_CONFIG_SHUTDOWN,
    BIT(TMP464_CONFIG_CONVERSION_RATE_B0) |
    BIT(TMP464_CONFIG_CONVERSION_RATE_B2));
    if (err)
    return err;
    data.update_interval = 500;
    return tmp464_enable_channels(data);
    }
    static int tmp464_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    char *name, *chip;
    int reg;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_WORD_DATA))
    return -ENODEV;
    reg = i2c_smbus_read_word_swapped(client, TMP464_MANUFACTURER_ID_REG);
    if (reg < 0)
    return reg;
    if (reg != TMP464_MANUFACTURER_ID)
    return -ENODEV;
// Check for "always return zero" bits
    reg = i2c_smbus_read_word_swapped(client, TMP464_THERM_STATUS_REG);
    if (reg < 0)
    return reg;
    if (reg & 0x1f)
    return -ENODEV;
    reg = i2c_smbus_read_word_swapped(client, TMP464_THERM2_STATUS_REG);
    if (reg < 0)
    return reg;
    if (reg & 0x1f)
    return -ENODEV;
    reg = i2c_smbus_read_word_swapped(client, TMP464_DEVICE_ID_REG);
    if (reg < 0)
    return reg;
    switch (reg) {
    case TMP464_DEVICE_ID:
    name = "tmp464";
    chip = "TMP464";
    break;
    case TMP468_DEVICE_ID:
    name = "tmp468";
    chip = "TMP468";
    break;
    default:
    return -ENODEV;
    }
    strscpy(info.type, name, I2C_NAME_SIZE);
    dev_info(&adapter.dev, "Detected TI %s chip at 0x%02x\n", chip, client.addr);
    return 0;
    }
    static int tmp464_probe_child_from_dt(struct device *dev,
    struct device_node *child,
    struct tmp464_data *data)
    {
    struct regmap *regmap = data.regmap;
    u32 channel;
    s32 nfactor;
    int err;
    err = of_property_read_u32(child, "reg", &channel);
    if (err) {
    dev_err(dev, "missing reg property of %pOFn\n", child);
    return err;
    }
    if (channel >= data.channels) {
    dev_err(dev, "invalid reg %d of %pOFn\n", channel, child);
    return -EINVAL;
    }
    of_property_read_string(child, "label", &data.channel[channel].label);
    data.channel[channel].enabled = of_device_is_available(child);
    err = of_property_read_s32(child, "ti,n-factor", &nfactor);
    if (err && err != -EINVAL)
    return err;
    if (!err) {
    if (channel == 0) {
    dev_err(dev, "n-factor can't be set for internal channel\n");
    return -EINVAL;
    }
    if (nfactor > 127 || nfactor < -128) {
    dev_err(dev, "n-factor for channel %d invalid (%d)\n",
    channel, nfactor);
    return -EINVAL;
    }
    err = regmap_write(regmap, TMP464_N_FACTOR_REG(channel),
    (nfactor << 8) & 0xff00);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tmp464_probe_from_dt(dev: *mut device, data: *mut tmp464_data) -> c_int {
    static int tmp464_probe_from_dt(struct device *dev, struct tmp464_data *data)
    {
    const struct device_node *np = dev.of_node;
    int err;
    for_each_child_of_node_scoped(np, child) {
    if (strcmp(child.name, "channel"))
    continue;
    err = tmp464_probe_child_from_dt(dev, child, data);
    if (err)
    return err;
    }
    return 0;
    }
    static const struct hwmon_ops tmp464_ops = {
    .is_visible = tmp464_is_visible,
    .read = tmp464_read,
    .read_string = tmp464_read_string,
    .write = tmp464_write,
    };
    static const struct hwmon_channel_info * const tmp464_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_UPDATE_INTERVAL),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MAX_HYST | HWMON_T_CRIT |
    HWMON_T_CRIT_HYST | HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM |
    HWMON_T_LABEL | HWMON_T_ENABLE,
    HWMON_T_INPUT | HWMON_T_OFFSET | HWMON_T_MAX | HWMON_T_MAX_HYST |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MAX_ALARM |
    HWMON_T_CRIT_ALARM | HWMON_T_FAULT | HWMON_T_LABEL | HWMON_T_ENABLE,
    HWMON_T_INPUT | HWMON_T_OFFSET | HWMON_T_MAX | HWMON_T_MAX_HYST |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MAX_ALARM |
    HWMON_T_CRIT_ALARM | HWMON_T_FAULT | HWMON_T_LABEL | HWMON_T_ENABLE,
    HWMON_T_INPUT | HWMON_T_OFFSET | HWMON_T_MAX | HWMON_T_MAX_HYST |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MAX_ALARM |
    HWMON_T_CRIT_ALARM | HWMON_T_FAULT | HWMON_T_LABEL | HWMON_T_ENABLE,
    HWMON_T_INPUT | HWMON_T_OFFSET | HWMON_T_MAX | HWMON_T_MAX_HYST |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MAX_ALARM |
    HWMON_T_CRIT_ALARM | HWMON_T_FAULT | HWMON_T_LABEL | HWMON_T_ENABLE,
    HWMON_T_INPUT | HWMON_T_OFFSET | HWMON_T_MAX | HWMON_T_MAX_HYST |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MAX_ALARM |
    HWMON_T_CRIT_ALARM | HWMON_T_FAULT | HWMON_T_LABEL | HWMON_T_ENABLE,
    HWMON_T_INPUT | HWMON_T_OFFSET | HWMON_T_MAX | HWMON_T_MAX_HYST |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MAX_ALARM |
    HWMON_T_CRIT_ALARM | HWMON_T_FAULT | HWMON_T_LABEL | HWMON_T_ENABLE,
    HWMON_T_INPUT | HWMON_T_OFFSET | HWMON_T_MAX | HWMON_T_MAX_HYST |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MAX_ALARM |
    HWMON_T_CRIT_ALARM | HWMON_T_FAULT | HWMON_T_LABEL | HWMON_T_ENABLE,
    HWMON_T_INPUT | HWMON_T_OFFSET | HWMON_T_MAX | HWMON_T_MAX_HYST |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MAX_ALARM |
    HWMON_T_CRIT_ALARM | HWMON_T_FAULT | HWMON_T_LABEL | HWMON_T_ENABLE),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info tmp464_chip_info = {
    .ops = &tmp464_ops,
    .info = tmp464_info,
    };
// regmap
#[no_mangle]
unsafe extern "C" fn tmp464_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool tmp464_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    return (reg < TMP464_TEMP_REG(TMP468_NUM_CHANNELS) ||
    reg == TMP464_THERM_STATUS_REG ||
    reg == TMP464_THERM2_STATUS_REG ||
    reg == TMP464_REMOTE_OPEN_REG);
    }
    static const struct regmap_config tmp464_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = TMP464_DEVICE_ID_REG,
    .volatile_reg = tmp464_is_volatile_reg,
    .val_format_endian = REGMAP_ENDIAN_BIG,
    .cache_type = REGCACHE_MAPLE,
    .use_single_read = true,
    .use_single_write = true,
    };
#[no_mangle]
unsafe extern "C" fn tmp464_probe(client: *mut i2c_client) -> c_int {
    static int tmp464_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct tmp464_data *data;
    int i, err;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_WORD_DATA)) {
    dev_err(&client.dev, "i2c functionality check failed\n");
    return -ENODEV;
    }
    data = devm_kzalloc(dev, sizeof(struct tmp464_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.channels = (int)(unsigned long)i2c_get_match_data(client);
    data.regmap = devm_regmap_init_i2c(client, &tmp464_regmap_config);
    if (IS_ERR(data.regmap))
    return PTR_ERR(data.regmap);
    for (i = 0; i < data.channels; i++)
    data.channel[i].enabled = true;
    err = tmp464_init_client(dev, data);
    if (err)
    return err;
    if (dev.of_node) {
    err = tmp464_probe_from_dt(dev, data);
    if (err)
    return err;
    }
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data, &tmp464_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct i2c_driver tmp464_driver = {
    .class = I2C_CLASS_HWMON,
    .driver = {
    .name	= "tmp464",
    .of_match_table = of_match_ptr(tmp464_of_match),
    },
    .probe = tmp464_probe,
    .id_table = tmp464_id,
    .detect = tmp464_detect,
    .address_list = normal_i2c,
    };
    module_i2c_driver(tmp464_driver);
    MODULE_AUTHOR("Agathe Porte <agathe.porte@nokia.com>");
    MODULE_DESCRIPTION("Texas Instruments TMP464 temperature sensor driver");
    MODULE_LICENSE("GPL");
