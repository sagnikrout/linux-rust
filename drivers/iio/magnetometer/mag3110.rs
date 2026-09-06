//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/mag3110.c
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
// mag3110.c - Support for Freescale MAG3110 magnetometer sensor
//
// Copyright (c) 2013 Peter Meerwald <pmeerw@pmeerw.net>
//
// (7-bit I2C slave address 0x0e)
//
// TODO: irq, user offset, oversampling, continuous mode
//

pub const MAG3110_STATUS: c_uint = 0x00;
pub const MAG3110_OUT_X: c_uint = 0x01 /* MSB first */;
pub const MAG3110_OUT_Y: c_uint = 0x03;
pub const MAG3110_OUT_Z: c_uint = 0x05;
pub const MAG3110_WHO_AM_I: c_uint = 0x07;
pub const MAG3110_SYSMOD: c_uint = 0x08;
pub const MAG3110_OFF_X: c_uint = 0x09 /* MSB first */;
pub const MAG3110_OFF_Y: c_uint = 0x0b;
pub const MAG3110_OFF_Z: c_uint = 0x0d;
pub const MAG3110_DIE_TEMP: c_uint = 0x0f;
pub const MAG3110_CTRL_REG1: c_uint = 0x10;
pub const MAG3110_CTRL_REG2: c_uint = 0x11;

pub const MAG3110_CTRL_DR_SHIFT: c_int = 5;
pub const MAG3110_CTRL_DR_DEFAULT: c_int = 0;

pub const MAG3110_DEVICE_ID: c_uint = 0xc4;
// Each client has this additional data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mag3110_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub ctrl_reg1: u8,
    pub sleep_val: c_int,
    pub vdd_reg: *mut regulator,
    pub vddio_reg: *mut regulator,
// Ensure natural alignment of timestamp
    struct {
    pub channels: [__be16; 3],
    pub temperature: u8,
    pub ts: aligned_s64,
    pub scan: },
}

#[no_mangle]
unsafe extern "C" fn mag3110_request(data: *mut mag3110_data) -> c_int {
    static int mag3110_request(struct mag3110_data *data)
    {
    int ret, tries = 150;
    if ((data.ctrl_reg1 & MAG3110_CTRL_AC) == 0) {
// trigger measurement
    ret = i2c_smbus_write_byte_data(data.client, MAG3110_CTRL_REG1,
    data.ctrl_reg1 | MAG3110_CTRL_TM);
    if (ret < 0)
    return ret;
    }
    while (tries-- > 0) {
    ret = i2c_smbus_read_byte_data(data.client, MAG3110_STATUS);
    if (ret < 0)
    return ret;
// wait for data ready
    if ((ret & MAG3110_STATUS_DRDY) == MAG3110_STATUS_DRDY)
    break;
    if (data.sleep_val <= 20)
    usleep_range(data.sleep_val * 250, data.sleep_val * 500);
    else
    msleep(20);
    }
    if (tries < 0) {
    dev_err(&data.client.dev, "data not ready\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mag3110_read(data: *mut mag3110_data, buf[3]: __be16) -> c_int {
    static int mag3110_read(struct mag3110_data *data, __be16 buf[3])
    {
    int ret;
    guard(mutex)(&data.lock);
    ret = mag3110_request(data);
    if (ret < 0)
    return ret;
    return i2c_smbus_read_i2c_block_data(data.client, MAG3110_OUT_X,
    3 * sizeof(__be16), (u8 *) buf);
    }
    static ssize_t mag3110_show_int_plus_micros(char *buf,
#[no_mangle]
pub unsafe extern "C" fn int(_arg: *mut vals)[2], n: c_int) -> const {
    const int (*vals)[2], int n)
    {
    let mut len: usize = 0;
    while (n-- > 0)
    len += scnprintf(buf + len, PAGE_SIZE - len,
    "%d.%06d ", vals[n][0], vals[n][1]);
// replace trailing space by newline
    buf[len - 1] = '\n';
    return len;
    }
    static int mag3110_get_int_plus_micros_index(const int (*vals)[2], int n,
    int val, int val2)
    {
    while (n-- > 0)
    if (val == vals[n][0] && val2 == vals[n][1])
    return n;
    return -EINVAL;
    }
    static const int mag3110_samp_freq[8][2] = {
    {80, 0}, {40, 0}, {20, 0}, {10, 0}, {5, 0}, {2, 500000},
    {1, 250000}, {0, 625000}
    };
    static ssize_t mag3110_show_samp_freq_avail(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return mag3110_show_int_plus_micros(buf, mag3110_samp_freq, 8);
    }
    static IIO_DEV_ATTR_SAMP_FREQ_AVAIL(mag3110_show_samp_freq_avail);
    static int mag3110_get_samp_freq_index(struct mag3110_data *data,
    int val, int val2)
    {
    return mag3110_get_int_plus_micros_index(mag3110_samp_freq, 8, val,
    val2);
    }
#[no_mangle]
unsafe extern "C" fn mag3110_calculate_sleep(data: *mut mag3110_data) -> c_int {
    static int mag3110_calculate_sleep(struct mag3110_data *data)
    {
    int ret, i = data.ctrl_reg1 >> MAG3110_CTRL_DR_SHIFT;
    if (mag3110_samp_freq[i][0] > 0)
    ret = 1000 / mag3110_samp_freq[i][0];
    else
    ret = 1000;
    let mut ret: return = = 0 ? 1 : ret;
    }
#[no_mangle]
unsafe extern "C" fn mag3110_standby(data: *mut mag3110_data) -> c_int {
    static int mag3110_standby(struct mag3110_data *data)
    {
    return i2c_smbus_write_byte_data(data.client, MAG3110_CTRL_REG1,
    data.ctrl_reg1 & ~MAG3110_CTRL_AC);
    }
#[no_mangle]
unsafe extern "C" fn mag3110_wait_standby(data: *mut mag3110_data) -> c_int {
    static int mag3110_wait_standby(struct mag3110_data *data)
    {
    int ret, tries = 30;
//
// Takes up to 1/ODR to come out of active mode into stby
// Longest expected period is 12.5seconds.
// We'll sleep for 500ms between checks
//
    while (tries-- > 0) {
    ret = i2c_smbus_read_byte_data(data.client, MAG3110_SYSMOD);
    if (ret < 0) {
    dev_err(&data.client.dev, "i2c error\n");
    return ret;
    }
// wait for standby
    if ((ret & MAG3110_SYSMOD_MODE_MASK) == 0)
    break;
    msleep_interruptible(500);
    }
    if (tries < 0) {
    dev_err(&data.client.dev, "device not entering standby mode\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mag3110_active(data: *mut mag3110_data) -> c_int {
    static int mag3110_active(struct mag3110_data *data)
    {
    return i2c_smbus_write_byte_data(data.client, MAG3110_CTRL_REG1,
    data.ctrl_reg1);
    }
// returns >0 if active, 0 if in standby and <0 on error
#[no_mangle]
unsafe extern "C" fn mag3110_is_active(data: *mut mag3110_data) -> c_int {
    static int mag3110_is_active(struct mag3110_data *data)
    {
    int reg;
    reg = i2c_smbus_read_byte_data(data.client, MAG3110_CTRL_REG1);
    if (reg < 0)
    return reg;
    return reg & MAG3110_CTRL_AC;
    }
#[no_mangle]
unsafe extern "C" fn mag3110_change_config(data: *mut mag3110_data, reg: u8, val: u8) -> c_int {
    static int mag3110_change_config(struct mag3110_data *data, u8 reg, u8 val)
    {
    int ret;
    int is_active;
    guard(mutex)(&data.lock);
    is_active = mag3110_is_active(data);
    if (is_active < 0)
    return is_active;
// config can only be changed when in standby
    if (is_active > 0) {
    ret = mag3110_standby(data);
    if (ret < 0)
    return ret;
    }
//
// After coming out of active we must wait for the part
// to transition to STBY. This can take up to 1 /ODR to occur
//
    ret = mag3110_wait_standby(data);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(data.client, reg, val);
    if (ret < 0)
    return ret;
    if (is_active > 0) {
    ret = mag3110_active(data);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static int __mag3110_read_info_raw(struct mag3110_data *data,
    struct iio_chan_spec const *chan,
    int *val)
    {
    __be16 buffer[3];
    int ret;
    switch (chan.type) {
    case IIO_MAGN: /* in 0.1 uT / LSB */
    ret = mag3110_read(data, buffer);
    if (ret < 0)
    return ret;
// val = sign_extend32(be16_to_cpu(buffer[chan->scan_index]),
    chan.scan_type.realbits - 1);
    return IIO_VAL_INT;
    case IIO_TEMP: { /* in 1 C / LSB */
    guard(mutex)(&data.lock);
    ret = mag3110_request(data);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_byte_data(data.client,
    MAG3110_DIE_TEMP);
    if (ret < 0)
    return ret;
// val = sign_extend32(ret, chan->scan_type.realbits - 1);
    return IIO_VAL_INT;
    }
    default:
    return -EINVAL;
    }
    }
    static int mag3110_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct mag3110_data *data = iio_priv(indio_dev);
    int i, ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = __mag3110_read_info_raw(data, chan, val);
    iio_device_release_direct(indio_dev);
    return ret;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_MAGN:
// val = 0;
// val2 = 1000;
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_TEMP:
// val = 1000;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SAMP_FREQ:
    i = data.ctrl_reg1 >> MAG3110_CTRL_DR_SHIFT;
// val = mag3110_samp_freq[i][0];
// val2 = mag3110_samp_freq[i][1];
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_CALIBBIAS:
    ret = i2c_smbus_read_word_swapped(data.client,
    MAG3110_OFF_X +	2 * chan.scan_index);
    if (ret < 0)
    return ret;
// val = sign_extend32(ret >> 1, 14);
    return IIO_VAL_INT;
    }
    return -EINVAL;
    }
    static int __mag3110_write_raw(struct mag3110_data *data,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    int rate;
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    rate = mag3110_get_samp_freq_index(data, val, val2);
    if (rate < 0)
    return -EINVAL;
    data.ctrl_reg1 &= 0xff & ~MAG3110_CTRL_DR_MASK
    & ~MAG3110_CTRL_AC;
    data.ctrl_reg1 |= rate << MAG3110_CTRL_DR_SHIFT;
    data.sleep_val = mag3110_calculate_sleep(data);
    if (data.sleep_val < 40)
    data.ctrl_reg1 |= MAG3110_CTRL_AC;
    return mag3110_change_config(data, MAG3110_CTRL_REG1,
    data.ctrl_reg1);
    case IIO_CHAN_INFO_CALIBBIAS:
    if (val < -10000 || val > 10000)
    return -EINVAL;
    return i2c_smbus_write_word_swapped(data.client,
    MAG3110_OFF_X + 2 * chan.scan_index, val << 1);
    default:
    return -EINVAL;
    }
    }
    static int mag3110_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct mag3110_data *data = iio_priv(indio_dev);
    int ret;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = __mag3110_write_raw(data, chan, val, val2, mask);
    iio_device_release_direct(indio_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mag3110_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t mag3110_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct mag3110_data *data = iio_priv(indio_dev);
    int ret;
    ret = mag3110_read(data, data.scan.channels);
    if (ret < 0)
    goto done;
    if (test_bit(3, indio_dev.active_scan_mask)) {
    ret = i2c_smbus_read_byte_data(data.client,
    MAG3110_DIE_TEMP);
    if (ret < 0)
    goto done;
    data.scan.temperature = ret;
    }
    iio_push_to_buffers_with_ts(indio_dev, &data.scan, sizeof(data.scan),
    iio_get_time_ns(indio_dev));
    done:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }

    .type = IIO_MAGN, \
    .modified = 1, \
    .channel2 = IIO_MOD_##axis, \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) | \
    BIT(IIO_CHAN_INFO_CALIBBIAS), \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SAMP_FREQ) | \
    BIT(IIO_CHAN_INFO_SCALE), \
    .scan_index = idx, \
    .scan_type = { \
    .sign = 's', \
    .realbits = 16, \
    .storagebits = 16, \
    .endianness = IIO_BE, \
    }, \
    }
    static const struct iio_chan_spec mag3110_channels[] = {
    MAG3110_CHANNEL(X, 0),
    MAG3110_CHANNEL(Y, 1),
    MAG3110_CHANNEL(Z, 2),
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 3,
    .scan_type = {
    .sign = 's',
    .realbits = 8,
    .storagebits = 8,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(4),
    };
    static struct attribute *mag3110_attributes[] = {
    &iio_dev_attr_sampling_frequency_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group mag3110_group = {
    .attrs = mag3110_attributes,
    };
    static const struct iio_info mag3110_info = {
    .attrs = &mag3110_group,
    .read_raw = &mag3110_read_raw,
    .write_raw = &mag3110_write_raw,
    };
    static const unsigned long mag3110_scan_masks[] = {0x7, 0xf, 0};
#[no_mangle]
unsafe extern "C" fn mag3110_probe(client: *mut i2c_client) -> c_int {
    static int mag3110_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct mag3110_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.vdd_reg = devm_regulator_get(&client.dev, "vdd");
    if (IS_ERR(data.vdd_reg))
    return dev_err_probe(&client.dev, PTR_ERR(data.vdd_reg),
    "failed to get VDD regulator!\n");
    data.vddio_reg = devm_regulator_get(&client.dev, "vddio");
    if (IS_ERR(data.vddio_reg))
    return dev_err_probe(&client.dev, PTR_ERR(data.vddio_reg),
    "failed to get VDDIO regulator!\n");
    ret = regulator_enable(data.vdd_reg);
    if (ret) {
    dev_err(&client.dev, "failed to enable VDD regulator!\n");
    return ret;
    }
    ret = regulator_enable(data.vddio_reg);
    if (ret) {
    dev_err(&client.dev, "failed to enable VDDIO regulator!\n");
    goto disable_regulator_vdd;
    }
    ret = i2c_smbus_read_byte_data(client, MAG3110_WHO_AM_I);
    if (ret < 0)
    goto disable_regulators;
    if (ret != MAG3110_DEVICE_ID) {
    ret = -ENODEV;
    goto disable_regulators;
    }
    data.client = client;
    mutex_init(&data.lock);
    i2c_set_clientdata(client, indio_dev);
    indio_dev.info = &mag3110_info;
    indio_dev.name = id.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = mag3110_channels;
    indio_dev.num_channels = ARRAY_SIZE(mag3110_channels);
    indio_dev.available_scan_masks = mag3110_scan_masks;
    data.ctrl_reg1 = MAG3110_CTRL_DR_DEFAULT << MAG3110_CTRL_DR_SHIFT;
    data.sleep_val = mag3110_calculate_sleep(data);
    if (data.sleep_val < 40)
    data.ctrl_reg1 |= MAG3110_CTRL_AC;
    ret = mag3110_change_config(data, MAG3110_CTRL_REG1, data.ctrl_reg1);
    if (ret < 0)
    goto disable_regulators;
    ret = i2c_smbus_write_byte_data(client, MAG3110_CTRL_REG2,
    MAG3110_CTRL_AUTO_MRST_EN);
    if (ret < 0)
    goto standby_on_error;
    ret = iio_triggered_buffer_setup(indio_dev, core::ptr::null_mut(),
    mag3110_trigger_handler, core::ptr::null_mut());
    if (ret < 0)
    goto standby_on_error;
    ret = iio_device_register(indio_dev);
    if (ret < 0)
    goto buffer_cleanup;
    return 0;
    buffer_cleanup:
    iio_triggered_buffer_cleanup(indio_dev);
    standby_on_error:
    mag3110_standby(iio_priv(indio_dev));
    disable_regulators:
    regulator_disable(data.vddio_reg);
    disable_regulator_vdd:
    regulator_disable(data.vdd_reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mag3110_remove(client: *mut i2c_client) {
    static void mag3110_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct mag3110_data *data = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    iio_triggered_buffer_cleanup(indio_dev);
    mag3110_standby(iio_priv(indio_dev));
    regulator_disable(data.vddio_reg);
    regulator_disable(data.vdd_reg);
    }
#[no_mangle]
unsafe extern "C" fn mag3110_suspend(dev: *mut device) -> c_int {
    static int mag3110_suspend(struct device *dev)
    {
    struct mag3110_data *data = iio_priv(i2c_get_clientdata(
    to_i2c_client(dev)));
    int ret;
    ret = mag3110_standby(iio_priv(i2c_get_clientdata(
    to_i2c_client(dev))));
    if (ret)
    return ret;
    ret = regulator_disable(data.vddio_reg);
    if (ret) {
    dev_err(dev, "failed to disable VDDIO regulator\n");
    return ret;
    }
    ret = regulator_disable(data.vdd_reg);
    if (ret) {
    dev_err(dev, "failed to disable VDD regulator\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mag3110_resume(dev: *mut device) -> c_int {
    static int mag3110_resume(struct device *dev)
    {
    struct mag3110_data *data = iio_priv(i2c_get_clientdata(
    to_i2c_client(dev)));
    int ret;
    ret = regulator_enable(data.vdd_reg);
    if (ret) {
    dev_err(dev, "failed to enable VDD regulator\n");
    return ret;
    }
    ret = regulator_enable(data.vddio_reg);
    if (ret) {
    dev_err(dev, "failed to enable VDDIO regulator\n");
    regulator_disable(data.vdd_reg);
    return ret;
    }
    return i2c_smbus_write_byte_data(data.client, MAG3110_CTRL_REG1,
    data.ctrl_reg1);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(mag3110_pm_ops, mag3110_suspend,
    mag3110_resume);
    static const struct i2c_device_id mag3110_id[] = {
    { .name = "mag3110" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mag3110_id);
    static const struct of_device_id mag3110_of_match[] = {
    { .compatible = "fsl,mag3110" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mag3110_of_match);
    static struct i2c_driver mag3110_driver = {
    .driver = {
    .name	= "mag3110",
    .of_match_table = mag3110_of_match,
    .pm	= pm_sleep_ptr(&mag3110_pm_ops),
    },
    .probe = mag3110_probe,
    .remove = mag3110_remove,
    .id_table = mag3110_id,
    };
    module_i2c_driver(mag3110_driver);
    MODULE_AUTHOR("Peter Meerwald <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("Freescale MAG3110 magnetometer driver");
    MODULE_LICENSE("GPL");
