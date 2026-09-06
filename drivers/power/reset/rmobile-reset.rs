//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/rmobile-reset.c
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
// Renesas R-Mobile Reset Driver
//
// Copyright (C) 2014 Glider bvba
//

// SYSC Register Bank 2
pub const RESCNT2: c_uint = 0x20		/* Reset Control Register 2 */;
// Reset Control Register 2
pub const RESCNT2_PRES: c_uint = 0x80000000	/* Soft power-on reset */;
#[no_mangle]
unsafe extern "C" fn rmobile_reset_handler(data: *mut sys_off_data) -> c_int {
    static int rmobile_reset_handler(struct sys_off_data *data)
    {
    void __iomem *sysc_base2 = (void __iomem *)data.cb_data;
// Let's assume we have acquired the HPB semaphore
    writel(RESCNT2_PRES, sysc_base2 + RESCNT2);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn rmobile_reset_probe(pdev: *mut platform_device) -> c_int {
    static int rmobile_reset_probe(struct platform_device *pdev)
    {
    void __iomem *sysc_base2;
    int error;
    sysc_base2 = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(sysc_base2))
    return PTR_ERR(sysc_base2);
    error = devm_register_sys_off_handler(&pdev.dev,
    SYS_OFF_MODE_RESTART,
    SYS_OFF_PRIO_HIGH,
    rmobile_reset_handler,
    ( void *)sysc_base2);
    if (error) {
    dev_err(&pdev.dev,
    "cannot register restart handler (err=%d)\n", error);
    return error;
    }
    return 0;
    }
    static const struct of_device_id rmobile_reset_of_match[] = {
    { .compatible = "renesas,sysc-rmobile", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rmobile_reset_of_match);
    static struct platform_driver rmobile_reset_driver = {
    .probe = rmobile_reset_probe,
    .driver = {
    .name = "rmobile_reset",
    .of_match_table = rmobile_reset_of_match,
    },
    };
    module_platform_driver(rmobile_reset_driver);
    MODULE_DESCRIPTION("Renesas R-Mobile Reset Driver");
    MODULE_AUTHOR("Geert Uytterhoeven <geert+renesas@glider.be>");
    MODULE_LICENSE("GPL v2");
