//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/livepatch/test_modules/test_klp_callbacks_demo.c
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
// Copyright (C) 2018 Joe Lawrence <joe.lawrence@redhat.com>

    static int pre_patch_ret;
    module_param(pre_patch_ret, int, 0644);
    MODULE_PARM_DESC(pre_patch_ret, "pre_patch_ret (default=0)");
    static const char *const module_state[] = {
    [MODULE_STATE_LIVE]	= "[MODULE_STATE_LIVE] Normal state",
    [MODULE_STATE_COMING]	= "[MODULE_STATE_COMING] Full formed, running module_init",
    [MODULE_STATE_GOING]	= "[MODULE_STATE_GOING] Going away",
    [MODULE_STATE_UNFORMED]	= "[MODULE_STATE_UNFORMED] Still setting it up",
    };
#[no_mangle]
unsafe extern "C" fn callback_info(callback: *const c_char, obj: *mut klp_object) {
    static void callback_info(const char *callback, struct klp_object *obj)
    {
    if (obj.mod)
    pr_info("%s: %s . %s\n", callback, obj.mod.name,
    module_state[obj.mod.state]);
    else
    pr_info("%s: vmlinux\n", callback);
    }
// Executed on object patching (ie, patch enablement)
#[no_mangle]
unsafe extern "C" fn pre_patch_callback(obj: *mut klp_object) -> c_int {
    static int pre_patch_callback(struct klp_object *obj)
    {
    callback_info(__func__, obj);
    return pre_patch_ret;
    }
// Executed on object unpatching (ie, patch disablement)
#[no_mangle]
unsafe extern "C" fn post_patch_callback(obj: *mut klp_object) {
    static void post_patch_callback(struct klp_object *obj)
    {
    callback_info(__func__, obj);
    }
// Executed on object unpatching (ie, patch disablement)
#[no_mangle]
unsafe extern "C" fn pre_unpatch_callback(obj: *mut klp_object) {
    static void pre_unpatch_callback(struct klp_object *obj)
    {
    callback_info(__func__, obj);
    }
// Executed on object unpatching (ie, patch disablement)
#[no_mangle]
unsafe extern "C" fn post_unpatch_callback(obj: *mut klp_object) {
    static void post_unpatch_callback(struct klp_object *obj)
    {
    callback_info(__func__, obj);
    }
#[no_mangle]
unsafe extern "C" fn patched_work_func(work: *mut work_struct) {
    static void patched_work_func(struct work_struct *work)
    {
    pr_info("%s\n", __func__);
    }
    static struct klp_func no_funcs[] = {
    {}
    };
    static struct klp_func busymod_funcs[] = {
    {
    .old_name = "busymod_work_func",
    .new_func = patched_work_func,
    }, {}
    };
    static struct klp_object objs[] = {
    {
    .name = core::ptr::null_mut(),	/* vmlinux */
    .funcs = no_funcs,
    .callbacks = {
    .pre_patch = pre_patch_callback,
    .post_patch = post_patch_callback,
    .pre_unpatch = pre_unpatch_callback,
    .post_unpatch = post_unpatch_callback,
    },
    },	{
    .name = "test_klp_callbacks_mod",
    .funcs = no_funcs,
    .callbacks = {
    .pre_patch = pre_patch_callback,
    .post_patch = post_patch_callback,
    .pre_unpatch = pre_unpatch_callback,
    .post_unpatch = post_unpatch_callback,
    },
    },	{
    .name = "test_klp_callbacks_busy",
    .funcs = busymod_funcs,
    .callbacks = {
    .pre_patch = pre_patch_callback,
    .post_patch = post_patch_callback,
    .pre_unpatch = pre_unpatch_callback,
    .post_unpatch = post_unpatch_callback,
    },
    }, { }
    };
    static struct klp_patch patch = {
    .mod = THIS_MODULE,
    .objs = objs,
    };
#[no_mangle]
unsafe extern "C" fn test_klp_callbacks_demo_init() -> c_int {
    static int test_klp_callbacks_demo_init(void)
    {
    return klp_enable_patch(&patch);
    }
#[no_mangle]
unsafe extern "C" fn test_klp_callbacks_demo_exit() {
    static void test_klp_callbacks_demo_exit(void)
    {
    }
    module_init(test_klp_callbacks_demo_init);
    module_exit(test_klp_callbacks_demo_exit);
    MODULE_LICENSE("GPL");
    MODULE_INFO(livepatch, "Y");
    MODULE_AUTHOR("Joe Lawrence <joe.lawrence@redhat.com>");
    MODULE_DESCRIPTION("Livepatch test: livepatch demo");
