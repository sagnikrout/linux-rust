//! Automatically rewritten from C to Rust
//! Source: drivers/base/container.c
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
// System bus type for containers.
//
// Copyright (C) 2013, Intel Corporation
// Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//

#[no_mangle]
unsafe extern "C" fn trivial_online(dev: *mut device) -> c_int {
    static int trivial_online(struct device *dev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn container_offline(dev: *mut device) -> c_int {
    static int container_offline(struct device *dev)
    {
    struct container_dev *cdev = to_container_dev(dev);
    return cdev.offline ? cdev.offline(cdev) : 0;
    }
    const struct bus_type container_subsys = {
    .name = CONTAINER_BUS_NAME,
    .dev_name = CONTAINER_BUS_NAME,
    .online = trivial_online,
    .offline = container_offline,
    };
#[no_mangle]
pub unsafe extern "C" fn container_dev_init() -> void __init {
    void __init container_dev_init(void)
    {
    int ret;
    ret = subsys_system_register(&container_subsys, core::ptr::null_mut());
    if (ret)
    pr_err("%s() failed: %d\n", __func__, ret);
    }
