//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/as3722-poweroff.c
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
// Power off driver for ams AS3722 device.
//
// Copyright (c) 2013, NVIDIA CORPORATION.  All rights reserved.
//
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3722_poweroff {
    pub dev: *mut device,
    pub as3722: *mut as3722,
}

#[no_mangle]
unsafe extern "C" fn as3722_pm_power_off(data: *mut sys_off_data) -> c_int {
    static int as3722_pm_power_off(struct sys_off_data *data)
    {
    struct as3722_poweroff *as3722_pm_poweroff = data.cb_data;
    int ret;
    ret = as3722_update_bits(as3722_pm_poweroff.as3722,
    AS3722_RESET_CONTROL_REG, AS3722_POWER_OFF, AS3722_POWER_OFF);
    if (ret < 0)
    dev_err(as3722_pm_poweroff.dev,
    "RESET_CONTROL_REG update failed, %d\n", ret);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn as3722_poweroff_probe(pdev: *mut platform_device) -> c_int {
    static int as3722_poweroff_probe(struct platform_device *pdev)
    {
    struct as3722_poweroff *as3722_poweroff;
    struct device_node *np = pdev.dev.parent.of_node;
    if (!np)
    return -EINVAL;
    if (!of_property_read_bool(np, "ams,system-power-controller"))
    return 0;
    as3722_poweroff = devm_kzalloc(&pdev.dev, sizeof(*as3722_poweroff),
    GFP_KERNEL);
    if (!as3722_poweroff)
    return -ENOMEM;
    as3722_poweroff.as3722 = dev_get_drvdata(pdev.dev.parent);
    as3722_poweroff.dev = &pdev.dev;
    return devm_register_sys_off_handler(as3722_poweroff.dev,
    SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT,
    as3722_pm_power_off,
    as3722_poweroff);
    }
    static struct platform_driver as3722_poweroff_driver = {
    .driver = {
    .name = "as3722-power-off",
    },
    .probe = as3722_poweroff_probe,
    };
    module_platform_driver(as3722_poweroff_driver);
    MODULE_DESCRIPTION("Power off driver for ams AS3722 PMIC Device");
    MODULE_ALIAS("platform:as3722-power-off");
    MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
