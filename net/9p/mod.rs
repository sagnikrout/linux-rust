//! Automatically rewritten from C to Rust
//! Source: net/9p/mod.c
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
// 9P entry point
//
// Copyright (C) 2007 by Latchesar Ionkov <lucho@ionkov.net>
// Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
// Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
//

    unsigned int p9_debug_level;	/* feature-rific global debug level  */
    EXPORT_SYMBOL(p9_debug_level);
    module_param_named(debug, p9_debug_level, uint, 0);
    MODULE_PARM_DESC(debug, "9P debugging level");
    void _p9_debug(enum p9_debug_flags level, const char *func,
    const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    if ((p9_debug_level & level) != level)
    return;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    if (level == P9_DEBUG_9P)
    pr_notice("(%8.8d) %pV", task_pid_nr(current), &vaf);
    else
    pr_notice("-- %s (%d): %pV", func, task_pid_nr(current), &vaf);
    va_end(args);
    }
    EXPORT_SYMBOL(_p9_debug);

// Dynamic Transport Registration Routines
    static DEFINE_SPINLOCK(v9fs_trans_lock);
    static LIST_HEAD(v9fs_trans_list);
//
// v9fs_register_trans - register a new transport with 9p
// @m: structure describing the transport module and entry points
//
#[no_mangle]
pub unsafe extern "C" fn v9fs_register_trans(m: *mut p9_trans_module) {
    void v9fs_register_trans(struct p9_trans_module *m)
    {
    spin_lock(&v9fs_trans_lock);
    list_add_tail(&m.list, &v9fs_trans_list);
    spin_unlock(&v9fs_trans_lock);
    }
    EXPORT_SYMBOL(v9fs_register_trans);
//
// v9fs_unregister_trans - unregister a 9p transport
// @m: the transport to remove
//
#[no_mangle]
pub unsafe extern "C" fn v9fs_unregister_trans(m: *mut p9_trans_module) {
    void v9fs_unregister_trans(struct p9_trans_module *m)
    {
    spin_lock(&v9fs_trans_lock);
    list_del_init(&m.list);
    spin_unlock(&v9fs_trans_lock);
    }
    EXPORT_SYMBOL(v9fs_unregister_trans);
    static struct p9_trans_module *_p9_get_trans_by_name(const char *s)
    {
    struct p9_trans_module *t, *found = core::ptr::null_mut();
    spin_lock(&v9fs_trans_lock);
    list_for_each_entry(t, &v9fs_trans_list, list)
    if (strcmp(t.name, s) == 0 &&
    try_module_get(t.owner)) {
    found = t;
    break;
    }
    spin_unlock(&v9fs_trans_lock);
    return found;
    }
//
// v9fs_get_trans_by_name - get transport with the matching name
// @s: string identifying transport
//
    struct p9_trans_module *v9fs_get_trans_by_name(const char *s)
    {
    struct p9_trans_module *found = core::ptr::null_mut();
    found = _p9_get_trans_by_name(s);

    if (!found) {
    request_module("9p-%s", s);
    found = _p9_get_trans_by_name(s);
    }

    return found;
    }
    EXPORT_SYMBOL(v9fs_get_trans_by_name);
    static const char * const v9fs_default_transports[] = {
    "virtio", "tcp", "fd", "unix", "xen", "rdma",
    };
//
// v9fs_get_default_trans - get the default transport
//
    struct p9_trans_module *v9fs_get_default_trans(void)
    {
    struct p9_trans_module *t, *found = core::ptr::null_mut();
    int i;
    spin_lock(&v9fs_trans_lock);
    list_for_each_entry(t, &v9fs_trans_list, list)
    if (t.def && try_module_get(t.owner)) {
    found = t;
    break;
    }
    if (!found)
    list_for_each_entry(t, &v9fs_trans_list, list)
    if (try_module_get(t.owner)) {
    found = t;
    break;
    }
    spin_unlock(&v9fs_trans_lock);
    for (i = 0; !found && i < ARRAY_SIZE(v9fs_default_transports); i++)
    found = v9fs_get_trans_by_name(v9fs_default_transports[i]);
    return found;
    }
    EXPORT_SYMBOL(v9fs_get_default_trans);
//
// v9fs_put_trans - put trans
// @m: transport to put
//
#[no_mangle]
pub unsafe extern "C" fn v9fs_put_trans(m: *mut p9_trans_module) {
    void v9fs_put_trans(struct p9_trans_module *m)
    {
    if (m)
    module_put(m.owner);
    }
    EXPORT_SYMBOL(v9fs_put_trans);
//
// init_p9 - Initialize module
//
#[no_mangle]
unsafe extern "C" fn init_p9() -> int __init {
    static int __init init_p9(void)
    {
    int ret;
    ret = p9_client_init();
    if (ret)
    return ret;
    p9_error_init();
    pr_info("Installing 9P2000 support\n");
    return ret;
    }
//
// exit_p9 - shutdown module
//
#[no_mangle]
unsafe extern "C" fn exit_p9() -> void __exit {
    static void __exit exit_p9(void)
    {
    pr_info("Unloading 9P2000 support\n");
    p9_client_exit();
    }
    module_init(init_p9)
    module_exit(exit_p9)
    MODULE_AUTHOR("Latchesar Ionkov <lucho@ionkov.net>");
    MODULE_AUTHOR("Eric Van Hensbergen <ericvh@gmail.com>");
    MODULE_AUTHOR("Ron Minnich <rminnich@lanl.gov>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Plan 9 Resource Sharing Support (9P2000)");
