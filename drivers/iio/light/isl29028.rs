//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/isl29028.c
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
// IIO driver for the light sensor ISL29028.
// ISL29028 is Concurrent Ambient Light and Proximity Sensor
//
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
// Copyright (c) 2016-2017 Brian Masney <masneyb@onstation.org>
//
// Datasheets:
// - http://www.intersil.com/content/dam/Intersil/documents/isl2/isl29028.pdf
// - http://www.intersil.com/content/dam/Intersil/documents/isl2/isl29030.pdf
//

pub const ISL29028_CONV_TIME_MS: c_int = 100;
pub const ISL29028_REG_CONFIGURE: c_uint = 0x01;
pub const ISL29028_CONF_ALS_IR_MODE_ALS: c_int = 0;

pub const ISL29028_CONF_ALS_RANGE_LOW_LUX: c_int = 0;

pub const ISL29028_CONF_ALS_DIS: c_int = 0;

pub const ISL29028_CONF_PROX_SLP_SH: c_int = 4;

pub const ISL29028_REG_INTERRUPT: c_uint = 0x02;
pub const ISL29028_REG_PROX_DATA: c_uint = 0x08;
pub const ISL29028_REG_ALSIR_L: c_uint = 0x09;
pub const ISL29028_REG_ALSIR_U: c_uint = 0x0A;
pub const ISL29028_REG_TEST1_MODE: c_uint = 0x0E;
pub const ISL29028_REG_TEST2_MODE: c_uint = 0x0F;

pub const ISL29028_POWER_OFF_DELAY_MS: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl29028_prox_data {
    pub sampling_int: c_int,
    pub sampling_fract: c_int,
    pub sleep_time: c_int,
}

    static const struct isl29028_prox_data isl29028_prox_data[] = {
    {   1, 250000, 800 },
    {   2, 500000, 400 },
    {   5,      0, 200 },
    {  10,      0, 100 },
    {  13, 300000,  75 },
    {  20,      0,  50 },
    {  80,      0,  13 }, /*
// Note: Data sheet lists 12.5 ms sleep time.
// Round up a half millisecond for msleep().
//
    { 100,  0,   0 }
    };
    enum isl29028_als_ir_mode {
    ISL29028_MODE_NONE = 0,
    ISL29028_MODE_ALS,
    ISL29028_MODE_IR,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl29028_chip {
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub prox_sampling_int: c_int,
    pub prox_sampling_frac: c_int,
    pub enable_prox: bool,
    pub lux_scale: c_int,
    pub als_ir_mode: enum isl29028_als_ir_mode,
}

#[no_mangle]
unsafe extern "C" fn isl29028_find_prox_sleep_index(sampling_int: c_int, sampling_fract: c_int) -> c_int {
    static int isl29028_find_prox_sleep_index(int sampling_int, int sampling_fract)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(isl29028_prox_data); ++i) {
    if (isl29028_prox_data[i].sampling_int == sampling_int &&
    isl29028_prox_data[i].sampling_fract == sampling_fract)
    return i;
    }
    return -EINVAL;
    }
    static int isl29028_set_proxim_sampling(struct isl29028_chip *chip,
    int sampling_int, int sampling_fract)
    {
    struct device *dev = regmap_get_device(chip.regmap);
    int sleep_index, ret;
    sleep_index = isl29028_find_prox_sleep_index(sampling_int,
    sampling_fract);
    if (sleep_index < 0)
    return sleep_index;
    ret = regmap_update_bits(chip.regmap, ISL29028_REG_CONFIGURE,
    ISL29028_CONF_PROX_SLP_MASK,
    sleep_index << ISL29028_CONF_PROX_SLP_SH);
    if (ret < 0) {
    dev_err(dev, "%s(): Error %d setting the proximity sampling\n",
    __func__, ret);
    return ret;
    }
    chip.prox_sampling_int = sampling_int;
    chip.prox_sampling_frac = sampling_fract;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn isl29028_enable_proximity(chip: *mut isl29028_chip) -> c_int {
    static int isl29028_enable_proximity(struct isl29028_chip *chip)
    {
    int prox_index, ret;
    ret = isl29028_set_proxim_sampling(chip, chip.prox_sampling_int,
    chip.prox_sampling_frac);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(chip.regmap, ISL29028_REG_CONFIGURE,
    ISL29028_CONF_PROX_EN_MASK,
    ISL29028_CONF_PROX_EN);
    if (ret < 0)
    return ret;
// Wait for conversion to be complete for first sample
    prox_index = isl29028_find_prox_sleep_index(chip.prox_sampling_int,
    chip.prox_sampling_frac);
    if (prox_index < 0)
    return prox_index;
    msleep(isl29028_prox_data[prox_index].sleep_time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isl29028_set_als_scale(chip: *mut isl29028_chip, lux_scale: c_int) -> c_int {
    static int isl29028_set_als_scale(struct isl29028_chip *chip, int lux_scale)
    {
    struct device *dev = regmap_get_device(chip.regmap);
    int val = (lux_scale == 2000) ? ISL29028_CONF_ALS_RANGE_HIGH_LUX :
    ISL29028_CONF_ALS_RANGE_LOW_LUX;
    int ret;
    ret = regmap_update_bits(chip.regmap, ISL29028_REG_CONFIGURE,
    ISL29028_CONF_ALS_RANGE_MASK, val);
    if (ret < 0) {
    dev_err(dev, "%s(): Error %d setting the ALS scale\n", __func__,
    ret);
    return ret;
    }
    chip.lux_scale = lux_scale;
    return ret;
    }
    static int isl29028_set_als_ir_mode(struct isl29028_chip *chip,
    enum isl29028_als_ir_mode mode)
    {
    int ret;
    if (chip.als_ir_mode == mode)
    return 0;
    ret = isl29028_set_als_scale(chip, chip.lux_scale);
    if (ret < 0)
    return ret;
    switch (mode) {
    case ISL29028_MODE_ALS:
    ret = regmap_update_bits(chip.regmap, ISL29028_REG_CONFIGURE,
    ISL29028_CONF_ALS_IR_MODE_MASK,
    ISL29028_CONF_ALS_IR_MODE_ALS);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(chip.regmap, ISL29028_REG_CONFIGURE,
    ISL29028_CONF_ALS_RANGE_MASK,
    ISL29028_CONF_ALS_RANGE_HIGH_LUX);
    break;
    case ISL29028_MODE_IR:
    ret = regmap_update_bits(chip.regmap, ISL29028_REG_CONFIGURE,
    ISL29028_CONF_ALS_IR_MODE_MASK,
    ISL29028_CONF_ALS_IR_MODE_IR);
    break;
    case ISL29028_MODE_NONE:
    return regmap_update_bits(chip.regmap, ISL29028_REG_CONFIGURE,
    ISL29028_CONF_ALS_EN_MASK,
    ISL29028_CONF_ALS_DIS);
    }
    if (ret < 0)
    return ret;
// Enable the ALS/IR
    ret = regmap_update_bits(chip.regmap, ISL29028_REG_CONFIGURE,
    ISL29028_CONF_ALS_EN_MASK,
    ISL29028_CONF_ALS_EN);
    if (ret < 0)
    return ret;
// Need to wait for conversion time if ALS/IR mode enabled
    msleep(ISL29028_CONV_TIME_MS);
    chip.als_ir_mode = mode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isl29028_read_als_ir(chip: *mut isl29028_chip, als_ir: *mut c_int) -> c_int {
    static int isl29028_read_als_ir(struct isl29028_chip *chip, int *als_ir)
    {
    struct device *dev = regmap_get_device(chip.regmap);
    unsigned int lsb;
    unsigned int msb;
    int ret;
    ret = regmap_read(chip.regmap, ISL29028_REG_ALSIR_L, &lsb);
    if (ret < 0) {
    dev_err(dev,
    "%s(): Error %d reading register ALSIR_L\n",
    __func__, ret);
    return ret;
    }
    ret = regmap_read(chip.regmap, ISL29028_REG_ALSIR_U, &msb);
    if (ret < 0) {
    dev_err(dev,
    "%s(): Error %d reading register ALSIR_U\n",
    __func__, ret);
    return ret;
    }
// als_ir = ((msb & 0xF) << 8) | (lsb & 0xFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isl29028_read_proxim(chip: *mut isl29028_chip, prox: *mut c_int) -> c_int {
    static int isl29028_read_proxim(struct isl29028_chip *chip, int *prox)
    {
    struct device *dev = regmap_get_device(chip.regmap);
    unsigned int data;
    int ret;
    if (!chip.enable_prox) {
    ret = isl29028_enable_proximity(chip);
    if (ret < 0)
    return ret;
    chip.enable_prox = true;
    }
    ret = regmap_read(chip.regmap, ISL29028_REG_PROX_DATA, &data);
    if (ret < 0) {
    dev_err(dev, "%s(): Error %d reading register PROX_DATA\n",
    __func__, ret);
    return ret;
    }
// prox = data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isl29028_als_get(chip: *mut isl29028_chip, als_data: *mut c_int) -> c_int {
    static int isl29028_als_get(struct isl29028_chip *chip, int *als_data)
    {
    struct device *dev = regmap_get_device(chip.regmap);
    int ret;
    int als_ir_data;
    ret = isl29028_set_als_ir_mode(chip, ISL29028_MODE_ALS);
    if (ret < 0) {
    dev_err(dev, "%s(): Error %d enabling ALS mode\n", __func__,
    ret);
    return ret;
    }
    ret = isl29028_read_als_ir(chip, &als_ir_data);
    if (ret < 0)
    return ret;
//
// convert als data count to lux.
// if lux_scale = 125,  lux = count * 0.031
// if lux_scale = 2000, lux = count * 0.49
//
    if (chip.lux_scale == 125)
    als_ir_data = (als_ir_data * 31) / 1000;
    else
    als_ir_data = (als_ir_data * 49) / 100;
// als_data = als_ir_data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isl29028_ir_get(chip: *mut isl29028_chip, ir_data: *mut c_int) -> c_int {
    static int isl29028_ir_get(struct isl29028_chip *chip, int *ir_data)
    {
    struct device *dev = regmap_get_device(chip.regmap);
    int ret;
    ret = isl29028_set_als_ir_mode(chip, ISL29028_MODE_IR);
    if (ret < 0) {
    dev_err(dev, "%s(): Error %d enabling IR mode\n", __func__,
    ret);
    return ret;
    }
    return isl29028_read_als_ir(chip, ir_data);
    }
// Channel IO
    static int isl29028_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct isl29028_chip *chip = iio_priv(indio_dev);
    struct device *dev = regmap_get_device(chip.regmap);
    int ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    return ret;
    mutex_lock(&chip.lock);
    ret = -EINVAL;
    switch (chan.type) {
    case IIO_PROXIMITY:
    if (mask != IIO_CHAN_INFO_SAMP_FREQ) {
    dev_err(dev,
    "%s(): proximity: Mask value 0x%08lx is not supported\n",
    __func__, mask);
    break;
    }
    if (val < 1 || val > 100) {
    dev_err(dev,
    "%s(): proximity: Sampling frequency %d is not in the range [1:100]\n",
    __func__, val);
    break;
    }
    ret = isl29028_set_proxim_sampling(chip, val, val2);
    break;
    case IIO_LIGHT:
    if (mask != IIO_CHAN_INFO_SCALE) {
    dev_err(dev,
    "%s(): light: Mask value 0x%08lx is not supported\n",
    __func__, mask);
    break;
    }
    if (val != 125 && val != 2000) {
    dev_err(dev,
    "%s(): light: Lux scale %d is not in the set {125, 2000}\n",
    __func__, val);
    break;
    }
    ret = isl29028_set_als_scale(chip, val);
    break;
    default:
    dev_err(dev, "%s(): Unsupported channel type %x\n",
    __func__, chan.type);
    break;
    }
    mutex_unlock(&chip.lock);
    if (ret < 0)
    return ret;
    ret = pm_runtime_put_autosuspend(dev);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int isl29028_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct isl29028_chip *chip = iio_priv(indio_dev);
    struct device *dev = regmap_get_device(chip.regmap);
    int ret, pm_ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    return ret;
    mutex_lock(&chip.lock);
    ret = -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    case IIO_CHAN_INFO_PROCESSED:
    switch (chan.type) {
    case IIO_LIGHT:
    ret = isl29028_als_get(chip, val);
    break;
    case IIO_INTENSITY:
    ret = isl29028_ir_get(chip, val);
    break;
    case IIO_PROXIMITY:
    ret = isl29028_read_proxim(chip, val);
    break;
    default:
    break;
    }
    if (ret < 0)
    break;
    ret = IIO_VAL_INT;
    break;
    case IIO_CHAN_INFO_SAMP_FREQ:
    if (chan.type != IIO_PROXIMITY)
    break;
// val = chip->prox_sampling_int;
// val2 = chip->prox_sampling_frac;
    ret = IIO_VAL_INT;
    break;
    case IIO_CHAN_INFO_SCALE:
    if (chan.type != IIO_LIGHT)
    break;
// val = chip->lux_scale;
    ret = IIO_VAL_INT;
    break;
    default:
    dev_err(dev, "%s(): mask value 0x%08lx is not supported\n",
    __func__, mask);
    break;
    }
    mutex_unlock(&chip.lock);
    if (ret < 0)
    return ret;
//
// Preserve the ret variable if the call to
// pm_runtime_put_autosuspend() is successful so the reading
// (if applicable) is returned to user space.
//
    pm_ret = pm_runtime_put_autosuspend(dev);
    if (pm_ret < 0)
    return pm_ret;
    return ret;
    }
    static IIO_CONST_ATTR(in_proximity_sampling_frequency_available,
    "1.25 2.5 5 10 13.3 20 80 100");
    static IIO_CONST_ATTR(in_illuminance_scale_available, "125 2000");

    static struct attribute *isl29028_attributes[] = {
    ISL29028_CONST_ATTR(in_proximity_sampling_frequency_available),
    ISL29028_CONST_ATTR(in_illuminance_scale_available),
    core::ptr::null_mut(),
    };
    static const struct attribute_group isl29108_group = {
    .attrs = isl29028_attributes,
    };
    static const struct iio_chan_spec isl29028_channels[] = {
    {
    .type = IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED) |
    BIT(IIO_CHAN_INFO_SCALE),
    }, {
    .type = IIO_INTENSITY,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    }, {
    .type = IIO_PROXIMITY,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SAMP_FREQ),
    }
    };
    static const struct iio_info isl29028_info = {
    .attrs = &isl29108_group,
    .read_raw = isl29028_read_raw,
    .write_raw = isl29028_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn isl29028_clear_configure_reg(chip: *mut isl29028_chip) -> c_int {
    static int isl29028_clear_configure_reg(struct isl29028_chip *chip)
    {
    struct device *dev = regmap_get_device(chip.regmap);
    int ret;
    ret = regmap_write(chip.regmap, ISL29028_REG_CONFIGURE, 0x0);
    if (ret < 0)
    dev_err(dev, "%s(): Error %d clearing the CONFIGURE register\n",
    __func__, ret);
    chip.als_ir_mode = ISL29028_MODE_NONE;
    chip.enable_prox = false;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn isl29028_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool isl29028_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ISL29028_REG_INTERRUPT:
    case ISL29028_REG_PROX_DATA:
    case ISL29028_REG_ALSIR_L:
    case ISL29028_REG_ALSIR_U:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config isl29028_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .volatile_reg = isl29028_is_volatile_reg,
    .max_register = ISL29028_NUM_REGS - 1,
    .num_reg_defaults_raw = ISL29028_NUM_REGS,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn isl29028_probe(client: *mut i2c_client) -> c_int {
    static int isl29028_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct isl29028_chip *chip;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*chip));
    if (!indio_dev)
    return -ENOMEM;
    chip = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    mutex_init(&chip.lock);
    chip.regmap = devm_regmap_init_i2c(client, &isl29028_regmap_config);
    if (IS_ERR(chip.regmap)) {
    ret = PTR_ERR(chip.regmap);
    dev_err(&client.dev, "%s: Error %d initializing regmap\n",
    __func__, ret);
    return ret;
    }
    chip.enable_prox  = false;
    chip.prox_sampling_int = 20;
    chip.prox_sampling_frac = 0;
    chip.lux_scale = 2000;
    ret = regmap_write(chip.regmap, ISL29028_REG_TEST1_MODE, 0x0);
    if (ret < 0) {
    dev_err(&client.dev,
    "%s(): Error %d writing to TEST1_MODE register\n",
    __func__, ret);
    return ret;
    }
    ret = regmap_write(chip.regmap, ISL29028_REG_TEST2_MODE, 0x0);
    if (ret < 0) {
    dev_err(&client.dev,
    "%s(): Error %d writing to TEST2_MODE register\n",
    __func__, ret);
    return ret;
    }
    ret = isl29028_clear_configure_reg(chip);
    if (ret < 0)
    return ret;
    indio_dev.info = &isl29028_info;
    indio_dev.channels = isl29028_channels;
    indio_dev.num_channels = ARRAY_SIZE(isl29028_channels);
    indio_dev.name = id.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    pm_runtime_enable(&client.dev);
    pm_runtime_set_autosuspend_delay(&client.dev,
    ISL29028_POWER_OFF_DELAY_MS);
    pm_runtime_use_autosuspend(&client.dev);
    ret = iio_device_register(indio_dev);
    if (ret < 0) {
    dev_err(&client.dev,
    "%s(): iio registration failed with error %d\n",
    __func__, ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isl29028_remove(client: *mut i2c_client) {
    static void isl29028_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct isl29028_chip *chip = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    pm_runtime_disable(&client.dev);
    pm_runtime_set_suspended(&client.dev);
    isl29028_clear_configure_reg(chip);
    }
#[no_mangle]
unsafe extern "C" fn isl29028_suspend(dev: *mut device) -> c_int {
    static int isl29028_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct isl29028_chip *chip = iio_priv(indio_dev);
    int ret;
    mutex_lock(&chip.lock);
    ret = isl29028_clear_configure_reg(chip);
    mutex_unlock(&chip.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn isl29028_resume(dev: *mut device) -> c_int {
    static int isl29028_resume(struct device *dev)
    {
//
// The specific component (ALS/IR or proximity) will enable itself as
// needed the next time that the user requests a reading. This is done
// above in isl29028_set_als_ir_mode() and isl29028_enable_proximity().
//
    return 0;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(isl29028_pm_ops, isl29028_suspend,
    isl29028_resume, core::ptr::null_mut());
    static const struct i2c_device_id isl29028_id[] = {
    { .name = "isl29028" },
    { .name = "isl29030" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, isl29028_id);
    static const struct of_device_id isl29028_of_match[] = {
    { .compatible = "isl,isl29028", }, /* for backward compat., don't use */
    { .compatible = "isil,isl29028", },
    { .compatible = "isil,isl29030", },
    { }
    };
    MODULE_DEVICE_TABLE(of, isl29028_of_match);
    static struct i2c_driver isl29028_driver = {
    .driver  = {
    .name = "isl29028",
    .pm = pm_ptr(&isl29028_pm_ops),
    .of_match_table = isl29028_of_match,
    },
    .probe = isl29028_probe,
    .remove  = isl29028_remove,
    .id_table = isl29028_id,
    };
    module_i2c_driver(isl29028_driver);
    MODULE_DESCRIPTION("ISL29028 Ambient Light and Proximity Sensor driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
