//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/da280.c
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
// IIO driver for the MiraMEMS DA217 and DA280 3-axis accelerometer and
// IIO driver for the MiraMEMS DA226 2-axis accelerometer
//
// Copyright (c) 2016 Hans de Goede <hdegoede@redhat.com>
//

pub const DA280_REG_CHIP_ID: c_uint = 0x01;
pub const DA280_REG_ACC_X_LSB: c_uint = 0x02;
pub const DA280_REG_ACC_Y_LSB: c_uint = 0x04;
pub const DA280_REG_ACC_Z_LSB: c_uint = 0x06;
pub const DA280_REG_MODE_BW: c_uint = 0x11;
pub const DA280_CHIP_ID: c_uint = 0x13;
pub const DA280_MODE_ENABLE: c_uint = 0x1e;
pub const DA280_MODE_DISABLE: c_uint = 0x9e;
//
// a value of + or -4096 corresponds to + or - 1G
// scale = 9.81 / 4096 = 0.002395019
//
    let mut da280_nscale: static int = 2395019;

    .type = IIO_ACCEL,	\
    .address = reg,	\
    .modified = 1,	\
    .channel2 = IIO_MOD_##axis,	\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    }
    static const struct iio_chan_spec da280_channels[] = {
    DA280_CHANNEL(DA280_REG_ACC_X_LSB, X),
    DA280_CHANNEL(DA280_REG_ACC_Y_LSB, Y),
    DA280_CHANNEL(DA280_REG_ACC_Z_LSB, Z),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da280_match_data {
    pub name: *const c_char,
    pub num_channels: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da280_data {
    pub client: *mut i2c_client,
}

#[no_mangle]
unsafe extern "C" fn da280_enable(client: *mut i2c_client, enable: bool) -> c_int {
    static int da280_enable(struct i2c_client *client, bool enable)
    {
    let mut data: u8 = enable ? DA280_MODE_ENABLE : DA280_MODE_DISABLE;
    return i2c_smbus_write_byte_data(client, DA280_REG_MODE_BW, data);
    }
    static int da280_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct da280_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = i2c_smbus_read_word_data(data.client, chan.address);
    if (ret < 0)
    return ret;
//
// Values are 14 bits, stored as 16 bits with the 2
// least significant bits always 0.
//
// val = (short)ret >> 2;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// val2 = da280_nscale;
    return IIO_VAL_INT_PLUS_NANO;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info da280_info = {
    .read_raw	= da280_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn da280_disable(client: *mut c_void) {
    static void da280_disable(void *client)
    {
    da280_enable(client, false);
    }
#[no_mangle]
unsafe extern "C" fn da280_probe(client: *mut i2c_client) -> c_int {
    static int da280_probe(struct i2c_client *client)
    {
    const struct da280_match_data *match_data;
    struct iio_dev *indio_dev;
    struct da280_data *data;
    int ret;
    ret = i2c_smbus_read_byte_data(client, DA280_REG_CHIP_ID);
    if (ret != DA280_CHIP_ID)
    return (ret < 0) ? ret : -ENODEV;
    match_data = i2c_get_match_data(client);
    if (!match_data) {
    dev_err(&client.dev, "Error match-data not set\n");
    return -EINVAL;
    }
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    indio_dev.info = &da280_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = da280_channels;
    indio_dev.num_channels = match_data.num_channels;
    indio_dev.name = match_data.name;
    ret = da280_enable(client, true);
    if (ret < 0)
    return ret;
    ret = devm_add_action_or_reset(&client.dev, da280_disable, client);
    if (ret)
    return ret;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn da280_suspend(dev: *mut device) -> c_int {
    static int da280_suspend(struct device *dev)
    {
    return da280_enable(to_i2c_client(dev), false);
    }
#[no_mangle]
unsafe extern "C" fn da280_resume(dev: *mut device) -> c_int {
    static int da280_resume(struct device *dev)
    {
    return da280_enable(to_i2c_client(dev), true);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(da280_pm_ops, da280_suspend, da280_resume);
    let mut da217_match_data: static struct da280_match_data = { "da217", 3 };
    let mut da226_match_data: static struct da280_match_data = { "da226", 2 };
    let mut da280_match_data: static struct da280_match_data = { "da280", 3 };
    static const struct acpi_device_id da280_acpi_match[] = {
    { "NSA2513", (kernel_ulong_t)&da217_match_data },
    { "MIRAACC", (kernel_ulong_t)&da280_match_data },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, da280_acpi_match);
    static const struct i2c_device_id da280_i2c_id[] = {
    { .name = "da217", .driver_data = (kernel_ulong_t)&da217_match_data },
    { .name = "da226", .driver_data = (kernel_ulong_t)&da226_match_data },
    { .name = "da280", .driver_data = (kernel_ulong_t)&da280_match_data },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, da280_i2c_id);
    static struct i2c_driver da280_driver = {
    .driver = {
    .name = "da280",
    .acpi_match_table = da280_acpi_match,
    .pm = pm_sleep_ptr(&da280_pm_ops),
    },
    .probe		= da280_probe,
    .id_table	= da280_i2c_id,
    };
    module_i2c_driver(da280_driver);
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_DESCRIPTION("MiraMEMS DA280 3-Axis Accelerometer driver");
    MODULE_LICENSE("GPL v2");
