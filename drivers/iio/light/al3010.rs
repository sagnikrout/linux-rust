//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/al3010.c
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
// AL3010 - Dyna Image Ambient Light Sensor
//
// Copyright (c) 2014, Intel Corporation.
// Copyright (c) 2016, Dyna-Image Corp.
// Copyright (c) 2020, David Heidelberg, Michał Mirosław, Dmitry Osipenko
//
// IIO driver for AL3010 (7-bit I2C slave address 0x1C).
//
// TODO: interrupt support, thresholds
// When the driver will get support for interrupt handling, then interrupt
// will need to be disabled before turning sensor OFF in order to avoid
// potential races with the interrupt handling.
//

pub const AL3010_REG_SYSTEM: c_uint = 0x00;
pub const AL3010_REG_DATA_LOW: c_uint = 0x0c;
pub const AL3010_REG_CONFIG: c_uint = 0x10;
pub const AL3010_CONFIG_DISABLE: c_uint = 0x00;
pub const AL3010_CONFIG_ENABLE: c_uint = 0x01;

    enum al3xxxx_range {
    AL3XXX_RANGE_1, /* 77806 lx */
    AL3XXX_RANGE_2, /* 19542 lx */
    AL3XXX_RANGE_3, /*  4863 lx */
    AL3XXX_RANGE_4  /*  1216 lx */
    };
    static const int al3010_scales[][2] = {
    { 1, 187200 }, { 0, 296800 }, { 0, 74200 }, { 0, 18600 },
    };
    static const struct regmap_config al3010_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = AL3010_REG_CONFIG,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct al3010_data {
    pub regmap: *mut regmap,
}

    static const struct iio_chan_spec al3010_channels[] = {
    {
    .type	= IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    }
    };
    static IIO_CONST_ATTR(in_illuminance_scale_available, AL3010_SCALE_AVAILABLE);
    static struct attribute *al3010_attributes[] = {
    &iio_const_attr_in_illuminance_scale_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group al3010_attribute_group = {
    .attrs = al3010_attributes,
    };
#[no_mangle]
unsafe extern "C" fn al3010_set_pwr_on(data: *mut al3010_data) -> c_int {
    static int al3010_set_pwr_on(struct al3010_data *data)
    {
    return regmap_write(data.regmap, AL3010_REG_SYSTEM, AL3010_CONFIG_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn al3010_set_pwr_off(_data: *mut c_void) {
    static void al3010_set_pwr_off(void *_data)
    {
    struct al3010_data *data = _data;
    struct device *dev = regmap_get_device(data.regmap);
    int ret;
    ret = regmap_write(data.regmap, AL3010_REG_SYSTEM, AL3010_CONFIG_DISABLE);
    if (ret)
    dev_err(dev, "failed to write system register\n");
    }
#[no_mangle]
unsafe extern "C" fn al3010_init(data: *mut al3010_data) -> c_int {
    static int al3010_init(struct al3010_data *data)
    {
    struct device *dev = regmap_get_device(data.regmap);
    int ret;
    ret = al3010_set_pwr_on(data);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, al3010_set_pwr_off, data);
    if (ret)
    return ret;
    return regmap_write(data.regmap, AL3010_REG_CONFIG,
    FIELD_PREP(AL3010_GAIN_MASK, AL3XXX_RANGE_3));
    }
    static int al3010_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct al3010_data *data = iio_priv(indio_dev);
    int ret, gain;
    __le16 raw;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
//
// ALS ADC value is stored in two adjacent registers:
// - low byte of output is stored at AL3010_REG_DATA_LOW
// - high byte of output is stored at AL3010_REG_DATA_LOW + 1
//
    ret = regmap_bulk_read(data.regmap, AL3010_REG_DATA_LOW,
    &raw, sizeof(raw));
    if (ret)
    return ret;
// val = le16_to_cpu(raw);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    ret = regmap_read(data.regmap, AL3010_REG_CONFIG, &gain);
    if (ret)
    return ret;
    gain = FIELD_GET(AL3010_GAIN_MASK, gain);
// val = al3010_scales[gain][0];
// val2 = al3010_scales[gain][1];
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static int al3010_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val,
    int val2, long mask)
    {
    struct al3010_data *data = iio_priv(indio_dev);
    unsigned int i;
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    for (i = 0; i < ARRAY_SIZE(al3010_scales); i++) {
    if (val != al3010_scales[i][0] ||
    val2 != al3010_scales[i][1])
    continue;
    return regmap_write(data.regmap, AL3010_REG_CONFIG,
    FIELD_PREP(AL3010_GAIN_MASK, i));
    }
    break;
    }
    return -EINVAL;
    }
    static const struct iio_info al3010_info = {
    .read_raw	= al3010_read_raw,
    .write_raw	= al3010_write_raw,
    .attrs		= &al3010_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn al3010_probe(client: *mut i2c_client) -> c_int {
    static int al3010_probe(struct i2c_client *client)
    {
    struct al3010_data *data;
    struct device *dev = &client.dev;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.regmap = devm_regmap_init_i2c(client, &al3010_regmap_config);
    if (IS_ERR(data.regmap))
    return dev_err_probe(dev, PTR_ERR(data.regmap),
    "cannot allocate regmap\n");
    indio_dev.info = &al3010_info;
    indio_dev.name = "al3010";
    indio_dev.channels = al3010_channels;
    indio_dev.num_channels = ARRAY_SIZE(al3010_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = al3010_init(data);
    if (ret)
    return dev_err_probe(dev, ret, "failed to init ALS\n");
    return devm_iio_device_register(dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn al3010_suspend(dev: *mut device) -> c_int {
    static int al3010_suspend(struct device *dev)
    {
    struct al3010_data *data = iio_priv(dev_get_drvdata(dev));
    al3010_set_pwr_off(data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn al3010_resume(dev: *mut device) -> c_int {
    static int al3010_resume(struct device *dev)
    {
    struct al3010_data *data = iio_priv(dev_get_drvdata(dev));
    return al3010_set_pwr_on(data);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(al3010_pm_ops, al3010_suspend, al3010_resume);
    static const struct i2c_device_id al3010_id[] = {
    { .name = "al3010" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, al3010_id);
    static const struct of_device_id al3010_of_match[] = {
    { .compatible = "dynaimage,al3010", },
    { }
    };
    MODULE_DEVICE_TABLE(of, al3010_of_match);
    static struct i2c_driver al3010_driver = {
    .driver = {
    .name = "al3010",
    .of_match_table = al3010_of_match,
    .pm = pm_sleep_ptr(&al3010_pm_ops),
    },
    .probe		= al3010_probe,
    .id_table	= al3010_id,
    };
    module_i2c_driver(al3010_driver);
    MODULE_AUTHOR("Daniel Baluta <daniel.baluta@nxp.com>");
    MODULE_AUTHOR("David Heidelberg <david@ixit.cz>");
    MODULE_DESCRIPTION("AL3010 Ambient Light Sensor driver");
    MODULE_LICENSE("GPL v2");
