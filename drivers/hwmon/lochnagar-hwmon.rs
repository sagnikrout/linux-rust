//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/lochnagar-hwmon.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Lochnagar hardware monitoring features
//
// Copyright (c) 2016-2019 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// Author: Lucas Tanure <tanureal@opensource.cirrus.com>
//

pub const LN2_MAX_NSAMPLE: c_int = 1023;
pub const LN2_SAMPLE_US: c_int = 1670;
pub const LN2_CURR_UNITS: c_int = 1000;
pub const LN2_VOLT_UNITS: c_int = 1000;
pub const LN2_TEMP_UNITS: c_int = 1000;
pub const LN2_PWR_UNITS: c_int = 1000000;
    static const char * const lochnagar_chan_names[] = {
    "DBVDD1",
    "1V8 DSP",
    "1V8 CDC",
    "VDDCORE DSP",
    "AVDD 1V8",
    "SYSVDD",
    "VDDCORE CDC",
    "MICVDD",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lochnagar_hwmon {
    pub regmap: *mut regmap,
    pub power_nsamples: [c_long; ARRAY_SIZE(lochnagar_chan_names)],
}

    enum lochnagar_measure_mode {
    LN2_CURR = 0,
    LN2_VOLT,
    LN2_TEMP,
    };
//
// float_to_long - Convert ieee754 reading from hardware to an integer
//
// @data: Value read from the hardware
// @precision: Units to multiply up to eg. 1000 = milli, 1000000 = micro
//
// Return: Converted integer reading
//
// Depending on the measurement type the hardware returns an ieee754
// floating point value in either volts, amps or celsius. This function
// will convert that into an integer in a smaller unit such as micro-amps
// or milli-celsius. The hardware does not return NaN, so consideration of
// that is not required.
//
#[no_mangle]
unsafe extern "C" fn float_to_long(data: u32, precision: u32) -> c_long {
    static long float_to_long(u32 data, u32 precision)
    {
    let mut man: u64 = data & 0x007FFFFF;
    let mut exp: c_int = ((data & 0x7F800000) >> 23) - 127 - 23;
    let mut negative: bool = data & 0x80000000;
    long result;
    man = (man + (1 << 23)) * precision;
    if (fls64(man) + exp > (int)sizeof(long) * 8 - 1)
    result = LONG_MAX;
#[no_mangle]
pub unsafe extern "C" fn if(0: exp <) -> else {
    else if (exp < 0)
    result = (man + (1ull << (-exp - 1))) >> -exp;
    else
    result = man << exp;
    return negative ? -result : result;
    }
    static int do_measurement(struct regmap *regmap, int chan,
    enum lochnagar_measure_mode mode, int nsamples)
    {
    unsigned int val;
    int ret;
    chan = 1 << (chan + LOCHNAGAR2_IMON_MEASURED_CHANNELS_SHIFT);
    ret = regmap_write(regmap, LOCHNAGAR2_IMON_CTRL1,
    LOCHNAGAR2_IMON_ENA_MASK | chan | mode);
    if (ret < 0)
    return ret;
    ret = regmap_write(regmap, LOCHNAGAR2_IMON_CTRL2, nsamples);
    if (ret < 0)
    return ret;
    ret = regmap_write(regmap, LOCHNAGAR2_IMON_CTRL3,
    LOCHNAGAR2_IMON_CONFIGURE_MASK);
    if (ret < 0)
    return ret;
    ret =  regmap_read_poll_timeout(regmap, LOCHNAGAR2_IMON_CTRL3, val,
    val & LOCHNAGAR2_IMON_DONE_MASK,
    1000, 10000);
    if (ret < 0)
    return ret;
    ret = regmap_write(regmap, LOCHNAGAR2_IMON_CTRL3,
    LOCHNAGAR2_IMON_MEASURE_MASK);
    if (ret < 0)
    return ret;
//
// Actual measurement time is ~1.67mS per sample, approximate this
// with a 1.5mS per sample msleep and then poll for success up to
// ~0.17mS * 1023 (LN2_MAX_NSAMPLES). Normally for smaller values
// of nsamples the poll will complete on the first loop due to
// other latency in the system.
//
    msleep((nsamples * 3) / 2);
    ret =  regmap_read_poll_timeout(regmap, LOCHNAGAR2_IMON_CTRL3, val,
    val & LOCHNAGAR2_IMON_DONE_MASK,
    5000, 200000);
    if (ret < 0)
    return ret;
    return regmap_write(regmap, LOCHNAGAR2_IMON_CTRL3, 0);
    }
#[no_mangle]
unsafe extern "C" fn request_data(regmap: *mut regmap, chan: c_int, data: *mut u32) -> c_int {
    static int request_data(struct regmap *regmap, int chan, u32 *data)
    {
    unsigned int val;
    int ret;
    ret = regmap_write(regmap, LOCHNAGAR2_IMON_CTRL4,
    LOCHNAGAR2_IMON_DATA_REQ_MASK |
    chan << LOCHNAGAR2_IMON_CH_SEL_SHIFT);
    if (ret < 0)
    return ret;
    ret =  regmap_read_poll_timeout(regmap, LOCHNAGAR2_IMON_CTRL4, val,
    val & LOCHNAGAR2_IMON_DATA_RDY_MASK,
    1000, 10000);
    if (ret < 0)
    return ret;
    ret = regmap_read(regmap, LOCHNAGAR2_IMON_DATA1, &val);
    if (ret < 0)
    return ret;
// data = val << 16;
    ret = regmap_read(regmap, LOCHNAGAR2_IMON_DATA2, &val);
    if (ret < 0)
    return ret;
// data |= val;
    return regmap_write(regmap, LOCHNAGAR2_IMON_CTRL4, 0);
    }
    static int read_sensor(struct device *dev, int chan,
    enum lochnagar_measure_mode mode, int nsamples,
    unsigned int precision, long *val)
    {
    struct lochnagar_hwmon *priv = dev_get_drvdata(dev);
    struct regmap *regmap = priv.regmap;
    u32 data;
    int ret;
    ret = do_measurement(regmap, chan, mode, nsamples);
    if (ret < 0) {
    dev_err(dev, "Failed to perform measurement: %d\n", ret);
    return ret;
    }
    ret = request_data(regmap, chan, &data);
    if (ret < 0) {
    dev_err(dev, "Failed to read measurement: %d\n", ret);
    return ret;
    }
// val = float_to_long(data, precision);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_power(dev: *mut device, chan: c_int, val: *mut c_long) -> c_int {
    static int read_power(struct device *dev, int chan, long *val)
    {
    struct lochnagar_hwmon *priv = dev_get_drvdata(dev);
    let mut nsamples: c_int = priv.power_nsamples[chan];
    u64 power;
    int ret;
    if (!strcmp("SYSVDD", lochnagar_chan_names[chan])) {
    power = 5 * LN2_PWR_UNITS;
    } else {
    ret = read_sensor(dev, chan, LN2_VOLT, 1, LN2_PWR_UNITS, val);
    if (ret < 0)
    return ret;
    power = abs(*val);
    }
    ret = read_sensor(dev, chan, LN2_CURR, nsamples, LN2_PWR_UNITS, val);
    if (ret < 0)
    return ret;
    power *= abs(*val);
    power = DIV_ROUND_CLOSEST_ULL(power, LN2_PWR_UNITS);
    if (power > LONG_MAX)
// val = LONG_MAX;
    else
// val = power;
    return 0;
    }
    static umode_t lochnagar_is_visible(const void *drvdata,
    enum hwmon_sensor_types type,
    u32 attr, int chan)
    {
    switch (type) {
    case hwmon_in:
    if (!strcmp("SYSVDD", lochnagar_chan_names[chan]))
    return 0;
    break;
    case hwmon_power:
    if (attr == hwmon_power_average_interval)
    return 0644;
    break;
    default:
    break;
    }
    return 0444;
    }
    static int lochnagar_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int chan, long *val)
    {
    struct lochnagar_hwmon *priv = dev_get_drvdata(dev);
    int interval;
    switch (type) {
    case hwmon_in:
    return read_sensor(dev, chan, LN2_VOLT, 1, LN2_VOLT_UNITS, val);
    case hwmon_curr:
    return read_sensor(dev, chan, LN2_CURR, 1, LN2_CURR_UNITS, val);
    case hwmon_temp:
    return read_sensor(dev, chan, LN2_TEMP, 1, LN2_TEMP_UNITS, val);
    case hwmon_power:
    switch (attr) {
    case hwmon_power_average:
    return read_power(dev, chan, val);
    case hwmon_power_average_interval:
    interval = priv.power_nsamples[chan] * LN2_SAMPLE_US;
// val = DIV_ROUND_CLOSEST(interval, 1000);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    default:
    return -EOPNOTSUPP;
    }
    }
    static int lochnagar_read_string(struct device *dev,
    enum hwmon_sensor_types type, u32 attr,
    int chan, const char **str)
    {
    switch (type) {
    case hwmon_in:
    case hwmon_curr:
    case hwmon_power:
// str = lochnagar_chan_names[chan];
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int lochnagar_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int chan, long val)
    {
    struct lochnagar_hwmon *priv = dev_get_drvdata(dev);
    if (type != hwmon_power || attr != hwmon_power_average_interval)
    return -EOPNOTSUPP;
    val = clamp_t(long, val, 1, (LN2_MAX_NSAMPLE * LN2_SAMPLE_US) / 1000);
    val = DIV_ROUND_CLOSEST(val * 1000, LN2_SAMPLE_US);
    priv.power_nsamples[chan] = val;
    return 0;
    }
    static const struct hwmon_ops lochnagar_ops = {
    .is_visible = lochnagar_is_visible,
    .read = lochnagar_read,
    .read_string = lochnagar_read_string,
    .write = lochnagar_write,
    };
    static const struct hwmon_channel_info * const lochnagar_info[] = {
    HWMON_CHANNEL_INFO(temp,  HWMON_T_INPUT),
    HWMON_CHANNEL_INFO(in,    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL),
    HWMON_CHANNEL_INFO(curr,  HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL),
    HWMON_CHANNEL_INFO(power, HWMON_P_AVERAGE | HWMON_P_AVERAGE_INTERVAL |
    HWMON_P_LABEL,
    HWMON_P_AVERAGE | HWMON_P_AVERAGE_INTERVAL |
    HWMON_P_LABEL,
    HWMON_P_AVERAGE | HWMON_P_AVERAGE_INTERVAL |
    HWMON_P_LABEL,
    HWMON_P_AVERAGE | HWMON_P_AVERAGE_INTERVAL |
    HWMON_P_LABEL,
    HWMON_P_AVERAGE | HWMON_P_AVERAGE_INTERVAL |
    HWMON_P_LABEL,
    HWMON_P_AVERAGE | HWMON_P_AVERAGE_INTERVAL |
    HWMON_P_LABEL,
    HWMON_P_AVERAGE | HWMON_P_AVERAGE_INTERVAL |
    HWMON_P_LABEL,
    HWMON_P_AVERAGE | HWMON_P_AVERAGE_INTERVAL |
    HWMON_P_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info lochnagar_chip_info = {
    .ops = &lochnagar_ops,
    .info = lochnagar_info,
    };
    static const struct of_device_id lochnagar_of_match[] = {
    { .compatible = "cirrus,lochnagar2-hwmon" },
    {}
    };
    MODULE_DEVICE_TABLE(of, lochnagar_of_match);
#[no_mangle]
unsafe extern "C" fn lochnagar_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int lochnagar_hwmon_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device *hwmon_dev;
    struct lochnagar_hwmon *priv;
    int i;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!priv.regmap) {
    dev_err(dev, "No register map found\n");
    return -EINVAL;
    }
    for (i = 0; i < ARRAY_SIZE(priv.power_nsamples); i++)
    priv.power_nsamples[i] = 96;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, "Lochnagar", priv,
    &lochnagar_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct platform_driver lochnagar_hwmon_driver = {
    .driver = {
    .name = "lochnagar-hwmon",
    .of_match_table = lochnagar_of_match,
    },
    .probe = lochnagar_hwmon_probe,
    };
    module_platform_driver(lochnagar_hwmon_driver);
    MODULE_AUTHOR("Lucas Tanure <tanureal@opensource.cirrus.com>");
    MODULE_DESCRIPTION("Lochnagar hardware monitoring features");
    MODULE_LICENSE("GPL");
