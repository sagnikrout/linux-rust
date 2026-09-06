//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/isl28022.c
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
// isl28022.c - driver for Renesas ISL28022 power monitor chip monitoring
//
// Copyright (c) 2023 Carsten Spieß <mail@carsten-spiess.de>
//

// ISL28022 registers
pub const ISL28022_REG_CONFIG: c_uint = 0x00;
pub const ISL28022_REG_SHUNT: c_uint = 0x01;
pub const ISL28022_REG_BUS: c_uint = 0x02;
pub const ISL28022_REG_POWER: c_uint = 0x03;
pub const ISL28022_REG_CURRENT: c_uint = 0x04;
pub const ISL28022_REG_CALIB: c_uint = 0x05;
pub const ISL28022_REG_SHUNT_THR: c_uint = 0x06;
pub const ISL28022_REG_BUS_THR: c_uint = 0x07;
pub const ISL28022_REG_INT: c_uint = 0x08;
pub const ISL28022_REG_AUX: c_uint = 0x09;

// ISL28022 config flags
// mode flags
pub const ISL28022_MODE_SHIFT: c_int = 0;
pub const ISL28022_MODE_MASK: c_uint = 0x0007;
pub const ISL28022_MODE_PWR_DOWN: c_uint = 0x0;
pub const ISL28022_MODE_TRG_S: c_uint = 0x1;
pub const ISL28022_MODE_TRG_B: c_uint = 0x2;
pub const ISL28022_MODE_TRG_SB: c_uint = 0x3;
pub const ISL28022_MODE_ADC_OFF: c_uint = 0x4;
pub const ISL28022_MODE_CONT_S: c_uint = 0x5;
pub const ISL28022_MODE_CONT_B: c_uint = 0x6;
pub const ISL28022_MODE_CONT_SB: c_uint = 0x7;
// shunt ADC settings
pub const ISL28022_SADC_SHIFT: c_int = 3;
pub const ISL28022_SADC_MASK: c_uint = 0x0078;
pub const ISL28022_BADC_SHIFT: c_int = 7;
pub const ISL28022_BADC_MASK: c_uint = 0x0780;
pub const ISL28022_ADC_12: c_uint = 0x0	/* 12 bit ADC */;
pub const ISL28022_ADC_13: c_uint = 0x1	/* 13 bit ADC */;
pub const ISL28022_ADC_14: c_uint = 0x2	/* 14 bit ADC */;
pub const ISL28022_ADC_15: c_uint = 0x3	/* 15 bit ADC */;
pub const ISL28022_ADC_15_1: c_uint = 0x8	/* 15 bit ADC, 1 sample */;
pub const ISL28022_ADC_15_2: c_uint = 0x9	/* 15 bit ADC, 2 samples */;
pub const ISL28022_ADC_15_4: c_uint = 0xA	/* 15 bit ADC, 4 samples */;
pub const ISL28022_ADC_15_8: c_uint = 0xB	/* 15 bit ADC, 8 samples */;
pub const ISL28022_ADC_15_16: c_uint = 0xC	/* 15 bit ADC, 16 samples */;
pub const ISL28022_ADC_15_32: c_uint = 0xD	/* 15 bit ADC, 32 samples */;
pub const ISL28022_ADC_15_64: c_uint = 0xE	/* 15 bit ADC, 64 samples */;
pub const ISL28022_ADC_15_128: c_uint = 0xF	/* 15 bit ADC, 128 samples */;
// shunt voltage range
pub const ISL28022_PG_SHIFT: c_int = 11;
pub const ISL28022_PG_MASK: c_uint = 0x1800;
pub const ISL28022_PG_40: c_uint = 0x0	/* +/-40 mV */;
pub const ISL28022_PG_80: c_uint = 0x1	/* +/-80 mV */;
pub const ISL28022_PG_160: c_uint = 0x2	/* +/-160 mV */;
pub const ISL28022_PG_320: c_uint = 0x3	/* +/-3200 mV */;
// bus voltage range
pub const ISL28022_BRNG_SHIFT: c_int = 13;
pub const ISL28022_BRNG_MASK: c_uint = 0x6000;
pub const ISL28022_BRNG_16: c_uint = 0x0	/* 16 V */;
pub const ISL28022_BRNG_32: c_uint = 0x1	/* 32 V */;
pub const ISL28022_BRNG_60: c_uint = 0x3	/* 60 V */;
// reset
pub const ISL28022_RESET: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl28022_data {
    pub regmap: *mut regmap,
    pub shunt: u32,
    pub gain: u32,
    pub average: u32,
}

#[no_mangle]
unsafe extern "C" fn isl28022_read_in(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int isl28022_read_in(struct device *dev, u32 attr, int channel, long *val)
    {
    struct isl28022_data *data = dev_get_drvdata(dev);
    unsigned int regval;
    int err;
    u16 sign_bit;
    switch (channel) {
    case 0:
    switch (attr) {
    case hwmon_in_input:
    err = regmap_read(data.regmap,
    ISL28022_REG_BUS, &regval);
    if (err < 0)
    return err;
// driver supports only 60V mode (BRNG 11)
// val = (long)(((u16)regval) & 0xFFFC);
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    case 1:
    switch (attr) {
    case hwmon_in_input:
    err = regmap_read(data.regmap,
    ISL28022_REG_SHUNT, &regval);
    if (err < 0)
    return err;
    switch (data.gain) {
    case 8:
    sign_bit = (regval >> 15) & 0x01;
// val = (long)((((u16)regval) & 0x7FFF) -
    (sign_bit * 32768)) / 100;
    break;
    case 4:
    sign_bit = (regval >> 14) & 0x01;
// val = (long)((((u16)regval) & 0x3FFF) -
    (sign_bit * 16384)) / 100;
    break;
    case 2:
    sign_bit = (regval >> 13) & 0x01;
// val = (long)((((u16)regval) & 0x1FFF) -
    (sign_bit * 8192)) / 100;
    break;
    case 1:
    sign_bit = (regval >> 12) & 0x01;
// val = (long)((((u16)regval) & 0x0FFF) -
    (sign_bit * 4096)) / 100;
    break;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isl28022_read_current(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int isl28022_read_current(struct device *dev, u32 attr, long *val)
    {
    struct isl28022_data *data = dev_get_drvdata(dev);
    unsigned int regval;
    int err;
    u16 sign_bit;
    switch (attr) {
    case hwmon_curr_input:
    err = regmap_read(data.regmap,
    ISL28022_REG_CURRENT, &regval);
    if (err < 0)
    return err;
    sign_bit = (regval >> 15) & 0x01;
// val = (((long)(((u16)regval) & 0x7FFF) - (sign_bit * 32768))
    1250L * (long)data.gain) / (long)data.shunt;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isl28022_read_power(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int isl28022_read_power(struct device *dev, u32 attr, long *val)
    {
    struct isl28022_data *data = dev_get_drvdata(dev);
    unsigned int regval;
    int err;
    switch (attr) {
    case hwmon_power_input:
    err = regmap_read(data.regmap,
    ISL28022_REG_POWER, &regval);
    if (err < 0)
    return err;
// val = min(div_u64(51200000ULL * data->gain * regval,
    data.shunt), LONG_MAX);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int isl28022_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_in:
    return isl28022_read_in(dev, attr, channel, val);
    case hwmon_curr:
    return isl28022_read_current(dev, attr, val);
    case hwmon_power:
    return isl28022_read_power(dev, attr, val);
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static umode_t isl28022_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_in:
    switch (attr) {
    case hwmon_in_input:
    return 0444;
    default:
    break;
    }
    break;
    case hwmon_curr:
    switch (attr) {
    case hwmon_curr_input:
    return 0444;
    default:
    break;
    }
    break;
    case hwmon_power:
    switch (attr) {
    case hwmon_power_input:
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
    static const struct hwmon_channel_info *isl28022_info[] = {
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT,	/* channel 0: bus voltage (mV) */
    HWMON_I_INPUT),	/* channel 1: shunt voltage (mV) */
    HWMON_CHANNEL_INFO(curr,
    HWMON_C_INPUT),	/* channel 1: current (mA) */
    HWMON_CHANNEL_INFO(power,
    HWMON_P_INPUT),	/* channel 1: power (µW) */
    core::ptr::null_mut()
    };
    static const struct hwmon_ops isl28022_hwmon_ops = {
    .is_visible = isl28022_is_visible,
    .read = isl28022_read,
    };
    static const struct hwmon_chip_info isl28022_chip_info = {
    .ops = &isl28022_hwmon_ops,
    .info = isl28022_info,
    };
#[no_mangle]
unsafe extern "C" fn isl28022_is_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool isl28022_is_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ISL28022_REG_CONFIG:
    case ISL28022_REG_CALIB:
    case ISL28022_REG_SHUNT_THR:
    case ISL28022_REG_BUS_THR:
    case ISL28022_REG_INT:
    case ISL28022_REG_AUX:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn isl28022_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool isl28022_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ISL28022_REG_CONFIG:
    case ISL28022_REG_SHUNT:
    case ISL28022_REG_BUS:
    case ISL28022_REG_POWER:
    case ISL28022_REG_CURRENT:
    case ISL28022_REG_INT:
    case ISL28022_REG_AUX:
    return true;
    }
    return true;
    }
    static const struct regmap_config isl28022_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = ISL28022_REG_MAX,
    .writeable_reg = isl28022_is_writeable_reg,
    .volatile_reg = isl28022_is_volatile_reg,
    .val_format_endian = REGMAP_ENDIAN_BIG,
    .cache_type = REGCACHE_MAPLE,
    .use_single_read = true,
    .use_single_write = true,
    };
#[no_mangle]
unsafe extern "C" fn shunt_voltage_show(seqf: *mut seq_file, unused: *mut c_void) -> c_int {
    static int shunt_voltage_show(struct seq_file *seqf, void *unused)
    {
    struct isl28022_data *data = seqf.private;
    unsigned int regval;
    int err;
    err = regmap_read(data.regmap,
    ISL28022_REG_SHUNT, &regval);
    if (err)
    return err;
// print shunt voltage in micro volt
    seq_printf(seqf, "%d\n", regval * 10);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(shunt_voltage);
//
// read property values and make consistency checks.
//
// following values for shunt range and resistor are allowed:
// 40 mV -> gain 1, shunt min.  800 micro ohms
// 80 mV -> gain 2, shunt min. 1600 micro ohms
// 160 mV -> gain 4, shunt min. 3200 micro ohms
// 320 mV -> gain 8, shunt min. 6400 micro ohms
//
#[no_mangle]
unsafe extern "C" fn isl28022_read_properties(dev: *mut device, data: *mut isl28022_data) -> c_int {
    static int isl28022_read_properties(struct device *dev, struct isl28022_data *data)
    {
    const char *propname;
    u32 val;
    int err;
    propname = "shunt-resistor-micro-ohms";
    if (device_property_present(dev, propname)) {
    err = device_property_read_u32(dev, propname, &val);
    if (err)
    return err;
    } else {
    val = 10000;
    }
    data.shunt = val;
    propname = "renesas,shunt-range-microvolt";
    if (device_property_present(dev, propname)) {
    err = device_property_read_u32(dev, propname, &val);
    if (err)
    return err;
    } else {
    val = 320000;
    }
    switch (val) {
    case 40000:
    data.gain = 1;
    if (data.shunt < 800)
    goto shunt_invalid;
    break;
    case 80000:
    data.gain = 2;
    if (data.shunt < 1600)
    goto shunt_invalid;
    break;
    case 160000:
    data.gain = 4;
    if (data.shunt < 3200)
    goto shunt_invalid;
    break;
    case 320000:
    data.gain = 8;
    if (data.shunt < 6400)
    goto shunt_invalid;
    break;
    default:
    return dev_err_probe(dev, -EINVAL, "%s invalid value %u\n", propname, val);
    }
    propname = "renesas,average-samples";
    if (device_property_present(dev, propname)) {
    err = device_property_read_u32(dev, propname, &val);
    if (err)
    return err;
    } else {
    val = 1;
    }
    if (val > 128 || hweight32(val) != 1)
    return dev_err_probe(dev, -EINVAL, "%s invalid value %u\n", propname, val);
    data.average = val;
    return 0;
    shunt_invalid:
    return dev_err_probe(dev, -EINVAL,
    "renesas,shunt-resistor-microvolt invalid value %d\n",
    data.shunt);
    }
//
// write configuration and calibration registers
//
// The driver supports only shunt and bus continuous ADC mode at 15bit resolution
// with averaging from 1 to 128 samples (pow of 2) on both channels.
// Shunt voltage gain 1,2,4 or 8 is allowed.
// The bus voltage range is 60V fixed.
//
#[no_mangle]
unsafe extern "C" fn isl28022_config(data: *mut isl28022_data) -> c_int {
    static int isl28022_config(struct isl28022_data *data)
    {
    int err;
    u16 config;
    u16 calib;
    config = (ISL28022_MODE_CONT_SB << ISL28022_MODE_SHIFT) |
    (ISL28022_BRNG_60 << ISL28022_BRNG_SHIFT) |
    (__ffs(data.gain) << ISL28022_PG_SHIFT) |
    ((ISL28022_ADC_15_1 + __ffs(data.average)) << ISL28022_SADC_SHIFT) |
    ((ISL28022_ADC_15_1 + __ffs(data.average)) << ISL28022_BADC_SHIFT);
    calib = data.shunt ? 0x8000 / data.gain : 0;
    err = regmap_write(data.regmap, ISL28022_REG_CONFIG, config);
    if (err < 0)
    return err;
    return regmap_write(data.regmap, ISL28022_REG_CALIB, calib);
    }
#[no_mangle]
unsafe extern "C" fn isl28022_probe(client: *mut i2c_client) -> c_int {
    static int isl28022_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct isl28022_data *data;
    int err;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_WORD_DATA))
    return -ENODEV;
    data = devm_kzalloc(dev, sizeof(struct isl28022_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    err = isl28022_read_properties(dev, data);
    if (err)
    return err;
    data.regmap = devm_regmap_init_i2c(client, &isl28022_regmap_config);
    if (IS_ERR(data.regmap))
    return PTR_ERR(data.regmap);
    err = isl28022_config(data);
    if (err)
    return err;
    debugfs_create_file("shunt_voltage", 0444, client.debugfs, data, &shunt_voltage_fops);
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data, &isl28022_chip_info, core::ptr::null_mut());
    if (IS_ERR(hwmon_dev))
    return PTR_ERR(hwmon_dev);
    return 0;
    }
    static const struct i2c_device_id isl28022_ids[] = {
    { .name = "isl28022" },
    { /* LIST END */ }
    };
    MODULE_DEVICE_TABLE(i2c, isl28022_ids);
    static const struct of_device_id __maybe_unused isl28022_of_match[] = {
    { .compatible = "renesas,isl28022"},
    { /* LIST END */ }
    };
    MODULE_DEVICE_TABLE(of, isl28022_of_match);
    static struct i2c_driver isl28022_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "isl28022",
    },
    .probe	= isl28022_probe,
    .id_table	= isl28022_ids,
    };
    module_i2c_driver(isl28022_driver);
    MODULE_AUTHOR("Carsten Spieß <mail@carsten-spiess.de>");
    MODULE_DESCRIPTION("ISL28022 driver");
    MODULE_LICENSE("GPL");
