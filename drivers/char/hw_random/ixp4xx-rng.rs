//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/ixp4xx-rng.c
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


// SPDX-License-Identifier: GPL-2.0
//
// drivers/char/hw_random/ixp4xx-rng.c
//
// RNG driver for Intel IXP4xx family of NPUs
//
// Author: Deepak Saxena <dsaxena@plexity.net>
//
// Copyright 2005 (c) MontaVista Software, Inc.
//
// Fixes by Michael Buesch
//

#[no_mangle]
unsafe extern "C" fn ixp4xx_rng_data_read(rng: *mut hwrng, buffer: *mut u32) -> c_int {
    static int ixp4xx_rng_data_read(struct hwrng *rng, u32 *buffer)
    {
    let mut rng_base: *mut void __iomem = (void __iomem *)rng.priv;
// buffer = __raw_readl(rng_base);
    return 4;
    }
    static struct hwrng ixp4xx_rng_ops = {
    .name		= "ixp4xx",
    .data_read	= ixp4xx_rng_data_read,
    };
#[no_mangle]
unsafe extern "C" fn ixp4xx_rng_probe(pdev: *mut platform_device) -> c_int {
    static int ixp4xx_rng_probe(struct platform_device *pdev)
    {
    void __iomem * rng_base;
    struct device *dev = &pdev.dev;
    if (!cpu_is_ixp46x()) /* includes IXP455 */
    return -ENOSYS;
    rng_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rng_base))
    return PTR_ERR(rng_base);
    ixp4xx_rng_ops.priv = (unsigned long)rng_base;
    return devm_hwrng_register(dev, &ixp4xx_rng_ops);
    }
    static const struct of_device_id ixp4xx_rng_of_match[] = {
    {
    .compatible = "intel,ixp46x-rng",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, ixp4xx_rng_of_match);
    static struct platform_driver ixp4xx_rng_driver = {
    .driver = {
    .name = "ixp4xx-hwrandom",
    .of_match_table = ixp4xx_rng_of_match,
    },
    .probe = ixp4xx_rng_probe,
    };
    module_platform_driver(ixp4xx_rng_driver);
    MODULE_AUTHOR("Deepak Saxena <dsaxena@plexity.net>");
    MODULE_DESCRIPTION("H/W Pseudo-Random Number Generator (RNG) driver for IXP45x/46x");
    MODULE_LICENSE("GPL");
