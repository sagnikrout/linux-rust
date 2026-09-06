//! Automatically rewritten from C to Rust
//! Source: kernel/locking/semaphore.c
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
// Copyright (c) 2008 Intel Corporation
// Author: Matthew Wilcox <willy@linux.intel.com>
//
// This file implements counting semaphores.
// A counting semaphore may be acquired 'n' times before sleeping.
// See mutex.c for single-acquisition sleeping locks which enforce
// rules which allow code to be debugged more easily.
//
// Some notes on the implementation:
//
// The spinlock controls access to the other members of the semaphore.
// down_trylock() and up() can be called from interrupt context, so we
// have to disable interrupts when taking the lock.  It turns out various
// parts of the kernel expect to be able to use down() on a semaphore in
// interrupt context when they know it will succeed, so we have to use
// irqsave variants for down(), down_interruptible() and down_killable()
// too.
//
// The ->count variable represents how many more tasks can acquire this
// semaphore.  If it's zero, there may be waiters.
//

    static noinline void __down(struct semaphore *sem);
    static noinline int __down_interruptible(struct semaphore *sem);
    static noinline int __down_killable(struct semaphore *sem);
    static noinline int __down_timeout(struct semaphore *sem, long timeout);
    static noinline void __up(struct semaphore *sem, struct wake_q_head *wake_q);

#[no_mangle]
pub unsafe extern "C" fn hung_task_sem_set_holder(sem: *mut semaphore) {
    static inline void hung_task_sem_set_holder(struct semaphore *sem)
    {
    WRITE_ONCE((sem).last_holder, (unsigned long)current);
    }
#[no_mangle]
pub unsafe extern "C" fn hung_task_sem_clear_if_holder(sem: *mut semaphore) {
    static inline void hung_task_sem_clear_if_holder(struct semaphore *sem)
    {
    if (READ_ONCE((sem).last_holder) == (unsigned long)current)
    WRITE_ONCE((sem).last_holder, 0UL);
    }
#[no_mangle]
pub unsafe extern "C" fn sem_last_holder(sem: *mut semaphore) -> c_ulong {
    unsigned long sem_last_holder(struct semaphore *sem)
    {
    return READ_ONCE(sem.last_holder);
    }

#[no_mangle]
pub unsafe extern "C" fn hung_task_sem_set_holder(sem: *mut semaphore) {
    static inline void hung_task_sem_set_holder(struct semaphore *sem)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn hung_task_sem_clear_if_holder(sem: *mut semaphore) {
    static inline void hung_task_sem_clear_if_holder(struct semaphore *sem)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn sem_last_holder(sem: *mut semaphore) -> c_ulong {
    unsigned long sem_last_holder(struct semaphore *sem)
    {
    return 0UL;
    }

#[no_mangle]
pub unsafe extern "C" fn __sem_acquire(sem: *mut semaphore) {
    static inline void __sem_acquire(struct semaphore *sem)
    {
    sem.count--;
    hung_task_sem_set_holder(sem);
    }
//
// down - acquire the semaphore
// @sem: the semaphore to be acquired
//
// Acquires the semaphore.  If no more tasks are allowed to acquire the
// semaphore, calling this function will put the task to sleep until the
// semaphore is released.
//
// Use of this function is deprecated, please use down_interruptible() or
// down_killable() instead.
//
#[no_mangle]
pub unsafe extern "C" fn down(sem: *mut semaphore) -> void __sched {
    void __sched down(struct semaphore *sem)
    {
    unsigned long flags;
    might_sleep();
    raw_spin_lock_irqsave(&sem.lock, flags);
    if (likely(sem.count > 0))
    __sem_acquire(sem);
    else
    __down(sem);
    raw_spin_unlock_irqrestore(&sem.lock, flags);
    }
    EXPORT_SYMBOL(down);
//
// down_interruptible - acquire the semaphore unless interrupted
// @sem: the semaphore to be acquired
//
// Attempts to acquire the semaphore.  If no more tasks are allowed to
// acquire the semaphore, calling this function will put the task to sleep.
// If the sleep is interrupted by a signal, this function will return -EINTR.
// If the semaphore is successfully acquired, this function returns 0.
//
#[no_mangle]
pub unsafe extern "C" fn down_interruptible(sem: *mut semaphore) -> int __sched {
    int __sched down_interruptible(struct semaphore *sem)
    {
    unsigned long flags;
    let mut result: c_int = 0;
    might_sleep();
    raw_spin_lock_irqsave(&sem.lock, flags);
    if (likely(sem.count > 0))
    __sem_acquire(sem);
    else
    result = __down_interruptible(sem);
    raw_spin_unlock_irqrestore(&sem.lock, flags);
    return result;
    }
    EXPORT_SYMBOL(down_interruptible);
//
// down_killable - acquire the semaphore unless killed
// @sem: the semaphore to be acquired
//
// Attempts to acquire the semaphore.  If no more tasks are allowed to
// acquire the semaphore, calling this function will put the task to sleep.
// If the sleep is interrupted by a fatal signal, this function will return
// -EINTR.  If the semaphore is successfully acquired, this function returns
// 0.
//
#[no_mangle]
pub unsafe extern "C" fn down_killable(sem: *mut semaphore) -> int __sched {
    int __sched down_killable(struct semaphore *sem)
    {
    unsigned long flags;
    let mut result: c_int = 0;
    might_sleep();
    raw_spin_lock_irqsave(&sem.lock, flags);
    if (likely(sem.count > 0))
    __sem_acquire(sem);
    else
    result = __down_killable(sem);
    raw_spin_unlock_irqrestore(&sem.lock, flags);
    return result;
    }
    EXPORT_SYMBOL(down_killable);
//
// down_trylock - try to acquire the semaphore, without waiting
// @sem: the semaphore to be acquired
//
// Try to acquire the semaphore atomically.  Returns 0 if the semaphore has
// been acquired successfully or 1 if it cannot be acquired.
//
// NOTE: This return value is inverted from both spin_trylock and
// mutex_trylock!  Be careful about this when converting code.
//
// Unlike mutex_trylock, this function can be used from interrupt context,
// and the semaphore can be released by any task or interrupt.
//
#[no_mangle]
pub unsafe extern "C" fn down_trylock(sem: *mut semaphore) -> int __sched {
    int __sched down_trylock(struct semaphore *sem)
    {
    unsigned long flags;
    int count;
    raw_spin_lock_irqsave(&sem.lock, flags);
    count = sem.count - 1;
    if (likely(count >= 0))
    __sem_acquire(sem);
    raw_spin_unlock_irqrestore(&sem.lock, flags);
    return (count < 0);
    }
    EXPORT_SYMBOL(down_trylock);
//
// down_timeout - acquire the semaphore within a specified time
// @sem: the semaphore to be acquired
// @timeout: how long to wait before failing
//
// Attempts to acquire the semaphore.  If no more tasks are allowed to
// acquire the semaphore, calling this function will put the task to sleep.
// If the semaphore is not released within the specified number of jiffies,
// this function returns -ETIME.  It returns 0 if the semaphore was acquired.
//
#[no_mangle]
pub unsafe extern "C" fn down_timeout(sem: *mut semaphore, timeout: c_long) -> int __sched {
    int __sched down_timeout(struct semaphore *sem, long timeout)
    {
    unsigned long flags;
    let mut result: c_int = 0;
    might_sleep();
    raw_spin_lock_irqsave(&sem.lock, flags);
    if (likely(sem.count > 0))
    __sem_acquire(sem);
    else
    result = __down_timeout(sem, timeout);
    raw_spin_unlock_irqrestore(&sem.lock, flags);
    return result;
    }
    EXPORT_SYMBOL(down_timeout);
//
// up - release the semaphore
// @sem: the semaphore to release
//
// Release the semaphore.  Unlike mutexes, up() may be called from any
// context and even by tasks which have never called down().
//
#[no_mangle]
pub unsafe extern "C" fn up(sem: *mut semaphore) -> void __sched {
    void __sched up(struct semaphore *sem)
    {
    unsigned long flags;
    DEFINE_WAKE_Q(wake_q);
    raw_spin_lock_irqsave(&sem.lock, flags);
    hung_task_sem_clear_if_holder(sem);
    if (likely(!sem.first_waiter))
    sem.count++;
    else
    __up(sem, &wake_q);
    if (trace_contended_release_enabled() && !wake_q_empty(&wake_q))
    trace_call__contended_release(sem);
    raw_spin_unlock_irqrestore(&sem.lock, flags);
    if (!wake_q_empty(&wake_q))
    wake_up_q(&wake_q);
    }
    EXPORT_SYMBOL(up);
// Functions for the contended case
#[repr(C)]
#[derive(Copy, Clone)]
pub struct semaphore_waiter {
    pub list: list_head,
    pub task: *mut task_struct,
    pub up: bool,
}

    static inline
#[no_mangle]
pub unsafe extern "C" fn sem_del_waiter(sem: *mut semaphore, waiter: *mut semaphore_waiter) {
    void sem_del_waiter(struct semaphore *sem, struct semaphore_waiter *waiter)
    {
    if (list_empty(&waiter.list)) {
    sem.first_waiter = core::ptr::null_mut();
    return;
    }
    if (sem.first_waiter == waiter) {
    sem.first_waiter = list_first_entry(&waiter.list,
    struct semaphore_waiter, list);
    }
    list_del(&waiter.list);
    }
//
// Because this function is inlined, the 'state' parameter will be
// constant, and thus optimised away by the compiler.  Likewise the
// 'timeout' parameter for the cases without timeouts.
//
    static inline int __sched ___down_common(struct semaphore *sem, long state,
    long timeout)
    {
    struct semaphore_waiter waiter, *first;
    first = sem.first_waiter;
    if (first) {
    list_add_tail(&waiter.list, &first.list);
    } else {
    INIT_LIST_HEAD(&waiter.list);
    sem.first_waiter = &waiter;
    }
    waiter.task = current;
    waiter.up = false;
    for (;;) {
    if (signal_pending_state(state, current))
    goto interrupted;
    if (unlikely(timeout <= 0))
    goto timed_out;
    __set_current_state(state);
    raw_spin_unlock_irq(&sem.lock);
    timeout = schedule_timeout(timeout);
    raw_spin_lock_irq(&sem.lock);
    if (waiter.up) {
    hung_task_sem_set_holder(sem);
    return 0;
    }
    }
    timed_out:
    sem_del_waiter(sem, &waiter);
    return -ETIME;
    interrupted:
    sem_del_waiter(sem, &waiter);
    return -EINTR;
    }
    static inline int __sched __down_common(struct semaphore *sem, long state,
    long timeout)
    {
    int ret;
    hung_task_set_blocker(sem, BLOCKER_TYPE_SEM);
    trace_contention_begin(sem, 0);
    ret = ___down_common(sem, state, timeout);
    trace_contention_end(sem, ret);
    hung_task_clear_blocker();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __down(sem: *mut semaphore) -> noinline void __sched {
    static noinline void __sched __down(struct semaphore *sem)
    {
    __down_common(sem, TASK_UNINTERRUPTIBLE, MAX_SCHEDULE_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn __down_interruptible(sem: *mut semaphore) -> noinline int __sched {
    static noinline int __sched __down_interruptible(struct semaphore *sem)
    {
    return __down_common(sem, TASK_INTERRUPTIBLE, MAX_SCHEDULE_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn __down_killable(sem: *mut semaphore) -> noinline int __sched {
    static noinline int __sched __down_killable(struct semaphore *sem)
    {
    return __down_common(sem, TASK_KILLABLE, MAX_SCHEDULE_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn __down_timeout(sem: *mut semaphore, timeout: c_long) -> noinline int __sched {
    static noinline int __sched __down_timeout(struct semaphore *sem, long timeout)
    {
    return __down_common(sem, TASK_UNINTERRUPTIBLE, timeout);
    }
    static noinline void __sched __up(struct semaphore *sem,
    struct wake_q_head *wake_q)
    {
    struct semaphore_waiter *waiter = sem.first_waiter;
    sem_del_waiter(sem, waiter);
    waiter.up = true;
    wake_q_add(wake_q, waiter.task);
    }
