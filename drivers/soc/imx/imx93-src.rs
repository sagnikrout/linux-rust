//! Automatically rewritten from C to Rust
//! Source: drivers/soc/imx/imx93-src.c
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
// Copyright 2022 NXP
//

#[no_mangle]
unsafe extern "C" fn imx93_src_probe(pdev: *mut platform_device) -> c_int {
    static int imx93_src_probe(struct platform_device *pdev)
    {
    return devm_of_platform_populate(&pdev.dev);
    }
    static const struct of_device_id imx93_src_ids[] = {
    { .compatible = "fsl,imx93-src" },
    { }
    };
    MODULE_DEVICE_TABLE(of, imx93_src_ids);
    static struct platform_driver imx93_src_driver = {
    .driver = {
    .name	= "imx93_src",
    .of_match_table = imx93_src_ids,
    },
    .probe = imx93_src_probe,
    };
    module_platform_driver(imx93_src_driver);
    MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
    MODULE_DESCRIPTION("NXP i.MX93 src driver");
    MODULE_LICENSE("GPL");
