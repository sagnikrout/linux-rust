//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/tvs-mpfs.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Author: Lars Randers <lranders@mail.dk>
//

pub const MPFS_TVS_CTRL: c_uint = 0x08;
pub const MPFS_TVS_OUTPUT0: c_uint = 0x24;
pub const MPFS_TVS_OUTPUT1: c_uint = 0x28;

//
// For all of these the value in millivolts is stored in 16 bits, with an upper
// sign bit and a lower 3 bits of decimal. These masks discard the sign bit and
// decimal places, because if Linux is running these voltages cannot be negative
// and so avoid having to convert to two's complement.
//

//
// The register map claims that the temperature is stored in bits 31:16, but
// application note "AN4682: PolarFire FPGA Temperature and Voltage Sensor"
// says that 31 is reserved. Temperature is in kelvin, so what's probably a
// sign bit has no value anyway.
//

pub const MPFS_TVS_INTERVAL_OFFSET: c_int = 8;
// The interval register is in increments of 32 us
pub const MPFS_TVS_INTERVAL_SCALE: c_int = 32;
// with 254 usable increments of 32 us available, 8 ms is the integer limit
pub const MPFS_TVS_INTERVAL_MAX_MS: c_int = 8;
// 273.1875 in 11.4 fixed-point notation
pub const MPFS_TVS_K_TO_C: c_uint = 0x1113;
    enum mpfs_tvs_sensors {
    SENSOR_V1P05 = 0,
    SENSOR_V1P8,
    SENSOR_V2P5,
    };
    static const char * const mpfs_tvs_voltage_labels[] = { "1P05", "1P8", "2P5" };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_tvs {
    pub regmap: *mut regmap,
}

    static int mpfs_tvs_voltage_read(struct mpfs_tvs *data, u32 attr,
    int channel, long *val)
    {
    u32 tmp, control;
    if (attr != hwmon_in_input && attr != hwmon_in_enable)
    return -EOPNOTSUPP;
    regmap_read(data.regmap, MPFS_TVS_CTRL, &control);
    switch (channel) {
    case SENSOR_V2P5:
    if (attr == hwmon_in_enable) {
// val = FIELD_GET(MPFS_TVS_CTRL_V2P5_ENABLE, control);
    break;
    }
    if (!(control & MPFS_TVS_CTRL_V2P5_VALID))
    return -ENODATA;
    regmap_read(data.regmap, MPFS_TVS_OUTPUT1, &tmp);
// val = FIELD_GET(MPFS_OUTPUT1_V2P5_MASK, tmp);
    break;
    case SENSOR_V1P8:
    if (attr == hwmon_in_enable) {
// val = FIELD_GET(MPFS_TVS_CTRL_V1P8_ENABLE, control);
    break;
    }
    if (!(control & MPFS_TVS_CTRL_V1P8_VALID))
    return -ENODATA;
    regmap_read(data.regmap, MPFS_TVS_OUTPUT0, &tmp);
// val = FIELD_GET(MPFS_OUTPUT0_V1P8_MASK, tmp);
    break;
    case SENSOR_V1P05:
    if (attr == hwmon_in_enable) {
// val = FIELD_GET(MPFS_TVS_CTRL_V1P05_ENABLE, control);
    break;
    }
    if (!(control & MPFS_TVS_CTRL_V1P05_VALID))
    return -ENODATA;
    regmap_read(data.regmap, MPFS_TVS_OUTPUT0, &tmp);
// val = FIELD_GET(MPFS_OUTPUT0_V1P05_MASK, tmp);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int mpfs_tvs_voltage_write(struct mpfs_tvs *data, u32 attr,
    int channel, long val)
    {
    u32 tmp;
    if (attr != hwmon_in_enable)
    return -EOPNOTSUPP;
    if (val > 1 || val < 0)
    return -EINVAL;
    switch (channel) {
    case SENSOR_V2P5:
    tmp = FIELD_PREP(MPFS_TVS_CTRL_V2P5_ENABLE, val);
    regmap_update_bits(data.regmap, MPFS_TVS_CTRL,
    MPFS_TVS_CTRL_V2P5_ENABLE, tmp);
    break;
    case SENSOR_V1P8:
    tmp = FIELD_PREP(MPFS_TVS_CTRL_V1P8_ENABLE, val);
    regmap_update_bits(data.regmap, MPFS_TVS_CTRL,
    MPFS_TVS_CTRL_V1P8_ENABLE, tmp);
    break;
    case SENSOR_V1P05:
    tmp = FIELD_PREP(MPFS_TVS_CTRL_V1P05_ENABLE, val);
    regmap_update_bits(data.regmap, MPFS_TVS_CTRL,
    MPFS_TVS_CTRL_V1P05_ENABLE, tmp);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_tvs_temp_read(data: *mut mpfs_tvs, attr: u32, val: *mut c_long) -> c_int {
    static int mpfs_tvs_temp_read(struct mpfs_tvs *data, u32 attr, long *val)
    {
    u32 tmp, control;
    if (attr != hwmon_temp_input && attr != hwmon_temp_enable)
    return -EOPNOTSUPP;
    regmap_read(data.regmap, MPFS_TVS_CTRL, &control);
    if (attr == hwmon_temp_enable) {
// val = FIELD_GET(MPFS_TVS_CTRL_TEMP_ENABLE, control);
    return 0;
    }
    if (!(control & MPFS_TVS_CTRL_TEMP_VALID))
    return -ENODATA;
    regmap_read(data.regmap, MPFS_TVS_OUTPUT1, &tmp);
// val = FIELD_GET(MPFS_OUTPUT1_TEMP_MASK, tmp);
// val -= MPFS_TVS_K_TO_C;
// val = (1000 * *val) >> 4; /* fixed point (11.4) to millidegrees
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_tvs_temp_write(data: *mut mpfs_tvs, attr: u32, val: c_long) -> c_int {
    static int mpfs_tvs_temp_write(struct mpfs_tvs *data, u32 attr, long val)
    {
    u32 tmp;
    if (attr != hwmon_temp_enable)
    return -EOPNOTSUPP;
    if (val > 1 || val < 0)
    return -EINVAL;
    tmp = FIELD_PREP(MPFS_TVS_CTRL_TEMP_ENABLE, val);
    regmap_update_bits(data.regmap, MPFS_TVS_CTRL,
    MPFS_TVS_CTRL_TEMP_ENABLE, tmp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_tvs_interval_read(data: *mut mpfs_tvs, attr: u32, val: *mut c_long) -> c_int {
    static int mpfs_tvs_interval_read(struct mpfs_tvs *data, u32 attr, long *val)
    {
    u32 tmp;
    if (attr != hwmon_chip_update_interval)
    return -EOPNOTSUPP;
    regmap_read(data.regmap, MPFS_TVS_CTRL, &tmp);
// val = FIELD_GET(MPFS_TVS_INTERVAL_MASK, tmp);
// val *= MPFS_TVS_INTERVAL_SCALE;
// val = roundup(*val, 1000);
// val /= 1000;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_tvs_interval_write(data: *mut mpfs_tvs, attr: u32, val: c_long) -> c_int {
    static int mpfs_tvs_interval_write(struct mpfs_tvs *data, u32 attr, long val)
    {
    let mut temp: c_long = val;
    if (attr != hwmon_chip_update_interval)
    return -EOPNOTSUPP;
    temp = clamp(temp, 0, MPFS_TVS_INTERVAL_MAX_MS);
    temp *= 1000;
    temp /= MPFS_TVS_INTERVAL_SCALE;
    temp <<= MPFS_TVS_INTERVAL_OFFSET;
    regmap_update_bits(data.regmap, MPFS_TVS_CTRL,
    MPFS_TVS_INTERVAL_MASK, temp);
    return 0;
    }
    static umode_t mpfs_tvs_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (type == hwmon_chip && attr == hwmon_chip_update_interval)
    return 0644;
    if (type == hwmon_temp) {
    switch (attr) {
    case hwmon_temp_enable:
    return 0644;
    case hwmon_temp_input:
    case hwmon_temp_label:
    return 0444;
    default:
    return 0;
    }
    }
    if (type == hwmon_in) {
    switch (attr) {
    case hwmon_in_enable:
    return 0644;
    case hwmon_in_input:
    case hwmon_in_label:
    return 0444;
    default:
    return 0;
    }
    }
    return 0;
    }
    static int mpfs_tvs_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct mpfs_tvs *data = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_temp:
    return mpfs_tvs_temp_read(data, attr, val);
    case hwmon_in:
    return mpfs_tvs_voltage_read(data, attr, channel, val);
    case hwmon_chip:
    return mpfs_tvs_interval_read(data, attr, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int mpfs_tvs_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct mpfs_tvs *data = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_temp:
    return mpfs_tvs_temp_write(data, attr, val);
    case hwmon_in:
    return mpfs_tvs_voltage_write(data, attr, channel, val);
    case hwmon_chip:
    return mpfs_tvs_interval_write(data, attr, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int mpfs_tvs_read_labels(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel,
    const char **str)
    {
    switch (type) {
    case hwmon_temp:
// str = "Die Temp";
    return 0;
    case hwmon_in:
// str = mpfs_tvs_voltage_labels[channel];
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static const struct hwmon_ops mpfs_tvs_ops = {
    .is_visible = mpfs_tvs_is_visible,
    .read_string = mpfs_tvs_read_labels,
    .read = mpfs_tvs_read,
    .write = mpfs_tvs_write,
    };
    static const struct hwmon_channel_info *mpfs_tvs_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_REGISTER_TZ | HWMON_C_UPDATE_INTERVAL),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_ENABLE),
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_LABEL | HWMON_I_ENABLE,
    HWMON_I_INPUT | HWMON_I_LABEL | HWMON_I_ENABLE,
    HWMON_I_INPUT | HWMON_I_LABEL | HWMON_I_ENABLE),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info mpfs_tvs_chip_info = {
    .ops = &mpfs_tvs_ops,
    .info = mpfs_tvs_info,
    };
#[no_mangle]
unsafe extern "C" fn mpfs_tvs_probe(pdev: *mut platform_device) -> c_int {
    static int mpfs_tvs_probe(struct platform_device *pdev)
    {
    struct device *hwmon_dev;
    struct mpfs_tvs *data;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = device_node_to_regmap(pdev.dev.parent.of_node);
    if (IS_ERR(data.regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(data.regmap),
    "Failed to find syscon regmap\n");
//
// It's an MMIO regmap with no resources, there's nothing that can fail
// and return an error
//
    regmap_write(data.regmap, MPFS_TVS_CTRL, MPFS_TVS_CTRL_ENABLE_ALL);
    hwmon_dev = devm_hwmon_device_register_with_info(&pdev.dev, "mpfs_tvs",
    data,
    &mpfs_tvs_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(hwmon_dev))
    return dev_err_probe(&pdev.dev, PTR_ERR(hwmon_dev),
    "hwmon device registration failed.\n");
    return 0;
    }
    static struct platform_driver mpfs_tvs_driver = {
    .probe = mpfs_tvs_probe,
    .driver = {
    .name = "mpfs-tvs",
    },
    };
    module_platform_driver(mpfs_tvs_driver);
    MODULE_AUTHOR("Lars Randers <lranders@mail.dk>");
    MODULE_DESCRIPTION("PolarFire SoC temperature & voltage sensor driver");
    MODULE_LICENSE("GPL");
