//! Automatically rewritten from C to Rust
//! Source: samples/livepatch/livepatch-shadow-fix2.c
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
// livepatch-shadow-fix2.c - Shadow variables, livepatch demo
//
// Purpose
// -------
//
// Adds functionality to livepatch-shadow-mod's in-flight data
// structures through a shadow variable.  The livepatch patches a
// routine that periodically inspects data structures, incrementing a
// per-data-structure counter, creating the counter if needed.
//
// Usage
// -----
//
// This module is not intended to be standalone.  See the "Usage"
// section of livepatch-shadow-mod.c.
//

// Shadow variable enums
pub const SV_LEAK: c_int = 1;
pub const SV_COUNTER: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dummy {
    pub list: list_head,
    pub jiffies_expire: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn livepatch_fix2_dummy_check(d: *mut dummy, jiffies: c_ulong) -> bool {
    static bool livepatch_fix2_dummy_check(struct dummy *d, unsigned long jiffies)
    {
    int *shadow_count;
//
// Patch: handle in-flight dummy structures, if they do not
// already have a SV_COUNTER shadow variable, then attach a
// new one.
//
    shadow_count = klp_shadow_get_or_alloc(d, SV_COUNTER,
    sizeof(*shadow_count), GFP_NOWAIT,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (shadow_count)
// shadow_count += 1;
    return time_after(jiffies, d.jiffies_expire);
    }
#[no_mangle]
unsafe extern "C" fn livepatch_fix2_dummy_leak_dtor(obj: *mut c_void, shadow_data: *mut c_void) {
    static void livepatch_fix2_dummy_leak_dtor(void *obj, void *shadow_data)
    {
    void *d = obj;
    int **shadow_leak = shadow_data;
    pr_info("%s: dummy @ %p, prevented leak @ %p\n",
    __func__, d, *shadow_leak);
    kfree(*shadow_leak);
    }
#[no_mangle]
unsafe extern "C" fn livepatch_fix2_dummy_free(d: *mut dummy) {
    static void livepatch_fix2_dummy_free(struct dummy *d)
    {
    int **shadow_leak;
    int *shadow_count;
// Patch: copy the memory leak patch from the fix1 module.
    shadow_leak = klp_shadow_get(d, SV_LEAK);
    if (shadow_leak)
    klp_shadow_free(d, SV_LEAK, livepatch_fix2_dummy_leak_dtor);
    else
    pr_info("%s: dummy @ %p leaked!\n", __func__, d);
//
// Patch: fetch the SV_COUNTER shadow variable and display
// the final count.  Detach the shadow variable.
//
    shadow_count = klp_shadow_get(d, SV_COUNTER);
    if (shadow_count) {
    pr_info("%s: dummy @ %p, check counter = %d\n",
    __func__, d, *shadow_count);
    klp_shadow_free(d, SV_COUNTER, core::ptr::null_mut());
    }
    kfree(d);
    }
    static struct klp_func funcs[] = {
    {
    .old_name = "dummy_check",
    .new_func = livepatch_fix2_dummy_check,
    },
    {
    .old_name = "dummy_free",
    .new_func = livepatch_fix2_dummy_free,
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
unsafe extern "C" fn livepatch_shadow_fix2_init() -> c_int {
    static int livepatch_shadow_fix2_init(void)
    {
    return klp_enable_patch(&patch);
    }
#[no_mangle]
unsafe extern "C" fn livepatch_shadow_fix2_exit() {
    static void livepatch_shadow_fix2_exit(void)
    {
// Cleanup any existing SV_COUNTER shadow variables
    klp_shadow_free_all(SV_COUNTER, core::ptr::null_mut());
    }
    module_init(livepatch_shadow_fix2_init);
    module_exit(livepatch_shadow_fix2_exit);
    MODULE_DESCRIPTION("Live patching demo for shadow variables");
    MODULE_LICENSE("GPL");
    MODULE_INFO(livepatch, "Y");
