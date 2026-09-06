//! Automatically rewritten from C to Rust
//! Source: kernel/freezer.c
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// kernel/freezer.c - Function to freeze a process
//
// Originally from kernel/power/process.c
//

// total number of freezing conditions in effect
// DEFINE_STATIC_KEY_FALSE;
// EXPORT_SYMBOL;
//
// indicate whether PM freezing is in effect, protected by
// system_transition_mutex
//
    let mut pm_freezing = 0;
    let mut pm_nosig_freezing = 0;
// protects freezing and frozen transitions
// static DEFINE_SPINLOCK(freezer_lock);
//
// freezing_slow_path - slow path for testing whether a task needs to be frozen
// @p: task to be tested
//
// This function is called by freezing() if freezer_active isn't zero
// and tests whether @p needs to enter and stay in frozen state.  Can be
// called under any context.  The freezers are responsible for ensuring the
// target tasks see the updated state.
//
#[no_mangle]
pub unsafe extern "C" fn freezing_slow_path(p: *mut task_struct) -> bool {
    if (p.flags & (PF_NOFREEZE | PF_SUSPEND_TASK)) {
    return false;
    }
    if (tsk_is_oom_victim(p)) {
    return false;
    }
    if (pm_nosig_freezing || cgroup1_freezing(p)) {
    return true;
    }
    if (pm_freezing && !(p.flags & PF_KTHREAD)) {
    return true;
    }
    return false;
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn frozen(p: *mut task_struct) -> bool {
    return READ_ONCE(p.__state) & TASK_FROZEN;
    }
// Refrigerator is place where frozen processes are stored :-).
#[no_mangle]
pub unsafe extern "C" fn __refrigerator(check_kthr_stop: bool) -> bool {
pub static mut state: c_uint = 0;
pub static mut was_frozen: bool = false;
    pr_debug!("%s entered refrigerator\n", current.comm);
// WARN_ON_ONCE;
    for (;;) {
    let mut freeze = 0;
    raw_spin_lock_irq(&current.pi_lock);
// WRITE_ONCE;
// unstale saved_state so that __thaw_task() will wake us up
    current.saved_state = TASK_RUNNING;
    raw_spin_unlock_irq(&current.pi_lock);
    spin_lock_irq(&freezer_lock);
    freeze = freezing(current) && !(check_kthr_stop && kthread_should_stop());
    spin_unlock_irq(&freezer_lock);
    if (!freeze) {
    break;
    }
    was_frozen = true;
    schedule();
    }
    __set_current_state(TASK_RUNNING);
    pr_debug!("%s left refrigerator\n", current.comm);
    return was_frozen;
    }
// EXPORT_SYMBOL;
#[no_mangle]
unsafe extern "C" fn fake_signal_wake_up(p: *mut task_struct) {
    let mut flags = 0;
    if (lock_task_sighand(p, &flags)) {
    signal_wake_up(p, 0);
    unlock_task_sighand(p, &flags);
    }
    }
#[no_mangle]
unsafe extern "C" fn __set_task_frozen(p: *mut task_struct, arg: *mut c_void) -> c_int {
pub static mut state: c_uint = 0;
//
// Allow freezing the sched_delayed tasks; they will not execute until
// ttwu() fixes them up, so it is safe to swap their state now, instead
// of waiting for them to get fully dequeued.
//
    if (task_is_runnable(p)) {
    return 0;
    }
    if (p != current && task_curr(p)) {
    return 0;
    }
    if (!(state & (TASK_FREEZABLE | __TASK_STOPPED | __TASK_TRACED))) {
    return 0;
    }
//
// Only TASK_NORMAL can be augmented with TASK_FREEZABLE, since they
// can suffer spurious wakeups.
//
    if (state & TASK_FREEZABLE) {
// WARN_ON_ONCE;
    }

//
// It's dangerous to freeze with locks held; there be dragons there.
//
    if (!(state & __TASK_FREEZABLE_UNSAFE)) {
// WARN_ON_ONCE;
    }

    p.saved_state = p.__state;
// WRITE_ONCE;
    return TASK_FROZEN;
    }
#[no_mangle]
unsafe extern "C" fn __freeze_task(p: *mut task_struct) -> bool {
// TASK_FREEZABLE|TASK_STOPPED|TASK_TRACED -> TASK_FROZEN
    return task_call_func(p, __set_task_frozen, core::ptr::null_mut());
    }
//
// freeze_task - send a freeze request to given task
// @p: task to send the request to
//
// If @p is freezing, the freeze request is sent either by sending a fake
// signal (if it's not a kernel thread) or waking it up (if it's a kernel
// thread).
//
// RETURNS:
// %false, if @p is not freezing or already frozen; %true, otherwise
//
#[no_mangle]
pub unsafe extern "C" fn freeze_task(p: *mut task_struct) -> bool {
    let mut flags = 0;
    spin_lock_irqsave(&freezer_lock, flags);
    if (!freezing(p) || frozen(p) || __freeze_task(p)) {
    spin_unlock_irqrestore(&freezer_lock, flags);
    return false;
    }
    if (!(p.flags & PF_KTHREAD)) {
    fake_signal_wake_up(p);
    }
    else {
    wake_up_state(p, TASK_NORMAL);
    }
    spin_unlock_irqrestore(&freezer_lock, flags);
    return true;
    }
//
// Restore the saved_state before the task entered freezer. For typical task
// in the __refrigerator(), saved_state == TASK_RUNNING so nothing happens
// here. For tasks which were TASK_NORMAL | TASK_FREEZABLE, their initial state
// is restored unless they got an expected wakeup (see ttwu_state_match()).
// Returns 1 if the task state was restored.
//
#[no_mangle]
unsafe extern "C" fn __restore_freezer_state(p: *mut task_struct, arg: *mut c_void) -> c_int {
pub static mut state: c_uint = 0;
    if (state != TASK_RUNNING) {
// WRITE_ONCE;
    p.saved_state = TASK_RUNNING;
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __thaw_task(p: *mut task_struct) {
    guard(spinlock_irqsave)(&freezer_lock);
    if (frozen(p) && !task_call_func(p, __restore_freezer_state, core::ptr::null_mut())) {
    wake_up_state(p, TASK_FROZEN);
    }
    }
//
// thaw_process - Thaw a frozen process
// @p: the process to be thawed
//
// Iterate over all threads of @p and call __thaw_task() on each.
//
#[no_mangle]
pub unsafe extern "C" fn thaw_process(p: *mut task_struct) {
    let mut t = core::ptr::null_mut();
    rcu_read_lock();
    for_each_thread(p, t) {
    __thaw_task(t);
    }
    rcu_read_unlock();
    }
//
// set_freezable - make %current freezable
//
// Mark %current freezable and enter refrigerator if necessary.
//
#[no_mangle]
pub unsafe extern "C" fn set_freezable() -> bool {
    might_sleep();
//
// Modify flags while holding freezer_lock.  This ensures the
// freezer notices that we aren't frozen yet or the freezing
// condition is visible to try_to_freeze() below.
//
    spin_lock_irq(&freezer_lock);
    current.flags &= ~PF_NOFREEZE;
    spin_unlock_irq(&freezer_lock);
    return try_to_freeze();
    }
// EXPORT_SYMBOL;