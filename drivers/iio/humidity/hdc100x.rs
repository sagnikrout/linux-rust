//! Automatically rewritten from C to Rust
//! Source: drivers/iio/humidity/hdc100x.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// hdc100x.c - Support for the TI HDC100x temperature + humidity sensors
//
// Copyright (C) 2015, 2018
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//
// Datasheets:
// https://www.ti.com/product/HDC1000/datasheet
// https://www.ti.com/product/HDC1008/datasheet
// https://www.ti.com/product/HDC1010/datasheet
// https://www.ti.com/product/HDC1050/datasheet
// https://www.ti.com/product/HDC1080/datasheet
//

pub const HDC100X_REG_TEMP: c_uint = 0x00;
pub const HDC100X_REG_HUMIDITY: c_uint = 0x01;
pub const HDC100X_REG_CONFIG: c_uint = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdc100x_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub config: u16,
// integration time of the sensor
    pub adc_int_us: [c_int; 2],
// Ensure natural alignment of timestamp
    struct {
    pub channels: [__be16; 2],
    pub ts: aligned_s64,
    pub scan: },
}

// integration time in us
    static const int hdc100x_int_time[][3] = {
    { 6350, 3650, 0 },	/* IIO_TEMP channel*/
    { 6500, 3850, 2500 },	/* IIO_HUMIDITYRELATIVE channel */
    };
// HDC100X_REG_CONFIG shift and mask values
    static const struct {
    int shift;
    int mask;
    } hdc100x_resolution_shift[2] = {
    { /* IIO_TEMP channel */
    .shift = 10,
    .mask = 1
    },
    { /* IIO_HUMIDITYRELATIVE channel */
    .shift = 8,
    .mask = 3,
    },
    };
    static IIO_CONST_ATTR(temp_integration_time_available,
    "0.00365 0.00635");
    static IIO_CONST_ATTR(humidityrelative_integration_time_available,
    "0.0025 0.00385 0.0065");
    static IIO_CONST_ATTR(out_current_heater_raw_available,
    "0 1");
    static struct attribute *hdc100x_attributes[] = {
    &iio_const_attr_temp_integration_time_available.dev_attr.attr,
    &iio_const_attr_humidityrelative_integration_time_available.dev_attr.attr,
    &iio_const_attr_out_current_heater_raw_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group hdc100x_attribute_group = {
    .attrs = hdc100x_attributes,
    };
    static const struct iio_chan_spec hdc100x_channels[] = {
    {
    .type = IIO_TEMP,
    .address = HDC100X_REG_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_INT_TIME) |
    BIT(IIO_CHAN_INFO_OFFSET),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_BE,
    },
    },
    {
    .type = IIO_HUMIDITYRELATIVE,
    .address = HDC100X_REG_HUMIDITY,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_INT_TIME),
    .scan_index = 1,
    .scan_type = {
    .sign = 'u',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_BE,
    },
    },
    {
    .type = IIO_CURRENT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .extend_name = "heater",
    .output = 1,
    .scan_index = -1,
    },
    IIO_CHAN_SOFT_TIMESTAMP(2),
    };
    static const unsigned long hdc100x_scan_masks[] = {0x3, 0};
#[no_mangle]
unsafe extern "C" fn hdc100x_update_config(data: *mut hdc100x_data, mask: c_int, val: c_int) -> c_int {
    static int hdc100x_update_config(struct hdc100x_data *data, int mask, int val)
    {
    let mut tmp: c_int = (~mask & data.config) | val;
    int ret;
    ret = i2c_smbus_write_word_swapped(data.client,
    HDC100X_REG_CONFIG, tmp);
    if (!ret)
    data.config = tmp;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hdc100x_set_it_time(data: *mut hdc100x_data, chan: c_int, val2: c_int) -> c_int {
    static int hdc100x_set_it_time(struct hdc100x_data *data, int chan, int val2)
    {
    let mut shift: c_int = hdc100x_resolution_shift[chan].shift;
    let mut ret: c_int = -EINVAL;
    int i;
    for (i = 0; i < ARRAY_SIZE(hdc100x_int_time[chan]); i++) {
    if (val2 && val2 == hdc100x_int_time[chan][i]) {
    ret = hdc100x_update_config(data,
    hdc100x_resolution_shift[chan].mask << shift,
    i << shift);
    if (!ret)
    data.adc_int_us[chan] = val2;
    break;
    }
    }
    return ret;
    }
    static int hdc100x_get_measurement(struct hdc100x_data *data,
    struct iio_chan_spec const *chan)
    {
    struct i2c_client *client = data.client;
    let mut delay: c_int = data.adc_int_us[chan.address] + 1*USEC_PER_MSEC;
    int ret;
    __be16 val;
// start measurement
    ret = i2c_smbus_write_byte(client, chan.address);
    if (ret < 0) {
    dev_err(&client.dev, "cannot start measurement");
    return ret;
    }
// wait for integration time to pass
    usleep_range(delay, delay + 1000);
// read measurement
    ret = i2c_master_recv(data.client, (char *)&val, sizeof(val));
    if (ret < 0) {
    dev_err(&client.dev, "cannot read sensor data\n");
    return ret;
    }
    return be16_to_cpu(val);
    }
#[no_mangle]
unsafe extern "C" fn hdc100x_get_heater_status(data: *mut hdc100x_data) -> c_int {
    static int hdc100x_get_heater_status(struct hdc100x_data *data)
    {
    return !!(data.config & HDC100X_REG_CONFIG_HEATER_EN);
    }
    static int hdc100x_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct hdc100x_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW: {
    int ret;
    guard(mutex)(&data.lock);
    if (chan.type == IIO_CURRENT) {
// val = hdc100x_get_heater_status(data);
    return IIO_VAL_INT;
    }
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = hdc100x_get_measurement(data, chan);
    iio_device_release_direct(indio_dev);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_INT_TIME:
// val = 0;
// val2 = data->adc_int_us[chan->address];
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_SCALE:
    if (chan.type == IIO_TEMP) {
// val = 165000;
// val2 = 65536;
    return IIO_VAL_FRACTIONAL;
    } else {
// val = 100000;
// val2 = 65536;
    return IIO_VAL_FRACTIONAL;
    }
    break;
    case IIO_CHAN_INFO_OFFSET:
// val = -15887;
// val2 = 515151;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static int hdc100x_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct hdc100x_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_INT_TIME: {
    if (val != 0)
    return -EINVAL;
    guard(mutex)(&data.lock);
    return hdc100x_set_it_time(data, chan.address, val2);
    }
    case IIO_CHAN_INFO_RAW: {
    if (chan.type != IIO_CURRENT || val2 != 0)
    return -EINVAL;
    guard(mutex)(&data.lock);
    return hdc100x_update_config(data, HDC100X_REG_CONFIG_HEATER_EN,
    val ? HDC100X_REG_CONFIG_HEATER_EN : 0);
    }
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn hdc100x_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int hdc100x_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct hdc100x_data *data = iio_priv(indio_dev);
// Buffer is enabled. First set ACQ Mode, then attach poll func
    guard(mutex)(&data.lock);
    return hdc100x_update_config(data, HDC100X_REG_CONFIG_ACQ_MODE,
    HDC100X_REG_CONFIG_ACQ_MODE);
    }
#[no_mangle]
unsafe extern "C" fn hdc100x_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int hdc100x_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct hdc100x_data *data = iio_priv(indio_dev);
    guard(mutex)(&data.lock);
    return hdc100x_update_config(data, HDC100X_REG_CONFIG_ACQ_MODE, 0);
    }
    static const struct iio_buffer_setup_ops hdc_buffer_setup_ops = {
    .postenable  = hdc100x_buffer_postenable,
    .predisable  = hdc100x_buffer_predisable,
    };
#[no_mangle]
unsafe extern "C" fn hdc100x_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t hdc100x_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct hdc100x_data *data = iio_priv(indio_dev);
    struct i2c_client *client = data.client;
    let mut delay: c_int = data.adc_int_us[0] + data.adc_int_us[1] + 2*USEC_PER_MSEC;
    int ret;
// dual read starts at temp register
    mutex_lock(&data.lock);
    ret = i2c_smbus_write_byte(client, HDC100X_REG_TEMP);
    if (ret < 0) {
    dev_err(&client.dev, "cannot start measurement\n");
    goto err;
    }
    usleep_range(delay, delay + 1000);
    ret = i2c_master_recv(client, (u8 *)data.scan.channels, 4);
    if (ret < 0) {
    dev_err(&client.dev, "cannot read sensor data\n");
    goto err;
    }
    iio_push_to_buffers_with_timestamp(indio_dev, &data.scan,
    iio_get_time_ns(indio_dev));
    err:
    mutex_unlock(&data.lock);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static const struct iio_info hdc100x_info = {
    .read_raw = hdc100x_read_raw,
    .write_raw = hdc100x_write_raw,
    .attrs = &hdc100x_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn hdc100x_probe(client: *mut i2c_client) -> c_int {
    static int hdc100x_probe(struct i2c_client *client)
    {
    struct iio_dev *indio_dev;
    struct hdc100x_data *data;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_WORD_DATA |
    I2C_FUNC_SMBUS_BYTE | I2C_FUNC_I2C))
    return -EOPNOTSUPP;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    mutex_init(&data.lock);
    indio_dev.name = dev_name(&client.dev);
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &hdc100x_info;
    indio_dev.channels = hdc100x_channels;
    indio_dev.num_channels = ARRAY_SIZE(hdc100x_channels);
    indio_dev.available_scan_masks = hdc100x_scan_masks;
// be sure we are in a known state
    hdc100x_set_it_time(data, 0, hdc100x_int_time[0][0]);
    hdc100x_set_it_time(data, 1, hdc100x_int_time[1][0]);
    hdc100x_update_config(data, HDC100X_REG_CONFIG_ACQ_MODE, 0);
    ret = devm_iio_triggered_buffer_setup(&client.dev,
    indio_dev, core::ptr::null_mut(),
    hdc100x_trigger_handler,
    &hdc_buffer_setup_ops);
    if (ret < 0) {
    dev_err(&client.dev, "iio triggered buffer setup failed\n");
    return ret;
    }
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id hdc100x_id[] = {
    { .name = "hdc100x" },
    { .name = "hdc1000" },
    { .name = "hdc1008" },
    { .name = "hdc1010" },
    { .name = "hdc1050" },
    { .name = "hdc1080" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, hdc100x_id);
    static const struct of_device_id hdc100x_dt_ids[] = {
    { .compatible = "ti,hdc1000" },
    { .compatible = "ti,hdc1008" },
    { .compatible = "ti,hdc1010" },
    { .compatible = "ti,hdc1050" },
    { .compatible = "ti,hdc1080" },
    { }
    };
    MODULE_DEVICE_TABLE(of, hdc100x_dt_ids);
    static const struct acpi_device_id hdc100x_acpi_match[] = {
    { "TXNW1010" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, hdc100x_acpi_match);
    static struct i2c_driver hdc100x_driver = {
    .driver = {
    .name	= "hdc100x",
    .of_match_table = hdc100x_dt_ids,
    .acpi_match_table = hdc100x_acpi_match,
    },
    .probe = hdc100x_probe,
    .id_table = hdc100x_id,
    };
    module_i2c_driver(hdc100x_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("TI HDC100x humidity and temperature sensor driver");
    MODULE_LICENSE("GPL");
