//! Automatically rewritten from C to Rust
//! Source: drivers/iio/common/st_sensors/st_sensors_spi.c
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
// STMicroelectronics sensors spi library driver
//
// Copyright 2012-2013 STMicroelectronics Inc.
//
// Denis Ciocca <denis.ciocca@st.com>
//

pub const ST_SENSORS_SPI_MULTIREAD: c_uint = 0xc0;
    static const struct regmap_config st_sensors_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    static const struct regmap_config st_sensors_spi_regmap_multiread_bit_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .read_flag_mask = ST_SENSORS_SPI_MULTIREAD,
    };
//
// st_sensors_is_spi_3_wire() - check if SPI 3-wire mode has been selected
// @spi: spi device reference.
//
// Return: true if SPI 3-wire mode is selected, false otherwise.
//
#[no_mangle]
unsafe extern "C" fn st_sensors_is_spi_3_wire(spi: *mut spi_device) -> bool {
    static bool st_sensors_is_spi_3_wire(struct spi_device *spi)
    {
    struct st_sensors_platform_data *pdata;
    struct device *dev = &spi.dev;
    if (device_property_read_bool(dev, "spi-3wire"))
    return true;
    pdata = dev_get_platdata(dev);
    if (pdata && pdata.spi_3wire)
    return true;
    return false;
    }
//
// st_sensors_configure_spi_3_wire() - configure SPI 3-wire if needed
// @spi: spi device reference.
// @settings: sensor specific settings reference.
//
// Return: 0 on success, else a negative error code.
//
    static int st_sensors_configure_spi_3_wire(struct spi_device *spi,
    struct st_sensor_settings *settings)
    {
    if (settings.sim.addr) {
    u8 buffer[] = {
    settings.sim.addr,
    settings.sim.value
    };
    return spi_write(spi, buffer, 2);
    }
    return 0;
    }
//
// st_sensors_spi_configure() - configure SPI interface
// @indio_dev: IIO device reference.
// @spi: spi device reference.
//
// Return: 0 on success, else a negative error code.
//
    int st_sensors_spi_configure(struct iio_dev *indio_dev,
    struct spi_device *spi)
    {
    struct st_sensor_data *sdata = iio_priv(indio_dev);
    const struct regmap_config *config;
    int err;
    if (st_sensors_is_spi_3_wire(spi)) {
    err = st_sensors_configure_spi_3_wire(spi,
    sdata.sensor_settings);
    if (err < 0)
    return err;
    }
    if (sdata.sensor_settings.multi_read_bit)
    config = &st_sensors_spi_regmap_multiread_bit_config;
    else
    config = &st_sensors_spi_regmap_config;
    sdata.regmap = devm_regmap_init_spi(spi, config);
    if (IS_ERR(sdata.regmap)) {
    dev_err(&spi.dev, "Failed to register spi regmap (%ld)\n",
    PTR_ERR(sdata.regmap));
    return PTR_ERR(sdata.regmap);
    }
    spi_set_drvdata(spi, indio_dev);
    indio_dev.name = spi.modalias;
    sdata.irq = spi.irq;
    return 0;
    }
    EXPORT_SYMBOL_NS(st_sensors_spi_configure, "IIO_ST_SENSORS");
    MODULE_AUTHOR("Denis Ciocca <denis.ciocca@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics ST-sensors spi driver");
    MODULE_LICENSE("GPL v2");
