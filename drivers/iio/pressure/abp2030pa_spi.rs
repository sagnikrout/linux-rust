//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/abp2030pa_spi.c
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
// Honeywell ABP2 series pressure sensor driver
//
// Copyright (c) 2025 Petre Rodan <petre.rodan@subdimension.ro>
//

#[no_mangle]
unsafe extern "C" fn abp2_spi_xfer(data: *mut abp2_data, cmd: u8, nbytes: u8) -> c_int {
    static int abp2_spi_xfer(struct abp2_data *data, u8 cmd, u8 nbytes)
    {
    struct spi_device *spi = to_spi_device(data.dev);
    let mut xfer: spi_transfer = { };
    if (nbytes > ABP2_MEASUREMENT_RD_SIZE)
    return -EOVERFLOW;
    data.tx_buf[0] = cmd;
    xfer.tx_buf = data.tx_buf;
    xfer.rx_buf = data.rx_buf;
    xfer.len = nbytes;
    return spi_sync_transfer(spi, &xfer, 1);
    }
    static const struct abp2_ops abp2_spi_ops = {
    .read = abp2_spi_xfer,
    .write = abp2_spi_xfer,
    };
#[no_mangle]
unsafe extern "C" fn abp2_spi_probe(spi: *mut spi_device) -> c_int {
    static int abp2_spi_probe(struct spi_device *spi)
    {
    return abp2_common_probe(&spi.dev, &abp2_spi_ops, spi.irq);
    }
    static const struct of_device_id abp2_spi_match[] = {
    { .compatible = "honeywell,abp2030pa" },
    { }
    };
    MODULE_DEVICE_TABLE(of, abp2_spi_match);
    static const struct spi_device_id abp2_spi_id[] = {
    { .name = "abp2030pa" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, abp2_spi_id);
    static struct spi_driver abp2_spi_driver = {
    .driver = {
    .name = "abp2030pa",
    .of_match_table = abp2_spi_match,
    },
    .probe = abp2_spi_probe,
    .id_table = abp2_spi_id,
    };
    module_spi_driver(abp2_spi_driver);
    MODULE_AUTHOR("Petre Rodan <petre.rodan@subdimension.ro>");
    MODULE_DESCRIPTION("Honeywell ABP2 pressure sensor spi driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_HONEYWELL_ABP2030PA");
