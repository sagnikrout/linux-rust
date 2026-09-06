//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/isl29125.c
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
// isl29125.c - Support for Intersil ISL29125 RGB light sensor
//
// Copyright (c) 2014 Peter Meerwald <pmeerw@pmeerw.net>
//
// RGB light sensor with 16-bit channels for red, green, blue);
// 7-bit I2C slave address 0x44
//
// TODO: interrupt support, IR compensation, thresholds, 12bit
//

pub const ISL29125_DEVICE_ID: c_uint = 0x00;
pub const ISL29125_CONF1: c_uint = 0x01;
pub const ISL29125_CONF2: c_uint = 0x02;
pub const ISL29125_CONF3: c_uint = 0x03;
pub const ISL29125_STATUS: c_uint = 0x08;
pub const ISL29125_GREEN_DATA: c_uint = 0x09;
pub const ISL29125_RED_DATA: c_uint = 0x0b;
pub const ISL29125_BLUE_DATA: c_uint = 0x0d;
pub const ISL29125_ID: c_uint = 0x7d;

pub const ISL29125_MODE_PD: c_uint = 0x0;
pub const ISL29125_MODE_G: c_uint = 0x1;
pub const ISL29125_MODE_R: c_uint = 0x2;
pub const ISL29125_MODE_B: c_uint = 0x3;
pub const ISL29125_MODE_RGB: c_uint = 0x5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl29125_data {
    pub client: *mut i2c_client,
    pub conf1: u8,
}

    .type = IIO_INTENSITY, \
    .modified = 1, \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW), \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE), \
    .channel2 = IIO_MOD_LIGHT_##_color, \
    .scan_index = _si, \
    .scan_type = { \
    .sign = 'u', \
    .realbits = 16, \
    .storagebits = 16, \
    .endianness = IIO_CPU, \
    }, \
    }
    static const struct iio_chan_spec isl29125_channels[] = {
    ISL29125_CHANNEL(GREEN, 0),
    ISL29125_CHANNEL(RED, 1),
    ISL29125_CHANNEL(BLUE, 2),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    };
    static const struct {
    u8 mode, data;
    } isl29125_regs[] = {
    {ISL29125_MODE_G, ISL29125_GREEN_DATA},
    {ISL29125_MODE_R, ISL29125_RED_DATA},
    {ISL29125_MODE_B, ISL29125_BLUE_DATA},
    };
#[no_mangle]
unsafe extern "C" fn isl29125_read_data(data: *mut isl29125_data, si: c_int) -> c_int {
    static int isl29125_read_data(struct isl29125_data *data, int si)
    {
    let mut tries: c_int = 5;
    int ret;
    ret = i2c_smbus_write_byte_data(data.client, ISL29125_CONF1,
    data.conf1 | isl29125_regs[si].mode);
    if (ret < 0)
    return ret;
    msleep(101);
    while (tries--) {
    ret = i2c_smbus_read_byte_data(data.client, ISL29125_STATUS);
    if (ret < 0)
    goto fail;
    if (ret & ISL29125_STATUS_CONV)
    break;
    msleep(20);
    }
    if (tries < 0) {
    dev_err(&data.client.dev, "data not ready\n");
    ret = -EIO;
    goto fail;
    }
    ret = i2c_smbus_read_word_data(data.client, isl29125_regs[si].data);
    fail:
    i2c_smbus_write_byte_data(data.client, ISL29125_CONF1, data.conf1);
    return ret;
    }
    static int isl29125_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct isl29125_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = isl29125_read_data(data, chan.scan_index);
    iio_device_release_direct(indio_dev);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
    if (data.conf1 & ISL29125_MODE_RANGE)
// val2 = ISL29125_SENSING_RANGE_1; /*10k lux full range
    else
// val2 = ISL29125_SENSING_RANGE_0; /*375 lux full range
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static int isl29125_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct isl29125_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    if (val != 0)
    return -EINVAL;
    if (val2 == ISL29125_SENSING_RANGE_1)
    data.conf1 |= ISL29125_MODE_RANGE;
#[no_mangle]
pub unsafe extern "C" fn if(ISL29125_SENSING_RANGE_0: val2 ==) -> else {
    else if (val2 == ISL29125_SENSING_RANGE_0)
    data.conf1 &= ~ISL29125_MODE_RANGE;
    else
    return -EINVAL;
    return i2c_smbus_write_byte_data(data.client, ISL29125_CONF1,
    data.conf1);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn isl29125_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t isl29125_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct isl29125_data *data = iio_priv(indio_dev);
    int i, j = 0;
// Ensure timestamp is naturally aligned
    struct {
    u16 chans[3];
    aligned_s64 timestamp;
    } scan = { };
    iio_for_each_active_channel(indio_dev, i) {
    int ret = i2c_smbus_read_word_data(data.client,
    isl29125_regs[i].data);
    if (ret < 0)
    goto done;
    scan.chans[j++] = ret;
    }
    iio_push_to_buffers_with_ts(indio_dev, &scan, sizeof(scan),
    iio_get_time_ns(indio_dev));
    done:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static IIO_CONST_ATTR(scale_available, "0.005722 0.152590");
    static struct attribute *isl29125_attributes[] = {
    &iio_const_attr_scale_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group isl29125_attribute_group = {
    .attrs = isl29125_attributes,
    };
    static const struct iio_info isl29125_info = {
    .read_raw = isl29125_read_raw,
    .write_raw = isl29125_write_raw,
    .attrs = &isl29125_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn isl29125_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int isl29125_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct isl29125_data *data = iio_priv(indio_dev);
    data.conf1 |= ISL29125_MODE_RGB;
    return i2c_smbus_write_byte_data(data.client, ISL29125_CONF1,
    data.conf1);
    }
#[no_mangle]
unsafe extern "C" fn isl29125_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int isl29125_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct isl29125_data *data = iio_priv(indio_dev);
    data.conf1 &= ~ISL29125_MODE_MASK;
    data.conf1 |= ISL29125_MODE_PD;
    return i2c_smbus_write_byte_data(data.client, ISL29125_CONF1,
    data.conf1);
    }
    static const struct iio_buffer_setup_ops isl29125_buffer_setup_ops = {
    .postenable = isl29125_buffer_postenable,
    .predisable = isl29125_buffer_predisable,
    };
#[no_mangle]
unsafe extern "C" fn isl29125_probe(client: *mut i2c_client) -> c_int {
    static int isl29125_probe(struct i2c_client *client)
    {
    struct isl29125_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    indio_dev.info = &isl29125_info;
    indio_dev.name = ISL29125_DRV_NAME;
    indio_dev.channels = isl29125_channels;
    indio_dev.num_channels = ARRAY_SIZE(isl29125_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = i2c_smbus_read_byte_data(data.client, ISL29125_DEVICE_ID);
    if (ret < 0)
    return ret;
    if (ret != ISL29125_ID)
    return -ENODEV;
    data.conf1 = ISL29125_MODE_PD | ISL29125_MODE_RANGE;
    ret = i2c_smbus_write_byte_data(data.client, ISL29125_CONF1,
    data.conf1);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(data.client, ISL29125_STATUS, 0);
    if (ret < 0)
    return ret;
    ret = iio_triggered_buffer_setup(indio_dev, core::ptr::null_mut(),
    isl29125_trigger_handler, &isl29125_buffer_setup_ops);
    if (ret < 0)
    return ret;
    ret = iio_device_register(indio_dev);
    if (ret < 0)
    goto buffer_cleanup;
    return 0;
    buffer_cleanup:
    iio_triggered_buffer_cleanup(indio_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn isl29125_powerdown(data: *mut isl29125_data) -> c_int {
    static int isl29125_powerdown(struct isl29125_data *data)
    {
    return i2c_smbus_write_byte_data(data.client, ISL29125_CONF1,
    (data.conf1 & ~ISL29125_MODE_MASK) | ISL29125_MODE_PD);
    }
#[no_mangle]
unsafe extern "C" fn isl29125_remove(client: *mut i2c_client) {
    static void isl29125_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    iio_device_unregister(indio_dev);
    iio_triggered_buffer_cleanup(indio_dev);
    isl29125_powerdown(iio_priv(indio_dev));
    }
#[no_mangle]
unsafe extern "C" fn isl29125_suspend(dev: *mut device) -> c_int {
    static int isl29125_suspend(struct device *dev)
    {
    struct isl29125_data *data = iio_priv(i2c_get_clientdata(
    to_i2c_client(dev)));
    return isl29125_powerdown(data);
    }
#[no_mangle]
unsafe extern "C" fn isl29125_resume(dev: *mut device) -> c_int {
    static int isl29125_resume(struct device *dev)
    {
    struct isl29125_data *data = iio_priv(i2c_get_clientdata(
    to_i2c_client(dev)));
    return i2c_smbus_write_byte_data(data.client, ISL29125_CONF1,
    data.conf1);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(isl29125_pm_ops, isl29125_suspend,
    isl29125_resume);
    static const struct i2c_device_id isl29125_id[] = {
    { .name = "isl29125" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, isl29125_id);
    static struct i2c_driver isl29125_driver = {
    .driver = {
    .name	= ISL29125_DRV_NAME,
    .pm	= pm_sleep_ptr(&isl29125_pm_ops),
    },
    .probe		= isl29125_probe,
    .remove		= isl29125_remove,
    .id_table	= isl29125_id,
    };
    module_i2c_driver(isl29125_driver);
    MODULE_AUTHOR("Peter Meerwald <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("ISL29125 RGB light sensor driver");
    MODULE_LICENSE("GPL");
