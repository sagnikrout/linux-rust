//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/al3320a.c
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
// AL3320A - Dyna Image Ambient Light Sensor
//
// Copyright (c) 2014, Intel Corporation.
//
// IIO driver for AL3320A (7-bit I2C slave address 0x1C).
//
// TODO: interrupt support, thresholds
// When the driver will get support for interrupt handling, then interrupt
// will need to be disabled before turning sensor OFF in order to avoid
// potential races with the interrupt handling.
//

pub const AL3320A_REG_CONFIG: c_uint = 0x00;
pub const AL3320A_REG_STATUS: c_uint = 0x01;
pub const AL3320A_REG_INT: c_uint = 0x02;
pub const AL3320A_REG_WAIT: c_uint = 0x06;
pub const AL3320A_REG_CONFIG_RANGE: c_uint = 0x07;
pub const AL3320A_REG_PERSIST: c_uint = 0x08;
pub const AL3320A_REG_MEAN_TIME: c_uint = 0x09;
pub const AL3320A_REG_ADUMMY: c_uint = 0x0A;
pub const AL3320A_REG_DATA_LOW: c_uint = 0x22;
pub const AL3320A_REG_LOW_THRESH_LOW: c_uint = 0x30;
pub const AL3320A_REG_LOW_THRESH_HIGH: c_uint = 0x31;
pub const AL3320A_REG_HIGH_THRESH_LOW: c_uint = 0x32;
pub const AL3320A_REG_HIGH_THRESH_HIGH: c_uint = 0x33;
pub const AL3320A_CONFIG_DISABLE: c_uint = 0x00;
pub const AL3320A_CONFIG_ENABLE: c_uint = 0x01;

// chip params default values
pub const AL3320A_DEFAULT_MEAN_TIME: c_int = 4;

    enum al3320a_range {
    AL3320A_RANGE_1, /* 33.28 Klx */
    AL3320A_RANGE_2, /* 8.32 Klx  */
    AL3320A_RANGE_3, /* 2.08 Klx  */
    AL3320A_RANGE_4  /* 0.65 Klx  */
    };
    static const int al3320a_scales[][2] = {
    {0, 512000}, {0, 128000}, {0, 32000}, {0, 10000}
    };
    static const struct regmap_config al3320a_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = AL3320A_REG_HIGH_THRESH_HIGH,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct al3320a_data {
    pub regmap: *mut regmap,
}

    static const struct iio_chan_spec al3320a_channels[] = {
    {
    .type	= IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    }
    };
    static IIO_CONST_ATTR(in_illuminance_scale_available, AL3320A_SCALE_AVAILABLE);
    static struct attribute *al3320a_attributes[] = {
    &iio_const_attr_in_illuminance_scale_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group al3320a_attribute_group = {
    .attrs = al3320a_attributes,
    };
#[no_mangle]
unsafe extern "C" fn al3320a_set_pwr_on(data: *mut al3320a_data) -> c_int {
    static int al3320a_set_pwr_on(struct al3320a_data *data)
    {
    return regmap_write(data.regmap, AL3320A_REG_CONFIG, AL3320A_CONFIG_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn al3320a_set_pwr_off(_data: *mut c_void) {
    static void al3320a_set_pwr_off(void *_data)
    {
    struct al3320a_data *data = _data;
    struct device *dev = regmap_get_device(data.regmap);
    int ret;
    ret = regmap_write(data.regmap, AL3320A_REG_CONFIG, AL3320A_CONFIG_DISABLE);
    if (ret)
    dev_err(dev, "failed to write system register\n");
    }
#[no_mangle]
unsafe extern "C" fn al3320a_init(data: *mut al3320a_data) -> c_int {
    static int al3320a_init(struct al3320a_data *data)
    {
    struct device *dev = regmap_get_device(data.regmap);
    int ret;
    ret = al3320a_set_pwr_on(data);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, al3320a_set_pwr_off, data);
    if (ret)
    return ret;
    ret = regmap_write(data.regmap, AL3320A_REG_CONFIG_RANGE,
    FIELD_PREP(AL3320A_GAIN_MASK, AL3320A_RANGE_3));
    if (ret)
    return ret;
    ret = regmap_write(data.regmap, AL3320A_REG_MEAN_TIME,
    AL3320A_DEFAULT_MEAN_TIME);
    if (ret)
    return ret;
    return regmap_write(data.regmap, AL3320A_REG_WAIT,
    AL3320A_DEFAULT_WAIT_TIME);
    }
    static int al3320a_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct al3320a_data *data = iio_priv(indio_dev);
    int ret, gain;
    __le16 raw;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
//
// ALS ADC value is stored in two adjacent registers:
// - low byte of output is stored at AL3320A_REG_DATA_LOW
// - high byte of output is stored at AL3320A_REG_DATA_LOW + 1
//
    ret = regmap_bulk_read(data.regmap, AL3320A_REG_DATA_LOW,
    &raw, sizeof(raw));
    if (ret)
    return ret;
// val = le16_to_cpu(raw);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    ret = regmap_read(data.regmap, AL3320A_REG_CONFIG_RANGE, &gain);
    if (ret)
    return ret;
    gain = FIELD_GET(AL3320A_GAIN_MASK, gain);
// val = al3320a_scales[gain][0];
// val2 = al3320a_scales[gain][1];
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static int al3320a_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val,
    int val2, long mask)
    {
    struct al3320a_data *data = iio_priv(indio_dev);
    unsigned int i;
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    for (i = 0; i < ARRAY_SIZE(al3320a_scales); i++) {
    if (val != al3320a_scales[i][0] ||
    val2 != al3320a_scales[i][1])
    continue;
    return regmap_write(data.regmap, AL3320A_REG_CONFIG_RANGE,
    FIELD_PREP(AL3320A_GAIN_MASK, i));
    }
    break;
    }
    return -EINVAL;
    }
    static const struct iio_info al3320a_info = {
    .read_raw	= al3320a_read_raw,
    .write_raw	= al3320a_write_raw,
    .attrs		= &al3320a_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn al3320a_probe(client: *mut i2c_client) -> c_int {
    static int al3320a_probe(struct i2c_client *client)
    {
    struct al3320a_data *data;
    struct device *dev = &client.dev;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.regmap = devm_regmap_init_i2c(client, &al3320a_regmap_config);
    if (IS_ERR(data.regmap))
    return dev_err_probe(dev, PTR_ERR(data.regmap),
    "cannot allocate regmap\n");
    indio_dev.info = &al3320a_info;
    indio_dev.name = "al3320a";
    indio_dev.channels = al3320a_channels;
    indio_dev.num_channels = ARRAY_SIZE(al3320a_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = al3320a_init(data);
    if (ret < 0) {
    dev_err(dev, "al3320a chip init failed\n");
    return ret;
    }
    return devm_iio_device_register(dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn al3320a_suspend(dev: *mut device) -> c_int {
    static int al3320a_suspend(struct device *dev)
    {
    struct al3320a_data *data = iio_priv(dev_get_drvdata(dev));
    al3320a_set_pwr_off(data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn al3320a_resume(dev: *mut device) -> c_int {
    static int al3320a_resume(struct device *dev)
    {
    struct al3320a_data *data = iio_priv(dev_get_drvdata(dev));
    return al3320a_set_pwr_on(data);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(al3320a_pm_ops, al3320a_suspend,
    al3320a_resume);
    static const struct i2c_device_id al3320a_id[] = {
    { .name = "al3320a" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, al3320a_id);
    static const struct of_device_id al3320a_of_match[] = {
    { .compatible = "dynaimage,al3320a", },
    { }
    };
    MODULE_DEVICE_TABLE(of, al3320a_of_match);
    static const struct acpi_device_id al3320a_acpi_match[] = {
    {"CALS0001"},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, al3320a_acpi_match);
    static struct i2c_driver al3320a_driver = {
    .driver = {
    .name = "al3320a",
    .of_match_table = al3320a_of_match,
    .pm = pm_sleep_ptr(&al3320a_pm_ops),
    .acpi_match_table = al3320a_acpi_match,
    },
    .probe		= al3320a_probe,
    .id_table	= al3320a_id,
    };
    module_i2c_driver(al3320a_driver);
    MODULE_AUTHOR("Daniel Baluta <daniel.baluta@intel.com>");
    MODULE_DESCRIPTION("AL3320A Ambient Light Sensor driver");
    MODULE_LICENSE("GPL v2");
