//! Automatically rewritten from C to Rust
//! Source: kernel/sched/wait.c
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
// Generic waiting primitives.
//
// (C) 2004 Nadia Yvette Chambers, Oracle
//

#[no_mangle]
pub unsafe extern "C" fn __init_waitqueue_head(wq_head: *mut wait_queue_head, name: *const c_char, key: *mut lock_class_key) {
    spin_lock_init(&wq_head.lock);
    lockdep_set_class_and_name(&wq_head.lock, key, name);
    INIT_LIST_HEAD(&wq_head.head);
    }
    EXPORT_SYMBOL(__init_waitqueue_head);
#[no_mangle]
pub unsafe extern "C" fn add_wait_queue(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    let mut flags = 0;
    wq_entry.flags &= ~WQ_FLAG_EXCLUSIVE;
    spin_lock_irqsave(&wq_head.lock, flags);
    __add_wait_queue(wq_head, wq_entry);
    spin_unlock_irqrestore(&wq_head.lock, flags);
    }
    EXPORT_SYMBOL(add_wait_queue);
#[no_mangle]
pub unsafe extern "C" fn add_wait_queue_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    let mut flags = 0;
    wq_entry.flags |= WQ_FLAG_EXCLUSIVE;
    spin_lock_irqsave(&wq_head.lock, flags);
    __add_wait_queue_entry_tail(wq_head, wq_entry);
    spin_unlock_irqrestore(&wq_head.lock, flags);
    }
    EXPORT_SYMBOL(add_wait_queue_exclusive);
#[no_mangle]
pub unsafe extern "C" fn add_wait_queue_priority(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    let mut flags = 0;
    wq_entry.flags |= WQ_FLAG_PRIORITY;
    spin_lock_irqsave(&wq_head.lock, flags);
    __add_wait_queue(wq_head, wq_entry);
    spin_unlock_irqrestore(&wq_head.lock, flags);
    }
    EXPORT_SYMBOL_GPL(add_wait_queue_priority);
#[no_mangle]
pub unsafe extern "C" fn add_wait_queue_priority_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) -> c_int {
    let mut head = &wq_head.head;
    wq_entry.flags |= WQ_FLAG_EXCLUSIVE | WQ_FLAG_PRIORITY;
    guard(spinlock_irqsave)(&wq_head.lock);
    if (!list_empty(head) &&
    (list_first_entry(head, typeof(*wq_entry), entry).flags & WQ_FLAG_PRIORITY)) {
    return -EBUSY;
    }
    list_add(&wq_entry.entry, head);
    return 0;
    }
    EXPORT_SYMBOL_GPL(add_wait_queue_priority_exclusive);
#[no_mangle]
pub unsafe extern "C" fn remove_wait_queue(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    let mut flags = 0;
    spin_lock_irqsave(&wq_head.lock, flags);
    __remove_wait_queue(wq_head, wq_entry);
    spin_unlock_irqrestore(&wq_head.lock, flags);
    }
    EXPORT_SYMBOL(remove_wait_queue);
//
// The core wakeup function. Non-exclusive wakeups (nr_exclusive == 0) just
// wake everything up. If it's an exclusive wakeup (nr_exclusive == small +ve
// number) then we wake that number of exclusive tasks, and potentially all
// the non-exclusive tasks. Normally, exclusive tasks will be at the end of
// the list and any non-exclusive tasks will be woken first. A priority task
// may be at the head of the list, and can consume the event without any other
// tasks being woken if it's also an exclusive task.
//
// There are circumstances in which we can try to wake a task which has already
// started to run but is not in state TASK_RUNNING. try_to_wake_up() returns
// zero in this (rare) case, and we handle it by continuing to scan the queue.
//
#[no_mangle]
pub unsafe extern "C" fn __wake_up_common(wq_head: *mut wait_queue_head, mode: c_uint, nr_exclusive: c_int, wake_flags: c_int, key: *mut c_void) -> c_int {
    let mut curr = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    lockdep_assert_held(&wq_head.lock);
    curr = list_first_entry(&wq_head.head, wait_queue_entry_t, entry);
    if (&curr.entry == &wq_head.head) {
    return nr_exclusive;
    }
    list_for_each_entry_safe_from(curr, next, &wq_head.head, entry) {
pub static mut flags: unsigned = 0;
    let mut ret = 0;
    ret = curr.func(curr, mode, wake_flags, key);
    if (ret < 0) {
    break;
    }
    if (ret && (flags & WQ_FLAG_EXCLUSIVE) && !--nr_exclusive) {
    break;
    }
    }
    return nr_exclusive;
    }
#[no_mangle]
pub unsafe extern "C" fn __wake_up_common_lock(wq_head: *mut wait_queue_head, mode: c_uint, nr_exclusive: c_int, wake_flags: c_int, key: *mut c_void) -> c_int {
    let mut flags = 0;
    let mut remaining = 0;
    spin_lock_irqsave(&wq_head.lock, flags);
    remaining = __wake_up_common(wq_head, mode, nr_exclusive, wake_flags,
    key);
    spin_unlock_irqrestore(&wq_head.lock, flags);
    return nr_exclusive - remaining;
    }
//
// __wake_up - wake up threads blocked on a waitqueue.
// @wq_head: the waitqueue
// @mode: which threads
// @nr_exclusive: how many wake-one or wake-many threads to wake up
// @key: is directly passed to the wakeup function
//
// If this function wakes up a task, it executes a full memory barrier
// before accessing the task state.  Returns the number of exclusive
// tasks that were awaken.
//
#[no_mangle]
pub unsafe extern "C" fn __wake_up(wq_head: *mut wait_queue_head, mode: c_uint, nr_exclusive: c_int, key: *mut c_void) -> c_int {
    return __wake_up_common_lock(wq_head, mode, nr_exclusive, 0, key);
    }
    EXPORT_SYMBOL(__wake_up);
#[no_mangle]
pub unsafe extern "C" fn __wake_up_on_current_cpu(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) {
    __wake_up_common_lock(wq_head, mode, 1, WF_CURRENT_CPU, key);
    }
//
// Same as __wake_up but called with the spinlock in wait_queue_head_t held.
//
#[no_mangle]
pub unsafe extern "C" fn __wake_up_locked(wq_head: *mut wait_queue_head, mode: c_uint, nr: c_int) {
    __wake_up_common(wq_head, mode, nr, 0, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(__wake_up_locked);
#[no_mangle]
pub unsafe extern "C" fn __wake_up_locked_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) {
    __wake_up_common(wq_head, mode, 1, 0, key);
    }
    EXPORT_SYMBOL_GPL(__wake_up_locked_key);
//
// __wake_up_sync_key - wake up threads blocked on a waitqueue.
// @wq_head: the waitqueue
// @mode: which threads
// @key: opaque value to be passed to wakeup targets
//
// The sync wakeup differs that the waker knows that it will schedule
// away soon, so while the target thread will be woken up, it will not
// be migrated to another CPU - ie. the two threads are 'synchronized'
// with each other. This can prevent needless bouncing between CPUs.
//
// On UP it can prevent extra preemption.
//
// If this function wakes up a task, it executes a full memory barrier before
// accessing the task state.
//
#[no_mangle]
pub unsafe extern "C" fn __wake_up_sync_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) {
    if (unlikely(!wq_head)) {
    return;
    }
    __wake_up_common_lock(wq_head, mode, 1, WF_SYNC, key);
    }
    EXPORT_SYMBOL_GPL(__wake_up_sync_key);
//
// __wake_up_locked_sync_key - wake up a thread blocked on a locked waitqueue.
// @wq_head: the waitqueue
// @mode: which threads
// @key: opaque value to be passed to wakeup targets
//
// The sync wakeup differs in that the waker knows that it will schedule
// away soon, so while the target thread will be woken up, it will not
// be migrated to another CPU - ie. the two threads are 'synchronized'
// with each other. This can prevent needless bouncing between CPUs.
//
// On UP it can prevent extra preemption.
//
// If this function wakes up a task, it executes a full memory barrier before
// accessing the task state.
//
#[no_mangle]
pub unsafe extern "C" fn __wake_up_locked_sync_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) {
    __wake_up_common(wq_head, mode, 1, WF_SYNC, key);
    }
    EXPORT_SYMBOL_GPL(__wake_up_locked_sync_key);
//
// __wake_up_sync - see __wake_up_sync_key()
//
#[no_mangle]
pub unsafe extern "C" fn __wake_up_sync(wq_head: *mut wait_queue_head, mode: c_uint) {
    __wake_up_sync_key(wq_head, mode, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(__wake_up_sync);	/* For internal use only */
#[no_mangle]
pub unsafe extern "C" fn __wake_up_pollfree(wq_head: *mut wait_queue_head) {
    __wake_up(wq_head, TASK_NORMAL, 0, poll_to_key(EPOLLHUP | POLLFREE));
// POLLFREE must have cleared the queue.
    WARN_ON_ONCE!(waitqueue_active(wq_head));
    }
//
// Note: we use "set_current_state()" _after_ the wait-queue add,
// because we need a memory barrier there on SMP, so that any
// wake-function that tests for the wait-queue being active
// will be guaranteed to see waitqueue addition _or_ subsequent
// tests in this thread will see the wakeup having taken place.
//
// The spin_unlock() itself is semi-permeable and only protects
// one way (it only protects stuff inside the critical region and
// stops them from bleeding out - it would still allow subsequent
// loads to move into the critical region).
//
#[no_mangle]
pub unsafe extern "C" fn prepare_to_wait(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) {
    let mut flags = 0;
    wq_entry.flags &= ~WQ_FLAG_EXCLUSIVE;
    spin_lock_irqsave(&wq_head.lock, flags);
    if (list_empty(&wq_entry.entry)) {
    __add_wait_queue(wq_head, wq_entry);
    }
    set_current_state(state);
    spin_unlock_irqrestore(&wq_head.lock, flags);
    }
    EXPORT_SYMBOL(prepare_to_wait);
// Returns true if we are the first waiter in the queue, false otherwise.
#[no_mangle]
pub unsafe extern "C" fn prepare_to_wait_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) -> bool {
    let mut flags = 0;
pub static mut was_empty: bool = false;
    wq_entry.flags |= WQ_FLAG_EXCLUSIVE;
    spin_lock_irqsave(&wq_head.lock, flags);
    if (list_empty(&wq_entry.entry)) {
    was_empty = list_empty(&wq_head.head);
    __add_wait_queue_entry_tail(wq_head, wq_entry);
    }
    set_current_state(state);
    spin_unlock_irqrestore(&wq_head.lock, flags);
    return was_empty;
    }
    EXPORT_SYMBOL(prepare_to_wait_exclusive);
#[no_mangle]
pub unsafe extern "C" fn init_wait_entry(wq_entry: *mut wait_queue_entry, flags: c_int) {
    wq_entry.flags = flags;
    wq_entry.private = current;
    wq_entry.func = autoremove_wake_function;
    INIT_LIST_HEAD(&wq_entry.entry);
    }
    EXPORT_SYMBOL(init_wait_entry);
#[no_mangle]
pub unsafe extern "C" fn prepare_to_wait_event(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) -> c_long {
    let mut flags = 0;
pub static mut ret: c_long = 0;
    spin_lock_irqsave(&wq_head.lock, flags);
    if (signal_pending_state(state, current)) {
//
// Exclusive waiter must not fail if it was selected by wakeup,
// it should "consume" the condition we were waiting for.
//
// The caller will recheck the condition and return success if
// we were already woken up, we can not miss the event because
// wakeup locks/unlocks the same wq_head->lock.
//
// But we need to ensure that set-condition + wakeup after that
// can't see us, it should wake up another exclusive waiter if
// we fail.
//
    list_del_init(&wq_entry.entry);
    ret = -ERESTARTSYS;
    } else {
    if (list_empty(&wq_entry.entry)) {
    if (wq_entry.flags & WQ_FLAG_EXCLUSIVE) {
    __add_wait_queue_entry_tail(wq_head, wq_entry);
    }
    else {
    __add_wait_queue(wq_head, wq_entry);
    }
    }
    set_current_state(state);
    }
    spin_unlock_irqrestore(&wq_head.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL(prepare_to_wait_event);
//
// Note! These two wait functions are entered with the
// wait-queue lock held (and interrupts off in the _irq
// case), so there is no race with testing the wakeup
// condition in the caller before they add the wait
// entry to the wake queue.
//
#[no_mangle]
pub unsafe extern "C" fn do_wait_intr(wq: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t) -> c_int {
    if (likely(list_empty(&wait.entry))) {
    __add_wait_queue_entry_tail(wq, wait);
    }
    set_current_state(TASK_INTERRUPTIBLE);
    if (signal_pending(current)) {
    return -ERESTARTSYS;
    }
    spin_unlock(&wq.lock);
    schedule();
    spin_lock(&wq.lock);
    return 0;
    }
    EXPORT_SYMBOL(do_wait_intr);
#[no_mangle]
pub unsafe extern "C" fn do_wait_intr_irq(wq: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t) -> c_int {
    if (likely(list_empty(&wait.entry))) {
    __add_wait_queue_entry_tail(wq, wait);
    }
    set_current_state(TASK_INTERRUPTIBLE);
    if (signal_pending(current)) {
    return -ERESTARTSYS;
    }
    spin_unlock_irq(&wq.lock);
    schedule();
    spin_lock_irq(&wq.lock);
    return 0;
    }
    EXPORT_SYMBOL(do_wait_intr_irq);
//
// finish_wait - clean up after waiting in a queue
// @wq_head: waitqueue waited on
// @wq_entry: wait descriptor
//
// Sets current thread back to running state and removes
// the wait descriptor from the given waitqueue if still
// queued.
//
#[no_mangle]
pub unsafe extern "C" fn finish_wait(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    let mut flags = 0;
    __set_current_state(TASK_RUNNING);
//
// We can check for list emptiness outside the lock
// IFF:
// - we use the "careful" check that verifies both
// the next and prev pointers, so that there cannot
// be any half-pending updates in progress on other
// CPU's that we haven't seen yet (and that might
// still change the stack area.
// and
// - all other users take the lock (ie we can only
// have _one_ other CPU that looks at or modifies
// the list).
//
    if (!list_empty_careful(&wq_entry.entry)) {
    spin_lock_irqsave(&wq_head.lock, flags);
    list_del_init(&wq_entry.entry);
    spin_unlock_irqrestore(&wq_head.lock, flags);
    }
    }
    EXPORT_SYMBOL(finish_wait);
#[no_mangle]
pub unsafe extern "C" fn autoremove_wake_function(wq_entry: *mut wait_queue_entry, mode: unsigned, sync: c_int, key: *mut c_void) -> c_int {
pub static mut ret: c_int = 0;
    if (ret) {
    list_del_init_careful(&wq_entry.entry);
    }
    return ret;
    }
    EXPORT_SYMBOL(autoremove_wake_function);
//
// DEFINE_WAIT_FUNC(wait, woken_wake_func);
//
// add_wait_queue(&wq_head, &wait);
// for (;;) {
// if (condition)
// break;
//
// // in wait_woken()			// in woken_wake_function()
//
// p->state = mode;				wq_entry->flags |= WQ_FLAG_WOKEN;
// smp_mb(); // A				try_to_wake_up():
// if (!(wq_entry->flags & WQ_FLAG_WOKEN))	   <full barrier>
// schedule()				   if (p->state & mode)
// p->state = TASK_RUNNING;			      p->state = TASK_RUNNING;
// wq_entry->flags &= ~WQ_FLAG_WOKEN;	~~~~~~~~~~~~~~~~~~
// smp_mb(); // B				condition = true;
// }						smp_mb(); // C
// remove_wait_queue(&wq_head, &wait);		wq_entry->flags |= WQ_FLAG_WOKEN;
//
#[no_mangle]
pub unsafe extern "C" fn wait_woken(wq_entry: *mut wait_queue_entry, mode: unsigned, timeout: c_long) -> c_long {
//
// The below executes an smp_mb(), which matches with the full barrier
// executed by the try_to_wake_up() in woken_wake_function() such that
// either we see the store to wq_entry->flags in woken_wake_function()
// or woken_wake_function() sees our store to current->state.
//
    set_current_state(mode); /* A */
    if (!(wq_entry.flags & WQ_FLAG_WOKEN) && !kthread_should_stop_or_park()) {
    timeout = schedule_timeout(timeout);
    }
    __set_current_state(TASK_RUNNING);
//
// The below executes an smp_mb(), which matches with the smp_mb() (C)
// in woken_wake_function() such that either we see the wait condition
// being true or the store to wq_entry->flags in woken_wake_function()
// follows ours in the coherence order.
//
    smp_store_mb(wq_entry.flags, wq_entry.flags & ~WQ_FLAG_WOKEN); /* B */
    return timeout;
    }
    EXPORT_SYMBOL(wait_woken);
#[no_mangle]
pub unsafe extern "C" fn woken_wake_function(wq_entry: *mut wait_queue_entry, mode: unsigned, sync: c_int, key: *mut c_void) -> c_int {
// Pairs with the smp_store_mb() in wait_woken().
    smp_mb(); /* C */
    wq_entry.flags |= WQ_FLAG_WOKEN;
    return default_wake_function(wq_entry, mode, sync, key);
    }
    EXPORT_SYMBOL(woken_wake_function);
#[no_mangle]
pub unsafe extern "C" fn woken_wake_bit_function(wq_entry: *mut wait_queue_entry, mode: unsigned, sync: c_int, arg: *mut c_void) -> c_int {
    let mut key = __var_wake_key(wq_entry, arg);
    if (!key) {
    return 0;
    }
// Pairs with the smp_store_mb() in wait_woken().
    smp_mb(); /* C */
    wq_entry.flags |= WQ_FLAG_WOKEN;
    return default_wake_function(wq_entry, mode, sync, key);
    }
    EXPORT_SYMBOL(woken_wake_bit_function);