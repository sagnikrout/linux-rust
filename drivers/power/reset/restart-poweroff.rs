//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/restart-poweroff.c
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
// Power off by restarting and let u-boot keep hold of the machine
// until the user presses a button for example.
//
// Andrew Lunn <andrew@lunn.ch>
//
// Copyright (C) 2012 Andrew Lunn
//

#[no_mangle]
unsafe extern "C" fn restart_poweroff_do_poweroff(data: *mut sys_off_data) -> c_int {
    static int restart_poweroff_do_poweroff(struct sys_off_data *data)
    {
    reboot_mode = REBOOT_HARD;
    machine_restart(core::ptr::null_mut());
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn restart_poweroff_probe(pdev: *mut platform_device) -> c_int {
    static int restart_poweroff_probe(struct platform_device *pdev)
    {
// Set this handler to low priority to not override an existing handler
    return devm_register_sys_off_handler(&pdev.dev,
    SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_LOW,
    restart_poweroff_do_poweroff,
    core::ptr::null_mut());
    }
    static const struct of_device_id of_restart_poweroff_match[] = {
    { .compatible = "restart-poweroff", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_restart_poweroff_match);
    static struct platform_driver restart_poweroff_driver = {
    .probe = restart_poweroff_probe,
    .driver = {
    .name = "poweroff-restart",
    .of_match_table = of_restart_poweroff_match,
    },
    };
    module_platform_driver(restart_poweroff_driver);
    MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch");
    MODULE_DESCRIPTION("restart poweroff driver");
    MODULE_ALIAS("platform:poweroff-restart");
