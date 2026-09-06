//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/spacemit-p1-reboot.c
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
// Copyright (C) 2025 by Aurelien Jarno
//

// Power Control Register 2
pub const PWR_CTRL2: c_uint = 0x7e;

#[no_mangle]
unsafe extern "C" fn spacemit_p1_pwroff_handler(data: *mut sys_off_data) -> c_int {
    static int spacemit_p1_pwroff_handler(struct sys_off_data *data)
    {
    struct regmap *regmap = data.cb_data;
    int ret;
// Put the PMIC into shutdown state
    ret = regmap_set_bits(regmap, PWR_CTRL2, PWR_CTRL2_SHUTDOWN);
    if (ret) {
    dev_err(data.dev, "shutdown failed: %d\n", ret);
    return notifier_from_errno(ret);
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_p1_restart_handler(data: *mut sys_off_data) -> c_int {
    static int spacemit_p1_restart_handler(struct sys_off_data *data)
    {
    struct regmap *regmap = data.cb_data;
    int ret;
// Put the PMIC into reset state
    ret = regmap_set_bits(regmap, PWR_CTRL2, PWR_CTRL2_RST);
    if (ret) {
    dev_err(data.dev, "restart failed: %d\n", ret);
    return notifier_from_errno(ret);
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_p1_reboot_probe(pdev: *mut platform_device) -> c_int {
    static int spacemit_p1_reboot_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct regmap *regmap;
    int ret;
    regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!regmap)
    return -ENODEV;
    ret = devm_register_power_off_handler(dev, &spacemit_p1_pwroff_handler,
    regmap);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to register power off handler\n");
    ret = devm_register_restart_handler(dev, spacemit_p1_restart_handler,
    regmap);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to register restart handler\n");
    return 0;
    }
    static const struct platform_device_id spacemit_p1_reboot_id_table[] = {
    { .name = "spacemit-p1-reboot" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, spacemit_p1_reboot_id_table);
    static struct platform_driver spacemit_p1_reboot_driver = {
    .driver = {
    .name = "spacemit-p1-reboot",
    },
    .probe = spacemit_p1_reboot_probe,
    .id_table = spacemit_p1_reboot_id_table,
    };
    module_platform_driver(spacemit_p1_reboot_driver);
    MODULE_DESCRIPTION("SpacemiT P1 reboot/poweroff driver");
    MODULE_LICENSE("GPL");
