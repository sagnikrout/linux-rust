//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/dmard06.c
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
// IIO driver for Domintech DMARD06 accelerometer
//
// Copyright (C) 2016 Aleksei Mamlin <mamlinav@gmail.com>
//

// Device data registers
pub const DMARD06_CHIP_ID_REG: c_uint = 0x0f;
pub const DMARD06_TOUT_REG: c_uint = 0x40;
pub const DMARD06_XOUT_REG: c_uint = 0x41;
pub const DMARD06_YOUT_REG: c_uint = 0x42;
pub const DMARD06_ZOUT_REG: c_uint = 0x43;
pub const DMARD06_CTRL1_REG: c_uint = 0x44;
// Device ID value
pub const DMARD05_CHIP_ID: c_uint = 0x05;
pub const DMARD06_CHIP_ID: c_uint = 0x06;
pub const DMARD07_CHIP_ID: c_uint = 0x07;
// Device values
pub const DMARD05_AXIS_SCALE_VAL: c_int = 15625;
pub const DMARD06_AXIS_SCALE_VAL: c_int = 31250;
pub const DMARD06_TEMP_CENTER_VAL: c_int = 25;
pub const DMARD06_SIGN_BIT: c_int = 7;
// Device power modes
pub const DMARD06_MODE_NORMAL: c_uint = 0x27;
pub const DMARD06_MODE_POWERDOWN: c_uint = 0x00;
// Device channels

    .type = IIO_ACCEL,					\
    .address = _reg,					\
    .channel2 = IIO_MOD_##_axis,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .modified = 1,						\
    }

    .type = IIO_TEMP,					\
    .address = _reg,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		\
    BIT(IIO_CHAN_INFO_OFFSET),	\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmard06_data {
    pub client: *mut i2c_client,
    pub chip_id: u8,
}

    static const struct iio_chan_spec dmard06_channels[] = {
    DMARD06_ACCEL_CHANNEL(X, DMARD06_XOUT_REG),
    DMARD06_ACCEL_CHANNEL(Y, DMARD06_YOUT_REG),
    DMARD06_ACCEL_CHANNEL(Z, DMARD06_ZOUT_REG),
    DMARD06_TEMP_CHANNEL(DMARD06_TOUT_REG),
    };
    static int dmard06_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct dmard06_data *dmard06 = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = i2c_smbus_read_byte_data(dmard06.client,
    chan.address);
    if (ret < 0) {
    dev_err(&dmard06.client.dev,
    "Error reading data: %d\n", ret);
    return ret;
    }
// val = sign_extend32(ret, DMARD06_SIGN_BIT);
    if (dmard06.chip_id == DMARD06_CHIP_ID)
// val = *val >> 1;
    switch (chan.type) {
    case IIO_ACCEL:
    return IIO_VAL_INT;
    case IIO_TEMP:
    if (dmard06.chip_id != DMARD06_CHIP_ID)
// val = *val / 2;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_OFFSET:
    switch (chan.type) {
    case IIO_TEMP:
// val = DMARD06_TEMP_CENTER_VAL;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_ACCEL:
// val = 0;
    if (dmard06.chip_id == DMARD06_CHIP_ID)
// val2 = DMARD06_AXIS_SCALE_VAL;
    else
// val2 = DMARD05_AXIS_SCALE_VAL;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info dmard06_info = {
    .read_raw	= dmard06_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn dmard06_probe(client: *mut i2c_client) -> c_int {
    static int dmard06_probe(struct i2c_client *client)
    {
    int ret;
    struct iio_dev *indio_dev;
    struct dmard06_data *dmard06;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(&client.dev, "I2C check functionality failed\n");
    return -ENXIO;
    }
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*dmard06));
    if (!indio_dev)
    return -ENOMEM;
    dmard06 = iio_priv(indio_dev);
    dmard06.client = client;
    ret = i2c_smbus_read_byte_data(dmard06.client, DMARD06_CHIP_ID_REG);
    if (ret < 0) {
    dev_err(&client.dev, "Error reading chip id: %d\n", ret);
    return ret;
    }
    if (ret != DMARD05_CHIP_ID && ret != DMARD06_CHIP_ID &&
    ret != DMARD07_CHIP_ID) {
    dev_err(&client.dev, "Invalid chip id: %02d\n", ret);
    return -ENODEV;
    }
    dmard06.chip_id = ret;
    i2c_set_clientdata(client, indio_dev);
    indio_dev.name = DMARD06_DRV_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = dmard06_channels;
    indio_dev.num_channels = ARRAY_SIZE(dmard06_channels);
    indio_dev.info = &dmard06_info;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn dmard06_suspend(dev: *mut device) -> c_int {
    static int dmard06_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct dmard06_data *dmard06 = iio_priv(indio_dev);
    int ret;
    ret = i2c_smbus_write_byte_data(dmard06.client, DMARD06_CTRL1_REG,
    DMARD06_MODE_POWERDOWN);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmard06_resume(dev: *mut device) -> c_int {
    static int dmard06_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct dmard06_data *dmard06 = iio_priv(indio_dev);
    int ret;
    ret = i2c_smbus_write_byte_data(dmard06.client, DMARD06_CTRL1_REG,
    DMARD06_MODE_NORMAL);
    if (ret < 0)
    return ret;
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(dmard06_pm_ops, dmard06_suspend,
    dmard06_resume);
    static const struct i2c_device_id dmard06_id[] = {
    { .name = "dmard05" },
    { .name = "dmard06" },
    { .name = "dmard07" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, dmard06_id);
    static const struct of_device_id dmard06_of_match[] = {
    { .compatible = "domintech,dmard05" },
    { .compatible = "domintech,dmard06" },
    { .compatible = "domintech,dmard07" },
    { }
    };
    MODULE_DEVICE_TABLE(of, dmard06_of_match);
    static struct i2c_driver dmard06_driver = {
    .probe = dmard06_probe,
    .id_table = dmard06_id,
    .driver = {
    .name = DMARD06_DRV_NAME,
    .of_match_table = dmard06_of_match,
    .pm = pm_sleep_ptr(&dmard06_pm_ops),
    },
    };
    module_i2c_driver(dmard06_driver);
    MODULE_AUTHOR("Aleksei Mamlin <mamlinav@gmail.com>");
    MODULE_DESCRIPTION("Domintech DMARD06 accelerometer driver");
    MODULE_LICENSE("GPL v2");
