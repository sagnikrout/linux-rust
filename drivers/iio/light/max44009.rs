//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/max44009.c
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
// max44009.c - Support for MAX44009 Ambient Light Sensor
//
// Copyright (c) 2019 Robert Eshleman <bobbyeshleman@gmail.com>
//
// Datasheet: https://datasheets.maximintegrated.com/en/ds/MAX44009.pdf
//
// TODO: Support continuous mode and configuring from manual mode to
// automatic mode.
//
// Default I2C address: 0x4a
//

// Registers in datasheet order
pub const MAX44009_REG_INT_STATUS: c_uint = 0x0;
pub const MAX44009_REG_INT_EN: c_uint = 0x1;
pub const MAX44009_REG_CFG: c_uint = 0x2;
pub const MAX44009_REG_LUX_HI: c_uint = 0x3;
pub const MAX44009_REG_LUX_LO: c_uint = 0x4;
pub const MAX44009_REG_UPPER_THR: c_uint = 0x5;
pub const MAX44009_REG_LOWER_THR: c_uint = 0x6;
pub const MAX44009_REG_THR_TIMER: c_uint = 0x7;

// The maximum rising threshold for the max44009
pub const MAX44009_MAXIMUM_THRESHOLD: c_int = 7520256;

pub const MAX44009_THRESH_EXP_RSHIFT: c_int = 4;
pub const MAX44009_THRESH_MANT_LSHIFT: c_int = 4;
pub const MAX44009_THRESH_MANT_MASK: c_uint = 0xf;
pub const MAX44009_UPPER_THR_MINIMUM: c_int = 15;
// The max44009 always scales raw readings by 0.045 and is non-configurable
pub const MAX44009_SCALE_NUMERATOR: c_int = 45;
pub const MAX44009_SCALE_DENOMINATOR: c_int = 1000;
// The fixed-point fractional multiplier for de-scaling threshold values
pub const MAX44009_FRACT_MULT: c_int = 1000000;
    static const u32 max44009_int_time_ns_array[] = {
    800000000,
    400000000,
    200000000,
    100000000,
    50000000, /* Manual mode only */
    25000000, /* Manual mode only */
    12500000, /* Manual mode only */
    6250000,  /* Manual mode only */
    };
    static const char max44009_int_time_str[] =
    "0.8 "
    "0.4 "
    "0.2 "
    "0.1 "
    "0.05 "
    "0.025 "
    "0.0125 "
    "0.00625";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max44009_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
}

    static const struct iio_event_spec max44009_event_spec[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE),
    },
    };
    static const struct iio_chan_spec max44009_channels[] = {
    {
    .type = IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED) |
    BIT(IIO_CHAN_INFO_INT_TIME),
    .event_spec = max44009_event_spec,
    .num_event_specs = ARRAY_SIZE(max44009_event_spec),
    },
    };
#[no_mangle]
unsafe extern "C" fn max44009_read_int_time(data: *mut max44009_data) -> c_int {
    static int max44009_read_int_time(struct max44009_data *data)
    {
    let mut ret: c_int = i2c_smbus_read_byte_data(data.client, MAX44009_REG_CFG);
    if (ret < 0)
    return ret;
    return max44009_int_time_ns_array[ret & MAX44009_CFG_TIM_MASK];
    }
    static int max44009_write_int_time(struct max44009_data *data,
    int val, int val2)
    {
    struct i2c_client *client = data.client;
    int ret, int_time, config;
    s64 ns;
    ns = val * NSEC_PER_SEC + val2;
    int_time = find_closest_descending(
    ns,
    max44009_int_time_ns_array,
    ARRAY_SIZE(max44009_int_time_ns_array));
    ret = i2c_smbus_read_byte_data(client, MAX44009_REG_CFG);
    if (ret < 0)
    return ret;
    config = ret;
    config &= int_time;
//
// To set the integration time, the device must also be in manual
// mode.
//
    config |= MAX44009_CFG_MAN_MODE_MASK;
    return i2c_smbus_write_byte_data(client, MAX44009_REG_CFG, config);
    }
    static int max44009_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val,
    int val2, long mask)
    {
    struct max44009_data *data = iio_priv(indio_dev);
    int ret;
    if (mask == IIO_CHAN_INFO_INT_TIME && chan.type == IIO_LIGHT) {
    mutex_lock(&data.lock);
    ret = max44009_write_int_time(data, val, val2);
    mutex_unlock(&data.lock);
    return ret;
    }
    return -EINVAL;
    }
    static int max44009_write_raw_get_fmt(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    long mask)
    {
    return IIO_VAL_INT_PLUS_NANO;
    }
#[no_mangle]
unsafe extern "C" fn max44009_lux_raw(hi: u8, lo: u8) -> c_int {
    static int max44009_lux_raw(u8 hi, u8 lo)
    {
    int mantissa;
    int exponent;
//
// The mantissa consists of the low nibble of the Lux High Byte
// and the low nibble of the Lux Low Byte.
//
    mantissa = ((hi & 0xf) << 4) | (lo & 0xf);
// The exponent byte is just the upper nibble of the Lux High Byte
    exponent = (hi >> 4) & 0xf;
//
// The exponent value is base 2 to the power of the raw exponent byte.
//
    exponent = 1 << exponent;
    return exponent * mantissa;
    }

#[no_mangle]
unsafe extern "C" fn max44009_read_lux_raw(data: *mut max44009_data) -> c_int {
    static int max44009_read_lux_raw(struct max44009_data *data)
    {
    int ret;
    let mut hireg: u8 = MAX44009_REG_LUX_HI;
    let mut loreg: u8 = MAX44009_REG_LUX_LO;
    let mut lo: u8 = 0;
    let mut hi: u8 = 0;
    struct i2c_msg msgs[] = {
    {
    .addr = data.client.addr,
    .flags = 0,
    .len = sizeof(hireg),
    .buf = &hireg,
    },
    {
    .addr = data.client.addr,
    .flags = I2C_M_RD,
    .len = sizeof(hi),
    .buf = &hi,
    },
    {
    .addr = data.client.addr,
    .flags = 0,
    .len = sizeof(loreg),
    .buf = &loreg,
    },
    {
    .addr = data.client.addr,
    .flags = I2C_M_RD,
    .len = sizeof(lo),
    .buf = &lo,
    }
    };
//
// Use i2c_transfer instead of smbus read because i2c_transfer
// does NOT use a stop bit between address write and data read.
// Using a stop bit causes disjoint upper/lower byte reads and
// reduces accuracy.
//
    ret = i2c_transfer(data.client.adapter,
    msgs, MAX44009_READ_LUX_XFER_LEN);
    if (ret != MAX44009_READ_LUX_XFER_LEN)
    return -EIO;
    return max44009_lux_raw(hi, lo);
    }
    static int max44009_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct max44009_data *data = iio_priv(indio_dev);
    int lux_raw;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_PROCESSED:
    switch (chan.type) {
    case IIO_LIGHT:
    ret = max44009_read_lux_raw(data);
    if (ret < 0)
    return ret;
    lux_raw = ret;
// val = lux_raw * MAX44009_SCALE_NUMERATOR;
// val2 = MAX44009_SCALE_DENOMINATOR;
    return IIO_VAL_FRACTIONAL;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_INT_TIME:
    switch (chan.type) {
    case IIO_LIGHT:
    ret = max44009_read_int_time(data);
    if (ret < 0)
    return ret;
// val2 = ret;
// val = 0;
    return IIO_VAL_INT_PLUS_NANO;
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static IIO_CONST_ATTR(illuminance_integration_time_available,
    max44009_int_time_str);
    static struct attribute *max44009_attributes[] = {
    &iio_const_attr_illuminance_integration_time_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group max44009_attribute_group = {
    .attrs = max44009_attributes,
    };
#[no_mangle]
unsafe extern "C" fn max44009_threshold_byte_from_fraction(integral: c_int, fractional: c_int) -> c_int {
    static int max44009_threshold_byte_from_fraction(int integral, int fractional)
    {
    int mantissa, exp;
    if ((integral <= 0 && fractional <= 0) ||
    integral > MAX44009_MAXIMUM_THRESHOLD ||
    (integral == MAX44009_MAXIMUM_THRESHOLD && fractional != 0))
    return -EINVAL;
// Reverse scaling of fixed-point integral
    mantissa = integral * MAX44009_SCALE_DENOMINATOR;
    mantissa /= MAX44009_SCALE_NUMERATOR;
// Reverse scaling of fixed-point fractional
    mantissa += fractional / MAX44009_FRACT_MULT *
    (MAX44009_SCALE_DENOMINATOR / MAX44009_SCALE_NUMERATOR);
    for (exp = 0; mantissa > 0xff; exp++)
    mantissa >>= 1;
    mantissa >>= 4;
    mantissa &= 0xf;
    exp <<= 4;
    return exp | mantissa;
    }
#[no_mangle]
unsafe extern "C" fn max44009_get_thr_reg(dir: enum iio_event_direction) -> c_int {
    static int max44009_get_thr_reg(enum iio_event_direction dir)
    {
    switch (dir) {
    case IIO_EV_DIR_RISING:
    return MAX44009_REG_UPPER_THR;
    case IIO_EV_DIR_FALLING:
    return MAX44009_REG_LOWER_THR;
    default:
    return -EINVAL;
    }
    }
    static int max44009_write_event_value(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int val, int val2)
    {
    struct max44009_data *data = iio_priv(indio_dev);
    int reg, threshold;
    if (info != IIO_EV_INFO_VALUE || chan.type != IIO_LIGHT)
    return -EINVAL;
    threshold = max44009_threshold_byte_from_fraction(val, val2);
    if (threshold < 0)
    return threshold;
    reg = max44009_get_thr_reg(dir);
    if (reg < 0)
    return reg;
    return i2c_smbus_write_byte_data(data.client, reg, threshold);
    }
    static int max44009_read_threshold(struct iio_dev *indio_dev,
    enum iio_event_direction dir)
    {
    struct max44009_data *data = iio_priv(indio_dev);
    int byte, reg;
    int mantissa, exponent;
    reg = max44009_get_thr_reg(dir);
    if (reg < 0)
    return reg;
    byte = i2c_smbus_read_byte_data(data.client, reg);
    if (byte < 0)
    return byte;
    mantissa = byte & MAX44009_THRESH_MANT_MASK;
    mantissa <<= MAX44009_THRESH_MANT_LSHIFT;
//
// To get the upper threshold, always adds the minimum upper threshold
// value to the shifted byte value (see datasheet).
//
    if (dir == IIO_EV_DIR_RISING)
    mantissa += MAX44009_UPPER_THR_MINIMUM;
//
// Exponent is base 2 to the power of the threshold exponent byte
// value
//
    exponent = byte & MAX44009_THRESH_EXP_MASK;
    exponent >>= MAX44009_THRESH_EXP_RSHIFT;
    return (1 << exponent) * mantissa;
    }
    static int max44009_read_event_value(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int *val, int *val2)
    {
    int ret;
    int threshold;
    if (chan.type != IIO_LIGHT || type != IIO_EV_TYPE_THRESH)
    return -EINVAL;
    ret = max44009_read_threshold(indio_dev, dir);
    if (ret < 0)
    return ret;
    threshold = ret;
// val = threshold * MAX44009_SCALE_NUMERATOR;
// val2 = MAX44009_SCALE_DENOMINATOR;
    return IIO_VAL_FRACTIONAL;
    }
    static int max44009_write_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    bool state)
    {
    struct max44009_data *data = iio_priv(indio_dev);
    int ret;
    if (chan.type != IIO_LIGHT || type != IIO_EV_TYPE_THRESH)
    return -EINVAL;
    ret = i2c_smbus_write_byte_data(data.client,
    MAX44009_REG_INT_EN, state);
    if (ret < 0)
    return ret;
//
// Set device to trigger interrupt immediately upon exceeding
// the threshold limit.
//
    return i2c_smbus_write_byte_data(data.client,
    MAX44009_REG_THR_TIMER, 0);
    }
    static int max44009_read_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct max44009_data *data = iio_priv(indio_dev);
    if (chan.type != IIO_LIGHT || type != IIO_EV_TYPE_THRESH)
    return -EINVAL;
    return i2c_smbus_read_byte_data(data.client, MAX44009_REG_INT_EN);
    }
    static const struct iio_info max44009_info = {
    .read_raw = max44009_read_raw,
    .write_raw = max44009_write_raw,
    .write_raw_get_fmt = max44009_write_raw_get_fmt,
    .read_event_value = max44009_read_event_value,
    .read_event_config = max44009_read_event_config,
    .write_event_value = max44009_write_event_value,
    .write_event_config = max44009_write_event_config,
    .attrs = &max44009_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn max44009_threaded_irq_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t max44009_threaded_irq_handler(int irq, void *p)
    {
    struct iio_dev *indio_dev = p;
    struct max44009_data *data = iio_priv(indio_dev);
    int ret;
    ret = i2c_smbus_read_byte_data(data.client, MAX44009_REG_INT_STATUS);
    if (ret) {
    iio_push_event(indio_dev,
    IIO_UNMOD_EVENT_CODE(IIO_LIGHT, 0,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_EITHER),
    iio_get_time_ns(indio_dev));
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn max44009_probe(client: *mut i2c_client) -> c_int {
    static int max44009_probe(struct i2c_client *client)
    {
    struct max44009_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    indio_dev.info = &max44009_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.name = MAX44009_DRV_NAME;
    indio_dev.channels = max44009_channels;
    indio_dev.num_channels = ARRAY_SIZE(max44009_channels);
    mutex_init(&data.lock);
// Clear any stale interrupt bit
    ret = i2c_smbus_read_byte_data(client, MAX44009_REG_CFG);
    if (ret < 0)
    return ret;
    if (client.irq > 0) {
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(),
    max44009_threaded_irq_handler,
    IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT | IRQF_SHARED,
    "max44009_event",
    indio_dev);
    if (ret < 0)
    return ret;
    }
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct of_device_id max44009_of_match[] = {
    { .compatible = "maxim,max44009" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max44009_of_match);
    static const struct i2c_device_id max44009_id[] = {
    { .name = "max44009" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max44009_id);
    static struct i2c_driver max44009_driver = {
    .driver = {
    .name = MAX44009_DRV_NAME,
    .of_match_table = max44009_of_match,
    },
    .probe = max44009_probe,
    .id_table = max44009_id,
    };
    module_i2c_driver(max44009_driver);
    MODULE_AUTHOR("Robert Eshleman <bobbyeshleman@gmail.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("MAX44009 ambient light sensor driver");
