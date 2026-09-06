//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/da311.c
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
// IIO driver for the MiraMEMS DA311 3-axis accelerometer
//
// Copyright (c) 2016 Hans de Goede <hdegoede@redhat.com>
// Copyright (c) 2011-2013 MiraMEMS Sensing Technology Co., Ltd.
//

pub const DA311_CHIP_ID: c_uint = 0x13;
//
// Note register addressed go from 0 - 0x3f and then wrap.
// For some reason there are 2 banks with 0 - 0x3f addresses,
// rather then a single 0-0x7f bank.
//
// Bank 0 regs
pub const DA311_REG_BANK: c_uint = 0x0000;
pub const DA311_REG_LDO_REG: c_uint = 0x0006;
pub const DA311_REG_CHIP_ID: c_uint = 0x000f;
pub const DA311_REG_TEMP_CFG_REG: c_uint = 0x001f;
pub const DA311_REG_CTRL_REG1: c_uint = 0x0020;
pub const DA311_REG_CTRL_REG3: c_uint = 0x0022;
pub const DA311_REG_CTRL_REG4: c_uint = 0x0023;
pub const DA311_REG_CTRL_REG5: c_uint = 0x0024;
pub const DA311_REG_CTRL_REG6: c_uint = 0x0025;
pub const DA311_REG_STATUS_REG: c_uint = 0x0027;
pub const DA311_REG_OUT_X_L: c_uint = 0x0028;
pub const DA311_REG_OUT_X_H: c_uint = 0x0029;
pub const DA311_REG_OUT_Y_L: c_uint = 0x002a;
pub const DA311_REG_OUT_Y_H: c_uint = 0x002b;
pub const DA311_REG_OUT_Z_L: c_uint = 0x002c;
pub const DA311_REG_OUT_Z_H: c_uint = 0x002d;
pub const DA311_REG_INT1_CFG: c_uint = 0x0030;
pub const DA311_REG_INT1_SRC: c_uint = 0x0031;
pub const DA311_REG_INT1_THS: c_uint = 0x0032;
pub const DA311_REG_INT1_DURATION: c_uint = 0x0033;
pub const DA311_REG_INT2_CFG: c_uint = 0x0034;
pub const DA311_REG_INT2_SRC: c_uint = 0x0035;
pub const DA311_REG_INT2_THS: c_uint = 0x0036;
pub const DA311_REG_INT2_DURATION: c_uint = 0x0037;
pub const DA311_REG_CLICK_CFG: c_uint = 0x0038;
pub const DA311_REG_CLICK_SRC: c_uint = 0x0039;
pub const DA311_REG_CLICK_THS: c_uint = 0x003a;
pub const DA311_REG_TIME_LIMIT: c_uint = 0x003b;
pub const DA311_REG_TIME_LATENCY: c_uint = 0x003c;
pub const DA311_REG_TIME_WINDOW: c_uint = 0x003d;
// Bank 1 regs
pub const DA311_REG_SOFT_RESET: c_uint = 0x0105;
pub const DA311_REG_OTP_XOFF_L: c_uint = 0x0110;
pub const DA311_REG_OTP_XOFF_H: c_uint = 0x0111;
pub const DA311_REG_OTP_YOFF_L: c_uint = 0x0112;
pub const DA311_REG_OTP_YOFF_H: c_uint = 0x0113;
pub const DA311_REG_OTP_ZOFF_L: c_uint = 0x0114;
pub const DA311_REG_OTP_ZOFF_H: c_uint = 0x0115;
pub const DA311_REG_OTP_XSO: c_uint = 0x0116;
pub const DA311_REG_OTP_YSO: c_uint = 0x0117;
pub const DA311_REG_OTP_ZSO: c_uint = 0x0118;
pub const DA311_REG_OTP_TRIM_OSC: c_uint = 0x011b;
pub const DA311_REG_LPF_ABSOLUTE: c_uint = 0x011c;
pub const DA311_REG_TEMP_OFF1: c_uint = 0x0127;
pub const DA311_REG_TEMP_OFF2: c_uint = 0x0128;
pub const DA311_REG_TEMP_OFF3: c_uint = 0x0129;
pub const DA311_REG_OTP_TRIM_THERM_H: c_uint = 0x011a;
//
// a value of + or -1024 corresponds to + or - 1G
// scale = 9.81 / 1024 = 0.009580078
//
    let mut da311_nscale: static int = 9580078;

    .type = IIO_ACCEL,	\
    .address = reg,	\
    .modified = 1,	\
    .channel2 = IIO_MOD_##axis,	\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    }
    static const struct iio_chan_spec da311_channels[] = {
// | 0x80 comes from the android driver
    DA311_CHANNEL(DA311_REG_OUT_X_L | 0x80, X),
    DA311_CHANNEL(DA311_REG_OUT_Y_L | 0x80, Y),
    DA311_CHANNEL(DA311_REG_OUT_Z_L | 0x80, Z),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da311_data {
    pub client: *mut i2c_client,
}

    static int da311_register_mask_write(struct i2c_client *client, u16 addr,
    u8 mask, u8 data)
    {
    int ret;
    let mut tmp_data: u8 = 0;
    if (addr & 0xff00) {
// Select bank 1
    ret = i2c_smbus_write_byte_data(client, DA311_REG_BANK, 0x01);
    if (ret < 0)
    return ret;
    }
    if (mask != 0xff) {
    ret = i2c_smbus_read_byte_data(client, addr);
    if (ret < 0)
    return ret;
    tmp_data = ret;
    }
    tmp_data &= ~mask;
    tmp_data |= data & mask;
    ret = i2c_smbus_write_byte_data(client, addr & 0xff, tmp_data);
    if (ret < 0)
    return ret;
    if (addr & 0xff00) {
// Back to bank 0
    ret = i2c_smbus_write_byte_data(client, DA311_REG_BANK, 0x00);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
// Init sequence taken from the android driver
#[no_mangle]
unsafe extern "C" fn da311_reset(client: *mut i2c_client) -> c_int {
    static int da311_reset(struct i2c_client *client)
    {
    static const struct {
    u16 addr;
    u8 mask;
    u8 data;
    } init_data[] = {
    { DA311_REG_TEMP_CFG_REG,       0xff,   0x08 },
    { DA311_REG_CTRL_REG5,          0xff,   0x80 },
    { DA311_REG_CTRL_REG4,          0x30,   0x00 },
    { DA311_REG_CTRL_REG1,          0xff,   0x6f },
    { DA311_REG_TEMP_CFG_REG,       0xff,   0x88 },
    { DA311_REG_LDO_REG,            0xff,   0x02 },
    { DA311_REG_OTP_TRIM_OSC,       0xff,   0x27 },
    { DA311_REG_LPF_ABSOLUTE,       0xff,   0x30 },
    { DA311_REG_TEMP_OFF1,          0xff,   0x3f },
    { DA311_REG_TEMP_OFF2,          0xff,   0xff },
    { DA311_REG_TEMP_OFF3,          0xff,   0x0f },
    };
    int i, ret;
// Reset
    ret = da311_register_mask_write(client, DA311_REG_SOFT_RESET,
    0xff, 0xaa);
    if (ret < 0)
    return ret;
    for (i = 0; i < ARRAY_SIZE(init_data); i++) {
    ret = da311_register_mask_write(client,
    init_data[i].addr,
    init_data[i].mask,
    init_data[i].data);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da311_enable(client: *mut i2c_client, enable: bool) -> c_int {
    static int da311_enable(struct i2c_client *client, bool enable)
    {
    let mut data: u8 = enable ? 0x00 : 0x20;
    return da311_register_mask_write(client, DA311_REG_TEMP_CFG_REG,
    0x20, data);
    }
    static int da311_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct da311_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = i2c_smbus_read_word_data(data.client, chan.address);
    if (ret < 0)
    return ret;
//
// Values are 12 bits, stored as 16 bits with the 4
// least significant bits always 0.
//
// val = (short)ret >> 4;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// val2 = da311_nscale;
    return IIO_VAL_INT_PLUS_NANO;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info da311_info = {
    .read_raw	= da311_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn da311_disable(client: *mut c_void) {
    static void da311_disable(void *client)
    {
    da311_enable(client, false);
    }
#[no_mangle]
unsafe extern "C" fn da311_probe(client: *mut i2c_client) -> c_int {
    static int da311_probe(struct i2c_client *client)
    {
    int ret;
    struct iio_dev *indio_dev;
    struct da311_data *data;
    ret = i2c_smbus_read_byte_data(client, DA311_REG_CHIP_ID);
    if (ret != DA311_CHIP_ID)
    return (ret < 0) ? ret : -ENODEV;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    indio_dev.info = &da311_info;
    indio_dev.name = "da311";
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = da311_channels;
    indio_dev.num_channels = ARRAY_SIZE(da311_channels);
    ret = da311_reset(client);
    if (ret < 0)
    return ret;
    ret = da311_enable(client, true);
    if (ret < 0)
    return ret;
    ret = devm_add_action_or_reset(&client.dev, da311_disable, client);
    if (ret)
    return ret;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn da311_suspend(dev: *mut device) -> c_int {
    static int da311_suspend(struct device *dev)
    {
    return da311_enable(to_i2c_client(dev), false);
    }
#[no_mangle]
unsafe extern "C" fn da311_resume(dev: *mut device) -> c_int {
    static int da311_resume(struct device *dev)
    {
    return da311_enable(to_i2c_client(dev), true);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(da311_pm_ops, da311_suspend, da311_resume);
    static const struct i2c_device_id da311_i2c_id[] = {
    { .name = "da311" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, da311_i2c_id);
    static struct i2c_driver da311_driver = {
    .driver = {
    .name = "da311",
    .pm = pm_sleep_ptr(&da311_pm_ops),
    },
    .probe		= da311_probe,
    .id_table	= da311_i2c_id,
    };
    module_i2c_driver(da311_driver);
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_DESCRIPTION("MiraMEMS DA311 3-Axis Accelerometer driver");
    MODULE_LICENSE("GPL v2");
