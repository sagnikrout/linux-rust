//! Automatically rewritten from C to Rust
//! Source: kernel/locking/rwbase_rt.c
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
// RT-specific reader/writer semaphores and reader/writer locks
//
// down_write/write_lock()
// 1) Lock rtmutex
// 2) Remove the reader BIAS to force readers into the slow path
// 3) Wait until all readers have left the critical section
// 4) Mark it write locked
//
// up_write/write_unlock()
// 1) Remove the write locked marker
// 2) Set the reader BIAS, so readers can use the fast path again
// 3) Unlock rtmutex, to release blocked readers
//
// down_read/read_lock()
// 1) Try fast path acquisition (reader BIAS is set)
// 2) Take tmutex::wait_lock, which protects the writelocked flag
// 3) If !writelocked, acquire it for read
// 4) If writelocked, block on tmutex
// 5) unlock rtmutex, goto 1)
//
// up_read/read_unlock()
// 1) Try fast path release (reader count != 1)
// 2) Wake the writer waiting in down_write()/write_lock() #3
//
// down_read/read_lock()#3 has the consequence, that rw semaphores and rw
// locks on RT are not writer fair, but writers, which should be avoided in
// RT tasks (think mmap_sem), are subject to the rtmutex priority/DL
// inheritance mechanism.
//
// It's possible to make the rw primitives writer fair by keeping a list of
// active readers. A blocked writer would force all newly incoming readers
// to block on the rtmutex, but the rtmutex would have to be proxy locked
// for one reader after the other. We can't use multi-reader inheritance
// because there is no way to support that with SCHED_DEADLINE.
// Implementing the one by one reader boosting/handover mechanism is a
// major surgery for a very dubious value.
//
// The risk of writer starvation is there, but the pathological use cases
// which trigger it are not necessarily the typical RT workloads.
//
// Fast-path orderings:
// The lock/unlock of readers can run in fast paths: lock and unlock are only
// atomic ops, and there is no inner lock to provide ACQUIRE and RELEASE
// semantics of rwbase_rt. Atomic ops should thus provide _acquire()
// and _release() (or stronger).
//
// Common code shared between RT rw_semaphore and rwlock
//
#[no_mangle]
unsafe extern "C" fn rwbase_read_trylock(rwb: *mut rwbase_rt) -> __always_inline int {
    let mut r = 0;
//
// Increment reader count, if sem->readers < 0, i.e. READER_BIAS is
// set.
//
    while (r < 0) {
    if (likely(atomic_try_cmpxchg_acquire(&rwb.readers, &r, r + 1))) {
    return 1;
    }
    }
    return 0;
    }
    static int __sched __rwbase_read_lock(rwbase_rt *rwb,
    unsigned int state)
    {
    let mut rtm = &rwb.rtmutex;
pub static mut wake_q: usize = 0;
    let mut ret = 0;
    rwbase_pre_schedule();
    raw_spin_lock_irq(&rtm.wait_lock);
//
// Call into the slow lock path with the rtmutex->wait_lock
// held, so this can't result in the following race:
//
// Reader1		Reader2		Writer
// down_read()
// down_write()
// rtmutex_lock(m)
// wait()
// down_read()
// unlock(m->wait_lock)
// up_read()
// wake(Writer)
// lock(m->wait_lock)
// sem->writelocked=true
// unlock(m->wait_lock)
//
// up_write()
// sem->writelocked=false
// rtmutex_unlock(m)
// down_read()
// down_write()
// rtmutex_lock(m)
// wait()
// rtmutex_lock(m)
//
// That would put Reader1 behind the writer waiting on
// Reader2 to call up_read(), which might be unbound.
//
    trace_contention_begin(rwb, LCB_F_RT | LCB_F_READ);
//
// For rwlocks this returns 0 unconditionally, so the below
// !ret conditionals are optimized out.
//
    ret = rwbase_rtmutex_slowlock_locked(rtm, state, &wake_q);
//
// On success the rtmutex is held, so there can't be a writer
// active. Increment the reader count and immediately drop the
// rtmutex again.
//
// rtmutex->wait_lock has to be unlocked in any case of course.
//
    if (!ret) {
    atomic_inc(&rwb.readers);
    }
    preempt_disable();
    raw_spin_unlock_irq(&rtm.wait_lock);
    wake_up_q(&wake_q);
    preempt_enable();
    if (!ret) {
    rwbase_rtmutex_unlock(rtm);
    }
    trace_contention_end(rwb, ret);
    rwbase_post_schedule();
    return ret;
    }
    static __always_inline int rwbase_read_lock(rwbase_rt *rwb,
    unsigned int state)
    {
    lockdep_assert(!current.pi_blocked_on);
    if (rwbase_read_trylock(rwb)) {
    return 0;
    }
    return __rwbase_read_lock(rwb, state);
    }
    static void __sched __rwbase_read_unlock(rwbase_rt *rwb,
    unsigned int state)
    {
    let mut rtm = &rwb.rtmutex;
pub static mut owner: *mut c_void = core::ptr::null_mut();
pub static mut wqh: usize = 0;
    raw_spin_lock_irq(&rtm.wait_lock);
//
// Wake the writer, i.e. the rtmutex owner. It might release the
// rtmutex concurrently in the fast path (due to a signal), but to
// clean up rwb->readers it needs to acquire rtm->wait_lock. The
// worst case which can happen is a spurious wakeup.
//
    owner = rt_mutex_owner(rtm);
    if (owner) {
    rt_mutex_wake_q_add_task(&wqh, owner, state);
    }
// Pairs with the preempt_enable in rt_mutex_wake_up_q()
    preempt_disable();
    raw_spin_unlock_irq(&rtm.wait_lock);
    rt_mutex_wake_up_q(&wqh);
    }
    static __always_inline void rwbase_read_unlock(rwbase_rt *rwb,
    unsigned int state)
    {
    if (trace_contended_release_enabled() && rt_mutex_owner(&rwb.rtmutex)) {
    trace_call__contended_release(rwb);
    }
//
// rwb->readers can only hit 0 when a writer is waiting for the
// active readers to leave the critical section.
//
// dec_and_test() is fully ordered, provides RELEASE.
//
    if (unlikely(atomic_dec_and_test(&rwb.readers))) {
    __rwbase_read_unlock(rwb, state);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __rwbase_write_unlock(rwb: *mut rwbase_rt, bias: c_int, wait_lock: unsigned long flags)
    __releases(&rwb.rtmutex.) {
    let mut rtm = &rwb.rtmutex;
//
// _release() is needed in case that reader is in fast path, pairing
// with atomic_try_cmpxchg_acquire() in rwbase_read_trylock().
//
    (void)atomic_add_return_release(READER_BIAS - bias, &rwb.readers);
    raw_spin_unlock_irqrestore(&rtm.wait_lock, flags);
    rwbase_rtmutex_unlock(rtm);
    }
#[no_mangle]
pub unsafe extern "C" fn rwbase_write_unlock(rwb: *mut rwbase_rt) {
    let mut rtm = &rwb.rtmutex;
    let mut flags = 0;
    raw_spin_lock_irqsave(&rtm.wait_lock, flags);
    if (trace_contended_release_enabled() && rt_mutex_has_waiters(rtm)) {
    trace_call__contended_release(rwb);
    }
    __rwbase_write_unlock(rwb, WRITER_BIAS, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn rwbase_write_downgrade(rwb: *mut rwbase_rt) {
    let mut rtm = &rwb.rtmutex;
    let mut flags = 0;
    raw_spin_lock_irqsave(&rtm.wait_lock, flags);
    if (trace_contended_release_enabled() && rt_mutex_has_waiters(rtm)) {
    trace_call__contended_release(rwb);
    }
// Release it and account current as reader
    __rwbase_write_unlock(rwb, WRITER_BIAS - 1, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __rwbase_write_trylock(rwb: *mut rwbase_rt) -> bool {
// Can do without CAS because we're serialized by wait_lock.
    lockdep_assert_held(&rwb.rtmutex.wait_lock);
//
// _acquire is needed in case the reader is in the fast path, pairing
// with rwbase_read_unlock(), provides ACQUIRE.
//
    if (!atomic_read_acquire(&rwb.readers)) {
    atomic_set(&rwb.readers, WRITER_BIAS);
    return 1;
    }
    return 0;
    }
    static int __sched rwbase_write_lock(rwbase_rt *rwb,
    unsigned int state)
    {
    let mut rtm = &rwb.rtmutex;
    let mut flags = 0;
// Take the rtmutex as a first step
    if (rwbase_rtmutex_lock_state(rtm, state)) {
    return -EINTR;
    }
// Force readers into slow path
    atomic_sub(READER_BIAS, &rwb.readers);
    rwbase_pre_schedule();
    raw_spin_lock_irqsave(&rtm.wait_lock, flags);
    if (__rwbase_write_trylock(rwb)) {
// goto;
    }
    rwbase_set_and_save_current_state(state);
    trace_contention_begin(rwb, LCB_F_RT | LCB_F_WRITE);
    for (;;) {
// Optimized out for rwlocks
    if (rwbase_signal_pending_state(state, current)) {
    rwbase_restore_current_state();
    __rwbase_write_unlock(rwb, 0, flags);
    rwbase_post_schedule();
    trace_contention_end(rwb, -EINTR);
    return -EINTR;
    }
    if (__rwbase_write_trylock(rwb)) {
    break;
    }
    raw_spin_unlock_irqrestore(&rtm.wait_lock, flags);
    rwbase_schedule();
    raw_spin_lock_irqsave(&rtm.wait_lock, flags);
    set_current_state(state);
    }
    rwbase_restore_current_state();
    trace_contention_end(rwb, 0);
// label;
    raw_spin_unlock_irqrestore(&rtm.wait_lock, flags);
    rwbase_post_schedule();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rwbase_write_trylock(rwb: *mut rwbase_rt) -> c_int {
    let mut rtm = &rwb.rtmutex;
    let mut flags = 0;
    if (!rwbase_rtmutex_trylock(rtm)) {
    return 0;
    }
    atomic_sub(READER_BIAS, &rwb.readers);
    raw_spin_lock_irqsave(&rtm.wait_lock, flags);
    if (__rwbase_write_trylock(rwb)) {
    raw_spin_unlock_irqrestore(&rtm.wait_lock, flags);
    return 1;
    }
    __rwbase_write_unlock(rwb, 0, flags);
    return 0;
    }