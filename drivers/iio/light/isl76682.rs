//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/isl76682.c
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
// IIO driver for the light sensor ISL76682.
// ISL76682 is Ambient Light Sensor
//
// Copyright (c) 2023 Marek Vasut <marex@denx.de>
//

pub const ISL76682_REG_COMMAND: c_uint = 0x00;

pub const ISL76682_COMMAND_RANGE_LUX_1K: c_uint = 0x0;
pub const ISL76682_COMMAND_RANGE_LUX_4K: c_uint = 0x1;
pub const ISL76682_COMMAND_RANGE_LUX_16K: c_uint = 0x2;
pub const ISL76682_COMMAND_RANGE_LUX_64K: c_uint = 0x3;

pub const ISL76682_REG_ALSIR_L: c_uint = 0x01;
pub const ISL76682_REG_ALSIR_U: c_uint = 0x02;

pub const ISL76682_CONV_TIME_MS: c_int = 100;
pub const ISL76682_INT_TIME_US: c_int = 90000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl76682_chip {
//
// Lock to synchronize access to device command register
// and the content of range variable below.
//
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub range: u8,
    pub command: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl76682_range {
    pub range: u8,
    pub als: u32,
    pub ir: u32,
}

    static const struct isl76682_range isl76682_range_table[] = {
    { ISL76682_COMMAND_RANGE_LUX_1K, 15000, 10500 },
    { ISL76682_COMMAND_RANGE_LUX_4K, 60000, 42000 },
    { ISL76682_COMMAND_RANGE_LUX_16K, 240000, 168000 },
    { ISL76682_COMMAND_RANGE_LUX_64K, 960000, 673000 }
    };
#[no_mangle]
unsafe extern "C" fn isl76682_get(chip: *mut isl76682_chip, mode_ir: bool, data: *mut c_int) -> c_int {
    static int isl76682_get(struct isl76682_chip *chip, bool mode_ir, int *data)
    {
    u8 command;
    int ret;
    command = ISL76682_COMMAND_EN | ISL76682_COMMAND_MODE_CONTINUOUS |
    chip.range;
    if (mode_ir)
    command |= ISL76682_COMMAND_LIGHT_IR;
    if (command != chip.command) {
    ret = regmap_write(chip.regmap, ISL76682_REG_COMMAND, command);
    if (ret)
    return ret;
// Need to wait for conversion time if ALS/IR mode enabled
    msleep(ISL76682_CONV_TIME_MS);
    chip.command = command;
    }
    ret = regmap_bulk_read(chip.regmap, ISL76682_REG_ALSIR_L, data, 2);
// data &= ISL76682_ADC_MAX;
    return ret;
    }
    static int isl76682_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct isl76682_chip *chip = iio_priv(indio_dev);
    int i;
    if (mask != IIO_CHAN_INFO_SCALE)
    return -EINVAL;
    if (val != 0)
    return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(isl76682_range_table); i++) {
    if (chan.type == IIO_LIGHT && val2 != isl76682_range_table[i].als)
    continue;
    if (chan.type == IIO_INTENSITY && val2 != isl76682_range_table[i].ir)
    continue;
    scoped_guard(mutex, &chip.lock)
    chip.range = isl76682_range_table[i].range;
    return 0;
    }
    return -EINVAL;
    }
    static int isl76682_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct isl76682_chip *chip = iio_priv(indio_dev);
    int ret;
    int i;
    guard(mutex)(&chip.lock);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    switch (chan.type) {
    case IIO_LIGHT:
    ret = isl76682_get(chip, false, val);
    return (ret < 0) ? ret : IIO_VAL_INT;
    case IIO_INTENSITY:
    ret = isl76682_get(chip, true, val);
    return (ret < 0) ? ret : IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SCALE:
    for (i = 0; i < ARRAY_SIZE(isl76682_range_table); i++) {
    if (chip.range != isl76682_range_table[i].range)
    continue;
// val = 0;
    switch (chan.type) {
    case IIO_LIGHT:
// val2 = isl76682_range_table[i].als;
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_INTENSITY:
// val2 = isl76682_range_table[i].ir;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    return -EINVAL;
    case IIO_CHAN_INFO_INT_TIME:
// val = 0;
// val2 = ISL76682_INT_TIME_US;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static int illuminance_scale_available[] = {
    0, 15000,
    0, 60000,
    0, 240000,
    0, 960000,
    };
    static int intensity_scale_available[] = {
    0, 10500,
    0, 42000,
    0, 168000,
    0, 673000,
    };
    static int isl76682_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type,
    int *length, long mask)
    {
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_LIGHT:
// vals = illuminance_scale_available;
// length = ARRAY_SIZE(illuminance_scale_available);
// type = IIO_VAL_INT_PLUS_MICRO;
    return IIO_AVAIL_LIST;
    case IIO_INTENSITY:
// vals = intensity_scale_available;
// length = ARRAY_SIZE(intensity_scale_available);
// type = IIO_VAL_INT_PLUS_MICRO;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static const struct iio_chan_spec isl76682_channels[] = {
    {
    .type = IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    }, {
    .type = IIO_INTENSITY,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    }
    };
    static const struct iio_info isl76682_info = {
    .read_avail	= isl76682_read_avail,
    .read_raw	= isl76682_read_raw,
    .write_raw	= isl76682_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn isl76682_clear_configure_reg(chip: *mut isl76682_chip) -> c_int {
    static int isl76682_clear_configure_reg(struct isl76682_chip *chip)
    {
    struct device *dev = regmap_get_device(chip.regmap);
    int ret;
    ret = regmap_write(chip.regmap, ISL76682_REG_COMMAND, 0x0);
    if (ret < 0)
    dev_err(dev, "Error %d clearing the CONFIGURE register\n", ret);
//
// In the success case, the command register was zeroed out.
//
// In the error case, we do not know in which state the command
// register is, so we assume it is zeroed out, so that it would
// be reprogrammed at the next data read out, and at that time
// we hope it would be reprogrammed successfully. That is very
// much a best effort approach.
//
    chip.command = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn isl76682_reset_action(chip: *mut c_void) {
    static void isl76682_reset_action(void *chip)
    {
    isl76682_clear_configure_reg(chip);
    }
#[no_mangle]
unsafe extern "C" fn isl76682_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool isl76682_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ISL76682_REG_ALSIR_L:
    case ISL76682_REG_ALSIR_U:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config isl76682_regmap_config = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .volatile_reg		= isl76682_is_volatile_reg,
    .max_register		= ISL76682_NUM_REGS - 1,
    .num_reg_defaults_raw	= ISL76682_NUM_REGS,
    .cache_type		= REGCACHE_FLAT,
    };
#[no_mangle]
unsafe extern "C" fn isl76682_probe(client: *mut i2c_client) -> c_int {
    static int isl76682_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct isl76682_chip *chip;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*chip));
    if (!indio_dev)
    return -ENOMEM;
    chip = iio_priv(indio_dev);
    mutex_init(&chip.lock);
    chip.regmap = devm_regmap_init_i2c(client, &isl76682_regmap_config);
    ret = PTR_ERR_OR_ZERO(chip.regmap);
    if (ret)
    return dev_err_probe(dev, ret, "Error initializing regmap\n");
    chip.range = ISL76682_COMMAND_RANGE_LUX_1K;
    ret = isl76682_clear_configure_reg(chip);
    if (ret < 0)
    return ret;
    ret = devm_add_action_or_reset(dev, isl76682_reset_action, chip);
    if (ret)
    return ret;
    indio_dev.info = &isl76682_info;
    indio_dev.channels = isl76682_channels;
    indio_dev.num_channels = ARRAY_SIZE(isl76682_channels);
    indio_dev.name = "isl76682";
    indio_dev.modes = INDIO_DIRECT_MODE;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct i2c_device_id isl76682_id[] = {
    { .name = "isl76682" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, isl76682_id);
    static const struct of_device_id isl76682_of_match[] = {
    { .compatible = "isil,isl76682" },
    { }
    };
    MODULE_DEVICE_TABLE(of, isl76682_of_match);
    static struct i2c_driver isl76682_driver = {
    .driver  = {
    .name		= "isl76682",
    .of_match_table	= isl76682_of_match,
    },
    .probe		= isl76682_probe,
    .id_table	= isl76682_id,
    };
    module_i2c_driver(isl76682_driver);
    MODULE_DESCRIPTION("ISL76682 Ambient Light Sensor driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Marek Vasut <marex@denx.de>");
