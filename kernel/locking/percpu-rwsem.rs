//! Automatically rewritten from C to Rust
//! Source: kernel/locking/percpu-rwsem.c
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

#[no_mangle]
pub unsafe extern "C" fn __percpu_init_rwsem(sem: *mut percpu_rw_semaphore, name: *mut c_char, key: *mut lock_class_key) -> c_int {
    sem.read_count = alloc_percpu(int);
    if (unlikely(!sem.read_count)) {
    return -ENOMEM;
    }
    rcu_sync_init(&sem.rss);
    rcuwait_init(&sem.writer);
    init_waitqueue_head(&sem.waiters);
    atomic_set(&sem.block, 0);

    debug_check_no_locks_freed(sem, sizeof!(*sem));
    lockdep_init_map(&sem.dep_map, name, key, 0);

    return 0;
    }
    EXPORT_SYMBOL_GPL(__percpu_init_rwsem);
#[no_mangle]
pub unsafe extern "C" fn percpu_free_rwsem(sem: *mut percpu_rw_semaphore) {
//
// XXX: temporary kludge. The error path in alloc_super()
// assumes that percpu_free_rwsem() is safe after kzalloc().
//
    if (!sem.read_count) {
    return;
    }
    rcu_sync_dtor(&sem.rss);
    free_percpu(sem.read_count);
    sem.read_count = core::ptr::null_mut(); /* catch use after free bugs */
    }
    EXPORT_SYMBOL_GPL(percpu_free_rwsem);
#[no_mangle]
unsafe extern "C" fn __percpu_down_read_trylock(sem: *mut percpu_rw_semaphore) -> bool {
    this_cpu_inc(*sem.read_count);
//
// Due to having preemption disabled the decrement happens on
// the same CPU as the increment, avoiding the
// increment-on-one-CPU-and-decrement-on-another problem.
//
// If the reader misses the writer's assignment of sem->block, then the
// writer is guaranteed to see the reader's increment.
//
// Conversely, any readers that increment their sem->read_count after
// the writer looks are guaranteed to see the sem->block value, which
// in turn means that they are guaranteed to immediately decrement
// their sem->read_count, so that it doesn't matter that the writer
// missed them.
//
    smp_mb(); /* A matches D */
//
// If !sem->block the critical section starts here, matched by the
// release in percpu_up_write().
//
    if (likely(!atomic_read_acquire(&sem.block))) {
    return true;
    }
    this_cpu_dec(*sem.read_count);
// Prod writer to re-evaluate readers_active_check()
    rcuwait_wake_up(&sem.writer);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn __percpu_down_write_trylock(sem: *mut percpu_rw_semaphore) -> bool {
    if (atomic_read(&sem.block)) {
    return false;
    }
    return atomic_xchg(&sem.block, 1) == 0;
    }
#[no_mangle]
unsafe extern "C" fn __percpu_rwsem_trylock(sem: *mut percpu_rw_semaphore, reader: bool) -> bool {
    if (reader) {
    let mut ret = 0;
    preempt_disable();
    ret = __percpu_down_read_trylock(sem);
    preempt_enable();
    return ret;
    }
    return __percpu_down_write_trylock(sem);
    }
//
// The return value of wait_queue_entry::func means:
//
// <0 - error, wakeup is terminated and the error is returned
// 0 - no wakeup, a next waiter is tried
// >0 - woken, if EXCLUSIVE, counted towards @nr_exclusive.
//
// We use EXCLUSIVE for both readers and writers to preserve FIFO order,
// and play games with the return value to allow waking multiple readers.
//
// Specifically, we wake readers until we've woken a single writer, or until a
// trylock fails.
//
#[no_mangle]
pub unsafe extern "C" fn percpu_rwsem_wake_function(wq_entry: *mut wait_queue_entry, mode: c_uint, wake_flags: c_int, key: *mut c_void) -> c_int {
pub static mut reader: bool = false;
    let mut sem = key;
pub static mut p: *mut c_void = core::ptr::null_mut();
// concurrent against percpu_down_write(), can get stolen
    if (!__percpu_rwsem_trylock(sem, reader)) {
    return 1;
    }
    p = get_task_struct(wq_entry.private);
    list_del_init(&wq_entry.entry);
    smp_store_release(&wq_entry.private, core::ptr::null_mut());
    wake_up_process(p);
    put_task_struct(p);
    return !reader; /* wake (readers until) 1 writer */
    }
#[no_mangle]
pub unsafe extern "C" fn percpu_rwsem_wait(sem: *mut percpu_rw_semaphore, reader: bool, freeze: bool) {
pub static mut wq_entry: usize = 0;
    let mut wait = 0;
    spin_lock_irq(&sem.waiters.lock);
//
// Serialize against the wakeup in percpu_up_write(), if we fail
// the trylock, the wakeup must see us on the list.
//
    wait = !__percpu_rwsem_trylock(sem, reader);
    if (wait) {
    wq_entry.flags |= WQ_FLAG_EXCLUSIVE | reader * WQ_FLAG_CUSTOM;
    __add_wait_queue_entry_tail(&sem.waiters, &wq_entry);
    }
    spin_unlock_irq(&sem.waiters.lock);
    while (wait) {
    set_current_state(TASK_UNINTERRUPTIBLE |
    (freeze ? TASK_FREEZABLE : 0));
    if (!smp_load_acquire(&wq_entry.private)) {
    break;
    }
    schedule();
    }
    __set_current_state(TASK_RUNNING);
    }
    bool __sched __percpu_down_read(percpu_rw_semaphore *sem, bool try,
    bool freeze)
    {
    if (__percpu_down_read_trylock(sem)) {
    return true;
    }
    if (try) {
    return false;
    }
    trace_contention_begin(sem, LCB_F_PERCPU | LCB_F_READ);
    preempt_enable();
    percpu_rwsem_wait(sem, /* .reader = */ true, freeze);
    preempt_disable();
    trace_contention_end(sem, 0);
    return true;
    }
    EXPORT_SYMBOL_GPL(__percpu_down_read);

    ({									
    TYPEOF_UNQUAL(var) __sum = 0;					
    let mut cpu = 0;							
    compiletime_assert_atomic_type(__sum);				
    for_each_possible_cpu(cpu)					 {
    __sum += per_cpu(var, cpu);				
    }
    __sum;								
    })
#[no_mangle]
pub unsafe extern "C" fn percpu_is_read_locked(sem: *mut percpu_rw_semaphore) -> bool {
    return per_cpu_sum(*sem.read_count) != 0 && !atomic_read(&sem.block);
    }
    EXPORT_SYMBOL_GPL(percpu_is_read_locked);
//
// Return true if the modular sum of the sem->read_count per-CPU variable is
// zero.  If this sum is zero, then it is stable due to the fact that if any
// newly arriving readers increment a given counter, they will immediately
// decrement that same counter.
//
// Assumes sem->block is set.
//
#[no_mangle]
unsafe extern "C" fn readers_active_check(sem: *mut percpu_rw_semaphore) -> bool {
    if (data_race(per_cpu_sum(*sem.read_count)) != 0) {
    return false;
    }
//
// If we observed the decrement; ensure we see the entire critical
// section.
//
    smp_mb(); /* C matches B */
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn percpu_down_write(sem: *mut percpu_rw_semaphore) -> void __sched {
pub static mut contended: bool = false;
    might_sleep();
    rwsem_acquire(&sem.dep_map, 0, 0, _RET_IP_);
// Notify readers to take the slow path.
    rcu_sync_enter(&sem.rss);
//
// Try set sem->block; this provides writer-writer exclusion.
// Having sem->block set makes new readers block.
//
    if (!__percpu_down_write_trylock(sem)) {
    trace_contention_begin(sem, LCB_F_PERCPU | LCB_F_WRITE);
    percpu_rwsem_wait(sem, /* .reader = */ false, false);
    contended = true;
    }
// smp_mb() implied by __percpu_down_write_trylock() on success -- D matches A
//
// If they don't see our store of sem->block, then we are guaranteed to
// see their sem->read_count increment, and therefore will wait for
// them.
//
// Wait for all active readers to complete.
    rcuwait_wait_event(&sem.writer, readers_active_check(sem), TASK_UNINTERRUPTIBLE);
    if (contended) {
    trace_contention_end(sem, 0);
    }
    }
    EXPORT_SYMBOL_GPL(percpu_down_write);
#[no_mangle]
pub unsafe extern "C" fn percpu_up_write(sem: *mut percpu_rw_semaphore) {
    rwsem_release(&sem.dep_map, _RET_IP_);
    if (trace_contended_release_enabled() && wq_has_sleeper(&sem.waiters)) {
    trace_call__contended_release(sem);
    }
//
// Signal the writer is done, no fast path yet.
//
// One reason that we cannot just immediately flip to readers_fast is
// that new readers might fail to see the results of this writer's
// critical section.
//
// Therefore we force it through the slow path which guarantees an
// acquire and thereby guarantees the critical section's consistency.
//
    atomic_set_release(&sem.block, 0);
//
// Prod any pending reader/writer to make progress.
//
    __wake_up(&sem.waiters, TASK_NORMAL, 1, sem);
//
// Once this completes (at least one RCU-sched grace period hence) the
// reader fast path will be available again. Safe to use outside the
// exclusive write lock because its counting.
//
    rcu_sync_exit(&sem.rss);
    }
    EXPORT_SYMBOL_GPL(percpu_up_write);
#[no_mangle]
pub unsafe extern "C" fn __percpu_up_read(sem: *mut percpu_rw_semaphore) {
    lockdep_assert_preemption_disabled();
//
// After percpu_up_write() completes, rcu_sync_is_idle() can still
// return false during the grace period, forcing readers into this
// slowpath. Only trace when a writer is actually waiting for
// readers to drain.
//
    if (trace_contended_release_enabled() && rcuwait_active(&sem.writer)) {
    trace_call__contended_release(sem);
    }
//
// slowpath; reader will only ever wake a single blocked
// writer.
//
    smp_mb(); /* B matches C */
//
// In other words, if they see our decrement (presumably to
// aggregate zero, as that is the only time it matters) they
// will also see our critical section.
//
    this_cpu_dec(*sem.read_count);
    rcuwait_wake_up(&sem.writer);
    }
    EXPORT_SYMBOL_GPL(__percpu_up_read);