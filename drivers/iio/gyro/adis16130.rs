//! Automatically rewritten from C to Rust
//! Source: drivers/iio/gyro/adis16130.c
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
// ADIS16130 Digital Output, High Precision Angular Rate Sensor driver
//
// Copyright 2010 Analog Devices Inc.
//

pub const ADIS16130_CON: c_uint = 0x0;

pub const ADIS16130_IOP: c_uint = 0x1;
// 1 = data-ready signal low when unread data on all channels;

pub const ADIS16130_RATEDATA: c_uint = 0x8 /* Gyroscope output, rate of rotation */;
pub const ADIS16130_TEMPDATA: c_uint = 0xA /* Temperature output */;
pub const ADIS16130_RATECS: c_uint = 0x28 /* Gyroscope channel setup */;

pub const ADIS16130_TEMPCS: c_uint = 0x2A /* Temperature channel setup */;

pub const ADIS16130_RATECONV: c_uint = 0x30;
pub const ADIS16130_TEMPCONV: c_uint = 0x32;
pub const ADIS16130_MODE: c_uint = 0x38;

//
// struct adis16130_state - device instance specific data
// @us:			actual spi_device to write data
// @buf_lock:		mutex to protect tx and rx
// @buf:		unified tx/rx buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adis16130_state {
    pub us: *mut spi_device,
    pub buf_lock: mutex,
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[4],
}

#[no_mangle]
unsafe extern "C" fn adis16130_spi_read(indio_dev: *mut iio_dev, reg_addr: u8, val: *mut u32) -> c_int {
    static int adis16130_spi_read(struct iio_dev *indio_dev, u8 reg_addr, u32 *val)
    {
    int ret;
    struct adis16130_state *st = iio_priv(indio_dev);
    struct spi_transfer xfer = {
    .tx_buf = st.buf,
    .rx_buf = st.buf,
    .len = 4,
    };
    mutex_lock(&st.buf_lock);
    st.buf[0] = ADIS16130_CON_RD | reg_addr;
    st.buf[1] = st.buf[2] = st.buf[3] = 0;
    ret = spi_sync_transfer(st.us, &xfer, 1);
    if (ret == 0)
// val = get_unaligned_be24(&st->buf[1]);
    mutex_unlock(&st.buf_lock);
    return ret;
    }
    static int adis16130_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2,
    long mask)
    {
    int ret;
    u32 temp;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
// Take the iio_dev status lock
    ret = adis16130_spi_read(indio_dev, chan.address, &temp);
    if (ret)
    return ret;
// val = temp;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_ANGL_VEL:
// 0 degree = 838860, 250 degree = 14260608
// val = 250;
// val2 = 336440817; /* RAD_TO_DEGREE(14260608 - 8388608)
    return IIO_VAL_FRACTIONAL;
    case IIO_TEMP:
// 0C = 8036283, 105C = 9516048
// val = 105000;
// val2 = 9516048 - 8036283;
    return IIO_VAL_FRACTIONAL;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_OFFSET:
    switch (chan.type) {
    case IIO_ANGL_VEL:
// val = -8388608;
    return IIO_VAL_INT;
    case IIO_TEMP:
// val = -8036283;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    return -EINVAL;
    }
    static const struct iio_chan_spec adis16130_channels[] = {
    {
    .type = IIO_ANGL_VEL,
    .modified = 1,
    .channel2 = IIO_MOD_Z,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_OFFSET),
    .address = ADIS16130_RATEDATA,
    }, {
    .type = IIO_TEMP,
    .indexed = 1,
    .channel = 0,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_OFFSET),
    .address = ADIS16130_TEMPDATA,
    }
    };
    static const struct iio_info adis16130_info = {
    .read_raw = &adis16130_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn adis16130_probe(spi: *mut spi_device) -> c_int {
    static int adis16130_probe(struct spi_device *spi)
    {
    struct adis16130_state *st;
    struct iio_dev *indio_dev;
// setup the industrialio driver allocated elements
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
// this is only used for removal purposes
    spi_set_drvdata(spi, indio_dev);
    st.us = spi;
    mutex_init(&st.buf_lock);
    indio_dev.name = spi.dev.driver.name;
    indio_dev.channels = adis16130_channels;
    indio_dev.num_channels = ARRAY_SIZE(adis16130_channels);
    indio_dev.info = &adis16130_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static struct spi_driver adis16130_driver = {
    .driver = {
    .name = "adis16130",
    },
    .probe = adis16130_probe,
    };
    module_spi_driver(adis16130_driver);
    MODULE_AUTHOR("Barry Song <21cnbao@gmail.com>");
    MODULE_DESCRIPTION("Analog Devices ADIS16130 High Precision Angular Rate");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("spi:adis16130");
