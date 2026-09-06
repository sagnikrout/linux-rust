//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/max5522.c
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
// Maxim MAX5522
// Dual, Ultra-Low-Power 10-Bit, Voltage-Output DACs
//
// Copyright 2022 Timesys Corp.
//

pub const MAX5522_MAX_ADDR: c_int = 15;
pub const MAX5522_CTRL_NONE: c_int = 0;
pub const MAX5522_CTRL_LOAD_IN_A: c_int = 9;
pub const MAX5522_CTRL_LOAD_IN_B: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max5522_state {
    pub regmap: *mut regmap,
    pub dac_cache: [c_ushort; 2],
    pub vref_mV: c_int,
}

    .type = IIO_VOLTAGE, \
    .indexed = 1, \
    .output = 1, \
    .channel = chan, \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) | \
    BIT(IIO_CHAN_INFO_SCALE), \
    .scan_type = { \
    .sign = 'u', \
    .realbits = 10, \
    .storagebits = 16, \
    .shift = 2, \
    } \
    }
    static const struct iio_chan_spec max5522_channels[] = {
    MAX5522_CHANNEL(0),
    MAX5522_CHANNEL(1),
    };
#[no_mangle]
pub unsafe extern "C" fn max5522_info_to_reg(chan: *const iio_chan_spec) -> c_int {
    static inline int max5522_info_to_reg(struct iio_chan_spec const *chan)
    {
    return MAX5522_REG_DATA(chan.channel);
    }
    static int max5522_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long info)
    {
    struct max5522_state *state = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_RAW:
// val = state->dac_cache[chan->channel];
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = state->vref_mV;
// val2 = 10;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    return -EINVAL;
    }
    static int max5522_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long info)
    {
    struct max5522_state *state = iio_priv(indio_dev);
    int rval;
    if (val > 1023 || val < 0)
    return -EINVAL;
    rval = regmap_write(state.regmap, max5522_info_to_reg(chan),
    val << chan.scan_type.shift);
    if (rval < 0)
    return rval;
    state.dac_cache[chan.channel] = val;
    return 0;
    }
    static const struct iio_info max5522_info = {
    .read_raw = max5522_read_raw,
    .write_raw = max5522_write_raw,
    };
    static const struct regmap_config max5522_regmap_config = {
    .reg_bits = 4,
    .val_bits = 12,
    .max_register = MAX5522_MAX_ADDR,
    };
#[no_mangle]
unsafe extern "C" fn max5522_spi_probe(spi: *mut spi_device) -> c_int {
    static int max5522_spi_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct max5522_state *state;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*state));
    if (indio_dev == core::ptr::null_mut()) {
    dev_err(&spi.dev, "failed to allocate iio device\n");
    return  -ENOMEM;
    }
    state = iio_priv(indio_dev);
    ret = devm_regulator_get_enable_read_voltage(&spi.dev, "vrefin");
    if (ret < 0)
    return dev_err_probe(&spi.dev, ret,
    "Failed to get vrefin regulator\n");
    state.vref_mV = ret / (MICRO / MILLI);
    state.regmap = devm_regmap_init_spi(spi, &max5522_regmap_config);
    if (IS_ERR(state.regmap))
    return PTR_ERR(state.regmap);
    indio_dev.info = &max5522_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = max5522_channels;
    indio_dev.num_channels = ARRAY_SIZE(max5522_channels);
    indio_dev.name = "max5522";
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct spi_device_id max5522_ids[] = {
    { .name = "max5522" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, max5522_ids);
    static const struct of_device_id max5522_of_match[] = {
    { .compatible = "maxim,max5522" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max5522_of_match);
    static struct spi_driver max5522_spi_driver = {
    .driver = {
    .name = "max5522",
    .of_match_table = max5522_of_match,
    },
    .probe = max5522_spi_probe,
    .id_table = max5522_ids,
    };
    module_spi_driver(max5522_spi_driver);
    MODULE_AUTHOR("Angelo Dureghello <angelo.dureghello@timesys.com");
    MODULE_DESCRIPTION("MAX5522 DAC driver");
    MODULE_LICENSE("GPL");
