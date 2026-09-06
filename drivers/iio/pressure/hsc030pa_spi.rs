//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/hsc030pa_spi.c
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
// Honeywell TruStability HSC Series pressure/temperature sensor
//
// Copyright (c) 2023 Petre Rodan <petre.rodan@subdimension.ro>
//
// Datasheet: https://prod-edam.honeywell.com/content/dam/honeywell-edam/sps/siot/en-us/products/sensors/pressure-sensors/common/documents/sps-siot-spi-comms-digital-ouptu-pressure-sensors-tn-008202-3-en-ciid-45843.pdf
// Datasheet: https://prod-edam.honeywell.com/content/dam/honeywell-edam/sps/siot/en-us/products/sensors/pressure-sensors/common/documents/sps-siot-sleep-mode-technical-note-008286-1-en-ciid-155793.pdf
//

#[no_mangle]
unsafe extern "C" fn hsc_spi_recv(data: *mut hsc_data) -> c_int {
    static int hsc_spi_recv(struct hsc_data *data)
    {
    struct spi_device *spi = to_spi_device(data.dev);
    msleep_interruptible(HSC_RESP_TIME_MS);
    return spi_read(spi, data.buffer, HSC_REG_MEASUREMENT_RD_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn hsc_spi_probe(spi: *mut spi_device) -> c_int {
    static int hsc_spi_probe(struct spi_device *spi)
    {
    return hsc_common_probe(&spi.dev, hsc_spi_recv);
    }
    static const struct of_device_id hsc_spi_match[] = {
    { .compatible = "honeywell,hsc030pa" },
    { }
    };
    MODULE_DEVICE_TABLE(of, hsc_spi_match);
    static const struct spi_device_id hsc_spi_id[] = {
    { .name = "hsc030pa" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, hsc_spi_id);
    static struct spi_driver hsc_spi_driver = {
    .driver = {
    .name = "hsc030pa",
    .of_match_table = hsc_spi_match,
    },
    .probe = hsc_spi_probe,
    .id_table = hsc_spi_id,
    };
    module_spi_driver(hsc_spi_driver);
    MODULE_AUTHOR("Petre Rodan <petre.rodan@subdimension.ro>");
    MODULE_DESCRIPTION("Honeywell HSC and SSC pressure sensor spi driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_HONEYWELL_HSC030PA");
