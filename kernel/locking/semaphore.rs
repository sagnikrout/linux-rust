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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


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

    static noinline void __down(semaphore *sem);
    static noinline int __down_interruptible(semaphore *sem);
    static noinline int __down_killable(semaphore *sem);
    static noinline int __down_timeout(semaphore *sem, long timeout);
    static noinline void __up(semaphore *sem, wake_q_head *wake_q);

#[no_mangle]
pub unsafe extern "C" fn hung_task_sem_set_holder(sem: *mut semaphore) {
    WRITE_ONCE((sem).last_holder, (unsigned long)current);
    }
#[no_mangle]
pub unsafe extern "C" fn hung_task_sem_clear_if_holder(sem: *mut semaphore) {
    if (READ_ONCE((sem).last_holder) == (unsigned long)current) {
    WRITE_ONCE((sem).last_holder, 0UL);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sem_last_holder(sem: *mut semaphore) -> c_ulong {
    return READ_ONCE(sem.last_holder);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: hung_task_sem_set_holder
pub unsafe extern "C" fn hung_task_sem_set_holder_dup(sem: *mut semaphore) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: hung_task_sem_clear_if_holder
pub unsafe extern "C" fn hung_task_sem_clear_if_holder_dup(sem: *mut semaphore) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sem_last_holder
pub unsafe extern "C" fn sem_last_holder_dup(sem: *mut semaphore) -> c_ulong {
    return 0UL;
    }

#[no_mangle]
pub unsafe extern "C" fn __sem_acquire(sem: *mut semaphore) {
    sem.count -= 1;
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
    let mut flags = 0;
    might_sleep();
    raw_spin_lock_irqsave(&sem.lock, flags);
    if (likely(sem.count > 0)) {
    __sem_acquire(sem);
    }
    else {
    __down(sem);
    }
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
    let mut flags = 0;
pub static mut result: c_int = 0;
    might_sleep();
    raw_spin_lock_irqsave(&sem.lock, flags);
    if (likely(sem.count > 0)) {
    __sem_acquire(sem);
    }
    else {
    result = __down_interruptible(sem);
    }
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
    let mut flags = 0;
pub static mut result: c_int = 0;
    might_sleep();
    raw_spin_lock_irqsave(&sem.lock, flags);
    if (likely(sem.count > 0)) {
    __sem_acquire(sem);
    }
    else {
    result = __down_killable(sem);
    }
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
    let mut flags = 0;
    let mut count = 0;
    raw_spin_lock_irqsave(&sem.lock, flags);
    count = sem.count - 1;
    if (likely(count >= 0)) {
    __sem_acquire(sem);
    }
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
    let mut flags = 0;
pub static mut result: c_int = 0;
    might_sleep();
    raw_spin_lock_irqsave(&sem.lock, flags);
    if (likely(sem.count > 0)) {
    __sem_acquire(sem);
    }
    else {
    result = __down_timeout(sem, timeout);
    }
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
    let mut flags = 0;
pub static mut wake_q: usize = 0;
    raw_spin_lock_irqsave(&sem.lock, flags);
    hung_task_sem_clear_if_holder(sem);
    if (likely(!sem.first_waiter)) {
    sem.count += 1;
    }
    else {
    __up(sem, &wake_q);
    }
    if (trace_contended_release_enabled() && !wake_q_empty(&wake_q)) {
    trace_call__contended_release(sem);
    }
    raw_spin_unlock_irqrestore(&sem.lock, flags);
    if (!wake_q_empty(&wake_q)) {
    wake_up_q(&wake_q);
    }
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
    if (list_empty(&waiter.list)) {
    sem.first_waiter = core::ptr::null_mut();
    return;
    }
    if (sem.first_waiter == waiter) {
    sem.first_waiter = list_first_entry(&waiter.list, semaphore_waiter, list);
    }
    list_del(&waiter.list);
    }
//
// Because this function is inlined, the 'state' parameter will be
// constant, and thus optimised away by the compiler.  Likewise the
// 'timeout' parameter for the cases without timeouts.
//
    static inline int __sched ___down_common(semaphore *sem, long state,
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
    if (signal_pending_state(state, current)) {
// goto;
    }
    if (unlikely(timeout <= 0)) {
// goto;
    }
    __set_current_state(state);
    raw_spin_unlock_irq(&sem.lock);
    timeout = schedule_timeout(timeout);
    raw_spin_lock_irq(&sem.lock);
    if (waiter.up) {
    hung_task_sem_set_holder(sem);
    return 0;
    }
    }
// label;
    sem_del_waiter(sem, &waiter);
    return -ETIME;
// label;
    sem_del_waiter(sem, &waiter);
    return -EINTR;
    }
    static inline int __sched __down_common(semaphore *sem, long state,
    long timeout)
    {
    let mut ret = 0;
    hung_task_set_blocker(sem, BLOCKER_TYPE_SEM);
    trace_contention_begin(sem, 0);
    ret = ___down_common(sem, state, timeout);
    trace_contention_end(sem, ret);
    hung_task_clear_blocker();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __down(sem: *mut semaphore) -> noinline void __sched {
    __down_common(sem, TASK_UNINTERRUPTIBLE, MAX_SCHEDULE_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn __down_interruptible(sem: *mut semaphore) -> noinline int __sched {
    return __down_common(sem, TASK_INTERRUPTIBLE, MAX_SCHEDULE_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn __down_killable(sem: *mut semaphore) -> noinline int __sched {
    return __down_common(sem, TASK_KILLABLE, MAX_SCHEDULE_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn __down_timeout(sem: *mut semaphore, timeout: c_long) -> noinline int __sched {
    return __down_common(sem, TASK_UNINTERRUPTIBLE, timeout);
    }
    static noinline void __sched __up(semaphore *sem, wake_q_head *wake_q)
    {
    let mut waiter = sem.first_waiter;
    sem_del_waiter(sem, waiter);
    waiter.up = true;
    wake_q_add(wake_q, waiter.task);
    }