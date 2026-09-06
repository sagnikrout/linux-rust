//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/int340x_thermal/int3401_thermal.c
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
// INT3401 processor thermal device
// Copyright (c) 2020, Intel Corporation.
//

    static const struct acpi_device_id int3401_device_ids[] = {
    {"INT3401", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, int3401_device_ids);
#[no_mangle]
unsafe extern "C" fn int3401_add(pdev: *mut platform_device) -> c_int {
    static int int3401_add(struct platform_device *pdev)
    {
    struct proc_thermal_device *proc_priv;
    int ret;
    proc_priv = devm_kzalloc(&pdev.dev, sizeof(*proc_priv), GFP_KERNEL);
    if (!proc_priv)
    return -ENOMEM;
    ret = proc_thermal_add(&pdev.dev, proc_priv);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, proc_priv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn int3401_remove(pdev: *mut platform_device) {
    static void int3401_remove(struct platform_device *pdev)
    {
    proc_thermal_remove(platform_get_drvdata(pdev));
    }

#[no_mangle]
unsafe extern "C" fn int3401_thermal_suspend(dev: *mut device) -> c_int {
    static int int3401_thermal_suspend(struct device *dev)
    {
    return proc_thermal_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn int3401_thermal_resume(dev: *mut device) -> c_int {
    static int int3401_thermal_resume(struct device *dev)
    {
    return proc_thermal_resume(dev);
    }

    static SIMPLE_DEV_PM_OPS(int3401_proc_thermal_pm, int3401_thermal_suspend,
    int3401_thermal_resume);
    static struct platform_driver int3401_driver = {
    .probe = int3401_add,
    .remove = int3401_remove,
    .driver = {
    .name = "int3401 thermal",
    .acpi_match_table = int3401_device_ids,
    .pm = &int3401_proc_thermal_pm,
    },
    };
    module_platform_driver(int3401_driver);
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_DESCRIPTION("Processor Thermal Reporting Device Driver");
    MODULE_LICENSE("GPL v2");
