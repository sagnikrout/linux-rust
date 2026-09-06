//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/jsa1212.c
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
// JSA1212 Ambient Light & Proximity Sensor Driver
//
// Copyright (c) 2014, Intel Corporation.
//
// JSA1212 I2C slave address: 0x44(ADDR tied to GND), 0x45(ADDR tied to VDD)
//
// TODO: Interrupt support, thresholds, range support.
//

// JSA1212 reg address
pub const JSA1212_CONF_REG: c_uint = 0x01;
pub const JSA1212_INT_REG: c_uint = 0x02;
pub const JSA1212_PXS_LT_REG: c_uint = 0x03;
pub const JSA1212_PXS_HT_REG: c_uint = 0x04;
pub const JSA1212_ALS_TH1_REG: c_uint = 0x05;
pub const JSA1212_ALS_TH2_REG: c_uint = 0x06;
pub const JSA1212_ALS_TH3_REG: c_uint = 0x07;
pub const JSA1212_PXS_DATA_REG: c_uint = 0x08;
pub const JSA1212_ALS_DT1_REG: c_uint = 0x09;
pub const JSA1212_ALS_DT2_REG: c_uint = 0x0A;
pub const JSA1212_ALS_RNG_REG: c_uint = 0x0B;
pub const JSA1212_MAX_REG: c_uint = 0x0C;
// JSA1212 reg masks
pub const JSA1212_CONF_MASK: c_uint = 0xFF;
pub const JSA1212_INT_MASK: c_uint = 0xFF;
pub const JSA1212_PXS_LT_MASK: c_uint = 0xFF;
pub const JSA1212_PXS_HT_MASK: c_uint = 0xFF;
pub const JSA1212_ALS_TH1_MASK: c_uint = 0xFF;
pub const JSA1212_ALS_TH2_LT_MASK: c_uint = 0x0F;
pub const JSA1212_ALS_TH2_HT_MASK: c_uint = 0xF0;
pub const JSA1212_ALS_TH3_MASK: c_uint = 0xFF;
pub const JSA1212_PXS_DATA_MASK: c_uint = 0xFF;
pub const JSA1212_ALS_DATA_MASK: c_uint = 0x0FFF;
pub const JSA1212_ALS_DT1_MASK: c_uint = 0xFF;
pub const JSA1212_ALS_DT2_MASK: c_uint = 0x0F;
pub const JSA1212_ALS_RNG_MASK: c_uint = 0x07;
// JSA1212 CONF REG bits
pub const JSA1212_CONF_PXS_MASK: c_uint = 0x80;
pub const JSA1212_CONF_PXS_ENABLE: c_uint = 0x80;
pub const JSA1212_CONF_PXS_DISABLE: c_uint = 0x00;
pub const JSA1212_CONF_ALS_MASK: c_uint = 0x04;
pub const JSA1212_CONF_ALS_ENABLE: c_uint = 0x04;
pub const JSA1212_CONF_ALS_DISABLE: c_uint = 0x00;
pub const JSA1212_CONF_IRDR_MASK: c_uint = 0x08;
// Proxmity sensing IRDR current sink settings
pub const JSA1212_CONF_IRDR_200MA: c_uint = 0x08;
pub const JSA1212_CONF_IRDR_100MA: c_uint = 0x00;
pub const JSA1212_CONF_PXS_SLP_MASK: c_uint = 0x70;
pub const JSA1212_CONF_PXS_SLP_0MS: c_uint = 0x70;
pub const JSA1212_CONF_PXS_SLP_12MS: c_uint = 0x60;
pub const JSA1212_CONF_PXS_SLP_50MS: c_uint = 0x50;
pub const JSA1212_CONF_PXS_SLP_75MS: c_uint = 0x40;
pub const JSA1212_CONF_PXS_SLP_100MS: c_uint = 0x30;
pub const JSA1212_CONF_PXS_SLP_200MS: c_uint = 0x20;
pub const JSA1212_CONF_PXS_SLP_400MS: c_uint = 0x10;
pub const JSA1212_CONF_PXS_SLP_800MS: c_uint = 0x00;
// JSA1212 INT REG bits
pub const JSA1212_INT_CTRL_MASK: c_uint = 0x01;
pub const JSA1212_INT_CTRL_EITHER: c_uint = 0x00;
pub const JSA1212_INT_CTRL_BOTH: c_uint = 0x01;
pub const JSA1212_INT_ALS_PRST_MASK: c_uint = 0x06;
pub const JSA1212_INT_ALS_PRST_1CONV: c_uint = 0x00;
pub const JSA1212_INT_ALS_PRST_4CONV: c_uint = 0x02;
pub const JSA1212_INT_ALS_PRST_8CONV: c_uint = 0x04;
pub const JSA1212_INT_ALS_PRST_16CONV: c_uint = 0x06;
pub const JSA1212_INT_ALS_FLAG_MASK: c_uint = 0x08;
pub const JSA1212_INT_ALS_FLAG_CLR: c_uint = 0x00;
pub const JSA1212_INT_PXS_PRST_MASK: c_uint = 0x60;
pub const JSA1212_INT_PXS_PRST_1CONV: c_uint = 0x00;
pub const JSA1212_INT_PXS_PRST_4CONV: c_uint = 0x20;
pub const JSA1212_INT_PXS_PRST_8CONV: c_uint = 0x40;
pub const JSA1212_INT_PXS_PRST_16CONV: c_uint = 0x60;
pub const JSA1212_INT_PXS_FLAG_MASK: c_uint = 0x80;
pub const JSA1212_INT_PXS_FLAG_CLR: c_uint = 0x00;
// JSA1212 ALS RNG REG bits
pub const JSA1212_ALS_RNG_0_2048: c_uint = 0x00;
pub const JSA1212_ALS_RNG_0_1024: c_uint = 0x01;
pub const JSA1212_ALS_RNG_0_512: c_uint = 0x02;
pub const JSA1212_ALS_RNG_0_256: c_uint = 0x03;
pub const JSA1212_ALS_RNG_0_128: c_uint = 0x04;
// JSA1212 INT threshold range
pub const JSA1212_ALS_TH_MIN: c_uint = 0x0000;
pub const JSA1212_ALS_TH_MAX: c_uint = 0x0FFF;
pub const JSA1212_PXS_TH_MIN: c_uint = 0x00;
pub const JSA1212_PXS_TH_MAX: c_uint = 0xFF;
pub const JSA1212_ALS_DELAY_MS: c_int = 200;
pub const JSA1212_PXS_DELAY_MS: c_int = 100;

    enum jsa1212_op_mode {
    JSA1212_OPMODE_ALS_EN,
    JSA1212_OPMODE_PXS_EN,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jsa1212_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub als_rng_idx: u8,
    pub /: *mut *mut bool als_en; / ALS enable status,
    pub /: *mut *mut bool pxs_en; / proximity enable status,
    pub regmap: *mut regmap,
}

// ALS range idx to val mapping
    static const int jsa1212_als_range_val[] = {2048, 1024, 512, 256, 128,
    128, 128, 128};
// Enables or disables ALS function based on status
#[no_mangle]
unsafe extern "C" fn jsa1212_als_enable(data: *mut jsa1212_data, status: u8) -> c_int {
    static int jsa1212_als_enable(struct jsa1212_data *data, u8 status)
    {
    int ret;
    ret = regmap_update_bits(data.regmap, JSA1212_CONF_REG,
    JSA1212_CONF_ALS_MASK,
    status);
    if (ret < 0)
    return ret;
    data.als_en = !!status;
    return 0;
    }
// Enables or disables PXS function based on status
#[no_mangle]
unsafe extern "C" fn jsa1212_pxs_enable(data: *mut jsa1212_data, status: u8) -> c_int {
    static int jsa1212_pxs_enable(struct jsa1212_data *data, u8 status)
    {
    int ret;
    ret = regmap_update_bits(data.regmap, JSA1212_CONF_REG,
    JSA1212_CONF_PXS_MASK,
    status);
    if (ret < 0)
    return ret;
    data.pxs_en = !!status;
    return 0;
    }
    static int jsa1212_read_als_data(struct jsa1212_data *data,
    unsigned int *val)
    {
    int ret;
    __le16 als_data;
    ret = jsa1212_als_enable(data, JSA1212_CONF_ALS_ENABLE);
    if (ret < 0)
    return ret;
// Delay for data output
    msleep(JSA1212_ALS_DELAY_MS);
// Read 12 bit data
    ret = regmap_bulk_read(data.regmap, JSA1212_ALS_DT1_REG, &als_data, 2);
    if (ret < 0) {
    dev_err(&data.client.dev, "als data read err\n");
    goto als_data_read_err;
    }
// val = le16_to_cpu(als_data);
    als_data_read_err:
    return jsa1212_als_enable(data, JSA1212_CONF_ALS_DISABLE);
    }
    static int jsa1212_read_pxs_data(struct jsa1212_data *data,
    unsigned int *val)
    {
    int ret;
    unsigned int pxs_data;
    ret = jsa1212_pxs_enable(data, JSA1212_CONF_PXS_ENABLE);
    if (ret < 0)
    return ret;
// Delay for data output
    msleep(JSA1212_PXS_DELAY_MS);
// Read out all data
    ret = regmap_read(data.regmap, JSA1212_PXS_DATA_REG, &pxs_data);
    if (ret < 0) {
    dev_err(&data.client.dev, "pxs data read err\n");
    goto pxs_data_read_err;
    }
// val = pxs_data & JSA1212_PXS_DATA_MASK;
    pxs_data_read_err:
    return jsa1212_pxs_enable(data, JSA1212_CONF_PXS_DISABLE);
    }
    static int jsa1212_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    int ret;
    struct jsa1212_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&data.lock);
    switch (chan.type) {
    case IIO_LIGHT:
    ret = jsa1212_read_als_data(data, val);
    break;
    case IIO_PROXIMITY:
    ret = jsa1212_read_pxs_data(data, val);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    mutex_unlock(&data.lock);
    return ret < 0 ? ret : IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_LIGHT:
// val = jsa1212_als_range_val[data->als_rng_idx];
// val2 = BIT(12); /* Max 12 bit value
    return IIO_VAL_FRACTIONAL;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return -EINVAL;
    }
    static const struct iio_chan_spec jsa1212_channels[] = {
    {
    .type = IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    },
    {
    .type = IIO_PROXIMITY,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    }
    };
    static const struct iio_info jsa1212_info = {
    .read_raw		= &jsa1212_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn jsa1212_chip_init(data: *mut jsa1212_data) -> c_int {
    static int jsa1212_chip_init(struct jsa1212_data *data)
    {
    int ret;
    ret = regmap_write(data.regmap, JSA1212_CONF_REG,
    (JSA1212_CONF_PXS_SLP_50MS |
    JSA1212_CONF_IRDR_200MA));
    if (ret < 0)
    return ret;
    ret = regmap_write(data.regmap, JSA1212_INT_REG,
    JSA1212_INT_ALS_PRST_4CONV);
    if (ret < 0)
    return ret;
    data.als_rng_idx = JSA1212_ALS_RNG_0_2048;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jsa1212_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool jsa1212_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case JSA1212_PXS_DATA_REG:
    case JSA1212_ALS_DT1_REG:
    case JSA1212_ALS_DT2_REG:
    case JSA1212_INT_REG:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config jsa1212_regmap_config = {
    .name = "jsa1212_regmap",
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = JSA1212_MAX_REG,
    .cache_type = REGCACHE_RBTREE,
    .volatile_reg = jsa1212_is_volatile_reg,
    };
#[no_mangle]
unsafe extern "C" fn jsa1212_probe(client: *mut i2c_client) -> c_int {
    static int jsa1212_probe(struct i2c_client *client)
    {
    struct jsa1212_data *data;
    struct iio_dev *indio_dev;
    struct regmap *regmap;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    regmap = devm_regmap_init_i2c(client, &jsa1212_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "Regmap initialization failed.\n");
    return PTR_ERR(regmap);
    }
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    data.regmap = regmap;
    mutex_init(&data.lock);
    ret = jsa1212_chip_init(data);
    if (ret < 0)
    return ret;
    indio_dev.channels = jsa1212_channels;
    indio_dev.num_channels = ARRAY_SIZE(jsa1212_channels);
    indio_dev.name = JSA1212_DRIVER_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &jsa1212_info;
    ret = iio_device_register(indio_dev);
    if (ret < 0)
    dev_err(&client.dev, "%s: register device failed\n", __func__);
    return ret;
    }
// power off the device
#[no_mangle]
unsafe extern "C" fn jsa1212_power_off(data: *mut jsa1212_data) -> c_int {
    static int jsa1212_power_off(struct jsa1212_data *data)
    {
    int ret;
    mutex_lock(&data.lock);
    ret = regmap_update_bits(data.regmap, JSA1212_CONF_REG,
    JSA1212_CONF_ALS_MASK |
    JSA1212_CONF_PXS_MASK,
    JSA1212_CONF_ALS_DISABLE |
    JSA1212_CONF_PXS_DISABLE);
    if (ret < 0)
    dev_err(&data.client.dev, "power off cmd failed\n");
    mutex_unlock(&data.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jsa1212_remove(client: *mut i2c_client) {
    static void jsa1212_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct jsa1212_data *data = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    jsa1212_power_off(data);
    }
#[no_mangle]
unsafe extern "C" fn jsa1212_suspend(dev: *mut device) -> c_int {
    static int jsa1212_suspend(struct device *dev)
    {
    struct jsa1212_data *data;
    data = iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    return jsa1212_power_off(data);
    }
#[no_mangle]
unsafe extern "C" fn jsa1212_resume(dev: *mut device) -> c_int {
    static int jsa1212_resume(struct device *dev)
    {
    let mut ret: c_int = 0;
    struct jsa1212_data *data;
    data = iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    mutex_lock(&data.lock);
    if (data.als_en) {
    ret = jsa1212_als_enable(data, JSA1212_CONF_ALS_ENABLE);
    if (ret < 0) {
    dev_err(dev, "als resume failed\n");
    goto unlock_and_ret;
    }
    }
    if (data.pxs_en) {
    ret = jsa1212_pxs_enable(data, JSA1212_CONF_PXS_ENABLE);
    if (ret < 0)
    dev_err(dev, "pxs resume failed\n");
    }
    unlock_and_ret:
    mutex_unlock(&data.lock);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(jsa1212_pm_ops, jsa1212_suspend,
    jsa1212_resume);
    static const struct acpi_device_id jsa1212_acpi_match[] = {
    {"JSA1212", 0},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, jsa1212_acpi_match);
    static const struct i2c_device_id jsa1212_id[] = {
    { .name = JSA1212_DRIVER_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, jsa1212_id);
    static struct i2c_driver jsa1212_driver = {
    .driver = {
    .name	= JSA1212_DRIVER_NAME,
    .pm	= pm_sleep_ptr(&jsa1212_pm_ops),
    .acpi_match_table = jsa1212_acpi_match,
    },
    .probe		= jsa1212_probe,
    .remove		= jsa1212_remove,
    .id_table	= jsa1212_id,
    };
    module_i2c_driver(jsa1212_driver);
    MODULE_AUTHOR("Sathya Kuppuswamy <sathyanarayanan.kuppuswamy@linux.intel.com>");
    MODULE_DESCRIPTION("JSA1212 proximity/ambient light sensor driver");
    MODULE_LICENSE("GPL v2");
