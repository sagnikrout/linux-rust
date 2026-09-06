//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/st_pressure_spi.c
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
// STMicroelectronics pressures driver
//
// Copyright 2013 STMicroelectronics Inc.
//
// Denis Ciocca <denis.ciocca@st.com>
//

//
// For new single-chip sensors use <device_name> as compatible string.
// For old single-chip devices keep <device_name>-press to maintain
// compatibility
//
    static const struct of_device_id st_press_of_match[] = {
    {
    .compatible = "st,lps001wp-press",
    .data = LPS001WP_PRESS_DEV_NAME,
    },
    {
    .compatible = "st,lps25h-press",
    .data = LPS25H_PRESS_DEV_NAME,
    },
    {
    .compatible = "st,lps331ap-press",
    .data = LPS331AP_PRESS_DEV_NAME,
    },
    {
    .compatible = "st,lps22hb-press",
    .data = LPS22HB_PRESS_DEV_NAME,
    },
    {
    .compatible = "st,lps33hw",
    .data = LPS33HW_PRESS_DEV_NAME,
    },
    {
    .compatible = "st,lps35hw",
    .data = LPS35HW_PRESS_DEV_NAME,
    },
    {
    .compatible = "st,lps22hh",
    .data = LPS22HH_PRESS_DEV_NAME,
    },
    {
    .compatible = "st,lps22df",
    .data = LPS22DF_PRESS_DEV_NAME,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, st_press_of_match);
#[no_mangle]
unsafe extern "C" fn st_press_spi_probe(spi: *mut spi_device) -> c_int {
    static int st_press_spi_probe(struct spi_device *spi)
    {
    const struct st_sensor_settings *settings;
    struct st_sensor_data *press_data;
    struct iio_dev *indio_dev;
    int err;
    st_sensors_dev_name_probe(&spi.dev, spi.modalias, sizeof(spi.modalias));
    settings = st_press_get_settings(spi.modalias);
    if (!settings) {
    dev_err(&spi.dev, "device name %s not recognized.\n",
    spi.modalias);
    return -ENODEV;
    }
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*press_data));
    if (!indio_dev)
    return -ENOMEM;
    press_data = iio_priv(indio_dev);
    press_data.sensor_settings = (struct st_sensor_settings *)settings;
    err = st_sensors_spi_configure(indio_dev, spi);
    if (err < 0)
    return err;
    err = st_sensors_power_enable(indio_dev);
    if (err)
    return err;
    return st_press_common_probe(indio_dev);
    }
    static const struct spi_device_id st_press_id_table[] = {
    { .name = LPS001WP_PRESS_DEV_NAME },
    { .name = LPS25H_PRESS_DEV_NAME },
    { .name = LPS331AP_PRESS_DEV_NAME },
    { .name = LPS22HB_PRESS_DEV_NAME },
    { .name = LPS33HW_PRESS_DEV_NAME },
    { .name = LPS35HW_PRESS_DEV_NAME },
    { .name = LPS22HH_PRESS_DEV_NAME },
    { .name = LPS22DF_PRESS_DEV_NAME },
    { .name = "lps001wp-press" },
    { .name = "lps25h-press" },
    { .name = "lps331ap-press" },
    { .name = "lps22hb-press" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, st_press_id_table);
    static struct spi_driver st_press_driver = {
    .driver = {
    .name = "st-press-spi",
    .of_match_table = st_press_of_match,
    },
    .probe = st_press_spi_probe,
    .id_table = st_press_id_table,
    };
    module_spi_driver(st_press_driver);
    MODULE_AUTHOR("Denis Ciocca <denis.ciocca@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics pressures spi driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ST_SENSORS");
