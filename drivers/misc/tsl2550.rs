//! Automatically rewritten from C to Rust
//! Source: drivers/misc/tsl2550.c
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
// tsl2550.c - Linux kernel modules for ambient light sensor
//
// Copyright (C) 2007 Rodolfo Giometti <giometti@linux.it>
// Copyright (C) 2007 Eurotech S.p.A. <info@eurotech.it>
//

//
// Defines
//
pub const TSL2550_POWER_DOWN: c_uint = 0x00;
pub const TSL2550_POWER_UP: c_uint = 0x03;
pub const TSL2550_STANDARD_RANGE: c_uint = 0x18;
pub const TSL2550_EXTENDED_RANGE: c_uint = 0x1d;
pub const TSL2550_READ_ADC0: c_uint = 0x43;
pub const TSL2550_READ_ADC1: c_uint = 0x83;
//
// Structs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsl2550_data {
    pub client: *mut i2c_client,
    pub update_lock: mutex,
    pub power_state:1: c_uint,
    pub operating_mode:1: c_uint,
}

//
// Global data
//
    static const u8 TSL2550_MODE_RANGE[2] = {
    TSL2550_STANDARD_RANGE, TSL2550_EXTENDED_RANGE,
    };
//
// Management functions
//
#[no_mangle]
unsafe extern "C" fn tsl2550_set_operating_mode(client: *mut i2c_client, mode: c_int) -> c_int {
    static int tsl2550_set_operating_mode(struct i2c_client *client, int mode)
    {
    struct tsl2550_data *data = i2c_get_clientdata(client);
    let mut ret: c_int = i2c_smbus_write_byte(client, TSL2550_MODE_RANGE[mode]);
    data.operating_mode = mode;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tsl2550_set_power_state(client: *mut i2c_client, state: c_int) -> c_int {
    static int tsl2550_set_power_state(struct i2c_client *client, int state)
    {
    struct tsl2550_data *data = i2c_get_clientdata(client);
    int ret;
    if (state == 0)
    ret = i2c_smbus_write_byte(client, TSL2550_POWER_DOWN);
    else {
    ret = i2c_smbus_write_byte(client, TSL2550_POWER_UP);
// On power up we should reset operating mode also...
    tsl2550_set_operating_mode(client, data.operating_mode);
    }
    data.power_state = state;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tsl2550_get_adc_value(client: *mut i2c_client, cmd: u8) -> c_int {
    static int tsl2550_get_adc_value(struct i2c_client *client, u8 cmd)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(client, cmd);
    if (ret < 0)
    return ret;
    if (!(ret & 0x80))
    return -EAGAIN;
    return ret & 0x7f;	/* remove the "valid" bit */
    }
//
// LUX calculation
//
pub const TSL2550_MAX_LUX: c_int = 1846;
    static const u8 ratio_lut[] = {
    100, 100, 100, 100, 100, 100, 100, 100,
    100, 100, 100, 100, 100, 100, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 98, 98, 98, 98, 98,
    98, 98, 97, 97, 97, 97, 97, 96,
    96, 96, 96, 95, 95, 95, 94, 94,
    93, 93, 93, 92, 92, 91, 91, 90,
    89, 89, 88, 87, 87, 86, 85, 84,
    83, 82, 81, 80, 79, 78, 77, 75,
    74, 73, 71, 69, 68, 66, 64, 62,
    60, 58, 56, 54, 52, 49, 47, 44,
    42, 41, 40, 40, 39, 39, 38, 38,
    37, 37, 37, 36, 36, 36, 35, 35,
    35, 35, 34, 34, 34, 34, 33, 33,
    33, 33, 32, 32, 32, 32, 32, 31,
    31, 31, 31, 31, 30, 30, 30, 30,
    30,
    };
    static const u16 count_lut[] = {
    0, 1, 2, 3, 4, 5, 6, 7,
    8, 9, 10, 11, 12, 13, 14, 15,
    16, 18, 20, 22, 24, 26, 28, 30,
    32, 34, 36, 38, 40, 42, 44, 46,
    49, 53, 57, 61, 65, 69, 73, 77,
    81, 85, 89, 93, 97, 101, 105, 109,
    115, 123, 131, 139, 147, 155, 163, 171,
    179, 187, 195, 203, 211, 219, 227, 235,
    247, 263, 279, 295, 311, 327, 343, 359,
    375, 391, 407, 423, 439, 455, 471, 487,
    511, 543, 575, 607, 639, 671, 703, 735,
    767, 799, 831, 863, 895, 927, 959, 991,
    1039, 1103, 1167, 1231, 1295, 1359, 1423, 1487,
    1551, 1615, 1679, 1743, 1807, 1871, 1935, 1999,
    2095, 2223, 2351, 2479, 2607, 2735, 2863, 2991,
    3119, 3247, 3375, 3503, 3631, 3759, 3887, 4015,
    };
//
// This function is described into Taos TSL2550 Designer's Notebook
// pages 2, 3.
//
#[no_mangle]
unsafe extern "C" fn tsl2550_calculate_lux(ch0: u8, ch1: u8) -> c_int {
    static int tsl2550_calculate_lux(u8 ch0, u8 ch1)
    {
    unsigned int lux;
// Look up count from channel values
    let mut c0: u16 = count_lut[ch0];
    let mut c1: u16 = count_lut[ch1];
// Avoid division by 0 and count 1 cannot be greater than count 0
    if (c1 <= c0)
    if (c0) {
//
// Calculate ratio.
// Note: the "128" is a scaling factor
//
    let mut r: u8 = c1 * 128 / c0;
// Calculate LUX
    lux = ((c0 - c1) * ratio_lut[r]) / 256;
    } else
    lux = 0;
    else
    return 0;
// LUX range check
    return lux > TSL2550_MAX_LUX ? TSL2550_MAX_LUX : lux;
    }
//
// SysFS support
//
    static ssize_t tsl2550_show_power_state(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct tsl2550_data *data = i2c_get_clientdata(to_i2c_client(dev));
    return sprintf(buf, "%u\n", data.power_state);
    }
    static ssize_t tsl2550_store_power_state(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct tsl2550_data *data = i2c_get_clientdata(client);
    unsigned long val;
    int ret;
    if (kstrtoul(buf, 10, &val) || val > 1)
    return -EINVAL;
    mutex_lock(&data.update_lock);
    ret = tsl2550_set_power_state(client, val);
    mutex_unlock(&data.update_lock);
    if (ret < 0)
    return ret;
    return count;
    }
    static DEVICE_ATTR(power_state, S_IWUSR | S_IRUGO,
    tsl2550_show_power_state, tsl2550_store_power_state);
    static ssize_t tsl2550_show_operating_mode(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct tsl2550_data *data = i2c_get_clientdata(to_i2c_client(dev));
    return sprintf(buf, "%u\n", data.operating_mode);
    }
    static ssize_t tsl2550_store_operating_mode(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct tsl2550_data *data = i2c_get_clientdata(client);
    unsigned long val;
    int ret;
    if (kstrtoul(buf, 10, &val) || val > 1)
    return -EINVAL;
    if (data.power_state == 0)
    return -EBUSY;
    mutex_lock(&data.update_lock);
    ret = tsl2550_set_operating_mode(client, val);
    mutex_unlock(&data.update_lock);
    if (ret < 0)
    return ret;
    return count;
    }
    static DEVICE_ATTR(operating_mode, S_IWUSR | S_IRUGO,
    tsl2550_show_operating_mode, tsl2550_store_operating_mode);
#[no_mangle]
unsafe extern "C" fn __tsl2550_show_lux(client: *mut i2c_client, buf: *mut c_char) -> isize {
    static ssize_t __tsl2550_show_lux(struct i2c_client *client, char *buf)
    {
    struct tsl2550_data *data = i2c_get_clientdata(client);
    u8 ch0, ch1;
    int ret;
    ret = tsl2550_get_adc_value(client, TSL2550_READ_ADC0);
    if (ret < 0)
    return ret;
    ch0 = ret;
    ret = tsl2550_get_adc_value(client, TSL2550_READ_ADC1);
    if (ret < 0)
    return ret;
    ch1 = ret;
// Do the job
    ret = tsl2550_calculate_lux(ch0, ch1);
    if (ret < 0)
    return ret;
    if (data.operating_mode == 1)
    ret *= 5;
    return sprintf(buf, "%d\n", ret);
    }
    static ssize_t tsl2550_show_lux1_input(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct tsl2550_data *data = i2c_get_clientdata(client);
    int ret;
// No LUX data if not operational
    if (!data.power_state)
    return -EBUSY;
    mutex_lock(&data.update_lock);
    ret = __tsl2550_show_lux(client, buf);
    mutex_unlock(&data.update_lock);
    return ret;
    }
    static DEVICE_ATTR(lux1_input, S_IRUGO,
    tsl2550_show_lux1_input, core::ptr::null_mut());
    static struct attribute *tsl2550_attributes[] = {
    &dev_attr_power_state.attr,
    &dev_attr_operating_mode.attr,
    &dev_attr_lux1_input.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group tsl2550_attr_group = {
    .attrs = tsl2550_attributes,
    };
//
// Initialization function
//
#[no_mangle]
unsafe extern "C" fn tsl2550_init_client(client: *mut i2c_client) -> c_int {
    static int tsl2550_init_client(struct i2c_client *client)
    {
    struct tsl2550_data *data = i2c_get_clientdata(client);
    int err;
//
// Probe the chip. To do so we try to power up the device and then to
// read back the 0x03 code
//
    err = i2c_smbus_read_byte_data(client, TSL2550_POWER_UP);
    if (err < 0)
    return err;
    if (err != TSL2550_POWER_UP)
    return -ENODEV;
    data.power_state = 1;
// Set the default operating mode
    err = i2c_smbus_write_byte(client,
    TSL2550_MODE_RANGE[data.operating_mode]);
    if (err < 0)
    return err;
    return 0;
    }
//
// I2C init/probing/exit functions
//
    static struct i2c_driver tsl2550_driver;
#[no_mangle]
unsafe extern "C" fn tsl2550_probe(client: *mut i2c_client) -> c_int {
    static int tsl2550_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct tsl2550_data *data;
    int *opmode, err = 0;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_WRITE_BYTE
    | I2C_FUNC_SMBUS_READ_BYTE_DATA)) {
    err = -EIO;
    goto exit;
    }
    data = kzalloc_obj(struct tsl2550_data);
    if (!data) {
    err = -ENOMEM;
    goto exit;
    }
    data.client = client;
    i2c_set_clientdata(client, data);
// Check platform data
    opmode = client.dev.platform_data;
    if (opmode) {
    if (*opmode < 0 || *opmode > 1) {
    dev_err(&client.dev, "invalid operating_mode (%d)\n",
// opmode);
    err = -EINVAL;
    goto exit_kfree;
    }
    data.operating_mode = *opmode;
    } else
    data.operating_mode = 0;	/* default mode is standard */
    dev_info(&client.dev, "%s operating mode\n",
    data.operating_mode ? "extended" : "standard");
    mutex_init(&data.update_lock);
// Initialize the TSL2550 chip
    err = tsl2550_init_client(client);
    if (err)
    goto exit_kfree;
// Register sysfs hooks
    err = sysfs_create_group(&client.dev.kobj, &tsl2550_attr_group);
    if (err)
    goto exit_kfree;
    dev_info(&client.dev, "support ver. %s enabled\n", DRIVER_VERSION);
    return 0;
    exit_kfree:
    kfree(data);
    exit:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tsl2550_remove(client: *mut i2c_client) {
    static void tsl2550_remove(struct i2c_client *client)
    {
    sysfs_remove_group(&client.dev.kobj, &tsl2550_attr_group);
// Power down the device
    tsl2550_set_power_state(client, 0);
    kfree(i2c_get_clientdata(client));
    }

#[no_mangle]
unsafe extern "C" fn tsl2550_suspend(dev: *mut device) -> c_int {
    static int tsl2550_suspend(struct device *dev)
    {
    return tsl2550_set_power_state(to_i2c_client(dev), 0);
    }
#[no_mangle]
unsafe extern "C" fn tsl2550_resume(dev: *mut device) -> c_int {
    static int tsl2550_resume(struct device *dev)
    {
    return tsl2550_set_power_state(to_i2c_client(dev), 1);
    }
    static SIMPLE_DEV_PM_OPS(tsl2550_pm_ops, tsl2550_suspend, tsl2550_resume);

    static const struct i2c_device_id tsl2550_id[] = {
    { .name = "tsl2550" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tsl2550_id);
    static const struct of_device_id tsl2550_of_match[] = {
    { .compatible = "taos,tsl2550" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tsl2550_of_match);
    static struct i2c_driver tsl2550_driver = {
    .driver = {
    .name	= TSL2550_DRV_NAME,
    .of_match_table = tsl2550_of_match,
    .pm	= TSL2550_PM_OPS,
    },
    .probe = tsl2550_probe,
    .remove	= tsl2550_remove,
    .id_table = tsl2550_id,
    };
    module_i2c_driver(tsl2550_driver);
    MODULE_AUTHOR("Rodolfo Giometti <giometti@linux.it>");
    MODULE_DESCRIPTION("TSL2550 ambient light sensor driver");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(DRIVER_VERSION);
