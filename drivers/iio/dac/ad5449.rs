//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ad5449.c
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
// AD5415, AD5426, AD5429, AD5432, AD5439, AD5443, AD5449 Digital to Analog
// Converter driver.
//
// Copyright 2012 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

pub const AD5449_MAX_CHANNELS: c_int = 2;
pub const AD5449_MAX_VREFS: c_int = 2;
pub const AD5449_CMD_NOOP: c_uint = 0x0;

pub const AD5449_CMD_CTRL: c_int = 13;
pub const AD5449_CTRL_SDO_OFFSET: c_int = 10;

//
// struct ad5449_chip_info - chip specific information
// @channels:		Channel specification
// @num_channels:	Number of channels
// @has_ctrl:		Chip has a control register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5449_chip_info {
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
    pub has_ctrl: bool,
}

//
// struct ad5449 - driver instance specific data
// @spi:		the SPI device for this driver instance
// @chip_info:		chip model specific constants, available modes etc
// @vref_reg:		vref supply regulators
// @has_sdo:		whether the SDO line is connected
// @dac_cache:		Cache for the DAC values
// @data:		spi transfer buffers
// @lock:		lock to protect the data buffer during SPI ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5449 {
    pub spi: *mut spi_device,
    pub chip_info: *const ad5449_chip_info,
    pub vref_reg: [regulator_bulk_data; AD5449_MAX_VREFS],
    pub lock: mutex,
    pub has_sdo: bool,
    pub dac_cache: [u16; AD5449_MAX_CHANNELS],
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
//
    pub __aligned(IIO_DMA_MINALIGN): __be16 data[2],
}

    enum ad5449_type {
    ID_AD5426,
    ID_AD5429,
    ID_AD5432,
    ID_AD5439,
    ID_AD5443,
    ID_AD5449,
    };
    static int ad5449_write(struct iio_dev *indio_dev, unsigned int addr,
    unsigned int val)
    {
    struct ad5449 *st = iio_priv(indio_dev);
    int ret;
    mutex_lock(&st.lock);
    st.data[0] = cpu_to_be16((addr << 12) | val);
    ret = spi_write(st.spi, st.data, 2);
    mutex_unlock(&st.lock);
    return ret;
    }
    static int ad5449_read(struct iio_dev *indio_dev, unsigned int addr,
    unsigned int *val)
    {
    struct ad5449 *st = iio_priv(indio_dev);
    int ret;
    struct spi_transfer t[] = {
    {
    .tx_buf = &st.data[0],
    .len = 2,
    .cs_change = 1,
    }, {
    .tx_buf = &st.data[1],
    .rx_buf = &st.data[1],
    .len = 2,
    },
    };
    mutex_lock(&st.lock);
    st.data[0] = cpu_to_be16(addr << 12);
    st.data[1] = cpu_to_be16(AD5449_CMD_NOOP);
    ret = spi_sync_transfer(st.spi, t, ARRAY_SIZE(t));
    if (ret < 0)
    goto out_unlock;
// val = be16_to_cpu(st->data[1]);
    out_unlock:
    mutex_unlock(&st.lock);
    return ret;
    }
    static int ad5449_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val, int *val2, long info)
    {
    struct ad5449 *st = iio_priv(indio_dev);
    struct regulator_bulk_data *reg;
    int scale_uv;
    int ret;
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    if (st.has_sdo) {
    ret = ad5449_read(indio_dev,
    AD5449_CMD_READ(chan.address), val);
    if (ret)
    return ret;
// val &= 0xfff;
    } else {
// val = st->dac_cache[chan->address];
    }
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    reg = &st.vref_reg[chan.channel];
    scale_uv = regulator_get_voltage(reg.consumer);
    if (scale_uv < 0)
    return scale_uv;
// val = scale_uv / 1000;
// val2 = chan->scan_type.realbits;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    break;
    }
    return -EINVAL;
    }
    static int ad5449_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val, int val2, long info)
    {
    struct ad5449 *st = iio_priv(indio_dev);
    int ret;
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    if (val < 0 || val >= (1 << chan.scan_type.realbits))
    return -EINVAL;
    ret = ad5449_write(indio_dev,
    AD5449_CMD_LOAD_AND_UPDATE(chan.address),
    val << chan.scan_type.shift);
    if (ret == 0)
    st.dac_cache[chan.address] = val;
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static const struct iio_info ad5449_info = {
    .read_raw = ad5449_read_raw,
    .write_raw = ad5449_write_raw,
    };

    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .output = 1,						\
    .channel = (chan),					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		\
    BIT(IIO_CHAN_INFO_SCALE),			\
    .address = (chan),					\
    .scan_type = {						\
    .sign = 'u',					\
    .realbits = (bits),				\
    .storagebits = 16,				\
    .shift = 12 - (bits),				\
    },							\
    }

    const struct iio_chan_spec name[] = { \
    AD5449_CHANNEL(0, bits), \
    AD5449_CHANNEL(1, bits), \
    }
    static DECLARE_AD5449_CHANNELS(ad5429_channels, 8);
    static DECLARE_AD5449_CHANNELS(ad5439_channels, 10);
    static DECLARE_AD5449_CHANNELS(ad5449_channels, 12);
    static const struct ad5449_chip_info ad5449_chip_info[] = {
    [ID_AD5426] = {
    .channels = ad5429_channels,
    .num_channels = 1,
    .has_ctrl = false,
    },
    [ID_AD5429] = {
    .channels = ad5429_channels,
    .num_channels = 2,
    .has_ctrl = true,
    },
    [ID_AD5432] = {
    .channels = ad5439_channels,
    .num_channels = 1,
    .has_ctrl = false,
    },
    [ID_AD5439] = {
    .channels = ad5439_channels,
    .num_channels = 2,
    .has_ctrl = true,
    },
    [ID_AD5443] = {
    .channels = ad5449_channels,
    .num_channels = 1,
    .has_ctrl = false,
    },
    [ID_AD5449] = {
    .channels = ad5449_channels,
    .num_channels = 2,
    .has_ctrl = true,
    },
    };
    static const char *ad5449_vref_name(struct ad5449 *st, int n)
    {
    if (st.chip_info.num_channels == 1)
    return "VREF";
    if (n == 0)
    return "VREFA";
    else
    return "VREFB";
    }
#[no_mangle]
unsafe extern "C" fn ad5449_spi_probe(spi: *mut spi_device) -> c_int {
    static int ad5449_spi_probe(struct spi_device *spi)
    {
    const struct spi_device_id *id = spi_get_device_id(spi);
    struct iio_dev *indio_dev;
    struct ad5449 *st;
    unsigned int i;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    st = iio_priv(indio_dev);
    spi_set_drvdata(spi, indio_dev);
    st.chip_info = &ad5449_chip_info[id.driver_data];
    st.spi = spi;
    for (i = 0; i < st.chip_info.num_channels; ++i)
    st.vref_reg[i].supply = ad5449_vref_name(st, i);
    ret = devm_regulator_bulk_get(&spi.dev, st.chip_info.num_channels,
    st.vref_reg);
    if (ret)
    return ret;
    ret = regulator_bulk_enable(st.chip_info.num_channels, st.vref_reg);
    if (ret)
    return ret;
    indio_dev.name = id.name;
    indio_dev.info = &ad5449_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = st.chip_info.channels;
    indio_dev.num_channels = st.chip_info.num_channels;
    mutex_init(&st.lock);
    if (st.chip_info.has_ctrl) {
    st.has_sdo = true;
    ad5449_write(indio_dev, AD5449_CMD_CTRL, 0x0);
    }
    ret = iio_device_register(indio_dev);
    if (ret)
    goto error_disable_reg;
    return 0;
    error_disable_reg:
    regulator_bulk_disable(st.chip_info.num_channels, st.vref_reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5449_spi_remove(spi: *mut spi_device) {
    static void ad5449_spi_remove(struct spi_device *spi)
    {
    struct iio_dev *indio_dev = spi_get_drvdata(spi);
    struct ad5449 *st = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    regulator_bulk_disable(st.chip_info.num_channels, st.vref_reg);
    }
    static const struct spi_device_id ad5449_spi_ids[] = {
    { .name = "ad5415", .driver_data = ID_AD5449 },
    { .name = "ad5426", .driver_data = ID_AD5426 },
    { .name = "ad5429", .driver_data = ID_AD5429 },
    { .name = "ad5432", .driver_data = ID_AD5432 },
    { .name = "ad5439", .driver_data = ID_AD5439 },
    { .name = "ad5443", .driver_data = ID_AD5443 },
    { .name = "ad5449", .driver_data = ID_AD5449 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad5449_spi_ids);
    static struct spi_driver ad5449_spi_driver = {
    .driver = {
    .name = "ad5449",
    },
    .probe = ad5449_spi_probe,
    .remove = ad5449_spi_remove,
    .id_table = ad5449_spi_ids,
    };
    module_spi_driver(ad5449_spi_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("Analog Devices AD5449 and similar DACs");
    MODULE_LICENSE("GPL v2");
