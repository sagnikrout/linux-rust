//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/adjd_s311.c
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
// adjd_s311.c - Support for ADJD-S311-CR999 digital color sensor
//
// Copyright (C) 2012 Peter Meerwald <pmeerw@pmeerw.net>
//
// driver for ADJD-S311-CR999 digital color sensor (10-bit channels for
// red, green, blue, clear); 7-bit I2C slave address 0x74
//
// limitations: no calibration, no offset mode, no sleep mode
//

pub const ADJD_S311_CTRL: c_uint = 0x00;
pub const ADJD_S311_CONFIG: c_uint = 0x01;
pub const ADJD_S311_CAP_RED: c_uint = 0x06;
pub const ADJD_S311_CAP_GREEN: c_uint = 0x07;
pub const ADJD_S311_CAP_BLUE: c_uint = 0x08;
pub const ADJD_S311_CAP_CLEAR: c_uint = 0x09;
pub const ADJD_S311_INT_RED: c_uint = 0x0a;
pub const ADJD_S311_INT_GREEN: c_uint = 0x0c;
pub const ADJD_S311_INT_BLUE: c_uint = 0x0e;
pub const ADJD_S311_INT_CLEAR: c_uint = 0x10;
pub const ADJD_S311_DATA_RED: c_uint = 0x40;
pub const ADJD_S311_DATA_GREEN: c_uint = 0x42;
pub const ADJD_S311_DATA_BLUE: c_uint = 0x44;
pub const ADJD_S311_DATA_CLEAR: c_uint = 0x46;
pub const ADJD_S311_OFFSET_RED: c_uint = 0x48;
pub const ADJD_S311_OFFSET_GREEN: c_uint = 0x49;
pub const ADJD_S311_OFFSET_BLUE: c_uint = 0x4a;
pub const ADJD_S311_OFFSET_CLEAR: c_uint = 0x4b;
pub const ADJD_S311_CTRL_GOFS: c_uint = 0x02;
pub const ADJD_S311_CTRL_GSSR: c_uint = 0x01;
pub const ADJD_S311_CAP_MASK: c_uint = 0x0f;
pub const ADJD_S311_INT_MASK: c_uint = 0x0fff;
pub const ADJD_S311_DATA_MASK: c_uint = 0x03ff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adjd_s311_data {
    pub client: *mut i2c_client,
}

    enum adjd_s311_channel_idx {
    IDX_RED, IDX_GREEN, IDX_BLUE, IDX_CLEAR
    };

#[no_mangle]
unsafe extern "C" fn adjd_s311_req_data(indio_dev: *mut iio_dev) -> c_int {
    static int adjd_s311_req_data(struct iio_dev *indio_dev)
    {
    struct adjd_s311_data *data = iio_priv(indio_dev);
    let mut tries: c_int = 10;
    int ret = i2c_smbus_write_byte_data(data.client, ADJD_S311_CTRL,
    ADJD_S311_CTRL_GSSR);
    if (ret < 0)
    return ret;
    while (tries--) {
    ret = i2c_smbus_read_byte_data(data.client, ADJD_S311_CTRL);
    if (ret < 0)
    return ret;
    if (!(ret & ADJD_S311_CTRL_GSSR))
    break;
    msleep(20);
    }
    if (tries < 0) {
    dev_err(&data.client.dev,
    "adjd_s311_req_data() failed, data not ready\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adjd_s311_read_data(indio_dev: *mut iio_dev, reg: u8, val: *mut c_int) -> c_int {
    static int adjd_s311_read_data(struct iio_dev *indio_dev, u8 reg, int *val)
    {
    struct adjd_s311_data *data = iio_priv(indio_dev);
    let mut ret: c_int = adjd_s311_req_data(indio_dev);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_word_data(data.client, reg);
    if (ret < 0)
    return ret;
// val = ret & ADJD_S311_DATA_MASK;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adjd_s311_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t adjd_s311_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct adjd_s311_data *data = iio_priv(indio_dev);
    let mut time_ns: i64 = iio_get_time_ns(indio_dev);
    int i, j = 0;
    struct {
    s16 chans[4];
    aligned_s64 ts;
    } scan = { };
    let mut ret: c_int = adjd_s311_req_data(indio_dev);
    if (ret < 0)
    goto done;
    iio_for_each_active_channel(indio_dev, i) {
    ret = i2c_smbus_read_word_data(data.client,
    ADJD_S311_DATA_REG(i));
    if (ret < 0)
    goto done;
    scan.chans[j++] = ret & ADJD_S311_DATA_MASK;
    }
    iio_push_to_buffers_with_ts(indio_dev, &scan, sizeof(scan), time_ns);
    done:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }

    .type = IIO_INTENSITY, \
    .modified = 1, \
    .address = (IDX_##_color), \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) | \
    BIT(IIO_CHAN_INFO_HARDWAREGAIN) | \
    BIT(IIO_CHAN_INFO_INT_TIME), \
    .channel2 = (IIO_MOD_LIGHT_##_color), \
    .scan_index = (_scan_idx), \
    .scan_type = { \
    .sign = 'u', \
    .realbits = 10, \
    .storagebits = 16, \
    .endianness = IIO_CPU, \
    }, \
    }
    static const struct iio_chan_spec adjd_s311_channels[] = {
    ADJD_S311_CHANNEL(RED, 0),
    ADJD_S311_CHANNEL(GREEN, 1),
    ADJD_S311_CHANNEL(BLUE, 2),
    ADJD_S311_CHANNEL(CLEAR, 3),
    IIO_CHAN_SOFT_TIMESTAMP(4),
    };
    static int adjd_s311_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct adjd_s311_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = adjd_s311_read_data(indio_dev,
    ADJD_S311_DATA_REG(chan.address), val);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_HARDWAREGAIN:
    ret = i2c_smbus_read_byte_data(data.client,
    ADJD_S311_CAP_REG(chan.address));
    if (ret < 0)
    return ret;
// val = ret & ADJD_S311_CAP_MASK;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_INT_TIME:
    ret = i2c_smbus_read_word_data(data.client,
    ADJD_S311_INT_REG(chan.address));
    if (ret < 0)
    return ret;
// val = 0;
//
// not documented, based on measurement:
// 4095 LSBs correspond to roughly 4 ms
//
// val2 = ret & ADJD_S311_INT_MASK;
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static int adjd_s311_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct adjd_s311_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_HARDWAREGAIN:
    if (val < 0 || val > ADJD_S311_CAP_MASK)
    return -EINVAL;
    return i2c_smbus_write_byte_data(data.client,
    ADJD_S311_CAP_REG(chan.address), val);
    case IIO_CHAN_INFO_INT_TIME:
    if (val != 0 || val2 < 0 || val2 > ADJD_S311_INT_MASK)
    return -EINVAL;
    return i2c_smbus_write_word_data(data.client,
    ADJD_S311_INT_REG(chan.address), val2);
    }
    return -EINVAL;
    }
    static const struct iio_info adjd_s311_info = {
    .read_raw = adjd_s311_read_raw,
    .write_raw = adjd_s311_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn adjd_s311_probe(client: *mut i2c_client) -> c_int {
    static int adjd_s311_probe(struct i2c_client *client)
    {
    struct adjd_s311_data *data;
    struct iio_dev *indio_dev;
    int err;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    indio_dev.info = &adjd_s311_info;
    indio_dev.name = ADJD_S311_DRV_NAME;
    indio_dev.channels = adjd_s311_channels;
    indio_dev.num_channels = ARRAY_SIZE(adjd_s311_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    err = devm_iio_triggered_buffer_setup(&client.dev, indio_dev, core::ptr::null_mut(),
    adjd_s311_trigger_handler, core::ptr::null_mut());
    if (err < 0)
    return err;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id adjd_s311_id[] = {
    { .name = "adjd_s311" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adjd_s311_id);
    static struct i2c_driver adjd_s311_driver = {
    .driver = {
    .name	= ADJD_S311_DRV_NAME,
    },
    .probe		= adjd_s311_probe,
    .id_table	= adjd_s311_id,
    };
    module_i2c_driver(adjd_s311_driver);
    MODULE_AUTHOR("Peter Meerwald <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("ADJD-S311 color sensor");
    MODULE_LICENSE("GPL");
