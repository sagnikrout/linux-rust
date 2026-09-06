//! Automatically rewritten from C to Rust
//! Source: kernel/sched/completion.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Generic wait-for-completion handler;
//
// It differs from semaphores in that their default case is the opposite,
// wait_for_completion default blocks whereas semaphore default non-block. The
// interface also makes it easy to 'complete' multiple waiting threads,
// something which isn't entirely natural for semaphores.
//
// But more importantly, the primitive documents the usage. Semaphores would
// typically be used for exclusion which gives rise to priority inversion.
// Waiting for completion is a typically sync point, but not an exclusion point.
//

#[no_mangle]
unsafe extern "C" fn complete_with_flags(x: *mut completion, wake_flags: c_int) {
    let mut flags = 0;
    raw_spin_lock_irqsave(&x.wait.lock, flags);
    if (x.done != UINT_MAX) {
    x.done += 1;
    }
    swake_up_locked(&x.wait, wake_flags);
    raw_spin_unlock_irqrestore(&x.wait.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn complete_on_current_cpu(x: *mut completion) {
    return complete_with_flags(x, WF_CURRENT_CPU);
    }
//
// complete: - signals a single thread waiting on this completion
// @x:  holds the state of this particular completion
//
// This will wake up a single thread waiting on this completion. Threads will be
// awakened in the same order in which they were queued.
//
// See also complete_all(), wait_for_completion() and related routines.
//
// If this function wakes up a task, it executes a full memory barrier before
// accessing the task state.
//
#[no_mangle]
pub unsafe extern "C" fn complete(x: *mut completion) {
    complete_with_flags(x, 0);
    }
    EXPORT_SYMBOL(complete);
//
// complete_all: - signals all threads waiting on this completion
// @x:  holds the state of this particular completion
//
// This will wake up all threads waiting on this particular completion event.
//
// If this function wakes up a task, it executes a full memory barrier before
// accessing the task state.
//
// Since complete_all() sets the completion of @x permanently to done
// to allow multiple waiters to finish, a call to reinit_completion()
// must be used on @x if @x is to be used again. The code must make
// sure that all waiters have woken and finished before reinitializing
// @x. Also note that the function completion_done() can not be used
// to know if there are still waiters after complete_all() has been called.
//
#[no_mangle]
pub unsafe extern "C" fn complete_all(x: *mut completion) {
    let mut flags = 0;
    lockdep_assert_RT_in_threaded_ctx();
    raw_spin_lock_irqsave(&x.wait.lock, flags);
    x.done = UINT_MAX;
    swake_up_all_locked(&x.wait);
    raw_spin_unlock_irqrestore(&x.wait.lock, flags);
    }
    EXPORT_SYMBOL(complete_all);
    static inline long __sched
    do_wait_for_common(completion *x,
    long (*action)(long), long timeout, int state)
    {
    if (!x.done) {
pub static mut wait: usize = 0;
    do {
    if (signal_pending_state(state, current)) {
    timeout = -ERESTARTSYS;
    break;
    }
    __prepare_to_swait(&x.wait, &wait);
    __set_current_state(state);
    raw_spin_unlock_irq(&x.wait.lock);
    timeout = action(timeout);
    raw_spin_lock_irq(&x.wait.lock);
    } while (!x.done && timeout);
    __finish_swait(&x.wait, &wait);
    if (!x.done) {
    return timeout;
    }
    }
    if (x.done != UINT_MAX) {
    x.done -= 1;
    }
    return timeout ?: 1;
    }
    static inline long __sched
    __wait_for_common(completion *x,
    long (*action)(long), long timeout, int state)
    {
    might_sleep();
    complete_acquire(x);
    raw_spin_lock_irq(&x.wait.lock);
    timeout = do_wait_for_common(x, action, timeout, state);
    raw_spin_unlock_irq(&x.wait.lock);
    complete_release(x);
    return timeout;
    }
    static long __sched
    wait_for_common(completion *x, long timeout, int state)
    {
    return __wait_for_common(x, schedule_timeout, timeout, state);
    }
    static long __sched
    wait_for_common_io(completion *x, long timeout, int state)
    {
    return __wait_for_common(x, io_schedule_timeout, timeout, state);
    }
//
// wait_for_completion: - waits for completion of a task
// @x:  holds the state of this particular completion
//
// This waits to be signaled for completion of a specific task. It is NOT
// interruptible and there is no timeout.
//
// See also similar routines (i.e. wait_for_completion_timeout()) with timeout
// and interrupt capability. Also see complete().
//
#[no_mangle]
pub unsafe extern "C" fn wait_for_completion(x: *mut completion) -> void __sched {
    wait_for_common(x, MAX_SCHEDULE_TIMEOUT, TASK_UNINTERRUPTIBLE);
    }
    EXPORT_SYMBOL(wait_for_completion);
//
// wait_for_completion_timeout: - waits for completion of a task (w/timeout)
// @x:  holds the state of this particular completion
// @timeout:  timeout value in jiffies
//
// This waits for either a completion of a specific task to be signaled or for a
// specified timeout to expire. The timeout is in jiffies. It is not
// interruptible.
//
// Return: 0 if timed out, and positive (at least 1, or number of jiffies left
// till timeout) if completed.
//
    unsigned long __sched
    wait_for_completion_timeout(completion *x, unsigned long timeout)
    {
    return wait_for_common(x, timeout, TASK_UNINTERRUPTIBLE);
    }
    EXPORT_SYMBOL(wait_for_completion_timeout);
//
// wait_for_completion_io: - waits for completion of a task
// @x:  holds the state of this particular completion
//
// This waits to be signaled for completion of a specific task. It is NOT
// interruptible and there is no timeout. The caller is accounted as waiting
// for IO (which traditionally means blkio only).
//
#[no_mangle]
pub unsafe extern "C" fn wait_for_completion_io(x: *mut completion) -> void __sched {
    wait_for_common_io(x, MAX_SCHEDULE_TIMEOUT, TASK_UNINTERRUPTIBLE);
    }
    EXPORT_SYMBOL(wait_for_completion_io);
//
// wait_for_completion_io_timeout: - waits for completion of a task (w/timeout)
// @x:  holds the state of this particular completion
// @timeout:  timeout value in jiffies
//
// This waits for either a completion of a specific task to be signaled or for a
// specified timeout to expire. The timeout is in jiffies. It is not
// interruptible. The caller is accounted as waiting for IO (which traditionally
// means blkio only).
//
// Return: 0 if timed out, and positive (at least 1, or number of jiffies left
// till timeout) if completed.
//
    unsigned long __sched
    wait_for_completion_io_timeout(completion *x, unsigned long timeout)
    {
    return wait_for_common_io(x, timeout, TASK_UNINTERRUPTIBLE);
    }
    EXPORT_SYMBOL(wait_for_completion_io_timeout);
//
// wait_for_completion_interruptible: - waits for completion of a task (w/intr)
// @x:  holds the state of this particular completion
//
// This waits for completion of a specific task to be signaled. It is
// interruptible.
//
// Return: -ERESTARTSYS if interrupted, 0 if completed.
//
#[no_mangle]
pub unsafe extern "C" fn wait_for_completion_interruptible(x: *mut completion) -> int __sched {
pub static mut t: c_long = 0;
    if (t == -ERESTARTSYS) {
    return t;
    }
    return 0;
    }
    EXPORT_SYMBOL(wait_for_completion_interruptible);
//
// wait_for_completion_interruptible_timeout: - waits for completion (w/(to,intr))
// @x:  holds the state of this particular completion
// @timeout:  timeout value in jiffies
//
// This waits for either a completion of a specific task to be signaled or for a
// specified timeout to expire. It is interruptible. The timeout is in jiffies.
//
// Return: -ERESTARTSYS if interrupted, 0 if timed out, positive (at least 1,
// or number of jiffies left till timeout) if completed.
//
    long __sched
    wait_for_completion_interruptible_timeout(completion *x,
    unsigned long timeout)
    {
    return wait_for_common(x, timeout, TASK_INTERRUPTIBLE);
    }
    EXPORT_SYMBOL(wait_for_completion_interruptible_timeout);
//
// wait_for_completion_killable: - waits for completion of a task (killable)
// @x:  holds the state of this particular completion
//
// This waits to be signaled for completion of a specific task. It can be
// interrupted by a kill signal.
//
// Return: -ERESTARTSYS if interrupted, 0 if completed.
//
#[no_mangle]
pub unsafe extern "C" fn wait_for_completion_killable(x: *mut completion) -> int __sched {
pub static mut t: c_long = 0;
    if (t == -ERESTARTSYS) {
    return t;
    }
    return 0;
    }
    EXPORT_SYMBOL(wait_for_completion_killable);
#[no_mangle]
pub unsafe extern "C" fn wait_for_completion_state(x: *mut completion, state: c_uint) -> int __sched {
pub static mut t: c_long = 0;
    if (t == -ERESTARTSYS) {
    return t;
    }
    return 0;
    }
    EXPORT_SYMBOL(wait_for_completion_state);
//
// wait_for_completion_killable_timeout: - waits for completion of a task (w/(to,killable))
// @x:  holds the state of this particular completion
// @timeout:  timeout value in jiffies
//
// This waits for either a completion of a specific task to be
// signaled or for a specified timeout to expire. It can be
// interrupted by a kill signal. The timeout is in jiffies.
//
// Return: -ERESTARTSYS if interrupted, 0 if timed out, positive (at least 1,
// or number of jiffies left till timeout) if completed.
//
    long __sched
    wait_for_completion_killable_timeout(completion *x,
    unsigned long timeout)
    {
    return wait_for_common(x, timeout, TASK_KILLABLE);
    }
    EXPORT_SYMBOL(wait_for_completion_killable_timeout);
//
// try_wait_for_completion - try to decrement a completion without blocking
// @x:	completion structure
//
// Return: 0 if a decrement cannot be done without blocking
// 1 if a decrement succeeded.
//
// If a completion is being used as a counting completion,
// attempt to decrement the counter without blocking. This
// enables us to avoid waiting if the resource the completion
// is protecting is not available.
//
#[no_mangle]
pub unsafe extern "C" fn try_wait_for_completion(x: *mut completion) -> bool {
    let mut flags = 0;
pub static mut ret: bool = true;
//
// Since x->done will need to be locked only
// in the non-blocking case, we check x->done
// first without taking the lock so we can
// return early in the blocking case.
//
    if (!READ_ONCE(x.done)) {
    return false;
    }
    raw_spin_lock_irqsave(&x.wait.lock, flags);
    if (!x.done) {
    ret = false;
    }

    else if (x.done != UINT_MAX) {
    x.done -= 1;
    }
    raw_spin_unlock_irqrestore(&x.wait.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL(try_wait_for_completion);
//
// completion_done - Test to see if a completion has any waiters
// @x:	completion structure
//
// Return: 0 if there are waiters (wait_for_completion() in progress)
// 1 if there are no waiters.
//
// Note, this will always return true if complete_all() was called on @X.
//
#[no_mangle]
pub unsafe extern "C" fn completion_done(x: *mut completion) -> bool {
    let mut flags = 0;
    if (!READ_ONCE(x.done)) {
    return false;
    }
//
// If ->done, we need to wait for complete() to release ->wait.lock
// otherwise we can end up freeing the completion before complete()
// is done referencing it.
//
    raw_spin_lock_irqsave(&x.wait.lock, flags);
    raw_spin_unlock_irqrestore(&x.wait.lock, flags);
    return true;
    }
    EXPORT_SYMBOL(completion_done);