//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/tc74.c
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
// An hwmon driver for the Microchip TC74
//
// Copyright 2015 Maciej Szmigiero <mail@maciej.szmigiero.name>
//
// Based on ad7414.c:
// Copyright 2006 Stefan Roese, DENX Software Engineering
// Copyright 2008 Sean MacLennan, PIKA Technologies
// Copyright 2008 Frank Edelhaeuser, Spansion Inc.
//

// TC74 registers
pub const TC74_REG_TEMP: c_uint = 0x00;
pub const TC74_REG_CONFIG: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc74_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut mutex lock; / atomic read data updates,
    pub /: *mut *mut bool valid; / validity of fields below,
    pub /: *mut *mut unsigned long next_update; / In jiffies,
    pub /: *mut *mut s8 temp_input; / Temp value in dC,
}

#[no_mangle]
unsafe extern "C" fn tc74_update_device(dev: *mut device) -> c_int {
    static int tc74_update_device(struct device *dev)
    {
    struct tc74_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret;
    ret = mutex_lock_interruptible(&data.lock);
    if (ret)
    return ret;
    if (time_after(jiffies, data.next_update) || !data.valid) {
    s32 value;
    value = i2c_smbus_read_byte_data(client, TC74_REG_CONFIG);
    if (value < 0) {
    dev_dbg(&client.dev, "TC74_REG_CONFIG read err %d\n",
    (int)value);
    ret = value;
    goto ret_unlock;
    }
    if (!(value & BIT(6))) {
// not ready yet
    ret = -EAGAIN;
    goto ret_unlock;
    }
    value = i2c_smbus_read_byte_data(client, TC74_REG_TEMP);
    if (value < 0) {
    dev_dbg(&client.dev, "TC74_REG_TEMP read err %d\n",
    (int)value);
    ret = value;
    goto ret_unlock;
    }
    data.temp_input = value;
    data.next_update = jiffies + HZ / 4;
    data.valid = true;
    }
    ret_unlock:
    mutex_unlock(&data.lock);
    return ret;
    }
    static ssize_t temp_input_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct tc74_data *data = dev_get_drvdata(dev);
    int ret;
    ret = tc74_update_device(dev);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%d\n", data.temp_input * 1000);
    }
    static SENSOR_DEVICE_ATTR_RO(temp1_input, temp_input, 0);
    static struct attribute *tc74_attrs[] = {
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(tc74);
#[no_mangle]
unsafe extern "C" fn tc74_probe(client: *mut i2c_client) -> c_int {
    static int tc74_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct tc74_data *data;
    struct device *hwmon_dev;
    s32 conf;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EOPNOTSUPP;
    data = devm_kzalloc(dev, sizeof(struct tc74_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    mutex_init(&data.lock);
// Make sure the chip is powered up.
    conf = i2c_smbus_read_byte_data(client, TC74_REG_CONFIG);
    if (conf < 0) {
    dev_err(dev, "unable to read config register\n");
    return conf;
    }
    if (conf & 0x3f) {
    dev_err(dev, "invalid config register value\n");
    return -ENODEV;
    }
    if (conf & BIT(7)) {
    s32 ret;
    conf &= ~BIT(7);
    ret = i2c_smbus_write_byte_data(client, TC74_REG_CONFIG, conf);
    if (ret)
    dev_warn(dev, "unable to disable STANDBY\n");
    }
    hwmon_dev = devm_hwmon_device_register_with_groups(dev,
    client.name,
    data, tc74_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id tc74_id[] = {
    { .name = "tc74" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tc74_id);
    static struct i2c_driver tc74_driver = {
    .driver = {
    .name	= "tc74",
    },
    .probe = tc74_probe,
    .id_table = tc74_id,
    };
    module_i2c_driver(tc74_driver);
    MODULE_AUTHOR("Maciej Szmigiero <mail@maciej.szmigiero.name>");
    MODULE_DESCRIPTION("TC74 driver");
    MODULE_LICENSE("GPL");
