//! Automatically rewritten from C to Rust
//! Source: kernel/sched/core_sched.c
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
// A simple wrapper around refcount. An allocated sched_core_cookie's
// address is used to compute the cookie of the task.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_core_cookie {
    pub refcnt: refcount_t,
}

#[no_mangle]
unsafe extern "C" fn sched_core_alloc_cookie() -> c_ulong {
    let mut ck = kmalloc_obj(*ck);
    if (!ck) {
    return 0;
    }
    refcount_set(&ck.refcnt, 1);
    sched_core_get();
    return (unsigned long)ck;
    }
#[no_mangle]
unsafe extern "C" fn sched_core_put_cookie(cookie: c_ulong) {
    let mut ptr = cookie;
    if (ptr && refcount_dec_and_test(&ptr.refcnt)) {
    kfree(ptr);
    sched_core_put();
    }
    }
#[no_mangle]
unsafe extern "C" fn sched_core_get_cookie(cookie: c_ulong) -> c_ulong {
    let mut ptr = cookie;
    if (ptr) {
    refcount_inc(&ptr.refcnt);
    }
    return cookie;
    }
//
// sched_core_update_cookie - replace the cookie on a task
// @p: the task to update
// @cookie: the new cookie
//
// Effectively exchange the task cookie; caller is responsible for lifetimes on
// both ends.
//
// Returns: the old cookie
//
#[no_mangle]
pub unsafe extern "C" fn sched_core_update_cookie(p: *mut task_struct, cookie: c_ulong) -> c_ulong {
    let mut old_cookie = 0;
pub static mut rf: usize = 0;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    rq = task_rq_lock(p, &rf);
//
// Since creating a cookie implies sched_core_get(), and we cannot set
// a cookie until after we've created it, similarly, we cannot destroy
// a cookie until after we've removed it, we must have core scheduling
// enabled here.
//
    WARN_ON_ONCE!((p.core_cookie || cookie) && !sched_core_enabled(rq));
    if (sched_core_enqueued(p)) {
    sched_core_dequeue(rq, p, DEQUEUE_SAVE);
    }
    old_cookie = p.core_cookie;
    p.core_cookie = cookie;
//
// Consider the cases: !prev_cookie and !cookie.
//
    if (cookie && task_on_rq_queued(p)) {
    sched_core_enqueue(rq, p);
    }
//
// If task is currently running, it may not be compatible anymore after
// the cookie change, so enter the scheduler on its CPU to schedule it
// away.
//
// Note that it is possible that as a result of this cookie change, the
// core has now entered/left forced idle state. Defer accounting to the
// next scheduling edge, rather than always forcing a reschedule here.
//
    if (task_on_cpu(rq, p)) {
    resched_curr(rq);
    }
    task_rq_unlock(rq, p, &rf);
    return old_cookie;
    }
#[no_mangle]
unsafe extern "C" fn sched_core_clone_cookie(p: *mut task_struct) -> c_ulong {
    unsigned long cookie, flags;
    raw_spin_lock_irqsave(&p.pi_lock, flags);
    cookie = sched_core_get_cookie(p.core_cookie);
    raw_spin_unlock_irqrestore(&p.pi_lock, flags);
    return cookie;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_core_fork(p: *mut task_struct) {
    RB_CLEAR_NODE(&p.core_node);
    p.core_cookie = sched_core_clone_cookie(current);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_core_free(p: *mut task_struct) {
    sched_core_put_cookie(p.core_cookie);
    }
#[no_mangle]
unsafe extern "C" fn __sched_core_set(p: *mut task_struct, cookie: c_ulong) {
    cookie = sched_core_get_cookie(cookie);
    cookie = sched_core_update_cookie(p, cookie);
    sched_core_put_cookie(cookie);
    }
// Called from prctl interface: PR_SCHED_CORE
#[no_mangle]
pub unsafe extern "C" fn sched_core_share_pid(cmd: c_uint, pid: pid_t, type: pid_type, uaddr: c_ulong) -> c_int {
pub static mut cookie: c_ulong = 0;
    let mut task = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut grp: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (!sched_smt_active()) {
    return -ENODEV;
    }
    BUILD_BUG_ON!(PR_SCHED_CORE_SCOPE_THREAD != PIDTYPE_PID);
    BUILD_BUG_ON!(PR_SCHED_CORE_SCOPE_THREAD_GROUP != PIDTYPE_TGID);
    BUILD_BUG_ON!(PR_SCHED_CORE_SCOPE_PROCESS_GROUP != PIDTYPE_PGID);
    if (type > PIDTYPE_PGID || cmd >= PR_SCHED_CORE_MAX || pid < 0 ||
    (cmd != PR_SCHED_CORE_GET && uaddr)) {
    return -EINVAL;
    }
    rcu_read_lock();
    if (pid == 0) {
    task = current;
    } else {
    task = find_task_by_vpid(pid);
    if (!task) {
    rcu_read_unlock();
    return -ESRCH;
    }
    }
    get_task_struct(task);
    rcu_read_unlock();
//
// Check if this process has the right to modify the specified
// process. Use the regular "ptrace_may_access()" checks.
//
    if (!ptrace_may_access(task, PTRACE_MODE_READ_REALCREDS)) {
    err = -EPERM;
// goto;
    }
    match (cmd) {
    PR_SCHED_CORE_GET => {
    if (type != PIDTYPE_PID || uaddr & 7) {
    err = -EINVAL;
// goto;
    }
    cookie = sched_core_clone_cookie(task);
    if (cookie) {
// XXX improve ?
    ptr_to_hashval(cookie, &id);
    }
    err = put_user(id, uaddr);
// goto;
    }
    PR_SCHED_CORE_CREATE => {
    cookie = sched_core_alloc_cookie();
    if (!cookie) {
    err = -ENOMEM;
// goto;
    }
    // break;
    }
    PR_SCHED_CORE_SHARE_TO => {
    cookie = sched_core_clone_cookie(current);
    // break;
    }
    PR_SCHED_CORE_SHARE_FROM => {
    if (type != PIDTYPE_PID) {
    err = -EINVAL;
// goto;
    }
    cookie = sched_core_clone_cookie(task);
    __sched_core_set(current, cookie);
// goto;
    }
    _ => {
    err = -EINVAL;
// goto;
    }
    }
    if (type == PIDTYPE_PID) {
    __sched_core_set(task, cookie);
// goto;
    }
    read_lock(&tasklist_lock);
    grp = task_pid_type(task, type);
    do_each_pid_thread(grp, type, p) {
    if (!ptrace_may_access(p, PTRACE_MODE_READ_REALCREDS)) {
    err = -EPERM;
// goto;
    }
    } while_each_pid_thread(grp, type, p);
    do_each_pid_thread(grp, type, p) {
    __sched_core_set(p, cookie);
    } while_each_pid_thread(grp, type, p);
// label;
    read_unlock(&tasklist_lock);
// label;
    sched_core_put_cookie(cookie);
    put_task_struct(task);
    return err;
    }

// REQUIRES: rq->core's clock recently updated.
#[no_mangle]
pub unsafe extern "C" fn __sched_core_account_forceidle(rq: *mut rq) {
    let mut smt_mask = cpu_smt_mask(cpu_of(rq));
    u64 delta, now = rq_clock(rq.core);
pub static mut rq_i: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    lockdep_assert_rq_held(rq);
    WARN_ON_ONCE!(!rq.core.core_forceidle_count);
    if (rq.core.core_forceidle_start == 0) {
    return;
    }
    delta = now - rq.core.core_forceidle_start;
    if (unlikely((s64)delta <= 0)) {
    return;
    }
    rq.core.core_forceidle_start = now;
    if (WARN_ON_ONCE!(!rq.core.core_forceidle_occupation)) {
// can't be forced idle without a running task
    } else if (rq.core.core_forceidle_count > 1 ||
    rq.core.core_forceidle_occupation > 1) {
//
// For larger SMT configurations, we need to scale the charged
// forced idle amount since there can be more than one forced
// idle sibling and more than one running cookied task.
//
    delta *= rq.core.core_forceidle_count;
    delta = div_u64(delta, rq.core.core_forceidle_occupation);
    }
    for_each_cpu(i, smt_mask) {
    rq_i = cpu_rq(i);
    p = rq_i.core_pick ?: rq_i.curr;
    if (p == rq_i.idle) {
    continue;
    }
//
// Note: this will account forceidle to the current CPU, even
// if it comes from our SMT sibling.
//
    __account_forceidle_time(p, delta);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __sched_core_tick(rq: *mut rq) {
    if (!rq.core.core_forceidle_count) {
    return;
    }
    if (rq != rq.core) {
    update_rq_clock(rq.core);
    }
    __sched_core_account_forceidle(rq);
    }