//! Automatically rewritten from C to Rust
//! Source: drivers/soc/sophgo/sg2044-topsys.c
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
// Sophgo SG2044 multi-function system controller driver
//
// Copyright (C) 2025 Inochi Amaoto <inochiama@gmail.com>
//

    static const struct mfd_cell sg2044_topsys_subdev[] = {
    {
    .name = "sg2044-pll",
    },
    };
#[no_mangle]
unsafe extern "C" fn sg2044_topsys_probe(pdev: *mut platform_device) -> c_int {
    static int sg2044_topsys_probe(struct platform_device *pdev)
    {
    return devm_mfd_add_devices(&pdev.dev, PLATFORM_DEVID_AUTO,
    sg2044_topsys_subdev,
    ARRAY_SIZE(sg2044_topsys_subdev),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
    static const struct of_device_id sg2044_topsys_of_match[] = {
    { .compatible = "sophgo,sg2044-top-syscon" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sg2044_topsys_of_match);
    static struct platform_driver sg2044_topsys_driver = {
    .probe = sg2044_topsys_probe,
    .driver = {
    .name = "sg2044-topsys",
    .of_match_table = sg2044_topsys_of_match,
    },
    };
    module_platform_driver(sg2044_topsys_driver);
    MODULE_AUTHOR("Inochi Amaoto <inochiama@gmail.com>");
    MODULE_DESCRIPTION("Sophgo SG2044 multi-function system controller driver");
    MODULE_LICENSE("GPL");
