//! Automatically rewritten from C to Rust
//! Source: lib/netdev-notifier-error-inject.c
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
    MODULE_PARM_DESC(priority, "specify netdevice notifier priority");
    static struct notifier_err_inject netdev_notifier_err_inject = {
    .actions = {
    { NOTIFIER_ERR_INJECT_ACTION(NETDEV_REGISTER) },
    { NOTIFIER_ERR_INJECT_ACTION(NETDEV_CHANGEMTU) },
    { NOTIFIER_ERR_INJECT_ACTION(NETDEV_CHANGENAME) },
    { NOTIFIER_ERR_INJECT_ACTION(NETDEV_PRE_UP) },
    { NOTIFIER_ERR_INJECT_ACTION(NETDEV_PRE_TYPE_CHANGE) },
    { NOTIFIER_ERR_INJECT_ACTION(NETDEV_POST_INIT) },
    { NOTIFIER_ERR_INJECT_ACTION(NETDEV_PRECHANGEMTU) },
    { NOTIFIER_ERR_INJECT_ACTION(NETDEV_PRECHANGEUPPER) },
    { NOTIFIER_ERR_INJECT_ACTION(NETDEV_CHANGEUPPER) },
    {}
    }
    };
    static struct dentry *dir;
#[no_mangle]
unsafe extern "C" fn netdev_err_inject_init() -> c_int {
    static int netdev_err_inject_init(void)
    {
    int err;
    dir = notifier_err_inject_init("netdev", notifier_err_inject_dir,
    &netdev_notifier_err_inject, priority);
    if (IS_ERR(dir))
    return PTR_ERR(dir);
    err = register_netdevice_notifier(&netdev_notifier_err_inject.nb);
    if (err)
    debugfs_remove_recursive(dir);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn netdev_err_inject_exit() {
    static void netdev_err_inject_exit(void)
    {
    unregister_netdevice_notifier(&netdev_notifier_err_inject.nb);
    debugfs_remove_recursive(dir);
    }
    module_init(netdev_err_inject_init);
    module_exit(netdev_err_inject_exit);
    MODULE_DESCRIPTION("Netdevice notifier error injection module");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Nikolay Aleksandrov <razor@blackwall.org>");
