//! Automatically rewritten from C to Rust
//! Source: drivers/bus/sun50i-de2.c
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
// Allwinner A64 Display Engine 2.0 Bus Driver
//
// Copyright (C) 2018 Icenowy Zheng <icenowy@aosc.io>
//

#[no_mangle]
unsafe extern "C" fn sun50i_de2_bus_probe(pdev: *mut platform_device) -> c_int {
    static int sun50i_de2_bus_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    int ret;
    ret = sunxi_sram_claim(&pdev.dev);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Couldn't map SRAM to device\n");
    of_platform_populate(np, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun50i_de2_bus_remove(pdev: *mut platform_device) {
    static void sun50i_de2_bus_remove(struct platform_device *pdev)
    {
    sunxi_sram_release(&pdev.dev);
    }
    static const struct of_device_id sun50i_de2_bus_of_match[] = {
    { .compatible = "allwinner,sun50i-a64-de2", },
    { /* sentinel */ }
    };
    static struct platform_driver sun50i_de2_bus_driver = {
    .probe = sun50i_de2_bus_probe,
    .remove = sun50i_de2_bus_remove,
    .driver = {
    .name = "sun50i-de2-bus",
    .of_match_table = sun50i_de2_bus_of_match,
    },
    };
    builtin_platform_driver(sun50i_de2_bus_driver);
