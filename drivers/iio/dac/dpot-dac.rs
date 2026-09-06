//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/dpot-dac.c
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
// IIO DAC emulation driver using a digital potentiometer
//
// Copyright (C) 2016 Axentia Technologies AB
//
// Author: Peter Rosin <peda@axentia.se>
//
// It is assumed that the dpot is used as a voltage divider between the
// current dpot wiper setting and the maximum resistance of the dpot. The
// divided voltage is provided by a vref regulator.
//
// .------.
// .-----------.  |      |
// | vref      |--'    .---.
// | regulator |--.    |   |
// '-----------'  |    | d |
// |    | p |
// |    | o |  wiper
// |    | t |<---------+
// |    |   |
// |    '---'       dac output voltage
// |      |
// '------+------------+
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpot_dac {
    pub vref: *mut regulator,
    pub dpot: *mut iio_channel,
    pub max_ohms: u32,
}

    static const struct iio_chan_spec dpot_dac_iio_channel = {
    .type = IIO_VOLTAGE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW)
    | BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_separate_available = BIT(IIO_CHAN_INFO_RAW),
    .output = 1,
    .indexed = 1,
    };
    static int dpot_dac_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct dpot_dac *dac = iio_priv(indio_dev);
    int ret;
    unsigned long long tmp;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    return iio_read_channel_raw(dac.dpot, val);
    case IIO_CHAN_INFO_SCALE:
    ret = iio_read_channel_scale(dac.dpot, val, val2);
    switch (ret) {
    case IIO_VAL_FRACTIONAL_LOG2:
    tmp = *val * 1000000000LL;
    do_div(tmp, dac.max_ohms);
    tmp *= regulator_get_voltage(dac.vref) / 1000;
    do_div(tmp, 1000000000LL);
// val = tmp;
    return ret;
    case IIO_VAL_INT:
//
// Convert integer scale to fractional scale by
// setting the denominator (val2) to one...
//
// val2 = 1;
    ret = IIO_VAL_FRACTIONAL;
// ...and fall through. Say it again for GCC.
    fallthrough;
    case IIO_VAL_FRACTIONAL:
// val *= regulator_get_voltage(dac->vref) / 1000;
// val2 *= dac->max_ohms;
    break;
    }
    return ret;
    }
    return -EINVAL;
    }
    static int dpot_dac_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    struct dpot_dac *dac = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
// type = IIO_VAL_INT;
    return iio_read_avail_channel_raw(dac.dpot, vals, length);
    }
    return -EINVAL;
    }
    static int dpot_dac_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct dpot_dac *dac = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    return iio_write_channel_raw(dac.dpot, val);
    }
    return -EINVAL;
    }
    static const struct iio_info dpot_dac_info = {
    .read_raw = dpot_dac_read_raw,
    .read_avail = dpot_dac_read_avail,
    .write_raw = dpot_dac_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn dpot_dac_channel_max_ohms(indio_dev: *mut iio_dev) -> c_int {
    static int dpot_dac_channel_max_ohms(struct iio_dev *indio_dev)
    {
    struct device *dev = &indio_dev.dev;
    struct dpot_dac *dac = iio_priv(indio_dev);
    unsigned long long tmp;
    int ret;
    int val;
    int val2;
    int max;
    ret = iio_read_max_channel_raw(dac.dpot, &max);
    if (ret < 0) {
    dev_err(dev, "dpot does not indicate its raw maximum value\n");
    return ret;
    }
    switch (iio_read_channel_scale(dac.dpot, &val, &val2)) {
    case IIO_VAL_INT:
    return max * val;
    case IIO_VAL_FRACTIONAL:
    tmp = (unsigned long long)max * val;
    do_div(tmp, val2);
    return tmp;
    case IIO_VAL_FRACTIONAL_LOG2:
    tmp = val * 1000000000LL * max >> val2;
    do_div(tmp, 1000000000LL);
    return tmp;
    default:
    dev_err(dev, "dpot has a scale that is too weird\n");
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn dpot_dac_probe(pdev: *mut platform_device) -> c_int {
    static int dpot_dac_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct iio_dev *indio_dev;
    struct dpot_dac *dac;
    enum iio_chan_type type;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*dac));
    if (!indio_dev)
    return -ENOMEM;
    platform_set_drvdata(pdev, indio_dev);
    dac = iio_priv(indio_dev);
    indio_dev.name = dev_name(dev);
    indio_dev.info = &dpot_dac_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = &dpot_dac_iio_channel;
    indio_dev.num_channels = 1;
    dac.vref = devm_regulator_get(dev, "vref");
    if (IS_ERR(dac.vref))
    return dev_err_probe(&pdev.dev, PTR_ERR(dac.vref),
    "failed to get vref regulator\n");
    dac.dpot = devm_iio_channel_get(dev, "dpot");
    if (IS_ERR(dac.dpot))
    return dev_err_probe(&pdev.dev, PTR_ERR(dac.dpot),
    "failed to get dpot input channel\n");
    ret = iio_get_channel_type(dac.dpot, &type);
    if (ret < 0)
    return ret;
    if (type != IIO_RESISTANCE) {
    dev_err(dev, "dpot is of the wrong type\n");
    return -EINVAL;
    }
    ret = dpot_dac_channel_max_ohms(indio_dev);
    if (ret < 0)
    return ret;
    dac.max_ohms = ret;
    ret = regulator_enable(dac.vref);
    if (ret) {
    dev_err(dev, "failed to enable the vref regulator\n");
    return ret;
    }
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err(dev, "failed to register iio device\n");
    goto disable_reg;
    }
    return 0;
    disable_reg:
    regulator_disable(dac.vref);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dpot_dac_remove(pdev: *mut platform_device) {
    static void dpot_dac_remove(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev = platform_get_drvdata(pdev);
    struct dpot_dac *dac = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    regulator_disable(dac.vref);
    }
    static const struct of_device_id dpot_dac_match[] = {
    { .compatible = "dpot-dac" },
    { }
    };
    MODULE_DEVICE_TABLE(of, dpot_dac_match);
    static struct platform_driver dpot_dac_driver = {
    .probe = dpot_dac_probe,
    .remove = dpot_dac_remove,
    .driver = {
    .name = "iio-dpot-dac",
    .of_match_table = dpot_dac_match,
    },
    };
    module_platform_driver(dpot_dac_driver);
    MODULE_DESCRIPTION("DAC emulation driver using a digital potentiometer");
    MODULE_AUTHOR("Peter Rosin <peda@axentia.se>");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_CONSUMER");
