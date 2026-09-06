//! Automatically rewritten from C to Rust
//! Source: drivers/iio/temperature/max31865.c
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
// Copyright (c) Linumiz 2021
//
// max31865.c - Maxim MAX31865 RTD-to-Digital Converter sensor driver
//
// Author: Navin Sankar Velliangiri <navin@linumiz.com>
//

//
// The MSB of the register value determines whether the following byte will
// be written or read. If it is 0, read will follow and if it is 1, write
// will follow.
//

// The MAX31865 registers
pub const MAX31865_CFG_REG: c_uint = 0x00;
pub const MAX31865_RTD_MSB: c_uint = 0x01;
pub const MAX31865_FAULT_STATUS: c_uint = 0x07;

    static const char max31865_show_samp_freq[] = "50 60";
    static const struct iio_chan_spec max31865_channels[] = {
    {	/* RTD Temperature */
    .type = IIO_TEMP,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE)
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max31865_data {
    pub spi: *mut spi_device,
    pub lock: mutex,
    pub filter_50hz: bool,
    pub three_wire: bool,
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[2],
}

    static int max31865_read(struct max31865_data *data, u8 reg,
    unsigned int read_size)
    {
    return spi_write_then_read(data.spi, &reg, 1, data.buf, read_size);
    }
#[no_mangle]
unsafe extern "C" fn max31865_write(data: *mut max31865_data, len: usize) -> c_int {
    static int max31865_write(struct max31865_data *data, size_t len)
    {
    return spi_write(data.spi, data.buf, len);
    }
#[no_mangle]
unsafe extern "C" fn enable_bias(data: *mut max31865_data) -> c_int {
    static int enable_bias(struct max31865_data *data)
    {
    u8 cfg;
    int ret;
    ret = max31865_read(data, MAX31865_CFG_REG, 1);
    if (ret)
    return ret;
    cfg = data.buf[0];
    data.buf[0] = MAX31865_CFG_REG | MAX31865_RD_WR_BIT;
    data.buf[1] = cfg | MAX31865_CFG_VBIAS;
    return max31865_write(data, 2);
    }
#[no_mangle]
unsafe extern "C" fn disable_bias(data: *mut max31865_data) -> c_int {
    static int disable_bias(struct max31865_data *data)
    {
    u8 cfg;
    int ret;
    ret = max31865_read(data, MAX31865_CFG_REG, 1);
    if (ret)
    return ret;
    cfg = data.buf[0];
    cfg &= ~MAX31865_CFG_VBIAS;
    data.buf[0] = MAX31865_CFG_REG | MAX31865_RD_WR_BIT;
    data.buf[1] = cfg;
    return max31865_write(data, 2);
    }
#[no_mangle]
unsafe extern "C" fn max31865_rtd_read(data: *mut max31865_data, val: *mut c_int) -> c_int {
    static int max31865_rtd_read(struct max31865_data *data, int *val)
    {
    u8 reg;
    int ret;
// Enable BIAS to start the conversion
    ret = enable_bias(data);
    if (ret)
    return ret;
// wait 10.5ms before initiating the conversion
    msleep(11);
    ret = max31865_read(data, MAX31865_CFG_REG, 1);
    if (ret)
    return ret;
    reg = data.buf[0];
    reg |= MAX31865_CFG_1SHOT | MAX31865_FAULT_STATUS_CLEAR;
    data.buf[0] = MAX31865_CFG_REG | MAX31865_RD_WR_BIT;
    data.buf[1] = reg;
    ret = max31865_write(data, 2);
    if (ret)
    return ret;
    if (data.filter_50hz) {
// 50Hz filter mode requires 62.5ms to complete
    msleep(63);
    } else {
// 60Hz filter mode requires 52ms to complete
    msleep(52);
    }
    ret = max31865_read(data, MAX31865_RTD_MSB, 2);
    if (ret)
    return ret;
// val = get_unaligned_be16(&data->buf) >> 1;
    return disable_bias(data);
    }
    static int max31865_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max31865_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&data.lock);
    ret = max31865_rtd_read(data, val);
    mutex_unlock(&data.lock);
    if (ret)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// Temp. Data resolution is 0.03125 degree centigrade
// val = 31;
// val2 = 250000; /* 1000 * 0.03125
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn max31865_init(data: *mut max31865_data) -> c_int {
    static int max31865_init(struct max31865_data *data)
    {
    u8 cfg;
    int ret;
    ret = max31865_read(data, MAX31865_CFG_REG, 1);
    if (ret)
    return ret;
    cfg = data.buf[0];
    if (data.three_wire)
// 3-wire RTD connection
    cfg |= MAX31865_3WIRE_RTD;
    if (data.filter_50hz)
// 50Hz noise rejection filter
    cfg |= MAX31865_FILTER_50HZ;
    data.buf[0] = MAX31865_CFG_REG | MAX31865_RD_WR_BIT;
    data.buf[1] = cfg;
    return max31865_write(data, 2);
    }
#[no_mangle]
unsafe extern "C" fn show_fault(dev: *mut device, faultbit: u8, buf: *mut c_char) -> isize {
    static ssize_t show_fault(struct device *dev, u8 faultbit, char *buf)
    {
    int ret;
    bool fault;
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    struct max31865_data *data = iio_priv(indio_dev);
    ret = max31865_read(data, MAX31865_FAULT_STATUS, 1);
    if (ret)
    return ret;
    fault = data.buf[0] & faultbit;
    return sysfs_emit(buf, "%d\n", fault);
    }
    static ssize_t show_fault_ovuv(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return show_fault(dev, MAX31865_FAULT_OVUV, buf);
    }
    static ssize_t show_filter(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    struct max31865_data *data = iio_priv(indio_dev);
    return sysfs_emit(buf, "%d\n", data.filter_50hz ? 50 : 60);
    }
    static ssize_t set_filter(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t len)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    struct max31865_data *data = iio_priv(indio_dev);
    unsigned int freq;
    int ret;
    ret = kstrtouint(buf, 10, &freq);
    if (ret)
    return ret;
    switch (freq) {
    case 50:
    data.filter_50hz = true;
    break;
    case 60:
    data.filter_50hz = false;
    break;
    default:
    return -EINVAL;
    }
    mutex_lock(&data.lock);
    ret = max31865_init(data);
    mutex_unlock(&data.lock);
    if (ret)
    return ret;
    return len;
    }
    static IIO_CONST_ATTR_SAMP_FREQ_AVAIL(max31865_show_samp_freq);
    static IIO_DEVICE_ATTR(fault_ovuv, 0444, show_fault_ovuv, core::ptr::null_mut(), 0);
    static IIO_DEVICE_ATTR(in_filter_notch_center_frequency, 0644,
    show_filter, set_filter, 0);
    static struct attribute *max31865_attributes[] = {
    &iio_dev_attr_fault_ovuv.dev_attr.attr,
    &iio_const_attr_sampling_frequency_available.dev_attr.attr,
    &iio_dev_attr_in_filter_notch_center_frequency.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group max31865_group = {
    .attrs = max31865_attributes,
    };
    static const struct iio_info max31865_info = {
    .read_raw = max31865_read_raw,
    .attrs = &max31865_group,
    };
#[no_mangle]
unsafe extern "C" fn max31865_probe(spi: *mut spi_device) -> c_int {
    static int max31865_probe(struct spi_device *spi)
    {
    const struct spi_device_id *id = spi_get_device_id(spi);
    struct iio_dev *indio_dev;
    struct max31865_data *data;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.spi = spi;
    data.filter_50hz = false;
    mutex_init(&data.lock);
    indio_dev.info = &max31865_info;
    indio_dev.name = id.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = max31865_channels;
    indio_dev.num_channels = ARRAY_SIZE(max31865_channels);
    if (device_property_read_bool(&spi.dev, "maxim,3-wire")) {
// select 3 wire
    data.three_wire = 1;
    } else {
// select 2 or 4 wire
    data.three_wire = 0;
    }
    ret = max31865_init(data);
    if (ret) {
    dev_err(&spi.dev, "error: Failed to configure max31865\n");
    return ret;
    }
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct spi_device_id max31865_id[] = {
    { .name = "max31865" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, max31865_id);
    static const struct of_device_id max31865_of_match[] = {
    { .compatible = "maxim,max31865" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max31865_of_match);
    static struct spi_driver max31865_driver = {
    .driver = {
    .name	= "max31865",
    .of_match_table = max31865_of_match,
    },
    .probe = max31865_probe,
    .id_table = max31865_id,
    };
    module_spi_driver(max31865_driver);
    MODULE_AUTHOR("Navin Sankar Velliangiri <navin@linumiz.com>");
    MODULE_DESCRIPTION("Maxim MAX31865 RTD-to-Digital Converter sensor driver");
    MODULE_LICENSE("GPL v2");
