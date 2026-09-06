//! Automatically rewritten from C to Rust
//! Source: drivers/bus/ti-pwmss.c
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
// TI PWM Subsystem driver
//
// Copyright (C) 2012 Texas Instruments Incorporated - http://www.ti.com
//

    static const struct of_device_id pwmss_of_match[] = {
    { .compatible	= "ti,am33xx-pwmss" },
    {},
    };
    MODULE_DEVICE_TABLE(of, pwmss_of_match);
#[no_mangle]
unsafe extern "C" fn pwmss_probe(pdev: *mut platform_device) -> c_int {
    static int pwmss_probe(struct platform_device *pdev)
    {
    int ret;
    struct device_node *node = pdev.dev.of_node;
    pm_runtime_enable(&pdev.dev);
// Populate all the child nodes here...
    ret = of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    if (ret)
    dev_err(&pdev.dev, "no child node found\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pwmss_remove(pdev: *mut platform_device) {
    static void pwmss_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    }
    static struct platform_driver pwmss_driver = {
    .driver	= {
    .name	= "pwmss",
    .of_match_table	= pwmss_of_match,
    },
    .probe	= pwmss_probe,
    .remove	= pwmss_remove,
    };
    module_platform_driver(pwmss_driver);
    MODULE_DESCRIPTION("PWM Subsystem driver");
    MODULE_AUTHOR("Texas Instruments");
    MODULE_LICENSE("GPL");
