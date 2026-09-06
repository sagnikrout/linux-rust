//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/tsc2005.c
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
// TSC2005 touchscreen driver
//
// Copyright (C) 2006-2010 Nokia Corporation
// Copyright (C) 2015 QWERTY Embedded Design
// Copyright (C) 2015 EMAC Inc.
//
// Based on original tsc2005.c by Lauri Leukkunen <lauri.leukkunen@nokia.com>
//

    static const struct input_id tsc2005_input_id = {
    .bustype = BUS_SPI,
    .product = 2005,
    };
#[no_mangle]
unsafe extern "C" fn tsc2005_cmd(dev: *mut device, cmd: u8) -> c_int {
    static int tsc2005_cmd(struct device *dev, u8 cmd)
    {
    let mut tx: u8 = TSC200X_CMD | TSC200X_CMD_12BIT | cmd;
    struct spi_transfer xfer = {
    .tx_buf         = &tx,
    .len            = 1,
    .bits_per_word  = 8,
    };
    struct spi_message msg;
    struct spi_device *spi = to_spi_device(dev);
    int error;
    spi_message_init(&msg);
    spi_message_add_tail(&xfer, &msg);
    error = spi_sync(spi, &msg);
    if (error) {
    dev_err(dev, "%s: failed, command: %x, spi error: %d\n",
    __func__, cmd, error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tsc2005_probe(spi: *mut spi_device) -> c_int {
    static int tsc2005_probe(struct spi_device *spi)
    {
    int error;
    spi.mode = SPI_MODE_0;
    spi.bits_per_word = 8;
    if (!spi.max_speed_hz)
    spi.max_speed_hz = TSC2005_SPI_MAX_SPEED_HZ;
    error = spi_setup(spi);
    if (error)
    return error;
    return tsc200x_probe(&spi.dev, spi.irq, &tsc2005_input_id,
    devm_regmap_init_spi(spi, &tsc200x_regmap_config),
    tsc2005_cmd);
    }

    static const struct of_device_id tsc2005_of_match[] = {
    { .compatible = "ti,tsc2005" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, tsc2005_of_match);

    static struct spi_driver tsc2005_driver = {
    .driver	= {
    .name		= "tsc2005",
    .dev_groups	= tsc200x_groups,
    .of_match_table	= of_match_ptr(tsc2005_of_match),
    .pm		= pm_sleep_ptr(&tsc200x_pm_ops),
    },
    .probe	= tsc2005_probe,
    };
    module_spi_driver(tsc2005_driver);
    MODULE_AUTHOR("Michael Welling <mwelling@ieee.org>");
    MODULE_DESCRIPTION("TSC2005 Touchscreen Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:tsc2005");
