//! Automatically rewritten from C to Rust
//! Source: drivers/iio/industrialio-configfs.c
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
// Industrial I/O configfs bits
//
// Copyright (c) 2015 Intel Corporation
//

    static const struct config_item_type iio_root_group_type = {
    .ct_owner       = THIS_MODULE,
    };
    struct configfs_subsystem iio_configfs_subsys = {
    .su_group = {
    .cg_item = {
    .ci_namebuf = "iio",
    .ci_type = &iio_root_group_type,
    },
    },
    .su_mutex = __MUTEX_INITIALIZER(iio_configfs_subsys.su_mutex),
    };
    EXPORT_SYMBOL(iio_configfs_subsys);
#[no_mangle]
unsafe extern "C" fn iio_configfs_init() -> int __init {
    static int __init iio_configfs_init(void)
    {
    config_group_init(&iio_configfs_subsys.su_group);
    return configfs_register_subsystem(&iio_configfs_subsys);
    }
    module_init(iio_configfs_init);
#[no_mangle]
unsafe extern "C" fn iio_configfs_exit() -> void __exit {
    static void __exit iio_configfs_exit(void)
    {
    configfs_unregister_subsystem(&iio_configfs_subsys);
    }
    module_exit(iio_configfs_exit);
    MODULE_AUTHOR("Daniel Baluta <daniel.baluta@intel.com>");
    MODULE_DESCRIPTION("Industrial I/O configfs support");
    MODULE_LICENSE("GPL v2");
