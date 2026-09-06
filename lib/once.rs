//! Automatically rewritten from C to Rust
//! Source: lib/once.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct once_work {
    pub work: work_struct,
    pub key: *mut static_key_true,
    pub module: *mut module,
}

#[no_mangle]
unsafe extern "C" fn once_deferred(w: *mut work_struct) {
    static void once_deferred(struct work_struct *w)
    {
    struct once_work *work;
    work = container_of(w, struct once_work, work);
    BUG_ON(!static_key_enabled(work.key));
    static_branch_disable(work.key);
    module_put(work.module);
    kfree(work);
    }
#[no_mangle]
unsafe extern "C" fn once_disable_jump(key: *mut static_key_true, mod: *mut module) {
    static void once_disable_jump(struct static_key_true *key, struct module *mod)
    {
    struct once_work *w;
    w = kmalloc_obj(*w, GFP_ATOMIC);
    if (!w)
    return;
    INIT_WORK(&w.work, once_deferred);
    w.key = key;
    w.module = mod;
    __module_get(mod);
    schedule_work(&w.work);
    }
    static DEFINE_SPINLOCK(once_lock);
#[no_mangle]
pub unsafe extern "C" fn __do_once_start(done: *mut bool, flags: *mut c_ulong) -> bool {
    bool __do_once_start(bool *done, unsigned long *flags)
    __acquires(once_lock)
    {
    spin_lock_irqsave(&once_lock, *flags);
    if (*done) {
    spin_unlock_irqrestore(&once_lock, *flags);
// Keep sparse happy by restoring an even lock count on
// this lock. In case we return here, we don't call into
// __do_once_done but return early in the DO_ONCE() macro.
//
    __acquire(once_lock);
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL(__do_once_start);
    void __do_once_done(bool *done, struct static_key_true *once_key,
    unsigned long *flags, struct module *mod)
    __releases(once_lock)
    {
// done = true;
    spin_unlock_irqrestore(&once_lock, *flags);
    once_disable_jump(once_key, mod);
    }
    EXPORT_SYMBOL(__do_once_done);
    static DEFINE_MUTEX(once_mutex);
#[no_mangle]
pub unsafe extern "C" fn __do_once_sleepable_start(done: *mut bool) -> bool {
    bool __do_once_sleepable_start(bool *done)
    __acquires(once_mutex)
    {
    mutex_lock(&once_mutex);
    if (*done) {
    mutex_unlock(&once_mutex);
// Keep sparse happy by restoring an even lock count on
// this mutex. In case we return here, we don't call into
// __do_once_done but return early in the DO_ONCE_SLEEPABLE() macro.
//
    __acquire(once_mutex);
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL(__do_once_sleepable_start);
    void __do_once_sleepable_done(bool *done, struct static_key_true *once_key,
    struct module *mod)
    __releases(once_mutex)
    {
// done = true;
    mutex_unlock(&once_mutex);
    once_disable_jump(once_key, mod);
    }
    EXPORT_SYMBOL(__do_once_sleepable_done);
