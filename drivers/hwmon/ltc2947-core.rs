//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ltc2947-core.c
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
// Analog Devices LTC2947 high precision power and energy monitor
//
// Copyright 2019 Analog Devices Inc.
//

// register's
pub const LTC2947_REG_PAGE_CTRL: c_uint = 0xFF;
pub const LTC2947_REG_CTRL: c_uint = 0xF0;
pub const LTC2947_REG_TBCTL: c_uint = 0xE9;

pub const LTC2947_REG_ACCUM_POL: c_uint = 0xE1;

pub const LTC2947_REG_ACCUM_DEADBAND: c_uint = 0xE4;
pub const LTC2947_REG_GPIOSTATCTL: c_uint = 0x67;

pub const LTC2947_REG_GPIO_ACCUM: c_uint = 0xE3;
// 200Khz
pub const LTC2947_CLK_MIN: c_int = 200000;
// 25Mhz
pub const LTC2947_CLK_MAX: c_int = 25000000;
pub const LTC2947_PAGE0: c_int = 0;
pub const LTC2947_PAGE1: c_int = 1;
// Voltage registers
pub const LTC2947_REG_VOLTAGE: c_uint = 0xA0;
pub const LTC2947_REG_VOLTAGE_MAX: c_uint = 0x50;
pub const LTC2947_REG_VOLTAGE_MIN: c_uint = 0x52;
pub const LTC2947_REG_VOLTAGE_THRE_H: c_uint = 0x90;
pub const LTC2947_REG_VOLTAGE_THRE_L: c_uint = 0x92;
pub const LTC2947_REG_DVCC: c_uint = 0xA4;
pub const LTC2947_REG_DVCC_MAX: c_uint = 0x58;
pub const LTC2947_REG_DVCC_MIN: c_uint = 0x5A;
pub const LTC2947_REG_DVCC_THRE_H: c_uint = 0x98;
pub const LTC2947_REG_DVCC_THRE_L: c_uint = 0x9A;
pub const LTC2947_VOLTAGE_GEN_CHAN: c_int = 0;
pub const LTC2947_VOLTAGE_DVCC_CHAN: c_int = 1;
// in mV
pub const VOLTAGE_MAX: c_int = 15500;

pub const VDVCC_MAX: c_int = 15000;
pub const VDVCC_MIN: c_int = 4750;
// Current registers
pub const LTC2947_REG_CURRENT: c_uint = 0x90;
pub const LTC2947_REG_CURRENT_MAX: c_uint = 0x40;
pub const LTC2947_REG_CURRENT_MIN: c_uint = 0x42;
pub const LTC2947_REG_CURRENT_THRE_H: c_uint = 0x80;
pub const LTC2947_REG_CURRENT_THRE_L: c_uint = 0x82;
// in mA
pub const CURRENT_MAX: c_int = 30000;

// Power registers
pub const LTC2947_REG_POWER: c_uint = 0x93;
pub const LTC2947_REG_POWER_MAX: c_uint = 0x44;
pub const LTC2947_REG_POWER_MIN: c_uint = 0x46;
pub const LTC2947_REG_POWER_THRE_H: c_uint = 0x84;
pub const LTC2947_REG_POWER_THRE_L: c_uint = 0x86;
// in uW
pub const POWER_MAX: c_int = 450000000;

// Temperature registers
pub const LTC2947_REG_TEMP: c_uint = 0xA2;
pub const LTC2947_REG_TEMP_MAX: c_uint = 0x54;
pub const LTC2947_REG_TEMP_MIN: c_uint = 0x56;
pub const LTC2947_REG_TEMP_THRE_H: c_uint = 0x94;
pub const LTC2947_REG_TEMP_THRE_L: c_uint = 0x96;
pub const LTC2947_REG_TEMP_FAN_THRE_H: c_uint = 0x9C;
pub const LTC2947_REG_TEMP_FAN_THRE_L: c_uint = 0x9E;
pub const LTC2947_TEMP_FAN_CHAN: c_int = 1;
// in millidegress Celsius
pub const TEMP_MAX: c_int = 85000;

// Energy registers
pub const LTC2947_REG_ENERGY1: c_uint = 0x06;
pub const LTC2947_REG_ENERGY2: c_uint = 0x16;
// Status/Alarm/Overflow registers
pub const LTC2947_REG_STATUS: c_uint = 0x80;
pub const LTC2947_REG_STATVT: c_uint = 0x81;
pub const LTC2947_REG_STATIP: c_uint = 0x82;
pub const LTC2947_REG_STATVDVCC: c_uint = 0x87;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2947_data {
    pub map: *mut regmap,
    pub dev: *mut device,
    pub lsb_energy: u32,
    pub gpio_out: bool,
}

    static int __ltc2947_val_read16(const struct ltc2947_data *st, const u8 reg,
    u64 *val)
    {
    let mut __val: __be16 = 0;
    int ret;
    ret = regmap_bulk_read(st.map, reg, &__val, 2);
    if (ret)
    return ret;
// val = be16_to_cpu(__val);
    return 0;
    }
    static int __ltc2947_val_read24(const struct ltc2947_data *st, const u8 reg,
    u64 *val)
    {
    let mut __val: __be32 = 0;
    int ret;
    ret = regmap_bulk_read(st.map, reg, &__val, 3);
    if (ret)
    return ret;
// val = be32_to_cpu(__val) >> 8;
    return 0;
    }
    static int __ltc2947_val_read64(const struct ltc2947_data *st, const u8 reg,
    u64 *val)
    {
    let mut __val: __be64 = 0;
    int ret;
    ret = regmap_bulk_read(st.map, reg, &__val, 6);
    if (ret)
    return ret;
// val = be64_to_cpu(__val) >> 16;
    return 0;
    }
    static int ltc2947_val_read(struct ltc2947_data *st, const u8 reg,
    const u8 page, const size_t size, s64 *val)
    {
    int ret;
    let mut __val: u64 = 0;
    ret = regmap_write(st.map, LTC2947_REG_PAGE_CTRL, page);
    if (ret)
    return ret;
    dev_dbg(st.dev, "Read val, reg:%02X, p:%d sz:%zu\n", reg, page,
    size);
    switch (size) {
    case 2:
    ret = __ltc2947_val_read16(st, reg, &__val);
    break;
    case 3:
    ret = __ltc2947_val_read24(st, reg, &__val);
    break;
    case 6:
    ret = __ltc2947_val_read64(st, reg, &__val);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    if (ret)
    return ret;
// val = sign_extend64(__val, (8 * size) - 1);
    dev_dbg(st.dev, "Got s:%lld, u:%016llX\n", *val, __val);
    return 0;
    }
    static int __ltc2947_val_write64(const struct ltc2947_data *st, const u8 reg,
    const u64 val)
    {
    __be64 __val;
    __val = cpu_to_be64(val << 16);
    return regmap_bulk_write(st.map, reg, &__val, 6);
    }
    static int __ltc2947_val_write16(const struct ltc2947_data *st, const u8 reg,
    const u16 val)
    {
    __be16 __val;
    __val = cpu_to_be16(val);
    return regmap_bulk_write(st.map, reg, &__val, 2);
    }
    static int ltc2947_val_write(struct ltc2947_data *st, const u8 reg,
    const u8 page, const size_t size, const u64 val)
    {
    int ret;
// set device on correct page
    ret = regmap_write(st.map, LTC2947_REG_PAGE_CTRL, page);
    if (ret)
    return ret;
    dev_dbg(st.dev, "Write val, r:%02X, p:%d, sz:%zu, val:%016llX\n",
    reg, page, size, val);
    switch (size) {
    case 2:
    ret = __ltc2947_val_write16(st, reg, val);
    break;
    case 6:
    ret = __ltc2947_val_write64(st, reg, val);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static int ltc2947_reset_history(struct ltc2947_data *st, const u8 reg_h,
    const u8 reg_l)
    {
    int ret;
//
// let's reset the tracking register's. Tracking register's have all
// 2 bytes size
//
    ret = ltc2947_val_write(st, reg_h, LTC2947_PAGE0, 2, 0x8000U);
    if (ret)
    return ret;
    return ltc2947_val_write(st, reg_l, LTC2947_PAGE0, 2, 0x7FFFU);
    }
    static int ltc2947_alarm_read(struct ltc2947_data *st, const u8 reg,
    const u32 mask, long *val)
    {
    let mut offset: u8 = reg - LTC2947_REG_STATUS;
// +1 to include status reg
    char alarms[LTC2947_ALERTS_SIZE + 1];
    let mut ret: c_int = 0;
    memset(alarms, 0, sizeof(alarms));
    ret = regmap_write(st.map, LTC2947_REG_PAGE_CTRL, LTC2947_PAGE0);
    if (ret)
    return ret;
    dev_dbg(st.dev, "Read alarm, reg:%02X, mask:%02X\n", reg, mask);
//
// As stated in the datasheet, when Threshold and Overflow registers
// are used, the status and all alert registers must be read in one
// multi-byte transaction.
//
    ret = regmap_bulk_read(st.map, LTC2947_REG_STATUS, alarms,
    sizeof(alarms));
    if (ret)
    return ret;
// get the alarm
// val = !!(alarms[offset] & mask);
    return 0;
    }
    static int ltc2947_read_temp(struct device *dev, const u32 attr, long *val,
    const int channel)
    {
    int ret;
    struct ltc2947_data *st = dev_get_drvdata(dev);
    let mut __val: i64 = 0;
    switch (attr) {
    case hwmon_temp_input:
    ret = ltc2947_val_read(st, LTC2947_REG_TEMP, LTC2947_PAGE0,
    2, &__val);
    break;
    case hwmon_temp_highest:
    ret = ltc2947_val_read(st, LTC2947_REG_TEMP_MAX, LTC2947_PAGE0,
    2, &__val);
    break;
    case hwmon_temp_lowest:
    ret = ltc2947_val_read(st, LTC2947_REG_TEMP_MIN, LTC2947_PAGE0,
    2, &__val);
    break;
    case hwmon_temp_max_alarm:
    if (channel == LTC2947_TEMP_FAN_CHAN)
    return ltc2947_alarm_read(st, LTC2947_REG_STATVT,
    LTC2947_MAX_TEMP_FAN_MASK,
    val);
    return ltc2947_alarm_read(st, LTC2947_REG_STATVT,
    LTC2947_MAX_TEMP_MASK, val);
    case hwmon_temp_min_alarm:
    if (channel == LTC2947_TEMP_FAN_CHAN)
    return	ltc2947_alarm_read(st, LTC2947_REG_STATVT,
    LTC2947_MIN_TEMP_FAN_MASK,
    val);
    return	ltc2947_alarm_read(st, LTC2947_REG_STATVT,
    LTC2947_MIN_TEMP_MASK, val);
    case hwmon_temp_max:
    if (channel == LTC2947_TEMP_FAN_CHAN)
    ret = ltc2947_val_read(st, LTC2947_REG_TEMP_FAN_THRE_H,
    LTC2947_PAGE1, 2, &__val);
    else
    ret = ltc2947_val_read(st, LTC2947_REG_TEMP_THRE_H,
    LTC2947_PAGE1, 2, &__val);
    break;
    case hwmon_temp_min:
    if (channel == LTC2947_TEMP_FAN_CHAN)
    ret = ltc2947_val_read(st, LTC2947_REG_TEMP_FAN_THRE_L,
    LTC2947_PAGE1, 2, &__val);
    else
    ret = ltc2947_val_read(st, LTC2947_REG_TEMP_THRE_L,
    LTC2947_PAGE1, 2, &__val);
    break;
    default:
    return -ENOTSUPP;
    }
    if (ret)
    return ret;
// in milidegrees celcius, temp is given by:
// val = (__val * 204) + 5500;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc2947_read_power(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int ltc2947_read_power(struct device *dev, const u32 attr, long *val)
    {
    struct ltc2947_data *st = dev_get_drvdata(dev);
    int ret;
    u32 lsb = 200000; /* in uW */
    let mut __val: i64 = 0;
    switch (attr) {
    case hwmon_power_input:
    ret = ltc2947_val_read(st, LTC2947_REG_POWER, LTC2947_PAGE0,
    3, &__val);
    lsb = 50000;
    break;
    case hwmon_power_input_highest:
    ret = ltc2947_val_read(st, LTC2947_REG_POWER_MAX, LTC2947_PAGE0,
    2, &__val);
    break;
    case hwmon_power_input_lowest:
    ret = ltc2947_val_read(st, LTC2947_REG_POWER_MIN, LTC2947_PAGE0,
    2, &__val);
    break;
    case hwmon_power_max_alarm:
    return ltc2947_alarm_read(st, LTC2947_REG_STATIP,
    LTC2947_MAX_POWER_MASK, val);
    case hwmon_power_min_alarm:
    return ltc2947_alarm_read(st, LTC2947_REG_STATIP,
    LTC2947_MIN_POWER_MASK, val);
    case hwmon_power_max:
    ret = ltc2947_val_read(st, LTC2947_REG_POWER_THRE_H,
    LTC2947_PAGE1, 2, &__val);
    break;
    case hwmon_power_min:
    ret = ltc2947_val_read(st, LTC2947_REG_POWER_THRE_L,
    LTC2947_PAGE1, 2, &__val);
    break;
    default:
    return -ENOTSUPP;
    }
    if (ret)
    return ret;
// val = __val * lsb;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc2947_read_curr(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int ltc2947_read_curr(struct device *dev, const u32 attr, long *val)
    {
    struct ltc2947_data *st = dev_get_drvdata(dev);
    int ret;
    u8 lsb = 12; /* in mA */
    let mut __val: i64 = 0;
    switch (attr) {
    case hwmon_curr_input:
    ret = ltc2947_val_read(st, LTC2947_REG_CURRENT,
    LTC2947_PAGE0, 3, &__val);
    lsb = 3;
    break;
    case hwmon_curr_highest:
    ret = ltc2947_val_read(st, LTC2947_REG_CURRENT_MAX,
    LTC2947_PAGE0, 2, &__val);
    break;
    case hwmon_curr_lowest:
    ret = ltc2947_val_read(st, LTC2947_REG_CURRENT_MIN,
    LTC2947_PAGE0, 2, &__val);
    break;
    case hwmon_curr_max_alarm:
    return ltc2947_alarm_read(st, LTC2947_REG_STATIP,
    LTC2947_MAX_CURRENT_MASK, val);
    case hwmon_curr_min_alarm:
    return ltc2947_alarm_read(st, LTC2947_REG_STATIP,
    LTC2947_MIN_CURRENT_MASK, val);
    case hwmon_curr_max:
    ret = ltc2947_val_read(st, LTC2947_REG_CURRENT_THRE_H,
    LTC2947_PAGE1, 2, &__val);
    break;
    case hwmon_curr_min:
    ret = ltc2947_val_read(st, LTC2947_REG_CURRENT_THRE_L,
    LTC2947_PAGE1, 2, &__val);
    break;
    default:
    return -ENOTSUPP;
    }
    if (ret)
    return ret;
// val = __val * lsb;
    return 0;
    }
    static int ltc2947_read_in(struct device *dev, const u32 attr, long *val,
    const int channel)
    {
    struct ltc2947_data *st = dev_get_drvdata(dev);
    int ret;
    u8 lsb = 2; /* in mV */
    let mut __val: i64 = 0;
    if (channel < 0 || channel > LTC2947_VOLTAGE_DVCC_CHAN) {
    dev_err(st.dev, "Invalid chan%d for voltage", channel);
    return -EINVAL;
    }
    switch (attr) {
    case hwmon_in_input:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN) {
    ret = ltc2947_val_read(st, LTC2947_REG_DVCC,
    LTC2947_PAGE0, 2, &__val);
    lsb = 145;
    } else {
    ret = ltc2947_val_read(st, LTC2947_REG_VOLTAGE,
    LTC2947_PAGE0, 2, &__val);
    }
    break;
    case hwmon_in_highest:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN) {
    ret = ltc2947_val_read(st, LTC2947_REG_DVCC_MAX,
    LTC2947_PAGE0, 2, &__val);
    lsb = 145;
    } else {
    ret = ltc2947_val_read(st, LTC2947_REG_VOLTAGE_MAX,
    LTC2947_PAGE0, 2, &__val);
    }
    break;
    case hwmon_in_lowest:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN) {
    ret = ltc2947_val_read(st, LTC2947_REG_DVCC_MIN,
    LTC2947_PAGE0, 2, &__val);
    lsb = 145;
    } else {
    ret = ltc2947_val_read(st, LTC2947_REG_VOLTAGE_MIN,
    LTC2947_PAGE0, 2, &__val);
    }
    break;
    case hwmon_in_max_alarm:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN)
    return ltc2947_alarm_read(st, LTC2947_REG_STATVDVCC,
    LTC2947_MAX_VOLTAGE_MASK,
    val);
    return ltc2947_alarm_read(st, LTC2947_REG_STATVT,
    LTC2947_MAX_VOLTAGE_MASK, val);
    case hwmon_in_min_alarm:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN)
    return ltc2947_alarm_read(st, LTC2947_REG_STATVDVCC,
    LTC2947_MIN_VOLTAGE_MASK,
    val);
    return ltc2947_alarm_read(st, LTC2947_REG_STATVT,
    LTC2947_MIN_VOLTAGE_MASK, val);
    case hwmon_in_max:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN) {
    ret = ltc2947_val_read(st, LTC2947_REG_DVCC_THRE_H,
    LTC2947_PAGE1, 2, &__val);
    lsb = 145;
    } else {
    ret = ltc2947_val_read(st, LTC2947_REG_VOLTAGE_THRE_H,
    LTC2947_PAGE1, 2, &__val);
    }
    break;
    case hwmon_in_min:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN) {
    ret = ltc2947_val_read(st, LTC2947_REG_DVCC_THRE_L,
    LTC2947_PAGE1, 2, &__val);
    lsb = 145;
    } else {
    ret = ltc2947_val_read(st, LTC2947_REG_VOLTAGE_THRE_L,
    LTC2947_PAGE1, 2, &__val);
    }
    break;
    default:
    return -ENOTSUPP;
    }
    if (ret)
    return ret;
// val = __val * lsb;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc2947_read_energy(dev: *mut device, val: *mut i64, channel: c_int) -> c_int {
    static int ltc2947_read_energy(struct device *dev, s64 *val, const int channel)
    {
    let mut reg: c_int = channel ? LTC2947_REG_ENERGY2 : LTC2947_REG_ENERGY1;
    struct ltc2947_data *st = dev_get_drvdata(dev);
    let mut __val: i64 = 0;
    int ret;
    ret = ltc2947_val_read(st, reg, LTC2947_PAGE0, 6, &__val);
    if (ret)
    return ret;
// value in microJoule. st->lsb_energy was multiplied by 10E9
// val = DIV_S64_ROUND_CLOSEST(__val * st->lsb_energy, 1000);
    return 0;
    }
    static int ltc2947_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_in:
    return ltc2947_read_in(dev, attr, val, channel);
    case hwmon_curr:
    return ltc2947_read_curr(dev, attr, val);
    case hwmon_power:
    return ltc2947_read_power(dev, attr, val);
    case hwmon_temp:
    return ltc2947_read_temp(dev, attr, val, channel);
    case hwmon_energy64:
    return ltc2947_read_energy(dev, (s64 *)val, channel);
    default:
    return -ENOTSUPP;
    }
    }
    static int ltc2947_write_temp(struct device *dev, const u32 attr,
    long val, const int channel)
    {
    struct ltc2947_data *st = dev_get_drvdata(dev);
    if (channel < 0 || channel > LTC2947_TEMP_FAN_CHAN) {
    dev_err(st.dev, "Invalid chan%d for temperature", channel);
    return -EINVAL;
    }
    switch (attr) {
    case hwmon_temp_reset_history:
    if (val != 1)
    return -EINVAL;
    return ltc2947_reset_history(st, LTC2947_REG_TEMP_MAX,
    LTC2947_REG_TEMP_MIN);
    case hwmon_temp_max:
    val = clamp_val(val, TEMP_MIN, TEMP_MAX);
    if (channel == LTC2947_TEMP_FAN_CHAN) {
    if (!st.gpio_out)
    return -ENOTSUPP;
    return ltc2947_val_write(st,
    LTC2947_REG_TEMP_FAN_THRE_H,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val - 550, 204));
    }
    return ltc2947_val_write(st, LTC2947_REG_TEMP_THRE_H,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val - 550, 204));
    case hwmon_temp_min:
    val = clamp_val(val, TEMP_MIN, TEMP_MAX);
    if (channel == LTC2947_TEMP_FAN_CHAN) {
    if (!st.gpio_out)
    return -ENOTSUPP;
    return ltc2947_val_write(st,
    LTC2947_REG_TEMP_FAN_THRE_L,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val - 550, 204));
    }
    return ltc2947_val_write(st, LTC2947_REG_TEMP_THRE_L,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val - 550, 204));
    default:
    return -ENOTSUPP;
    }
    }
    static int ltc2947_write_power(struct device *dev, const u32 attr,
    long val)
    {
    struct ltc2947_data *st = dev_get_drvdata(dev);
    switch (attr) {
    case hwmon_power_reset_history:
    if (val != 1)
    return -EINVAL;
    return ltc2947_reset_history(st, LTC2947_REG_POWER_MAX,
    LTC2947_REG_POWER_MIN);
    case hwmon_power_max:
    val = clamp_val(val, POWER_MIN, POWER_MAX);
    return ltc2947_val_write(st, LTC2947_REG_POWER_THRE_H,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val, 200000));
    case hwmon_power_min:
    val = clamp_val(val, POWER_MIN, POWER_MAX);
    return ltc2947_val_write(st, LTC2947_REG_POWER_THRE_L,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val, 200000));
    default:
    return -ENOTSUPP;
    }
    }
    static int ltc2947_write_curr(struct device *dev, const u32 attr,
    long val)
    {
    struct ltc2947_data *st = dev_get_drvdata(dev);
    switch (attr) {
    case hwmon_curr_reset_history:
    if (val != 1)
    return -EINVAL;
    return ltc2947_reset_history(st, LTC2947_REG_CURRENT_MAX,
    LTC2947_REG_CURRENT_MIN);
    case hwmon_curr_max:
    val = clamp_val(val, CURRENT_MIN, CURRENT_MAX);
    return ltc2947_val_write(st, LTC2947_REG_CURRENT_THRE_H,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val, 12));
    case hwmon_curr_min:
    val = clamp_val(val, CURRENT_MIN, CURRENT_MAX);
    return ltc2947_val_write(st, LTC2947_REG_CURRENT_THRE_L,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val, 12));
    default:
    return -ENOTSUPP;
    }
    }
    static int ltc2947_write_in(struct device *dev, const u32 attr, long val,
    const int channel)
    {
    struct ltc2947_data *st = dev_get_drvdata(dev);
    if (channel > LTC2947_VOLTAGE_DVCC_CHAN) {
    dev_err(st.dev, "Invalid chan%d for voltage", channel);
    return -EINVAL;
    }
    switch (attr) {
    case hwmon_in_reset_history:
    if (val != 1)
    return -EINVAL;
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN)
    return ltc2947_reset_history(st, LTC2947_REG_DVCC_MAX,
    LTC2947_REG_DVCC_MIN);
    return ltc2947_reset_history(st, LTC2947_REG_VOLTAGE_MAX,
    LTC2947_REG_VOLTAGE_MIN);
    case hwmon_in_max:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN) {
    val = clamp_val(val, VDVCC_MIN, VDVCC_MAX);
    return ltc2947_val_write(st, LTC2947_REG_DVCC_THRE_H,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val, 145));
    }
    val = clamp_val(val, VOLTAGE_MIN, VOLTAGE_MAX);
    return ltc2947_val_write(st, LTC2947_REG_VOLTAGE_THRE_H,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val, 2));
    case hwmon_in_min:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN) {
    val = clamp_val(val, VDVCC_MIN, VDVCC_MAX);
    return ltc2947_val_write(st, LTC2947_REG_DVCC_THRE_L,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val, 145));
    }
    val = clamp_val(val, VOLTAGE_MIN, VOLTAGE_MAX);
    return ltc2947_val_write(st, LTC2947_REG_VOLTAGE_THRE_L,
    LTC2947_PAGE1, 2,
    DIV_ROUND_CLOSEST(val, 2));
    default:
    return -ENOTSUPP;
    }
    }
    static int ltc2947_write(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_in:
    return ltc2947_write_in(dev, attr, val, channel);
    case hwmon_curr:
    return ltc2947_write_curr(dev, attr, val);
    case hwmon_power:
    return ltc2947_write_power(dev, attr, val);
    case hwmon_temp:
    return ltc2947_write_temp(dev, attr, val, channel);
    default:
    return -ENOTSUPP;
    }
    }
    static int ltc2947_read_labels(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    switch (type) {
    case hwmon_in:
    if (channel == LTC2947_VOLTAGE_DVCC_CHAN)
// str = "DVCC";
    else
// str = "VP-VM";
    return 0;
    case hwmon_curr:
// str = "IP-IM";
    return 0;
    case hwmon_temp:
    if (channel == LTC2947_TEMP_FAN_CHAN)
// str = "TEMPFAN";
    else
// str = "Ambient";
    return 0;
    case hwmon_power:
// str = "Power";
    return 0;
    default:
    return -ENOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn ltc2947_in_is_visible(attr: u32) -> c_int {
    static int ltc2947_in_is_visible(const u32 attr)
    {
    switch (attr) {
    case hwmon_in_input:
    case hwmon_in_highest:
    case hwmon_in_lowest:
    case hwmon_in_max_alarm:
    case hwmon_in_min_alarm:
    case hwmon_in_label:
    return 0444;
    case hwmon_in_reset_history:
    return 0200;
    case hwmon_in_max:
    case hwmon_in_min:
    return 0644;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn ltc2947_curr_is_visible(attr: u32) -> c_int {
    static int ltc2947_curr_is_visible(const u32 attr)
    {
    switch (attr) {
    case hwmon_curr_input:
    case hwmon_curr_highest:
    case hwmon_curr_lowest:
    case hwmon_curr_max_alarm:
    case hwmon_curr_min_alarm:
    case hwmon_curr_label:
    return 0444;
    case hwmon_curr_reset_history:
    return 0200;
    case hwmon_curr_max:
    case hwmon_curr_min:
    return 0644;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn ltc2947_power_is_visible(attr: u32) -> c_int {
    static int ltc2947_power_is_visible(const u32 attr)
    {
    switch (attr) {
    case hwmon_power_input:
    case hwmon_power_input_highest:
    case hwmon_power_input_lowest:
    case hwmon_power_label:
    case hwmon_power_max_alarm:
    case hwmon_power_min_alarm:
    return 0444;
    case hwmon_power_reset_history:
    return 0200;
    case hwmon_power_max:
    case hwmon_power_min:
    return 0644;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn ltc2947_temp_is_visible(attr: u32) -> c_int {
    static int ltc2947_temp_is_visible(const u32 attr)
    {
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_highest:
    case hwmon_temp_lowest:
    case hwmon_temp_max_alarm:
    case hwmon_temp_min_alarm:
    case hwmon_temp_label:
    return 0444;
    case hwmon_temp_reset_history:
    return 0200;
    case hwmon_temp_max:
    case hwmon_temp_min:
    return 0644;
    default:
    return 0;
    }
    }
    static umode_t ltc2947_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_in:
    return ltc2947_in_is_visible(attr);
    case hwmon_curr:
    return ltc2947_curr_is_visible(attr);
    case hwmon_power:
    return ltc2947_power_is_visible(attr);
    case hwmon_temp:
    return ltc2947_temp_is_visible(attr);
    case hwmon_energy64:
    return 0444;
    default:
    return 0;
    }
    }
    static const struct hwmon_channel_info * const ltc2947_info[] = {
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_LOWEST | HWMON_I_HIGHEST |
    HWMON_I_MAX | HWMON_I_MIN | HWMON_I_RESET_HISTORY |
    HWMON_I_MIN_ALARM | HWMON_I_MAX_ALARM |
    HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LOWEST | HWMON_I_HIGHEST |
    HWMON_I_MAX | HWMON_I_MIN | HWMON_I_RESET_HISTORY |
    HWMON_I_MIN_ALARM | HWMON_I_MAX_ALARM |
    HWMON_I_LABEL),
    HWMON_CHANNEL_INFO(curr,
    HWMON_C_INPUT | HWMON_C_LOWEST | HWMON_C_HIGHEST |
    HWMON_C_MAX | HWMON_C_MIN | HWMON_C_RESET_HISTORY |
    HWMON_C_MIN_ALARM | HWMON_C_MAX_ALARM |
    HWMON_C_LABEL),
    HWMON_CHANNEL_INFO(power,
    HWMON_P_INPUT | HWMON_P_INPUT_LOWEST |
    HWMON_P_INPUT_HIGHEST | HWMON_P_MAX | HWMON_P_MIN |
    HWMON_P_RESET_HISTORY | HWMON_P_MAX_ALARM |
    HWMON_P_MIN_ALARM | HWMON_P_LABEL),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LOWEST | HWMON_T_HIGHEST |
    HWMON_T_MAX | HWMON_T_MIN | HWMON_T_RESET_HISTORY |
    HWMON_T_MIN_ALARM | HWMON_T_MAX_ALARM |
    HWMON_T_LABEL,
    HWMON_T_MAX_ALARM | HWMON_T_MIN_ALARM | HWMON_T_MAX |
    HWMON_T_MIN | HWMON_T_LABEL),
    HWMON_CHANNEL_INFO(energy64,
    HWMON_E_INPUT,
    HWMON_E_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops ltc2947_hwmon_ops = {
    .is_visible = ltc2947_is_visible,
    .read = ltc2947_read,
    .write = ltc2947_write,
    .read_string = ltc2947_read_labels,
    };
    static const struct hwmon_chip_info ltc2947_chip_info = {
    .ops = &ltc2947_hwmon_ops,
    .info = ltc2947_info,
    };
#[no_mangle]
unsafe extern "C" fn ltc2947_setup(st: *mut ltc2947_data) -> c_int {
    static int ltc2947_setup(struct ltc2947_data *st)
    {
    int ret;
    struct clk *extclk;
    u32 dummy, deadband, pol;
    u32 accum[2];
// clear status register by reading it
    ret = regmap_read(st.map, LTC2947_REG_STATUS, &dummy);
    if (ret)
    return ret;
//
// Set max/min for power here since the default values x scale
// would overflow on 32bit arch
//
    ret = ltc2947_val_write(st, LTC2947_REG_POWER_THRE_H, LTC2947_PAGE1, 2,
    POWER_MAX / 200000);
    if (ret)
    return ret;
    ret = ltc2947_val_write(st, LTC2947_REG_POWER_THRE_L, LTC2947_PAGE1, 2,
    POWER_MIN / 200000);
    if (ret)
    return ret;
// check external clock presence
    extclk = devm_clk_get_optional_enabled(st.dev, core::ptr::null_mut());
    if (IS_ERR(extclk))
    return dev_err_probe(st.dev, PTR_ERR(extclk),
    "Failed to get external clock\n");
    if (extclk) {
    unsigned long rate_hz;
    let mut pre: u8 = 0, div, tbctl;
    u64 aux;
// let's calculate and set the right valus in TBCTL
    rate_hz = clk_get_rate(extclk);
    if (rate_hz < LTC2947_CLK_MIN || rate_hz > LTC2947_CLK_MAX) {
    dev_err(st.dev, "Invalid rate:%lu for external clock",
    rate_hz);
    return -EINVAL;
    }
// as in table 1 of the datasheet
    if (rate_hz >= LTC2947_CLK_MIN && rate_hz <= 1000000)
    pre = 0;
#[no_mangle]
pub unsafe extern "C" fn if(2000000: rate_hz > 1000000 && rate_hz <=) -> else {
    else if (rate_hz > 1000000 && rate_hz <= 2000000)
    pre = 1;
#[no_mangle]
pub unsafe extern "C" fn if(4000000: rate_hz > 2000000 && rate_hz <=) -> else {
    else if (rate_hz > 2000000 && rate_hz <= 4000000)
    pre = 2;
#[no_mangle]
pub unsafe extern "C" fn if(8000000: rate_hz > 4000000 && rate_hz <=) -> else {
    else if (rate_hz > 4000000 && rate_hz <= 8000000)
    pre = 3;
#[no_mangle]
pub unsafe extern "C" fn if(16000000: rate_hz > 8000000 && rate_hz <=) -> else {
    else if (rate_hz > 8000000 && rate_hz <= 16000000)
    pre = 4;
#[no_mangle]
pub unsafe extern "C" fn if(LTC2947_CLK_MAX: rate_hz > 16000000 && rate_hz <=) -> else {
    else if (rate_hz > 16000000 && rate_hz <= LTC2947_CLK_MAX)
    pre = 5;
//
// Div is given by:
// floor(fref / (2^PRE * 32768))
//
    div = rate_hz / ((1 << pre) * 32768);
    tbctl = LTC2947_PRE(pre) | LTC2947_DIV(div);
    ret = regmap_write(st.map, LTC2947_REG_TBCTL, tbctl);
    if (ret)
    return ret;
//
// The energy lsb is given by (in W*s):
// 06416 * (1/fref) * 2^PRE * (DIV + 1)
// The value is multiplied by 10E9
//
    aux = (div + 1) * ((1 << pre) * 641600000ULL);
    st.lsb_energy = DIV_ROUND_CLOSEST_ULL(aux, rate_hz);
    } else {
// 19.89E-6 * 10E9
    st.lsb_energy = 19890;
    }
    ret = device_property_read_u32_array(st.dev, "adi,accumulator-ctl-pol",
    accum, ARRAY_SIZE(accum));
    if (!ret) {
    u32 accum_reg = LTC2947_ACCUM_POL_1(accum[0]) |
    LTC2947_ACCUM_POL_2(accum[1]);
    ret = regmap_write(st.map, LTC2947_REG_ACCUM_POL, accum_reg);
    if (ret)
    return ret;
    }
    ret = device_property_read_u32(st.dev,
    "adi,accumulation-deadband-microamp",
    &deadband);
    if (!ret) {
// the LSB is the same as the current, so 3mA
    ret = regmap_write(st.map, LTC2947_REG_ACCUM_DEADBAND,
    deadband / (1000 * 3));
    if (ret)
    return ret;
    }
// check gpio cfg
    ret = device_property_read_u32(st.dev, "adi,gpio-out-pol", &pol);
    if (!ret) {
// setup GPIO as output
    u32 gpio_ctl = LTC2947_GPIO_EN(1) | LTC2947_GPIO_FAN_EN(1) |
    LTC2947_GPIO_FAN_POL(pol);
    st.gpio_out = true;
    ret = regmap_write(st.map, LTC2947_REG_GPIOSTATCTL, gpio_ctl);
    if (ret)
    return ret;
    }
    ret = device_property_read_u32_array(st.dev, "adi,gpio-in-accum",
    accum, ARRAY_SIZE(accum));
    if (!ret) {
//
// Setup the accum options. The gpioctl is already defined as
// input by default.
//
    u32 accum_val = LTC2947_ACCUM_POL_1(accum[0]) |
    LTC2947_ACCUM_POL_2(accum[1]);
    if (st.gpio_out) {
    dev_err(st.dev,
    "Cannot have input gpio config if already configured as output");
    return -EINVAL;
    }
    ret = regmap_write(st.map, LTC2947_REG_GPIO_ACCUM, accum_val);
    if (ret)
    return ret;
    }
// set continuos mode
    return regmap_update_bits(st.map, LTC2947_REG_CTRL,
    LTC2947_CONT_MODE_MASK, LTC2947_CONT_MODE(1));
    }
#[no_mangle]
pub unsafe extern "C" fn ltc2947_core_probe(map: *mut regmap, name: *const c_char) -> c_int {
    int ltc2947_core_probe(struct regmap *map, const char *name)
    {
    struct ltc2947_data *st;
    struct device *dev = regmap_get_device(map);
    struct device *hwmon;
    int ret;
    st = devm_kzalloc(dev, sizeof(*st), GFP_KERNEL);
    if (!st)
    return -ENOMEM;
    st.map = map;
    st.dev = dev;
    dev_set_drvdata(dev, st);
    ret = ltc2947_setup(st);
    if (ret)
    return ret;
    hwmon = devm_hwmon_device_register_with_info(dev, name, st,
    &ltc2947_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon);
    }
    EXPORT_SYMBOL_GPL(ltc2947_core_probe);
#[no_mangle]
unsafe extern "C" fn ltc2947_resume(dev: *mut device) -> c_int {
    static int ltc2947_resume(struct device *dev)
    {
    struct ltc2947_data *st = dev_get_drvdata(dev);
    let mut ctrl: u32 = 0;
    int ret;
// dummy read to wake the device
    ret = regmap_read(st.map, LTC2947_REG_CTRL, &ctrl);
    if (ret)
    return ret;
//
// Wait for the device. It takes 100ms to wake up so, 10ms extra
// should be enough.
//
    msleep(110);
    ret = regmap_read(st.map, LTC2947_REG_CTRL, &ctrl);
    if (ret)
    return ret;
// ctrl should be 0
    if (ctrl != 0) {
    dev_err(st.dev, "Device failed to wake up, ctl:%02X\n", ctrl);
    return -ETIMEDOUT;
    }
// set continuous mode
    return regmap_update_bits(st.map, LTC2947_REG_CTRL,
    LTC2947_CONT_MODE_MASK, LTC2947_CONT_MODE(1));
    }
#[no_mangle]
unsafe extern "C" fn ltc2947_suspend(dev: *mut device) -> c_int {
    static int ltc2947_suspend(struct device *dev)
    {
    struct ltc2947_data *st = dev_get_drvdata(dev);
    return regmap_update_bits(st.map, LTC2947_REG_CTRL,
    LTC2947_SHUTDOWN_MASK, 1);
    }
    EXPORT_SIMPLE_DEV_PM_OPS(ltc2947_pm_ops, ltc2947_suspend, ltc2947_resume);
    const struct of_device_id ltc2947_of_match[] = {
    { .compatible = "adi,ltc2947" },
    {}
    };
    EXPORT_SYMBOL_GPL(ltc2947_of_match);
    MODULE_DEVICE_TABLE(of, ltc2947_of_match);
    MODULE_AUTHOR("Nuno Sa <nuno.sa@analog.com>");
    MODULE_DESCRIPTION("LTC2947 power and energy monitor core driver");
    MODULE_LICENSE("GPL");
