//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/mprls0025pa_spi.c
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
// MPRLS0025PA - Honeywell MicroPressure MPR series SPI sensor driver
//
// Copyright (c) 2024 Petre Rodan <petre.rodan@subdimension.ro>
//
// Data sheet:
// https://prod-edam.honeywell.com/content/dam/honeywell-edam/sps/siot/en-us/products/sensors/pressure-sensors/board-mount-pressure-sensors/micropressure-mpr-series/documents/sps-siot-mpr-series-datasheet-32332628-ciid-172626.pdf
//

#[no_mangle]
unsafe extern "C" fn mpr_spi_xfer(data: *mut mpr_data, cmd: u8, pkt_len: u8) -> c_int {
    static int mpr_spi_xfer(struct mpr_data *data, const u8 cmd, const u8 pkt_len)
    {
    struct spi_device *spi = to_spi_device(data.dev);
    struct spi_transfer xfers[2] = { };
    if (pkt_len > MPR_MEASUREMENT_RD_SIZE)
    return -EOVERFLOW;
    data.tx_buf[0] = cmd;
//
// Dummy transfer with no data, just cause a 2.5us+ delay between the CS assert
// and the first clock edge as per the datasheet tHDSS timing requirement.
//
    xfers[0].delay.value = 2500;
    xfers[0].delay.unit = SPI_DELAY_UNIT_NSECS;
    xfers[1].tx_buf = data.tx_buf;
    xfers[1].rx_buf = data.rx_buf;
    xfers[1].len = pkt_len;
    return spi_sync_transfer(spi, xfers, ARRAY_SIZE(xfers));
    }
    static const struct mpr_ops mpr_spi_ops = {
    .read = mpr_spi_xfer,
    .write = mpr_spi_xfer,
    };
#[no_mangle]
unsafe extern "C" fn mpr_spi_probe(spi: *mut spi_device) -> c_int {
    static int mpr_spi_probe(struct spi_device *spi)
    {
    return mpr_common_probe(&spi.dev, &mpr_spi_ops, spi.irq);
    }
    static const struct of_device_id mpr_spi_match[] = {
    { .compatible = "honeywell,mprls0025pa" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mpr_spi_match);
    static const struct spi_device_id mpr_spi_id[] = {
    { .name = "mprls0025pa" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, mpr_spi_id);
    static struct spi_driver mpr_spi_driver = {
    .driver = {
    .name = "mprls0025pa",
    .of_match_table = mpr_spi_match,
    },
    .probe = mpr_spi_probe,
    .id_table = mpr_spi_id,
    };
    module_spi_driver(mpr_spi_driver);
    MODULE_AUTHOR("Petre Rodan <petre.rodan@subdimension.ro>");
    MODULE_DESCRIPTION("Honeywell MPR pressure sensor spi driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_HONEYWELL_MPRLS0025PA");
