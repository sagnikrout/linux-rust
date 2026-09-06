//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/zopt2201.c
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
// zopt2201.c - Support for IDT ZOPT2201 ambient light and UV B sensor
//
// Copyright 2017 Peter Meerwald-Stadler <pmeerw@pmeerw.net>
//
// Datasheet: https://www.idt.com/document/dst/zopt2201-datasheet
// 7-bit I2C slave addresses 0x53 (default) or 0x52 (programmed)
//
// TODO: interrupt support, ALS/UVB raw mode
//

// Registers
pub const ZOPT2201_MAIN_CTRL: c_uint = 0x00;
pub const ZOPT2201_LS_MEAS_RATE: c_uint = 0x04;
pub const ZOPT2201_LS_GAIN: c_uint = 0x05;
pub const ZOPT2201_PART_ID: c_uint = 0x06;
pub const ZOPT2201_MAIN_STATUS: c_uint = 0x07;
pub const ZOPT2201_ALS_DATA: c_uint = 0x0d /* LSB first, 13 to 20 bits */;
pub const ZOPT2201_UVB_DATA: c_uint = 0x10 /* LSB first, 13 to 20 bits */;
pub const ZOPT2201_UV_COMP_DATA: c_uint = 0x13 /* LSB first, 13 to 20 bits */;
pub const ZOPT2201_COMP_DATA: c_uint = 0x16 /* LSB first, 13 to 20 bits */;
pub const ZOPT2201_INT_CFG: c_uint = 0x19;
pub const ZOPT2201_INT_PST: c_uint = 0x1a;

// Values for ZOPT2201_LS_MEAS_RATE resolution / bit width

pub const ZOPT2201_MEAS_RES_SHIFT: c_int = 4;
// Values for ZOPT2201_LS_MEAS_RATE measurement rate
pub const ZOPT2201_MEAS_FREQ_25MS: c_int = 0;
pub const ZOPT2201_MEAS_FREQ_50MS: c_int = 1;

pub const ZOPT2201_MEAS_FREQ_200MS: c_int = 3;
pub const ZOPT2201_MEAS_FREQ_500MS: c_int = 4;
pub const ZOPT2201_MEAS_FREQ_1000MS: c_int = 5;
pub const ZOPT2201_MEAS_FREQ_2000MS: c_int = 6;
// Values for ZOPT2201_LS_GAIN
pub const ZOPT2201_LS_GAIN_1: c_int = 0;
pub const ZOPT2201_LS_GAIN_3: c_int = 1;
pub const ZOPT2201_LS_GAIN_6: c_int = 2;
pub const ZOPT2201_LS_GAIN_9: c_int = 3;
pub const ZOPT2201_LS_GAIN_18: c_int = 4;
// Values for ZOPT2201_MAIN_STATUS

pub const ZOPT2201_PART_NUMBER: c_uint = 0xb2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zopt2201_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub gain: u8,
    pub res: u8,
    pub rate: u8,
}

    static const struct {
    unsigned int gain; /* gain factor */
    unsigned int scale; /* micro lux per count */
    } zopt2201_gain_als[] = {
    {  1, 19200000 },
    {  3,  6400000 },
    {  6,  3200000 },
    {  9,  2133333 },
    { 18,  1066666 },
    };
    static const struct {
    unsigned int gain; /* gain factor */
    unsigned int scale; /* micro W/m2 per count */
    } zopt2201_gain_uvb[] = {
    {  1, 460800 },
    {  3, 153600 },
    {  6,  76800 },
    {  9,  51200 },
    { 18,  25600 },
    };
    static const struct {
    unsigned int bits; /* sensor resolution in bits */
    unsigned long us; /* measurement time in micro seconds */
    } zopt2201_resolution[] = {
    { 20, 400000 },
    { 19, 200000 },
    { 18, 100000 },
    { 17,  50000 },
    { 16,  25000 },
    { 13,   3125 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zopt2201_scale {
    pub /: *mut *mut unsigned int scale, uscale; / scale factor as integer + micro,
    pub /: *mut *mut u8 gain; / gain register value,
    pub /: *mut *mut u8 res; / resolution register value,
}

    static const struct zopt2201_scale zopt2201_scale_als[] = {
    { 19, 200000, 0, 5 },
    {  6, 400000, 1, 5 },
    {  3, 200000, 2, 5 },
    {  2, 400000, 0, 4 },
    {  2, 133333, 3, 5 },
    {  1, 200000, 0, 3 },
    {  1,  66666, 4, 5 },
    {  0, 800000, 1, 4 },
    {  0, 600000, 0, 2 },
    {  0, 400000, 2, 4 },
    {  0, 300000, 0, 1 },
    {  0, 266666, 3, 4 },
    {  0, 200000, 2, 3 },
    {  0, 150000, 0, 0 },
    {  0, 133333, 4, 4 },
    {  0, 100000, 2, 2 },
    {  0,  66666, 4, 3 },
    {  0,  50000, 2, 1 },
    {  0,  33333, 4, 2 },
    {  0,  25000, 2, 0 },
    {  0,  16666, 4, 1 },
    {  0,   8333, 4, 0 },
    };
    static const struct zopt2201_scale zopt2201_scale_uvb[] = {
    { 0, 460800, 0, 5 },
    { 0, 153600, 1, 5 },
    { 0,  76800, 2, 5 },
    { 0,  57600, 0, 4 },
    { 0,  51200, 3, 5 },
    { 0,  28800, 0, 3 },
    { 0,  25600, 4, 5 },
    { 0,  19200, 1, 4 },
    { 0,  14400, 0, 2 },
    { 0,   9600, 2, 4 },
    { 0,   7200, 0, 1 },
    { 0,   6400, 3, 4 },
    { 0,   4800, 2, 3 },
    { 0,   3600, 0, 0 },
    { 0,   3200, 4, 4 },
    { 0,   2400, 2, 2 },
    { 0,   1600, 4, 3 },
    { 0,   1200, 2, 1 },
    { 0,    800, 4, 2 },
    { 0,    600, 2, 0 },
    { 0,    400, 4, 1 },
    { 0,    200, 4, 0 },
    };
#[no_mangle]
unsafe extern "C" fn zopt2201_enable_mode(data: *mut zopt2201_data, uvb_mode: bool) -> c_int {
    static int zopt2201_enable_mode(struct zopt2201_data *data, bool uvb_mode)
    {
    let mut out: u8 = ZOPT2201_MAIN_CTRL_LS_EN;
    if (uvb_mode)
    out |= ZOPT2201_MAIN_CTRL_LS_MODE;
    return i2c_smbus_write_byte_data(data.client, ZOPT2201_MAIN_CTRL, out);
    }
#[no_mangle]
unsafe extern "C" fn zopt2201_read(data: *mut zopt2201_data, reg: u8) -> c_int {
    static int zopt2201_read(struct zopt2201_data *data, u8 reg)
    {
    struct i2c_client *client = data.client;
    let mut tries: c_int = 10;
    u8 buf[3];
    int ret;
    guard(mutex)(&data.lock);
    ret = zopt2201_enable_mode(data, reg == ZOPT2201_UVB_DATA);
    if (ret < 0)
    return ret;
    while (tries--) {
    let mut t: c_ulong = zopt2201_resolution[data.res].us;
    if (t <= 20000)
    usleep_range(t, t + 1000);
    else
    msleep(t / 1000);
    ret = i2c_smbus_read_byte_data(client, ZOPT2201_MAIN_STATUS);
    if (ret < 0)
    return ret;
    if (ret & ZOPT2201_MAIN_STATUS_DRDY)
    break;
    }
    if (tries < 0) {
    ret = -ETIMEDOUT;
    return ret;
    }
    ret = i2c_smbus_read_i2c_block_data(client, reg, sizeof(buf), buf);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(client, ZOPT2201_MAIN_CTRL, 0x00);
    if (ret < 0)
    return ret;
    return get_unaligned_le24(&buf[0]);
    }
    static const struct iio_chan_spec zopt2201_channels[] = {
    {
    .type = IIO_LIGHT,
    .address = ZOPT2201_ALS_DATA,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    },
    {
    .type = IIO_INTENSITY,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_UV,
    .address = ZOPT2201_UVB_DATA,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    },
    {
    .type = IIO_UVINDEX,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    },
    };
    static int zopt2201_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct zopt2201_data *data = iio_priv(indio_dev);
    u64 tmp;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = zopt2201_read(data, chan.address);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_PROCESSED:
    ret = zopt2201_read(data, ZOPT2201_UVB_DATA);
    if (ret < 0)
    return ret;
// val = ret * 18
    (1 << (20 - zopt2201_resolution[data.res].bits)) /
    zopt2201_gain_uvb[data.gain].gain;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.address) {
    case ZOPT2201_ALS_DATA:
// val = zopt2201_gain_als[data->gain].scale;
    break;
    case ZOPT2201_UVB_DATA:
// val = zopt2201_gain_uvb[data->gain].scale;
    break;
    default:
    return -EINVAL;
    }
// val2 = 1000000;
// val2 *= (1 << (zopt2201_resolution[data->res].bits - 13));
    tmp = div_s64(*val * 1000000ULL, *val2);
// val = div_s64_rem(tmp, 1000000, val2);
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_INT_TIME:
// val = 0;
// val2 = zopt2201_resolution[data->res].us;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn zopt2201_set_resolution(data: *mut zopt2201_data, res: u8) -> c_int {
    static int zopt2201_set_resolution(struct zopt2201_data *data, u8 res)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(data.client, ZOPT2201_LS_MEAS_RATE,
    (res << ZOPT2201_MEAS_RES_SHIFT) |
    data.rate);
    if (ret < 0)
    return ret;
    data.res = res;
    return 0;
    }
    static int zopt2201_write_resolution(struct zopt2201_data *data,
    int val, int val2)
    {
    int i;
    if (val != 0)
    return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(zopt2201_resolution); i++)
    if (val2 == zopt2201_resolution[i].us) {
    guard(mutex)(&data.lock);
    return zopt2201_set_resolution(data, i);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn zopt2201_set_gain(data: *mut zopt2201_data, gain: u8) -> c_int {
    static int zopt2201_set_gain(struct zopt2201_data *data, u8 gain)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(data.client, ZOPT2201_LS_GAIN, gain);
    if (ret < 0)
    return ret;
    data.gain = gain;
    return 0;
    }
    static int zopt2201_write_scale_by_idx(struct zopt2201_data *data, int idx,
    const struct zopt2201_scale *zopt2201_scale_array)
    {
    int ret;
    guard(mutex)(&data.lock);
    ret = zopt2201_set_resolution(data, zopt2201_scale_array[idx].res);
    if (ret < 0)
    return ret;
    return zopt2201_set_gain(data, zopt2201_scale_array[idx].gain);
    }
    static int zopt2201_write_scale_als(struct zopt2201_data *data,
    int val, int val2)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(zopt2201_scale_als); i++)
    if (val == zopt2201_scale_als[i].scale &&
    val2 == zopt2201_scale_als[i].uscale)
    return zopt2201_write_scale_by_idx(data, i, zopt2201_scale_als);
    return -EINVAL;
    }
    static int zopt2201_write_scale_uvb(struct zopt2201_data *data,
    int val, int val2)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(zopt2201_scale_uvb); i++)
    if (val == zopt2201_scale_uvb[i].scale &&
    val2 == zopt2201_scale_uvb[i].uscale)
    return zopt2201_write_scale_by_idx(data, i, zopt2201_scale_uvb);
    return -EINVAL;
    }
    static int zopt2201_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct zopt2201_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_INT_TIME:
    return zopt2201_write_resolution(data, val, val2);
    case IIO_CHAN_INFO_SCALE:
    switch (chan.address) {
    case ZOPT2201_ALS_DATA:
    return zopt2201_write_scale_als(data, val, val2);
    case ZOPT2201_UVB_DATA:
    return zopt2201_write_scale_uvb(data, val, val2);
    default:
    return -EINVAL;
    }
    }
    return -EINVAL;
    }
    static ssize_t zopt2201_show_int_time_available(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut len: usize = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(zopt2201_resolution); i++)
    len += scnprintf(buf + len, PAGE_SIZE - len, "0.%06lu ",
    zopt2201_resolution[i].us);
    buf[len - 1] = '\n';
    return len;
    }
    static IIO_DEV_ATTR_INT_TIME_AVAIL(zopt2201_show_int_time_available);
    static ssize_t zopt2201_show_als_scale_avail(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut len: isize = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(zopt2201_scale_als); i++)
    len += scnprintf(buf + len, PAGE_SIZE - len, "%d.%06u ",
    zopt2201_scale_als[i].scale,
    zopt2201_scale_als[i].uscale);
    buf[len - 1] = '\n';
    return len;
    }
    static ssize_t zopt2201_show_uvb_scale_avail(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut len: isize = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(zopt2201_scale_uvb); i++)
    len += scnprintf(buf + len, PAGE_SIZE - len, "%d.%06u ",
    zopt2201_scale_uvb[i].scale,
    zopt2201_scale_uvb[i].uscale);
    buf[len - 1] = '\n';
    return len;
    }
    static IIO_DEVICE_ATTR(in_illuminance_scale_available, 0444,
    zopt2201_show_als_scale_avail, core::ptr::null_mut(), 0);
    static IIO_DEVICE_ATTR(in_intensity_uv_scale_available, 0444,
    zopt2201_show_uvb_scale_avail, core::ptr::null_mut(), 0);
    static struct attribute *zopt2201_attributes[] = {
    &iio_dev_attr_integration_time_available.dev_attr.attr,
    &iio_dev_attr_in_illuminance_scale_available.dev_attr.attr,
    &iio_dev_attr_in_intensity_uv_scale_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group zopt2201_attribute_group = {
    .attrs = zopt2201_attributes,
    };
    static const struct iio_info zopt2201_info = {
    .read_raw = zopt2201_read_raw,
    .write_raw = zopt2201_write_raw,
    .attrs = &zopt2201_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn zopt2201_probe(client: *mut i2c_client) -> c_int {
    static int zopt2201_probe(struct i2c_client *client)
    {
    struct zopt2201_data *data;
    struct iio_dev *indio_dev;
    int ret;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_I2C_BLOCK))
    return -EOPNOTSUPP;
    ret = i2c_smbus_read_byte_data(client, ZOPT2201_PART_ID);
    if (ret < 0)
    return ret;
    if (ret != ZOPT2201_PART_NUMBER)
    return -ENODEV;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    mutex_init(&data.lock);
    indio_dev.info = &zopt2201_info;
    indio_dev.channels = zopt2201_channels;
    indio_dev.num_channels = ARRAY_SIZE(zopt2201_channels);
    indio_dev.name = ZOPT2201_DRV_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    data.rate = ZOPT2201_MEAS_FREQ_100MS;
    ret = zopt2201_set_resolution(data, ZOPT2201_MEAS_RES_18BIT);
    if (ret < 0)
    return ret;
    ret = zopt2201_set_gain(data, ZOPT2201_LS_GAIN_3);
    if (ret < 0)
    return ret;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id zopt2201_id[] = {
    { .name = "zopt2201" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, zopt2201_id);
    static struct i2c_driver zopt2201_driver = {
    .driver = {
    .name   = ZOPT2201_DRV_NAME,
    },
    .probe = zopt2201_probe,
    .id_table = zopt2201_id,
    };
    module_i2c_driver(zopt2201_driver);
    MODULE_AUTHOR("Peter Meerwald-Stadler <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("IDT ZOPT2201 ambient light and UV B sensor driver");
    MODULE_LICENSE("GPL");
