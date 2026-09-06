//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/speed_select_if/isst_tpmi.c
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
// isst_tpmi.c: SST TPMI interface
//
// Copyright (c) 2023, Intel Corporation.
// All Rights Reserved.
//

#[no_mangle]
unsafe extern "C" fn intel_sst_probe(auxdev: *mut auxiliary_device, id: *const auxiliary_device_id) -> c_int {
    static int intel_sst_probe(struct auxiliary_device *auxdev, const struct auxiliary_device_id *id)
    {
    int ret;
    ret = tpmi_sst_init();
    if (ret)
    return ret;
    ret = tpmi_sst_dev_add(auxdev);
    if (ret)
    tpmi_sst_exit();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn intel_sst_remove(auxdev: *mut auxiliary_device) {
    static void intel_sst_remove(struct auxiliary_device *auxdev)
    {
    tpmi_sst_dev_remove(auxdev);
    tpmi_sst_exit();
    }
#[no_mangle]
unsafe extern "C" fn intel_sst_suspend(dev: *mut device) -> c_int {
    static int intel_sst_suspend(struct device *dev)
    {
    tpmi_sst_dev_suspend(to_auxiliary_dev(dev));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_sst_resume(dev: *mut device) -> c_int {
    static int intel_sst_resume(struct device *dev)
    {
    tpmi_sst_dev_resume(to_auxiliary_dev(dev));
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(intel_sst_pm, intel_sst_suspend, intel_sst_resume);
    static const struct auxiliary_device_id intel_sst_id_table[] = {
    { .name = "intel_vsec.tpmi-sst" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, intel_sst_id_table);
    static struct auxiliary_driver intel_sst_aux_driver = {
    .id_table       = intel_sst_id_table,
    .remove         = intel_sst_remove,
    .probe          = intel_sst_probe,
    .driver = {
    .pm = pm_sleep_ptr(&intel_sst_pm),
    },
    };
    module_auxiliary_driver(intel_sst_aux_driver);
    MODULE_IMPORT_NS("INTEL_TPMI_SST");
    MODULE_DESCRIPTION("Intel TPMI SST Driver");
    MODULE_LICENSE("GPL");
