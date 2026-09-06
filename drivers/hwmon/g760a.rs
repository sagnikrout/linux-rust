//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/g760a.c
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
// g760a - Driver for the Global Mixed-mode Technology Inc. G760A
// fan speed PWM controller chip
//
// Copyright (C) 2007  Herbert Valerio Riedel <hvr@gnu.org>
//
// Complete datasheet is available at GMT's website:
// http://www.gmt.com.tw/product/datasheet/EDS-760A.pdf
//

    enum g760a_regs {
    G760A_REG_SET_CNT = 0x00,
    G760A_REG_ACT_CNT = 0x01,
    G760A_REG_FAN_STA = 0x02
    };
pub const G760A_REG_FAN_STA_RPM_OFF: c_uint = 0x1 /* +/-20% off */;
pub const G760A_REG_FAN_STA_RPM_LOW: c_uint = 0x2 /* below 1920rpm */;
// register data is read (and cached) at most once per second

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g760a_data {
    pub client: *mut i2c_client,
    pub update_lock: mutex,
// board specific parameters
    pub /: *mut *mut u32 clk; / default 32kHz,
    pub /: *mut *mut u16 fan_div; / default P=2,
// g760a register cache
    pub valid:1: c_uint,
    pub /: *mut *mut unsigned long last_updated; / In jiffies,
    pub /: *mut *mut u8 set_cnt; / PWM (period) count number; 0xff stops fan,
    pub /: *mut *mut *mut *mut u8 act_cnt; / formula: cnt = (CLK  30)/(rpm  P),
    pub 20%: *mut *mut u8 fan_sta; / bit 0: set when actual fan speed more than,
// outside requested fan speed
// bit 1: set when fan speed below 1920 rpm
//
}

pub const G760A_DEFAULT_CLK: c_int = 32768;
pub const G760A_DEFAULT_FAN_DIV: c_int = 2;

#[no_mangle]
pub unsafe extern "C" fn rpm_from_cnt(val: u8, clk: u32, div: u16) -> c_uint {
    static inline unsigned int rpm_from_cnt(u8 val, u32 clk, u16 div)
    {
    return ((val == 0x00) ? 0 : ((clk*30)/(val*div)));
    }
// read/write wrappers
#[no_mangle]
unsafe extern "C" fn g760a_read_value(client: *mut i2c_client, reg: enum g760a_regs) -> c_int {
    static int g760a_read_value(struct i2c_client *client, enum g760a_regs reg)
    {
    return i2c_smbus_read_byte_data(client, reg);
    }
    static int g760a_write_value(struct i2c_client *client, enum g760a_regs reg,
    u16 value)
    {
    return i2c_smbus_write_byte_data(client, reg, value);
    }
//
// sysfs attributes
//
    static struct g760a_data *g760a_update_client(struct device *dev)
    {
    struct g760a_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    mutex_lock(&data.update_lock);
    if (time_after(jiffies, data.last_updated + G760A_UPDATE_INTERVAL)
    || !data.valid) {
    dev_dbg(&client.dev, "Starting g760a update\n");
    data.set_cnt = g760a_read_value(client, G760A_REG_SET_CNT);
    data.act_cnt = g760a_read_value(client, G760A_REG_ACT_CNT);
    data.fan_sta = g760a_read_value(client, G760A_REG_FAN_STA);
    data.last_updated = jiffies;
    data.valid = true;
    }
    mutex_unlock(&data.update_lock);
    return data;
    }
    static ssize_t fan1_input_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct g760a_data *data = g760a_update_client(dev);
    let mut rpm: c_uint = 0;
    mutex_lock(&data.update_lock);
    if (!(data.fan_sta & G760A_REG_FAN_STA_RPM_LOW))
    rpm = rpm_from_cnt(data.act_cnt, data.clk, data.fan_div);
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "%d\n", rpm);
    }
    static ssize_t fan1_alarm_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct g760a_data *data = g760a_update_client(dev);
    let mut fan_alarm: c_int = (data.fan_sta & G760A_REG_FAN_STA_RPM_OFF) ? 1 : 0;
    return sprintf(buf, "%d\n", fan_alarm);
    }
    static ssize_t pwm1_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct g760a_data *data = g760a_update_client(dev);
    return sprintf(buf, "%d\n", PWM_FROM_CNT(data.set_cnt));
    }
    static ssize_t pwm1_store(struct device *dev, struct device_attribute *da,
    const char *buf, size_t count)
    {
    struct g760a_data *data = g760a_update_client(dev);
    struct i2c_client *client = data.client;
    unsigned long val;
    if (kstrtoul(buf, 10, &val))
    return -EINVAL;
    mutex_lock(&data.update_lock);
    data.set_cnt = PWM_TO_CNT(clamp_val(val, 0, 255));
    g760a_write_value(client, G760A_REG_SET_CNT, data.set_cnt);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static DEVICE_ATTR_RW(pwm1);
    static DEVICE_ATTR_RO(fan1_input);
    static DEVICE_ATTR_RO(fan1_alarm);
    static struct attribute *g760a_attrs[] = {
    &dev_attr_pwm1.attr,
    &dev_attr_fan1_input.attr,
    &dev_attr_fan1_alarm.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(g760a);
//
// new-style driver model code
//
#[no_mangle]
unsafe extern "C" fn g760a_probe(client: *mut i2c_client) -> c_int {
    static int g760a_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct g760a_data *data;
    struct device *hwmon_dev;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    data = devm_kzalloc(dev, sizeof(struct g760a_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    mutex_init(&data.update_lock);
// setup default configuration for now
    data.fan_div = G760A_DEFAULT_FAN_DIV;
    data.clk = G760A_DEFAULT_CLK;
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data,
    g760a_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id g760a_id[] = {
    { .name = "g760a" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, g760a_id);
    static struct i2c_driver g760a_driver = {
    .driver = {
    .name	= "g760a",
    },
    .probe = g760a_probe,
    .id_table = g760a_id,
    };
    module_i2c_driver(g760a_driver);
    MODULE_AUTHOR("Herbert Valerio Riedel <hvr@gnu.org>");
    MODULE_DESCRIPTION("GMT G760A driver");
    MODULE_LICENSE("GPL");
