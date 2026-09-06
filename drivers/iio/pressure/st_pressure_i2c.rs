//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/st_pressure_i2c.c
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
    static const struct acpi_device_id st_press_acpi_match[] = {
    {"SNO9210", LPS22HB},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, st_press_acpi_match);
    static const struct i2c_device_id st_press_id_table[] = {
    { .name = LPS001WP_PRESS_DEV_NAME, .driver_data = LPS001WP },
    { .name = LPS25H_PRESS_DEV_NAME, .driver_data = LPS25H },
    { .name = LPS331AP_PRESS_DEV_NAME, .driver_data = LPS331AP },
    { .name = LPS22HB_PRESS_DEV_NAME, .driver_data = LPS22HB },
    { .name = LPS33HW_PRESS_DEV_NAME, .driver_data = LPS33HW },
    { .name = LPS35HW_PRESS_DEV_NAME, .driver_data = LPS35HW },
    { .name = LPS22HH_PRESS_DEV_NAME, .driver_data = LPS22HH },
    { .name = LPS22DF_PRESS_DEV_NAME, .driver_data = LPS22DF },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, st_press_id_table);
#[no_mangle]
unsafe extern "C" fn st_press_i2c_probe(client: *mut i2c_client) -> c_int {
    static int st_press_i2c_probe(struct i2c_client *client)
    {
    const struct st_sensor_settings *settings;
    struct st_sensor_data *press_data;
    struct iio_dev *indio_dev;
    int ret;
    st_sensors_dev_name_probe(&client.dev, client.name, sizeof(client.name));
    settings = st_press_get_settings(client.name);
    if (!settings) {
    dev_err(&client.dev, "device name %s not recognized.\n",
    client.name);
    return -ENODEV;
    }
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*press_data));
    if (!indio_dev)
    return -ENOMEM;
    press_data = iio_priv(indio_dev);
    press_data.sensor_settings = (struct st_sensor_settings *)settings;
    ret = st_sensors_i2c_configure(indio_dev, client);
    if (ret < 0)
    return ret;
    ret = st_sensors_power_enable(indio_dev);
    if (ret)
    return ret;
    return st_press_common_probe(indio_dev);
    }
    static struct i2c_driver st_press_driver = {
    .driver = {
    .name = "st-press-i2c",
    .of_match_table = st_press_of_match,
    .acpi_match_table = st_press_acpi_match,
    },
    .probe = st_press_i2c_probe,
    .id_table = st_press_id_table,
    };
    module_i2c_driver(st_press_driver);
    MODULE_AUTHOR("Denis Ciocca <denis.ciocca@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics pressures i2c driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ST_SENSORS");
