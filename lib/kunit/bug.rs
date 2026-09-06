//! Automatically rewritten from C to Rust
//! Source: lib/kunit/bug.c
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
// KUnit helpers for backtrace suppression
//
// Copyright (C) 2025 Alessandro Carminati <acarmina@redhat.com>
// Copyright (C) 2024 Guenter Roeck <linux@roeck-us.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_suppressed_warning {
    pub node: list_head,
    pub task: *mut task_struct,
    pub test: *mut kunit,
    pub counter: core::sync::atomic::AtomicI32,
}

    static LIST_HEAD(suppressed_warnings);
    static DEFINE_SPINLOCK(suppressed_warnings_lock);
#[no_mangle]
unsafe extern "C" fn kunit_suppress_warning_remove(w: *mut kunit_suppressed_warning) {
    static void kunit_suppress_warning_remove(struct kunit_suppressed_warning *w)
    {
    unsigned long flags;
    spin_lock_irqsave(&suppressed_warnings_lock, flags);
    list_del_rcu(&w.node);
    spin_unlock_irqrestore(&suppressed_warnings_lock, flags);
    put_task_struct(w.task);
    }
    KUNIT_DEFINE_ACTION_WRAPPER(kunit_suppress_warning_cleanup,
    kunit_suppress_warning_remove,
    struct kunit_suppressed_warning *);
#[no_mangle]
pub unsafe extern "C" fn kunit_has_active_suppress_warning() -> bool {
    bool kunit_has_active_suppress_warning(void)
    {
    return __kunit_is_suppressed_warning_impl(false);
    }
    EXPORT_SYMBOL_GPL(kunit_has_active_suppress_warning);
    struct kunit_suppressed_warning *
    kunit_start_suppress_warning(struct kunit *test)
    {
    struct kunit_suppressed_warning *w;
    unsigned long flags;
    int ret;
    if (kunit_has_active_suppress_warning()) {
    KUNIT_FAIL(test, "Another suppression block is already active");
    return core::ptr::null_mut();
    }
    w = kunit_kzalloc(test, sizeof(*w), GFP_KERNEL);
    if (!w) {
    KUNIT_FAIL(test, "Failed to allocate suppression handle.");
    return core::ptr::null_mut();
    }
    w.task = get_task_struct(current);
    w.test = test;
    spin_lock_irqsave(&suppressed_warnings_lock, flags);
    list_add_rcu(&w.node, &suppressed_warnings);
    spin_unlock_irqrestore(&suppressed_warnings_lock, flags);
    ret = kunit_add_action_or_reset(test,
    kunit_suppress_warning_cleanup, w);
    if (ret) {
    KUNIT_FAIL(test, "Failed to add suppression cleanup action.");
    return core::ptr::null_mut();
    }
    return w;
    }
    EXPORT_SYMBOL_GPL(kunit_start_suppress_warning);
    void kunit_end_suppress_warning(struct kunit *test,
    struct kunit_suppressed_warning *w)
    {
    if (!w)
    return;
    kunit_release_action(test, kunit_suppress_warning_cleanup, w);
    }
    EXPORT_SYMBOL_GPL(kunit_end_suppress_warning);
#[no_mangle]
pub unsafe extern "C" fn __kunit_suppress_auto_cleanup(wp: *mut kunit_suppressed_warning) {
    void __kunit_suppress_auto_cleanup(struct kunit_suppressed_warning **wp)
    {
    if (*wp)
    kunit_end_suppress_warning((*wp).test, *wp);
    }
    EXPORT_SYMBOL_GPL(__kunit_suppress_auto_cleanup);
#[no_mangle]
pub unsafe extern "C" fn kunit_suppressed_warning_count(w: *mut kunit_suppressed_warning) -> c_int {
    int kunit_suppressed_warning_count(struct kunit_suppressed_warning *w)
    {
    return w ? atomic_read(&w.counter) : 0;
    }
    EXPORT_SYMBOL_GPL(kunit_suppressed_warning_count);
#[no_mangle]
pub unsafe extern "C" fn __kunit_is_suppressed_warning_impl(count: bool) -> bool {
    bool __kunit_is_suppressed_warning_impl(bool count)
    {
    struct kunit_suppressed_warning *w;
    guard(rcu)();
    list_for_each_entry_rcu(w, &suppressed_warnings, node) {
    if (w.task == current) {
    if (count)
    atomic_inc(&w.counter);
    return true;
    }
    }
    return false;
    }
