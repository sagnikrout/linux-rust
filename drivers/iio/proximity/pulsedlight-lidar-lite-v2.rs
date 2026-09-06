//! Automatically rewritten from C to Rust
//! Source: drivers/iio/proximity/pulsedlight-lidar-lite-v2.c
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
// pulsedlight-lidar-lite-v2.c - Support for PulsedLight LIDAR sensor
//
// Copyright (C) 2015, 2017-2018
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//
// TODO: interrupt mode, and signal strength reporting
//

pub const LIDAR_REG_CONTROL: c_uint = 0x00;

pub const LIDAR_REG_STATUS: c_uint = 0x01;

pub const LIDAR_REG_DATA_HBYTE: c_uint = 0x0f;
pub const LIDAR_REG_DATA_LBYTE: c_uint = 0x10;

pub const LIDAR_REG_PWR_CONTROL: c_uint = 0x65;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lidar_data {
    pub indio_dev: *mut iio_dev,
    pub client: *mut i2c_client,
    pub len): *mut *mut *mut *mut int (xfer)(struct lidar_data data, u8 reg, u8 val, int,
    pub i2c_enabled: c_int,
}

    static const struct iio_chan_spec lidar_channels[] = {
    {
    .type = IIO_DISTANCE,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 'u',
    .realbits = 16,
    .storagebits = 16,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
#[no_mangle]
unsafe extern "C" fn lidar_i2c_xfer(data: *mut lidar_data, reg: u8, val: *mut u8, len: c_int) -> c_int {
    static int lidar_i2c_xfer(struct lidar_data *data, u8 reg, u8 *val, int len)
    {
    struct i2c_client *client = data.client;
    struct i2c_msg msg[2];
    int ret;
    msg[0].addr = client.addr;
    msg[0].flags = client.flags | I2C_M_STOP;
    msg[0].len = 1;
    msg[0].buf  = (char *) &reg;
    msg[1].addr = client.addr;
    msg[1].flags = client.flags | I2C_M_RD;
    msg[1].len = len;
    msg[1].buf = (char *) val;
    ret = i2c_transfer(client.adapter, msg, 2);
    return (ret == 2) ? 0 : -EIO;
    }
#[no_mangle]
unsafe extern "C" fn lidar_smbus_xfer(data: *mut lidar_data, reg: u8, val: *mut u8, len: c_int) -> c_int {
    static int lidar_smbus_xfer(struct lidar_data *data, u8 reg, u8 *val, int len)
    {
    struct i2c_client *client = data.client;
    int ret;
//
// Device needs a STOP condition between address write, and data read
// so in turn i2c_smbus_read_byte_data cannot be used
//
    while (len--) {
    ret = i2c_smbus_write_byte(client, reg++);
    if (ret < 0) {
    dev_err(&client.dev, "cannot write addr value");
    return ret;
    }
    ret = i2c_smbus_read_byte(client);
    if (ret < 0) {
    dev_err(&client.dev, "cannot read data value");
    return ret;
    }
// (val++) = ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lidar_read_byte(data: *mut lidar_data, reg: u8) -> c_int {
    static int lidar_read_byte(struct lidar_data *data, u8 reg)
    {
    int ret;
    u8 val;
    ret = data.xfer(data, reg, &val, 1);
    if (ret < 0)
    return ret;
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn lidar_write_control(data: *mut lidar_data, val: c_int) -> c_int {
    static inline int lidar_write_control(struct lidar_data *data, int val)
    {
    return i2c_smbus_write_byte_data(data.client, LIDAR_REG_CONTROL, val);
    }
#[no_mangle]
pub unsafe extern "C" fn lidar_write_power(data: *mut lidar_data, val: c_int) -> c_int {
    static inline int lidar_write_power(struct lidar_data *data, int val)
    {
    return i2c_smbus_write_byte_data(data.client,
    LIDAR_REG_PWR_CONTROL, val);
    }
#[no_mangle]
unsafe extern "C" fn lidar_read_measurement(data: *mut lidar_data, reg: *mut u16) -> c_int {
    static int lidar_read_measurement(struct lidar_data *data, u16 *reg)
    {
    __be16 value;
    int ret = data.xfer(data, LIDAR_REG_DATA_HBYTE |
    (data.i2c_enabled ? LIDAR_REG_DATA_WORD_READ : 0),
    (u8 *) &value, 2);
    if (!ret)
// reg = be16_to_cpu(value);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lidar_get_measurement(data: *mut lidar_data, reg: *mut u16) -> c_int {
    static int lidar_get_measurement(struct lidar_data *data, u16 *reg)
    {
    struct i2c_client *client = data.client;
    let mut tries: c_int = 10;
    int ret;
    ret = pm_runtime_resume_and_get(&client.dev);
    if (ret < 0)
    return ret;
// start sample
    ret = lidar_write_control(data, LIDAR_REG_CONTROL_ACQUIRE);
    if (ret < 0) {
    dev_err(&client.dev, "cannot send start measurement command");
    pm_runtime_put_noidle(&client.dev);
    return ret;
    }
    while (tries--) {
    usleep_range(1000, 2000);
    ret = lidar_read_byte(data, LIDAR_REG_STATUS);
    if (ret < 0)
    break;
// return -EINVAL since laser is likely pointed out of range
    if (ret & LIDAR_REG_STATUS_INVALID) {
// reg = 0;
    ret = -EINVAL;
    break;
    }
// sample ready to read
    if (!(ret & LIDAR_REG_STATUS_READY)) {
    ret = lidar_read_measurement(data, reg);
    break;
    }
    ret = -EIO;
    }
    pm_runtime_put_autosuspend(&client.dev);
    return ret;
    }
    static int lidar_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct lidar_data *data = iio_priv(indio_dev);
    let mut ret: c_int = -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW: {
    u16 reg;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = lidar_get_measurement(data, &reg);
    if (!ret) {
// val = reg;
    ret = IIO_VAL_INT;
    }
    iio_device_release_direct(indio_dev);
    break;
    }
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// val2 = 10000;
    ret = IIO_VAL_INT_PLUS_MICRO;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lidar_trigger_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t lidar_trigger_handler(int irq, void *private)
    {
    struct iio_poll_func *pf = private;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct lidar_data *data = iio_priv(indio_dev);
    int ret;
    struct {
    u16 chan;
    aligned_s64 timestamp;
    } scan = { };
    ret = lidar_get_measurement(data, &scan.chan);
    if (!ret) {
    iio_push_to_buffers_with_ts(indio_dev, &scan, sizeof(scan),
    iio_get_time_ns(indio_dev));
    } else if (ret != -EINVAL) {
    dev_err(&data.client.dev, "cannot read LIDAR measurement");
    }
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static const struct iio_info lidar_info = {
    .read_raw = lidar_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn lidar_probe(client: *mut i2c_client) -> c_int {
    static int lidar_probe(struct i2c_client *client)
    {
    struct lidar_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    if (i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    data.xfer = lidar_i2c_xfer;
    data.i2c_enabled = 1;
    } else if (i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_WORD_DATA | I2C_FUNC_SMBUS_BYTE))
    data.xfer = lidar_smbus_xfer;
    else
    return -EOPNOTSUPP;
    indio_dev.info = &lidar_info;
    indio_dev.name = LIDAR_DRV_NAME;
    indio_dev.channels = lidar_channels;
    indio_dev.num_channels = ARRAY_SIZE(lidar_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    data.indio_dev = indio_dev;
    ret = iio_triggered_buffer_setup(indio_dev, core::ptr::null_mut(),
    lidar_trigger_handler, core::ptr::null_mut());
    if (ret)
    return ret;
    ret = iio_device_register(indio_dev);
    if (ret)
    goto error_unreg_buffer;
    pm_runtime_set_autosuspend_delay(&client.dev, 1000);
    pm_runtime_use_autosuspend(&client.dev);
    ret = pm_runtime_set_active(&client.dev);
    if (ret)
    goto error_unreg_buffer;
    pm_runtime_enable(&client.dev);
    pm_runtime_idle(&client.dev);
    return 0;
    error_unreg_buffer:
    iio_triggered_buffer_cleanup(indio_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lidar_remove(client: *mut i2c_client) {
    static void lidar_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    iio_device_unregister(indio_dev);
    iio_triggered_buffer_cleanup(indio_dev);
    pm_runtime_disable(&client.dev);
    pm_runtime_set_suspended(&client.dev);
    }
    static const struct i2c_device_id lidar_id[] = {
    { .name = "lidar-lite-v2" },
    { .name = "lidar-lite-v3" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lidar_id);
    static const struct of_device_id lidar_dt_ids[] = {
    { .compatible = "pulsedlight,lidar-lite-v2" },
    { .compatible = "grmn,lidar-lite-v3" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lidar_dt_ids);
#[no_mangle]
unsafe extern "C" fn lidar_pm_runtime_suspend(dev: *mut device) -> c_int {
    static int lidar_pm_runtime_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct lidar_data *data = iio_priv(indio_dev);
    return lidar_write_power(data, 0x0f);
    }
#[no_mangle]
unsafe extern "C" fn lidar_pm_runtime_resume(dev: *mut device) -> c_int {
    static int lidar_pm_runtime_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct lidar_data *data = iio_priv(indio_dev);
    let mut ret: c_int = lidar_write_power(data, 0);
// regulator and FPGA needs settling time
    usleep_range(15000, 20000);
    return ret;
    }
    static const struct dev_pm_ops lidar_pm_ops = {
    RUNTIME_PM_OPS(lidar_pm_runtime_suspend, lidar_pm_runtime_resume, core::ptr::null_mut())
    };
    static struct i2c_driver lidar_driver = {
    .driver = {
    .name	= LIDAR_DRV_NAME,
    .of_match_table	= lidar_dt_ids,
    .pm	= pm_ptr(&lidar_pm_ops),
    },
    .probe		= lidar_probe,
    .remove		= lidar_remove,
    .id_table	= lidar_id,
    };
    module_i2c_driver(lidar_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("PulsedLight LIDAR sensor");
    MODULE_LICENSE("GPL");
