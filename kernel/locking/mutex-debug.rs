//! Automatically rewritten from C to Rust
//! Source: kernel/locking/mutex-debug.c
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


//
// Debugging code for mutexes
//
// Started by Ingo Molnar:
//
// Copyright (C) 2004, 2005, 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
//
// lock debugging, locking tree, deadlock detection started by:
//
// Copyright (C) 2004, LynuxWorks, Inc., Igor Manyilov, Bill Huey
// Released under the General Public License (GPL).
//

//
// Must be called with lock->wait_lock held.
//
#[no_mangle]
pub unsafe extern "C" fn debug_mutex_lock_common(lock: *mut mutex, waiter: *mut mutex_waiter) {
    void debug_mutex_lock_common(struct mutex *lock, struct mutex_waiter *waiter)
    {
    memset(waiter, MUTEX_DEBUG_INIT, sizeof(*waiter));
    waiter.magic = waiter;
    INIT_LIST_HEAD(&waiter.list);
    waiter.ww_ctx = MUTEX_POISON_WW_CTX;
    }
#[no_mangle]
pub unsafe extern "C" fn debug_mutex_wake_waiter(lock: *mut mutex, waiter: *mut mutex_waiter) {
    void debug_mutex_wake_waiter(struct mutex *lock, struct mutex_waiter *waiter)
    {
    lockdep_assert_held(&lock.wait_lock);
    DEBUG_LOCKS_WARN_ON(!lock.first_waiter);
    DEBUG_LOCKS_WARN_ON(waiter.magic != waiter);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_mutex_free_waiter(waiter: *mut mutex_waiter) {
    void debug_mutex_free_waiter(struct mutex_waiter *waiter)
    {
    DEBUG_LOCKS_WARN_ON(!list_empty(&waiter.list));
    memset(waiter, MUTEX_DEBUG_FREE, sizeof(*waiter));
    }
    void debug_mutex_add_waiter(struct mutex *lock, struct mutex_waiter *waiter,
    struct task_struct *task)
    {
    lockdep_assert_held(&lock.wait_lock);
// Current thread can't be already blocked (since it's executing!)
    DEBUG_LOCKS_WARN_ON(get_task_blocked_on(task));
    }
    void debug_mutex_remove_waiter(struct mutex *lock, struct mutex_waiter *waiter,
    struct task_struct *task)
    {
    struct mutex *blocked_on = get_task_blocked_on(task);
    DEBUG_LOCKS_WARN_ON(waiter.task != task);
    DEBUG_LOCKS_WARN_ON(blocked_on && blocked_on != lock);
    INIT_LIST_HEAD(&waiter.list);
    waiter.task = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn debug_mutex_unlock(lock: *mut mutex) {
    void debug_mutex_unlock(struct mutex *lock)
    {
    if (likely(debug_locks)) {
    DEBUG_LOCKS_WARN_ON(lock.magic != lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn debug_mutex_init(lock: *mut mutex) {
    void debug_mutex_init(struct mutex *lock)
    {
    lock.magic = lock;
    }
#[no_mangle]
unsafe extern "C" fn devm_mutex_release(res: *mut c_void) {
    static void devm_mutex_release(void *res)
    {
    mutex_destroy(res);
    }
#[no_mangle]
pub unsafe extern "C" fn __devm_mutex_init(dev: *mut device, lock: *mut mutex) -> c_int {
    int __devm_mutex_init(struct device *dev, struct mutex *lock)
    {
    return devm_add_action_or_reset(dev, devm_mutex_release, lock);
    }
    EXPORT_SYMBOL_GPL(__devm_mutex_init);
//
// mutex_destroy - mark a mutex unusable
// @lock: the mutex to be destroyed
//
// This function marks the mutex uninitialized, and any subsequent
// use of the mutex is forbidden. The mutex must not be locked when
// this function is called.
//
#[no_mangle]
pub unsafe extern "C" fn mutex_destroy(lock: *mut mutex) {
    void mutex_destroy(struct mutex *lock)
    {
    DEBUG_LOCKS_WARN_ON(mutex_is_locked(lock));
    lock.magic = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(mutex_destroy);
