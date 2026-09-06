//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ltc2309.c
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
// The LTC2305 is a  2-Channel, 12-Bit SAR ADC with an I2C Interface.
// The LTC2309 is an 8-Channel, 12-Bit SAR ADC with an I2C Interface.
//
// Datasheet:
// https://www.analog.com/media/en/technical-documentation/data-sheets/23015fb.pdf
// https://www.analog.com/media/en/technical-documentation/data-sheets/2309fd.pdf
//
// Copyright (c) 2023, Liam Beguin <liambeguin@gmail.com>
//

pub const LTC2309_ADC_RESOLUTION: c_int = 12;
pub const LTC2309_INTERNAL_REF_MV: c_int = 4096;

//
// struct ltc2309 - internal device data structure
// @dev:	Device reference
// @client:	I2C reference
// @lock:	Lock to serialize data access
// @vref_mv:	Internal voltage reference
// @read_delay_us:	Chip-specific read delay in microseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2309 {
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub /: *mut *mut mutex lock; / serialize data access,
    pub vref_mv: c_int,
    pub read_delay_us: c_uint,
}

// Order matches expected channel address, See datasheet Table 1.
    enum ltc2305_channels {
    LTC2305_CH0_CH1 = 0x0,
    LTC2305_CH1_CH0 = 0x4,
    LTC2305_CH0     = 0x8,
    LTC2305_CH1     = 0xc,
    };
    enum ltc2309_channels {
    LTC2309_CH0_CH1 = 0x0,
    LTC2309_CH2_CH3 = 0x1,
    LTC2309_CH4_CH5 = 0x2,
    LTC2309_CH6_CH7 = 0x3,
    LTC2309_CH1_CH0 = 0x4,
    LTC2309_CH3_CH2 = 0x5,
    LTC2309_CH5_CH4 = 0x6,
    LTC2309_CH7_CH6 = 0x7,
    LTC2309_CH0     = 0x8,
    LTC2309_CH2     = 0x9,
    LTC2309_CH4     = 0xa,
    LTC2309_CH6     = 0xb,
    LTC2309_CH1     = 0xc,
    LTC2309_CH3     = 0xd,
    LTC2309_CH5     = 0xe,
    LTC2309_CH7     = 0xf,
    };

    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .address = _addr,					\
    .channel = _chan,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    }

    .type = IIO_VOLTAGE,					\
    .differential = 1,					\
    .indexed = 1,						\
    .address = _addr,					\
    .channel = _chan,					\
    .channel2 = _chan2,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    }
    static const struct iio_chan_spec ltc2305_channels[] = {
    LTC2309_CHAN(0, LTC2305_CH0),
    LTC2309_CHAN(1, LTC2305_CH1),
    LTC2309_DIFF_CHAN(0, 1, LTC2305_CH0_CH1),
    LTC2309_DIFF_CHAN(1, 0, LTC2305_CH1_CH0),
    };
    static const struct iio_chan_spec ltc2309_channels[] = {
    LTC2309_CHAN(0, LTC2309_CH0),
    LTC2309_CHAN(1, LTC2309_CH1),
    LTC2309_CHAN(2, LTC2309_CH2),
    LTC2309_CHAN(3, LTC2309_CH3),
    LTC2309_CHAN(4, LTC2309_CH4),
    LTC2309_CHAN(5, LTC2309_CH5),
    LTC2309_CHAN(6, LTC2309_CH6),
    LTC2309_CHAN(7, LTC2309_CH7),
    LTC2309_DIFF_CHAN(0, 1, LTC2309_CH0_CH1),
    LTC2309_DIFF_CHAN(2, 3, LTC2309_CH2_CH3),
    LTC2309_DIFF_CHAN(4, 5, LTC2309_CH4_CH5),
    LTC2309_DIFF_CHAN(6, 7, LTC2309_CH6_CH7),
    LTC2309_DIFF_CHAN(1, 0, LTC2309_CH1_CH0),
    LTC2309_DIFF_CHAN(3, 2, LTC2309_CH3_CH2),
    LTC2309_DIFF_CHAN(5, 4, LTC2309_CH5_CH4),
    LTC2309_DIFF_CHAN(7, 6, LTC2309_CH7_CH6),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2309_chip_info {
    pub name: *const c_char,
    pub read_delay_us: c_uint,
    pub num_channels: c_int,
    pub __counted_by_ptr(num_channels): *const *const iio_chan_spec channels,
}

    static const struct ltc2309_chip_info ltc2305_chip_info = {
    .name = "ltc2305",
    .read_delay_us = 2,
    .num_channels = ARRAY_SIZE(ltc2305_channels),
    .channels = ltc2305_channels,
    };
    static const struct ltc2309_chip_info ltc2309_chip_info = {
    .name = "ltc2309",
    .num_channels = ARRAY_SIZE(ltc2309_channels),
    .channels = ltc2309_channels,
    };
    static int ltc2309_read_raw_channel(struct ltc2309 *ltc2309,
    unsigned long address, int *val)
    {
    int ret;
    __be16 buf;
    u8 din;
    din = FIELD_PREP(LTC2309_DIN_CH_MASK, address & 0x0f) |
    FIELD_PREP(LTC2309_DIN_UNI, 1) |
    FIELD_PREP(LTC2309_DIN_SLEEP, 0);
    ret = i2c_smbus_write_byte(ltc2309.client, din);
    if (ret < 0) {
    dev_err(ltc2309.dev, "i2c command failed: %pe\n",
    ERR_PTR(ret));
    return ret;
    }
    if (ltc2309.read_delay_us)
    fsleep(ltc2309.read_delay_us);
    ret = i2c_master_recv(ltc2309.client, (char *)&buf, 2);
    if (ret < 0) {
    dev_err(ltc2309.dev, "i2c read failed: %pe\n", ERR_PTR(ret));
    return ret;
    }
// val = be16_to_cpu(buf) >> 4;
    return ret;
    }
    static int ltc2309_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct ltc2309 *ltc2309 = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&ltc2309.lock);
    ret = ltc2309_read_raw_channel(ltc2309, chan.address, val);
    mutex_unlock(&ltc2309.lock);
    if (ret < 0)
    return -EINVAL;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = ltc2309->vref_mv;
// val2 = LTC2309_ADC_RESOLUTION;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info ltc2309_info = {
    .read_raw = ltc2309_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ltc2309_probe(client: *mut i2c_client) -> c_int {
    static int ltc2309_probe(struct i2c_client *client)
    {
    const struct ltc2309_chip_info *chip_info;
    struct iio_dev *indio_dev;
    struct ltc2309 *ltc2309;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*ltc2309));
    if (!indio_dev)
    return -ENOMEM;
    ltc2309 = iio_priv(indio_dev);
    chip_info = i2c_get_match_data(client);
    ltc2309.dev = &indio_dev.dev;
    ltc2309.client = client;
    ltc2309.read_delay_us = chip_info.read_delay_us;
    indio_dev.name = chip_info.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = chip_info.channels;
    indio_dev.num_channels = chip_info.num_channels;
    indio_dev.info = &ltc2309_info;
    ret = devm_regulator_get_enable_read_voltage(&client.dev, "vref");
    if (ret < 0 && ret != -ENODEV)
    return dev_err_probe(ltc2309.dev, ret,
    "failed to get vref voltage\n");
    ltc2309.vref_mv = ret == -ENODEV ? LTC2309_INTERNAL_REF_MV : ret / 1000;
    mutex_init(&ltc2309.lock);
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct of_device_id ltc2309_of_match[] = {
    { .compatible = "lltc,ltc2305", .data = &ltc2305_chip_info },
    { .compatible = "lltc,ltc2309", .data = &ltc2309_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, ltc2309_of_match);
    static const struct i2c_device_id ltc2309_id[] = {
    { .name = "ltc2305", .driver_data = (kernel_ulong_t)&ltc2305_chip_info },
    { .name = "ltc2309", .driver_data = (kernel_ulong_t)&ltc2309_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc2309_id);
    static struct i2c_driver ltc2309_driver = {
    .driver = {
    .name = "ltc2309",
    .of_match_table = ltc2309_of_match,
    },
    .probe		= ltc2309_probe,
    .id_table	= ltc2309_id,
    };
    module_i2c_driver(ltc2309_driver);
    MODULE_AUTHOR("Liam Beguin <liambeguin@gmail.com>");
    MODULE_DESCRIPTION("Linear Technology LTC2309 ADC");
    MODULE_LICENSE("GPL v2");
