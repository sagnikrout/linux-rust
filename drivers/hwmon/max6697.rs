//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/max6697.c
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
// Copyright (c) 2012 Guenter Roeck <linux@roeck-us.net>
//
// based on max1668.c
// Copyright (c) 2011 David George <david.george@ska.ac.za>
//

    enum chips { max6581, max6602, max6622, max6636, max6689, max6693, max6694,
    max6697, max6698, max6699 };
// Report local sensor as temp1
    static const u8 MAX6697_REG_TEMP[] = {
    0x07, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x08 };
    static const u8 MAX6697_REG_TEMP_EXT[] = {
    0x57, 0x09, 0x52, 0x53, 0x54, 0x55, 0x56, 0 };
    static const u8 MAX6697_REG_MAX[] = {
    0x17, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x18 };
    static const u8 MAX6697_REG_CRIT[] = {
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27 };
pub const MAX6697_REG_MIN: c_uint = 0x30;
//
// Map device tree / internal register bit map to chip bit map.
// Applies to alert register and over-temperature register.
//

// alert - local channel is in bit 6

    (((reg) & 0x01) << 6) | ((reg) & 0x80))
// over-temperature - local channel is in bit 7

    (FIELD_PREP(MAX6697_EXTERNAL_MASK_CHIP, FIELD_GET(MAX6697_EXTERNAL_MASK_DT, reg)) | \
    FIELD_PREP(MAX6697_LOCAL_MASK_CHIP, FIELD_GET(MAX6697_LOCAL_MASK_DT, reg)))
pub const MAX6697_REG_STAT_ALARM: c_uint = 0x44;
pub const MAX6697_REG_STAT_CRIT: c_uint = 0x45;
pub const MAX6697_REG_STAT_FAULT: c_uint = 0x46;
pub const MAX6697_REG_STAT_MIN_ALARM: c_uint = 0x47;
pub const MAX6697_REG_CONFIG: c_uint = 0x41;

pub const MAX6697_REG_ALERT_MASK: c_uint = 0x42;
pub const MAX6697_REG_OVERT_MASK: c_uint = 0x43;
pub const MAX6581_REG_RESISTANCE: c_uint = 0x4a;
pub const MAX6581_REG_IDEALITY: c_uint = 0x4b;
pub const MAX6581_REG_IDEALITY_SELECT: c_uint = 0x4c;
pub const MAX6581_REG_OFFSET: c_uint = 0x4d;
pub const MAX6581_REG_OFFSET_SELECT: c_uint = 0x4e;

pub const MAX6581_OFFSET_MAX: c_int = 31750;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max6697_chip_data {
    pub channels: c_int,
    pub have_ext: u32,
    pub have_crit: u32,
    pub have_fault: u32,
    pub valid_conf: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max6697_data {
    pub regmap: *mut regmap,
    pub type: enum chips,
    pub chip: *const max6697_chip_data,
    pub /: *mut *mut int temp_offset; / in degrees C,
pub const MAX6697_TEMP_INPUT: c_int = 0;
pub const MAX6697_TEMP_EXT: c_int = 1;
pub const MAX6697_TEMP_MAX: c_int = 2;
pub const MAX6697_TEMP_CRIT: c_int = 3;
    pub alarms: u32,
}

    static const struct max6697_chip_data max6697_chip_data[] = {
    [max6581] = {
    .channels = 8,
    .have_crit = 0xff,
    .have_ext = 0x7f,
    .have_fault = 0xfe,
    .valid_conf = MAX6581_CONF_EXTENDED | MAX6697_CONF_TIMEOUT,
    },
    [max6602] = {
    .channels = 5,
    .have_crit = 0x12,
    .have_ext = 0x02,
    .have_fault = 0x1e,
    .valid_conf = MAX6697_CONF_RESISTANCE | MAX6697_CONF_TIMEOUT,
    },
    [max6622] = {
    .channels = 5,
    .have_crit = 0x12,
    .have_ext = 0x02,
    .have_fault = 0x1e,
    .valid_conf = MAX6697_CONF_RESISTANCE | MAX6697_CONF_TIMEOUT,
    },
    [max6636] = {
    .channels = 7,
    .have_crit = 0x72,
    .have_ext = 0x02,
    .have_fault = 0x7e,
    .valid_conf = MAX6697_CONF_RESISTANCE | MAX6697_CONF_TIMEOUT,
    },
    [max6689] = {
    .channels = 7,
    .have_crit = 0x72,
    .have_ext = 0x02,
    .have_fault = 0x7e,
    .valid_conf = MAX6697_CONF_RESISTANCE | MAX6697_CONF_TIMEOUT,
    },
    [max6693] = {
    .channels = 7,
    .have_crit = 0x72,
    .have_ext = 0x02,
    .have_fault = 0x7e,
    .valid_conf = MAX6697_CONF_RESISTANCE | MAX6693_CONF_BETA |
    MAX6697_CONF_TIMEOUT,
    },
    [max6694] = {
    .channels = 5,
    .have_crit = 0x12,
    .have_ext = 0x02,
    .have_fault = 0x1e,
    .valid_conf = MAX6697_CONF_RESISTANCE | MAX6693_CONF_BETA |
    MAX6697_CONF_TIMEOUT,
    },
    [max6697] = {
    .channels = 7,
    .have_crit = 0x72,
    .have_ext = 0x02,
    .have_fault = 0x7e,
    .valid_conf = MAX6697_CONF_RESISTANCE | MAX6697_CONF_TIMEOUT,
    },
    [max6698] = {
    .channels = 7,
    .have_crit = 0x72,
    .have_ext = 0x02,
    .have_fault = 0x0e,
    .valid_conf = MAX6697_CONF_RESISTANCE | MAX6697_CONF_TIMEOUT,
    },
    [max6699] = {
    .channels = 5,
    .have_crit = 0x12,
    .have_ext = 0x02,
    .have_fault = 0x1e,
    .valid_conf = MAX6697_CONF_RESISTANCE | MAX6697_CONF_TIMEOUT,
    },
    };
#[no_mangle]
unsafe extern "C" fn max6697_alarm_channel_map(channel: c_int) -> c_int {
    static int max6697_alarm_channel_map(int channel)
    {
    switch (channel) {
    case 0:
    return 6;
    case 7:
    return 7;
    default:
    return channel - 1;
    }
    }
    static int max6697_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    unsigned int offset_regs[2] = { MAX6581_REG_OFFSET_SELECT, MAX6581_REG_OFFSET };
    unsigned int temp_regs[2] = { MAX6697_REG_TEMP[channel],
    MAX6697_REG_TEMP_EXT[channel] };
    struct max6697_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    u8 regdata[2] = { };
    u32 regval;
    int ret;
    switch (attr) {
    case hwmon_temp_input:
    ret = regmap_multi_reg_read(regmap, temp_regs, regdata,
    data.chip.have_ext & BIT(channel) ? 2 : 1);
    if (ret)
    return ret;
// val = (((regdata[0] - data->temp_offset) << 3) | (regdata[1] >> 5)) * 125;
    break;
    case hwmon_temp_max:
    ret = regmap_read(regmap, MAX6697_REG_MAX[channel], &regval);
    if (ret)
    return ret;
// val = ((int)regval - data->temp_offset) * 1000;
    break;
    case hwmon_temp_crit:
    ret = regmap_read(regmap, MAX6697_REG_CRIT[channel], &regval);
    if (ret)
    return ret;
// val = ((int)regval - data->temp_offset) * 1000;
    break;
    case hwmon_temp_min:
    ret = regmap_read(regmap, MAX6697_REG_MIN, &regval);
    if (ret)
    return ret;
// val = ((int)regval - data->temp_offset) * 1000;
    break;
    case hwmon_temp_offset:
    ret = regmap_multi_reg_read(regmap, offset_regs, regdata, 2);
    if (ret)
    return ret;
    if (!(regdata[0] & BIT(channel - 1)))
    regdata[1] = 0;
// val = sign_extend32(regdata[1], 7) * 250;
    break;
    case hwmon_temp_fault:
    ret = regmap_read(regmap, MAX6697_REG_STAT_FAULT, &regval);
    if (ret)
    return ret;
    if (data.type == max6581)
// val = !!(regval & BIT(channel - 1));
    else
// val = !!(regval & BIT(channel));
    break;
    case hwmon_temp_crit_alarm:
    ret = regmap_read(regmap, MAX6697_REG_STAT_CRIT, &regval);
    if (ret)
    return ret;
//
// In the MAX6581 datasheet revision 0 to 3, the local channel
// overtemperature status is reported in bit 6 of register 0x45,
// and the overtemperature status for remote channel 7 is
// reported in bit 7. In Revision 4 and later, the local channel
// overtemperature status is reported in bit 7, and the remote
// channel 7 overtemperature status is reported in bit 6. A real
// chip was found to match the functionality documented in
// Revision 4 and later.
//
// val = !!(regval & BIT(channel ? channel - 1 : 7));
    break;
    case hwmon_temp_max_alarm:
    ret = regmap_read(regmap, MAX6697_REG_STAT_ALARM, &regval);
    if (ret)
    return ret;
// val = !!(regval & BIT(max6697_alarm_channel_map(channel)));
    break;
    case hwmon_temp_min_alarm:
    ret = regmap_read(regmap, MAX6697_REG_STAT_MIN_ALARM, &regval);
    if (ret)
    return ret;
// val = !!(regval & BIT(max6697_alarm_channel_map(channel)));
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int max6697_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct max6697_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    int ret;
    switch (attr) {
    case hwmon_temp_max:
    val = clamp_val(val, -1000000, 1000000);	/* prevent underflow */
    val = DIV_ROUND_CLOSEST(val, 1000) + data.temp_offset;
    val = clamp_val(val, 0, data.type == max6581 ? 255 : 127);
    return regmap_write(regmap, MAX6697_REG_MAX[channel], val);
    case hwmon_temp_crit:
    val = clamp_val(val, -1000000, 1000000);	/* prevent underflow */
    val = DIV_ROUND_CLOSEST(val, 1000) + data.temp_offset;
    val = clamp_val(val, 0, data.type == max6581 ? 255 : 127);
    return regmap_write(regmap, MAX6697_REG_CRIT[channel], val);
    case hwmon_temp_min:
    val = clamp_val(val, -1000000, 1000000);	/* prevent underflow */
    val = DIV_ROUND_CLOSEST(val, 1000) + data.temp_offset;
    val = clamp_val(val, 0, 255);
    return regmap_write(regmap, MAX6697_REG_MIN, val);
    case hwmon_temp_offset:
    val = clamp_val(val, MAX6581_OFFSET_MIN, MAX6581_OFFSET_MAX);
    val = DIV_ROUND_CLOSEST(val, 250);
    if (!val) {	/* disable this (and only this) channel */
    ret = regmap_clear_bits(regmap, MAX6581_REG_OFFSET_SELECT,
    BIT(channel - 1));
    } else {
// enable channel and update offset
    ret = regmap_set_bits(regmap, MAX6581_REG_OFFSET_SELECT,
    BIT(channel - 1));
    if (ret)
    return ret;
    ret = regmap_write(regmap, MAX6581_REG_OFFSET, val);
    }
    return ret;
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t max6697_is_visible(const void *_data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct max6697_data *data = _data;
    const struct max6697_chip_data *chip = data.chip;
    if (channel >= chip.channels)
    return 0;
    switch (attr) {
    case hwmon_temp_max:
    return 0644;
    case hwmon_temp_input:
    case hwmon_temp_max_alarm:
    return 0444;
    case hwmon_temp_min:
    if (data.type == max6581)
    return channel ? 0444 : 0644;
    break;
    case hwmon_temp_min_alarm:
    if (data.type == max6581)
    return 0444;
    break;
    case hwmon_temp_crit:
    if (chip.have_crit & BIT(channel))
    return 0644;
    break;
    case hwmon_temp_crit_alarm:
    if (chip.have_crit & BIT(channel))
    return 0444;
    break;
    case hwmon_temp_fault:
    if (chip.have_fault & BIT(channel))
    return 0444;
    break;
    case hwmon_temp_offset:
    if (data.type == max6581 && channel)
    return 0644;
    break;
    default:
    break;
    }
    return 0;
    }
// Return 0 if detection is successful, -ENODEV otherwise
    static const struct hwmon_channel_info * const max6697_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM |
    HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_FAULT | HWMON_T_OFFSET,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM |
    HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_FAULT | HWMON_T_OFFSET,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM |
    HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_FAULT | HWMON_T_OFFSET,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM |
    HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_FAULT | HWMON_T_OFFSET,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM |
    HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_FAULT | HWMON_T_OFFSET,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM |
    HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_FAULT | HWMON_T_OFFSET,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM |
    HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_FAULT | HWMON_T_OFFSET),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops max6697_hwmon_ops = {
    .is_visible = max6697_is_visible,
    .read = max6697_read,
    .write = max6697_write,
    };
    static const struct hwmon_chip_info max6697_chip_info = {
    .ops = &max6697_hwmon_ops,
    .info = max6697_info,
    };
#[no_mangle]
unsafe extern "C" fn max6697_config_of(node: *mut device_node, data: *mut max6697_data) -> c_int {
    static int max6697_config_of(struct device_node *node, struct max6697_data *data)
    {
    const struct max6697_chip_data *chip = data.chip;
    struct regmap *regmap = data.regmap;
    int ret, confreg;
    u32 vals[2];
    confreg = 0;
    if (of_property_read_bool(node, "smbus-timeout-disable") &&
    (chip.valid_conf & MAX6697_CONF_TIMEOUT)) {
    confreg |= MAX6697_CONF_TIMEOUT;
    }
    if (of_property_read_bool(node, "extended-range-enable") &&
    (chip.valid_conf & MAX6581_CONF_EXTENDED)) {
    confreg |= MAX6581_CONF_EXTENDED;
    data.temp_offset = 64;
    }
    if (of_property_read_bool(node, "beta-compensation-enable") &&
    (chip.valid_conf & MAX6693_CONF_BETA)) {
    confreg |= MAX6693_CONF_BETA;
    }
    if (of_property_read_u32(node, "alert-mask", vals))
    vals[0] = 0;
    ret = regmap_write(regmap, MAX6697_REG_ALERT_MASK,
    MAX6697_ALERT_MAP_BITS(vals[0]));
    if (ret)
    return ret;
    if (of_property_read_u32(node, "over-temperature-mask", vals))
    vals[0] = 0;
    ret = regmap_write(regmap, MAX6697_REG_OVERT_MASK,
    MAX6697_OVERT_MAP_BITS(vals[0]));
    if (ret)
    return ret;
    if (data.type != max6581) {
    if (of_property_read_bool(node, "resistance-cancellation") &&
    chip.valid_conf & MAX6697_CONF_RESISTANCE) {
    confreg |= MAX6697_CONF_RESISTANCE;
    }
    } else {
    if (of_property_read_u32(node, "resistance-cancellation", &vals[0])) {
    if (of_property_read_bool(node, "resistance-cancellation"))
    vals[0] = 0xfe;
    else
    vals[0] = 0;
    }
    vals[0] &= 0xfe;
    ret = regmap_write(regmap, MAX6581_REG_RESISTANCE, vals[0] >> 1);
    if (ret < 0)
    return ret;
    if (of_property_read_u32_array(node, "transistor-ideality", vals, 2)) {
    vals[0] = 0;
    vals[1] = 0;
    }
    ret = regmap_write(regmap, MAX6581_REG_IDEALITY, vals[1]);
    if (ret < 0)
    return ret;
    ret = regmap_write(regmap, MAX6581_REG_IDEALITY_SELECT,
    (vals[0] & 0xfe) >> 1);
    if (ret < 0)
    return ret;
    }
    return regmap_write(regmap, MAX6697_REG_CONFIG, confreg);
    }
#[no_mangle]
unsafe extern "C" fn max6697_init_chip(np: *mut device_node, data: *mut max6697_data) -> c_int {
    static int max6697_init_chip(struct device_node *np, struct max6697_data *data)
    {
    unsigned int reg;
    int ret;
//
// Don't touch configuration if there is no devicetree configuration.
// If that is the case, use the current chip configuration.
//
    if (!np) {
    struct regmap *regmap = data.regmap;
    ret = regmap_read(regmap, MAX6697_REG_CONFIG, &reg);
    if (ret < 0)
    return ret;
    if (data.type == max6581) {
    if (reg & MAX6581_CONF_EXTENDED)
    data.temp_offset = 64;
    ret = regmap_read(regmap, MAX6581_REG_RESISTANCE, &reg);
    }
    } else {
    ret = max6697_config_of(np, data);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max6697_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max6697_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case 0x00 ... 0x09:	/* temperature high bytes */
    case 0x44 ... 0x47:	/* status */
    case 0x51 ... 0x58:	/* temperature low bytes */
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn max6697_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max6697_writeable_reg(struct device *dev, unsigned int reg)
    {
    return reg != 0x0a && reg != 0x0f && !max6697_volatile_reg(dev, reg);
    }
    static const struct regmap_config max6697_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x58,
    .writeable_reg = max6697_writeable_reg,
    .volatile_reg = max6697_volatile_reg,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn max6697_probe(client: *mut i2c_client) -> c_int {
    static int max6697_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct max6697_data *data;
    struct device *hwmon_dev;
    struct regmap *regmap;
    int err;
    regmap = devm_regmap_init_i2c(client, &max6697_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    data = devm_kzalloc(dev, sizeof(struct max6697_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = regmap;
    data.type = (uintptr_t)i2c_get_match_data(client);
    data.chip = &max6697_chip_data[data.type];
    err = max6697_init_chip(client.dev.of_node, data);
    if (err)
    return err;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name, data,
    &max6697_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id max6697_id[] = {
    { .name = "max6581", .driver_data = max6581 },
    { .name = "max6602", .driver_data = max6602 },
    { .name = "max6622", .driver_data = max6622 },
    { .name = "max6636", .driver_data = max6636 },
    { .name = "max6689", .driver_data = max6689 },
    { .name = "max6693", .driver_data = max6693 },
    { .name = "max6694", .driver_data = max6694 },
    { .name = "max6697", .driver_data = max6697 },
    { .name = "max6698", .driver_data = max6698 },
    { .name = "max6699", .driver_data = max6699 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max6697_id);
    static const struct of_device_id __maybe_unused max6697_of_match[] = {
    {
    .compatible = "maxim,max6581",
    .data = (void *)max6581
    },
    {
    .compatible = "maxim,max6602",
    .data = (void *)max6602
    },
    {
    .compatible = "maxim,max6622",
    .data = (void *)max6622
    },
    {
    .compatible = "maxim,max6636",
    .data = (void *)max6636
    },
    {
    .compatible = "maxim,max6689",
    .data = (void *)max6689
    },
    {
    .compatible = "maxim,max6693",
    .data = (void *)max6693
    },
    {
    .compatible = "maxim,max6694",
    .data = (void *)max6694
    },
    {
    .compatible = "maxim,max6697",
    .data = (void *)max6697
    },
    {
    .compatible = "maxim,max6698",
    .data = (void *)max6698
    },
    {
    .compatible = "maxim,max6699",
    .data = (void *)max6699
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, max6697_of_match);
    static struct i2c_driver max6697_driver = {
    .driver = {
    .name	= "max6697",
    .of_match_table = of_match_ptr(max6697_of_match),
    },
    .probe = max6697_probe,
    .id_table = max6697_id,
    };
    module_i2c_driver(max6697_driver);
    MODULE_AUTHOR("Guenter Roeck <linux@roeck-us.net>");
    MODULE_DESCRIPTION("MAX6697 temperature sensor driver");
    MODULE_LICENSE("GPL");
