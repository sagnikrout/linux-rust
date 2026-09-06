//! Automatically rewritten from C to Rust
//! Source: kernel/locking/ww_rt_mutex.c
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
// rtmutex API
//

// Macro flag: #define RT_MUTEX_BUILD_MUTEX
// Macro flag: #define WW_RT

#[no_mangle]
pub unsafe extern "C" fn ww_mutex_trylock(lock: *mut ww_mutex, ww_ctx: *mut ww_acquire_ctx) -> c_int {
    int ww_mutex_trylock(struct ww_mutex *lock, struct ww_acquire_ctx *ww_ctx)
    {
    struct rt_mutex *rtm = &lock.base;
    if (!ww_ctx)
    return rt_mutex_trylock(rtm);
//
// Reset the wounded flag after a kill. No other process can
// race and wound us here, since they can't have a valid owner
// pointer if we don't have any locks held.
//
    if (ww_ctx.acquired == 0)
    ww_ctx.wounded = 0;
    if (__rt_mutex_trylock(&rtm.rtmutex)) {
    ww_mutex_set_context_fastpath(lock, ww_ctx);
    mutex_acquire_nest(&rtm.dep_map, 0, 1, &ww_ctx.dep_map, _RET_IP_);
    return 1;
    }
    return 0;
    }
    EXPORT_SYMBOL(ww_mutex_trylock);
    static int __sched
    __ww_rt_mutex_lock(struct ww_mutex *lock, struct ww_acquire_ctx *ww_ctx,
    unsigned int state, unsigned long ip)
    {
    struct lockdep_map __maybe_unused *nest_lock = core::ptr::null_mut();
    struct rt_mutex *rtm = &lock.base;
    int ret;
    might_sleep();
    if (ww_ctx) {
    if (unlikely(ww_ctx == READ_ONCE(lock.ctx)))
    return -EALREADY;
//
// Reset the wounded flag after a kill. No other process can
// race and wound us here, since they can't have a valid owner
// pointer if we don't have any locks held.
//
    if (ww_ctx.acquired == 0)
    ww_ctx.wounded = 0;

    nest_lock = &ww_ctx.dep_map;

    }
    mutex_acquire_nest(&rtm.dep_map, 0, 0, nest_lock, ip);
    if (likely(rt_mutex_try_acquire(&rtm.rtmutex))) {
    if (ww_ctx)
    ww_mutex_set_context_fastpath(lock, ww_ctx);
    return 0;
    }
    ret = rt_mutex_slowlock(&rtm.rtmutex, ww_ctx, state);
    if (ret)
    mutex_release(&rtm.dep_map, ip);
    return ret;
    }
    int __sched
    ww_mutex_lock(struct ww_mutex *lock, struct ww_acquire_ctx *ctx)
    {
    return __ww_rt_mutex_lock(lock, ctx, TASK_UNINTERRUPTIBLE, _RET_IP_);
    }
    EXPORT_SYMBOL(ww_mutex_lock);
    int __sched
    ww_mutex_lock_interruptible(struct ww_mutex *lock, struct ww_acquire_ctx *ctx)
    {
    return __ww_rt_mutex_lock(lock, ctx, TASK_INTERRUPTIBLE, _RET_IP_);
    }
    EXPORT_SYMBOL(ww_mutex_lock_interruptible);
#[no_mangle]
pub unsafe extern "C" fn ww_mutex_unlock(lock: *mut ww_mutex) -> void __sched {
    void __sched ww_mutex_unlock(struct ww_mutex *lock)
    __no_context_analysis
    {
    struct rt_mutex *rtm = &lock.base;
    __ww_mutex_unlock(lock);
    mutex_release(&rtm.dep_map, _RET_IP_);
    __rt_mutex_unlock(&rtm.rtmutex);
    }
    EXPORT_SYMBOL(ww_mutex_unlock);
