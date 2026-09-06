//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/stts751.c
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
// STTS751 sensor driver
//
// Copyright (C) 2016-2017 Istituto Italiano di Tecnologia - RBCS - EDL
// Robotics, Brain and Cognitive Sciences department
// Electronic Design Laboratory
//
// Written by Andrea Merello <andrea.merello@gmail.com>
//
// Based on  LM95241 driver and LM90 driver
//

    static const unsigned short normal_i2c[] = {
    0x48, 0x49, 0x38, 0x39,  /* STTS751-0 */
    0x4A, 0x4B, 0x3A, 0x3B,  /* STTS751-1 */
    I2C_CLIENT_END };
pub const STTS751_REG_TEMP_H: c_uint = 0x00;
pub const STTS751_REG_STATUS: c_uint = 0x01;

pub const STTS751_REG_TEMP_L: c_uint = 0x02;
pub const STTS751_REG_CONF: c_uint = 0x03;
pub const STTS751_CONF_RES_MASK: c_uint = 0x0C;
pub const STTS751_CONF_RES_SHIFT: c_int = 2;

pub const STTS751_REG_RATE: c_uint = 0x04;
pub const STTS751_REG_HLIM_H: c_uint = 0x05;
pub const STTS751_REG_HLIM_L: c_uint = 0x06;
pub const STTS751_REG_LLIM_H: c_uint = 0x07;
pub const STTS751_REG_LLIM_L: c_uint = 0x08;
pub const STTS751_REG_TLIM: c_uint = 0x20;
pub const STTS751_REG_HYST: c_uint = 0x21;
pub const STTS751_REG_SMBUS_TO: c_uint = 0x22;
pub const STTS751_REG_PROD_ID: c_uint = 0xFD;
pub const STTS751_REG_MAN_ID: c_uint = 0xFE;
pub const STTS751_REG_REV_ID: c_uint = 0xFF;
pub const STTS751_0_PROD_ID: c_uint = 0x00;
pub const STTS751_1_PROD_ID: c_uint = 0x01;
pub const ST_MAN_ID: c_uint = 0x53;
//
// Possible update intervals are (in mS):
// 16000, 8000, 4000, 2000, 1000, 500, 250, 125, 62.5, 31.25
// However we are not going to complicate things too much and we stick to the
// approx value in mS.
//
    static const int stts751_intervals[] = {
    16000, 8000, 4000, 2000, 1000, 500, 250, 125, 63, 31
    };
    static const struct i2c_device_id stts751_id[] = {
    { .name = "stts751" },
    { }
    };
    static const struct of_device_id __maybe_unused stts751_of_match[] = {
    { .compatible = "st,stts751" },
    { },
    };
    MODULE_DEVICE_TABLE(of, stts751_of_match);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stts751_priv {
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub access_lock: mutex,
    pub interval: u8,
    pub res: c_int,
    pub event_min: int event_max,,
    pub therm: c_int,
    pub hyst: c_int,
    pub temp: c_int,
    pub last_alert_update: unsigned long last_update,,
    pub config: u8,
    pub therm_trip: bool min_alert, max_alert,,
    pub alert_valid: bool data_valid,,
    pub notify_min: bool notify_max,,
}

//
// These functions converts temperature from HW format to integer format and
// vice-vers. They are (mostly) taken from lm90 driver. Unit is in mC.
//
#[no_mangle]
unsafe extern "C" fn stts751_to_deg(hw_val: i16) -> c_int {
    static int stts751_to_deg(s16 hw_val)
    {
    return hw_val * 125 / 32;
    }
#[no_mangle]
unsafe extern "C" fn stts751_to_hw(val: c_int) -> i32 {
    static s32 stts751_to_hw(int val)
    {
    return DIV_ROUND_CLOSEST(val, 125) * 32;
    }
#[no_mangle]
unsafe extern "C" fn stts751_adjust_resolution(priv: *mut stts751_priv) -> c_int {
    static int stts751_adjust_resolution(struct stts751_priv *priv)
    {
    u8 res;
    switch (priv.interval) {
    case 9:
// 10 bits
    res = 0;
    break;
    case 8:
// 11 bits
    res = 1;
    break;
    default:
// 12 bits
    res = 3;
    break;
    }
    if (priv.res == res)
    return 0;
    priv.config &= ~STTS751_CONF_RES_MASK;
    priv.config |= res << STTS751_CONF_RES_SHIFT;
    dev_dbg(&priv.client.dev, "setting res %d. config %x",
    res, priv.config);
    priv.res = res;
    return i2c_smbus_write_byte_data(priv.client,
    STTS751_REG_CONF, priv.config);
    }
#[no_mangle]
unsafe extern "C" fn stts751_update_temp(priv: *mut stts751_priv) -> c_int {
    static int stts751_update_temp(struct stts751_priv *priv)
    {
    s32 integer1, integer2, frac;
//
// There is a trick here, like in the lm90 driver. We have to read two
// registers to get the sensor temperature, but we have to beware a
// conversion could occur between the readings. We could use the
// one-shot conversion register, but we don't want to do this (disables
// hardware monitoring). So the solution used here is to read the high
// byte once, then the low byte, then the high byte again. If the new
// high byte matches the old one, then we have a valid reading. Else we
// have to read the low byte again, and now we believe we have a correct
// reading.
//
    integer1 = i2c_smbus_read_byte_data(priv.client, STTS751_REG_TEMP_H);
    if (integer1 < 0) {
    dev_dbg(&priv.client.dev,
    "I2C read failed (temp H). ret: %x\n", integer1);
    return integer1;
    }
    frac = i2c_smbus_read_byte_data(priv.client, STTS751_REG_TEMP_L);
    if (frac < 0) {
    dev_dbg(&priv.client.dev,
    "I2C read failed (temp L). ret: %x\n", frac);
    return frac;
    }
    integer2 = i2c_smbus_read_byte_data(priv.client, STTS751_REG_TEMP_H);
    if (integer2 < 0) {
    dev_dbg(&priv.client.dev,
    "I2C 2nd read failed (temp H). ret: %x\n", integer2);
    return integer2;
    }
    if (integer1 != integer2) {
    frac = i2c_smbus_read_byte_data(priv.client,
    STTS751_REG_TEMP_L);
    if (frac < 0) {
    dev_dbg(&priv.client.dev,
    "I2C 2nd read failed (temp L). ret: %x\n",
    frac);
    return frac;
    }
    }
    priv.temp = stts751_to_deg((integer1 << 8) | frac);
    return 0;
    }
    static int stts751_set_temp_reg16(struct stts751_priv *priv, int temp,
    u8 hreg, u8 lreg)
    {
    s32 hwval;
    int ret;
    hwval = stts751_to_hw(temp);
    ret = i2c_smbus_write_byte_data(priv.client, hreg, hwval >> 8);
    if (ret)
    return ret;
    return i2c_smbus_write_byte_data(priv.client, lreg, hwval & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn stts751_set_temp_reg8(priv: *mut stts751_priv, temp: c_int, reg: u8) -> c_int {
    static int stts751_set_temp_reg8(struct stts751_priv *priv, int temp, u8 reg)
    {
    s32 hwval;
    hwval = stts751_to_hw(temp);
    return i2c_smbus_write_byte_data(priv.client, reg, hwval >> 8);
    }
    static int stts751_read_reg16(struct stts751_priv *priv, int *temp,
    u8 hreg, u8 lreg)
    {
    int integer, frac;
    integer = i2c_smbus_read_byte_data(priv.client, hreg);
    if (integer < 0)
    return integer;
    frac = i2c_smbus_read_byte_data(priv.client, lreg);
    if (frac < 0)
    return frac;
// temp = stts751_to_deg((integer << 8) | frac);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stts751_read_reg8(priv: *mut stts751_priv, temp: *mut c_int, reg: u8) -> c_int {
    static int stts751_read_reg8(struct stts751_priv *priv, int *temp, u8 reg)
    {
    int integer;
    integer = i2c_smbus_read_byte_data(priv.client, reg);
    if (integer < 0)
    return integer;
// temp = stts751_to_deg(integer << 8);
    return 0;
    }
//
// Update alert flags without waiting for cache to expire. We detects alerts
// immediately for the sake of the alert handler; we still need to deal with
// caching to workaround the fact that alarm flags int the status register,
// despite what the datasheet claims, gets always cleared on read.
//
#[no_mangle]
unsafe extern "C" fn stts751_update_alert(priv: *mut stts751_priv) -> c_int {
    static int stts751_update_alert(struct stts751_priv *priv)
    {
    int ret;
    bool conv_done;
    let mut cache_time: c_int = msecs_to_jiffies(stts751_intervals[priv.interval]);
//
// Add another 10% because if we run faster than the HW conversion
// rate we will end up in reporting incorrectly alarms.
//
    cache_time += cache_time / 10;
    ret = i2c_smbus_read_byte_data(priv.client, STTS751_REG_STATUS);
    if (ret < 0)
    return ret;
    dev_dbg(&priv.client.dev, "status reg %x\n", ret);
    conv_done = ret & (STTS751_STATUS_TRIPH | STTS751_STATUS_TRIPL);
//
// Reset the cache if the cache time expired, or if we are sure
// we have valid data from a device conversion, or if we know
// our cache has been never written.
//
// Note that when the cache has been never written the point is
// to correctly initialize the timestamp, rather than clearing
// the cache values.
//
// Note that updating the cache timestamp when we get an alarm flag
// is required, otherwise we could incorrectly report alarms to be zero.
//
    if (time_after(jiffies,	priv.last_alert_update + cache_time) ||
    conv_done || !priv.alert_valid) {
    priv.max_alert = false;
    priv.min_alert = false;
    priv.alert_valid = true;
    priv.last_alert_update = jiffies;
    dev_dbg(&priv.client.dev, "invalidating alert cache\n");
    }
    priv.max_alert |= !!(ret & STTS751_STATUS_TRIPH);
    priv.min_alert |= !!(ret & STTS751_STATUS_TRIPL);
    priv.therm_trip = !!(ret & STTS751_STATUS_TRIPT);
    dev_dbg(&priv.client.dev, "max_alert: %d, min_alert: %d, therm_trip: %d\n",
    priv.max_alert, priv.min_alert, priv.therm_trip);
    return 0;
    }
    static void stts751_alert(struct i2c_client *client,
    enum i2c_alert_protocol type, unsigned int data)
    {
    int ret;
    struct stts751_priv *priv = i2c_get_clientdata(client);
    if (type != I2C_PROTOCOL_SMBUS_ALERT)
    return;
    dev_dbg(&client.dev, "alert!");
    mutex_lock(&priv.access_lock);
    ret = stts751_update_alert(priv);
    if (ret < 0) {
// default to worst case
    priv.max_alert = true;
    priv.min_alert = true;
    dev_warn(priv.dev,
    "Alert received, but can't communicate to the device. Triggering all alarms!");
    }
    if (priv.max_alert) {
    if (priv.notify_max)
    dev_notice(priv.dev, "got alert for HIGH temperature");
    priv.notify_max = false;
// unblock alert poll
    sysfs_notify(&priv.dev.kobj, core::ptr::null_mut(), "temp1_max_alarm");
    }
    if (priv.min_alert) {
    if (priv.notify_min)
    dev_notice(priv.dev, "got alert for LOW temperature");
    priv.notify_min = false;
// unblock alert poll
    sysfs_notify(&priv.dev.kobj, core::ptr::null_mut(), "temp1_min_alarm");
    }
    if (priv.min_alert || priv.max_alert)
    kobject_uevent(&priv.dev.kobj, KOBJ_CHANGE);
    mutex_unlock(&priv.access_lock);
    }
#[no_mangle]
unsafe extern "C" fn stts751_update(priv: *mut stts751_priv) -> c_int {
    static int stts751_update(struct stts751_priv *priv)
    {
    int ret;
    let mut cache_time: c_int = msecs_to_jiffies(stts751_intervals[priv.interval]);
    if (time_after(jiffies,	priv.last_update + cache_time) ||
    !priv.data_valid) {
    ret = stts751_update_temp(priv);
    if (ret)
    return ret;
    ret = stts751_update_alert(priv);
    if (ret)
    return ret;
    priv.data_valid = true;
    priv.last_update = jiffies;
    }
    return 0;
    }
    static ssize_t max_alarm_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    struct stts751_priv *priv = dev_get_drvdata(dev);
    mutex_lock(&priv.access_lock);
    ret = stts751_update(priv);
    if (!ret)
    priv.notify_max = true;
    mutex_unlock(&priv.access_lock);
    if (ret < 0)
    return ret;
    return sysfs_emit(buf, "%d\n", priv.max_alert);
    }
    static ssize_t min_alarm_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    struct stts751_priv *priv = dev_get_drvdata(dev);
    mutex_lock(&priv.access_lock);
    ret = stts751_update(priv);
    if (!ret)
    priv.notify_min = true;
    mutex_unlock(&priv.access_lock);
    if (ret < 0)
    return ret;
    return sysfs_emit(buf, "%d\n", priv.min_alert);
    }
    static ssize_t input_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    int ret;
    struct stts751_priv *priv = dev_get_drvdata(dev);
    mutex_lock(&priv.access_lock);
    ret = stts751_update(priv);
    mutex_unlock(&priv.access_lock);
    if (ret < 0)
    return ret;
    return sysfs_emit(buf, "%d\n", priv.temp);
    }
    static ssize_t therm_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct stts751_priv *priv = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", priv.therm);
    }
    static ssize_t therm_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int ret;
    long temp;
    struct stts751_priv *priv = dev_get_drvdata(dev);
    if (kstrtol(buf, 10, &temp) < 0)
    return -EINVAL;
// HW works in range -64C to +127.937C
    temp = clamp_val(temp, -64000, 127937);
    mutex_lock(&priv.access_lock);
    ret = stts751_set_temp_reg8(priv, temp, STTS751_REG_TLIM);
    if (ret)
    goto exit;
    dev_dbg(&priv.client.dev, "setting therm %ld", temp);
//
// hysteresis reg is relative to therm, so the HW does not need to be
// adjusted, we need to update our local copy only.
//
    priv.hyst = temp - (priv.therm - priv.hyst);
    priv.therm = temp;
    exit:
    mutex_unlock(&priv.access_lock);
    if (ret)
    return ret;
    return count;
    }
    static ssize_t hyst_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct stts751_priv *priv = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", priv.hyst);
    }
    static ssize_t hyst_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int ret;
    long temp;
    struct stts751_priv *priv = dev_get_drvdata(dev);
    if (kstrtol(buf, 10, &temp) < 0)
    return -EINVAL;
    mutex_lock(&priv.access_lock);
// HW works in range -64C to +127.937C
    temp = clamp_val(temp, -64000, priv.therm);
    priv.hyst = temp;
    dev_dbg(&priv.client.dev, "setting hyst %ld", temp);
    temp = priv.therm - temp;
    ret = stts751_set_temp_reg8(priv, temp, STTS751_REG_HYST);
    mutex_unlock(&priv.access_lock);
    if (ret)
    return ret;
    return count;
    }
    static ssize_t therm_trip_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    struct stts751_priv *priv = dev_get_drvdata(dev);
    mutex_lock(&priv.access_lock);
    ret = stts751_update(priv);
    mutex_unlock(&priv.access_lock);
    if (ret < 0)
    return ret;
    return sysfs_emit(buf, "%d\n", priv.therm_trip);
    }
    static ssize_t max_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct stts751_priv *priv = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", priv.event_max);
    }
    static ssize_t max_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int ret;
    long temp;
    struct stts751_priv *priv = dev_get_drvdata(dev);
    if (kstrtol(buf, 10, &temp) < 0)
    return -EINVAL;
    mutex_lock(&priv.access_lock);
// HW works in range -64C to +127.937C
    temp = clamp_val(temp, priv.event_min, 127937);
    ret = stts751_set_temp_reg16(priv, temp,
    STTS751_REG_HLIM_H, STTS751_REG_HLIM_L);
    if (ret)
    goto exit;
    dev_dbg(&priv.client.dev, "setting event max %ld", temp);
    priv.event_max = temp;
    ret = count;
    exit:
    mutex_unlock(&priv.access_lock);
    return ret;
    }
    static ssize_t min_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct stts751_priv *priv = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", priv.event_min);
    }
    static ssize_t min_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int ret;
    long temp;
    struct stts751_priv *priv = dev_get_drvdata(dev);
    if (kstrtol(buf, 10, &temp) < 0)
    return -EINVAL;
    mutex_lock(&priv.access_lock);
// HW works in range -64C to +127.937C
    temp = clamp_val(temp, -64000, priv.event_max);
    ret = stts751_set_temp_reg16(priv, temp,
    STTS751_REG_LLIM_H, STTS751_REG_LLIM_L);
    if (ret)
    goto exit;
    dev_dbg(&priv.client.dev, "setting event min %ld", temp);
    priv.event_min = temp;
    ret = count;
    exit:
    mutex_unlock(&priv.access_lock);
    return ret;
    }
    static ssize_t interval_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct stts751_priv *priv = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n",
    stts751_intervals[priv.interval]);
    }
    static ssize_t interval_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    unsigned long val;
    int idx;
    let mut ret: c_int = count;
    struct stts751_priv *priv = dev_get_drvdata(dev);
    if (kstrtoul(buf, 10, &val) < 0)
    return -EINVAL;
    idx = find_closest_descending(val, stts751_intervals,
    ARRAY_SIZE(stts751_intervals));
    dev_dbg(&priv.client.dev, "setting interval. req:%lu, idx: %d, val: %d",
    val, idx, stts751_intervals[idx]);
    mutex_lock(&priv.access_lock);
    if (priv.interval == idx)
    goto exit;
//
// In early development stages I've become suspicious about the chip
// starting to misbehave if I ever set, even briefly, an invalid
// configuration. While I'm not sure this is really needed, be
// conservative and set rate/resolution in such an order that avoids
// passing through an invalid configuration.
//
// speed up: lower the resolution, then modify convrate
    if (priv.interval < idx) {
    dev_dbg(&priv.client.dev, "lower resolution, then modify convrate");
    priv.interval = idx;
    ret = stts751_adjust_resolution(priv);
    if (ret)
    goto exit;
    }
    ret = i2c_smbus_write_byte_data(priv.client, STTS751_REG_RATE, idx);
    if (ret)
    goto exit;
// slow down: modify convrate, then raise resolution
    if (priv.interval != idx) {
    dev_dbg(&priv.client.dev, "modify convrate, then raise resolution");
    priv.interval = idx;
    ret = stts751_adjust_resolution(priv);
    if (ret)
    goto exit;
    }
    ret = count;
    exit:
    mutex_unlock(&priv.access_lock);
    return ret;
    }
    static int stts751_detect(struct i2c_client *new_client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = new_client.adapter;
    const char *name;
    int tmp;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    tmp = i2c_smbus_read_byte_data(new_client, STTS751_REG_MAN_ID);
    if (tmp != ST_MAN_ID)
    return -ENODEV;
// lower temperaure registers always have bits 0-3 set to zero
    tmp = i2c_smbus_read_byte_data(new_client, STTS751_REG_TEMP_L);
    if (tmp & 0xf)
    return -ENODEV;
    tmp = i2c_smbus_read_byte_data(new_client, STTS751_REG_HLIM_L);
    if (tmp & 0xf)
    return -ENODEV;
    tmp = i2c_smbus_read_byte_data(new_client, STTS751_REG_LLIM_L);
    if (tmp & 0xf)
    return -ENODEV;
// smbus timeout register always have bits 0-7 set to zero
    tmp = i2c_smbus_read_byte_data(new_client, STTS751_REG_SMBUS_TO);
    if (tmp & 0x7f)
    return -ENODEV;
    tmp = i2c_smbus_read_byte_data(new_client, STTS751_REG_PROD_ID);
    switch (tmp) {
    case STTS751_0_PROD_ID:
    name = "STTS751-0";
    break;
    case STTS751_1_PROD_ID:
    name = "STTS751-1";
    break;
    default:
    return -ENODEV;
    }
    dev_dbg(&new_client.dev, "Chip %s detected", name);
    strscpy(info.type, stts751_id[0].name, I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stts751_read_chip_config(priv: *mut stts751_priv) -> c_int {
    static int stts751_read_chip_config(struct stts751_priv *priv)
    {
    int ret;
    int tmp;
    ret = i2c_smbus_read_byte_data(priv.client, STTS751_REG_CONF);
    if (ret < 0)
    return ret;
    priv.config = ret;
    priv.res = (ret & STTS751_CONF_RES_MASK) >> STTS751_CONF_RES_SHIFT;
    ret = i2c_smbus_read_byte_data(priv.client, STTS751_REG_RATE);
    if (ret < 0)
    return ret;
    if (ret >= ARRAY_SIZE(stts751_intervals)) {
    dev_err(priv.dev, "Unrecognized conversion rate 0x%x\n", ret);
    return -ENODEV;
    }
    priv.interval = ret;
    ret = stts751_read_reg16(priv, &priv.event_max,
    STTS751_REG_HLIM_H, STTS751_REG_HLIM_L);
    if (ret)
    return ret;
    ret = stts751_read_reg16(priv, &priv.event_min,
    STTS751_REG_LLIM_H, STTS751_REG_LLIM_L);
    if (ret)
    return ret;
    ret = stts751_read_reg8(priv, &priv.therm, STTS751_REG_TLIM);
    if (ret)
    return ret;
    ret = stts751_read_reg8(priv, &tmp, STTS751_REG_HYST);
    if (ret)
    return ret;
    priv.hyst = priv.therm - tmp;
    return 0;
    }
    static SENSOR_DEVICE_ATTR_RO(temp1_input, input, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_min, min, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_max, max, 0);
    static SENSOR_DEVICE_ATTR_RO(temp1_min_alarm, min_alarm, 0);
    static SENSOR_DEVICE_ATTR_RO(temp1_max_alarm, max_alarm, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_crit, therm, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_crit_hyst, hyst, 0);
    static SENSOR_DEVICE_ATTR_RO(temp1_crit_alarm, therm_trip, 0);
    static SENSOR_DEVICE_ATTR_RW(update_interval, interval, 0);
    static struct attribute *stts751_attrs[] = {
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_min.dev_attr.attr,
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp1_min_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_max_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_crit.dev_attr.attr,
    &sensor_dev_attr_temp1_crit_hyst.dev_attr.attr,
    &sensor_dev_attr_temp1_crit_alarm.dev_attr.attr,
    &sensor_dev_attr_update_interval.dev_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(stts751);
#[no_mangle]
unsafe extern "C" fn stts751_probe(client: *mut i2c_client) -> c_int {
    static int stts751_probe(struct i2c_client *client)
    {
    struct stts751_priv *priv;
    int ret;
    bool smbus_nto;
    int rev_id;
    priv = devm_kzalloc(&client.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.client = client;
    priv.notify_max = true;
    priv.notify_min = true;
    i2c_set_clientdata(client, priv);
    mutex_init(&priv.access_lock);
    if (device_property_present(&client.dev,
    "smbus-timeout-disable")) {
    smbus_nto = device_property_read_bool(&client.dev,
    "smbus-timeout-disable");
    ret = i2c_smbus_write_byte_data(client,	STTS751_REG_SMBUS_TO,
    smbus_nto ? 0 : 0x80);
    if (ret)
    return ret;
    }
    rev_id = i2c_smbus_read_byte_data(client, STTS751_REG_REV_ID);
    if (rev_id < 0)
    return -ENODEV;
    if (rev_id != 0x1) {
    dev_dbg(&client.dev, "Chip revision 0x%x is untested\n",
    rev_id);
    }
    ret = stts751_read_chip_config(priv);
    if (ret)
    return ret;
    priv.config &= ~(STTS751_CONF_STOP | STTS751_CONF_EVENT_DIS);
    ret = i2c_smbus_write_byte_data(client,	STTS751_REG_CONF, priv.config);
    if (ret)
    return ret;
    priv.dev = devm_hwmon_device_register_with_groups(&client.dev,
    client.name, priv,
    stts751_groups);
    return PTR_ERR_OR_ZERO(priv.dev);
    }
    MODULE_DEVICE_TABLE(i2c, stts751_id);
    static struct i2c_driver stts751_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= DEVNAME,
    .of_match_table = of_match_ptr(stts751_of_match),
    },
    .probe		= stts751_probe,
    .id_table	= stts751_id,
    .detect		= stts751_detect,
    .alert		= stts751_alert,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(stts751_driver);
    MODULE_AUTHOR("Andrea Merello <andrea.merello@gmail.com>");
    MODULE_DESCRIPTION("STTS751 sensor driver");
    MODULE_LICENSE("GPL");
