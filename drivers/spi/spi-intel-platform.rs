//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-intel-platform.c
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
// Intel PCH/PCU SPI flash platform driver.
//
// Copyright (C) 2016 - 2022, Intel Corporation
// Author: Mika Westerberg <mika.westerberg@linux.intel.com>
//

#[no_mangle]
unsafe extern "C" fn intel_spi_platform_probe(pdev: *mut platform_device) -> c_int {
    static int intel_spi_platform_probe(struct platform_device *pdev)
    {
    struct intel_spi_boardinfo *info;
    void __iomem *base;
    info = dev_get_platdata(&pdev.dev);
    if (!info)
    return -EINVAL;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    return intel_spi_probe(&pdev.dev, base, info);
    }
    static struct platform_driver intel_spi_platform_driver = {
    .probe = intel_spi_platform_probe,
    .driver = {
    .name = "intel-spi",
    .dev_groups = intel_spi_groups,
    },
    };
    module_platform_driver(intel_spi_platform_driver);
    MODULE_DESCRIPTION("Intel PCH/PCU SPI flash platform driver");
    MODULE_AUTHOR("Mika Westerberg <mika.westerberg@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:intel-spi");
