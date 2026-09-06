//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/stk8312.c
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
// Sensortek STK8312 3-Axis Accelerometer
//
// Copyright (c) 2015, Intel Corporation.
//
// IIO driver for STK8312; 7-bit I2C address: 0x3D.
//

pub const STK8312_REG_XOUT: c_uint = 0x00;
pub const STK8312_REG_YOUT: c_uint = 0x01;
pub const STK8312_REG_ZOUT: c_uint = 0x02;
pub const STK8312_REG_INTSU: c_uint = 0x06;
pub const STK8312_REG_MODE: c_uint = 0x07;
pub const STK8312_REG_SR: c_uint = 0x08;
pub const STK8312_REG_STH: c_uint = 0x13;
pub const STK8312_REG_RESET: c_uint = 0x20;
pub const STK8312_REG_AFECTRL: c_uint = 0x24;
pub const STK8312_REG_OTPADDR: c_uint = 0x3D;
pub const STK8312_REG_OTPDATA: c_uint = 0x3E;
pub const STK8312_REG_OTPCTRL: c_uint = 0x3F;

pub const STK8312_MODE_STANDBY: c_uint = 0x00;
pub const STK8312_MODE_INT_AH_PP: c_uint = 0xC0	/* active-high, push-pull */;

pub const STK8312_RNG_6G: c_int = 1;
pub const STK8312_RNG_SHIFT: c_int = 6;

pub const STK8312_SR_400HZ_IDX: c_int = 0;

pub const STK8312_ALL_CHANNEL_SIZE: c_int = 3;

//
// The accelerometer has two measurement ranges:
//
// -6g - +6g (8-bit, signed)
// -16g - +16g (8-bit, signed)
//
// scale1 = (6 + 6) * 9.81 / (2^8 - 1)     = 0.4616
// scale2 = (16 + 16) * 9.81 / (2^8 - 1)   = 1.2311
//

    static const int stk8312_scale_table[][2] = {
    {0, 461600}, {1, 231100}
    };
    static const struct {
    int val;
    int val2;
    } stk8312_samp_freq_table[] = {
    {400, 0}, {200, 0}, {100, 0}, {50, 0}, {25, 0},
    {12, 500000}, {6, 250000}, {3, 125000}
    };

    .type = IIO_ACCEL,						\
    .address = reg,							\
    .modified = 1,							\
    .channel2 = IIO_MOD_##axis,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),			\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_SAMP_FREQ),	\
    .scan_index = index,						\
    .scan_type = {							\
    .sign = 's',						\
    .realbits = 8,						\
    .storagebits = 8,					\
    .endianness = IIO_CPU,					\
    },								\
    }
    static const struct iio_chan_spec stk8312_channels[] = {
    STK8312_ACCEL_CHANNEL(0, STK8312_REG_XOUT, X),
    STK8312_ACCEL_CHANNEL(1, STK8312_REG_YOUT, Y),
    STK8312_ACCEL_CHANNEL(2, STK8312_REG_ZOUT, Z),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stk8312_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub range: u8,
    pub sample_rate_idx: u8,
    pub mode: u8,
    pub dready_trig: *mut iio_trigger,
    pub dready_trigger_on: bool,
// Ensure timestamp is naturally aligned
    struct {
    pub chans: [i8; 3],
    pub timestamp: aligned_s64,
    pub scan: },
}

    static IIO_CONST_ATTR(in_accel_scale_available, STK8312_SCALE_AVAIL);
    static IIO_CONST_ATTR_SAMP_FREQ_AVAIL("3.125 6.25 12.5 25 50 100 200 400");
    static struct attribute *stk8312_attributes[] = {
    &iio_const_attr_in_accel_scale_available.dev_attr.attr,
    &iio_const_attr_sampling_frequency_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group stk8312_attribute_group = {
    .attrs = stk8312_attributes
    };
#[no_mangle]
unsafe extern "C" fn stk8312_otp_init(data: *mut stk8312_data) -> c_int {
    static int stk8312_otp_init(struct stk8312_data *data)
    {
    int ret;
    let mut count: c_int = 10;
    struct i2c_client *client = data.client;
    ret = i2c_smbus_write_byte_data(client, STK8312_REG_OTPADDR, 0x70);
    if (ret < 0)
    goto exit_err;
    ret = i2c_smbus_write_byte_data(client, STK8312_REG_OTPCTRL, 0x02);
    if (ret < 0)
    goto exit_err;
    do {
    usleep_range(1000, 5000);
    ret = i2c_smbus_read_byte_data(client, STK8312_REG_OTPCTRL);
    if (ret < 0)
    goto exit_err;
    count--;
    } while (!(ret & BIT(7)) && count > 0);
    if (count == 0) {
    ret = -ETIMEDOUT;
    goto exit_err;
    }
    ret = i2c_smbus_read_byte_data(client, STK8312_REG_OTPDATA);
    if (ret == 0)
    ret = -EINVAL;
    if (ret < 0)
    goto exit_err;
    ret = i2c_smbus_write_byte_data(data.client, STK8312_REG_AFECTRL, ret);
    if (ret < 0)
    goto exit_err;
    msleep(150);
    return 0;
    exit_err:
    dev_err(&client.dev, "failed to initialize sensor\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stk8312_set_mode(data: *mut stk8312_data, mode: u8) -> c_int {
    static int stk8312_set_mode(struct stk8312_data *data, u8 mode)
    {
    int ret;
    struct i2c_client *client = data.client;
    if (mode == data.mode)
    return 0;
    ret = i2c_smbus_write_byte_data(client, STK8312_REG_MODE, mode);
    if (ret < 0) {
    dev_err(&client.dev, "failed to change sensor mode\n");
    return ret;
    }
    data.mode = mode;
    if (mode & STK8312_MODE_ACTIVE) {
// Need to run OTP sequence before entering active mode
    usleep_range(1000, 5000);
    ret = stk8312_otp_init(data);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stk8312_set_interrupts(data: *mut stk8312_data, int_mask: u8) -> c_int {
    static int stk8312_set_interrupts(struct stk8312_data *data, u8 int_mask)
    {
    int ret;
    u8 mode;
    struct i2c_client *client = data.client;
    mode = data.mode;
// We need to go in standby mode to modify registers
    ret = stk8312_set_mode(data, STK8312_MODE_STANDBY);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(client, STK8312_REG_INTSU, int_mask);
    if (ret < 0) {
    dev_err(&client.dev, "failed to set interrupts\n");
    stk8312_set_mode(data, mode);
    return ret;
    }
    return stk8312_set_mode(data, mode);
    }
    static int stk8312_data_rdy_trigger_set_state(struct iio_trigger *trig,
    bool state)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct stk8312_data *data = iio_priv(indio_dev);
    int ret;
    if (state)
    ret = stk8312_set_interrupts(data, STK8312_DREADY_BIT);
    else
    ret = stk8312_set_interrupts(data, 0x00);
    if (ret < 0) {
    dev_err(&data.client.dev, "failed to set trigger state\n");
    return ret;
    }
    data.dready_trigger_on = state;
    return 0;
    }
    static const struct iio_trigger_ops stk8312_trigger_ops = {
    .set_trigger_state = stk8312_data_rdy_trigger_set_state,
    };
#[no_mangle]
unsafe extern "C" fn stk8312_set_sample_rate(data: *mut stk8312_data, rate: u8) -> c_int {
    static int stk8312_set_sample_rate(struct stk8312_data *data, u8 rate)
    {
    int ret;
    u8 masked_reg;
    u8 mode;
    struct i2c_client *client = data.client;
    if (rate == data.sample_rate_idx)
    return 0;
    mode = data.mode;
// We need to go in standby mode to modify registers
    ret = stk8312_set_mode(data, STK8312_MODE_STANDBY);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_byte_data(client, STK8312_REG_SR);
    if (ret < 0)
    goto err_activate;
    masked_reg = (ret & (~STK8312_SR_MASK)) | rate;
    ret = i2c_smbus_write_byte_data(client, STK8312_REG_SR, masked_reg);
    if (ret < 0)
    goto err_activate;
    data.sample_rate_idx = rate;
    return stk8312_set_mode(data, mode);
    err_activate:
    dev_err(&client.dev, "failed to set sampling rate\n");
    stk8312_set_mode(data, mode);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stk8312_set_range(data: *mut stk8312_data, range: u8) -> c_int {
    static int stk8312_set_range(struct stk8312_data *data, u8 range)
    {
    int ret;
    u8 masked_reg;
    u8 mode;
    struct i2c_client *client = data.client;
    if (range != 1 && range != 2)
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(data->range: range ==) -> else {
    else if (range == data.range)
    return 0;
    mode = data.mode;
// We need to go in standby mode to modify registers
    ret = stk8312_set_mode(data, STK8312_MODE_STANDBY);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_byte_data(client, STK8312_REG_STH);
    if (ret < 0)
    goto err_activate;
    masked_reg = ret & (~STK8312_RNG_MASK);
    masked_reg |= range << STK8312_RNG_SHIFT;
    ret = i2c_smbus_write_byte_data(client, STK8312_REG_STH, masked_reg);
    if (ret < 0)
    goto err_activate;
    data.range = range;
    return stk8312_set_mode(data, mode);
    err_activate:
    dev_err(&client.dev, "failed to change sensor range\n");
    stk8312_set_mode(data, mode);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stk8312_read_accel(data: *mut stk8312_data, address: u8) -> c_int {
    static int stk8312_read_accel(struct stk8312_data *data, u8 address)
    {
    int ret;
    struct i2c_client *client = data.client;
    if (address > 2)
    return -EINVAL;
    ret = i2c_smbus_read_byte_data(client, address);
    if (ret < 0)
    dev_err(&client.dev, "register read failed\n");
    return ret;
    }
    static int stk8312_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct stk8312_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (iio_buffer_enabled(indio_dev))
    return -EBUSY;
    mutex_lock(&data.lock);
    ret = stk8312_set_mode(data, data.mode | STK8312_MODE_ACTIVE);
    if (ret < 0) {
    mutex_unlock(&data.lock);
    return ret;
    }
    ret = stk8312_read_accel(data, chan.address);
    if (ret < 0) {
    stk8312_set_mode(data,
    data.mode & (~STK8312_MODE_ACTIVE));
    mutex_unlock(&data.lock);
    return ret;
    }
// val = sign_extend32(ret, chan->scan_type.realbits - 1);
    ret = stk8312_set_mode(data,
    data.mode & (~STK8312_MODE_ACTIVE));
    mutex_unlock(&data.lock);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = stk8312_scale_table[data->range - 1][0];
// val2 = stk8312_scale_table[data->range - 1][1];
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_SAMP_FREQ:
// val = stk8312_samp_freq_table[data->sample_rate_idx].val;
// val2 = stk8312_samp_freq_table[data->sample_rate_idx].val2;
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static int stk8312_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    int i;
    let mut index: c_int = -1;
    int ret;
    struct stk8312_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    for (i = 0; i < ARRAY_SIZE(stk8312_scale_table); i++)
    if (val == stk8312_scale_table[i][0] &&
    val2 == stk8312_scale_table[i][1]) {
    index = i + 1;
    break;
    }
    if (index < 0)
    return -EINVAL;
    mutex_lock(&data.lock);
    ret = stk8312_set_range(data, index);
    mutex_unlock(&data.lock);
    return ret;
    case IIO_CHAN_INFO_SAMP_FREQ:
    for (i = 0; i < ARRAY_SIZE(stk8312_samp_freq_table); i++)
    if (val == stk8312_samp_freq_table[i].val &&
    val2 == stk8312_samp_freq_table[i].val2) {
    index = i;
    break;
    }
    if (index < 0)
    return -EINVAL;
    mutex_lock(&data.lock);
    ret = stk8312_set_sample_rate(data, index);
    mutex_unlock(&data.lock);
    return ret;
    }
    return -EINVAL;
    }
    static const struct iio_info stk8312_info = {
    .read_raw		= stk8312_read_raw,
    .write_raw		= stk8312_write_raw,
    .attrs			= &stk8312_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn stk8312_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t stk8312_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct stk8312_data *data = iio_priv(indio_dev);
    int bit, ret, i = 0;
    mutex_lock(&data.lock);
//
// Do a bulk read if all channels are requested,
// from 0x00 (XOUT) to 0x02 (ZOUT)
//
    if (*(indio_dev.active_scan_mask) == STK8312_ALL_CHANNEL_MASK) {
    ret = i2c_smbus_read_i2c_block_data(data.client,
    STK8312_REG_XOUT,
    STK8312_ALL_CHANNEL_SIZE,
    data.scan.chans);
    if (ret < STK8312_ALL_CHANNEL_SIZE) {
    dev_err(&data.client.dev, "register read failed\n");
    mutex_unlock(&data.lock);
    goto err;
    }
    } else {
    iio_for_each_active_channel(indio_dev, bit) {
    ret = stk8312_read_accel(data, bit);
    if (ret < 0) {
    mutex_unlock(&data.lock);
    goto err;
    }
    data.scan.chans[i++] = ret;
    }
    }
    mutex_unlock(&data.lock);
    iio_push_to_buffers_with_ts(indio_dev, &data.scan, sizeof(data.scan),
    pf.timestamp);
    err:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stk8312_data_rdy_trig_poll(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t stk8312_data_rdy_trig_poll(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct stk8312_data *data = iio_priv(indio_dev);
    if (data.dready_trigger_on)
    iio_trigger_poll(data.dready_trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stk8312_buffer_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int stk8312_buffer_preenable(struct iio_dev *indio_dev)
    {
    struct stk8312_data *data = iio_priv(indio_dev);
    return stk8312_set_mode(data, data.mode | STK8312_MODE_ACTIVE);
    }
#[no_mangle]
unsafe extern "C" fn stk8312_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int stk8312_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct stk8312_data *data = iio_priv(indio_dev);
    return stk8312_set_mode(data, data.mode & (~STK8312_MODE_ACTIVE));
    }
    static const struct iio_buffer_setup_ops stk8312_buffer_setup_ops = {
    .preenable   = stk8312_buffer_preenable,
    .postdisable = stk8312_buffer_postdisable,
    };
#[no_mangle]
unsafe extern "C" fn stk8312_probe(client: *mut i2c_client) -> c_int {
    static int stk8312_probe(struct i2c_client *client)
    {
    int ret;
    struct iio_dev *indio_dev;
    struct stk8312_data *data;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    i2c_set_clientdata(client, indio_dev);
    mutex_init(&data.lock);
    indio_dev.info = &stk8312_info;
    indio_dev.name = STK8312_DRIVER_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = stk8312_channels;
    indio_dev.num_channels = ARRAY_SIZE(stk8312_channels);
// A software reset is recommended at power-on
    ret = i2c_smbus_write_byte_data(data.client, STK8312_REG_RESET, 0x00);
    if (ret < 0) {
    dev_err(&client.dev, "failed to reset sensor\n");
    return ret;
    }
    data.sample_rate_idx = STK8312_SR_400HZ_IDX;
    ret = stk8312_set_range(data, STK8312_RNG_6G);
    if (ret < 0)
    return ret;
    ret = stk8312_set_mode(data,
    STK8312_MODE_INT_AH_PP | STK8312_MODE_ACTIVE);
    if (ret < 0)
    return ret;
    if (client.irq > 0) {
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    stk8312_data_rdy_trig_poll,
    core::ptr::null_mut(),
    IRQF_TRIGGER_RISING |
    IRQF_ONESHOT,
    "stk8312_event",
    indio_dev);
    if (ret)
    goto err_power_off;
    data.dready_trig = devm_iio_trigger_alloc(&client.dev,
    "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!data.dready_trig) {
    ret = -ENOMEM;
    goto err_power_off;
    }
    data.dready_trig.ops = &stk8312_trigger_ops;
    iio_trigger_set_drvdata(data.dready_trig, indio_dev);
    ret = iio_trigger_register(data.dready_trig);
    if (ret) {
    dev_err(&client.dev, "iio trigger register failed\n");
    goto err_power_off;
    }
    }
    ret = iio_triggered_buffer_setup(indio_dev,
    iio_pollfunc_store_time,
    stk8312_trigger_handler,
    &stk8312_buffer_setup_ops);
    if (ret < 0) {
    dev_err(&client.dev, "iio triggered buffer setup failed\n");
    goto err_trigger_unregister;
    }
    ret = iio_device_register(indio_dev);
    if (ret < 0) {
    dev_err(&client.dev, "device_register failed\n");
    goto err_buffer_cleanup;
    }
    return 0;
    err_buffer_cleanup:
    iio_triggered_buffer_cleanup(indio_dev);
    err_trigger_unregister:
    if (data.dready_trig)
    iio_trigger_unregister(data.dready_trig);
    err_power_off:
    stk8312_set_mode(data, STK8312_MODE_STANDBY);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stk8312_remove(client: *mut i2c_client) {
    static void stk8312_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct stk8312_data *data = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    iio_triggered_buffer_cleanup(indio_dev);
    if (data.dready_trig)
    iio_trigger_unregister(data.dready_trig);
    stk8312_set_mode(data, STK8312_MODE_STANDBY);
    }
#[no_mangle]
unsafe extern "C" fn stk8312_suspend(dev: *mut device) -> c_int {
    static int stk8312_suspend(struct device *dev)
    {
    struct stk8312_data *data;
    data = iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    return stk8312_set_mode(data, data.mode & (~STK8312_MODE_ACTIVE));
    }
#[no_mangle]
unsafe extern "C" fn stk8312_resume(dev: *mut device) -> c_int {
    static int stk8312_resume(struct device *dev)
    {
    struct stk8312_data *data;
    data = iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    return stk8312_set_mode(data, data.mode | STK8312_MODE_ACTIVE);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(stk8312_pm_ops, stk8312_suspend,
    stk8312_resume);
    static const struct i2c_device_id stk8312_i2c_id[] = {
// Deprecated in favour of lowercase form
    { .name = "STK8312" },
    { .name = "stk8312" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, stk8312_i2c_id);
    static struct i2c_driver stk8312_driver = {
    .driver = {
    .name = STK8312_DRIVER_NAME,
    .pm = pm_sleep_ptr(&stk8312_pm_ops),
    },
    .probe =        stk8312_probe,
    .remove =           stk8312_remove,
    .id_table =         stk8312_i2c_id,
    };
    module_i2c_driver(stk8312_driver);
    MODULE_AUTHOR("Tiberiu Breana <tiberiu.a.breana@intel.com>");
    MODULE_DESCRIPTION("STK8312 3-Axis Accelerometer driver");
    MODULE_LICENSE("GPL v2");
