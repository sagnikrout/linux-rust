//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/max34408.c
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
// IIO driver for Maxim MAX34409/34408 ADC, 4-Channels/2-Channels, 8bits, I2C
//
// Datasheet: https://www.analog.com/media/en/technical-documentation/data-sheets/MAX34408-MAX34409.pdf
//
// TODO: ALERT interrupt, Overcurrent delay, Shutdown delay
//

pub const MAX34408_STATUS_REG: c_uint = 0x0;
pub const MAX34408_CONTROL_REG: c_uint = 0x1;
pub const MAX34408_OCDELAY_REG: c_uint = 0x2;
pub const MAX34408_SDDELAY_REG: c_uint = 0x3;
pub const MAX34408_ADC1_REG: c_uint = 0x4;
pub const MAX34408_ADC2_REG: c_uint = 0x5;
// ADC3 & ADC4 always returns 0x0 on 34408
pub const MAX34409_ADC3_REG: c_uint = 0x6;
pub const MAX34409_ADC4_REG: c_uint = 0x7;
pub const MAX34408_OCT1_REG: c_uint = 0x8;
pub const MAX34408_OCT2_REG: c_uint = 0x9;
pub const MAX34409_OCT3_REG: c_uint = 0xA;
pub const MAX34409_OCT4_REG: c_uint = 0xB;
pub const MAX34408_DID_REG: c_uint = 0xC;
pub const MAX34408_DCYY_REG: c_uint = 0xD;
pub const MAX34408_DCWW_REG: c_uint = 0xE;
// Bit masks for status register

// Bit masks for control register

pub const MAX34408_DEFAULT_AVG: c_uint = 0x4;
// Bit masks for over current delay

// Bit masks for shutdown delay

pub const MAX34408_DEFAULT_RSENSE: c_int = 1000;
//
// struct max34408_data - max34408/max34409 specific data.
// @regmap:	device register map.
// @dev:	max34408 device.
// @lock:	lock for protecting access to device hardware registers, mostly
// for read modify write cycles for control registers.
// @input_rsense:	Rsense values in uOhm, will be overwritten by
// values from channel nodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max34408_data {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub lock: mutex,
    pub input_rsense: [u32; 4],
}

    static const struct regmap_config max34408_regmap_config = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .max_register	= MAX34408_DCWW_REG,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max34408_adc_model_data {
    pub model_name: *const c_char,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_int,
}

    {							\
    .type = IIO_CURRENT,				\
    .info_mask_separate	= BIT(IIO_CHAN_INFO_RAW) | \
    BIT(IIO_CHAN_INFO_SCALE) | \
    BIT(IIO_CHAN_INFO_OFFSET), \
    .channel = (_index),				\
    .address = (_address),				\
    .indexed = 1,					\
    }
    static const struct iio_chan_spec max34408_channels[] = {
    MAX34008_CHANNEL(0, MAX34408_ADC1_REG),
    MAX34008_CHANNEL(1, MAX34408_ADC2_REG),
    };
    static const struct iio_chan_spec max34409_channels[] = {
    MAX34008_CHANNEL(0, MAX34408_ADC1_REG),
    MAX34008_CHANNEL(1, MAX34408_ADC2_REG),
    MAX34008_CHANNEL(2, MAX34409_ADC3_REG),
    MAX34008_CHANNEL(3, MAX34409_ADC4_REG),
    };
    static int max34408_read_adc_avg(struct max34408_data *max34408,
    const struct iio_chan_spec *chan, int *val)
    {
    unsigned int ctrl;
    int rc;
    guard(mutex)(&max34408.lock);
    rc = regmap_read(max34408.regmap, MAX34408_CONTROL_REG, (u32 *)&ctrl);
    if (rc)
    return rc;
// set averaging (0b100) default values
    rc = regmap_write(max34408.regmap, MAX34408_CONTROL_REG,
    MAX34408_DEFAULT_AVG);
    if (rc) {
    dev_err(max34408.dev,
    "Error (%d) writing control register\n", rc);
    return rc;
    }
    rc = regmap_read(max34408.regmap, chan.address, val);
    if (rc)
    return rc;
// back to old values
    rc = regmap_write(max34408.regmap, MAX34408_CONTROL_REG, ctrl);
    if (rc)
    dev_err(max34408.dev,
    "Error (%d) writing control register\n", rc);
    return rc;
    }
    static int max34408_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max34408_data *max34408 = iio_priv(indio_dev);
    int rc;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    rc = max34408_read_adc_avg(max34408, chan, val);
    if (rc)
    return rc;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
//
// calculate current for 8bit ADC with Rsense
// value.
// 10 mV * 1000 / Rsense uOhm = max current
// (max current * adc val * 1000) / (2^8 - 1) mA
//
// val = 10000 / max34408->input_rsense[chan->channel];
// val2 = 8;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info max34408_info = {
    .read_raw	= max34408_read_raw,
    };
    static const struct max34408_adc_model_data max34408_model_data = {
    .model_name = "max34408",
    .channels = max34408_channels,
    .num_channels = 2,
    };
    static const struct max34408_adc_model_data max34409_model_data = {
    .model_name = "max34409",
    .channels = max34409_channels,
    .num_channels = 4,
    };
#[no_mangle]
unsafe extern "C" fn max34408_probe(client: *mut i2c_client) -> c_int {
    static int max34408_probe(struct i2c_client *client)
    {
    const struct max34408_adc_model_data *model_data;
    struct device *dev = &client.dev;
    struct max34408_data *max34408;
    struct fwnode_handle *node;
    struct iio_dev *indio_dev;
    struct regmap *regmap;
    int rc, i = 0;
    model_data = i2c_get_match_data(client);
    if (!model_data)
    return -EINVAL;
    regmap = devm_regmap_init_i2c(client, &max34408_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err_probe(dev, PTR_ERR(regmap),
    "regmap_init failed\n");
    return PTR_ERR(regmap);
    }
    indio_dev = devm_iio_device_alloc(dev, sizeof(*max34408));
    if (!indio_dev)
    return -ENOMEM;
    max34408 = iio_priv(indio_dev);
    max34408.regmap = regmap;
    max34408.dev = dev;
    mutex_init(&max34408.lock);
    device_for_each_child_node(dev, node) {
    fwnode_property_read_u32(node, "maxim,rsense-val-micro-ohms",
    &max34408.input_rsense[i]);
    i++;
    }
// disable ALERT and averaging
    rc = regmap_write(max34408.regmap, MAX34408_CONTROL_REG, 0x0);
    if (rc)
    return rc;
    indio_dev.channels = model_data.channels;
    indio_dev.num_channels = model_data.num_channels;
    indio_dev.name = model_data.model_name;
    indio_dev.info = &max34408_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id max34408_of_match[] = {
    {
    .compatible = "maxim,max34408",
    .data = &max34408_model_data,
    },
    {
    .compatible = "maxim,max34409",
    .data = &max34409_model_data,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, max34408_of_match);
    static const struct i2c_device_id max34408_id[] = {
    { .name = "max34408", .driver_data = (kernel_ulong_t)&max34408_model_data },
    { .name = "max34409", .driver_data = (kernel_ulong_t)&max34409_model_data },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max34408_id);
    static struct i2c_driver max34408_driver = {
    .driver = {
    .name   = "max34408",
    .of_match_table = max34408_of_match,
    },
    .probe = max34408_probe,
    .id_table = max34408_id,
    };
    module_i2c_driver(max34408_driver);
    MODULE_AUTHOR("Ivan Mikhaylov <fr0st61te@gmail.com>");
    MODULE_DESCRIPTION("Maxim MAX34408/34409 ADC driver");
    MODULE_LICENSE("GPL");
