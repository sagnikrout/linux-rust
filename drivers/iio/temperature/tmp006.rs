//! Automatically rewritten from C to Rust
//! Source: drivers/iio/temperature/tmp006.c
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
// tmp006.c - Support for TI TMP006 IR thermopile sensor
//
// Copyright (c) 2013 Peter Meerwald <pmeerw@pmeerw.net>
//
// Driver for the Texas Instruments I2C 16-bit IR thermopile sensor
//
// (7-bit I2C slave address 0x40, changeable via ADR pins)
//

pub const TMP006_VOBJECT: c_uint = 0x00;
pub const TMP006_TAMBIENT: c_uint = 0x01;
pub const TMP006_CONFIG: c_uint = 0x02;
pub const TMP006_MANUFACTURER_ID: c_uint = 0xfe;
pub const TMP006_DEVICE_ID: c_uint = 0xff;
pub const TMP006_TAMBIENT_SHIFT: c_int = 2;

pub const TMP006_CONFIG_CR_SHIFT: c_int = 9;
pub const TMP006_MANUFACTURER_MAGIC: c_uint = 0x5449;
pub const TMP006_DEVICE_MAGIC: c_uint = 0x0067;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmp006_data {
    pub client: *mut i2c_client,
    pub config: u16,
    pub drdy_trig: *mut iio_trigger,
}

#[no_mangle]
unsafe extern "C" fn tmp006_read_measurement(data: *mut tmp006_data, reg: u8) -> c_int {
    static int tmp006_read_measurement(struct tmp006_data *data, u8 reg)
    {
    s32 ret;
    let mut tries: c_int = 50;
    while (tries-- > 0) {
    ret = i2c_smbus_read_word_swapped(data.client,
    TMP006_CONFIG);
    if (ret < 0)
    return ret;
    if (ret & TMP006_CONFIG_DRDY)
    break;
    msleep(100);
    }
    if (tries < 0)
    return -EIO;
    return i2c_smbus_read_word_swapped(data.client, reg);
    }
    static const int tmp006_freqs[5][2] = { {4, 0}, {2, 0}, {1, 0},
    {0, 500000}, {0, 250000} };
    static int tmp006_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *channel, int *val,
    int *val2, long mask)
    {
    struct tmp006_data *data = iio_priv(indio_dev);
    s32 ret;
    int cr;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (channel.type == IIO_VOLTAGE) {
// LSB is 156.25 nV
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = tmp006_read_measurement(data, TMP006_VOBJECT);
    iio_device_release_direct(indio_dev);
    if (ret < 0)
    return ret;
// val = sign_extend32(ret, 15);
    } else if (channel.type == IIO_TEMP) {
// LSB is 0.03125 degrees Celsius
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = tmp006_read_measurement(data, TMP006_TAMBIENT);
    iio_device_release_direct(indio_dev);
    if (ret < 0)
    return ret;
// val = sign_extend32(ret, 15) >> TMP006_TAMBIENT_SHIFT;
    } else {
    break;
    }
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    if (channel.type == IIO_VOLTAGE) {
// val = 0;
// val2 = 156250;
    } else if (channel.type == IIO_TEMP) {
// val = 31;
// val2 = 250000;
    } else {
    break;
    }
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_SAMP_FREQ:
    cr = (data.config & TMP006_CONFIG_CR_MASK)
    >> TMP006_CONFIG_CR_SHIFT;
// val = tmp006_freqs[cr][0];
// val2 = tmp006_freqs[cr][1];
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    break;
    }
    return -EINVAL;
    }
    static int tmp006_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val,
    int val2,
    long mask)
    {
    struct tmp006_data *data = iio_priv(indio_dev);
    int ret, i;
    if (mask != IIO_CHAN_INFO_SAMP_FREQ)
    return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(tmp006_freqs); i++)
    if ((val == tmp006_freqs[i][0]) &&
    (val2 == tmp006_freqs[i][1])) {
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    data.config &= ~TMP006_CONFIG_CR_MASK;
    data.config |= i << TMP006_CONFIG_CR_SHIFT;
    ret = i2c_smbus_write_word_swapped(data.client,
    TMP006_CONFIG,
    data.config);
    iio_device_release_direct(indio_dev);
    return ret;
    }
    return -EINVAL;
    }
    static IIO_CONST_ATTR(sampling_frequency_available, "4 2 1 0.5 0.25");
    static struct attribute *tmp006_attributes[] = {
    &iio_const_attr_sampling_frequency_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group tmp006_attribute_group = {
    .attrs = tmp006_attributes,
    };
    static const struct iio_chan_spec tmp006_channels[] = {
    {
    .type = IIO_VOLTAGE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_BE,
    }
    },
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),
    .scan_index = 1,
    .scan_type = {
    .sign = 's',
    .realbits = 14,
    .storagebits = 16,
    .shift = TMP006_TAMBIENT_SHIFT,
    .endianness = IIO_BE,
    }
    },
    IIO_CHAN_SOFT_TIMESTAMP(2),
    };
    static const struct iio_info tmp006_info = {
    .read_raw = tmp006_read_raw,
    .write_raw = tmp006_write_raw,
    .attrs = &tmp006_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn tmp006_check_identification(client: *mut i2c_client) -> bool {
    static bool tmp006_check_identification(struct i2c_client *client)
    {
    int mid, did;
    mid = i2c_smbus_read_word_swapped(client, TMP006_MANUFACTURER_ID);
    if (mid < 0)
    return false;
    did = i2c_smbus_read_word_swapped(client, TMP006_DEVICE_ID);
    if (did < 0)
    return false;
    let mut mid: return = = TMP006_MANUFACTURER_MAGIC && did == TMP006_DEVICE_MAGIC;
    }
#[no_mangle]
unsafe extern "C" fn tmp006_power(dev: *mut device, up: bool) -> c_int {
    static int tmp006_power(struct device *dev, bool up)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct tmp006_data *data = iio_priv(indio_dev);
    if (up)
    data.config |= TMP006_CONFIG_MOD_MASK;
    else
    data.config &= ~TMP006_CONFIG_MOD_MASK;
    return i2c_smbus_write_word_swapped(data.client, TMP006_CONFIG,
    data.config);
    }
#[no_mangle]
unsafe extern "C" fn tmp006_powerdown_cleanup(dev: *mut c_void) {
    static void tmp006_powerdown_cleanup(void *dev)
    {
    tmp006_power(dev, false);
    }
#[no_mangle]
unsafe extern "C" fn tmp006_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t tmp006_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct tmp006_data *data = iio_priv(indio_dev);
    struct {
    s16 channels[2];
    aligned_s64 ts;
    } scan = { };
    s32 ret;
    ret = i2c_smbus_read_word_data(data.client, TMP006_VOBJECT);
    if (ret < 0)
    goto err;
    scan.channels[0] = ret;
    ret = i2c_smbus_read_word_data(data.client, TMP006_TAMBIENT);
    if (ret < 0)
    goto err;
    scan.channels[1] = ret;
    iio_push_to_buffers_with_ts(indio_dev, &scan, sizeof(scan),
    iio_get_time_ns(indio_dev));
    err:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tmp006_set_trigger_state(trig: *mut iio_trigger, state: bool) -> c_int {
    static int tmp006_set_trigger_state(struct iio_trigger *trig, bool state)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct tmp006_data *data = iio_priv(indio_dev);
    if (state)
    data.config |= TMP006_CONFIG_DRDY_EN;
    else
    data.config &= ~TMP006_CONFIG_DRDY_EN;
    return i2c_smbus_write_word_swapped(data.client, TMP006_CONFIG,
    data.config);
    }
    static const struct iio_trigger_ops tmp006_trigger_ops = {
    .set_trigger_state = tmp006_set_trigger_state,
    };
    static const unsigned long tmp006_scan_masks[] = { 0x3, 0 };
#[no_mangle]
unsafe extern "C" fn tmp006_probe(client: *mut i2c_client) -> c_int {
    static int tmp006_probe(struct i2c_client *client)
    {
    struct iio_dev *indio_dev;
    struct tmp006_data *data;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_WORD_DATA))
    return -EOPNOTSUPP;
    if (!tmp006_check_identification(client)) {
    dev_err(&client.dev, "no TMP006 sensor\n");
    return -ENODEV;
    }
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    indio_dev.name = dev_name(&client.dev);
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &tmp006_info;
    indio_dev.channels = tmp006_channels;
    indio_dev.num_channels = ARRAY_SIZE(tmp006_channels);
    indio_dev.available_scan_masks = tmp006_scan_masks;
    ret = i2c_smbus_read_word_swapped(data.client, TMP006_CONFIG);
    if (ret < 0)
    return ret;
    data.config = ret;
    if ((ret & TMP006_CONFIG_MOD_MASK) != TMP006_CONFIG_MOD_MASK) {
    ret = tmp006_power(&client.dev, true);
    if (ret < 0)
    return ret;
    }
    ret = devm_add_action_or_reset(&client.dev, tmp006_powerdown_cleanup,
    &client.dev);
    if (ret < 0)
    return ret;
    if (client.irq > 0) {
    data.drdy_trig = devm_iio_trigger_alloc(&client.dev,
    "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!data.drdy_trig)
    return -ENOMEM;
    data.drdy_trig.ops = &tmp006_trigger_ops;
    iio_trigger_set_drvdata(data.drdy_trig, indio_dev);
    ret = devm_iio_trigger_register(&client.dev, data.drdy_trig);
    if (ret)
    return ret;
    indio_dev.trig = iio_trigger_get(data.drdy_trig);
    ret = devm_request_irq(&client.dev, client.irq,
    iio_trigger_generic_data_rdy_poll,
    IRQF_NO_THREAD, "tmp006_irq",
    data.drdy_trig);
    if (ret < 0)
    return ret;
    }
    ret = devm_iio_triggered_buffer_setup(&client.dev, indio_dev, core::ptr::null_mut(),
    tmp006_trigger_handler, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn tmp006_suspend(dev: *mut device) -> c_int {
    static int tmp006_suspend(struct device *dev)
    {
    return tmp006_power(dev, false);
    }
#[no_mangle]
unsafe extern "C" fn tmp006_resume(dev: *mut device) -> c_int {
    static int tmp006_resume(struct device *dev)
    {
    return tmp006_power(dev, true);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(tmp006_pm_ops, tmp006_suspend, tmp006_resume);
    static const struct of_device_id tmp006_of_match[] = {
    { .compatible = "ti,tmp006" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tmp006_of_match);
    static const struct i2c_device_id tmp006_id[] = {
    { .name = "tmp006" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tmp006_id);
    static struct i2c_driver tmp006_driver = {
    .driver = {
    .name	= "tmp006",
    .of_match_table = tmp006_of_match,
    .pm	= pm_sleep_ptr(&tmp006_pm_ops),
    },
    .probe = tmp006_probe,
    .id_table = tmp006_id,
    };
    module_i2c_driver(tmp006_driver);
    MODULE_AUTHOR("Peter Meerwald <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("TI TMP006 IR thermopile sensor driver");
    MODULE_LICENSE("GPL");
