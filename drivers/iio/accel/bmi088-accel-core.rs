//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/bmi088-accel-core.c
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


// SPDX-License-Identifier: GPL-2.0
//
// 3-axis accelerometer driver supporting following Bosch-Sensortec chips:
// - BMI088
// - BMI085
// - BMI090L
//
// Copyright (c) 2018-2021, Topic Embedded Products
//

pub const BMI088_ACCEL_REG_CHIP_ID: c_uint = 0x00;
pub const BMI088_ACCEL_REG_ERROR: c_uint = 0x02;
pub const BMI088_ACCEL_REG_INT_STATUS: c_uint = 0x1D;

pub const BMI088_ACCEL_REG_RESET: c_uint = 0x7E;
pub const BMI088_ACCEL_RESET_VAL: c_uint = 0xB6;
pub const BMI088_ACCEL_REG_PWR_CTRL: c_uint = 0x7D;
pub const BMI088_ACCEL_REG_PWR_CONF: c_uint = 0x7C;
pub const BMI088_ACCEL_REG_INT_MAP_DATA: c_uint = 0x58;

pub const BMI088_ACCEL_REG_INT1_IO_CONF: c_uint = 0x53;

pub const BMI088_ACCEL_REG_INT2_IO_CONF: c_uint = 0x54;

pub const BMI088_ACCEL_REG_ACC_CONF: c_uint = 0x40;
pub const BMI088_ACCEL_MODE_ODR_MASK: c_uint = 0x0f;
pub const BMI088_ACCEL_REG_ACC_RANGE: c_uint = 0x41;
pub const BMI088_ACCEL_RANGE_3G: c_uint = 0x00;
pub const BMI088_ACCEL_RANGE_6G: c_uint = 0x01;
pub const BMI088_ACCEL_RANGE_12G: c_uint = 0x02;
pub const BMI088_ACCEL_RANGE_24G: c_uint = 0x03;
pub const BMI088_ACCEL_REG_TEMP: c_uint = 0x22;
pub const BMI088_ACCEL_REG_TEMP_SHIFT: c_int = 5;
pub const BMI088_ACCEL_TEMP_UNIT: c_int = 125;
pub const BMI088_ACCEL_TEMP_OFFSET: c_int = 23000;
pub const BMI088_ACCEL_REG_XOUT_L: c_uint = 0x12;

    (BMI088_ACCEL_REG_XOUT_L + (axis * 2))
pub const BMI088_ACCEL_MAX_STARTUP_TIME_US: c_int = 1000;
pub const BMI088_AUTO_SUSPEND_DELAY_MS: c_int = 2000;
pub const BMI088_ACCEL_REG_FIFO_STATUS: c_uint = 0x0E;
pub const BMI088_ACCEL_REG_FIFO_CONFIG0: c_uint = 0x48;
pub const BMI088_ACCEL_REG_FIFO_CONFIG1: c_uint = 0x49;
pub const BMI088_ACCEL_REG_FIFO_DATA: c_uint = 0x3F;
pub const BMI088_ACCEL_FIFO_LENGTH: c_int = 100;
pub const BMI088_ACCEL_FIFO_MODE_FIFO: c_uint = 0x40;
pub const BMI088_ACCEL_FIFO_MODE_STREAM: c_uint = 0x80;

    enum bmi088_accel_axis {
    AXIS_X,
    AXIS_Y,
    AXIS_Z,
    };
    static const int bmi088_sample_freqs[] = {
    12, 500000,
    25, 0,
    50, 0,
    100, 0,
    200, 0,
    400, 0,
    800, 0,
    1600, 0,
    };
// Available OSR (over sampling rate) sets the 3dB cut-off frequency
    enum bmi088_osr_modes {
    BMI088_ACCEL_MODE_OSR_NORMAL = 0xA,
    BMI088_ACCEL_MODE_OSR_2 = 0x9,
    BMI088_ACCEL_MODE_OSR_4 = 0x8,
    };
// Available ODR (output data rates) in Hz
    enum bmi088_odr_modes {
    BMI088_ACCEL_MODE_ODR_12_5 = 0x5,
    BMI088_ACCEL_MODE_ODR_25 = 0x6,
    BMI088_ACCEL_MODE_ODR_50 = 0x7,
    BMI088_ACCEL_MODE_ODR_100 = 0x8,
    BMI088_ACCEL_MODE_ODR_200 = 0x9,
    BMI088_ACCEL_MODE_ODR_400 = 0xa,
    BMI088_ACCEL_MODE_ODR_800 = 0xb,
    BMI088_ACCEL_MODE_ODR_1600 = 0xc,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi088_accel_chip_info {
    pub name: *const c_char,
    pub chip_id: u8,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_int,
    pub scale_table: [c_int; 4][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi088_accel_data {
    pub regmap: *mut regmap,
    pub chip_info: *const bmi088_accel_chip_info,
    pub /: *mut *mut u8 buffer[2] __aligned(IIO_DMA_MINALIGN); / shared DMA safe buffer,
}

    static const struct regmap_range bmi088_volatile_ranges[] = {
// All registers below 0x40 are volatile, except the CHIP ID.
    regmap_reg_range(BMI088_ACCEL_REG_ERROR, 0x3f),
// Mark the RESET as volatile too, it is self-clearing
    regmap_reg_range(BMI088_ACCEL_REG_RESET, BMI088_ACCEL_REG_RESET),
    };
    static const struct regmap_access_table bmi088_volatile_table = {
    .yes_ranges	= bmi088_volatile_ranges,
    .n_yes_ranges	= ARRAY_SIZE(bmi088_volatile_ranges),
    };
    const struct regmap_config bmi088_regmap_conf = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x7E,
    .volatile_table = &bmi088_volatile_table,
    .cache_type = REGCACHE_MAPLE,
    };
    EXPORT_SYMBOL_NS_GPL(bmi088_regmap_conf, "IIO_BMI088");
#[no_mangle]
unsafe extern "C" fn bmi088_accel_power_up(data: *mut bmi088_accel_data) -> c_int {
    static int bmi088_accel_power_up(struct bmi088_accel_data *data)
    {
    int ret;
// Enable accelerometer and temperature sensor
    ret = regmap_write(data.regmap, BMI088_ACCEL_REG_PWR_CTRL, 0x4);
    if (ret)
    return ret;
// Datasheet recommends to wait at least 5ms before communication
    usleep_range(5000, 6000);
// Disable suspend mode
    ret = regmap_write(data.regmap, BMI088_ACCEL_REG_PWR_CONF, 0x0);
    if (ret)
    return ret;
// Recommended at least 1ms before further communication
    usleep_range(1000, 1200);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bmi088_accel_power_down(data: *mut bmi088_accel_data) -> c_int {
    static int bmi088_accel_power_down(struct bmi088_accel_data *data)
    {
    int ret;
// Enable suspend mode
    ret = regmap_write(data.regmap, BMI088_ACCEL_REG_PWR_CONF, 0x3);
    if (ret)
    return ret;
// Recommended at least 1ms before further communication
    usleep_range(1000, 1200);
// Disable accelerometer and temperature sensor
    ret = regmap_write(data.regmap, BMI088_ACCEL_REG_PWR_CTRL, 0x0);
    if (ret)
    return ret;
// Datasheet recommends to wait at least 5ms before communication
    usleep_range(5000, 6000);
    return 0;
    }
    static int bmi088_accel_get_sample_freq(struct bmi088_accel_data *data,
    int *val, int *val2)
    {
    unsigned int value;
    int ret;
    ret = regmap_read(data.regmap, BMI088_ACCEL_REG_ACC_CONF,
    &value);
    if (ret)
    return ret;
    value &= BMI088_ACCEL_MODE_ODR_MASK;
    value -= BMI088_ACCEL_MODE_ODR_12_5;
    value <<= 1;
    if (value >= ARRAY_SIZE(bmi088_sample_freqs) - 1)
    return -EINVAL;
// val = bmi088_sample_freqs[value];
// val2 = bmi088_sample_freqs[value + 1];
    return IIO_VAL_INT_PLUS_MICRO;
    }
#[no_mangle]
unsafe extern "C" fn bmi088_accel_set_sample_freq(data: *mut bmi088_accel_data, val: c_int) -> c_int {
    static int bmi088_accel_set_sample_freq(struct bmi088_accel_data *data, int val)
    {
    unsigned int regval;
    let mut index: c_int = 0;
    while (index < ARRAY_SIZE(bmi088_sample_freqs) &&
    bmi088_sample_freqs[index] != val)
    index += 2;
    if (index >= ARRAY_SIZE(bmi088_sample_freqs))
    return -EINVAL;
    regval = (index >> 1) + BMI088_ACCEL_MODE_ODR_12_5;
    return regmap_update_bits(data.regmap, BMI088_ACCEL_REG_ACC_CONF,
    BMI088_ACCEL_MODE_ODR_MASK, regval);
    }
#[no_mangle]
unsafe extern "C" fn bmi088_accel_set_scale(data: *mut bmi088_accel_data, val: c_int, val2: c_int) -> c_int {
    static int bmi088_accel_set_scale(struct bmi088_accel_data *data, int val, int val2)
    {
    unsigned int i;
    for (i = 0; i < 4; i++)
    if (val  == data.chip_info.scale_table[i][0] &&
    val2 == data.chip_info.scale_table[i][1])
    break;
    if (i == 4)
    return -EINVAL;
    return regmap_write(data.regmap, BMI088_ACCEL_REG_ACC_RANGE, i);
    }
#[no_mangle]
unsafe extern "C" fn bmi088_accel_get_temp(data: *mut bmi088_accel_data, val: *mut c_int) -> c_int {
    static int bmi088_accel_get_temp(struct bmi088_accel_data *data, int *val)
    {
    int ret;
    s16 temp;
    ret = regmap_bulk_read(data.regmap, BMI088_ACCEL_REG_TEMP,
    &data.buffer, sizeof(__be16));
    if (ret)
    return ret;
// data->buffer is cacheline aligned
    temp = be16_to_cpu(*(__be16 *)data.buffer);
// val = temp >> BMI088_ACCEL_REG_TEMP_SHIFT;
    return IIO_VAL_INT;
    }
    static int bmi088_accel_get_axis(struct bmi088_accel_data *data,
    struct iio_chan_spec const *chan,
    int *val)
    {
    int ret;
    s16 raw_val;
    ret = regmap_bulk_read(data.regmap,
    BMI088_ACCEL_AXIS_TO_REG(chan.scan_index),
    data.buffer, sizeof(__le16));
    if (ret)
    return ret;
    raw_val = le16_to_cpu(*(__le16 *)data.buffer);
// val = raw_val;
    return IIO_VAL_INT;
    }
    static int bmi088_accel_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct bmi088_accel_data *data = iio_priv(indio_dev);
    struct device *dev = regmap_get_device(data.regmap);
    int ret;
    int reg;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    switch (chan.type) {
    case IIO_TEMP:
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
    ret = bmi088_accel_get_temp(data, val);
    goto out_read_raw_pm_put;
    case IIO_ACCEL:
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
    if (!iio_device_claim_direct(indio_dev)) {
    ret = -EBUSY;
    goto out_read_raw_pm_put;
    }
    ret = bmi088_accel_get_axis(data, chan, val);
    iio_device_release_direct(indio_dev);
    if (!ret)
    ret = IIO_VAL_INT;
    goto out_read_raw_pm_put;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_OFFSET:
    switch (chan.type) {
    case IIO_TEMP:
// Offset applies before scale
// val = BMI088_ACCEL_TEMP_OFFSET/BMI088_ACCEL_TEMP_UNIT;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_TEMP:
// 0.125 degrees per LSB
// val = BMI088_ACCEL_TEMP_UNIT;
    return IIO_VAL_INT;
    case IIO_ACCEL:
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
    ret = regmap_read(data.regmap,
    BMI088_ACCEL_REG_ACC_RANGE, &reg);
    if (ret)
    goto out_read_raw_pm_put;
    reg = FIELD_GET(BMIO088_ACCEL_ACC_RANGE_MSK, reg);
// val  = data->chip_info->scale_table[reg][0];
// val2 = data->chip_info->scale_table[reg][1];
    ret = IIO_VAL_INT_PLUS_MICRO;
    goto out_read_raw_pm_put;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SAMP_FREQ:
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
    ret = bmi088_accel_get_sample_freq(data, val, val2);
    goto out_read_raw_pm_put;
    default:
    break;
    }
    return -EINVAL;
    out_read_raw_pm_put:
    pm_runtime_put_autosuspend(dev);
    return ret;
    }
    static int bmi088_accel_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    struct bmi088_accel_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
// vals = (const int *)data->chip_info->scale_table;
// length = 8;
// type = IIO_VAL_INT_PLUS_MICRO;
    return IIO_AVAIL_LIST;
    case IIO_CHAN_INFO_SAMP_FREQ:
// type = IIO_VAL_INT_PLUS_MICRO;
// vals = bmi088_sample_freqs;
// length = ARRAY_SIZE(bmi088_sample_freqs);
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
    static int bmi088_accel_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct bmi088_accel_data *data = iio_priv(indio_dev);
    struct device *dev = regmap_get_device(data.regmap);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
    ret = bmi088_accel_set_scale(data, val, val2);
    pm_runtime_put_autosuspend(dev);
    return ret;
    case IIO_CHAN_INFO_SAMP_FREQ:
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
    ret = bmi088_accel_set_sample_freq(data, val);
    pm_runtime_put_autosuspend(dev);
    return ret;
    default:
    return -EINVAL;
    }
    }

    .type = IIO_ACCEL, \
    .modified = 1, \
    .channel2 = IIO_MOD_##_axis, \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW), \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) | \
    BIT(IIO_CHAN_INFO_SAMP_FREQ), \
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_SAMP_FREQ) | \
    BIT(IIO_CHAN_INFO_SCALE), \
    .scan_index = AXIS_##_axis, \
    }
    static const struct iio_chan_spec bmi088_accel_channels[] = {
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_OFFSET),
    .scan_index = -1,
    },
    BMI088_ACCEL_CHANNEL(X),
    BMI088_ACCEL_CHANNEL(Y),
    BMI088_ACCEL_CHANNEL(Z),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    };
    static const struct bmi088_accel_chip_info bmi088_accel_chip_info_tbl[] = {
    [BOSCH_BMI085] = {
    .name = "bmi085-accel",
    .chip_id = 0x1F,
    .channels = bmi088_accel_channels,
    .num_channels = ARRAY_SIZE(bmi088_accel_channels),
    .scale_table = {{0, 598}, {0, 1196}, {0, 2393}, {0, 4785}},
    },
    [BOSCH_BMI088] = {
    .name = "bmi088-accel",
    .chip_id = 0x1E,
    .channels = bmi088_accel_channels,
    .num_channels = ARRAY_SIZE(bmi088_accel_channels),
    .scale_table = {{0, 897}, {0, 1794}, {0, 3589}, {0, 7178}},
    },
    [BOSCH_BMI090L] = {
    .name = "bmi090l-accel",
    .chip_id = 0x1A,
    .channels = bmi088_accel_channels,
    .num_channels = ARRAY_SIZE(bmi088_accel_channels),
    .scale_table = {{0, 897}, {0, 1794}, {0, 3589}, {0, 7178}},
    },
    };
    static const struct iio_info bmi088_accel_info = {
    .read_raw	= bmi088_accel_read_raw,
    .write_raw	= bmi088_accel_write_raw,
    .read_avail	= bmi088_accel_read_avail,
    };
    static const unsigned long bmi088_accel_scan_masks[] = {
    BIT(AXIS_X) | BIT(AXIS_Y) | BIT(AXIS_Z),
    0
    };
#[no_mangle]
unsafe extern "C" fn bmi088_accel_chip_init(data: *mut bmi088_accel_data, type: enum bmi_device_type) -> c_int {
    static int bmi088_accel_chip_init(struct bmi088_accel_data *data, enum bmi_device_type type)
    {
    struct device *dev = regmap_get_device(data.regmap);
    int ret, i;
    unsigned int val;
    if (type >= BOSCH_UNKNOWN)
    return -ENODEV;
// Do a dummy read to enable SPI interface, won't harm I2C
    regmap_read(data.regmap, BMI088_ACCEL_REG_INT_STATUS, &val);
//
// Reset chip to get it in a known good state. A delay of 1ms after
// reset is required according to the data sheet
//
    ret = regmap_write(data.regmap, BMI088_ACCEL_REG_RESET,
    BMI088_ACCEL_RESET_VAL);
    if (ret)
    return ret;
    usleep_range(1000, 2000);
// Do a dummy read again after a reset to enable the SPI interface
    regmap_read(data.regmap, BMI088_ACCEL_REG_INT_STATUS, &val);
// Read chip ID
    ret = regmap_read(data.regmap, BMI088_ACCEL_REG_CHIP_ID, &val);
    if (ret) {
    dev_err(dev, "Error: Reading chip id\n");
    return ret;
    }
// Validate chip ID
    for (i = 0; i < ARRAY_SIZE(bmi088_accel_chip_info_tbl); i++)
    if (bmi088_accel_chip_info_tbl[i].chip_id == val)
    break;
    if (i == ARRAY_SIZE(bmi088_accel_chip_info_tbl))
    data.chip_info = &bmi088_accel_chip_info_tbl[type];
    else
    data.chip_info = &bmi088_accel_chip_info_tbl[i];
    if (i != type)
    dev_warn(dev, "unexpected chip id 0x%X\n", val);
    return 0;
    }
    int bmi088_accel_core_probe(struct device *dev, struct regmap *regmap,
    int irq, enum bmi_device_type type)
    {
    struct bmi088_accel_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    dev_set_drvdata(dev, indio_dev);
    data.regmap = regmap;
    ret = bmi088_accel_chip_init(data, type);
    if (ret)
    return ret;
    indio_dev.channels = data.chip_info.channels;
    indio_dev.num_channels = data.chip_info.num_channels;
    indio_dev.name = data.chip_info.name;
    indio_dev.available_scan_masks = bmi088_accel_scan_masks;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &bmi088_accel_info;
// Enable runtime PM
    pm_runtime_get_noresume(dev);
    pm_runtime_set_suspended(dev);
    pm_runtime_enable(dev);
// We need ~6ms to startup, so set the delay to 6 seconds
    pm_runtime_set_autosuspend_delay(dev, 6000);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_put(dev);
    ret = iio_device_register(indio_dev);
    if (ret)
    dev_err(dev, "Unable to register iio device\n");
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(bmi088_accel_core_probe, "IIO_BMI088");
#[no_mangle]
pub unsafe extern "C" fn bmi088_accel_core_remove(dev: *mut device) {
    void bmi088_accel_core_remove(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct bmi088_accel_data *data = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    pm_runtime_disable(dev);
    pm_runtime_set_suspended(dev);
    bmi088_accel_power_down(data);
    }
    EXPORT_SYMBOL_NS_GPL(bmi088_accel_core_remove, "IIO_BMI088");
#[no_mangle]
unsafe extern "C" fn bmi088_accel_runtime_suspend(dev: *mut device) -> c_int {
    static int bmi088_accel_runtime_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct bmi088_accel_data *data = iio_priv(indio_dev);
    return bmi088_accel_power_down(data);
    }
#[no_mangle]
unsafe extern "C" fn bmi088_accel_runtime_resume(dev: *mut device) -> c_int {
    static int bmi088_accel_runtime_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct bmi088_accel_data *data = iio_priv(indio_dev);
    return bmi088_accel_power_up(data);
    }
    EXPORT_NS_GPL_RUNTIME_DEV_PM_OPS(bmi088_accel_pm_ops,
    bmi088_accel_runtime_suspend,
    bmi088_accel_runtime_resume, core::ptr::null_mut(),
    IIO_BMI088);
    MODULE_AUTHOR("Niek van Agt <niek.van.agt@topicproducts.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("BMI088 accelerometer driver (core)");
