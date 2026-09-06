//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/st_accel_spi.c
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
// STMicroelectronics accelerometers driver
//
// Copyright 2012-2013 STMicroelectronics Inc.
//
// Denis Ciocca <denis.ciocca@st.com>
//

//
// For new single-chip sensors use <device_name> as compatible string.
// For old single-chip devices keep <device_name>-accel to maintain
// compatibility
//
    static const struct of_device_id st_accel_of_match[] = {
    {
// An older compatible
    .compatible = "st,lis302dl-spi",
    .data = LIS3LV02DL_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis3lv02dl-accel",
    .data = LIS3LV02DL_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis3dh-accel",
    .data = LIS3DH_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lsm330d-accel",
    .data = LSM330D_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lsm330dl-accel",
    .data = LSM330DL_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lsm330dlc-accel",
    .data = LSM330DLC_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis331dlh-accel",
    .data = LIS331DLH_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lsm330-accel",
    .data = LSM330_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lsm303agr-accel",
    .data = LSM303AGR_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis2dh12-accel",
    .data = LIS2DH12_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis2ds12",
    .data = LIS2DS12_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis3l02dq",
    .data = LIS3L02DQ_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lng2dm-accel",
    .data = LNG2DM_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,h3lis331dl-accel",
    .data = H3LIS331DL_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis331dl-accel",
    .data = LIS331DL_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis2dw12",
    .data = LIS2DW12_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis3dhh",
    .data = LIS3DHH_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis3de",
    .data = LIS3DE_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lis302dl",
    .data = LIS302DL_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,lsm303c-accel",
    .data = LSM303C_ACCEL_DEV_NAME,
    },
    {
    .compatible = "st,iis328dq",
    .data = IIS328DQ_ACCEL_DEV_NAME,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, st_accel_of_match);
#[no_mangle]
unsafe extern "C" fn st_accel_spi_probe(spi: *mut spi_device) -> c_int {
    static int st_accel_spi_probe(struct spi_device *spi)
    {
    const struct st_sensor_settings *settings;
    struct st_sensor_data *adata;
    struct iio_dev *indio_dev;
    int err;
    st_sensors_dev_name_probe(&spi.dev, spi.modalias, sizeof(spi.modalias));
    settings = st_accel_get_settings(spi.modalias);
    if (!settings) {
    dev_err(&spi.dev, "device name %s not recognized.\n",
    spi.modalias);
    return -ENODEV;
    }
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*adata));
    if (!indio_dev)
    return -ENOMEM;
    adata = iio_priv(indio_dev);
    adata.sensor_settings = (struct st_sensor_settings *)settings;
    err = st_sensors_spi_configure(indio_dev, spi);
    if (err < 0)
    return err;
    err = st_sensors_power_enable(indio_dev);
    if (err)
    return err;
    return st_accel_common_probe(indio_dev);
    }
    static const struct spi_device_id st_accel_id_table[] = {
    { .name = LIS3DH_ACCEL_DEV_NAME },
    { .name = LSM330D_ACCEL_DEV_NAME },
    { .name = LSM330DL_ACCEL_DEV_NAME },
    { .name = LSM330DLC_ACCEL_DEV_NAME },
    { .name = LIS331DLH_ACCEL_DEV_NAME },
    { .name = LSM330_ACCEL_DEV_NAME },
    { .name = LSM303AGR_ACCEL_DEV_NAME },
    { .name = LIS2DH12_ACCEL_DEV_NAME },
    { .name = LIS2DS12_ACCEL_DEV_NAME },
    { .name = LIS3L02DQ_ACCEL_DEV_NAME },
    { .name = LNG2DM_ACCEL_DEV_NAME },
    { .name = H3LIS331DL_ACCEL_DEV_NAME },
    { .name = LIS331DL_ACCEL_DEV_NAME },
    { .name = LIS3LV02DL_ACCEL_DEV_NAME },
    { .name = LIS2DW12_ACCEL_DEV_NAME },
    { .name = LIS3DHH_ACCEL_DEV_NAME },
    { .name = LIS3DE_ACCEL_DEV_NAME },
    { .name = LIS302DL_ACCEL_DEV_NAME },
    { .name = LSM303C_ACCEL_DEV_NAME },
    { .name = IIS328DQ_ACCEL_DEV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(spi, st_accel_id_table);
    static struct spi_driver st_accel_driver = {
    .driver = {
    .name = "st-accel-spi",
    .of_match_table = st_accel_of_match,
    },
    .probe = st_accel_spi_probe,
    .id_table = st_accel_id_table,
    };
    module_spi_driver(st_accel_driver);
    MODULE_AUTHOR("Denis Ciocca <denis.ciocca@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics accelerometers spi driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ST_SENSORS");
