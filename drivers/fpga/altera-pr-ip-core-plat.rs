//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/altera-pr-ip-core-plat.c
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
// Driver for Altera Partial Reconfiguration IP Core
//
// Copyright (C) 2016-2017 Intel Corporation
//
// Based on socfpga-a10.c Copyright (C) 2015-2016 Altera Corporation
// by Alan Tull <atull@opensource.altera.com>
//

#[no_mangle]
unsafe extern "C" fn alt_pr_platform_probe(pdev: *mut platform_device) -> c_int {
    static int alt_pr_platform_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    void __iomem *reg_base;
// First mmio base is for register access
    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg_base))
    return PTR_ERR(reg_base);
    return alt_pr_register(dev, reg_base);
    }
    static const struct of_device_id alt_pr_of_match[] = {
    { .compatible = "altr,a10-pr-ip", },
    {},
    };
    MODULE_DEVICE_TABLE(of, alt_pr_of_match);
    static struct platform_driver alt_pr_platform_driver = {
    .probe = alt_pr_platform_probe,
    .driver = {
    .name	= "alt_a10_pr_ip",
    .of_match_table = alt_pr_of_match,
    },
    };
    module_platform_driver(alt_pr_platform_driver);
    MODULE_AUTHOR("Matthew Gerlach <matthew.gerlach@linux.intel.com>");
    MODULE_DESCRIPTION("Altera Partial Reconfiguration IP Platform Driver");
    MODULE_LICENSE("GPL v2");
