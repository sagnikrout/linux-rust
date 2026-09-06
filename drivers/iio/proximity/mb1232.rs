//! Automatically rewritten from C to Rust
//! Source: drivers/iio/proximity/mb1232.c
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
// mb1232.c - Support for MaxBotix I2CXL-MaxSonar-EZ series ultrasonic
// ranger with i2c interface
// actually tested with mb1232 type
//
// Copyright (c) 2019 Andreas Klinger <ak@it-klinger.de>
//
// For details about the device see:
// https://www.maxbotix.com/documents/I2CXL-MaxSonar-EZ_Datasheet.pdf
//

// registers of MaxSonar device
pub const MB1232_RANGE_COMMAND: c_uint = 0x51	/* Command for reading range */;
pub const MB1232_ADDR_UNLOCK_1: c_uint = 0xAA	/* Command 1 for changing address */;
pub const MB1232_ADDR_UNLOCK_2: c_uint = 0xA5	/* Command 2 for changing address */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mb1232_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
//
// optionally a gpio can be used to announce when ranging has
// finished
// since we are just using the falling trigger of it we request
// only the interrupt for announcing when data is ready to be read
//
    pub ranging: completion,
    pub irqnr: c_int,
}

#[no_mangle]
unsafe extern "C" fn mb1232_handle_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mb1232_handle_irq(int irq, void *dev_id)
    {
    struct iio_dev *indio_dev = dev_id;
    struct mb1232_data *data = iio_priv(indio_dev);
    complete(&data.ranging);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mb1232_read_distance(data: *mut mb1232_data) -> i16 {
    static s16 mb1232_read_distance(struct mb1232_data *data)
    {
    struct i2c_client *client = data.client;
    int ret;
    s16 distance;
    __be16 buf;
    mutex_lock(&data.lock);
    reinit_completion(&data.ranging);
    ret = i2c_smbus_write_byte(client, MB1232_RANGE_COMMAND);
    if (ret < 0) {
    dev_err(&client.dev, "write command - err: %d\n", ret);
    goto error_unlock;
    }
    if (data.irqnr > 0) {
// it cannot take more than 100 ms
    ret = wait_for_completion_killable_timeout(&data.ranging,
    HZ/10);
    if (ret < 0)
    goto error_unlock;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret ==) -> else {
    ret = -ETIMEDOUT;
    goto error_unlock;
    }
    } else {
// use simple sleep if announce irq is not connected
    msleep(15);
    }
    ret = i2c_master_recv(client, (char *)&buf, sizeof(buf));
    if (ret < 0) {
    dev_err(&client.dev, "i2c_master_recv: ret=%d\n", ret);
    goto error_unlock;
    }
    distance = __be16_to_cpu(buf);
// check for not returning misleading error codes
    if (distance < 0) {
    dev_err(&client.dev, "distance=%d\n", distance);
    ret = -EINVAL;
    goto error_unlock;
    }
    mutex_unlock(&data.lock);
    return distance;
    error_unlock:
    mutex_unlock(&data.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mb1232_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t mb1232_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct mb1232_data *data = iio_priv(indio_dev);
    struct {
    s16 distance;
    aligned_s64 ts;
    } scan = { };
    scan.distance = mb1232_read_distance(data);
    if (scan.distance < 0)
    goto err;
    iio_push_to_buffers_with_ts(indio_dev, &scan, sizeof(scan),
    pf.timestamp);
    err:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static int mb1232_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *channel, int *val,
    int *val2, long mask)
    {
    struct mb1232_data *data = iio_priv(indio_dev);
    int ret;
    if (channel.type != IIO_DISTANCE)
    return -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = mb1232_read_distance(data);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// 1 LSB is 1 cm
// val = 0;
// val2 = 10000;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_chan_spec mb1232_channels[] = {
    {
    .type = IIO_DISTANCE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_CPU,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
    static const struct iio_info mb1232_info = {
    .read_raw = mb1232_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn mb1232_probe(client: *mut i2c_client) -> c_int {
    static int mb1232_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct iio_dev *indio_dev;
    struct mb1232_data *data;
    int ret;
    struct device *dev = &client.dev;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_BYTE |
    I2C_FUNC_SMBUS_WRITE_BYTE))
    return -ENODEV;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    indio_dev.info = &mb1232_info;
    indio_dev.name = id.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = mb1232_channels;
    indio_dev.num_channels = ARRAY_SIZE(mb1232_channels);
    mutex_init(&data.lock);
    init_completion(&data.ranging);
    data.irqnr = fwnode_irq_get(dev_fwnode(&client.dev), 0);
    if (data.irqnr > 0) {
    ret = devm_request_irq(dev, data.irqnr, mb1232_handle_irq,
    IRQF_TRIGGER_FALLING, id.name, indio_dev);
    if (ret)
    return ret;
    }
    ret = devm_iio_triggered_buffer_setup(dev, indio_dev,
    iio_pollfunc_store_time, mb1232_trigger_handler, core::ptr::null_mut());
    if (ret < 0) {
    dev_err(dev, "setup of iio triggered buffer failed\n");
    return ret;
    }
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id of_mb1232_match[] = {
    { .compatible = "maxbotix,mb1202", },
    { .compatible = "maxbotix,mb1212", },
    { .compatible = "maxbotix,mb1222", },
    { .compatible = "maxbotix,mb1232", },
    { .compatible = "maxbotix,mb1242", },
    { .compatible = "maxbotix,mb7040", },
    { .compatible = "maxbotix,mb7137", },
    { }
    };
    MODULE_DEVICE_TABLE(of, of_mb1232_match);
    static const struct i2c_device_id mb1232_id[] = {
    { .name = "maxbotix-mb1202" },
    { .name = "maxbotix-mb1212" },
    { .name = "maxbotix-mb1222" },
    { .name = "maxbotix-mb1232" },
    { .name = "maxbotix-mb1242" },
    { .name = "maxbotix-mb7040" },
    { .name = "maxbotix-mb7137" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mb1232_id);
    static struct i2c_driver mb1232_driver = {
    .driver = {
    .name	= "maxbotix-mb1232",
    .of_match_table	= of_mb1232_match,
    },
    .probe = mb1232_probe,
    .id_table = mb1232_id,
    };
    module_i2c_driver(mb1232_driver);
    MODULE_AUTHOR("Andreas Klinger <ak@it-klinger.de>");
    MODULE_DESCRIPTION("Maxbotix I2CXL-MaxSonar i2c ultrasonic ranger driver");
    MODULE_LICENSE("GPL");
