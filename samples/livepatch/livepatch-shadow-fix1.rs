//! Automatically rewritten from C to Rust
//! Source: samples/livepatch/livepatch-shadow-fix1.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
//
// livepatch-shadow-fix1.c - Shadow variables, livepatch demo
//
// Purpose
// -------
//
// Fixes the memory leak introduced in livepatch-shadow-mod through the
// use of a shadow variable.  This fix demonstrates the "extending" of
// short-lived data structures by patching its allocation and release
// functions.
//
// Usage
// -----
//
// This module is not intended to be standalone.  See the "Usage"
// section of livepatch-shadow-mod.c.
//

// Shadow variable enums
pub const SV_LEAK: c_int = 1;
// Allocate new dummies every second
pub const ALLOC_PERIOD: c_int = 1;
// Check for expired dummies after a few new ones have been allocated

// Dummies expire after a few cleanup instances

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dummy {
    pub list: list_head,
    pub jiffies_expire: c_ulong,
}

//
// The constructor makes more sense together with klp_shadow_get_or_alloc().
// In this example, it would be safe to assign the pointer also to the shadow
// variable returned by klp_shadow_alloc().  But we wanted to show the more
// complicated use of the API.
//
#[no_mangle]
unsafe extern "C" fn shadow_leak_ctor(obj: *mut c_void, shadow_data: *mut c_void, ctor_data: *mut c_void) -> c_int {
    static int shadow_leak_ctor(void *obj, void *shadow_data, void *ctor_data)
    {
    int **shadow_leak = shadow_data;
    int **leak = ctor_data;
    if (!ctor_data)
    return -EINVAL;
// shadow_leak = *leak;
    return 0;
    }
    static struct dummy *livepatch_fix1_dummy_alloc(void)
    {
    struct dummy *d;
    int *leak;
    int **shadow_leak;
    d = kzalloc(sizeof(*d), GFP_KERNEL);
    if (!d)
    return core::ptr::null_mut();
    d.jiffies_expire = jiffies + secs_to_jiffies(EXPIRE_PERIOD);
//
// Patch: save the extra memory location into a SV_LEAK shadow
// variable.  A patched dummy_free routine can later fetch this
// pointer to handle resource release.
//
    leak = kzalloc(sizeof(*leak), GFP_KERNEL);
    if (!leak)
    goto err_leak;
    shadow_leak = klp_shadow_alloc(d, SV_LEAK, sizeof(leak), GFP_KERNEL,
    shadow_leak_ctor, &leak);
    if (!shadow_leak) {
    pr_err("%s: failed to allocate shadow variable for the leaking pointer: dummy @ %p, leak @ %p\n",
    __func__, d, leak);
    goto err_shadow;
    }
    pr_info("%s: dummy @ %p, expires @ %lx\n",
    __func__, d, d.jiffies_expire);
    return d;
    err_shadow:
    kfree(leak);
    err_leak:
    kfree(d);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn livepatch_fix1_dummy_leak_dtor(obj: *mut c_void, shadow_data: *mut c_void) {
    static void livepatch_fix1_dummy_leak_dtor(void *obj, void *shadow_data)
    {
    void *d = obj;
    int **shadow_leak = shadow_data;
    pr_info("%s: dummy @ %p, prevented leak @ %p\n",
    __func__, d, *shadow_leak);
    kfree(*shadow_leak);
    }
#[no_mangle]
unsafe extern "C" fn livepatch_fix1_dummy_free(d: *mut dummy) {
    static void livepatch_fix1_dummy_free(struct dummy *d)
    {
    int **shadow_leak;
//
// Patch: fetch the saved SV_LEAK shadow variable, detach and
// free it.  Note: handle cases where this shadow variable does
// not exist (ie, dummy structures allocated before this livepatch
// was loaded.)
//
    shadow_leak = klp_shadow_get(d, SV_LEAK);
    if (shadow_leak)
    klp_shadow_free(d, SV_LEAK, livepatch_fix1_dummy_leak_dtor);
    else
    pr_info("%s: dummy @ %p leaked!\n", __func__, d);
    kfree(d);
    }
    static struct klp_func funcs[] = {
    {
    .old_name = "dummy_alloc",
    .new_func = livepatch_fix1_dummy_alloc,
    },
    {
    .old_name = "dummy_free",
    .new_func = livepatch_fix1_dummy_free,
    }, { }
    };
    static struct klp_object objs[] = {
    {
    .name = "livepatch_shadow_mod",
    .funcs = funcs,
    }, { }
    };
    static struct klp_patch patch = {
    .mod = THIS_MODULE,
    .objs = objs,
    };
#[no_mangle]
unsafe extern "C" fn livepatch_shadow_fix1_init() -> c_int {
    static int livepatch_shadow_fix1_init(void)
    {
    return klp_enable_patch(&patch);
    }
#[no_mangle]
unsafe extern "C" fn livepatch_shadow_fix1_exit() {
    static void livepatch_shadow_fix1_exit(void)
    {
// Cleanup any existing SV_LEAK shadow variables
    klp_shadow_free_all(SV_LEAK, livepatch_fix1_dummy_leak_dtor);
    }
    module_init(livepatch_shadow_fix1_init);
    module_exit(livepatch_shadow_fix1_exit);
    MODULE_DESCRIPTION("Live patching demo for shadow variables");
    MODULE_LICENSE("GPL");
    MODULE_INFO(livepatch, "Y");
