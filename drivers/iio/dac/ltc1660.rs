//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ltc1660.c
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
// Driver for Linear Technology LTC1665/LTC1660, 8 channels DAC
//
// Copyright (C) 2018 Marcus Folkesson <marcus.folkesson@gmail.com>
//

pub const LTC1660_REG_WAKE: c_uint = 0x0;
pub const LTC1660_REG_DAC_A: c_uint = 0x1;
pub const LTC1660_REG_DAC_B: c_uint = 0x2;
pub const LTC1660_REG_DAC_C: c_uint = 0x3;
pub const LTC1660_REG_DAC_D: c_uint = 0x4;
pub const LTC1660_REG_DAC_E: c_uint = 0x5;
pub const LTC1660_REG_DAC_F: c_uint = 0x6;
pub const LTC1660_REG_DAC_G: c_uint = 0x7;
pub const LTC1660_REG_DAC_H: c_uint = 0x8;
pub const LTC1660_REG_SLEEP: c_uint = 0xe;
pub const LTC1660_NUM_CHANNELS: c_int = 8;
    static const struct regmap_config ltc1660_regmap_config = {
    .reg_bits = 4,
    .val_bits = 12,
    };
    enum ltc1660_supported_device_ids {
    ID_LTC1660,
    ID_LTC1665,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc1660_priv {
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
    pub vref_reg: *mut regulator,
    pub value: [c_uint; LTC1660_NUM_CHANNELS],
    pub vref_mv: c_uint,
}

    static int ltc1660_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long mask)
    {
    struct ltc1660_priv *priv = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
// val = priv->value[chan->channel];
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = regulator_get_voltage(priv->vref_reg);
    if (*val < 0) {
    dev_err(&priv.spi.dev, "failed to read vref regulator: %d\n",
// val);
    return *val;
    }
// Convert to mV
// val /= 1000;
// val2 = chan->scan_type.realbits;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    }
    static int ltc1660_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val,
    int val2,
    long mask)
    {
    struct ltc1660_priv *priv = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (val2 != 0)
    return -EINVAL;
    if (val < 0 || val > GENMASK(chan.scan_type.realbits - 1, 0))
    return -EINVAL;
    ret = regmap_write(priv.regmap, chan.channel,
    (val << chan.scan_type.shift));
    if (!ret)
    priv.value[chan.channel] = val;
    return ret;
    default:
    return -EINVAL;
    }
    }

    .type = IIO_VOLTAGE,				\
    .indexed = 1,					\
    .output = 1,					\
    .channel = chan,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .scan_type = {					\
    .sign = 'u',				\
    .realbits = (bits),			\
    .storagebits = 16,			\
    .shift = 12 - (bits),			\
    },						\
    }

    LTC1660_CHAN(LTC1660_REG_DAC_A, bits),	\
    LTC1660_CHAN(LTC1660_REG_DAC_B, bits),	\
    LTC1660_CHAN(LTC1660_REG_DAC_C, bits),	\
    LTC1660_CHAN(LTC1660_REG_DAC_D, bits),	\
    LTC1660_CHAN(LTC1660_REG_DAC_E, bits),	\
    LTC1660_CHAN(LTC1660_REG_DAC_F, bits),	\
    LTC1660_CHAN(LTC1660_REG_DAC_G, bits),	\
    LTC1660_CHAN(LTC1660_REG_DAC_H, bits),	\
    }
    static const struct iio_chan_spec ltc1660_channels[][LTC1660_NUM_CHANNELS] = {
    [ID_LTC1660] = LTC1660_OCTAL_CHANNELS(10),
    [ID_LTC1665] = LTC1660_OCTAL_CHANNELS(8),
    };
    static const struct iio_info ltc1660_info = {
    .read_raw = &ltc1660_read_raw,
    .write_raw = &ltc1660_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn ltc1660_suspend(dev: *mut device) -> c_int {
    static int ltc1660_suspend(struct device *dev)
    {
    struct ltc1660_priv *priv = iio_priv(spi_get_drvdata(
    to_spi_device(dev)));
    return regmap_write(priv.regmap, LTC1660_REG_SLEEP, 0x00);
    }
#[no_mangle]
unsafe extern "C" fn ltc1660_resume(dev: *mut device) -> c_int {
    static int ltc1660_resume(struct device *dev)
    {
    struct ltc1660_priv *priv = iio_priv(spi_get_drvdata(
    to_spi_device(dev)));
    return regmap_write(priv.regmap, LTC1660_REG_WAKE, 0x00);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ltc1660_pm_ops, ltc1660_suspend,
    ltc1660_resume);
#[no_mangle]
unsafe extern "C" fn ltc1660_probe(spi: *mut spi_device) -> c_int {
    static int ltc1660_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct ltc1660_priv *priv;
    const struct spi_device_id *id = spi_get_device_id(spi);
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*priv));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    priv = iio_priv(indio_dev);
    priv.regmap = devm_regmap_init_spi(spi, &ltc1660_regmap_config);
    if (IS_ERR(priv.regmap)) {
    dev_err(&spi.dev, "failed to register spi regmap %ld\n",
    PTR_ERR(priv.regmap));
    return PTR_ERR(priv.regmap);
    }
    priv.vref_reg = devm_regulator_get(&spi.dev, "vref");
    if (IS_ERR(priv.vref_reg))
    return dev_err_probe(&spi.dev, PTR_ERR(priv.vref_reg),
    "vref regulator not specified\n");
    ret = regulator_enable(priv.vref_reg);
    if (ret) {
    dev_err(&spi.dev, "failed to enable vref regulator: %d\n",
    ret);
    return ret;
    }
    priv.spi = spi;
    spi_set_drvdata(spi, indio_dev);
    indio_dev.info = &ltc1660_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = ltc1660_channels[id.driver_data];
    indio_dev.num_channels = LTC1660_NUM_CHANNELS;
    indio_dev.name = id.name;
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err(&spi.dev, "failed to register iio device: %d\n",
    ret);
    goto error_disable_reg;
    }
    return 0;
    error_disable_reg:
    regulator_disable(priv.vref_reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ltc1660_remove(spi: *mut spi_device) {
    static void ltc1660_remove(struct spi_device *spi)
    {
    struct iio_dev *indio_dev = spi_get_drvdata(spi);
    struct ltc1660_priv *priv = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    regulator_disable(priv.vref_reg);
    }
    static const struct of_device_id ltc1660_dt_ids[] = {
    { .compatible = "lltc,ltc1660", .data = (void *)ID_LTC1660 },
    { .compatible = "lltc,ltc1665", .data = (void *)ID_LTC1665 },
    { }
    };
    MODULE_DEVICE_TABLE(of, ltc1660_dt_ids);
    static const struct spi_device_id ltc1660_id[] = {
    { .name = "ltc1660", .driver_data = ID_LTC1660 },
    { .name = "ltc1665", .driver_data = ID_LTC1665 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ltc1660_id);
    static struct spi_driver ltc1660_driver = {
    .driver = {
    .name = "ltc1660",
    .of_match_table = ltc1660_dt_ids,
    .pm = pm_sleep_ptr(&ltc1660_pm_ops),
    },
    .probe	= ltc1660_probe,
    .remove = ltc1660_remove,
    .id_table = ltc1660_id,
    };
    module_spi_driver(ltc1660_driver);
    MODULE_AUTHOR("Marcus Folkesson <marcus.folkesson@gmail.com>");
    MODULE_DESCRIPTION("Linear Technology LTC1660/LTC1665 DAC");
    MODULE_LICENSE("GPL v2");
