//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ds620.c
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
// ds620.c - Support for temperature sensor and thermostat DS620
//
// Copyright (C) 2010, 2011 Roland Stigge <stigge@antcom.de>
//
// based on ds1621.c by Christian W. Zuckschwerdt  <zany@triq.net>
//

//
// Many DS620 constants specified below
// 15   14   13   12   11   10   09    08
// |Done|NVB |THF |TLF |R1  |R0  |AUTOC|1SHOT|
//
// 07   06   05   04   03   02   01    00
// |PO2 |PO1 |A2  |A1  |A0  |    |     |     |
//
pub const DS620_REG_CONFIG_DONE: c_uint = 0x8000;
pub const DS620_REG_CONFIG_NVB: c_uint = 0x4000;
pub const DS620_REG_CONFIG_THF: c_uint = 0x2000;
pub const DS620_REG_CONFIG_TLF: c_uint = 0x1000;
pub const DS620_REG_CONFIG_R1: c_uint = 0x0800;
pub const DS620_REG_CONFIG_R0: c_uint = 0x0400;
pub const DS620_REG_CONFIG_AUTOC: c_uint = 0x0200;
pub const DS620_REG_CONFIG_1SHOT: c_uint = 0x0100;
pub const DS620_REG_CONFIG_PO2: c_uint = 0x0080;
pub const DS620_REG_CONFIG_PO1: c_uint = 0x0040;
pub const DS620_REG_CONFIG_A2: c_uint = 0x0020;
pub const DS620_REG_CONFIG_A1: c_uint = 0x0010;
pub const DS620_REG_CONFIG_A0: c_uint = 0x0008;
// The DS620 registers
    static const u8 DS620_REG_TEMP[3] = {
    0xAA,			/* input, word, RO */
    0xA2,			/* min, word, RW */
    0xA0,			/* max, word, RW */
    };
pub const DS620_REG_CONF: c_uint = 0xAC	/* word, RW */;
pub const DS620_COM_START: c_uint = 0x51	/* no data */;
pub const DS620_COM_STOP: c_uint = 0x22	/* no data */;
// Each client has this additional data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds620_data {
    pub client: *mut i2c_client,
    pub update_lock: mutex,
    pub /: *mut *mut bool valid; / true if following fields are valid,
    pub /: *mut *mut unsigned long last_updated; / In jiffies,
    pub /: *mut *mut s16 temp[3]; / Register values, word,
}

#[no_mangle]
unsafe extern "C" fn ds620_init_client(client: *mut i2c_client) {
    static void ds620_init_client(struct i2c_client *client)
    {
    struct ds620_platform_data *ds620_info = dev_get_platdata(&client.dev);
    u16 conf, new_conf;
    new_conf = conf =
    i2c_smbus_read_word_swapped(client, DS620_REG_CONF);
// switch to continuous conversion mode
    new_conf &= ~DS620_REG_CONFIG_1SHOT;
// already high at power-on, but don't trust the BIOS!
    new_conf |= DS620_REG_CONFIG_PO2;
// thermostat mode according to platform data
    if (ds620_info && ds620_info.pomode == 1)
    new_conf &= ~DS620_REG_CONFIG_PO1; /* PO_LOW */
#[no_mangle]
pub unsafe extern "C" fn if(2: ds620_info && ds620_info->pomode ==) -> else {
    else if (ds620_info && ds620_info.pomode == 2)
    new_conf |= DS620_REG_CONFIG_PO1; /* PO_HIGH */
    else
    new_conf &= ~DS620_REG_CONFIG_PO2; /* always low */
// with highest precision
    new_conf |= DS620_REG_CONFIG_R1 | DS620_REG_CONFIG_R0;
    if (conf != new_conf)
    i2c_smbus_write_word_swapped(client, DS620_REG_CONF, new_conf);
// start conversion
    i2c_smbus_write_byte(client, DS620_COM_START);
    }
    static struct ds620_data *ds620_update_client(struct device *dev)
    {
    struct ds620_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    struct ds620_data *ret = data;
    mutex_lock(&data.update_lock);
    if (time_after(jiffies, data.last_updated + HZ + HZ / 2)
    || !data.valid) {
    int i;
    int res;
    dev_dbg(&client.dev, "Starting ds620 update\n");
    for (i = 0; i < ARRAY_SIZE(data.temp); i++) {
    res = i2c_smbus_read_word_swapped(client,
    DS620_REG_TEMP[i]);
    if (res < 0) {
    ret = ERR_PTR(res);
    goto abort;
    }
    data.temp[i] = res;
    }
    data.last_updated = jiffies;
    data.valid = true;
    }
    abort:
    mutex_unlock(&data.update_lock);
    return ret;
    }
    static ssize_t temp_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct ds620_data *data = ds620_update_client(dev);
    if (IS_ERR(data))
    return PTR_ERR(data);
    return sprintf(buf, "%d\n", ((data.temp[attr.index] / 8) * 625) / 10);
    }
    static ssize_t temp_store(struct device *dev, struct device_attribute *da,
    const char *buf, size_t count)
    {
    int res;
    long val;
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct ds620_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    res = kstrtol(buf, 10, &val);
    if (res)
    return res;
    val = (clamp_val(val, -128000, 128000) * 10 / 625) * 8;
    mutex_lock(&data.update_lock);
    data.temp[attr.index] = val;
    i2c_smbus_write_word_swapped(client, DS620_REG_TEMP[attr.index],
    data.temp[attr.index]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t alarm_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct ds620_data *data = ds620_update_client(dev);
    struct i2c_client *client;
    u16 conf, new_conf;
    int res;
    if (IS_ERR(data))
    return PTR_ERR(data);
    client = data.client;
// reset alarms if necessary
    res = i2c_smbus_read_word_swapped(client, DS620_REG_CONF);
    if (res < 0)
    return res;
    new_conf = conf = res;
    new_conf &= ~attr.index;
    if (conf != new_conf) {
    res = i2c_smbus_write_word_swapped(client, DS620_REG_CONF,
    new_conf);
    if (res < 0)
    return res;
    }
    return sprintf(buf, "%d\n", !!(conf & attr.index));
    }
    static SENSOR_DEVICE_ATTR_RO(temp1_input, temp, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_min, temp, 1);
    static SENSOR_DEVICE_ATTR_RW(temp1_max, temp, 2);
    static SENSOR_DEVICE_ATTR_RO(temp1_min_alarm, alarm, DS620_REG_CONFIG_TLF);
    static SENSOR_DEVICE_ATTR_RO(temp1_max_alarm, alarm, DS620_REG_CONFIG_THF);
    static struct attribute *ds620_attrs[] = {
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_min.dev_attr.attr,
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp1_min_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_max_alarm.dev_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(ds620);
#[no_mangle]
unsafe extern "C" fn ds620_probe(client: *mut i2c_client) -> c_int {
    static int ds620_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct ds620_data *data;
    data = devm_kzalloc(dev, sizeof(struct ds620_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    mutex_init(&data.update_lock);
// Initialize the DS620 chip
    ds620_init_client(client);
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data, ds620_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id ds620_id[] = {
    { .name = "ds620" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ds620_id);
// This is the driver that will be inserted
    static struct i2c_driver ds620_driver = {
    .driver = {
    .name = "ds620",
    },
    .probe = ds620_probe,
    .id_table = ds620_id,
    };
    module_i2c_driver(ds620_driver);
    MODULE_AUTHOR("Roland Stigge <stigge@antcom.de>");
    MODULE_DESCRIPTION("DS620 driver");
    MODULE_LICENSE("GPL");
