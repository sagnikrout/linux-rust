//! Automatically rewritten from C to Rust
//! Source: drivers/iio/potentiometer/x9250.c
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


// SPDX-License-Identifier: GPL-2.0
//
// x9250.c  --  Renesas X9250 potentiometers IIO driver
//
// Copyright 2023 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x9250_cfg {
    pub name: *const c_char,
    pub kohms: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x9250 {
    pub spi: *mut spi_device,
    pub cfg: *const x9250_cfg,
    pub wp_gpio: *mut gpio_desc,
}

pub const X9250_ID: c_uint = 0x50;

#[no_mangle]
unsafe extern "C" fn x9250_write8(x9250: *mut x9250, cmd: u8, val: u8) -> c_int {
    static int x9250_write8(struct x9250 *x9250, u8 cmd, u8 val)
    {
    u8 txbuf[3];
    txbuf[0] = X9250_ID;
    txbuf[1] = cmd;
    txbuf[2] = val;
    return spi_write_then_read(x9250.spi, txbuf, ARRAY_SIZE(txbuf), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn x9250_read8(x9250: *mut x9250, cmd: u8, val: *mut u8) -> c_int {
    static int x9250_read8(struct x9250 *x9250, u8 cmd, u8 *val)
    {
    u8 txbuf[2];
    txbuf[0] = X9250_ID;
    txbuf[1] = cmd;
    return spi_write_then_read(x9250.spi, txbuf, ARRAY_SIZE(txbuf), val, 1);
    }

    .type = IIO_RESISTANCE,						\
    .indexed = 1,							\
    .output = 1,							\
    .channel = (ch),						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),			\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),		\
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_RAW),	\
    }
    static const struct iio_chan_spec x9250_channels[] = {
    X9250_CHANNEL(0),
    X9250_CHANNEL(1),
    X9250_CHANNEL(2),
    X9250_CHANNEL(3),
    };
    static int x9250_read_raw(struct iio_dev *indio_dev, struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct x9250 *x9250 = iio_priv(indio_dev);
    let mut ch: c_int = chan.channel;
    int ret;
    u8 v;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = x9250_read8(x9250, X9250_CMD_RD_WCR(ch), &v);
    if (ret)
    return ret;
// val = v;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 1000 * x9250->cfg->kohms;
// val2 = U8_MAX;
    return IIO_VAL_FRACTIONAL;
    }
    return -EINVAL;
    }
    static int x9250_read_avail(struct iio_dev *indio_dev, struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length, long mask)
    {
    static const int range[] = {0, 1, 255}; /* min, step, max */
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
// length = ARRAY_SIZE(range);
// vals = range;
// type = IIO_VAL_INT;
    return IIO_AVAIL_RANGE;
    }
    return -EINVAL;
    }
    static int x9250_write_raw(struct iio_dev *indio_dev, struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct x9250 *x9250 = iio_priv(indio_dev);
    let mut ch: c_int = chan.channel;
    int ret;
    if (mask != IIO_CHAN_INFO_RAW)
    return -EINVAL;
    if (val > U8_MAX || val < 0)
    return -EINVAL;
    gpiod_set_value_cansleep(x9250.wp_gpio, 0);
    ret = x9250_write8(x9250, X9250_CMD_WR_WCR(ch), val);
    gpiod_set_value_cansleep(x9250.wp_gpio, 1);
    return ret;
    }
    static const struct iio_info x9250_info = {
    .read_raw = x9250_read_raw,
    .read_avail = x9250_read_avail,
    .write_raw = x9250_write_raw,
    };
    enum x9250_type {
    X9250T,
    X9250U,
    };
    static const struct x9250_cfg x9250_cfg[] = {
    [X9250T] = { .name = "x9250t", .kohms =  100, },
    [X9250U] = { .name = "x9250u", .kohms =  50, },
    };
    static const char *const x9250_regulator_names[] = {
    "vcc",
    "avp",
    "avn",
    };
#[no_mangle]
unsafe extern "C" fn x9250_probe(spi: *mut spi_device) -> c_int {
    static int x9250_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct x9250 *x9250;
    int ret;
    ret = devm_regulator_bulk_get_enable(&spi.dev, ARRAY_SIZE(x9250_regulator_names),
    x9250_regulator_names);
    if (ret)
    return dev_err_probe(&spi.dev, ret, "Failed to get regulators\n");
//
// The x9250 needs a 5ms maximum delay after the power-supplies are set
// before performing the first write (1ms for the first read).
//
    usleep_range(5000, 6000);
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*x9250));
    if (!indio_dev)
    return -ENOMEM;
    x9250 = iio_priv(indio_dev);
    x9250.spi = spi;
    x9250.cfg = spi_get_device_match_data(spi);
    x9250.wp_gpio = devm_gpiod_get_optional(&spi.dev, "wp", GPIOD_OUT_LOW);
    if (IS_ERR(x9250.wp_gpio))
    return dev_err_probe(&spi.dev, PTR_ERR(x9250.wp_gpio),
    "failed to get wp gpio\n");
    indio_dev.info = &x9250_info;
    indio_dev.channels = x9250_channels;
    indio_dev.num_channels = ARRAY_SIZE(x9250_channels);
    indio_dev.name = x9250.cfg.name;
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct of_device_id x9250_of_match[] = {
    { .compatible = "renesas,x9250t", .data = &x9250_cfg[X9250T]},
    { .compatible = "renesas,x9250u", .data = &x9250_cfg[X9250U]},
    { }
    };
    MODULE_DEVICE_TABLE(of, x9250_of_match);
    static const struct spi_device_id x9250_id_table[] = {
    { .name = "x9250t", .driver_data = (kernel_ulong_t)&x9250_cfg[X9250T] },
    { .name = "x9250u", .driver_data = (kernel_ulong_t)&x9250_cfg[X9250U] },
    { }
    };
    MODULE_DEVICE_TABLE(spi, x9250_id_table);
    static struct spi_driver x9250_spi_driver = {
    .driver  = {
    .name = "x9250",
    .of_match_table = x9250_of_match,
    },
    .id_table = x9250_id_table,
    .probe  = x9250_probe,
    };
    module_spi_driver(x9250_spi_driver);
    MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>");
    MODULE_DESCRIPTION("X9250 ALSA SoC driver");
    MODULE_LICENSE("GPL");
