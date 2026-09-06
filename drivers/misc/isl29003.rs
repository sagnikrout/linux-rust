//! Automatically rewritten from C to Rust
//! Source: drivers/misc/isl29003.c
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
// isl29003.c - Linux kernel module for
// Intersil ISL29003 ambient light sensor
//
// See file:Documentation/misc-devices/isl29003.rst
//
// Copyright (c) 2009 Daniel Mack <daniel@caiaq.de>
//
// Based on code written by
// Rodolfo Giometti <giometti@linux.it>
// Eurotech S.p.A. <info@eurotech.it>
//

pub const ISL29003_REG_COMMAND: c_uint = 0x00;

pub const ISL29003_REG_CONTROL: c_uint = 0x01;

pub const ISL29003_REG_IRQ_THRESH_HI: c_uint = 0x02;
pub const ISL29003_REG_IRQ_THRESH_LO: c_uint = 0x03;
pub const ISL29003_REG_LSB_SENSOR: c_uint = 0x04;
pub const ISL29003_REG_MSB_SENSOR: c_uint = 0x05;
pub const ISL29003_REG_LSB_TIMER: c_uint = 0x06;
pub const ISL29003_REG_MSB_TIMER: c_uint = 0x07;
pub const ISL29003_NUM_CACHABLE_REGS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl29003_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub reg_cache: [u8; ISL29003_NUM_CACHABLE_REGS],
    pub power_state_before_suspend: u8,
}

    static int gain_range[] = {
    1000, 4000, 16000, 64000
    };
//
// register access helpers
//
    static int __isl29003_read_reg(struct i2c_client *client,
    u32 reg, u8 mask, u8 shift)
    {
    struct isl29003_data *data = i2c_get_clientdata(client);
    return (data.reg_cache[reg] & mask) >> shift;
    }
    static int __isl29003_write_reg(struct i2c_client *client,
    u32 reg, u8 mask, u8 shift, u8 val)
    {
    struct isl29003_data *data = i2c_get_clientdata(client);
    let mut ret: c_int = 0;
    u8 tmp;
    if (reg >= ISL29003_NUM_CACHABLE_REGS)
    return -EINVAL;
    mutex_lock(&data.lock);
    tmp = data.reg_cache[reg];
    tmp &= ~mask;
    tmp |= val << shift;
    ret = i2c_smbus_write_byte_data(client, reg, tmp);
    if (!ret)
    data.reg_cache[reg] = tmp;
    mutex_unlock(&data.lock);
    return ret;
    }
//
// internally used functions
//
// range
#[no_mangle]
unsafe extern "C" fn isl29003_get_range(client: *mut i2c_client) -> c_int {
    static int isl29003_get_range(struct i2c_client *client)
    {
    return __isl29003_read_reg(client, ISL29003_REG_CONTROL,
    ISL29003_RANGE_MASK, ISL29003_RANGE_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn isl29003_set_range(client: *mut i2c_client, range: c_int) -> c_int {
    static int isl29003_set_range(struct i2c_client *client, int range)
    {
    return __isl29003_write_reg(client, ISL29003_REG_CONTROL,
    ISL29003_RANGE_MASK, ISL29003_RANGE_SHIFT, range);
    }
// resolution
#[no_mangle]
unsafe extern "C" fn isl29003_get_resolution(client: *mut i2c_client) -> c_int {
    static int isl29003_get_resolution(struct i2c_client *client)
    {
    return __isl29003_read_reg(client, ISL29003_REG_COMMAND,
    ISL29003_RES_MASK, ISL29003_RES_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn isl29003_set_resolution(client: *mut i2c_client, res: c_int) -> c_int {
    static int isl29003_set_resolution(struct i2c_client *client, int res)
    {
    return __isl29003_write_reg(client, ISL29003_REG_COMMAND,
    ISL29003_RES_MASK, ISL29003_RES_SHIFT, res);
    }
// mode
#[no_mangle]
unsafe extern "C" fn isl29003_get_mode(client: *mut i2c_client) -> c_int {
    static int isl29003_get_mode(struct i2c_client *client)
    {
    return __isl29003_read_reg(client, ISL29003_REG_COMMAND,
    ISL29003_MODE_MASK, ISL29003_MODE_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn isl29003_set_mode(client: *mut i2c_client, mode: c_int) -> c_int {
    static int isl29003_set_mode(struct i2c_client *client, int mode)
    {
    return __isl29003_write_reg(client, ISL29003_REG_COMMAND,
    ISL29003_MODE_MASK, ISL29003_MODE_SHIFT, mode);
    }
// power_state
#[no_mangle]
unsafe extern "C" fn isl29003_set_power_state(client: *mut i2c_client, state: c_int) -> c_int {
    static int isl29003_set_power_state(struct i2c_client *client, int state)
    {
    return __isl29003_write_reg(client, ISL29003_REG_COMMAND,
    ISL29003_ADC_ENABLED | ISL29003_ADC_PD, 0,
    state ? ISL29003_ADC_ENABLED : ISL29003_ADC_PD);
    }
#[no_mangle]
unsafe extern "C" fn isl29003_get_power_state(client: *mut i2c_client) -> c_int {
    static int isl29003_get_power_state(struct i2c_client *client)
    {
    struct isl29003_data *data = i2c_get_clientdata(client);
    let mut cmdreg: u8 = data.reg_cache[ISL29003_REG_COMMAND];
    return ~cmdreg & ISL29003_ADC_PD;
    }
#[no_mangle]
unsafe extern "C" fn isl29003_get_adc_value(client: *mut i2c_client) -> c_int {
    static int isl29003_get_adc_value(struct i2c_client *client)
    {
    struct isl29003_data *data = i2c_get_clientdata(client);
    int lsb, msb, range, bitdepth;
    mutex_lock(&data.lock);
    lsb = i2c_smbus_read_byte_data(client, ISL29003_REG_LSB_SENSOR);
    if (lsb < 0) {
    mutex_unlock(&data.lock);
    return lsb;
    }
    msb = i2c_smbus_read_byte_data(client, ISL29003_REG_MSB_SENSOR);
    mutex_unlock(&data.lock);
    if (msb < 0)
    return msb;
    range = isl29003_get_range(client);
    bitdepth = (4 - isl29003_get_resolution(client)) * 4;
    return (((msb << 8) | lsb) * gain_range[range]) >> bitdepth;
    }
//
// sysfs layer
//
// range
    static ssize_t isl29003_show_range(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    return sysfs_emit(buf, "%i\n", isl29003_get_range(client));
    }
    static ssize_t isl29003_store_range(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    unsigned long val;
    int ret;
    ret = kstrtoul(buf, 10, &val);
    if (ret)
    return ret;
    if (val > 3)
    return -EINVAL;
    ret = isl29003_set_range(client, val);
    if (ret < 0)
    return ret;
    return count;
    }
    static DEVICE_ATTR(range, S_IWUSR | S_IRUGO,
    isl29003_show_range, isl29003_store_range);
// resolution
    static ssize_t isl29003_show_resolution(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    return sysfs_emit(buf, "%d\n", isl29003_get_resolution(client));
    }
    static ssize_t isl29003_store_resolution(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    unsigned long val;
    int ret;
    ret = kstrtoul(buf, 10, &val);
    if (ret)
    return ret;
    if (val > 3)
    return -EINVAL;
    ret = isl29003_set_resolution(client, val);
    if (ret < 0)
    return ret;
    return count;
    }
    static DEVICE_ATTR(resolution, S_IWUSR | S_IRUGO,
    isl29003_show_resolution, isl29003_store_resolution);
// mode
    static ssize_t isl29003_show_mode(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    return sysfs_emit(buf, "%d\n", isl29003_get_mode(client));
    }
    static ssize_t isl29003_store_mode(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    unsigned long val;
    int ret;
    ret = kstrtoul(buf, 10, &val);
    if (ret)
    return ret;
    if (val > 2)
    return -EINVAL;
    ret = isl29003_set_mode(client, val);
    if (ret < 0)
    return ret;
    return count;
    }
    static DEVICE_ATTR(mode, S_IWUSR | S_IRUGO,
    isl29003_show_mode, isl29003_store_mode);
// power state
    static ssize_t isl29003_show_power_state(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    return sysfs_emit(buf, "%d\n", isl29003_get_power_state(client));
    }
    static ssize_t isl29003_store_power_state(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    unsigned long val;
    int ret;
    ret = kstrtoul(buf, 10, &val);
    if (ret)
    return ret;
    if (val > 1)
    return -EINVAL;
    ret = isl29003_set_power_state(client, val);
    return ret ? ret : count;
    }
    static DEVICE_ATTR(power_state, S_IWUSR | S_IRUGO,
    isl29003_show_power_state, isl29003_store_power_state);
// lux
    static ssize_t isl29003_show_lux(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
// No LUX data if not operational
    if (!isl29003_get_power_state(client))
    return -EBUSY;
    return sysfs_emit(buf, "%d\n", isl29003_get_adc_value(client));
    }
    static DEVICE_ATTR(lux, S_IRUGO, isl29003_show_lux, core::ptr::null_mut());
    static struct attribute *isl29003_attributes[] = {
    &dev_attr_range.attr,
    &dev_attr_resolution.attr,
    &dev_attr_mode.attr,
    &dev_attr_power_state.attr,
    &dev_attr_lux.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group isl29003_attr_group = {
    .attrs = isl29003_attributes,
    };
#[no_mangle]
unsafe extern "C" fn isl29003_init_client(client: *mut i2c_client) -> c_int {
    static int isl29003_init_client(struct i2c_client *client)
    {
    struct isl29003_data *data = i2c_get_clientdata(client);
    int i;
// read all the registers once to fill the cache.
// if one of the reads fails, we consider the init failed
    for (i = 0; i < ARRAY_SIZE(data.reg_cache); i++) {
    let mut v: c_int = i2c_smbus_read_byte_data(client, i);
    if (v < 0)
    return -ENODEV;
    data.reg_cache[i] = v;
    }
// set defaults
    isl29003_set_range(client, 0);
    isl29003_set_resolution(client, 0);
    isl29003_set_mode(client, 0);
    isl29003_set_power_state(client, 0);
    return 0;
    }
//
// I2C layer
//
#[no_mangle]
unsafe extern "C" fn isl29003_probe(client: *mut i2c_client) -> c_int {
    static int isl29003_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct isl29003_data *data;
    let mut err: c_int = 0;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE))
    return -EIO;
    data = kzalloc_obj(struct isl29003_data);
    if (!data)
    return -ENOMEM;
    data.client = client;
    i2c_set_clientdata(client, data);
    mutex_init(&data.lock);
// initialize the ISL29003 chip
    err = isl29003_init_client(client);
    if (err)
    goto exit_kfree;
// register sysfs hooks
    err = sysfs_create_group(&client.dev.kobj, &isl29003_attr_group);
    if (err)
    goto exit_kfree;
    dev_info(&client.dev, "driver version %s enabled\n", DRIVER_VERSION);
    return 0;
    exit_kfree:
    kfree(data);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn isl29003_remove(client: *mut i2c_client) {
    static void isl29003_remove(struct i2c_client *client)
    {
    sysfs_remove_group(&client.dev.kobj, &isl29003_attr_group);
    isl29003_set_power_state(client, 0);
    kfree(i2c_get_clientdata(client));
    }

#[no_mangle]
unsafe extern "C" fn isl29003_suspend(dev: *mut device) -> c_int {
    static int isl29003_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct isl29003_data *data = i2c_get_clientdata(client);
    data.power_state_before_suspend = isl29003_get_power_state(client);
    return isl29003_set_power_state(client, 0);
    }
#[no_mangle]
unsafe extern "C" fn isl29003_resume(dev: *mut device) -> c_int {
    static int isl29003_resume(struct device *dev)
    {
    int i;
    struct i2c_client *client = to_i2c_client(dev);
    struct isl29003_data *data = i2c_get_clientdata(client);
// restore registers from cache
    for (i = 0; i < ARRAY_SIZE(data.reg_cache); i++)
    if (i2c_smbus_write_byte_data(client, i, data.reg_cache[i]))
    return -EIO;
    return isl29003_set_power_state(client,
    data.power_state_before_suspend);
    }
    static SIMPLE_DEV_PM_OPS(isl29003_pm_ops, isl29003_suspend, isl29003_resume);

    static const struct i2c_device_id isl29003_id[] = {
    { .name = "isl29003" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, isl29003_id);
    static struct i2c_driver isl29003_driver = {
    .driver = {
    .name	= ISL29003_DRV_NAME,
    .pm	= ISL29003_PM_OPS,
    },
    .probe = isl29003_probe,
    .remove	= isl29003_remove,
    .id_table = isl29003_id,
    };
    module_i2c_driver(isl29003_driver);
    MODULE_AUTHOR("Daniel Mack <daniel@caiaq.de>");
    MODULE_DESCRIPTION("ISL29003 ambient light sensor driver");
    MODULE_LICENSE("GPL v2");
    MODULE_VERSION(DRIVER_VERSION);
