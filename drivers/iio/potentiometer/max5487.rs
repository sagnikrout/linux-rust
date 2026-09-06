//! Automatically rewritten from C to Rust
//! Source: drivers/iio/potentiometer/max5487.c
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
// max5487.c - Support for MAX5487, MAX5488, MAX5489 digital potentiometers
//
// Copyright (C) 2016 Cristina-Gabriela Moraru <cristina.moraru09@gmail.com>
//

// copy both wiper regs to NV regs

// copy both NV regs to wiper regs

pub const MAX5487_MAX_POS: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max5487_data {
    pub spi: *mut spi_device,
    pub kohms: c_int,
}

    .type = IIO_RESISTANCE,					\
    .indexed = 1,						\
    .output = 1,						\
    .channel = ch,						\
    .address = addr,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    }
    static const struct iio_chan_spec max5487_channels[] = {
    MAX5487_CHANNEL(0, MAX5487_WRITE_WIPER_A),
    MAX5487_CHANNEL(1, MAX5487_WRITE_WIPER_B),
    };
#[no_mangle]
unsafe extern "C" fn max5487_write_cmd(spi: *mut spi_device, cmd: u16) -> c_int {
    static int max5487_write_cmd(struct spi_device *spi, u16 cmd)
    {
    return spi_write(spi, (const void *) &cmd, sizeof(u16));
    }
    static int max5487_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max5487_data *data = iio_priv(indio_dev);
    if (mask != IIO_CHAN_INFO_SCALE)
    return -EINVAL;
// val = 1000 * data->kohms;
// val2 = MAX5487_MAX_POS;
    return IIO_VAL_FRACTIONAL;
    }
    static int max5487_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct max5487_data *data = iio_priv(indio_dev);
    if (mask != IIO_CHAN_INFO_RAW)
    return -EINVAL;
    if (val < 0 || val > MAX5487_MAX_POS)
    return -EINVAL;
    return max5487_write_cmd(data.spi, chan.address | val);
    }
    static const struct iio_info max5487_info = {
    .read_raw = max5487_read_raw,
    .write_raw = max5487_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn max5487_spi_probe(spi: *mut spi_device) -> c_int {
    static int max5487_spi_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct max5487_data *data;
    const struct spi_device_id *id = spi_get_device_id(spi);
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    spi_set_drvdata(spi, indio_dev);
    data = iio_priv(indio_dev);
    data.spi = spi;
    data.kohms = id.driver_data;
    indio_dev.info = &max5487_info;
    indio_dev.name = id.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = max5487_channels;
    indio_dev.num_channels = ARRAY_SIZE(max5487_channels);
// restore both wiper regs from NV regs
    ret = max5487_write_cmd(data.spi, MAX5487_COPY_NV_TO_AB);
    if (ret < 0)
    return ret;
    return iio_device_register(indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn max5487_spi_remove(spi: *mut spi_device) {
    static void max5487_spi_remove(struct spi_device *spi)
    {
    struct iio_dev *indio_dev = spi_get_drvdata(spi);
    int ret;
    iio_device_unregister(indio_dev);
// save both wiper regs to NV regs
    ret = max5487_write_cmd(spi, MAX5487_COPY_AB_TO_NV);
    if (ret)
    dev_warn(&spi.dev, "Failed to save wiper regs to NV regs\n");
    }
    static const struct spi_device_id max5487_id[] = {
    { .name = "MAX5487", .driver_data = 10 },
    { .name = "MAX5488", .driver_data = 50 },
    { .name = "MAX5489", .driver_data = 100 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, max5487_id);
    static const struct acpi_device_id max5487_acpi_match[] = {
    { "MAX5487", 10 },
    { "MAX5488", 50 },
    { "MAX5489", 100 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, max5487_acpi_match);
    static struct spi_driver max5487_driver = {
    .driver = {
    .name = "max5487",
    .acpi_match_table = max5487_acpi_match,
    },
    .id_table = max5487_id,
    .probe = max5487_spi_probe,
    .remove = max5487_spi_remove
    };
    module_spi_driver(max5487_driver);
    MODULE_AUTHOR("Cristina-Gabriela Moraru <cristina.moraru09@gmail.com>");
    MODULE_DESCRIPTION("max5487 SPI driver");
    MODULE_LICENSE("GPL v2");
