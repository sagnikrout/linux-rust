//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/al3000a.c
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

pub const AL3000A_REG_SYSTEM: c_uint = 0x00;
pub const AL3000A_REG_DATA: c_uint = 0x05;
pub const AL3000A_CONFIG_ENABLE: c_uint = 0x00;
pub const AL3000A_CONFIG_DISABLE: c_uint = 0x0b;
pub const AL3000A_CONFIG_RESET: c_uint = 0x0f;

//
// These are pre-calculated lux values based on possible output of sensor
// (range 0x00 - 0x3F)
//
    static const u32 lux_table[] = {
    1, 1, 1, 2, 2, 2, 3, 4,					/* 0 - 7 */
    4, 5, 6, 7, 9, 11, 13, 16,				/* 8 - 15 */
    19, 22, 27, 32, 39, 46, 56, 67,				/* 16 - 23 */
    80, 96, 116, 139, 167, 200, 240, 289,			/* 24 - 31 */
    347, 416, 499, 600, 720, 864, 1037, 1245,		/* 32 - 39 */
    1495, 1795, 2155, 2587, 3105, 3728, 4475, 5373,		/* 40 - 47 */
    6450, 7743, 9296, 11160, 13397, 16084, 19309, 23180,	/* 48 - 55 */
    27828, 33408, 40107, 48148, 57803, 69393, 83306, 100000 /* 56 - 63 */
    };
    static const struct regmap_config al3000a_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = AL3000A_REG_DATA,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct al3000a_data {
    pub regmap: *mut regmap,
    pub vdd_supply: *mut regulator,
}

    static const struct iio_chan_spec al3000a_channels[] = {
    {
    .type = IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    },
    };
#[no_mangle]
unsafe extern "C" fn al3000a_set_pwr_on(data: *mut al3000a_data) -> c_int {
    static int al3000a_set_pwr_on(struct al3000a_data *data)
    {
    struct device *dev = regmap_get_device(data.regmap);
    int ret;
    ret = regulator_enable(data.vdd_supply);
    if (ret) {
    dev_err(dev, "failed to enable vdd power supply\n");
    return ret;
    }
    return regmap_write(data.regmap, AL3000A_REG_SYSTEM, AL3000A_CONFIG_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn al3000a_set_pwr_off(_data: *mut c_void) {
    static void al3000a_set_pwr_off(void *_data)
    {
    struct al3000a_data *data = _data;
    struct device *dev = regmap_get_device(data.regmap);
    int ret;
    ret = regmap_write(data.regmap, AL3000A_REG_SYSTEM, AL3000A_CONFIG_DISABLE);
    if (ret)
    dev_err(dev, "failed to write system register\n");
    ret = regulator_disable(data.vdd_supply);
    if (ret)
    dev_err(dev, "failed to disable vdd power supply\n");
    }
#[no_mangle]
unsafe extern "C" fn al3000a_init(data: *mut al3000a_data) -> c_int {
    static int al3000a_init(struct al3000a_data *data)
    {
    struct device *dev = regmap_get_device(data.regmap);
    int ret;
    ret = al3000a_set_pwr_on(data);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, al3000a_set_pwr_off, data);
    if (ret)
    return ret;
    ret = regmap_write(data.regmap, AL3000A_REG_SYSTEM, AL3000A_CONFIG_RESET);
    if (ret)
    return ret;
    return regmap_write(data.regmap, AL3000A_REG_SYSTEM, AL3000A_CONFIG_ENABLE);
    }
    static int al3000a_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct al3000a_data *data = iio_priv(indio_dev);
    int ret, gain;
    switch (mask) {
    case IIO_CHAN_INFO_PROCESSED:
    ret = regmap_read(data.regmap, AL3000A_REG_DATA, &gain);
    if (ret)
    return ret;
// val = lux_table[gain & AL3000A_GAIN_MASK];
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info al3000a_info = {
    .read_raw = al3000a_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn al3000a_probe(client: *mut i2c_client) -> c_int {
    static int al3000a_probe(struct i2c_client *client)
    {
    struct al3000a_data *data;
    struct device *dev = &client.dev;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.regmap = devm_regmap_init_i2c(client, &al3000a_regmap_config);
    if (IS_ERR(data.regmap))
    return dev_err_probe(dev, PTR_ERR(data.regmap),
    "cannot allocate regmap\n");
    data.vdd_supply = devm_regulator_get(dev, "vdd");
    if (IS_ERR(data.vdd_supply))
    return dev_err_probe(dev, PTR_ERR(data.vdd_supply),
    "failed to get vdd regulator\n");
    indio_dev.info = &al3000a_info;
    indio_dev.name = "al3000a";
    indio_dev.channels = al3000a_channels;
    indio_dev.num_channels = ARRAY_SIZE(al3000a_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = al3000a_init(data);
    if (ret)
    return dev_err_probe(dev, ret, "failed to init ALS\n");
    return devm_iio_device_register(dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn al3000a_suspend(dev: *mut device) -> c_int {
    static int al3000a_suspend(struct device *dev)
    {
    struct al3000a_data *data = iio_priv(dev_get_drvdata(dev));
    al3000a_set_pwr_off(data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn al3000a_resume(dev: *mut device) -> c_int {
    static int al3000a_resume(struct device *dev)
    {
    struct al3000a_data *data = iio_priv(dev_get_drvdata(dev));
    return al3000a_set_pwr_on(data);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(al3000a_pm_ops, al3000a_suspend, al3000a_resume);
    static const struct i2c_device_id al3000a_id[] = {
    { .name = "al3000a" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, al3000a_id);
    static const struct of_device_id al3000a_of_match[] = {
    { .compatible = "dynaimage,al3000a" },
    { }
    };
    MODULE_DEVICE_TABLE(of, al3000a_of_match);
    static struct i2c_driver al3000a_driver = {
    .driver = {
    .name = "al3000a",
    .of_match_table = al3000a_of_match,
    .pm = pm_sleep_ptr(&al3000a_pm_ops),
    },
    .probe = al3000a_probe,
    .id_table = al3000a_id,
    };
    module_i2c_driver(al3000a_driver);
    MODULE_AUTHOR("Svyatolsav Ryhel <clamor95@gmail.com>");
    MODULE_DESCRIPTION("al3000a Ambient Light Sensor driver");
    MODULE_LICENSE("GPL");
