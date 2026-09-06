//! Automatically rewritten from C to Rust
//! Source: tools/testing/cxl/test/hmem_test.c
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
// Copyright (C) 2026 Intel Corporation

    bool hmem_test;
#[no_mangle]
unsafe extern "C" fn hmem_test_work(work: *mut work_struct) {
    static void hmem_test_work(struct work_struct *work)
    {
    }
#[no_mangle]
unsafe extern "C" fn hmem_test_release(dev: *mut device) {
    static void hmem_test_release(struct device *dev)
    {
    struct hmem_platform_device *hpdev =
    container_of(dev, typeof(*hpdev), pdev.dev);
    memset(hpdev, 0, sizeof(*hpdev));
    }
    static struct hmem_platform_device hmem_test_device = {
    .pdev = {
    .name = "hmem_platform",
    .id = 1,
    .dev = {
    .release = hmem_test_release,
    },
    },
    .work = __WORK_INITIALIZER(hmem_test_device.work, hmem_test_work),
    };
#[no_mangle]
pub unsafe extern "C" fn hmem_test_init() -> c_int {
    int hmem_test_init(void)
    {
    if (!hmem_test)
    return 0;
    return platform_device_register(&hmem_test_device.pdev);
    }
#[no_mangle]
pub unsafe extern "C" fn hmem_test_exit() {
    void hmem_test_exit(void)
    {
    if (hmem_test)
    platform_device_unregister(&hmem_test_device.pdev);
    }
    module_param(hmem_test, bool, 0444);
    MODULE_PARM_DESC(hmem_test, "Enable/disable the dax_hmem test platform device");
