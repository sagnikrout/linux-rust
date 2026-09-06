//! Automatically rewritten from C to Rust
//! Source: samples/livepatch/livepatch-shadow-mod.c
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
// livepatch-shadow-mod.c - Shadow variables, buggy module demo
//
// Purpose
// -------
//
// As a demonstration of livepatch shadow variable API, this module
// introduces memory leak behavior that livepatch modules
// livepatch-shadow-fix1.ko and livepatch-shadow-fix2.ko correct and
// enhance.
//
// WARNING - even though the livepatch-shadow-fix modules patch the
// memory leak, please load these modules at your own risk -- some
// amount of memory may leaked before the bug is patched.
//
// Usage
// -----
//
// Step 1 - Load the buggy demonstration module:
//
// insmod samples/livepatch/livepatch-shadow-mod.ko
//
// Watch dmesg output for a few moments to see new dummy being allocated
// and a periodic cleanup check.  (Note: a small amount of memory is
// being leaked.)
//
// Step 2 - Load livepatch fix1:
//
// insmod samples/livepatch/livepatch-shadow-fix1.ko
//
// Continue watching dmesg and note that now livepatch_fix1_dummy_free()
// and livepatch_fix1_dummy_alloc() are logging messages about leaked
// memory and eventually leaks prevented.
//
// Step 3 - Load livepatch fix2 (on top of fix1):
//
// insmod samples/livepatch/livepatch-shadow-fix2.ko
//
// This module extends functionality through shadow variables, as a new
// "check" counter is added to the dummy structure.  Periodic dmesg
// messages will log these as dummies are cleaned up.
//
// Step 4 - Cleanup
//
// Unwind the demonstration by disabling the livepatch fix modules, then
// removing them and the demo module:
//
// echo 0 > /sys/kernel/livepatch/livepatch_shadow_fix2/enabled
// echo 0 > /sys/kernel/livepatch/livepatch_shadow_fix1/enabled
// rmmod livepatch-shadow-fix2
// rmmod livepatch-shadow-fix1
// rmmod livepatch-shadow-mod
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Joe Lawrence <joe.lawrence@redhat.com>");
    MODULE_DESCRIPTION("Buggy module for shadow variable demo");
// Allocate new dummies every second
pub const ALLOC_PERIOD: c_int = 1;
// Check for expired dummies after a few new ones have been allocated

// Dummies expire after a few cleanup instances

//
// Keep a list of all the dummies so we can clean up any residual ones
// on module exit
//
    static LIST_HEAD(dummy_list);
    static DEFINE_MUTEX(dummy_list_mutex);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dummy {
    pub list: list_head,
    pub jiffies_expire: c_ulong,
}

    static __used noinline struct dummy *dummy_alloc(void)
    {
    struct dummy *d;
    int *leak;
    d = kzalloc(sizeof(*d), GFP_KERNEL);
    if (!d)
    return core::ptr::null_mut();
    d.jiffies_expire = jiffies + secs_to_jiffies(EXPIRE_PERIOD);
// Oops, forgot to save leak!
    leak = kzalloc(sizeof(*leak), GFP_KERNEL);
    if (!leak) {
    kfree(d);
    return core::ptr::null_mut();
    }
    pr_info("%s: dummy @ %p, expires @ %lx\n",
    __func__, d, d.jiffies_expire);
    return d;
    }
#[no_mangle]
unsafe extern "C" fn dummy_free(d: *mut dummy) -> __used noinline void {
    static __used noinline void dummy_free(struct dummy *d)
    {
    pr_info("%s: dummy @ %p, expired = %lx\n",
    __func__, d, d.jiffies_expire);
    kfree(d);
    }
    static __used noinline bool dummy_check(struct dummy *d,
    unsigned long jiffies)
    {
    return time_after(jiffies, d.jiffies_expire);
    }
//
// alloc_work_func: allocates new dummy structures, allocates additional
// memory, aptly named "leak", but doesn't keep
// permanent record of it.
//
    static void alloc_work_func(struct work_struct *work);
    static DECLARE_DELAYED_WORK(alloc_dwork, alloc_work_func);
#[no_mangle]
unsafe extern "C" fn alloc_work_func(work: *mut work_struct) {
    static void alloc_work_func(struct work_struct *work)
    {
    struct dummy *d;
    d = dummy_alloc();
    if (!d)
    return;
    mutex_lock(&dummy_list_mutex);
    list_add(&d.list, &dummy_list);
    mutex_unlock(&dummy_list_mutex);
    schedule_delayed_work(&alloc_dwork, secs_to_jiffies(ALLOC_PERIOD));
    }
//
// cleanup_work_func: frees dummy structures.  Without knownledge of
// "leak", it leaks the additional memory that
// alloc_work_func created.
//
    static void cleanup_work_func(struct work_struct *work);
    static DECLARE_DELAYED_WORK(cleanup_dwork, cleanup_work_func);
#[no_mangle]
unsafe extern "C" fn cleanup_work_func(work: *mut work_struct) {
    static void cleanup_work_func(struct work_struct *work)
    {
    struct dummy *d, *tmp;
    unsigned long j;
    j = jiffies;
    pr_info("%s: jiffies = %lx\n", __func__, j);
    mutex_lock(&dummy_list_mutex);
    list_for_each_entry_safe(d, tmp, &dummy_list, list) {
// Kick out and free any expired dummies
    if (dummy_check(d, j)) {
    list_del(&d.list);
    dummy_free(d);
    }
    }
    mutex_unlock(&dummy_list_mutex);
    schedule_delayed_work(&cleanup_dwork, secs_to_jiffies(CLEANUP_PERIOD));
    }
#[no_mangle]
unsafe extern "C" fn livepatch_shadow_mod_init() -> c_int {
    static int livepatch_shadow_mod_init(void)
    {
    schedule_delayed_work(&alloc_dwork, secs_to_jiffies(ALLOC_PERIOD));
    schedule_delayed_work(&cleanup_dwork, secs_to_jiffies(CLEANUP_PERIOD));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn livepatch_shadow_mod_exit() {
    static void livepatch_shadow_mod_exit(void)
    {
    struct dummy *d, *tmp;
// Wait for any dummies at work
    cancel_delayed_work_sync(&alloc_dwork);
    cancel_delayed_work_sync(&cleanup_dwork);
// Cleanup residual dummies
    list_for_each_entry_safe(d, tmp, &dummy_list, list) {
    list_del(&d.list);
    dummy_free(d);
    }
    }
    module_init(livepatch_shadow_mod_init);
    module_exit(livepatch_shadow_mod_exit);
