//! Automatically rewritten from C to Rust
//! Source: drivers/thunderbolt/configfs.c
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
// ConfigFS support
//
// Copyright (C) 2026, Intel Corporation
// Author: Mika Westerberg <mika.westerberg@linux.intel.com>
//

    static const struct config_item_type tb_root_group_type = {
    .ct_owner = THIS_MODULE,
    };
    static struct configfs_subsystem tb_configfs = {
    .su_group = {
    .cg_item = {
    .ci_namebuf = "thunderbolt",
    .ci_type = &tb_root_group_type,
    },
    },
    };
//
// tb_configfs_register_group() - Register Thunderbolt ConfigFS group
// @group: Group to register.
//
// Registers the new @group under Thunderbolt subsystem ConfigFS.
//
// Return: 0% in case of success, negative errno otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn tb_configfs_register_group(group: *mut config_group) -> c_int {
    int tb_configfs_register_group(struct config_group *group)
    {
    return configfs_register_group(&tb_configfs.su_group, group);
    }
    EXPORT_SYMBOL_GPL(tb_configfs_register_group);
//
// tb_configfs_unregister_group() - Unregister previously registered group
// @group: Group to unregister.
//
#[no_mangle]
pub unsafe extern "C" fn tb_configfs_unregister_group(group: *mut config_group) {
    void tb_configfs_unregister_group(struct config_group *group)
    {
    configfs_unregister_group(group);
    }
    EXPORT_SYMBOL_GPL(tb_configfs_unregister_group);
#[no_mangle]
pub unsafe extern "C" fn tb_configfs_init() -> c_int {
    int tb_configfs_init(void)
    {
    config_group_init(&tb_configfs.su_group);
    mutex_init(&tb_configfs.su_mutex);
    return configfs_register_subsystem(&tb_configfs);
    }
#[no_mangle]
pub unsafe extern "C" fn tb_configfs_exit() {
    void tb_configfs_exit(void)
    {
    configfs_unregister_subsystem(&tb_configfs);
    }
