//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/siemens/simatic-ipc-batt-elkhartlake.c
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
// Siemens SIMATIC IPC driver for CMOS battery monitoring
//
// Copyright (c) Siemens AG, 2023
//
// Authors:
// Henning Schild <henning.schild@siemens.com>
//

    static struct gpiod_lookup_table simatic_ipc_batt_gpio_table_bx_21a = {
    .table = {
    GPIO_LOOKUP_IDX("INTC1020:04", 18, core::ptr::null_mut(), 0, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("INTC1020:04", 19, core::ptr::null_mut(), 1, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("INTC1020:01", 66, core::ptr::null_mut(), 2, GPIO_ACTIVE_HIGH),
    {} /* Terminating entry */
    },
    };
#[no_mangle]
unsafe extern "C" fn simatic_ipc_batt_elkhartlake_remove(pdev: *mut platform_device) {
    static void simatic_ipc_batt_elkhartlake_remove(struct platform_device *pdev)
    {
    simatic_ipc_batt_remove(pdev, &simatic_ipc_batt_gpio_table_bx_21a);
    }
#[no_mangle]
unsafe extern "C" fn simatic_ipc_batt_elkhartlake_probe(pdev: *mut platform_device) -> c_int {
    static int simatic_ipc_batt_elkhartlake_probe(struct platform_device *pdev)
    {
    return simatic_ipc_batt_probe(pdev, &simatic_ipc_batt_gpio_table_bx_21a);
    }
    static struct platform_driver simatic_ipc_batt_driver = {
    .probe = simatic_ipc_batt_elkhartlake_probe,
    .remove = simatic_ipc_batt_elkhartlake_remove,
    .driver = {
    .name = KBUILD_MODNAME,
    },
    };
    module_platform_driver(simatic_ipc_batt_driver);
    MODULE_DESCRIPTION("CMOS Battery monitoring for Simatic IPCs based on Elkhart Lake GPIO");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" KBUILD_MODNAME);
    MODULE_SOFTDEP("pre: simatic-ipc-batt platform:elkhartlake-pinctrl");
    MODULE_AUTHOR("Henning Schild <henning.schild@siemens.com>");
