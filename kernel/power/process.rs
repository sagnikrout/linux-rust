//! Automatically rewritten from C to Rust
//! Source: kernel/power/process.c
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
// drivers/power/process.c - Functions for starting/stopping processes on
// suspend transitions.
//
// Originally from swsusp.
//

//
// Timeout for stopping processes
//
pub static mut freeze_timeout_msecs: unsigned int  = 0;
#[no_mangle]
unsafe extern "C" fn try_to_freeze_tasks(user_only: bool) -> c_int {
    let mut what = user_only ? "user space processes" :
    "remaining freezable tasks";
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut end_time = 0;
    let mut todo = 0;
pub static mut wq_busy: bool = false;
    ktime_t start, end, elapsed;
    let mut elapsed_msecs = 0;
pub static mut wakeup: bool = false;
pub static mut sleep_usecs: c_int = 0;
    pr_info!("Freezing %s\n", what);
    start = ktime_get_boottime();
    end_time = jiffies + msecs_to_jiffies(freeze_timeout_msecs);
    if (!user_only) {
    freeze_workqueues_begin();
    }
    while (true) {
    todo = 0;
    read_lock(&tasklist_lock);
    for_each_process_thread(g, p) {
    if (p == current || !freeze_task(p)) {
    continue;
    }
    todo += 1;
    }
    read_unlock(&tasklist_lock);
    if (!user_only) {
    wq_busy = freeze_workqueues_busy();
    todo += wq_busy;
    }
    if (!todo || time_after(jiffies, end_time)) {
    break;
    }
    if (pm_wakeup_pending()) {
    wakeup = true;
    break;
    }
//
// We need to retry, but first give the freezing tasks some
// time to enter the refrigerator.  Start with an initial
// 1 ms sleep followed by exponential backoff until 8 ms.
//
    usleep_range(sleep_usecs / 2, sleep_usecs);
    if (sleep_usecs < 8 * USEC_PER_MSEC) {
    sleep_usecs *= 2;
    }
    }
    end = ktime_get_boottime();
    elapsed = ktime_sub(end, start);
    elapsed_msecs = ktime_to_ms(elapsed);
    if (todo) {
    pr_err!("Freezing %s %s after %d.%03d seconds "
    "(%d tasks refusing to freeze, wq_busy=%d):\n", what,
    wakeup ? "aborted" : "failed",
    elapsed_msecs / 1000, elapsed_msecs % 1000,
    todo - wq_busy, wq_busy);
    if (wq_busy) {
    show_freezable_workqueues();
    }
    if (!wakeup || pm_debug_messages_on) {
    read_lock(&tasklist_lock);
    for_each_process_thread(g, p) {
    if (p != current && freezing(p) && !frozen(p)) {
    sched_show_task(p);
    }
    }
    read_unlock(&tasklist_lock);
    }
    } else {
    pr_info!("Freezing %s completed (elapsed %d.%03d seconds)\n",
    what, elapsed_msecs / 1000, elapsed_msecs % 1000);
    }
    return todo ? -EBUSY : 0;
    }
//
// freeze_processes - Signal user space processes to enter the refrigerator.
// The current thread will not be frozen.  The same process that calls
// freeze_processes must later call thaw_processes.
//
// On success, returns 0.  On failure, -errno and system is fully thawed.
//
#[no_mangle]
pub unsafe extern "C" fn freeze_processes() -> c_int {
    let mut error = 0;
    error = __usermodehelper_disable(UMH_FREEZING);
    if (error) {
    return error;
    }
// Make sure this task doesn't get frozen
    current.flags |= PF_SUSPEND_TASK;
    if (!pm_freezing) {
    static_branch_inc(&freezer_active);
    }
    pm_wakeup_clear(0);
    pm_freezing = true;
    error = try_to_freeze_tasks(true);
    if (!error) {
    __usermodehelper_set_disable_depth(UMH_DISABLED);
    }
    BUG_ON!(in_atomic());
//
// Now that the whole userspace is frozen we need to disable
// the OOM killer to disallow any further interference with
// killable tasks. There is no guarantee oom victims will
// ever reach a point they go away we have to wait with a timeout.
//
    if (!error && !oom_killer_disable(msecs_to_jiffies(freeze_timeout_msecs))) {
    error = -EBUSY;
    }
    if (error) {
    thaw_processes();
    }
    return error;
    }
//
// freeze_kernel_threads - Make freezable kernel threads go to the refrigerator.
//
// On success, returns 0.  On failure, -errno and only the kernel threads are
// thawed, so as to give a chance to the caller to do additional cleanups
// (if any) before thawing the userspace tasks. So, it is the responsibility
// of the caller to thaw the userspace tasks, when the time is right.
//
#[no_mangle]
pub unsafe extern "C" fn freeze_kernel_threads() -> c_int {
    let mut error = 0;
    pm_nosig_freezing = true;
    error = try_to_freeze_tasks(false);
    BUG_ON!(in_atomic());
    if (error) {
    thaw_kernel_threads();
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn thaw_processes() {
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut curr = current;
    trace_suspend_resume(TPS("thaw_processes"), 0, true);
    if (pm_freezing) {
    static_branch_dec(&freezer_active);
    }
    pm_freezing = false;
    pm_nosig_freezing = false;
    oom_killer_enable();
    pr_info!("Restarting tasks: Starting\n");
    __usermodehelper_set_disable_depth(UMH_FREEZING);
    thaw_workqueues();
    read_lock(&tasklist_lock);
    for_each_process_thread(g, p) {
// No other threads should have PF_SUSPEND_TASK set
    WARN_ON!((p != curr) && (p.flags & PF_SUSPEND_TASK));
    __thaw_task(p);
    }
    read_unlock(&tasklist_lock);
    WARN_ON!(!(curr.flags & PF_SUSPEND_TASK));
    curr.flags &= ~PF_SUSPEND_TASK;
    usermodehelper_enable();
    schedule();
    pr_info!("Restarting tasks: Done\n");
    trace_suspend_resume(TPS("thaw_processes"), 0, false);
    }
#[no_mangle]
pub unsafe extern "C" fn thaw_kernel_threads() {
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    pm_nosig_freezing = false;
    pr_info!("Restarting kernel threads ...\n");
    thaw_workqueues();
    read_lock(&tasklist_lock);
    for_each_process_thread(g, p) {
    if (p.flags & PF_KTHREAD) {
    __thaw_task(p);
    }
    }
    read_unlock(&tasklist_lock);
    schedule();
    pr_info!("Done restarting kernel threads.\n");
    }