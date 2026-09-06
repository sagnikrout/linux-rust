//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/msm-poweroff.c
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
// Copyright (c) 2013, The Linux Foundation. All rights reserved.
//

    static void __iomem *msm_ps_hold;
#[no_mangle]
unsafe extern "C" fn do_msm_poweroff(data: *mut sys_off_data) -> c_int {
    static int do_msm_poweroff(struct sys_off_data *data)
    {
    writel(0, msm_ps_hold);
    mdelay(10000);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn msm_restart_probe(pdev: *mut platform_device) -> c_int {
    static int msm_restart_probe(struct platform_device *pdev)
    {
    msm_ps_hold = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(msm_ps_hold))
    return PTR_ERR(msm_ps_hold);
    devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_RESTART,
    128, do_msm_poweroff, core::ptr::null_mut());
    devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT, do_msm_poweroff,
    core::ptr::null_mut());
    return 0;
    }
    static const struct of_device_id of_msm_restart_match[] = {
    { .compatible = "qcom,pshold", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_msm_restart_match);
    static struct platform_driver msm_restart_driver = {
    .probe = msm_restart_probe,
    .driver = {
    .name = "msm-restart",
    .of_match_table = of_match_ptr(of_msm_restart_match),
    },
    };
    builtin_platform_driver(msm_restart_driver);
