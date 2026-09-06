//! Automatically rewritten from C to Rust
//! Source: kernel/task_work.c
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



// SPDX-License-Identifier: GPL-2.0

pub static mut work_exited: usize = 0; /* all we need is .next == core::ptr::null_mut() */

#[no_mangle]
unsafe extern "C" fn task_work_set_notify_irq(entry: *mut irq_work) {
//
// no-op IPI
//
// TWA_NMI_CURRENT will already have set the TIF flag, all
// this interrupt does it tickle the return-to-user path.
//
    }
    static DEFINE_PER_CPU(irq_work, irq_work_NMI_resume) =
    IRQ_WORK_INIT_HARD(task_work_set_notify_irq);

//
// task_work_add - ask the @task to execute @work->func()
// @task: the task which should run the callback
// @work: the callback to run
// @notify: how to notify the targeted task
//
// Queue @work for task_work_run() below and notify the @task if @notify
// is @TWA_RESUME, @TWA_SIGNAL, @TWA_SIGNAL_NO_IPI or @TWA_NMI_CURRENT.
//
// @TWA_SIGNAL works like signals, in that the it will interrupt the targeted
// task and run the task_work, regardless of whether the task is currently
// running in the kernel or userspace.
// @TWA_SIGNAL_NO_IPI works like @TWA_SIGNAL, except it doesn't send a
// reschedule IPI to force the targeted task to reschedule and run task_work.
// This can be advantageous if there's no strict requirement that the
// task_work be run as soon as possible, just whenever the task enters the
// kernel anyway.
// @TWA_RESUME work is run only when the task exits the kernel and returns to
// user mode, or before entering guest mode.
// @TWA_NMI_CURRENT works like @TWA_RESUME, except it can only be used for the
// current @task and if the current context is NMI.
//
// Fails if the @task is exiting/exited and thus it can't process this @work.
// Otherwise @work->func() will be called when the @task goes through one of
// the aforementioned transitions, or exits.
//
// If the targeted task is exiting, then an error is returned and the work item
// is not queued. It's up to the caller to arrange for an alternative mechanism
// in that case.
//
// Note: there is no ordering guarantee on works queued here. The task_work
// list is LIFO.
//
// RETURNS:
// 0 if succeeds or -ESRCH.
//
#[no_mangle]
pub unsafe extern "C" fn task_work_add(task: *mut task_struct, work: *mut callback_head, notify: task_work_notify_mode) -> c_int {
pub static mut head: *mut c_void = core::ptr::null_mut();
    if (notify == TWA_NMI_CURRENT) {
    if (WARN_ON_ONCE!(task != current)) {
    return -EINVAL;
    }
    if (!IS_ENABLED!(CONFIG_IRQ_WORK)) {
    return -EINVAL;
    }
    } else {
    kasan_record_aux_stack(work);
    }
    head = READ_ONCE(task.task_works);
    do {
    if (unlikely(head == &work_exited)) {
    return -ESRCH;
    }
    work.next = head;
    } while (!try_cmpxchg(&task.task_works, &head, work));
    match (notify) {
    TWA_NONE => {
    // break;
    }
    TWA_RESUME => {
    set_notify_resume(task);
    // break;
    }
    TWA_SIGNAL => {
    set_notify_signal(task);
    // break;
    }
    TWA_SIGNAL_NO_IPI => {
    __set_notify_signal(task);
    // break;

    }
    TWA_NMI_CURRENT => {
    set_tsk_thread_flag(current, TIF_NOTIFY_RESUME);
    irq_work_queue(this_cpu_ptr(&irq_work_NMI_resume));
    // break;

    }
    _ => {
    WARN_ON_ONCE!(1);
    // break;
    }
    }
    return 0;
    }
//
// task_work_cancel_match - cancel a pending work added by task_work_add()
// @task: the task which should execute the work
// @match: match function to call
// @data: data to be passed in to match function
//
// RETURNS:
// The found work or NULL if not found.
//
#[no_mangle]
pub unsafe extern "C" fn task_work_cancel_match(task: *mut task_struct, data: *mut c_void) -> *mut c_void {
    let mut pprev = &task.task_works;
pub static mut work: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    if (likely(!task_work_pending(task))) {
    return core::ptr::null_mut();
    }
//
// If cmpxchg() fails we continue without updating pprev.
// Either we raced with task_work_add() which added the
// new entry before this work, we will find it again. Or
// we raced with task_work_run(), *pprev == NULL/exited.
//
    raw_spin_lock_irqsave(&task.pi_lock, flags);
    work = READ_ONCE(*pprev);
    while (work) {
    if (!match(work, data)) {
    pprev = &work.next;
    work = READ_ONCE(*pprev);
    } else if (try_cmpxchg(pprev, &work, work.next)) {
    break;
    }
    }
    raw_spin_unlock_irqrestore(&task.pi_lock, flags);
    return work;
    }
#[no_mangle]
unsafe extern "C" fn task_work_func_match(cb: *mut callback_head, data: *mut c_void) -> bool {
    return cb.func == data;
    }
//
// task_work_cancel_func - cancel a pending work matching a function added by task_work_add()
// @task: the task which should execute the func's work
// @func: identifies the func to match with a work to remove
//
// Find the last queued pending work with ->func == @func and remove
// it from queue.
//
// RETURNS:
// The found work or NULL if not found.
//
#[no_mangle]
pub unsafe extern "C" fn task_work_cancel_func(task: *mut task_struct, func: task_work_func_t) -> *mut c_void {
    return task_work_cancel_match(task, task_work_func_match, func);
    }
#[no_mangle]
unsafe extern "C" fn task_work_match(cb: *mut callback_head, data: *mut c_void) -> bool {
pub static mut cb: return = 0;
    }
//
// task_work_cancel - cancel a pending work added by task_work_add()
// @task: the task which should execute the work
// @cb: the callback to remove if queued
//
// Remove a callback from a task's queue if queued.
//
// RETURNS:
// True if the callback was queued and got cancelled, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn task_work_cancel(task: *mut task_struct, cb: *mut callback_head) -> bool {
pub static mut ret: *mut c_void = core::ptr::null_mut();
    ret = task_work_cancel_match(task, task_work_match, cb);
pub static mut ret: return = 0;
    }
//
// task_work_run - execute the works added by task_work_add()
//
// Flush the pending works. Should be used by the core kernel code.
// Called before the task returns to the user-mode or stops, or when
// it exits. In the latter case task_work_add() can no longer add the
// new work after task_work_run() returns.
//
#[no_mangle]
pub unsafe extern "C" fn task_work_run() {
    let mut task = current;
    let mut work = core::ptr::null_mut();
    let mut head = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    for (;;) {
//
// work->func() can do task_work_add(), do not set
// work_exited unless the list is empty.
//
    work = READ_ONCE(task.task_works);
    do {
    head = core::ptr::null_mut();
    if (!work) {
    if (task.flags & PF_EXITING) {
    head = &work_exited;
    }
    else {
    break;
    }
    }
    } while (!try_cmpxchg(&task.task_works, &work, head));
    if (!work) {
    break;
    }
//
// Synchronize with task_work_cancel_match(). It can not remove
// the first entry == work, cmpxchg(task_works) must fail.
// But it can remove another entry from the ->next list.
//
    raw_spin_lock_irq(&task.pi_lock);
    raw_spin_unlock_irq(&task.pi_lock);
    do {
    next = work.next;
    work.func(work);
    work = next;
    cond_resched();
    } while (work);
    }
    }