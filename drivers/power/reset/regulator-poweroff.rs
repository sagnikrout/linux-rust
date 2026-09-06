//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/regulator-poweroff.c
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
// Force-disables a regulator to power down a device
//
// Michael Klein <michael@fossekall.de>
//
// Copyright (C) 2020 Michael Klein
//
// Based on the gpio-poweroff driver.
//

pub const TIMEOUT_MS: c_int = 3000;
#[no_mangle]
unsafe extern "C" fn regulator_poweroff_do_poweroff(data: *mut sys_off_data) -> c_int {
    static int regulator_poweroff_do_poweroff(struct sys_off_data *data)
    {
    struct regulator *cpu_regulator = data.cb_data;
    if (cpu_regulator && regulator_is_enabled(cpu_regulator))
    regulator_force_disable(cpu_regulator);
// give it some time
    mdelay(TIMEOUT_MS);
    WARN_ON(1);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn regulator_poweroff_probe(pdev: *mut platform_device) -> c_int {
    static int regulator_poweroff_probe(struct platform_device *pdev)
    {
    struct regulator *cpu_regulator;
    cpu_regulator = devm_regulator_get(&pdev.dev, "cpu");
    if (IS_ERR(cpu_regulator))
    return PTR_ERR(cpu_regulator);
// Set this handler to low priority to not override an existing handler
    return devm_register_sys_off_handler(&pdev.dev,
    SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_LOW,
    regulator_poweroff_do_poweroff,
    cpu_regulator);
    }
    static const struct of_device_id of_regulator_poweroff_match[] = {
    { .compatible = "regulator-poweroff", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_regulator_poweroff_match);
    static struct platform_driver regulator_poweroff_driver = {
    .probe = regulator_poweroff_probe,
    .driver = {
    .name = "poweroff-regulator",
    .of_match_table = of_regulator_poweroff_match,
    },
    };
    module_platform_driver(regulator_poweroff_driver);
    MODULE_AUTHOR("Michael Klein <michael@fossekall.de>");
    MODULE_DESCRIPTION("Regulator poweroff driver");
    MODULE_ALIAS("platform:poweroff-regulator");
