//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/tps65086-restart.c
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
// Copyright (C) 2021 Emil Renner Berthing
//

#[no_mangle]
unsafe extern "C" fn tps65086_restart_notify(data: *mut sys_off_data) -> c_int {
    static int tps65086_restart_notify(struct sys_off_data *data)
    {
    struct tps65086 *tps65086 = data.cb_data;
    int ret;
    ret = regmap_write(tps65086.regmap, TPS65086_FORCESHUTDN, 1);
    if (ret) {
    dev_err(tps65086.dev, "%s: error writing to tps65086 pmic: %d\n",
    __func__, ret);
    return NOTIFY_DONE;
    }
// give it a little time
    mdelay(200);
    WARN_ON(1);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn tps65086_restart_probe(pdev: *mut platform_device) -> c_int {
    static int tps65086_restart_probe(struct platform_device *pdev)
    {
    struct tps65086 *tps65086 = dev_get_drvdata(pdev.dev.parent);
    return devm_register_sys_off_handler(&pdev.dev,
    SYS_OFF_MODE_RESTART,
    SYS_OFF_PRIO_HIGH,
    tps65086_restart_notify,
    tps65086);
    }
    static const struct platform_device_id tps65086_restart_id_table[] = {
    { .name = "tps65086-reset" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, tps65086_restart_id_table);
    static struct platform_driver tps65086_restart_driver = {
    .driver = {
    .name = "tps65086-restart",
    },
    .probe = tps65086_restart_probe,
    .id_table = tps65086_restart_id_table,
    };
    module_platform_driver(tps65086_restart_driver);
    MODULE_AUTHOR("Emil Renner Berthing <kernel@esmil.dk>");
    MODULE_DESCRIPTION("TPS65086 restart driver");
