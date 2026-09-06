//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/tcs3414.c
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
// tcs3414.c - Support for TAOS TCS3414 digital color sensor
//
// Copyright (c) 2014 Peter Meerwald <pmeerw@pmeerw.net>
//
// Digital color sensor with 16-bit channels for red, green, blue, clear);
// 7-bit I2C slave address 0x39 (TCS3414) or 0x29, 0x49, 0x59 (TCS3413,
// TCS3415, TCS3416, resp.)
//
// TODO: sync, interrupt support, thresholds, prescaler
//

pub const TCS3414_INTEG_12MS: c_uint = 0x0;
pub const TCS3414_INTEG_100MS: c_uint = 0x1;
pub const TCS3414_INTEG_400MS: c_uint = 0x2;

pub const TCS3414_GAIN_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcs3414_data {
    pub client: *mut i2c_client,
    pub control: u8,
    pub gain: u8,
    pub timing: u8,
}

    .type = IIO_INTENSITY, \
    .modified = 1, \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW), \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) | \
    BIT(IIO_CHAN_INFO_INT_TIME), \
    .channel2 = IIO_MOD_LIGHT_##_color, \
    .address = _addr, \
    .scan_index = _si, \
    .scan_type = { \
    .sign = 'u', \
    .realbits = 16, \
    .storagebits = 16, \
    .endianness = IIO_CPU, \
    }, \
    }
// scale factors: 1/gain
    static const int tcs3414_scales[][2] = {
    {1, 0}, {0, 250000}, {0, 62500}, {0, 15625}
    };
// integration time in ms
    static const int tcs3414_times[] = { 12, 100, 400 };
    static const struct iio_chan_spec tcs3414_channels[] = {
    TCS3414_CHANNEL(GREEN, 0, TCS3414_DATA_GREEN),
    TCS3414_CHANNEL(RED, 1, TCS3414_DATA_RED),
    TCS3414_CHANNEL(BLUE, 2, TCS3414_DATA_BLUE),
    TCS3414_CHANNEL(CLEAR, 3, TCS3414_DATA_CLEAR),
    IIO_CHAN_SOFT_TIMESTAMP(4),
    };
#[no_mangle]
unsafe extern "C" fn tcs3414_req_data(data: *mut tcs3414_data) -> c_int {
    static int tcs3414_req_data(struct tcs3414_data *data)
    {
    let mut tries: c_int = 25;
    int ret;
    ret = i2c_smbus_write_byte_data(data.client, TCS3414_CONTROL,
    data.control | TCS3414_CONTROL_ADC_EN);
    if (ret < 0)
    return ret;
    while (tries--) {
    ret = i2c_smbus_read_byte_data(data.client, TCS3414_CONTROL);
    if (ret < 0)
    return ret;
    if (ret & TCS3414_CONTROL_ADC_VALID)
    break;
    msleep(20);
    }
    ret = i2c_smbus_write_byte_data(data.client, TCS3414_CONTROL,
    data.control);
    if (ret < 0)
    return ret;
    if (tries < 0) {
    dev_err(&data.client.dev, "data not ready\n");
    return -EIO;
    }
    return 0;
    }
    static int tcs3414_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct tcs3414_data *data = iio_priv(indio_dev);
    int i, ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = tcs3414_req_data(data);
    if (ret < 0) {
    iio_device_release_direct(indio_dev);
    return ret;
    }
    ret = i2c_smbus_read_word_data(data.client, chan.address);
    iio_device_release_direct(indio_dev);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    i = (data.gain & TCS3414_GAIN_MASK) >> TCS3414_GAIN_SHIFT;
// val = tcs3414_scales[i][0];
// val2 = tcs3414_scales[i][1];
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_INT_TIME:
// val = 0;
// val2 = tcs3414_times[data->timing & TCS3414_INTEG_MASK] * 1000;
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static int tcs3414_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct tcs3414_data *data = iio_priv(indio_dev);
    int i;
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    for (i = 0; i < ARRAY_SIZE(tcs3414_scales); i++) {
    if (val == tcs3414_scales[i][0] &&
    val2 == tcs3414_scales[i][1]) {
    data.gain &= ~TCS3414_GAIN_MASK;
    data.gain |= i << TCS3414_GAIN_SHIFT;
    return i2c_smbus_write_byte_data(
    data.client, TCS3414_GAIN,
    data.gain);
    }
    }
    return -EINVAL;
    case IIO_CHAN_INFO_INT_TIME:
    if (val != 0)
    return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(tcs3414_times); i++) {
    if (val2 == tcs3414_times[i] * 1000) {
    data.timing &= ~TCS3414_INTEG_MASK;
    data.timing |= i;
    return i2c_smbus_write_byte_data(
    data.client, TCS3414_TIMING,
    data.timing);
    }
    }
    return -EINVAL;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn tcs3414_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t tcs3414_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct tcs3414_data *data = iio_priv(indio_dev);
    int i, j = 0;
// Ensure timestamp is naturally aligned
    struct {
    u16 chans[4];
    aligned_s64 timestamp;
    } scan = { };
    iio_for_each_active_channel(indio_dev, i) {
    int ret = i2c_smbus_read_word_data(data.client,
    TCS3414_DATA_GREEN + 2*i);
    if (ret < 0)
    goto done;
    scan.chans[j++] = ret;
    }
    iio_push_to_buffers_with_ts(indio_dev, &scan, sizeof(scan),
    iio_get_time_ns(indio_dev));
    done:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static IIO_CONST_ATTR(scale_available, "1 0.25 0.0625 0.015625");
    static IIO_CONST_ATTR_INT_TIME_AVAIL("0.012 0.1 0.4");
    static struct attribute *tcs3414_attributes[] = {
    &iio_const_attr_scale_available.dev_attr.attr,
    &iio_const_attr_integration_time_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group tcs3414_attribute_group = {
    .attrs = tcs3414_attributes,
    };
    static const struct iio_info tcs3414_info = {
    .read_raw = tcs3414_read_raw,
    .write_raw = tcs3414_write_raw,
    .attrs = &tcs3414_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn tcs3414_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int tcs3414_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct tcs3414_data *data = iio_priv(indio_dev);
    data.control |= TCS3414_CONTROL_ADC_EN;
    return i2c_smbus_write_byte_data(data.client, TCS3414_CONTROL,
    data.control);
    }
#[no_mangle]
unsafe extern "C" fn tcs3414_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int tcs3414_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct tcs3414_data *data = iio_priv(indio_dev);
    data.control &= ~TCS3414_CONTROL_ADC_EN;
    return i2c_smbus_write_byte_data(data.client, TCS3414_CONTROL,
    data.control);
    }
    static const struct iio_buffer_setup_ops tcs3414_buffer_setup_ops = {
    .postenable = tcs3414_buffer_postenable,
    .predisable = tcs3414_buffer_predisable,
    };
#[no_mangle]
unsafe extern "C" fn tcs3414_powerdown(data: *mut tcs3414_data) -> c_int {
    static int tcs3414_powerdown(struct tcs3414_data *data)
    {
    return i2c_smbus_write_byte_data(data.client, TCS3414_CONTROL,
    data.control & ~(TCS3414_CONTROL_POWER |
    TCS3414_CONTROL_ADC_EN));
    }
#[no_mangle]
unsafe extern "C" fn tcs3414_powerdown_cleanup(data: *mut c_void) {
    static void tcs3414_powerdown_cleanup(void *data)
    {
    tcs3414_powerdown(data);
    }
#[no_mangle]
unsafe extern "C" fn tcs3414_probe(client: *mut i2c_client) -> c_int {
    static int tcs3414_probe(struct i2c_client *client)
    {
    struct tcs3414_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    indio_dev.info = &tcs3414_info;
    indio_dev.name = TCS3414_DRV_NAME;
    indio_dev.channels = tcs3414_channels;
    indio_dev.num_channels = ARRAY_SIZE(tcs3414_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = i2c_smbus_read_byte_data(data.client, TCS3414_ID);
    if (ret < 0)
    return ret;
    switch (ret & 0xf0) {
    case 0x00:
    dev_info(&client.dev, "TCS3404 found\n");
    break;
    case 0x10:
    dev_info(&client.dev, "TCS3413/14/15/16 found\n");
    break;
    default:
    return -ENODEV;
    }
    data.control = TCS3414_CONTROL_POWER;
    ret = i2c_smbus_write_byte_data(data.client, TCS3414_CONTROL,
    data.control);
    if (ret < 0)
    return ret;
    ret = devm_add_action_or_reset(&client.dev, tcs3414_powerdown_cleanup,
    data);
    if (ret < 0)
    return ret;
    data.timing = TCS3414_INTEG_12MS; /* free running */
    ret = i2c_smbus_write_byte_data(data.client, TCS3414_TIMING,
    data.timing);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_byte_data(data.client, TCS3414_GAIN);
    if (ret < 0)
    return ret;
    data.gain = ret;
    ret = devm_iio_triggered_buffer_setup(&client.dev, indio_dev, core::ptr::null_mut(),
    tcs3414_trigger_handler, &tcs3414_buffer_setup_ops);
    if (ret < 0)
    return ret;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn tcs3414_suspend(dev: *mut device) -> c_int {
    static int tcs3414_suspend(struct device *dev)
    {
    struct tcs3414_data *data = iio_priv(i2c_get_clientdata(
    to_i2c_client(dev)));
    return tcs3414_powerdown(data);
    }
#[no_mangle]
unsafe extern "C" fn tcs3414_resume(dev: *mut device) -> c_int {
    static int tcs3414_resume(struct device *dev)
    {
    struct tcs3414_data *data = iio_priv(i2c_get_clientdata(
    to_i2c_client(dev)));
    return i2c_smbus_write_byte_data(data.client, TCS3414_CONTROL,
    data.control);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(tcs3414_pm_ops, tcs3414_suspend,
    tcs3414_resume);
    static const struct i2c_device_id tcs3414_id[] = {
    { .name = "tcs3414" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tcs3414_id);
    static struct i2c_driver tcs3414_driver = {
    .driver = {
    .name	= TCS3414_DRV_NAME,
    .pm	= pm_sleep_ptr(&tcs3414_pm_ops),
    },
    .probe		= tcs3414_probe,
    .id_table	= tcs3414_id,
    };
    module_i2c_driver(tcs3414_driver);
    MODULE_AUTHOR("Peter Meerwald <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("TCS3414 digital color sensors driver");
    MODULE_LICENSE("GPL");
