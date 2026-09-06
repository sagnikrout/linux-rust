//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/ad714x-spi.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AD714X CapTouch Programmable Controller driver (SPI bus)
//
// Copyright 2009-2011 Analog Devices Inc.
//

pub const AD714x_SPI_CMD_PREFIX: c_uint = 0xE000   /* bits 15:11 */;

    static int ad714x_spi_read(struct ad714x_chip *chip,
    unsigned short reg, unsigned short *data, size_t len)
    {
    struct spi_device *spi = to_spi_device(chip.dev);
    struct spi_message message;
    struct spi_transfer xfer[2];
    int i;
    int error;
    spi_message_init(&message);
    memset(xfer, 0, sizeof(xfer));
    chip.xfer_buf[0] = cpu_to_be16(AD714x_SPI_CMD_PREFIX |
    AD714x_SPI_READ | reg);
    xfer[0].tx_buf = &chip.xfer_buf[0];
    xfer[0].len = sizeof(chip.xfer_buf[0]);
    spi_message_add_tail(&xfer[0], &message);
    xfer[1].rx_buf = &chip.xfer_buf[1];
    xfer[1].len = sizeof(chip.xfer_buf[1]) * len;
    spi_message_add_tail(&xfer[1], &message);
    error = spi_sync(spi, &message);
    if (unlikely(error)) {
    dev_err(chip.dev, "SPI read error: %d\n", error);
    return error;
    }
    for (i = 0; i < len; i++)
    data[i] = be16_to_cpu(chip.xfer_buf[i + 1]);
    return 0;
    }
    static int ad714x_spi_write(struct ad714x_chip *chip,
    unsigned short reg, unsigned short data)
    {
    struct spi_device *spi = to_spi_device(chip.dev);
    int error;
    chip.xfer_buf[0] = cpu_to_be16(AD714x_SPI_CMD_PREFIX | reg);
    chip.xfer_buf[1] = cpu_to_be16(data);
    error = spi_write(spi, (u8 *)chip.xfer_buf,
    2 * sizeof(*chip.xfer_buf));
    if (unlikely(error)) {
    dev_err(chip.dev, "SPI write error: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad714x_spi_probe(spi: *mut spi_device) -> c_int {
    static int ad714x_spi_probe(struct spi_device *spi)
    {
    struct ad714x_chip *chip;
    int err;
    spi.bits_per_word = 8;
    err = spi_setup(spi);
    if (err < 0)
    return err;
    chip = ad714x_probe(&spi.dev, BUS_SPI, spi.irq,
    ad714x_spi_read, ad714x_spi_write);
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    spi_set_drvdata(spi, chip);
    return 0;
    }
    static struct spi_driver ad714x_spi_driver = {
    .driver = {
    .name	= "ad714x_captouch",
    .pm	= pm_sleep_ptr(&ad714x_pm),
    },
    .probe		= ad714x_spi_probe,
    };
    module_spi_driver(ad714x_spi_driver);
    MODULE_DESCRIPTION("Analog Devices AD714X Capacitance Touch Sensor SPI Bus Driver");
    MODULE_AUTHOR("Barry Song <21cnbao@gmail.com>");
    MODULE_LICENSE("GPL");
