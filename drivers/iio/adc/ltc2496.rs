//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ltc2496.c
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
// ltc2496.c - Driver for Analog Devices/Linear Technology LTC2496 ADC
//
// Based on ltc2497.c which has
// Copyright (C) 2017 Analog Devices Inc.
//
// Licensed under the GPL-2.
//
// Datasheet: https://www.analog.com/media/en/technical-documentation/data-sheets/2496fc.pdf
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2496_driverdata {
// this must be the first member
    pub common_ddata: ltc2497core_driverdata,
    pub spi: *mut spi_device,
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
//
    pub __aligned(IIO_DMA_MINALIGN): unsigned char rxbuf[3],
    pub txbuf: [c_uchar; 3],
}

    static int ltc2496_result_and_measure(struct ltc2497core_driverdata *ddata,
    u8 address, int *val)
    {
    struct ltc2496_driverdata *st =
    container_of(ddata, struct ltc2496_driverdata, common_ddata);
    struct spi_transfer t = {
    .tx_buf = st.txbuf,
    .rx_buf = st.rxbuf,
    .len = sizeof(st.txbuf),
    };
    int ret;
    st.txbuf[0] = LTC2497_ENABLE | address;
    ret = spi_sync_transfer(st.spi, &t, 1);
    if (ret < 0)  {
    dev_err(&st.spi.dev, "spi_sync_transfer failed: %pe\n",
    ERR_PTR(ret));
    return ret;
    }
    if (val)
// val = ((st->rxbuf[0] & 0x3f) << 12 |
    st.rxbuf[1] << 4 | st.rxbuf[2] >> 4) -
    (1 << 17);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc2496_probe(spi: *mut spi_device) -> c_int {
    static int ltc2496_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct ltc2496_driverdata *st;
    struct device *dev = &spi.dev;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    spi_set_drvdata(spi, indio_dev);
    st.spi = spi;
    st.common_ddata.result_and_measure = ltc2496_result_and_measure;
    st.common_ddata.chip_info = device_get_match_data(dev);
    return ltc2497core_probe(dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn ltc2496_remove(spi: *mut spi_device) {
    static void ltc2496_remove(struct spi_device *spi)
    {
    struct iio_dev *indio_dev = spi_get_drvdata(spi);
    ltc2497core_remove(indio_dev);
    }
    static const struct ltc2497_chip_info ltc2496_info = {
    .resolution = 16,
    .name = core::ptr::null_mut(),
    };
    static const struct of_device_id ltc2496_of_match[] = {
    { .compatible = "lltc,ltc2496", .data = &ltc2496_info, },
    { }
    };
    MODULE_DEVICE_TABLE(of, ltc2496_of_match);
    static struct spi_driver ltc2496_driver = {
    .driver = {
    .name = "ltc2496",
    .of_match_table = ltc2496_of_match,
    },
    .probe = ltc2496_probe,
    .remove = ltc2496_remove,
    };
    module_spi_driver(ltc2496_driver);
    MODULE_AUTHOR("Uwe Kleine-König <u.kleine-könig@pengutronix.de>");
    MODULE_DESCRIPTION("Linear Technology LTC2496 ADC driver");
    MODULE_LICENSE("GPL v2");
