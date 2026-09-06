//! Automatically rewritten from C to Rust
//! Source: lib/of-reconfig-notifier-error-inject.c
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

    static int priority;
    module_param(priority, int, 0);
    MODULE_PARM_DESC(priority, "specify OF reconfig notifier priority");
    static struct notifier_err_inject reconfig_err_inject = {
    .actions = {
    { NOTIFIER_ERR_INJECT_ACTION(OF_RECONFIG_ATTACH_NODE) },
    { NOTIFIER_ERR_INJECT_ACTION(OF_RECONFIG_DETACH_NODE) },
    { NOTIFIER_ERR_INJECT_ACTION(OF_RECONFIG_ADD_PROPERTY) },
    { NOTIFIER_ERR_INJECT_ACTION(OF_RECONFIG_REMOVE_PROPERTY) },
    { NOTIFIER_ERR_INJECT_ACTION(OF_RECONFIG_UPDATE_PROPERTY) },
    {}
    }
    };
    static struct dentry *dir;
#[no_mangle]
unsafe extern "C" fn err_inject_init() -> c_int {
    static int err_inject_init(void)
    {
    int err;
    dir = notifier_err_inject_init("OF-reconfig",
    notifier_err_inject_dir, &reconfig_err_inject, priority);
    if (IS_ERR(dir))
    return PTR_ERR(dir);
    err = of_reconfig_notifier_register(&reconfig_err_inject.nb);
    if (err)
    debugfs_remove_recursive(dir);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn err_inject_exit() {
    static void err_inject_exit(void)
    {
    of_reconfig_notifier_unregister(&reconfig_err_inject.nb);
    debugfs_remove_recursive(dir);
    }
    module_init(err_inject_init);
    module_exit(err_inject_exit);
    MODULE_DESCRIPTION("OF reconfig notifier error injection module");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Akinobu Mita <akinobu.mita@gmail.com>");
