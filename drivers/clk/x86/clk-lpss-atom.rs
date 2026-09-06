//! Automatically rewritten from C to Rust
//! Source: drivers/clk/x86/clk-lpss-atom.c
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
// Intel Low Power Subsystem clocks.
//
// Copyright (C) 2013, Intel Corporation
// Authors: Mika Westerberg <mika.westerberg@linux.intel.com>
// Heikki Krogerus <heikki.krogerus@linux.intel.com>
//

#[no_mangle]
unsafe extern "C" fn lpss_atom_clk_probe(pdev: *mut platform_device) -> c_int {
    static int lpss_atom_clk_probe(struct platform_device *pdev)
    {
    struct lpss_clk_data *drvdata;
    struct clk *clk;
    drvdata = devm_kzalloc(&pdev.dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
// LPSS free running clock
    drvdata.name = "lpss_clk";
    clk = clk_register_fixed_rate(&pdev.dev, drvdata.name, core::ptr::null_mut(),
    0, 100000000);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    drvdata.clk = clk;
    platform_set_drvdata(pdev, drvdata);
    return 0;
    }
    static struct platform_driver lpss_atom_clk_driver = {
    .driver = {
    .name = "clk-lpss-atom",
    },
    .probe = lpss_atom_clk_probe,
    };
#[no_mangle]
pub unsafe extern "C" fn lpss_atom_clk_init() -> int __init {
    int __init lpss_atom_clk_init(void)
    {
    return platform_driver_register(&lpss_atom_clk_driver);
    }
