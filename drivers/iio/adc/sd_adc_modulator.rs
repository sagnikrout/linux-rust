//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/sd_adc_modulator.c
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
// Generic sigma delta modulator driver
//
// Copyright (C) 2017, STMicroelectronics - All Rights Reserved
// Author: Arnaud Pouliquen <arnaud.pouliquen@st.com>.
//

    static const struct iio_info iio_sd_mod_iio_info;
    static const struct iio_chan_spec iio_sd_mod_ch = {
    .type = IIO_VOLTAGE,
    .indexed = 1,
    .scan_type = {
    .sign = 'u',
    .realbits = 1,
    .shift = 0,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_sd_backend_priv {
    pub vref: *mut regulator,
    pub vref_mv: c_int,
}

#[no_mangle]
unsafe extern "C" fn iio_sd_mod_enable(backend: *mut iio_backend) -> c_int {
    static int iio_sd_mod_enable(struct iio_backend *backend)
    {
    struct iio_sd_backend_priv *priv = iio_backend_get_priv(backend);
    if (priv.vref)
    return regulator_enable(priv.vref);
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn iio_sd_mod_disable(backend: *mut iio_backend) {
    static void iio_sd_mod_disable(struct iio_backend *backend)
    {
    struct iio_sd_backend_priv *priv = iio_backend_get_priv(backend);
    if (priv.vref)
    regulator_disable(priv.vref);
    };
    static int iio_sd_mod_read(struct iio_backend *backend, struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct iio_sd_backend_priv *priv = iio_backend_get_priv(backend);
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
// val = priv->vref_mv;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_OFFSET:
// val = 0;
    return IIO_VAL_INT;
    }
    return -EOPNOTSUPP;
    };
    static const struct iio_backend_ops sd_backend_ops = {
    .enable = iio_sd_mod_enable,
    .disable = iio_sd_mod_disable,
    .read_raw = iio_sd_mod_read,
    };
    static const struct iio_backend_info sd_backend_info = {
    .name = "sd-modulator",
    .ops = &sd_backend_ops,
    .caps = IIO_BACKEND_CAP_ENABLE,
    };
#[no_mangle]
unsafe extern "C" fn iio_sd_mod_register(pdev: *mut platform_device) -> c_int {
    static int iio_sd_mod_register(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct iio_dev *iio;
    iio = devm_iio_device_alloc(dev, 0);
    if (!iio)
    return -ENOMEM;
    iio.name = dev_name(dev);
    iio.info = &iio_sd_mod_iio_info;
    iio.modes = INDIO_BUFFER_HARDWARE;
    iio.num_channels = 1;
    iio.channels = &iio_sd_mod_ch;
    platform_set_drvdata(pdev, iio);
    return devm_iio_device_register(&pdev.dev, iio);
    }
#[no_mangle]
unsafe extern "C" fn iio_sd_mod_probe(pdev: *mut platform_device) -> c_int {
    static int iio_sd_mod_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct regulator *vref;
    struct iio_sd_backend_priv *priv;
    int ret;
// If sd modulator is not defined as an IIO backend device, fallback to legacy
    if (!device_property_present(dev, "#io-backend-cells"))
    return iio_sd_mod_register(pdev);
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
//
// Get regulator reference if any, but don't enable regulator right now.
// Rely on enable and disable callbacks to manage regulator power.
//
    vref = devm_regulator_get_optional(dev, "vref");
    if (IS_ERR(vref)) {
    if (PTR_ERR(vref) != -ENODEV)
    return dev_err_probe(dev, PTR_ERR(vref), "Failed to get vref\n");
    } else {
//
// Retrieve voltage right now, as regulator_get_voltage() provides it whatever
// the state of the regulator.
//
    ret = regulator_get_voltage(vref);
    if (ret < 0)
    return ret;
    priv.vref = vref;
    priv.vref_mv = ret / 1000;
    }
    return devm_iio_backend_register(&pdev.dev, &sd_backend_info, priv);
    };
    static const struct of_device_id sd_adc_of_match[] = {
    { .compatible = "sd-modulator" },
    { .compatible = "ads1201" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sd_adc_of_match);
    static struct platform_driver iio_sd_mod_adc = {
    .driver = {
    .name = "iio_sd_adc_mod",
    .of_match_table = sd_adc_of_match,
    },
    .probe = iio_sd_mod_probe,
    };
    module_platform_driver(iio_sd_mod_adc);
    MODULE_DESCRIPTION("Basic sigma delta modulator");
    MODULE_AUTHOR("Arnaud Pouliquen <arnaud.pouliquen@st.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_BACKEND");
