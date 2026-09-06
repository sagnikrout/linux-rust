//! Automatically rewritten from C to Rust
//! Source: block/ioprio.c
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
// fs/ioprio.c
//
// Copyright (C) 2004 Jens Axboe <axboe@kernel.dk>
//
// Helper functions for setting/querying io priorities of processes. The
// system calls closely mimmick getpriority/setpriority, see the man page for
// those. The prio argument is a composite of prio class and prio data, where
// the data argument has meaning within that class. The standard scheduling
// classes have 8 distinct prio levels, with 0 being the highest prio and 7
// being the lowest.
//
// IOW, setting BE scheduling class with prio 2 is done ala:
//
// unsigned int prio = (IOPRIO_CLASS_BE << IOPRIO_CLASS_SHIFT) | 2;
//
// ioprio_set(PRIO_PROCESS, pid, prio);
//
// See also Documentation/block/ioprio.rst
//

#[no_mangle]
pub unsafe extern "C" fn ioprio_check_cap(ioprio: c_int) -> c_int {
pub static mut class: c_int = 0;
pub static mut level: c_int = 0;
    match (class) {
    IOPRIO_CLASS_RT => {
//
// Originally this only checked for CAP_SYS_ADMIN,
// which was implicitly allowed for pid 0 by security
// modules such as SELinux. Make sure we check
// CAP_SYS_ADMIN first to avoid a denial/avc for
// possibly missing CAP_SYS_NICE permission.
//
    if (!capable(CAP_SYS_ADMIN) && !capable(CAP_SYS_NICE)) {
    return -EPERM;
    }
    // break;
    }
    IOPRIO_CLASS_BE => {
    }
    IOPRIO_CLASS_IDLE => {
    // break;
    }
    IOPRIO_CLASS_NONE => {
    if (level) {
    return -EINVAL;
    }
    // break;
    }
    IOPRIO_CLASS_INVALID => {
    }
    _ => {
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_ioprio_set(which: usize, who: usize, ioprio: usize) -> c_long {
    let mut p = core::ptr::null_mut();
    let mut g = core::ptr::null_mut();
pub static mut user: *mut c_void = core::ptr::null_mut();
pub static mut pgrp: *mut c_void = core::ptr::null_mut();
    let mut uid;
    let mut ret = 0;
    ret = ioprio_check_cap(ioprio);
    if (ret) {
    return ret;
    }
    ret = -ESRCH;
    rcu_read_lock();
    match (which) {
    IOPRIO_WHO_PROCESS => {
    if (!who) {
    p = current;
    }
    else {
    p = find_task_by_vpid(who);
    }
    if (p) {
    ret = set_task_ioprio(p, ioprio);
    }
    // break;
    }
    IOPRIO_WHO_PGRP => {
    if (!who) {
    pgrp = task_pgrp(current);
    }
    else {
    pgrp = find_vpid(who);
    }
    read_lock(&tasklist_lock);
    do_each_pid_thread(pgrp, PIDTYPE_PGID, p) {
    ret = set_task_ioprio(p, ioprio);
    if (ret) {
    read_unlock(&tasklist_lock);
// goto;
    }
    } while_each_pid_thread(pgrp, PIDTYPE_PGID, p);
    read_unlock(&tasklist_lock);
    // break;
    }
    IOPRIO_WHO_USER => {
    uid = make_kuid(current_user_ns(), who);
    if (!uid_valid(uid)) {
    // break;
    }
    if (!who) {
    user = current_user();
    }
    else {
    user = find_user(uid);
    }
    if (!user) {
    // break;
    }
    for_each_process_thread(g, p) {
    if (!uid_eq(task_uid(p), uid) ||
    !task_pid_vnr(p)) {
    continue;
    }
    ret = set_task_ioprio(p, ioprio);
    if (ret) {
// goto;
    }
    }
// label;
    if (who) {
    free_uid(user);
    }
    // break;
    }
    _ => {
    ret = -EINVAL;
    }
    }
// label;
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_task_ioprio(p: *mut task_struct) -> c_int {
    let mut ret = 0;
    ret = security_task_getioprio(p);
    if (ret) {
// goto;
    }
    task_lock(p);
    ret = __get_task_ioprio(p);
    task_unlock(p);
// label;
    return ret;
    }
//
// Return raw IO priority value as set by userspace. We use this for
// ioprio_get(pid, IOPRIO_WHO_PROCESS) so that we keep historical behavior and
// also so that userspace can distinguish unset IO priority (which just gets
// overriden based on task's nice value) from IO priority set to some value.
//
#[no_mangle]
unsafe extern "C" fn get_task_raw_ioprio(p: *mut task_struct) -> c_int {
    let mut ret = 0;
    ret = security_task_getioprio(p);
    if (ret) {
// goto;
    }
    task_lock(p);
    if (p.io_context) {
    ret = p.io_context.ioprio;
    }
    else {
    ret = IOPRIO_DEFAULT;
    }
    task_unlock(p);
// label;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ioprio_best(aprio: c_ushort, bprio: c_ushort) -> c_int {
    return min(aprio, bprio);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_ioprio_get(which: usize, who: usize) -> c_long {
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut user: *mut c_void = core::ptr::null_mut();
pub static mut pgrp: *mut c_void = core::ptr::null_mut();
    let mut uid;
pub static mut ret: c_int = 0;
    let mut tmpio = 0;
    rcu_read_lock();
    match (which) {
    IOPRIO_WHO_PROCESS => {
    if (!who) {
    p = current;
    }
    else {
    p = find_task_by_vpid(who);
    }
    if (p) {
    ret = get_task_raw_ioprio(p);
    }
    // break;
    }
    IOPRIO_WHO_PGRP => {
    if (!who) {
    pgrp = task_pgrp(current);
    }
    else {
    pgrp = find_vpid(who);
    }
    read_lock(&tasklist_lock);
    do_each_pid_thread(pgrp, PIDTYPE_PGID, p) {
    tmpio = get_task_ioprio(p);
    if (tmpio < 0) {
    continue;
    }
    if (ret == -ESRCH) {
    ret = tmpio;
    }
    else {
    ret = ioprio_best(ret, tmpio);
    }
    } while_each_pid_thread(pgrp, PIDTYPE_PGID, p);
    read_unlock(&tasklist_lock);
    // break;
    }
    IOPRIO_WHO_USER => {
    uid = make_kuid(current_user_ns(), who);
    if (!who) {
    user = current_user();
    }
    else {
    user = find_user(uid);
    }
    if (!user) {
    // break;
    }
    for_each_process_thread(g, p) {
    if (!uid_eq(task_uid(p), user.uid) ||
    !task_pid_vnr(p)) {
    continue;
    }
    tmpio = get_task_ioprio(p);
    if (tmpio < 0) {
    continue;
    }
    if (ret == -ESRCH) {
    ret = tmpio;
    }
    else {
    ret = ioprio_best(ret, tmpio);
    }
    }
    if (who) {
    free_uid(user);
    }
    // break;
    }
    _ => {
    ret = -EINVAL;
    }
    }
    rcu_read_unlock();
    return ret;
    }