//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/axxia-reset.c
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
// Reset driver for Axxia devices
//
// Copyright (C) 2014 LSI
//

pub const SC_CRIT_WRITE_KEY: c_uint = 0x1000;
pub const SC_LATCH_ON_RESET: c_uint = 0x1004;
pub const SC_RESET_CONTROL: c_uint = 0x1008;

pub const SC_EFUSE_INT_STATUS: c_uint = 0x180c;

#[no_mangle]
unsafe extern "C" fn axxia_restart_handler(data: *mut sys_off_data) -> c_int {
    static int axxia_restart_handler(struct sys_off_data *data)
    {
    struct regmap *syscon = data.cb_data;
// Access Key (0xab)
    regmap_write(syscon, SC_CRIT_WRITE_KEY, 0xab);
// Select internal boot from 0xffff0000
    regmap_write(syscon, SC_LATCH_ON_RESET, 0x00000040);
// Assert ResetReadDone (to avoid hanging in boot ROM)
    regmap_write(syscon, SC_EFUSE_INT_STATUS, EFUSE_READ_DONE);
// Assert chip reset
    regmap_update_bits(syscon, SC_RESET_CONTROL,
    RSTCTL_RST_CHIP, RSTCTL_RST_CHIP);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn axxia_reset_probe(pdev: *mut platform_device) -> c_int {
    static int axxia_reset_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct regmap *syscon;
    int err;
    syscon = syscon_regmap_lookup_by_phandle(dev.of_node, "syscon");
    if (IS_ERR(syscon)) {
    pr_err("%pOFn: syscon lookup failed\n", dev.of_node);
    return PTR_ERR(syscon);
    }
    err = devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_RESTART,
    128, axxia_restart_handler, syscon);
    if (err)
    dev_err(dev, "cannot register restart handler (err=%d)\n", err);
    return err;
    }
    static const struct of_device_id of_axxia_reset_match[] = {
    { .compatible = "lsi,axm55xx-reset", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_axxia_reset_match);
    static struct platform_driver axxia_reset_driver = {
    .probe = axxia_reset_probe,
    .driver = {
    .name = "axxia-reset",
    .of_match_table = of_match_ptr(of_axxia_reset_match),
    },
    };
    builtin_platform_driver(axxia_reset_driver);
