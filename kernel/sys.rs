//! Automatically rewritten from C to Rust
//! Source: kernel/sys.c
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
//
// linux/kernel/sys.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//

// Move somewhere else to avoid recompiling?

//
// this is where the system-wide overflow UID and GID are defined, for
// architectures that now have 32-bit UID/GID but didn't in the past
//
pub static mut overflowuid: c_int = 0;
pub static mut overflowgid: c_int = 0;
    EXPORT_SYMBOL(overflowuid);
    EXPORT_SYMBOL(overflowgid);
//
// the same as above, but for filesystems which can only store a 16-bit
// UID and GID. as such, this is needed on all architectures
//
pub static mut fs_overflowuid: c_int = 0;
pub static mut fs_overflowgid: c_int = 0;
    EXPORT_SYMBOL(fs_overflowuid);
    EXPORT_SYMBOL(fs_overflowgid);
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_overflow_sysctl() -> c_int {
    register_sysctl_init("kernel", overflow_sysctl_table);
    return 0;
    }
    postcore_initcall!(init_overflow_sysctl);
//
// Returns true if current's euid is same as p's uid or euid,
// or has CAP_SYS_NICE to p's user_ns.
//
// Called with rcu_read_lock, creds are safe
//
#[no_mangle]
unsafe extern "C" fn set_one_prio_perm(p: *mut task_struct) -> bool {
    let mut cred = current_cred(), *pcred = __task_cred(p);
    if (uid_eq(pcred.uid,  cred.euid) ||
    uid_eq(pcred.euid, cred.euid)) {
    return true;
    }
    if (ns_capable(pcred.user_ns, CAP_SYS_NICE)) {
    return true;
    }
    return false;
    }
//
// set the priority of a task
// - the caller must hold the RCU read lock
//
#[no_mangle]
unsafe extern "C" fn set_one_prio(p: *mut task_struct, niceval: c_int, error: c_int) -> c_int {
    let mut no_nice = 0;
    if (!set_one_prio_perm(p)) {
    error = -EPERM;
// goto;
    }
    if (niceval < task_nice(p) && !can_nice(p, niceval)) {
    error = -EACCES;
// goto;
    }
    no_nice = security_task_setnice(p, niceval);
    if (no_nice) {
    error = no_nice;
// goto;
    }
    if (error == -ESRCH) {
    error = 0;
    }
    set_user_nice(p, niceval);
// label;
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setpriority(which: usize, who: usize, niceval: usize) -> c_long {
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut user: *mut c_void = core::ptr::null_mut();
    let mut cred = current_cred();
pub static mut error: c_int = 0;
pub static mut pgrp: *mut c_void = core::ptr::null_mut();
    let mut uid;
    if (which > PRIO_USER || which < PRIO_PROCESS) {
// goto;
    }
// normalize: avoid signed division (rounding problems)
    error = -ESRCH;
    if (niceval < MIN_NICE) {
    niceval = MIN_NICE;
    }
    if (niceval > MAX_NICE) {
    niceval = MAX_NICE;
    }
    rcu_read_lock();
    match (which) {
    PRIO_PROCESS => {
    if (who) {
    p = find_task_by_vpid(who);
    }
    else {
    p = current;
    }
    if (p) {
    error = set_one_prio(p, niceval, error);
    }
    // break;
    }
    PRIO_PGRP => {
    if (who) {
    pgrp = find_vpid(who);
    }
    else {
    pgrp = task_pgrp(current);
    }
    read_lock(&tasklist_lock);
    do_each_pid_thread(pgrp, PIDTYPE_PGID, p) {
    error = set_one_prio(p, niceval, error);
    } while_each_pid_thread(pgrp, PIDTYPE_PGID, p);
    read_unlock(&tasklist_lock);
    // break;
    }
    PRIO_USER => {
    uid = make_kuid(cred.user_ns, who);
    user = cred.user;
    if (!who) {
    uid = cred.uid;
    }
if true {
    user = find_user(uid);
    if (!user) {
// goto;	/* No processes for this user */
    }
    }
    for_each_process_thread(g, p) {
    if (uid_eq(task_uid(p), uid) && task_pid_vnr(p)) {
    error = set_one_prio(p, niceval, error);
    }
    }
    if (!uid_eq(uid, cred.uid)) {
    free_uid(user);		/* For find_user() */
    }
    // break;
    }
    }
// label;
    rcu_read_unlock();
// label;
    return error;
    }
//
// Ugh. To avoid negative return values, "getpriority()" will
// not return the normal nice-value, but a negated value that
// has been offset by 20 (ie it returns 40..1 instead of -20..19)
// to stay compatible.
//
#[no_mangle]
pub unsafe extern "C" fn sys_getpriority(which: usize, who: usize) -> c_long {
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut user: *mut c_void = core::ptr::null_mut();
    let mut cred = current_cred();
    long niceval, retval = -ESRCH;
pub static mut pgrp: *mut c_void = core::ptr::null_mut();
    let mut uid;
    if (which > PRIO_USER || which < PRIO_PROCESS) {
    return -EINVAL;
    }
    rcu_read_lock();
    match (which) {
    PRIO_PROCESS => {
    if (who) {
    p = find_task_by_vpid(who);
    }
    else {
    p = current;
    }
    if (p) {
    niceval = nice_to_rlimit(task_nice(p));
    if (niceval > retval) {
    retval = niceval;
    }
    }
    // break;
    }
    PRIO_PGRP => {
    if (who) {
    pgrp = find_vpid(who);
    }
    else {
    pgrp = task_pgrp(current);
    }
    read_lock(&tasklist_lock);
    do_each_pid_thread(pgrp, PIDTYPE_PGID, p) {
    niceval = nice_to_rlimit(task_nice(p));
    if (niceval > retval) {
    retval = niceval;
    }
    } while_each_pid_thread(pgrp, PIDTYPE_PGID, p);
    read_unlock(&tasklist_lock);
    // break;
    }
    PRIO_USER => {
    uid = make_kuid(cred.user_ns, who);
    user = cred.user;
    if (!who) {
    uid = cred.uid;
    }
if true {
    user = find_user(uid);
    if (!user) {
// goto;	/* No processes for this user */
    }
    }
    for_each_process_thread(g, p) {
    if (uid_eq(task_uid(p), uid) && task_pid_vnr(p)) {
    niceval = nice_to_rlimit(task_nice(p));
    if (niceval > retval) {
    retval = niceval;
    }
    }
    }
    if (!uid_eq(uid, cred.uid)) {
    free_uid(user);		/* for find_user() */
    }
    // break;
    }
    }
// label;
    rcu_read_unlock();
    return retval;
    }
//
// Unprivileged users may change the real gid to the effective gid
// or vice versa.  (BSD-style)
//
// If you set the real gid at all, or set the effective gid to a value not
// equal to the real gid, then the saved gid is set to the new effective gid.
//
// This makes it possible for a setgid program to completely drop its
// privileges, which is often a useful assertion to make when you are doing
// a security audit over a program.
//
// The general idea is that a program which uses just setregid() will be
// 100% compatible with BSD.  A program which uses just setgid() will be
// 100% compatible with POSIX with saved IDs.
//
// SMP: There are not races, the GIDs are checked only by filesystem
// operations (as far as semantic preservation is concerned).
//

#[no_mangle]
pub unsafe extern "C" fn __sys_setregid(rgid: gid_t, egid: gid_t) -> c_long {
    let mut ns = current_user_ns();
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    kgid_t krgid, kegid;
    krgid = make_kgid(ns, rgid);
    kegid = make_kgid(ns, egid);
    if ((rgid != (gid_t) -1) && !gid_valid(krgid)) {
    return -EINVAL;
    }
    if ((egid != (gid_t) -1) && !gid_valid(kegid)) {
    return -EINVAL;
    }
    new = prepare_creds();
    if (!new) {
    return -ENOMEM;
    }
    old = current_cred();
    retval = -EPERM;
    if (rgid != (gid_t) -1) {
    if (gid_eq(old.gid, krgid) ||
    gid_eq(old.egid, krgid) ||
    ns_capable_setid(old.user_ns, CAP_SETGID)) {
    new.gid = krgid;
    }
    else {
// goto;
    }
    }
    if (egid != (gid_t) -1) {
    if (gid_eq(old.gid, kegid) ||
    gid_eq(old.egid, kegid) ||
    gid_eq(old.sgid, kegid) ||
    ns_capable_setid(old.user_ns, CAP_SETGID)) {
    new.egid = kegid;
    }
    else {
// goto;
    }
    }
    if (rgid != (gid_t) -1 ||
    (egid != (gid_t) -1 && !gid_eq(kegid, old.gid))) {
    new.sgid = new.egid;
    }
    new.fsgid = new.egid;
    retval = security_task_fix_setgid(new, old, LSM_SETID_RE);
    if (retval < 0) {
// goto;
    }
    return commit_creds(new);
// label;
    abort_creds(new);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setregid(rgid: usize, egid: usize) -> c_long {
    return __sys_setregid(rgid, egid);
    }
//
// setgid() is implemented like SysV w/ SAVED_IDS
//
// SMP: Same implicit races as above.
//
#[no_mangle]
pub unsafe extern "C" fn __sys_setgid(gid: gid_t) -> c_long {
    let mut ns = current_user_ns();
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    let mut kgid;
    kgid = make_kgid(ns, gid);
    if (!gid_valid(kgid)) {
    return -EINVAL;
    }
    new = prepare_creds();
    if (!new) {
    return -ENOMEM;
    }
    old = current_cred();
    retval = -EPERM;
    if (ns_capable_setid(old.user_ns, CAP_SETGID)) {
    new.gid = new.egid = new.sgid = new.fsgid = kgid;
    }

    else if (gid_eq(kgid, old.gid) || gid_eq(kgid, old.sgid)) {
    new.egid = new.fsgid = kgid;
    }
    else {
// goto;
    }
    retval = security_task_fix_setgid(new, old, LSM_SETID_ID);
    if (retval < 0) {
// goto;
    }
    return commit_creds(new);
// label;
    abort_creds(new);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setgid(gid: usize) -> c_long {
    return __sys_setgid(gid);
    }
//
// change the user struct in a credentials set to match the new UID
//
#[no_mangle]
unsafe extern "C" fn set_user(new: *mut cred) -> c_int {
pub static mut new_user: *mut c_void = core::ptr::null_mut();
    new_user = alloc_uid(new.uid);
    if (!new_user) {
    return -EAGAIN;
    }
    free_uid(new.user);
    new.user = new_user;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flag_nproc_exceeded(new: *mut cred) {
    if (new.ucounts == current_ucounts()) {
    return;
    }
//
// We don't fail in case of NPROC limit excess here because too many
// poorly written programs don't check set*uid() return code, assuming
// it never fails if called by root.  We may still enforce NPROC limit
// for programs doing set*uid()+execve() by harmlessly deferring the
// failure to the execve() stage.
//
    if (is_rlimit_overlimit(new.ucounts, UCOUNT_RLIMIT_NPROC, rlimit(RLIMIT_NPROC)) &&
    new.user != INIT_USER) {
    current.flags |= PF_NPROC_EXCEEDED;
    }
    else {
    current.flags &= ~PF_NPROC_EXCEEDED;
    }
    }
//
// Unprivileged users may change the real uid to the effective uid
// or vice versa.  (BSD-style)
//
// If you set the real uid at all, or set the effective uid to a value not
// equal to the real uid, then the saved uid is set to the new effective uid.
//
// This makes it possible for a setuid program to completely drop its
// privileges, which is often a useful assertion to make when you are doing
// a security audit over a program.
//
// The general idea is that a program which uses just setreuid() will be
// 100% compatible with BSD.  A program which uses just setuid() will be
// 100% compatible with POSIX with saved IDs.
//
#[no_mangle]
pub unsafe extern "C" fn __sys_setreuid(ruid: uid_t, euid: uid_t) -> c_long {
    let mut ns = current_user_ns();
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    kuid_t kruid, keuid;
    kruid = make_kuid(ns, ruid);
    keuid = make_kuid(ns, euid);
    if ((ruid != (uid_t) -1) && !uid_valid(kruid)) {
    return -EINVAL;
    }
    if ((euid != (uid_t) -1) && !uid_valid(keuid)) {
    return -EINVAL;
    }
    new = prepare_creds();
    if (!new) {
    return -ENOMEM;
    }
    old = current_cred();
    retval = -EPERM;
    if (ruid != (uid_t) -1) {
    new.uid = kruid;
    if (!uid_eq(old.uid, kruid) &&
    !uid_eq(old.euid, kruid) &&
    !ns_capable_setid(old.user_ns, CAP_SETUID)) {
// goto;
    }
    }
    if (euid != (uid_t) -1) {
    new.euid = keuid;
    if (!uid_eq(old.uid, keuid) &&
    !uid_eq(old.euid, keuid) &&
    !uid_eq(old.suid, keuid) &&
    !ns_capable_setid(old.user_ns, CAP_SETUID)) {
// goto;
    }
    }
    if (!uid_eq(new.uid, old.uid)) {
    retval = set_user(new);
    if (retval < 0) {
// goto;
    }
    }
    if (ruid != (uid_t) -1 ||
    (euid != (uid_t) -1 && !uid_eq(keuid, old.uid))) {
    new.suid = new.euid;
    }
    new.fsuid = new.euid;
    retval = security_task_fix_setuid(new, old, LSM_SETID_RE);
    if (retval < 0) {
// goto;
    }
    retval = set_cred_ucounts(new);
    if (retval < 0) {
// goto;
    }
    flag_nproc_exceeded(new);
    return commit_creds(new);
// label;
    abort_creds(new);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setreuid(ruid: usize, euid: usize) -> c_long {
    return __sys_setreuid(ruid, euid);
    }
//
// setuid() is implemented like SysV with SAVED_IDS
//
// Note that SAVED_ID's is deficient in that a setuid root program
// like sendmail, for example, cannot set its uid to be a normal
// user and then switch back, because if you're root, setuid() sets
// the saved uid too.  If you don't like this, blame the bright people
// in the POSIX committee and/or USG.  Note that the BSD-style setreuid()
// will allow a root program to temporarily drop privileges and be able to
// regain them by swapping the real and effective uid.
//
#[no_mangle]
pub unsafe extern "C" fn __sys_setuid(uid: uid_t) -> c_long {
    let mut ns = current_user_ns();
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    let mut kuid;
    kuid = make_kuid(ns, uid);
    if (!uid_valid(kuid)) {
    return -EINVAL;
    }
    new = prepare_creds();
    if (!new) {
    return -ENOMEM;
    }
    old = current_cred();
    retval = -EPERM;
    if (ns_capable_setid(old.user_ns, CAP_SETUID)) {
    new.suid = new.uid = kuid;
    if (!uid_eq(kuid, old.uid)) {
    retval = set_user(new);
    if (retval < 0) {
// goto;
    }
    }
    } else if (!uid_eq(kuid, old.uid) && !uid_eq(kuid, new.suid)) {
// goto;
    }
    new.fsuid = new.euid = kuid;
    retval = security_task_fix_setuid(new, old, LSM_SETID_ID);
    if (retval < 0) {
// goto;
    }
    retval = set_cred_ucounts(new);
    if (retval < 0) {
// goto;
    }
    flag_nproc_exceeded(new);
    return commit_creds(new);
// label;
    abort_creds(new);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setuid(uid: usize) -> c_long {
    return __sys_setuid(uid);
    }
//
// This function implements a generic ability to update ruid, euid,
// and suid.  This allows you to implement the 4.4 compatible seteuid().
//
#[no_mangle]
pub unsafe extern "C" fn __sys_setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_long {
    let mut ns = current_user_ns();
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    kuid_t kruid, keuid, ksuid;
    let mut ruid_new = 0;
    let mut euid_new = 0;
    let mut suid_new = 0;
    kruid = make_kuid(ns, ruid);
    keuid = make_kuid(ns, euid);
    ksuid = make_kuid(ns, suid);
    if ((ruid != (uid_t) -1) && !uid_valid(kruid)) {
    return -EINVAL;
    }
    if ((euid != (uid_t) -1) && !uid_valid(keuid)) {
    return -EINVAL;
    }
    if ((suid != (uid_t) -1) && !uid_valid(ksuid)) {
    return -EINVAL;
    }
    old = current_cred();
// check for no-op
    if ((ruid == (uid_t) -1 || uid_eq(kruid, old.uid)) &&
    (euid == (uid_t) -1 || (uid_eq(keuid, old.euid) &&
    uid_eq(keuid, old.fsuid))) &&
    (suid == (uid_t) -1 || uid_eq(ksuid, old.suid))) {
    return 0;
    }
    ruid_new = ruid != (uid_t) -1        && !uid_eq(kruid, old.uid) &&
    !uid_eq(kruid, old.euid) && !uid_eq(kruid, old.suid);
    euid_new = euid != (uid_t) -1        && !uid_eq(keuid, old.uid) &&
    !uid_eq(keuid, old.euid) && !uid_eq(keuid, old.suid);
    suid_new = suid != (uid_t) -1        && !uid_eq(ksuid, old.uid) &&
    !uid_eq(ksuid, old.euid) && !uid_eq(ksuid, old.suid);
    if ((ruid_new || euid_new || suid_new) &&
    !ns_capable_setid(old.user_ns, CAP_SETUID)) {
    return -EPERM;
    }
    new = prepare_creds();
    if (!new) {
    return -ENOMEM;
    }
    if (ruid != (uid_t) -1) {
    new.uid = kruid;
    if (!uid_eq(kruid, old.uid)) {
    retval = set_user(new);
    if (retval < 0) {
// goto;
    }
    }
    }
    if (euid != (uid_t) -1) {
    new.euid = keuid;
    }
    if (suid != (uid_t) -1) {
    new.suid = ksuid;
    }
    new.fsuid = new.euid;
    retval = security_task_fix_setuid(new, old, LSM_SETID_RES);
    if (retval < 0) {
// goto;
    }
    retval = set_cred_ucounts(new);
    if (retval < 0) {
// goto;
    }
    flag_nproc_exceeded(new);
    return commit_creds(new);
// label;
    abort_creds(new);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setresuid(ruid: usize, euid: usize, suid: usize) -> c_long {
    return __sys_setresuid(ruid, euid, suid);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getresuid(ruidp: usize, euidp: usize, suidp: usize) -> c_long {
    let mut cred = current_cred();
    let mut retval = 0;
    uid_t ruid, euid, suid;
    ruid = from_kuid_munged(cred.user_ns, cred.uid);
    euid = from_kuid_munged(cred.user_ns, cred.euid);
    suid = from_kuid_munged(cred.user_ns, cred.suid);
    retval = put_user(ruid, ruidp);
    if (!retval) {
    retval = put_user(euid, euidp);
    if (!retval) {
    return put_user(suid, suidp);
    }
    }
    return retval;
    }
//
// Same as above, but for rgid, egid, sgid.
//
#[no_mangle]
pub unsafe extern "C" fn __sys_setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_long {
    let mut ns = current_user_ns();
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    kgid_t krgid, kegid, ksgid;
    let mut rgid_new = 0;
    let mut egid_new = 0;
    let mut sgid_new = 0;
    krgid = make_kgid(ns, rgid);
    kegid = make_kgid(ns, egid);
    ksgid = make_kgid(ns, sgid);
    if ((rgid != (gid_t) -1) && !gid_valid(krgid)) {
    return -EINVAL;
    }
    if ((egid != (gid_t) -1) && !gid_valid(kegid)) {
    return -EINVAL;
    }
    if ((sgid != (gid_t) -1) && !gid_valid(ksgid)) {
    return -EINVAL;
    }
    old = current_cred();
// check for no-op
    if ((rgid == (gid_t) -1 || gid_eq(krgid, old.gid)) &&
    (egid == (gid_t) -1 || (gid_eq(kegid, old.egid) &&
    gid_eq(kegid, old.fsgid))) &&
    (sgid == (gid_t) -1 || gid_eq(ksgid, old.sgid))) {
    return 0;
    }
    rgid_new = rgid != (gid_t) -1        && !gid_eq(krgid, old.gid) &&
    !gid_eq(krgid, old.egid) && !gid_eq(krgid, old.sgid);
    egid_new = egid != (gid_t) -1        && !gid_eq(kegid, old.gid) &&
    !gid_eq(kegid, old.egid) && !gid_eq(kegid, old.sgid);
    sgid_new = sgid != (gid_t) -1        && !gid_eq(ksgid, old.gid) &&
    !gid_eq(ksgid, old.egid) && !gid_eq(ksgid, old.sgid);
    if ((rgid_new || egid_new || sgid_new) &&
    !ns_capable_setid(old.user_ns, CAP_SETGID)) {
    return -EPERM;
    }
    new = prepare_creds();
    if (!new) {
    return -ENOMEM;
    }
    if (rgid != (gid_t) -1) {
    new.gid = krgid;
    }
    if (egid != (gid_t) -1) {
    new.egid = kegid;
    }
    if (sgid != (gid_t) -1) {
    new.sgid = ksgid;
    }
    new.fsgid = new.egid;
    retval = security_task_fix_setgid(new, old, LSM_SETID_RES);
    if (retval < 0) {
// goto;
    }
    return commit_creds(new);
// label;
    abort_creds(new);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setresgid(rgid: usize, egid: usize, sgid: usize) -> c_long {
    return __sys_setresgid(rgid, egid, sgid);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getresgid(rgidp: usize, egidp: usize, sgidp: usize) -> c_long {
    let mut cred = current_cred();
    let mut retval = 0;
    gid_t rgid, egid, sgid;
    rgid = from_kgid_munged(cred.user_ns, cred.gid);
    egid = from_kgid_munged(cred.user_ns, cred.egid);
    sgid = from_kgid_munged(cred.user_ns, cred.sgid);
    retval = put_user(rgid, rgidp);
    if (!retval) {
    retval = put_user(egid, egidp);
    if (!retval) {
    retval = put_user(sgid, sgidp);
    }
    }
    return retval;
    }
//
// "setfsuid()" sets the fsuid - the uid used for filesystem checks. This
// is used for "access()" and for the NFS daemon (letting nfsd stay at
// whatever uid it wants to). It normally shadows "euid", except when
// explicitly set by setfsuid() or for access..
//
#[no_mangle]
pub unsafe extern "C" fn __sys_setfsuid(uid: uid_t) -> c_long {
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut old_fsuid = 0;
    let mut kuid;
    old = current_cred();
    old_fsuid = from_kuid_munged(old.user_ns, old.fsuid);
    kuid = make_kuid(old.user_ns, uid);
    if (!uid_valid(kuid)) {
    return old_fsuid;
    }
    new = prepare_creds();
    if (!new) {
    return old_fsuid;
    }
    if (uid_eq(kuid, old.uid)  || uid_eq(kuid, old.euid)  ||
    uid_eq(kuid, old.suid) || uid_eq(kuid, old.fsuid) ||
    ns_capable_setid(old.user_ns, CAP_SETUID)) {
    if (!uid_eq(kuid, old.fsuid)) {
    new.fsuid = kuid;
    if (security_task_fix_setuid(new, old, LSM_SETID_FS) == 0) {
// goto;
    }
    }
    }
    abort_creds(new);
    return old_fsuid;
// label;
    commit_creds(new);
    return old_fsuid;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setfsuid(uid: usize) -> c_long {
    return __sys_setfsuid(uid);
    }
//
// Samma på svenska..
//
#[no_mangle]
pub unsafe extern "C" fn __sys_setfsgid(gid: gid_t) -> c_long {
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut old_fsgid = 0;
    let mut kgid;
    old = current_cred();
    old_fsgid = from_kgid_munged(old.user_ns, old.fsgid);
    kgid = make_kgid(old.user_ns, gid);
    if (!gid_valid(kgid)) {
    return old_fsgid;
    }
    new = prepare_creds();
    if (!new) {
    return old_fsgid;
    }
    if (gid_eq(kgid, old.gid)  || gid_eq(kgid, old.egid)  ||
    gid_eq(kgid, old.sgid) || gid_eq(kgid, old.fsgid) ||
    ns_capable_setid(old.user_ns, CAP_SETGID)) {
    if (!gid_eq(kgid, old.fsgid)) {
    new.fsgid = kgid;
    if (security_task_fix_setgid(new,old,LSM_SETID_FS) == 0) {
// goto;
    }
    }
    }
    abort_creds(new);
    return old_fsgid;
// label;
    commit_creds(new);
    return old_fsgid;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setfsgid(gid: usize) -> c_long {
    return __sys_setfsgid(gid);
    }

//
// sys_getpid - return the thread group id of the current process
//
// Note, despite the name, this returns the tgid not the pid.  The tgid and
// the pid are identical unless CLONE_THREAD was specified on clone() in
// which case the tgid is the same in all threads of the same group.
//
// This is SMP safe as current->tgid does not change.
//
#[no_mangle]
pub unsafe extern "C" fn sys_getpid() -> c_long {
    return task_tgid_vnr(current);
    }
// Thread ID - the internal kernel "pid"
#[no_mangle]
pub unsafe extern "C" fn sys_gettid() -> c_long {
    return task_pid_vnr(current);
    }
//
// Accessing ->real_parent is not SMP-safe, it could
// change from under us. However, we can use a stale
// value of ->real_parent under rcu_read_lock(), see
// release_task()->call_rcu(delayed_put_task_struct).
//
#[no_mangle]
pub unsafe extern "C" fn sys_getppid() -> c_long {
    let mut pid = 0;
    rcu_read_lock();
    pid = task_tgid_vnr(rcu_dereference(current.real_parent));
    rcu_read_unlock();
    return pid;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getuid() -> c_long {
// Only we change this so SMP safe
    return from_kuid_munged(current_user_ns(), current_uid());
    }
#[no_mangle]
pub unsafe extern "C" fn sys_geteuid() -> c_long {
// Only we change this so SMP safe
    return from_kuid_munged(current_user_ns(), current_euid());
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getgid() -> c_long {
// Only we change this so SMP safe
    return from_kgid_munged(current_user_ns(), current_gid());
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getegid() -> c_long {
// Only we change this so SMP safe
    return from_kgid_munged(current_user_ns(), current_egid());
    }
#[no_mangle]
unsafe extern "C" fn do_sys_times(tms: *mut tms) {
    u64 tgutime, tgstime, cutime, cstime;
    thread_group_cputime_adjusted(current, &tgutime, &tgstime);
    cutime = current.signal.cutime;
    cstime = current.signal.cstime;
    tms.tms_utime = nsec_to_clock_t(tgutime);
    tms.tms_stime = nsec_to_clock_t(tgstime);
    tms.tms_cutime = nsec_to_clock_t(cutime);
    tms.tms_cstime = nsec_to_clock_t(cstime);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_times(tbuf: usize) -> c_long {
    if (tbuf) {
pub static mut tmp: usize = 0;
    do_sys_times(&tmp);
    if (copy_to_user(tbuf, &tmp, sizeof!(tms))) {
    return -EFAULT;
    }
    }
    force_successful_syscall_return();
    return (long) jiffies_64_to_clock_t(get_jiffies_64());
    }

#[no_mangle]
unsafe extern "C" fn clock_t_to_compat_clock_t(x: clock_t) -> compat_clock_t {
    return compat_jiffies_to_clock_t(clock_t_to_jiffies(x));
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_times
pub unsafe extern "C" fn sys_times_dup(tbuf: usize) -> c_long {
    if (tbuf) {
pub static mut tms: usize = 0;
pub static mut tmp: usize = 0;
    do_sys_times(&tms);
// Convert our struct tms to the compat version.
    tmp.tms_utime = clock_t_to_compat_clock_t(tms.tms_utime);
    tmp.tms_stime = clock_t_to_compat_clock_t(tms.tms_stime);
    tmp.tms_cutime = clock_t_to_compat_clock_t(tms.tms_cutime);
    tmp.tms_cstime = clock_t_to_compat_clock_t(tms.tms_cstime);
    if (copy_to_user(tbuf, &tmp, sizeof!(tmp))) {
    return -EFAULT;
    }
    }
    force_successful_syscall_return();
    return compat_jiffies_to_clock_t(jiffies);
    }

//
// This needs some heavy checking ...
// I just haven't the stomach for it. I also don't fully
// understand sessions/pgrp etc. Let somebody who does explain it.
//
// OK, I think I have the protection semantics right.... this is really
// only important on a multi-user system anyway, to make sure one user
// can't send a signal to a process owned by another.  -TYT, 12/12/91
//
// !PF_FORKNOEXEC check to conform completely to POSIX.
//
#[no_mangle]
pub unsafe extern "C" fn sys_setpgid(pid: usize, pgid: usize) -> c_long {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut group_leader = current.group_leader;
    struct pid *pids[PIDTYPE_MAX] = { 0 };
pub static mut pgrp: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!pid) {
    pid = task_pid_vnr(group_leader);
    }
    if (!pgid) {
    pgid = pid;
    }
    if (pgid < 0) {
    return -EINVAL;
    }
    rcu_read_lock();
// From this point forward we keep holding onto the tasklist lock
// so that our parent does not change from under us. -DaveM
//
    write_lock_irq(&tasklist_lock);
    err = -ESRCH;
    p = find_task_by_vpid(pid);
    if (!p) {
// goto;
    }
    err = -EINVAL;
    if (!thread_group_leader(p)) {
// goto;
    }
    if (same_thread_group(p.real_parent, group_leader)) {
    err = -EPERM;
    if (task_session(p) != task_session(group_leader)) {
// goto;
    }
    err = -EACCES;
    if (!(p.flags & PF_FORKNOEXEC)) {
// goto;
    }
    } else {
    err = -ESRCH;
    if (p != group_leader) {
// goto;
    }
    }
    err = -EPERM;
    if (p.signal.leader) {
// goto;
    }
    pgrp = task_pid(p);
    if (pgid != pid) {
pub static mut g: *mut c_void = core::ptr::null_mut();
    pgrp = find_vpid(pgid);
    g = pid_task(pgrp, PIDTYPE_PGID);
    if (!g || task_session(g) != task_session(group_leader)) {
// goto;
    }
    }
    err = security_task_setpgid(p, pgid);
    if (err) {
// goto;
    }
    if (task_pgrp(p) != pgrp) {
    change_pid(pids, p, PIDTYPE_PGID, pgrp);
    }
    err = 0;
// label;
// All paths lead to here, thus we are safe. -DaveM
    write_unlock_irq(&tasklist_lock);
    rcu_read_unlock();
    free_pids(pids);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn do_getpgid(pid: pid_t) -> c_int {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut grp: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    rcu_read_lock();
    if (!pid) {
    grp = task_pgrp(current);
    }
    else {
    retval = -ESRCH;
    p = find_task_by_vpid(pid);
    if (!p) {
// goto;
    }
    grp = task_pgrp(p);
    if (!grp) {
// goto;
    }
    retval = security_task_getpgid(p);
    if (retval) {
// goto;
    }
    }
    retval = pid_vnr(grp);
// label;
    rcu_read_unlock();
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getpgid(pid: usize) -> c_long {
    return do_getpgid(pid);
    }

#[no_mangle]
pub unsafe extern "C" fn sys_getpgrp() -> c_long {
    return do_getpgid(0);
    }

#[no_mangle]
pub unsafe extern "C" fn sys_getsid(pid: usize) -> c_long {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut sid: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    rcu_read_lock();
    if (!pid) {
    sid = task_session(current);
    }
    else {
    retval = -ESRCH;
    p = find_task_by_vpid(pid);
    if (!p) {
// goto;
    }
    sid = task_session(p);
    if (!sid) {
// goto;
    }
    retval = security_task_getsid(p);
    if (retval) {
// goto;
    }
    }
    retval = pid_vnr(sid);
// label;
    rcu_read_unlock();
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn set_special_pids(pids: *mut pid, pid: *mut pid) {
    let mut curr = current.group_leader;
    if (task_session(curr) != pid) {
    change_pid(pids, curr, PIDTYPE_SID, pid);
    }
    if (task_pgrp(curr) != pid) {
    change_pid(pids, curr, PIDTYPE_PGID, pid);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ksys_setsid() -> c_int {
    let mut group_leader = current.group_leader;
    let mut sid = task_pid(group_leader);
    struct pid *pids[PIDTYPE_MAX] = { 0 };
pub static mut session: pid_t = 0;
pub static mut err: c_int = 0;
    write_lock_irq(&tasklist_lock);
// Fail if I am already a session leader
    if (group_leader.signal.leader) {
// goto;
    }
// Fail if a process group id already exists that equals the
// proposed session id.
//
    if (pid_task(sid, PIDTYPE_PGID)) {
// goto;
    }
    group_leader.signal.leader = 1;
    set_special_pids(pids, sid);
    proc_clear_tty(group_leader);
    err = session;
// label;
    write_unlock_irq(&tasklist_lock);
    free_pids(pids);
    if (err > 0) {
    proc_sid_connector(group_leader);
    sched_autogroup_create_attach(group_leader);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setsid() -> c_long {
    return ksys_setsid();
    }
pub static mut uts_sem: usize = 0;

    (personality(current.personality) == PER_LINUX32 && 
    copy_to_user(name.machine, COMPAT_UTS_MACHINE, 
    sizeof!(COMPAT_UTS_MACHINE)))

pub const override_architecture(name): c_int = 0;

//
// Work around broken programs that cannot handle "Linux 3.0".
// Instead we map 3.x to 2.6.40+x, so e.g. 3.0 would be 2.6.40
// And we map 4.x and later versions to 2.6.60+x, so 4.0/5.0/6.0/... would be
// 2.6.60.
//
#[no_mangle]
unsafe extern "C" fn override_release(release: *mut char , len: usize) -> c_int {
pub static mut ret: c_int = 0;
    if (current.personality & UNAME26) {
    let mut rest = UTS_RELEASE;
    char buf[65] = { 0 };
pub static mut ndots: c_int = 0;
    let mut v: c_uint = 0;
    let mut copy = 0;
    while (*rest) {
    if (*rest == '.' && ++ndots >= 3) {
    break;
    }
    if (!isdigit(*rest) && *rest != '.') {
    break;
    }
    rest += 1;
    }
    v = LINUX_VERSION_PATCHLEVEL + 60;
    copy = clamp_t(size_t, len, 1, sizeof!(buf));
    copy = scnprintf(buf, copy, "2.6.%u%s", v, rest);
    ret = copy_to_user(release, buf, copy + 1);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_newuname(name: usize) -> c_long {
pub static mut tmp: usize = 0;
    down_read(&uts_sem);
    memcpy(&tmp, utsname(), sizeof!(tmp));
    up_read(&uts_sem);
    if (copy_to_user(name, &tmp, sizeof!(tmp))) {
    return -EFAULT;
    }
    if (override_release(name.release, sizeof!(name.release))) {
    return -EFAULT;
    }
    if (override_architecture(name)) {
    return -EFAULT;
    }
    return 0;
    }

//
// Old cruft
//
#[no_mangle]
pub unsafe extern "C" fn sys_uname(name: usize) -> c_long {
pub static mut tmp: usize = 0;
    if (!name) {
    return -EFAULT;
    }
    down_read(&uts_sem);
    memcpy(&tmp, utsname(), sizeof!(tmp));
    up_read(&uts_sem);
    if (copy_to_user(name, &tmp, sizeof!(tmp))) {
    return -EFAULT;
    }
    if (override_release(name.release, sizeof!(name.release))) {
    return -EFAULT;
    }
    if (override_architecture(name)) {
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_olduname(name: usize) -> c_long {
pub static mut tmp: usize = 0;
    if (!name) {
    return -EFAULT;
    }
    memset(&tmp, 0, sizeof!(tmp));
    down_read(&uts_sem);
    memcpy(&tmp.sysname, &utsname().sysname, __OLD_UTS_LEN);
    memcpy(&tmp.nodename, &utsname().nodename, __OLD_UTS_LEN);
    memcpy(&tmp.release, &utsname().release, __OLD_UTS_LEN);
    memcpy(&tmp.version, &utsname().version, __OLD_UTS_LEN);
    memcpy(&tmp.machine, &utsname().machine, __OLD_UTS_LEN);
    up_read(&uts_sem);
    if (copy_to_user(name, &tmp, sizeof!(tmp))) {
    return -EFAULT;
    }
    if (override_architecture(name)) {
    return -EFAULT;
    }
    if (override_release(name.release, sizeof!(name.release))) {
    return -EFAULT;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn sys_sethostname(name: usize, len: usize) -> c_long {
    let mut errno = 0;
    char tmp[__NEW_UTS_LEN];
    if (!ns_capable(current.nsproxy.uts_ns.user_ns, CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    if (len < 0 || len > __NEW_UTS_LEN) {
    return -EINVAL;
    }
    errno = -EFAULT;
    if (!copy_from_user(tmp, name, len)) {
pub static mut u: *mut c_void = core::ptr::null_mut();
    add_device_randomness(tmp, len);
    down_write(&uts_sem);
    u = utsname();
    memcpy(u.nodename, tmp, len);
    memset(u.nodename + len, 0, sizeof!(u.nodename) - len);
    errno = 0;
    uts_proc_notify(UTS_PROC_HOSTNAME);
    up_write(&uts_sem);
    }
    return errno;
    }

#[no_mangle]
pub unsafe extern "C" fn sys_gethostname(name: usize, len: usize) -> c_long {
    let mut i = 0;
pub static mut u: *mut c_void = core::ptr::null_mut();
    char tmp[__NEW_UTS_LEN + 1];
    if (len < 0) {
    return -EINVAL;
    }
    down_read(&uts_sem);
    u = utsname();
    i = 1 + strlen(u.nodename);
    if (i > len) {
    i = len;
    }
    memcpy(tmp, u.nodename, i);
    up_read(&uts_sem);
    if (copy_to_user(name, tmp, i)) {
    return -EFAULT;
    }
    return 0;
    }

//
// Only setdomainname; getdomainname can be implemented by calling
// uname()
//
#[no_mangle]
pub unsafe extern "C" fn sys_setdomainname(name: usize, len: usize) -> c_long {
    let mut errno = 0;
    char tmp[__NEW_UTS_LEN];
    if (!ns_capable(current.nsproxy.uts_ns.user_ns, CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    if (len < 0 || len > __NEW_UTS_LEN) {
    return -EINVAL;
    }
    errno = -EFAULT;
    if (!copy_from_user(tmp, name, len)) {
pub static mut u: *mut c_void = core::ptr::null_mut();
    add_device_randomness(tmp, len);
    down_write(&uts_sem);
    u = utsname();
    memcpy(u.domainname, tmp, len);
    memset(u.domainname + len, 0, sizeof!(u.domainname) - len);
    errno = 0;
    uts_proc_notify(UTS_PROC_DOMAINNAME);
    up_write(&uts_sem);
    }
    return errno;
    }
// make sure you are allowed to change @tsk limits before calling this
#[no_mangle]
pub unsafe extern "C" fn do_prlimit(tsk: *mut task_struct, resource: c_uint, new_rlim: *mut rlimit, old_rlim: *mut rlimit) -> c_int {
pub static mut rlim: *mut c_void = core::ptr::null_mut();
pub static mut retval: c_int = 0;
    if (resource >= RLIM_NLIMITS) {
    return -EINVAL;
    }
    resource = array_index_nospec(resource, RLIM_NLIMITS);
    if (new_rlim) {
    if (new_rlim.rlim_cur > new_rlim.rlim_max) {
    return -EINVAL;
    }
    if (resource == RLIMIT_NOFILE &&
    new_rlim.rlim_max > sysctl_nr_open) {
    return -EPERM;
    }
    }
// Holding a refcount on tsk protects tsk->signal from disappearing.
    rlim = tsk.signal.rlim + resource;
    task_lock(tsk.group_leader);
    if (new_rlim) {
//
// Keep the capable check against init_user_ns until cgroups can
// contain all limits.
//
    if (new_rlim.rlim_max > rlim.rlim_max &&
    !capable(CAP_SYS_RESOURCE)) {
    retval = -EPERM;
    }
    if (!retval) {
    retval = security_task_setrlimit(tsk, resource, new_rlim);
    }
    }
    if (!retval) {
    if (old_rlim) {
// old_rlim = *rlim;
    }
    if (new_rlim) {
// rlim = *new_rlim;
    }
    }
    task_unlock(tsk.group_leader);
//
// RLIMIT_CPU handling. Arm the posix CPU timer if the limit is not
// infinite. In case of RLIM_INFINITY the posix CPU timer code
// ignores the rlimit.
//
    if (!retval && new_rlim && resource == RLIMIT_CPU &&
    new_rlim.rlim_cur != RLIM_INFINITY &&
    IS_ENABLED!(CONFIG_POSIX_TIMERS)) {
//
// update_rlimit_cpu can fail if the task is exiting, but there
// may be other tasks in the thread group that are not exiting,
// and they need their cpu timers adjusted.
//
// The group_leader is the last task to be released, so if we
// cannot update_rlimit_cpu on it, then the entire process is
// exiting and we do not need to update at all.
//
    update_rlimit_cpu(tsk.group_leader, new_rlim.rlim_cur);
    }
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getrlimit(resource: usize, rlim: usize) -> c_long {
pub static mut value: usize = 0;
    let mut ret = 0;
    ret = do_prlimit(current, resource, core::ptr::null_mut(), &value);
    if (!ret) {
    ret = copy_to_user(rlim, &value, sizeof!(*rlim)) ? -EFAULT : 0;
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn sys_setrlimit(resource: usize, rlim: usize) -> c_long {
pub static mut r: usize = 0;
pub static mut r32: usize = 0;
    if (copy_from_user(&r32, rlim, sizeof!(compat_rlimit))) {
    return -EFAULT;
    }
    if (r32.rlim_cur == COMPAT_RLIM_INFINITY) {
    r.rlim_cur = RLIM_INFINITY;
    }
    else {
    r.rlim_cur = r32.rlim_cur;
    }
    if (r32.rlim_max == COMPAT_RLIM_INFINITY) {
    r.rlim_max = RLIM_INFINITY;
    }
    else {
    r.rlim_max = r32.rlim_max;
    }
    return do_prlimit(current, resource, &r, core::ptr::null_mut());
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_getrlimit
pub unsafe extern "C" fn sys_getrlimit_dup(resource: usize, rlim: usize) -> c_long {
pub static mut r: usize = 0;
    let mut ret = 0;
    ret = do_prlimit(current, resource, core::ptr::null_mut(), &r);
    if (!ret) {
pub static mut r32: usize = 0;
    if (r.rlim_cur > COMPAT_RLIM_INFINITY) {
    r32.rlim_cur = COMPAT_RLIM_INFINITY;
    }
    else {
    r32.rlim_cur = r.rlim_cur;
    }
    if (r.rlim_max > COMPAT_RLIM_INFINITY) {
    r32.rlim_max = COMPAT_RLIM_INFINITY;
    }
    else {
    r32.rlim_max = r.rlim_max;
    }
    if (copy_to_user(rlim, &r32, sizeof!(compat_rlimit))) {
    return -EFAULT;
    }
    }
    return ret;
    }

//
// Back compatibility for getrlimit. Needed for some apps.
//
#[no_mangle]
pub unsafe extern "C" fn sys_old_getrlimit(resource: usize, rlim: usize) -> c_long {
pub static mut x: usize = 0;
    if (resource >= RLIM_NLIMITS) {
    return -EINVAL;
    }
    resource = array_index_nospec(resource, RLIM_NLIMITS);
    task_lock(current.group_leader);
    x = current.signal.rlim[resource];
    task_unlock(current.group_leader);
    if (x.rlim_cur > 0x7FFFFFFF) {
    x.rlim_cur = 0x7FFFFFFF;
    }
    if (x.rlim_max > 0x7FFFFFFF) {
    x.rlim_max = 0x7FFFFFFF;
    }
    return copy_to_user(rlim, &x, sizeof!(x)) ? -EFAULT : 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: sys_old_getrlimit
pub unsafe extern "C" fn sys_old_getrlimit_dup(resource: usize, rlim: usize) -> c_long {
pub static mut r: usize = 0;
    if (resource >= RLIM_NLIMITS) {
    return -EINVAL;
    }
    resource = array_index_nospec(resource, RLIM_NLIMITS);
    task_lock(current.group_leader);
    r = current.signal.rlim[resource];
    task_unlock(current.group_leader);
    if (r.rlim_cur > 0x7FFFFFFF) {
    r.rlim_cur = 0x7FFFFFFF;
    }
    if (r.rlim_max > 0x7FFFFFFF) {
    r.rlim_max = 0x7FFFFFFF;
    }
    if (put_user(r.rlim_cur, &rlim.rlim_cur) ||
    put_user(r.rlim_max, &rlim.rlim_max)) {
    return -EFAULT;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn rlim64_is_infinity(rlim64: __u64) -> bool {

    return rlim64 >= ULONG_MAX;

pub static mut rlim64: return = 0;

    }
#[no_mangle]
unsafe extern "C" fn rlim_to_rlim64(rlim: *const rlimit, rlim64: *mut rlimit64) {
    if (rlim.rlim_cur == RLIM_INFINITY) {
    rlim64.rlim_cur = RLIM64_INFINITY;
    }
    else {
    rlim64.rlim_cur = rlim.rlim_cur;
    }
    if (rlim.rlim_max == RLIM_INFINITY) {
    rlim64.rlim_max = RLIM64_INFINITY;
    }
    else {
    rlim64.rlim_max = rlim.rlim_max;
    }
    }
#[no_mangle]
unsafe extern "C" fn rlim64_to_rlim(rlim64: *const rlimit64, rlim: *mut rlimit) {
    if (rlim64_is_infinity(rlim64.rlim_cur)) {
    rlim.rlim_cur = RLIM_INFINITY;
    }
    else {
    rlim.rlim_cur = (unsigned long)rlim64.rlim_cur;
    }
    if (rlim64_is_infinity(rlim64.rlim_max)) {
    rlim.rlim_max = RLIM_INFINITY;
    }
    else {
    rlim.rlim_max = (unsigned long)rlim64.rlim_max;
    }
    }
// rcu lock must be held
#[no_mangle]
pub unsafe extern "C" fn check_prlimit_permission(task: *mut task_struct, flags: c_uint) -> c_int {
    let mut cred = current_cred(), *tcred;
    let mut id_match = 0;
    if (current == task) {
    return 0;
    }
    tcred = __task_cred(task);
    id_match = (uid_eq(cred.uid, tcred.euid) &&
    uid_eq(cred.uid, tcred.suid) &&
    uid_eq(cred.uid, tcred.uid)  &&
    gid_eq(cred.gid, tcred.egid) &&
    gid_eq(cred.gid, tcred.sgid) &&
    gid_eq(cred.gid, tcred.gid));
    if (!id_match && !ns_capable(tcred.user_ns, CAP_SYS_RESOURCE)) {
    return -EPERM;
    }
    return security_task_prlimit(cred, tcred, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_prlimit64(pid: usize, resource: usize, new_rlim: usize, old_rlim: usize) -> c_long {
    struct rlimit64 old64, new64;
    struct rlimit old, new;
pub static mut tsk: *mut c_void = core::ptr::null_mut();
pub static mut checkflags: c_uint = 0;
    let mut need_tasklist = 0;
    let mut ret = 0;
    if (old_rlim) {
    checkflags |= LSM_PRLIMIT_READ;
    }
    if (new_rlim) {
    if (copy_from_user(&new64, new_rlim, sizeof!(new64))) {
    return -EFAULT;
    }
    rlim64_to_rlim(&new64, &new);
    checkflags |= LSM_PRLIMIT_WRITE;
    }
    rcu_read_lock();
    tsk = pid ? find_task_by_vpid(pid) : current;
    if (!tsk) {
    rcu_read_unlock();
    return -ESRCH;
    }
    ret = check_prlimit_permission(tsk, checkflags);
    if (ret) {
    rcu_read_unlock();
    return ret;
    }
    get_task_struct(tsk);
    rcu_read_unlock();
    need_tasklist = !same_thread_group(tsk, current);
    if (need_tasklist) {
//
// Ensure we can't race with group exit or de_thread(),
// so tsk->group_leader can't be freed or changed until
// read_unlock(tasklist_lock) below.
//
    read_lock(&tasklist_lock);
    if (!pid_alive(tsk)) {
    ret = -ESRCH;
    }
    }
    if (!ret) {
    ret = do_prlimit(tsk, resource, new_rlim ? &new : core::ptr::null_mut(),
    old_rlim ? &old : core::ptr::null_mut());
    }
    if (need_tasklist) {
    read_unlock(&tasklist_lock);
    }
    if (!ret && old_rlim) {
    rlim_to_rlim64(&old, &old64);
    if (copy_to_user(old_rlim, &old64, sizeof!(old64))) {
    ret = -EFAULT;
    }
    }
    put_task_struct(tsk);
    return ret;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_setrlimit
pub unsafe extern "C" fn sys_setrlimit_dup(resource: usize, rlim: usize) -> c_long {
pub static mut new_rlim: usize = 0;
    if (copy_from_user(&new_rlim, rlim, sizeof!(*rlim))) {
    return -EFAULT;
    }
    return do_prlimit(current, resource, &new_rlim, core::ptr::null_mut());
    }
//
// It would make sense to put struct rusage in the task_struct,
// except that would make the task_struct be *really big*.  After
// task_struct gets moved into malloc'ed memory, it would
// make sense to do this.  It will make moving the rest of the information
// a lot simpler!  (Which we're not doing right now because we're not
// measuring them yet).
//
// When sampling multiple threads for RUSAGE_SELF, under SMP we might have
// races with threads incrementing their own counters.  But since word
// reads are atomic, we either get new values or old values and we don't
// care which for the sums.  We always take the siglock to protect reading
// the c* fields from p->signal from races with exit.c updating those
// fields when reaping, so a sample either gets all the additions of a
// given child after it's reaped, or none so this sample is before reaping.
//
// Locking:
// We need to take the siglock for CHILDEREN, SELF and BOTH
// for  the cases current multithreaded, non-current single threaded
// non-current multithreaded.  Thread traversal is now safe with
// the siglock held.
// Strictly speaking, we donot need to take the siglock if we are current and
// single threaded,  as no one else can take our signal_struct away, no one
// else can  reap the  children to update signal->c* counters, and no one else
// can race with the signal-> fields. If we do not take any lock, the
// signal-> fields could be read out of order while another thread was just
// exiting. So we should  place a read memory barrier when we avoid the lock.
// On the writer side,  write memory barrier is implied in  __exit_signal
// as __exit_signal releases  the siglock spinlock after updating the signal->
// fields. But we don't do this yet to keep things simple.
//
#[no_mangle]
unsafe extern "C" fn accumulate_thread_rusage(t: *mut task_struct, r: *mut rusage) {
    r.ru_nvcsw += t.nvcsw;
    r.ru_nivcsw += t.nivcsw;
    r.ru_minflt += t.min_flt;
    r.ru_majflt += t.maj_flt;
    r.ru_inblock += task_io_get_inblock(t);
    r.ru_oublock += task_io_get_oublock(t);
    }
#[no_mangle]
pub unsafe extern "C" fn getrusage(p: *mut task_struct, who: c_int, r: *mut rusage) {
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    u64 tgutime, tgstime, utime, stime;
    let mut maxrss = 0;
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut sig = p.signal;
pub static mut seq: c_uint = 0;
// label;
    memset(r, 0, sizeof!(*r));
    utime = stime = 0;
    maxrss = 0;
    if (who == RUSAGE_THREAD) {
    task_cputime_adjusted(current, &utime, &stime);
    accumulate_thread_rusage(p, r);
    maxrss = sig.maxrss;
// goto;
    }
    flags = read_seqbegin_or_lock_irqsave(&sig.stats_lock, &seq);
    match (who) {
    RUSAGE_BOTH => {
    }
    RUSAGE_CHILDREN => {
    utime = sig.cutime;
    stime = sig.cstime;
    r.ru_nvcsw = sig.cnvcsw;
    r.ru_nivcsw = sig.cnivcsw;
    r.ru_minflt = sig.cmin_flt;
    r.ru_majflt = sig.cmaj_flt;
    r.ru_inblock = sig.cinblock;
    r.ru_oublock = sig.coublock;
    maxrss = sig.cmaxrss;
    if (who == RUSAGE_CHILDREN) {
    // break;
    }
    fallthrough;
    }
    RUSAGE_SELF => {
    r.ru_nvcsw += sig.nvcsw;
    r.ru_nivcsw += sig.nivcsw;
    r.ru_minflt += sig.min_flt;
    r.ru_majflt += sig.maj_flt;
    r.ru_inblock += sig.inblock;
    r.ru_oublock += sig.oublock;
    if (maxrss < sig.maxrss) {
    maxrss = sig.maxrss;
    }
    rcu_read_lock();
    __for_each_thread(sig, t)
    accumulate_thread_rusage(t, r);
    rcu_read_unlock();
    // break;
    }
    _ => {
    BUG();
    }
    }
    if (need_seqretry(&sig.stats_lock, seq)) {
    seq = 1;
// goto;
    }
    done_seqretry_irqrestore(&sig.stats_lock, seq, flags);
    if (who == RUSAGE_CHILDREN) {
// goto;
    }
    thread_group_cputime_adjusted(p, &tgutime, &tgstime);
    utime += tgutime;
    stime += tgstime;
// label;
    mm = get_task_mm(p);
    if (mm) {
    setmax_mm_hiwater_rss(&maxrss, mm);
    mmput(mm);
    }
// label;
    r.ru_maxrss = maxrss * (PAGE_SIZE / 1024); /* convert pages to KBs */
    r.ru_utime = ns_to_kernel_old_timeval(utime);
    r.ru_stime = ns_to_kernel_old_timeval(stime);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getrusage(who: usize, ru: usize) -> c_long {
pub static mut r: usize = 0;
    if (who != RUSAGE_SELF && who != RUSAGE_CHILDREN &&
    who != RUSAGE_THREAD) {
    return -EINVAL;
    }
    getrusage(current, who, &r);
    return copy_to_user(ru, &r, sizeof!(r)) ? -EFAULT : 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: sys_getrusage
pub unsafe extern "C" fn sys_getrusage_dup(who: usize, ru: usize) -> c_long {
pub static mut r: usize = 0;
    if (who != RUSAGE_SELF && who != RUSAGE_CHILDREN &&
    who != RUSAGE_THREAD) {
    return -EINVAL;
    }
    getrusage(current, who, &r);
    return put_compat_rusage(&r, ru);
    }

#[no_mangle]
pub unsafe extern "C" fn sys_umask(mask: usize) -> c_long {
    mask = xchg(&current.fs.umask, mask & S_IRWXUGO);
    return mask;
    }
#[no_mangle]
unsafe extern "C" fn prctl_set_mm_exe_file(mm: *mut mm_struct, fd: c_uint) -> c_int {
    CLASS(fd, exe)(fd);
pub static mut inode: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (fd_empty(exe)) {
    return -EBADF;
    }
    inode = file_inode(fd_file(exe));
//
// Because the original mm->exe_file points to executable file, make
// sure that this one is executable as well, to avoid breaking an
// overall picture.
//
    if (!S_ISREG(inode.i_mode) || path_noexec(&fd_file(exe).f_path)) {
    return -EACCES;
    }
    err = file_permission(fd_file(exe), MAY_EXEC);
    if (err) {
    return err;
    }
    return replace_mm_exe_file(mm, fd_file(exe));
    }
//
// Check arithmetic relations of passed addresses.
//
// WARNING: we don't require any capability here so be very careful
// in what is allowed for modification from userspace.
//
#[no_mangle]
unsafe extern "C" fn validate_prctl_map_addr(prctl_map: *mut prctl_mm_map) -> c_int {
pub static mut mmap_max_addr: c_ulong = 0;
pub static mut error: c_int = 0;
    static const unsigned char offsets[] = {
    offsetof(prctl_mm_map, start_code),
    offsetof(prctl_mm_map, end_code),
    offsetof(prctl_mm_map, start_data),
    offsetof(prctl_mm_map, end_data),
    offsetof(prctl_mm_map, start_brk),
    offsetof(prctl_mm_map, brk),
    offsetof(prctl_mm_map, start_stack),
    offsetof(prctl_mm_map, arg_start),
    offsetof(prctl_mm_map, arg_end),
    offsetof(prctl_mm_map, env_start),
    offsetof(prctl_mm_map, env_end),
    };
//
// Make sure the members are not somewhere outside
// of allowed address space.
//
    while (i < ARRAY_SIZE!(offsets)) {
pub static mut val: u64 = 0;
    if ((unsigned long)val >= mmap_max_addr ||
    (unsigned long)val < mmap_min_addr) {
// goto;
    }
    }
//
// Make sure the pairs are ordered.
//

    ((unsigned long)prctl_map.__m1 __op				
    (unsigned long)prctl_map.__m2) ? 0 : -EINVAL
    error  = __prctl_check_order(start_code, <, end_code);
    error |= __prctl_check_order(start_data,<=, end_data);
    error |= __prctl_check_order(start_brk, <=, brk);
    error |= __prctl_check_order(arg_start, <=, arg_end);
    error |= __prctl_check_order(env_start, <=, env_end);
    if (error) {
// goto;
    }

    error = -EINVAL;
//
// Neither we should allow to override limits if they set.
//
    if (check_data_rlimit(rlimit(RLIMIT_DATA), prctl_map.brk,
    prctl_map.start_brk, prctl_map.end_data,
    prctl_map.start_data)) {
// goto;
    }
    error = 0;
// label;
    return error;
    }

#[no_mangle]
unsafe extern "C" fn prctl_set_mm_map(opt: c_int, addr: *const c_void , data_size: c_ulong) -> c_int {
pub static mut prctl_map: prctl_mm_map = 0;
    unsigned long user_auxv[AT_VECTOR_SIZE];
    let mut mm = current.mm;
    let mut error = 0;
    BUILD_BUG_ON!(sizeof!(user_auxv) != sizeof!(mm.saved_auxv));
    BUILD_BUG_ON!(sizeof!(prctl_mm_map) > 256);
    if (opt == PR_SET_MM_MAP_SIZE) {
    return put_user((unsigned int)sizeof!(prctl_map),
    addr);
    }
    if (data_size != sizeof!(prctl_map)) {
    return -EINVAL;
    }
    if (copy_from_user(&prctl_map, addr, sizeof!(prctl_map))) {
    return -EFAULT;
    }
    error = validate_prctl_map_addr(&prctl_map);
    if (error) {
    return error;
    }
    if (prctl_map.auxv_size) {
//
// Someone is trying to cheat the auxv vector.
//
    if (!prctl_map.auxv ||
    prctl_map.auxv_size > sizeof!(mm.saved_auxv)) {
    return -EINVAL;
    }
    memset(user_auxv, 0, sizeof!(user_auxv));
    if (copy_from_user(user_auxv,
    prctl_map.auxv,
    prctl_map.auxv_size)) {
    return -EFAULT;
    }
// Last entry must be AT_NULL as specification requires
    user_auxv[AT_VECTOR_SIZE - 2] = AT_NULL;
    user_auxv[AT_VECTOR_SIZE - 1] = AT_NULL;
    }
    if (prctl_map.exe_fd != (u32)-1) {
//
// Check if the current user is checkpoint/restore capable.
// At the time of this writing, it checks for CAP_SYS_ADMIN
// or CAP_CHECKPOINT_RESTORE.
// Note that a user with access to ptrace can masquerade an
// arbitrary program as any executable, even setuid ones.
// This may have implications in the tomoyo subsystem.
//
    if (!checkpoint_restore_ns_capable(current_user_ns())) {
    return -EPERM;
    }
    error = prctl_set_mm_exe_file(mm, prctl_map.exe_fd);
    if (error) {
    return error;
    }
    }
//
// arg_lock protects concurrent updates but we still need mmap_lock for
// read to exclude races with sys_brk.
//
    mmap_read_lock(mm);
//
// We don't validate if these members are pointing to
// real present VMAs because application may have correspond
// VMAs already unmapped and kernel uses these members for statistics
// output in procfs mostly, except
//
// - @start_brk/@brk which are used in do_brk_flags but kernel lookups
// for VMAs when updating these members so anything wrong written
// here cause kernel to swear at userspace program but won't lead
// to any problem in kernel itself
//
    spin_lock(&mm.arg_lock);
    mm.start_code	= prctl_map.start_code;
    mm.end_code	= prctl_map.end_code;
    mm.start_data	= prctl_map.start_data;
    mm.end_data	= prctl_map.end_data;
    mm.start_brk	= prctl_map.start_brk;
    mm.brk		= prctl_map.brk;
    mm.start_stack	= prctl_map.start_stack;
    mm.arg_start	= prctl_map.arg_start;
    mm.arg_end	= prctl_map.arg_end;
    mm.env_start	= prctl_map.env_start;
    mm.env_end	= prctl_map.env_end;
    spin_unlock(&mm.arg_lock);
//
// Note this update of @saved_auxv is lockless thus
// if someone reads this member in procfs while we're
// updating -- it may get partly updated results. It's
// known and acceptable trade off: we leave it as is to
// not introduce additional locks here making the kernel
// more complex.
//
    if (prctl_map.auxv_size) {
    memcpy(mm.saved_auxv, user_auxv, sizeof!(user_auxv));
    }
    mmap_read_unlock(mm);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn prctl_set_auxv(mm: *mut mm_struct, addr: c_ulong, len: c_ulong) -> c_int {
//
// This doesn't move the auxiliary vector itself since it's pinned to
// mm_struct, but it permits filling the vector with new values.  It's
// up to the caller to provide sane values here, otherwise userspace
// tools which use this vector might be unhappy.
//
    unsigned long user_auxv[AT_VECTOR_SIZE] = {};
    if (len > sizeof!(user_auxv)) {
    return -EINVAL;
    }
    if (copy_from_user(user_auxv, addr, len)) {
    return -EFAULT;
    }
// Make sure the last entry is always AT_NULL
    user_auxv[AT_VECTOR_SIZE - 2] = 0;
    user_auxv[AT_VECTOR_SIZE - 1] = 0;
    BUILD_BUG_ON!(sizeof!(user_auxv) != sizeof!(mm.saved_auxv));
    task_lock(current);
    memcpy(mm.saved_auxv, user_auxv, sizeof!(user_auxv));
    task_unlock(current);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn prctl_set_mm(opt: c_int, addr: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int {
    let mut mm = current.mm;
pub static mut prctl_mm_map: usize = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    if (arg5 || (arg4 && (opt != PR_SET_MM_AUXV &&
    opt != PR_SET_MM_MAP &&
    opt != PR_SET_MM_MAP_SIZE))) {
    return -EINVAL;
    }

    if (opt == PR_SET_MM_MAP || opt == PR_SET_MM_MAP_SIZE) {
    return prctl_set_mm_map(opt, addr, arg4);
    }

    if (!capable(CAP_SYS_RESOURCE)) {
    return -EPERM;
    }
    if (opt == PR_SET_MM_EXE_FILE) {
    return prctl_set_mm_exe_file(mm, (unsigned int)addr);
    }
    if (opt == PR_SET_MM_AUXV) {
    return prctl_set_auxv(mm, addr, arg4);
    }
    if (addr >= TASK_SIZE || addr < mmap_min_addr) {
    return -EINVAL;
    }
    error = -EINVAL;
//
// arg_lock protects concurrent updates of arg boundaries, we need
// mmap_lock for a) concurrent sys_brk, b) finding VMA for addr
// validation.
//
    mmap_read_lock(mm);
    vma = find_vma(mm, addr);
    spin_lock(&mm.arg_lock);
    prctl_map.start_code	= mm.start_code;
    prctl_map.end_code	= mm.end_code;
    prctl_map.start_data	= mm.start_data;
    prctl_map.end_data	= mm.end_data;
    prctl_map.start_brk	= mm.start_brk;
    prctl_map.brk		= mm.brk;
    prctl_map.start_stack	= mm.start_stack;
    prctl_map.arg_start	= mm.arg_start;
    prctl_map.arg_end	= mm.arg_end;
    prctl_map.env_start	= mm.env_start;
    prctl_map.env_end	= mm.env_end;
    match (opt) {
    PR_SET_MM_START_CODE => {
    prctl_map.start_code = addr;
    // break;
    }
    PR_SET_MM_END_CODE => {
    prctl_map.end_code = addr;
    // break;
    }
    PR_SET_MM_START_DATA => {
    prctl_map.start_data = addr;
    // break;
    }
    PR_SET_MM_END_DATA => {
    prctl_map.end_data = addr;
    // break;
    }
    PR_SET_MM_START_STACK => {
    prctl_map.start_stack = addr;
    // break;
    }
    PR_SET_MM_START_BRK => {
    prctl_map.start_brk = addr;
    // break;
    }
    PR_SET_MM_BRK => {
    prctl_map.brk = addr;
    // break;
    }
    PR_SET_MM_ARG_START => {
    prctl_map.arg_start = addr;
    // break;
    }
    PR_SET_MM_ARG_END => {
    prctl_map.arg_end = addr;
    // break;
    }
    PR_SET_MM_ENV_START => {
    prctl_map.env_start = addr;
    // break;
    }
    PR_SET_MM_ENV_END => {
    prctl_map.env_end = addr;
    // break;
    }
    _ => {
// goto;
    }
    }
    error = validate_prctl_map_addr(&prctl_map);
    if (error) {
// goto;
    }
    match (opt) {
//
// If command line arguments and environment
// are placed somewhere else on stack, we can
// set them up here, ARG_START/END to setup
// command line arguments and ENV_START/END
// for environment.
//
    PR_SET_MM_START_STACK => {
    }
    PR_SET_MM_ARG_START => {
    }
    PR_SET_MM_ARG_END => {
    }
    PR_SET_MM_ENV_START => {
    }
    PR_SET_MM_ENV_END => {
    if (!vma) {
    error = -EFAULT;
// goto;
    }
    }
    }
    mm.start_code	= prctl_map.start_code;
    mm.end_code	= prctl_map.end_code;
    mm.start_data	= prctl_map.start_data;
    mm.end_data	= prctl_map.end_data;
    mm.start_brk	= prctl_map.start_brk;
    mm.brk		= prctl_map.brk;
    mm.start_stack	= prctl_map.start_stack;
    mm.arg_start	= prctl_map.arg_start;
    mm.arg_end	= prctl_map.arg_end;
    mm.env_start	= prctl_map.env_start;
    mm.env_end	= prctl_map.env_end;
    error = 0;
// label;
    spin_unlock(&mm.arg_lock);
    mmap_read_unlock(mm);
    return error;
    }

#[no_mangle]
unsafe extern "C" fn prctl_get_tid_address(me: *mut task_struct, tid_addr: *mut *mut int   ) -> c_int {
    return put_user(me.clear_child_tid, tid_addr);
    }

#[no_mangle]
unsafe extern "C" fn prctl_get_tid_address(me: *mut task_struct, tid_addr: *mut *mut int   ) -> c_int {
    return -EINVAL;
    }

#[no_mangle]
unsafe extern "C" fn propagate_has_child_subreaper(p: *mut task_struct, data: *mut c_void) -> c_int {
//
// If task has has_child_subreaper - all its descendants
// already have these flag too and new descendants will
// inherit it on fork, skip them.
//
// If we've found child_reaper - skip descendants in
// it's subtree as they will never get out pidns.
//
    if (p.signal.has_child_subreaper ||
    is_child_reaper(task_pid(p))) {
    return 0;
    }
    p.signal.has_child_subreaper = 1;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prctl_spec_ctrl_get(t: *mut task_struct, which: c_ulong) -> int __weak {
    return -EINVAL;
    }
    int __weak arch_prctl_spec_ctrl_set(task_struct *t, unsigned long which,
    unsigned long ctrl)
    {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_get_shadow_stack_status(t: *mut task_struct, status: *mut unsigned long ) -> int __weak {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_set_shadow_stack_status(t: *mut task_struct, status: c_ulong) -> int __weak {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_lock_shadow_stack_status(t: *mut task_struct, status: c_ulong) -> int __weak {
    return -EINVAL;
    }
    int __weak arch_prctl_get_branch_landing_pad_state(task_struct *t,
    unsigned long  *state)
    {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prctl_set_branch_landing_pad_state(t: *mut task_struct, state: c_ulong) -> int __weak {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prctl_lock_branch_landing_pad_state(t: *mut task_struct) -> int __weak {
    return -EINVAL;
    }

#[no_mangle]
pub unsafe extern "C" fn prctl_set_vma(opt: c_ulong, addr: c_ulong, size: c_ulong, arg: c_ulong) -> c_int {
    let mut error = 0;
    match (opt) {
    PR_SET_VMA_ANON_NAME => {
    error = set_anon_vma_name(addr, size, arg);
    // break;
    }
    _ => {
    error = -EINVAL;
    }
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn get_current_mdwe() -> c_ulong {
pub static mut ret: c_ulong = 0;
    if (mm_flags_test(MMF_HAS_MDWE, current.mm)) {
    ret |= PR_MDWE_REFUSE_EXEC_GAIN;
    }
    if (mm_flags_test(MMF_HAS_MDWE_NO_INHERIT, current.mm)) {
    ret |= PR_MDWE_NO_INHERIT;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn prctl_set_mdwe(bits: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int {
    let mut current_bits = 0;
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    if (bits & ~(PR_MDWE_REFUSE_EXEC_GAIN | PR_MDWE_NO_INHERIT)) {
    return -EINVAL;
    }
// NO_INHERIT only makes sense with REFUSE_EXEC_GAIN
    if (bits & PR_MDWE_NO_INHERIT && !(bits & PR_MDWE_REFUSE_EXEC_GAIN)) {
    return -EINVAL;
    }
//
// EOPNOTSUPP might be more appropriate here in principle, but
// existing userspace depends on EINVAL specifically.
//
    if (!arch_memory_deny_write_exec_supported()) {
    return -EINVAL;
    }
    current_bits = get_current_mdwe();
    if (current_bits && current_bits != bits) {
    return -EPERM; /* Cannot unset the flags */
    }
    if (bits & PR_MDWE_NO_INHERIT) {
    mm_flags_set(MMF_HAS_MDWE_NO_INHERIT, current.mm);
    }
    if (bits & PR_MDWE_REFUSE_EXEC_GAIN) {
    mm_flags_set(MMF_HAS_MDWE, current.mm);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn prctl_get_mdwe(arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int {
    if (arg2 || arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    return get_current_mdwe();
    }
#[no_mangle]
unsafe extern "C" fn prctl_get_auxv(addr: *mut c_void , len: c_ulong) -> c_int {
    let mut mm = current.mm;
pub static mut size: c_ulong = 0;
    if (size && copy_to_user(addr, mm.saved_auxv, size)) {
    return -EFAULT;
    }
    return sizeof!(mm.saved_auxv);
    }
#[no_mangle]
pub unsafe extern "C" fn prctl_get_thp_disable(arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int {
    let mut mm = current.mm;
    if (arg2 || arg3 || arg4 || arg5) {
    return -EINVAL;
    }
// If disabled, we return "1 | flags", otherwise 0.
    if (mm_flags_test(MMF_DISABLE_THP_COMPLETELY, mm)) {
    return 1;
    }

    else if (mm_flags_test(MMF_DISABLE_THP_EXCEPT_ADVISED, mm)) {
    return 1 | PR_THP_DISABLE_EXCEPT_ADVISED;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn prctl_set_thp_disable(thp_disable: bool, flags: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int {
    let mut mm = current.mm;
    if (arg4 || arg5) {
    return -EINVAL;
    }
// Flags are only allowed when disabling.
    if ((!thp_disable && flags) || (flags & ~PR_THP_DISABLE_EXCEPT_ADVISED)) {
    return -EINVAL;
    }
    if (mmap_write_lock_killable(current.mm)) {
    return -EINTR;
    }
    if (thp_disable) {
    if (flags & PR_THP_DISABLE_EXCEPT_ADVISED) {
    mm_flags_clear(MMF_DISABLE_THP_COMPLETELY, mm);
    mm_flags_set(MMF_DISABLE_THP_EXCEPT_ADVISED, mm);
    } else {
    mm_flags_set(MMF_DISABLE_THP_COMPLETELY, mm);
    mm_flags_clear(MMF_DISABLE_THP_EXCEPT_ADVISED, mm);
    }
    } else {
    mm_flags_clear(MMF_DISABLE_THP_COMPLETELY, mm);
    mm_flags_clear(MMF_DISABLE_THP_EXCEPT_ADVISED, mm);
    }
    mmap_write_unlock(current.mm);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_prctl(option: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> c_long {
    let mut me = current;
    unsigned char comm[sizeof!(me.comm)];
    let mut error = 0;
    error = security_task_prctl(option, arg2, arg3, arg4, arg5);
    if (error != -ENOSYS) {
    return error;
    }
    error = 0;
    match (option) {
    PR_SET_PDEATHSIG => {
    if (!valid_signal(arg2)) {
    error = -EINVAL;
    // break;
    }
//
// Ensure that either:
//
// 1. Subsequent getppid() calls reflect the parent process having died.
// 2. forget_original_parent() will send the new me->pdeath_signal.
//
// Also prevent the read of me->pdeath_signal from being a data race.
//
    read_lock(&tasklist_lock);
    me.pdeath_signal = arg2;
    read_unlock(&tasklist_lock);
    // break;
    }
    PR_GET_PDEATHSIG => {
    error = put_user(me.pdeath_signal, arg2);
    // break;
    }
    PR_GET_DUMPABLE => {
    error = task_exec_state_get_dumpable(me);
    // break;
    }
    PR_SET_DUMPABLE => {
    if (arg2 != TASK_DUMPABLE_OFF && arg2 != TASK_DUMPABLE_OWNER) {
    error = -EINVAL;
    // break;
    }
    task_exec_state_set_dumpable(arg2);
    // break;
    }
    PR_SET_UNALIGN => {
    error = SET_UNALIGN_CTL(me, arg2);
    // break;
    }
    PR_GET_UNALIGN => {
    error = GET_UNALIGN_CTL(me, arg2);
    // break;
    }
    PR_SET_FPEMU => {
    error = SET_FPEMU_CTL(me, arg2);
    // break;
    }
    PR_GET_FPEMU => {
    error = GET_FPEMU_CTL(me, arg2);
    // break;
    }
    PR_SET_FPEXC => {
    error = SET_FPEXC_CTL(me, arg2);
    // break;
    }
    PR_GET_FPEXC => {
    error = GET_FPEXC_CTL(me, arg2);
    // break;
    }
    PR_GET_TIMING => {
    error = PR_TIMING_STATISTICAL;
    // break;
    }
    PR_SET_TIMING => {
    if (arg2 != PR_TIMING_STATISTICAL) {
    error = -EINVAL;
    }
    // break;
    }
    PR_SET_NAME => {
    comm[sizeof!(me.comm) - 1] = 0;
    if (strncpy_from_user(comm, arg2,
    sizeof!(me.comm) - 1) < 0) {
    return -EFAULT;
    }
    set_task_comm(me, comm);
    proc_comm_connector(me);
    // break;
    }
    PR_GET_NAME => {
    get_task_comm(comm, me);
    if (copy_to_user(arg2, comm, sizeof!(comm))) {
    return -EFAULT;
    }
    // break;
    }
    PR_GET_ENDIAN => {
    error = GET_ENDIAN(me, arg2);
    // break;
    }
    PR_SET_ENDIAN => {
    error = SET_ENDIAN(me, arg2);
    // break;
    }
    PR_GET_SECCOMP => {
    error = prctl_get_seccomp();
    // break;
    }
    PR_SET_SECCOMP => {
    error = prctl_set_seccomp(arg2, arg3);
    // break;
    }
    PR_GET_TSC => {
    error = GET_TSC_CTL(arg2);
    // break;
    }
    PR_SET_TSC => {
    error = SET_TSC_CTL(arg2);
    // break;
    }
    PR_TASK_PERF_EVENTS_DISABLE => {
    error = perf_event_task_disable();
    // break;
    }
    PR_TASK_PERF_EVENTS_ENABLE => {
    error = perf_event_task_enable();
    // break;
    }
    PR_GET_TIMERSLACK => {
    if (current.timer_slack_ns > ULONG_MAX) {
    error = ULONG_MAX;
    }
    else {
    error = current.timer_slack_ns;
    }
    // break;
    }
    PR_SET_TIMERSLACK => {
    if (rt_or_dl_task_policy(current)) {
    // break;
    }
    if (arg2 <= 0) {
    current.timer_slack_ns =
    current.default_timer_slack_ns;
    }
    else {
    current.timer_slack_ns = arg2;
    }
    // break;
    }
    PR_MCE_KILL => {
    if (arg4 | arg5) {
    return -EINVAL;
    }
    match (arg2) {
    PR_MCE_KILL_CLEAR => {
    if (arg3 != 0) {
    return -EINVAL;
    }
    current.flags &= ~PF_MCE_PROCESS;
    // break;
    }
    PR_MCE_KILL_SET => {
    current.flags |= PF_MCE_PROCESS;
    if (arg3 == PR_MCE_KILL_EARLY) {
    current.flags |= PF_MCE_EARLY;
    }

    else if (arg3 == PR_MCE_KILL_LATE) {
    current.flags &= ~PF_MCE_EARLY;
    }

    else if (arg3 == PR_MCE_KILL_DEFAULT) {
    current.flags &=
    ~(PF_MCE_EARLY|PF_MCE_PROCESS);
    }
    else {
    return -EINVAL;
    }
    // break;
// label;
    return -EINVAL;
    }
    // break;
    case PR_MCE_KILL_GET:
    if (arg2 | arg3 | arg4 | arg5) {
    return -EINVAL;
    }
    if (current.flags & PF_MCE_PROCESS) {
    error = (current.flags & PF_MCE_EARLY) ?
    PR_MCE_KILL_EARLY : PR_MCE_KILL_LATE;
    }
    else {
    error = PR_MCE_KILL_DEFAULT;
    }
    // break;
    case PR_SET_MM:
    error = prctl_set_mm(arg2, arg3, arg4, arg5);
    // break;
    case PR_GET_TID_ADDRESS:
    error = prctl_get_tid_address(me, (int  *  *)arg2);
    // break;
    case PR_SET_CHILD_SUBREAPER:
    me.signal.is_child_subreaper = !!arg2;
    if (!arg2) {
    // break;
    }
    walk_process_tree(me, propagate_has_child_subreaper, core::ptr::null_mut());
    // break;
    case PR_GET_CHILD_SUBREAPER:
    error = put_user(me.signal.is_child_subreaper,
    arg2);
    // break;
    case PR_SET_NO_NEW_PRIVS:
    if (arg2 != 1 || arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    task_set_no_new_privs(current);
    // break;
    case PR_GET_NO_NEW_PRIVS:
    if (arg2 || arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    return task_no_new_privs(current) ? 1 : 0;
    case PR_GET_THP_DISABLE:
    error = prctl_get_thp_disable(arg2, arg3, arg4, arg5);
    // break;
    case PR_SET_THP_DISABLE:
    error = prctl_set_thp_disable(arg2, arg3, arg4, arg5);
    // break;
    case PR_MPX_ENABLE_MANAGEMENT:
    case PR_MPX_DISABLE_MANAGEMENT:
// No longer implemented:
    return -EINVAL;
    case PR_SET_FP_MODE:
    error = SET_FP_MODE(me, arg2);
    // break;
    case PR_GET_FP_MODE:
    error = GET_FP_MODE(me);
    // break;
    case PR_SVE_SET_VL:
    error = SVE_SET_VL(arg2);
    // break;
    case PR_SVE_GET_VL:
    error = SVE_GET_VL();
    // break;
    case PR_SME_SET_VL:
    error = SME_SET_VL(arg2);
    // break;
    case PR_SME_GET_VL:
    error = SME_GET_VL();
    // break;
    case PR_GET_SPECULATION_CTRL:
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = arch_prctl_spec_ctrl_get(me, arg2);
    // break;
    case PR_SET_SPECULATION_CTRL:
    if (arg4 || arg5) {
    return -EINVAL;
    }
    error = arch_prctl_spec_ctrl_set(me, arg2, arg3);
    // break;
    case PR_PAC_RESET_KEYS:
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = PAC_RESET_KEYS(me, arg2);
    // break;
    case PR_PAC_SET_ENABLED_KEYS:
    if (arg4 || arg5) {
    return -EINVAL;
    }
    error = PAC_SET_ENABLED_KEYS(me, arg2, arg3);
    // break;
    case PR_PAC_GET_ENABLED_KEYS:
    if (arg2 || arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = PAC_GET_ENABLED_KEYS(me);
    // break;
    case PR_SET_TAGGED_ADDR_CTRL:
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = SET_TAGGED_ADDR_CTRL(arg2);
    // break;
    case PR_GET_TAGGED_ADDR_CTRL:
    if (arg2 || arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = GET_TAGGED_ADDR_CTRL();
    // break;
    case PR_SET_IO_FLUSHER:
    if (!capable(CAP_SYS_RESOURCE)) {
    return -EPERM;
    }
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    if (arg2 == 1) {
    current.flags |= PR_IO_FLUSHER;
    }

    else if (!arg2) {
    current.flags &= ~PR_IO_FLUSHER;
    }
    else {
    return -EINVAL;
    }
    // break;
    case PR_GET_IO_FLUSHER:
    if (!capable(CAP_SYS_RESOURCE)) {
    return -EPERM;
    }
    if (arg2 || arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = (current.flags & PR_IO_FLUSHER) == PR_IO_FLUSHER;
    // break;
    case PR_SET_SYSCALL_USER_DISPATCH:
    error = set_syscall_user_dispatch(arg2, arg3, arg4,
     arg5);
    // break;

    case PR_SCHED_CORE:
    error = sched_core_share_pid(arg2, arg3, arg4, arg5);
    // break;

    case PR_SET_MDWE:
    error = prctl_set_mdwe(arg2, arg3, arg4, arg5);
    // break;
    case PR_GET_MDWE:
    error = prctl_get_mdwe(arg2, arg3, arg4, arg5);
    // break;
    case PR_PPC_GET_DEXCR:
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = PPC_GET_DEXCR_ASPECT(me, arg2);
    // break;
    case PR_PPC_SET_DEXCR:
    if (arg4 || arg5) {
    return -EINVAL;
    }
    error = PPC_SET_DEXCR_ASPECT(me, arg2, arg3);
    // break;
    case PR_SET_VMA:
    error = prctl_set_vma(arg2, arg3, arg4, arg5);
    // break;
    case PR_GET_AUXV:
    if (arg4 || arg5) {
    return -EINVAL;
    }
    error = prctl_get_auxv(arg2, arg3);
    // break;

    case PR_SET_MEMORY_MERGE:
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    if (mmap_write_lock_killable(me.mm)) {
    return -EINTR;
    }
    if (arg2) {
    error = ksm_enable_merge_any(me.mm);
    }
    else {
    error = ksm_disable_merge_any(me.mm);
    }
    mmap_write_unlock(me.mm);
    // break;
    case PR_GET_MEMORY_MERGE:
    if (arg2 || arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = !!mm_flags_test(MMF_VM_MERGE_ANY, me.mm);
    // break;

    case PR_RISCV_V_SET_CONTROL:
    error = RISCV_V_SET_CONTROL(arg2);
    // break;
    case PR_RISCV_V_GET_CONTROL:
    error = RISCV_V_GET_CONTROL();
    // break;
    case PR_RISCV_SET_ICACHE_FLUSH_CTX:
    error = RISCV_SET_ICACHE_FLUSH_CTX(arg2, arg3);
    // break;
    case PR_GET_SHADOW_STACK_STATUS:
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = arch_get_shadow_stack_status(me,  arg2);
    // break;
    case PR_SET_SHADOW_STACK_STATUS:
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = arch_set_shadow_stack_status(me, arg2);
    // break;
    case PR_LOCK_SHADOW_STACK_STATUS:
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = arch_lock_shadow_stack_status(me, arg2);
    // break;
    case PR_TIMER_CREATE_RESTORE_IDS:
    if (arg3 || arg4 || arg5) {
    return -EINVAL;
    }
    error = posixtimer_create_prctl(arg2);
    // break;
    case PR_FUTEX_HASH:
    error = futex_hash_prctl(arg2, arg3, arg4);
    // break;
    case PR_RSEQ_SLICE_EXTENSION:
    if (arg4 || arg5) {
    return -EINVAL;
    }
    error = rseq_slice_extension_prctl(arg2, arg3);
    // break;
    case PR_GET_CFI:
    if (arg2 != PR_CFI_BRANCH_LANDING_PADS) {
    return -EINVAL;
    }
    if (arg4 || arg5) {
    return -EINVAL;
    }
    error = arch_prctl_get_branch_landing_pad_state(me, arg3);
    // break;
    case PR_SET_CFI:
    if (arg2 != PR_CFI_BRANCH_LANDING_PADS) {
    return -EINVAL;
    }
    if (arg4 || arg5) {
    return -EINVAL;
    }
    error = arch_prctl_set_branch_landing_pad_state(me, arg3);
    if (error) {
    // break;
    }
    if (arg3 & PR_CFI_LOCK && !(arg3 & PR_CFI_DISABLE)) {
    error = arch_prctl_lock_branch_landing_pad_state(me);
    }
    // break;
// label;
    trace_task_prctl_unknown(option, arg2, arg3, arg4, arg5);
    error = -EINVAL;
    // break;
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getcpu(cpup: usize, nodep: usize, unused: usize) -> c_long {
pub static mut err: c_int = 0;
pub static mut cpu: c_int = 0;
    if (cpup) {
    err |= put_user(cpu, cpup);
    }
    if (nodep) {
    err |= put_user(cpu_to_node(cpu), nodep);
    }
    return err ? -EFAULT : 0;
    }
//
// do_sysinfo - fill in sysinfo struct
// @info: pointer to buffer to fill
//
#[no_mangle]
unsafe extern "C" fn do_sysinfo(info: *mut sysinfo) -> c_int {
    unsigned long mem_total, sav_total;
    let mut mem_unit = 0;
    let mut bitcount = 0;
pub static mut tp: usize = 0;
    memset(info, 0, sizeof!(sysinfo));
    ktime_get_boottime_ts64(&tp);
    timens_add_boottime(&tp);
    info.uptime = tp.tv_sec + (tp.tv_nsec ? 1 : 0);
    get_avenrun(info.loads, 0, SI_LOAD_SHIFT - FSHIFT);
    info.procs = nr_threads;
    si_meminfo(info);
    si_swapinfo(info);
//
// If the sum of all the available memory (i.e. ram + swap)
// is less than can be stored in a 32 bit unsigned long then
// we can be binary compatible with 2.2.x kernels.  If not,
// well, in that case 2.2.x was broken anyways...
//
// -Erik Andersen <andersee@debian.org>
//
    mem_total = info.totalram + info.totalswap;
    if (mem_total < info.totalram || mem_total < info.totalswap) {
// goto;
    }
    bitcount = 0;
    mem_unit = info.mem_unit;
    while (mem_unit > 1) {
    bitcount += 1;
    mem_unit >>= 1;
    sav_total = mem_total;
    mem_total <<= 1;
    if (mem_total < sav_total) {
// goto;
    }
    }
//
// If mem_total did not overflow, multiply all memory values by
// info->mem_unit and set it to 1.  This leaves things compatible
// with 2.2.x, and also retains compatibility with earlier 2.4.x
// kernels...
//
    info.mem_unit = 1;
    info.totalram <<= bitcount;
    info.freeram <<= bitcount;
    info.sharedram <<= bitcount;
    info.bufferram <<= bitcount;
    info.totalswap <<= bitcount;
    info.freeswap <<= bitcount;
    info.totalhigh <<= bitcount;
    info.freehigh <<= bitcount;
// label;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_sysinfo(info: usize) -> c_long {
pub static mut val: usize = 0;
    do_sysinfo(&val);
    if (copy_to_user(info, &val, sizeof!(sysinfo))) {
    return -EFAULT;
    }
    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_sysinfo {
    pub uptime: i32,
    pub loads: [u32; 3],
    pub totalram: u32,
    pub freeram: u32,
    pub sharedram: u32,
    pub bufferram: u32,
    pub totalswap: u32,
    pub freeswap: u32,
    pub procs: u16,
    pub pad: u16,
    pub totalhigh: u32,
    pub freehigh: u32,
    pub mem_unit: u32,
    pub _f: [*mut c_char; 20-2*sizeof!(u32)-sizeof!(int)],
}

#[no_mangle]
#[no_mangle]
// duplicate fn: sys_sysinfo
pub unsafe extern "C" fn sys_sysinfo_dup(info: usize) -> c_long {
pub static mut s: usize = 0;
pub static mut s_32: usize = 0;
    do_sysinfo(&s);
// Check to see if any memory value is too large for 32-bit and scale
// down if needed
//
    if (upper_32_bits(s.totalram) || upper_32_bits(s.totalswap)) {
pub static mut bitcount: c_int = 0;
    while (s.mem_unit < PAGE_SIZE) {
    s.mem_unit <<= 1;
    bitcount += 1;
    }
    s.totalram >>= bitcount;
    s.freeram >>= bitcount;
    s.sharedram >>= bitcount;
    s.bufferram >>= bitcount;
    s.totalswap >>= bitcount;
    s.freeswap >>= bitcount;
    s.totalhigh >>= bitcount;
    s.freehigh >>= bitcount;
    }
    memset(&s_32, 0, sizeof!(s_32));
    s_32.uptime = s.uptime;
    s_32.loads[0] = s.loads[0];
    s_32.loads[1] = s.loads[1];
    s_32.loads[2] = s.loads[2];
    s_32.totalram = s.totalram;
    s_32.freeram = s.freeram;
    s_32.sharedram = s.sharedram;
    s_32.bufferram = s.bufferram;
    s_32.totalswap = s.totalswap;
    s_32.freeswap = s.freeswap;
    s_32.procs = s.procs;
    s_32.totalhigh = s.totalhigh;
    s_32.freehigh = s.freehigh;
    s_32.mem_unit = s.mem_unit;
    if (copy_to_user(info, &s_32, sizeof!(s_32))) {
    return -EFAULT;
    }
    return 0;
    }

}
}