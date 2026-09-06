//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/tps65912-spi.c
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
// SPI access driver for TI TPS65912x PMICs
//
// Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
// Andrew F. Davis <afd@ti.com>
//
// Based on the TPS65218 driver and the previous TPS65912 driver by
// Margarita Olaya Cabrera <magi@slimlogic.co.uk>
//

    static const struct of_device_id tps65912_spi_of_match_table[] = {
    { .compatible = "ti,tps65912", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, tps65912_spi_of_match_table);
#[no_mangle]
unsafe extern "C" fn tps65912_spi_probe(spi: *mut spi_device) -> c_int {
    static int tps65912_spi_probe(struct spi_device *spi)
    {
    struct tps65912 *tps;
    tps = devm_kzalloc(&spi.dev, sizeof(*tps), GFP_KERNEL);
    if (!tps)
    return -ENOMEM;
    spi_set_drvdata(spi, tps);
    tps.dev = &spi.dev;
    tps.irq = spi.irq;
    tps.regmap = devm_regmap_init_spi(spi, &tps65912_regmap_config);
    if (IS_ERR(tps.regmap)) {
    dev_err(tps.dev, "Failed to initialize register map\n");
    return PTR_ERR(tps.regmap);
    }
    return tps65912_device_init(tps);
    }
    static const struct spi_device_id tps65912_spi_id_table[] = {
    { .name = "tps65912" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(spi, tps65912_spi_id_table);
    static struct spi_driver tps65912_spi_driver = {
    .driver		= {
    .name	= "tps65912",
    .of_match_table = tps65912_spi_of_match_table,
    },
    .probe		= tps65912_spi_probe,
    .id_table       = tps65912_spi_id_table,
    };
    module_spi_driver(tps65912_spi_driver);
    MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
    MODULE_DESCRIPTION("TPS65912x SPI Interface Driver");
    MODULE_LICENSE("GPL v2");
