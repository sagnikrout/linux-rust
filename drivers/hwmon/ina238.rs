//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ina238.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Driver for Texas Instruments INA238 power monitor chip
// Datasheet: https://www.ti.com/product/ina238
//
// Copyright (C) 2021 Nathan Rossi <nathan.rossi@digi.com>
//

// INA238 register definitions
pub const INA238_CONFIG: c_uint = 0x0;
pub const INA238_ADC_CONFIG: c_uint = 0x1;
pub const INA238_SHUNT_CALIBRATION: c_uint = 0x2;
pub const SQ52206_SHUNT_TEMPCO: c_uint = 0x3;
pub const INA238_SHUNT_VOLTAGE: c_uint = 0x4;
pub const INA238_BUS_VOLTAGE: c_uint = 0x5;
pub const INA238_DIE_TEMP: c_uint = 0x6;
pub const INA238_CURRENT: c_uint = 0x7;
pub const INA238_POWER: c_uint = 0x8;
pub const SQ52206_ENERGY: c_uint = 0x9;
pub const SQ52206_CHARGE: c_uint = 0xa;
pub const INA238_DIAG_ALERT: c_uint = 0xb;
pub const INA238_SHUNT_OVER_VOLTAGE: c_uint = 0xc;
pub const INA238_SHUNT_UNDER_VOLTAGE: c_uint = 0xd;
pub const INA238_BUS_OVER_VOLTAGE: c_uint = 0xe;
pub const INA238_BUS_UNDER_VOLTAGE: c_uint = 0xf;
pub const INA238_TEMP_LIMIT: c_uint = 0x10;
pub const INA238_POWER_LIMIT: c_uint = 0x11;
pub const SQ52206_POWER_PEAK: c_uint = 0x20;
pub const INA238_DEVICE_ID: c_uint = 0x3f /* not available on INA237 */;

// INA238_ADC_CONFIG register field masks and shifts

pub const INA238_ADC_CONFIG_VBUSCT_SHIFT: c_int = 9;

pub const INA238_ADC_CONFIG_VSHCT_SHIFT: c_int = 6;

pub const INA238_ADC_CONFIG_VTCT_SHIFT: c_int = 3;

pub const INA238_ADC_CONFIG_AVG_SHIFT: c_int = 0;

pub const INA238_REGISTERS: c_uint = 0x20;

// Default configuration of device on reset.
pub const INA238_CONFIG_DEFAULT: c_int = 0;
pub const SQ52206_CONFIG_DEFAULT: c_uint = 0x0005;
// 16 sample averaging, 1052us conversion time, continuous mode
pub const INA238_ADC_CONFIG_DEFAULT: c_uint = 0xfb6a;
// Configure alerts to be based on averaged value (SLOWALERT)
pub const INA238_DIAG_ALERT_DEFAULT: c_uint = 0x2000;

//
// This driver uses a fixed calibration value in order to scale current/power
// based on a fixed shunt resistor value. This allows for conversion within the
// device to avoid integer limits whilst current/power accuracy is scaled
// relative to the shunt resistor value within the driver. This is similar to
// how the ina2xx driver handles current/power scaling.
//
// To achieve the best possible dynamic range, the value of the shunt voltage
// register should match the value of the current register. With that, the shunt
// voltage of 0x7fff = 32,767 uV = 163,785 uV matches the maximum current,
// and no accuracy is lost. Experiments with a real chip show that this is
// achieved by setting the SHUNT_CAL register to a value of 0x1000 = 4,096.
// Per datasheet,
// SHUNT_CAL = 819.2 x 10^6 x CURRENT_LSB x Rshunt
// = 819,200,000 x CURRENT_LSB x Rshunt
// With SHUNT_CAL set to 4,096, we get
// CURRENT_LSB = 4,096 / (819,200,000 x Rshunt)
// Assuming an Rshunt value of 5 mOhm, we get
// CURRENT_LSB = 4,096 / (819,200,000 x 0.005) = 1mA
// and thus a dynamic range of 1mA ... 32,767mA, which is sufficient for most
// applications. The actual dynamic range is of course determined by the actual
// shunt resistor value.
//
// Power and energy values are scaled accordingly.
//
pub const INA238_CALIBRATION_VALUE: c_int = 4096;
pub const INA238_FIXED_SHUNT: c_int = 5000;

    static const struct regmap_config ina238_regmap_config = {
    .max_register = INA238_REGISTERS,
    .reg_bits = 8,
    .val_bits = 16,
    };
// Lookup table for conversion times in usec for INA238 family
    static const u16 ina238_conv_time[] = {
    50, 84, 150, 280, 540, 1052, 2074, 4120,
    };
// Lookup table for conversion times in usec for SQ52206
    static const u16 sq52206_conv_time[] = {
    66, 118, 310, 566, 1070, 2090, 4140, 8230,
    };
// Lookup table for number of samples used in averaging mode
    static const int ina238_avg_samples[] = {
    1, 4, 16, 64, 128, 256, 512, 1024,
    };
    enum ina238_ids { ina228, ina237, ina238, ina700, ina780, sq52206 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ina238_config {
    pub /: *mut *mut bool has_20bit_voltage_current; / vshunt, vbus and current are 20-bit fields,
    pub /: *mut *mut bool has_power_highest; / chip detection power peak,
    pub /: *mut *mut bool has_energy; / chip detection energy,
    pub /: *mut *mut u8 temp_resolution; / temperature register resolution in bit,
    pub /: *mut *mut u16 config_default; / Power-on default state,
    pub /: *mut *mut u32 power_calculate_factor; / fixed parameter for power calculation, from datasheet,
    pub /: *mut *mut u32 bus_voltage_lsb; / bus voltage LSB, in nV,
    pub /: *mut *mut int current_lsb; / current LSB, in uA,
    pub /: *const *const *const u16 conv_time; / conversion time lookup table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ina238_data {
    pub config: *const ina238_config,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub rshunt: u32,
    pub gain: c_int,
    pub /: *mut *mut u32 voltage_lsb[2]; / shunt, bus voltage LSB, in nV,
    pub /: *mut *mut int current_lsb; / current LSB, in uA,
    pub /: *mut *mut int power_lsb; / power LSB, in uW,
    pub /: *mut *mut int energy_lsb; / energy LSB, in uJ,
    pub /: *mut *mut u16 adc_config; / cached ADC_CONFIG register value,
}

    static const struct ina238_config ina238_config[] = {
    [ina228] = {
    .has_20bit_voltage_current = true,
    .has_energy = true,
    .has_power_highest = false,
    .power_calculate_factor = 20,
    .config_default = INA238_CONFIG_DEFAULT,
    .bus_voltage_lsb = INA238_BUS_VOLTAGE_LSB,
    .temp_resolution = 16,
    .conv_time = ina238_conv_time,
    },
    [ina237] = {
    .has_20bit_voltage_current = false,
    .has_energy = false,
    .has_power_highest = false,
    .power_calculate_factor = 20,
    .config_default = INA238_CONFIG_DEFAULT,
    .bus_voltage_lsb = INA238_BUS_VOLTAGE_LSB,
    .temp_resolution = 12,
    .conv_time = ina238_conv_time,
    },
    [ina238] = {
    .has_20bit_voltage_current = false,
    .has_energy = false,
    .has_power_highest = false,
    .power_calculate_factor = 20,
    .config_default = INA238_CONFIG_DEFAULT,
    .bus_voltage_lsb = INA238_BUS_VOLTAGE_LSB,
    .temp_resolution = 12,
    .conv_time = ina238_conv_time,
    },
    [ina700] = {
    .has_20bit_voltage_current = false,
    .has_energy = true,
    .has_power_highest = false,
    .power_calculate_factor = 20,
    .config_default = INA238_CONFIG_DEFAULT,
    .bus_voltage_lsb = INA238_BUS_VOLTAGE_LSB,
    .temp_resolution = 12,
    .current_lsb = 480,
    .conv_time = ina238_conv_time,
    },
    [ina780] = {
    .has_20bit_voltage_current = false,
    .has_energy = true,
    .has_power_highest = false,
    .power_calculate_factor = 20,
    .config_default = INA238_CONFIG_DEFAULT,
    .bus_voltage_lsb = INA238_BUS_VOLTAGE_LSB,
    .temp_resolution = 12,
    .current_lsb = 2400,
    .conv_time = ina238_conv_time,
    },
    [sq52206] = {
    .has_20bit_voltage_current = false,
    .has_energy = true,
    .has_power_highest = true,
    .power_calculate_factor = 24,
    .config_default = SQ52206_CONFIG_DEFAULT,
    .bus_voltage_lsb = SQ52206_BUS_VOLTAGE_LSB,
    .temp_resolution = 16,
    .conv_time = sq52206_conv_time,
    },
    };
#[no_mangle]
unsafe extern "C" fn ina238_read_reg24(client: *const i2c_client, reg: u8, val: *mut u32) -> c_int {
    static int ina238_read_reg24(const struct i2c_client *client, u8 reg, u32 *val)
    {
    u8 data[3];
    int err;
// 24-bit register read
    err = i2c_smbus_read_i2c_block_data(client, reg, 3, data);
    if (err < 0)
    return err;
    if (err != 3)
    return -EIO;
// val = (data[0] << 16) | (data[1] << 8) | data[2];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ina238_read_reg40(client: *const i2c_client, reg: u8, val: *mut u64) -> c_int {
    static int ina238_read_reg40(const struct i2c_client *client, u8 reg, u64 *val)
    {
    u8 data[5];
    u32 low;
    int err;
// 40-bit register read
    err = i2c_smbus_read_i2c_block_data(client, reg, 5, data);
    if (err < 0)
    return err;
    if (err != 5)
    return -EIO;
    low = (data[1] << 24) | (data[2] << 16) | (data[3] << 8) | data[4];
// val = ((long long)data[0] << 32) | low;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ina238_read_field_s20(client: *const i2c_client, reg: u8, val: *mut i32) -> c_int {
    static int ina238_read_field_s20(const struct i2c_client *client, u8 reg, s32 *val)
    {
    u32 regval;
    int err;
    err = ina238_read_reg24(client, reg, &regval);
    if (err)
    return err;
// bits 3-0 Reserved, always zero
    regval >>= 4;
// val = sign_extend32(regval, 19);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ina228_read_voltage(data: *mut ina238_data, channel: c_int, val: *mut c_long) -> c_int {
    static int ina228_read_voltage(struct ina238_data *data, int channel, long *val)
    {
    let mut reg: c_int = channel ? INA238_BUS_VOLTAGE : INA238_CURRENT;
    let mut lsb: u32 = data.voltage_lsb[channel];
    let mut factor: u32 = NUNIT_PER_MUNIT;
    int err, regval;
    if (data.config.has_20bit_voltage_current) {
    err = ina238_read_field_s20(data.client, reg, &regval);
    if (err)
    return err;
// Adjust accuracy: LSB in units of 500 pV
    lsb /= 8;
    factor *= 2;
    } else {
    err = regmap_read(data.regmap, reg, &regval);
    if (err)
    return err;
    regval = (s16)regval;
    }
// val = DIV_S64_ROUND_CLOSEST((s64)regval * lsb, factor);
    return 0;
    }
// Converting ADC_CONFIG register value to update_interval in usec
#[no_mangle]
pub unsafe extern "C" fn ina238_reg_to_interval_us(data: *mut ina238_data) -> u32 {
    static inline u32 ina238_reg_to_interval_us(struct ina238_data *data)
    {
    const u16 *ct = data.config.conv_time;
    u32 vbusct = ct[(data.adc_config & INA238_ADC_CONFIG_VBUSCT_MASK) >>
    INA238_ADC_CONFIG_VBUSCT_SHIFT];
    u32 vshct  = ct[(data.adc_config & INA238_ADC_CONFIG_VSHCT_MASK) >>
    INA238_ADC_CONFIG_VSHCT_SHIFT];
    u32 vtct   = ct[(data.adc_config & INA238_ADC_CONFIG_VTCT_MASK) >>
    INA238_ADC_CONFIG_VTCT_SHIFT];
    return vbusct + vshct + vtct;
    }
#[no_mangle]
pub unsafe extern "C" fn ina238_samples(data: *mut ina238_data) -> u32 {
    static inline u32 ina238_samples(struct ina238_data *data)
    {
    return ina238_avg_samples[(data.adc_config & INA238_ADC_CONFIG_AVG_MASK) >>
    INA238_ADC_CONFIG_AVG_SHIFT];
    }
// Converting update_interval(_us) to a per-field conversion time in usec.
// interval_us is the total ADC cycle time including averaging in microseconds.
// All three conversion fields (VBUSCT, VSHCT, VTCT) are set equal, so the
// per-field time is interval_us / (samples * 3).
//
#[no_mangle]
pub unsafe extern "C" fn ina238_interval_us_to_conv_time(interval_us: u32, samples: u32) -> u32 {
    static inline u32 ina238_interval_us_to_conv_time(u32 interval_us, u32 samples)
    {
    return DIV_ROUND_CLOSEST_ULL(interval_us, samples * 3);
    }
// Write a per-field conversion time (in usec) to the ADC_CONFIG register
#[no_mangle]
unsafe extern "C" fn ina238_write_conv_time(data: *mut ina238_data, conv_time_us: u32) -> c_int {
    static int ina238_write_conv_time(struct ina238_data *data, u32 conv_time_us)
    {
    u16 adc_config;
    int idx, ret;
    idx = find_closest(conv_time_us, data.config.conv_time,
    ARRAY_SIZE(ina238_conv_time));
    adc_config = (data.adc_config &
    ~(INA238_ADC_CONFIG_VBUSCT_MASK |
    INA238_ADC_CONFIG_VSHCT_MASK |
    INA238_ADC_CONFIG_VTCT_MASK)) |
    ((u16)idx << INA238_ADC_CONFIG_VBUSCT_SHIFT) |
    ((u16)idx << INA238_ADC_CONFIG_VSHCT_SHIFT) |
    ((u16)idx << INA238_ADC_CONFIG_VTCT_SHIFT);
    ret = regmap_write(data.regmap, INA238_ADC_CONFIG, adc_config);
    if (ret)
    return ret;
    data.adc_config = adc_config;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ina238_read_chip(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int ina238_read_chip(struct device *dev, u32 attr, long *val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    switch (attr) {
    case hwmon_chip_samples:
// val = ina238_samples(data);
    return 0;
    case hwmon_chip_update_interval:
// Return in msec
// val = DIV_ROUND_CLOSEST(ina238_reg_to_interval_us(data)
    ina238_samples(data), 1000);
    return 0;
    case hwmon_chip_update_interval_us:
// Return in usec
// val = ina238_reg_to_interval_us(data) * ina238_samples(data);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn ina238_write_chip(dev: *mut device, attr: u32, val: c_long) -> c_int {
    static int ina238_write_chip(struct device *dev, u32 attr, long val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    u16 adc_config;
    int idx, ret;
    switch (attr) {
    case hwmon_chip_samples:
    idx = find_closest(val, ina238_avg_samples,
    ARRAY_SIZE(ina238_avg_samples));
    adc_config = (data.adc_config & ~INA238_ADC_CONFIG_AVG_MASK) |
    (idx << INA238_ADC_CONFIG_AVG_SHIFT);
    ret = regmap_write(data.regmap, INA238_ADC_CONFIG, adc_config);
    if (ret)
    return ret;
    data.adc_config = adc_config;
    return 0;
    case hwmon_chip_update_interval:
// Convert ms to us before passing to the shared helper
    val = clamp_val(val, 0, INT_MAX / 1000) * 1000;
    return ina238_write_conv_time(data,
    ina238_interval_us_to_conv_time((u32)val, ina238_samples(data)));
    case hwmon_chip_update_interval_us:
    val = clamp_val(val, 0, INT_MAX);
    return ina238_write_conv_time(data,
    ina238_interval_us_to_conv_time((u32)val, ina238_samples(data)));
    default:
    return -EOPNOTSUPP;
    }
    }
    static int ina238_read_in(struct device *dev, u32 attr, int channel,
    long *val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    int reg, mask = 0;
    int regval;
    int err;
    if (attr == hwmon_in_input)
    return ina228_read_voltage(data, channel, val);
    switch (channel) {
    case 0:
    switch (attr) {
    case hwmon_in_max:
    reg = INA238_SHUNT_OVER_VOLTAGE;
    break;
    case hwmon_in_min:
    reg = INA238_SHUNT_UNDER_VOLTAGE;
    break;
    case hwmon_in_max_alarm:
    reg = INA238_DIAG_ALERT;
    mask = INA238_DIAG_ALERT_SHNTOL;
    break;
    case hwmon_in_min_alarm:
    reg = INA238_DIAG_ALERT;
    mask = INA238_DIAG_ALERT_SHNTUL;
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    case 1:
    switch (attr) {
    case hwmon_in_max:
    reg = INA238_BUS_OVER_VOLTAGE;
    break;
    case hwmon_in_min:
    reg = INA238_BUS_UNDER_VOLTAGE;
    break;
    case hwmon_in_max_alarm:
    reg = INA238_DIAG_ALERT;
    mask = INA238_DIAG_ALERT_BUSOL;
    break;
    case hwmon_in_min_alarm:
    reg = INA238_DIAG_ALERT;
    mask = INA238_DIAG_ALERT_BUSUL;
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    err = regmap_read(data.regmap, reg, &regval);
    if (err < 0)
    return err;
    if (mask)
// val = !!(regval & mask);
    else
// val = DIV_S64_ROUND_CLOSEST((s64)(s16)regval * data->voltage_lsb[channel],
    NUNIT_PER_MUNIT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ina238_write_in(dev: *mut device, attr: u32, channel: c_int, val: c_long) -> c_int {
    static int ina238_write_in(struct device *dev, u32 attr, int channel, long val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    static const int low_limits[2] = {-164, 0};
    static const int high_limits[2] = {164, 150000};
    static const u8 low_regs[2] = {INA238_SHUNT_UNDER_VOLTAGE, INA238_BUS_UNDER_VOLTAGE};
    static const u8 high_regs[2] = {INA238_SHUNT_OVER_VOLTAGE, INA238_BUS_OVER_VOLTAGE};
    int regval;
// Initial clamp to avoid overflows
    val = clamp_val(val, low_limits[channel], high_limits[channel]);
    val = DIV_S64_ROUND_CLOSEST((s64)val * NUNIT_PER_MUNIT, data.voltage_lsb[channel]);
// Final clamp to register limits
    regval = clamp_val(val, S16_MIN, S16_MAX) & 0xffff;
    switch (attr) {
    case hwmon_in_min:
    return regmap_write(data.regmap, low_regs[channel], regval);
    case hwmon_in_max:
    return regmap_write(data.regmap, high_regs[channel], regval);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn __ina238_read_curr(data: *mut ina238_data, val: *mut c_long) -> c_int {
    static int __ina238_read_curr(struct ina238_data *data, long *val)
    {
    let mut lsb: u32 = data.current_lsb;
    int err, regval;
    if (data.config.has_20bit_voltage_current) {
    err = ina238_read_field_s20(data.client, INA238_CURRENT, &regval);
    if (err)
    return err;
    lsb /= 16;	/* Adjust accuracy */
    } else {
    err = regmap_read(data.regmap, INA238_CURRENT, &regval);
    if (err)
    return err;
    regval = (s16)regval;
    }
// val = DIV_S64_ROUND_CLOSEST((s64)regval * lsb, 1000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ina238_read_curr(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int ina238_read_curr(struct device *dev, u32 attr, long *val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    int reg, mask = 0;
    int regval;
    int err;
    if (attr == hwmon_curr_input)
    return __ina238_read_curr(data, val);
    switch (attr) {
    case hwmon_curr_min:
    reg = INA238_SHUNT_UNDER_VOLTAGE;
    break;
    case hwmon_curr_min_alarm:
    reg = INA238_DIAG_ALERT;
    mask = INA238_DIAG_ALERT_SHNTUL;
    break;
    case hwmon_curr_max:
    reg = INA238_SHUNT_OVER_VOLTAGE;
    break;
    case hwmon_curr_max_alarm:
    reg = INA238_DIAG_ALERT;
    mask = INA238_DIAG_ALERT_SHNTOL;
    break;
    default:
    return -EOPNOTSUPP;
    }
    err = regmap_read(data.regmap, reg, &regval);
    if (err < 0)
    return err;
    if (mask)
// val = !!(regval & mask);
    else
// val = DIV_S64_ROUND_CLOSEST((s64)(s16)regval * data->current_lsb, 1000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ina238_write_curr(dev: *mut device, attr: u32, val: c_long) -> c_int {
    static int ina238_write_curr(struct device *dev, u32 attr, long val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    int regval;
// Set baseline range to avoid over/underflows
    val = clamp_val(val, -1000000, 1000000);
// Scale
    val = DIV_ROUND_CLOSEST(val * 1000, data.current_lsb);
// Clamp to register size
    regval = clamp_val(val, S16_MIN, S16_MAX) & 0xffff;
    switch (attr) {
    case hwmon_curr_min:
    return regmap_write(data.regmap, INA238_SHUNT_UNDER_VOLTAGE,
    regval);
    case hwmon_curr_max:
    return regmap_write(data.regmap, INA238_SHUNT_OVER_VOLTAGE,
    regval);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn ina238_read_power(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int ina238_read_power(struct device *dev, u32 attr, long *val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    long long power;
    int regval;
    int err;
    switch (attr) {
    case hwmon_power_input:
    err = ina238_read_reg24(data.client, INA238_POWER, &regval);
    if (err)
    return err;
    power = (long long)regval * data.power_lsb;
// Clamp value to maximum value of long
// val = clamp_val(power, 0, LONG_MAX);
    break;
    case hwmon_power_input_highest:
    err = ina238_read_reg24(data.client, SQ52206_POWER_PEAK, &regval);
    if (err)
    return err;
    power = (long long)regval * data.power_lsb;
// Clamp value to maximum value of long
// val = clamp_val(power, 0, LONG_MAX);
    break;
    case hwmon_power_max:
    err = regmap_read(data.regmap, INA238_POWER_LIMIT, &regval);
    if (err)
    return err;
//
// Truncated 24-bit compare register, lower 8-bits are
// truncated. Same conversion to/from uW as POWER register.
//
    power = ((long long)regval << 8) * data.power_lsb;
// Clamp value to maximum value of long
// val = clamp_val(power, 0, LONG_MAX);
    break;
    case hwmon_power_max_alarm:
    err = regmap_read(data.regmap, INA238_DIAG_ALERT, &regval);
    if (err)
    return err;
// val = !!(regval & INA238_DIAG_ALERT_POL);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ina238_write_power_max(dev: *mut device, val: c_long) -> c_int {
    static int ina238_write_power_max(struct device *dev, long val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
//
// Unsigned postive values. Compared against the 24-bit power register,
// lower 8-bits are truncated. Same conversion to/from uW as POWER
// register.
// The first clamp_val() is to establish a baseline to avoid overflows.
//
    val = clamp_val(val, 0, LONG_MAX / 2);
    val = DIV_ROUND_CLOSEST(val, data.power_lsb);
    val = clamp_val(val >> 8, 0, U16_MAX);
    return regmap_write(data.regmap, INA238_POWER_LIMIT, val);
    }
#[no_mangle]
unsafe extern "C" fn ina238_temp_from_reg(regval: i16, resolution: u8) -> c_int {
    static int ina238_temp_from_reg(s16 regval, u8 resolution)
    {
    return ((regval >> (16 - resolution)) * 1000) >> (resolution - 9);
    }
#[no_mangle]
unsafe extern "C" fn ina238_read_temp(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int ina238_read_temp(struct device *dev, u32 attr, long *val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    int regval;
    int err;
    switch (attr) {
    case hwmon_temp_input:
    err = regmap_read(data.regmap, INA238_DIE_TEMP, &regval);
    if (err)
    return err;
// val = ina238_temp_from_reg(regval, data->config->temp_resolution);
    break;
    case hwmon_temp_max:
    err = regmap_read(data.regmap, INA238_TEMP_LIMIT, &regval);
    if (err)
    return err;
// Signed, result in mC
// val = ina238_temp_from_reg(regval, data->config->temp_resolution);
    break;
    case hwmon_temp_max_alarm:
    err = regmap_read(data.regmap, INA238_DIAG_ALERT, &regval);
    if (err)
    return err;
// val = !!(regval & INA238_DIAG_ALERT_TMPOL);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ina238_temp_to_reg(val: c_long, resolution: u8) -> u16 {
    static u16 ina238_temp_to_reg(long val, u8 resolution)
    {
    let mut fraction: c_int = 1000 - DIV_ROUND_CLOSEST(1000, BIT(resolution - 9));
    val = clamp_val(val, -255000 - fraction, 255000 + fraction);
    return (DIV_ROUND_CLOSEST(val << (resolution - 9), 1000) << (16 - resolution)) & 0xffff;
    }
#[no_mangle]
unsafe extern "C" fn ina238_write_temp_max(dev: *mut device, val: c_long) -> c_int {
    static int ina238_write_temp_max(struct device *dev, long val)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    int regval;
    regval = ina238_temp_to_reg(val, data.config.temp_resolution);
    return regmap_write(data.regmap, INA238_TEMP_LIMIT, regval);
    }
#[no_mangle]
unsafe extern "C" fn ina238_read_energy(dev: *mut device, energy: *mut i64) -> c_int {
    static int ina238_read_energy(struct device *dev, s64 *energy)
    {
    struct ina238_data *data = dev_get_drvdata(dev);
    u64 regval;
    int ret;
    ret = ina238_read_reg40(data.client, SQ52206_ENERGY, &regval);
    if (ret)
    return ret;
// result in uJ
// energy = regval * data->energy_lsb;
    return 0;
    }
    static int ina238_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_chip:
    return ina238_read_chip(dev, attr, val);
    case hwmon_in:
    return ina238_read_in(dev, attr, channel, val);
    case hwmon_curr:
    return ina238_read_curr(dev, attr, val);
    case hwmon_power:
    return ina238_read_power(dev, attr, val);
    case hwmon_energy64:
    return ina238_read_energy(dev, (s64 *)val);
    case hwmon_temp:
    return ina238_read_temp(dev, attr, val);
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int ina238_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_chip:
    return ina238_write_chip(dev, attr, val);
    case hwmon_in:
    return ina238_write_in(dev, attr, channel, val);
    case hwmon_curr:
    return ina238_write_curr(dev, attr, val);
    case hwmon_power:
    return ina238_write_power_max(dev, val);
    case hwmon_temp:
    return ina238_write_temp_max(dev, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t ina238_is_visible(const void *drvdata,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct ina238_data *data = drvdata;
    let mut has_power_highest: bool = data.config.has_power_highest;
    let mut has_energy: bool = data.config.has_energy;
    switch (type) {
    case hwmon_chip:
    switch (attr) {
    case hwmon_chip_samples:
    case hwmon_chip_update_interval:
    case hwmon_chip_update_interval_us:
    return 0644;
    default:
    return 0;
    }
    case hwmon_in:
    switch (attr) {
    case hwmon_in_input:
    case hwmon_in_max_alarm:
    case hwmon_in_min_alarm:
    return 0444;
    case hwmon_in_max:
    case hwmon_in_min:
    return 0644;
    default:
    return 0;
    }
    case hwmon_curr:
    switch (attr) {
    case hwmon_curr_input:
    case hwmon_curr_max_alarm:
    case hwmon_curr_min_alarm:
    return 0444;
    case hwmon_curr_max:
    case hwmon_curr_min:
    return 0644;
    default:
    return 0;
    }
    case hwmon_power:
    switch (attr) {
    case hwmon_power_input:
    case hwmon_power_max_alarm:
    return 0444;
    case hwmon_power_max:
    return 0644;
    case hwmon_power_input_highest:
    if (has_power_highest)
    return 0444;
    return 0;
    default:
    return 0;
    }
    case hwmon_energy64:
// hwmon_energy_input
    if (has_energy)
    return 0444;
    return 0;
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_max_alarm:
    return 0444;
    case hwmon_temp_max:
    return 0644;
    default:
    return 0;
    }
    default:
    return 0;
    }
    }

    HWMON_I_MAX | HWMON_I_MAX_ALARM | \
    HWMON_I_MIN | HWMON_I_MIN_ALARM)
    static const struct hwmon_channel_info * const ina238_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_SAMPLES | HWMON_C_UPDATE_INTERVAL |
    HWMON_C_UPDATE_INTERVAL_US),
    HWMON_CHANNEL_INFO(in,
// 0: shunt voltage
    INA238_HWMON_IN_CONFIG,
// 1: bus voltage
    INA238_HWMON_IN_CONFIG),
    HWMON_CHANNEL_INFO(curr,
// 0: current through shunt
    HWMON_C_INPUT | HWMON_C_MIN | HWMON_C_MIN_ALARM |
    HWMON_C_MAX | HWMON_C_MAX_ALARM),
    HWMON_CHANNEL_INFO(power,
// 0: power
    HWMON_P_INPUT | HWMON_P_MAX |
    HWMON_P_MAX_ALARM | HWMON_P_INPUT_HIGHEST),
    HWMON_CHANNEL_INFO(energy64,
    HWMON_E_INPUT),
    HWMON_CHANNEL_INFO(temp,
// 0: die temperature
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MAX_ALARM),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops ina238_hwmon_ops = {
    .is_visible = ina238_is_visible,
    .read = ina238_read,
    .write = ina238_write,
    };
    static const struct hwmon_chip_info ina238_chip_info = {
    .ops = &ina238_hwmon_ops,
    .info = ina238_info,
    };
#[no_mangle]
unsafe extern "C" fn ina238_probe(client: *mut i2c_client) -> c_int {
    static int ina238_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct ina238_data *data;
    enum ina238_ids chip;
    int config;
    int ret;
    chip = (uintptr_t)i2c_get_match_data(client);
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
// set the device type
    data.config = &ina238_config[chip];
    data.regmap = devm_regmap_init_i2c(client, &ina238_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(dev, "failed to allocate register map\n");
    return PTR_ERR(data.regmap);
    }
// Setup CONFIG register
    config = data.config.config_default;
    if (data.config.current_lsb) {
    data.voltage_lsb[0] = INA238_SHUNT_VOLTAGE_LSB;
    data.current_lsb = data.config.current_lsb;
    } else {
// load shunt value
    if (device_property_read_u32(dev, "shunt-resistor", &data.rshunt) < 0)
    data.rshunt = INA238_RSHUNT_DEFAULT;
    if (data.rshunt == 0) {
    dev_err(dev, "invalid shunt resister value %u\n", data.rshunt);
    return -EINVAL;
    }
// load shunt gain value
    if (device_property_read_u32(dev, "ti,shunt-gain", &data.gain) < 0)
    data.gain = 4;	/* Default of ADCRANGE = 0 */
    if (data.gain != 1 && data.gain != 2 && data.gain != 4) {
    dev_err(dev, "invalid shunt gain value %u\n", data.gain);
    return -EINVAL;
    }
// Setup SHUNT_CALIBRATION register with fixed value
    ret = regmap_write(data.regmap, INA238_SHUNT_CALIBRATION,
    INA238_CALIBRATION_VALUE);
    if (ret < 0) {
    dev_err(dev, "error configuring the device: %d\n", ret);
    return -ENODEV;
    }
    if (chip == sq52206) {
    if (data.gain == 1)		/* ADCRANGE = 10/11 is /1 */
    config |= SQ52206_CONFIG_ADCRANGE_HIGH;
    else if (data.gain == 2)	/* ADCRANGE = 01 is /2 */
    config |= SQ52206_CONFIG_ADCRANGE_LOW;
    } else if (data.gain == 1) {		/* ADCRANGE = 1 is /1 */
    config |= INA238_CONFIG_ADCRANGE;
    }
    data.voltage_lsb[0] = INA238_SHUNT_VOLTAGE_LSB * data.gain / 4;
    data.current_lsb = DIV_U64_ROUND_CLOSEST(250ULL * INA238_FIXED_SHUNT * data.gain,
    data.rshunt);
    }
    ret = regmap_write(data.regmap, INA238_CONFIG, config);
    if (ret < 0) {
    dev_err(dev, "error configuring the device: %d\n", ret);
    return -ENODEV;
    }
// Setup ADC_CONFIG register
    data.adc_config = INA238_ADC_CONFIG_DEFAULT;
    ret = regmap_write(data.regmap, INA238_ADC_CONFIG, data.adc_config);
    if (ret < 0) {
    dev_err(dev, "error configuring the device: %d\n", ret);
    return -ENODEV;
    }
// Setup alert/alarm configuration
    config = INA238_DIAG_ALERT_DEFAULT;
    if (device_property_read_bool(dev, "ti,alert-polarity-active-high"))
    config |= INA238_DIAG_ALERT_APOL;
    ret = regmap_write(data.regmap, INA238_DIAG_ALERT, config);
    if (ret < 0) {
    dev_err(dev, "error configuring the device: %d\n", ret);
    return -ENODEV;
    }
    data.voltage_lsb[1] = data.config.bus_voltage_lsb;
    data.power_lsb = DIV_ROUND_CLOSEST(data.current_lsb *
    data.config.power_calculate_factor,
    100);
    data.energy_lsb = data.power_lsb * 16;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name, data,
    &ina238_chip_info, core::ptr::null_mut());
    if (IS_ERR(hwmon_dev))
    return PTR_ERR(hwmon_dev);
    if (data.rshunt)
    dev_info(dev, "power monitor %s (Rshunt = %u uOhm, gain = %u)\n",
    client.name, data.rshunt, data.gain);
    return 0;
    }
    static const struct i2c_device_id ina238_id[] = {
    { .name = "ina228", .driver_data = ina228 },
    { .name = "ina237", .driver_data = ina237 },
    { .name = "ina238", .driver_data = ina238 },
    { .name = "ina700", .driver_data = ina700 },
    { .name = "ina780", .driver_data = ina780 },
    { .name = "sq52206", .driver_data = sq52206 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ina238_id);
    static const struct of_device_id __maybe_unused ina238_of_match[] = {
    {
    .compatible = "ti,ina228",
    .data = (void *)ina228
    },
    {
    .compatible = "ti,ina237",
    .data = (void *)ina237
    },
    {
    .compatible = "ti,ina238",
    .data = (void *)ina238
    },
    {
    .compatible = "ti,ina700",
    .data = (void *)ina700
    },
    {
    .compatible = "ti,ina780",
    .data = (void *)ina780
    },
    {
    .compatible = "silergy,sq52206",
    .data = (void *)sq52206
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, ina238_of_match);
    static struct i2c_driver ina238_driver = {
    .driver = {
    .name	= "ina238",
    .of_match_table = of_match_ptr(ina238_of_match),
    },
    .probe		= ina238_probe,
    .id_table	= ina238_id,
    };
    module_i2c_driver(ina238_driver);
    MODULE_AUTHOR("Nathan Rossi <nathan.rossi@digi.com>");
    MODULE_DESCRIPTION("ina238 driver");
    MODULE_LICENSE("GPL");
