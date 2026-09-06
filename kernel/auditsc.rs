//! Automatically rewritten from C to Rust
//! Source: kernel/auditsc.c
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


























// SPDX-License-Identifier: GPL-2.0-or-later
// auditsc.c -- System-call auditing support
// Handles all system-call specific auditing features.
//
// Copyright 2003-2004 Red Hat Inc., Durham, North Carolina.
// Copyright 2005 Hewlett-Packard Development Company, L.P.
// Copyright (C) 2005, 2006 IBM Corporation
// All Rights Reserved.
//
// Written by Rickard E. (Rik) Faith <faith@redhat.com>
//
// Many of the ideas implemented here are from Stephen C. Tweedie,
// especially the idea of avoiding a copy by using getname.
//
// The method for actual interception of syscall entry and exit (not in
// this file -- see entry.S) is based on a GPL'd patch written by
// okir@suse.de and Copyright 2003 SuSE Linux AG.
//
// POSIX message queue support added by George Wilson <ltcgcw@us.ibm.com>,
// 2006.
//
// The support of additional filter rules compares (>, <, >=, <=) was
// added by Dustin Kirkland <dustin.kirkland@us.ibm.com>, 2005.
//
// Modified by Amy Griffis <amy.griffis@hp.com> to collect additional
// filesystem information.
//
// Subject and object context labeling support added by <danjones@us.ibm.com>
// and <dustin.kirkland@us.ibm.com> for LSPP certification compliance.
//

// flags stating the success for a syscall
pub const AUDITSC_INVALID: c_int = 0;
pub const AUDITSC_SUCCESS: c_int = 1;
pub const AUDITSC_FAILURE: c_int = 2;
// no execve audit message should be longer than this (userspace limits),
// see the note near the top of audit_log_execve_info() about this value
pub const MAX_EXECVE_AUDIT_LEN: c_int = 7500;
// max length to print of cmdline/proctitle value during audit
pub const MAX_PROCTITLE_AUDIT_LEN: c_int = 128;
// number of audit rules
    let mut audit_n_rules = 0;
// determines whether we collect data for signals sent
    let mut audit_signals = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_aux_data {
    pub next: *mut audit_aux_data,
    pub type: c_int,
}

// Number of target pids per aux struct.
pub const AUDIT_AUX_PIDS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_aux_data_pids {
    pub d: audit_aux_data,
    pub target_pid: [pid_t; AUDIT_AUX_PIDS],
    pub target_auid: [kuid_t; AUDIT_AUX_PIDS],
    pub target_uid: [kuid_t; AUDIT_AUX_PIDS],
    pub target_sessionid: [c_uint; AUDIT_AUX_PIDS],
    pub target_ref: [lsm_prop; AUDIT_AUX_PIDS],
    pub target_comm: [c_char; AUDIT_AUX_PIDS][TASK_COMM_LEN],
    pub pid_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_aux_data_bprm_fcaps {
    pub d: audit_aux_data,
    pub fcap: audit_cap_data,
    pub fcap_ver: c_uint,
    pub old_pcap: audit_cap_data,
    pub new_pcap: audit_cap_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_tree_refs {
    pub next: *mut audit_tree_refs,
    pub c: [*mut audit_chunk; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_nfcfgop_tab {
    pub op: audit_nfcfgop,
    pub s: *const c_char,
}

pub static mut audit_nfcfgop_tab: usize = 0;
#[no_mangle]
unsafe extern "C" fn audit_match_perm(ctx: *mut audit_context, mask: c_int) -> c_int {
    let mut n = 0;
    if (unlikely(!ctx)) {
    return 0;
    }
    n = ctx.major;
    switch (audit_classify_syscall(ctx.arch, n)) {
    AUDITSC_NATIVE => {
    if ((mask & AUDIT_PERM_WRITE) &&
    audit_match_class(AUDIT_CLASS_WRITE, n)) {
    return 1;
    }
    if ((mask & AUDIT_PERM_READ) &&
    audit_match_class(AUDIT_CLASS_READ, n)) {
    return 1;
    }
    if ((mask & AUDIT_PERM_ATTR) &&
    audit_match_class(AUDIT_CLASS_CHATTR, n)) {
    return 1;
    }
    return 0;
    AUDITSC_COMPAT => { /* 32bit on biarch */
    if ((mask & AUDIT_PERM_WRITE) &&
    audit_match_class(AUDIT_CLASS_WRITE_32, n)) {
    return 1;
    }
    if ((mask & AUDIT_PERM_READ) &&
    audit_match_class(AUDIT_CLASS_READ_32, n)) {
    return 1;
    }
    if ((mask & AUDIT_PERM_ATTR) &&
    audit_match_class(AUDIT_CLASS_CHATTR_32, n)) {
    return 1;
    }
    return 0;
    AUDITSC_OPEN => {
    return mask & ACC_MODE(ctx.argv[1]);
    AUDITSC_OPENAT => {
    return mask & ACC_MODE(ctx.argv[2]);
    AUDITSC_SOCKETCALL => {
    return ((mask & AUDIT_PERM_WRITE) && ctx.argv[0] == SYS_BIND);
    AUDITSC_EXECVE => {
    return mask & AUDIT_PERM_EXEC;
    AUDITSC_OPENAT2 => {
    return mask & ACC_MODE((u32)ctx.openat2.flags);
    _ => {
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn audit_match_filetype(ctx: *mut audit_context, val: c_int) -> c_int {
    let mut n = core::ptr::null_mut();
pub static mut mode: umode_t = 0;
    if (unlikely(!ctx)) {
    return 0;
    }
    list_for_each_entry(n, &ctx.names_list, list) {
    if ((n.ino != AUDIT_INO_UNSET) &&
    ((n.mode & S_IFMT) == mode)) {
    return 1;
    }
    }
    return 0;
    }
//
// We keep a linked list of fixed-sized (31 pointer) arrays of audit_chunk *;
// ->first_trees points to its beginning, ->trees - to the current end of data.
// ->tree_count is the number of free entries in array pointed to by ->trees.
// Original condition is (NULL, NULL, 0); as soon as it grows we never revert to NULL,
// "empty" becomes (p, p, 31) afterwards.  We don't shrink the list (and seriously,
// it's going to remain 1-element for almost any setup) until we free context itself.
// References in it _are_ dropped - at the same time we free/drop aux stuff.
//
#[no_mangle]
unsafe extern "C" fn audit_set_auditable(ctx: *mut audit_context) {
    if (!ctx.prio) {
    ctx.prio = 1;
    ctx.current_state = AUDIT_STATE_RECORD;
    }
    }
#[no_mangle]
unsafe extern "C" fn put_tree_ref(ctx: *mut audit_context, chunk: *mut audit_chunk) -> c_int {
    let mut p = ctx.trees;
pub static mut left: c_int = 0;
    if (likely(left)) {
    p.c[--left] = chunk;
    ctx.tree_count = left;
    return 1;
    }
    if (!p) {
    return 0;
    }
    p = p.next;
    if (p) {
    p.c[30] = chunk;
    ctx.trees = p;
    ctx.tree_count = 30;
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn grow_tree_refs(ctx: *mut audit_context) -> c_int {
    let mut p = ctx.trees;
    ctx.trees = kzalloc_obj(audit_tree_refs);
    if (!ctx.trees) {
    ctx.trees = p;
    return 0;
    }
    if (p) {
    p.next = ctx.trees;
    }
    else {
    ctx.first_trees = ctx.trees;
    }
    ctx.tree_count = 31;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn unroll_tree_refs() {
    let mut q = core::ptr::null_mut();
    let mut n = 0;
    if (!p) {
// we started with empty chain
    p = ctx.first_trees;
    count = 31;
// if the very first allocation has failed, nothing to do
    if (!p) {
    return;
    }
    }
    n = count;
    while (q != ctx.trees) {
    while (n--) {
    audit_put_chunk(q.c[n]);
    q.c[n] = core::ptr::null_mut();
    }
    }
    while (n-- > ctx.tree_count) {
    audit_put_chunk(q.c[n]);
    q.c[n] = core::ptr::null_mut();
    }
    ctx.trees = p;
    ctx.tree_count = count;
    }
#[no_mangle]
unsafe extern "C" fn free_tree_refs(ctx: *mut audit_context) {
    let mut p = core::ptr::null_mut();
    let mut q = core::ptr::null_mut();
    while (p) {
    q = p.next;
    kfree(p);
    }
    }
#[no_mangle]
unsafe extern "C" fn match_tree_refs(ctx: *mut audit_context, tree: *mut audit_tree) -> c_int {
    let mut p = core::ptr::null_mut();
    let mut n = 0;
    if (!tree) {
    return 0;
    }
// full ones
    while (p != ctx.trees) {
    for (n = 0; n < 31; n++) {
    if (audit_tree_match(p.c[n], tree)) {
    }
    return 1;
    }
    }
// partial
    if (p) {
    for (n = ctx.tree_count; n < 31; n++) {
    if (audit_tree_match(p.c[n], tree)) {
    }
    return 1;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_compare_uid() {
    let mut n = core::ptr::null_mut();
    let mut rc = 0;
    if (name) {
    rc = audit_uid_comparator(uid, f.op, name.uid);
    if (rc) {
    return rc;
    }
    }
    if (ctx) {
    list_for_each_entry(n, &ctx.names_list, list) {
    rc = audit_uid_comparator(uid, f.op, n.uid);
    if (rc) {
    return rc;
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_compare_gid() {
    let mut n = core::ptr::null_mut();
    let mut rc = 0;
    if (name) {
    rc = audit_gid_comparator(gid, f.op, name.gid);
    if (rc) {
    return rc;
    }
    }
    if (ctx) {
    list_for_each_entry(n, &ctx.names_list, list) {
    rc = audit_gid_comparator(gid, f.op, n.gid);
    if (rc) {
    return rc;
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_field_compare() {
    match (f.val) {
// process to file object comparisons
    AUDIT_COMPARE_UID_TO_OBJ_UID => {
    return audit_compare_uid(cred.uid, name, f, ctx);
    }
    AUDIT_COMPARE_GID_TO_OBJ_GID => {
    return audit_compare_gid(cred.gid, name, f, ctx);
    }
    AUDIT_COMPARE_EUID_TO_OBJ_UID => {
    return audit_compare_uid(cred.euid, name, f, ctx);
    }
    AUDIT_COMPARE_EGID_TO_OBJ_GID => {
    return audit_compare_gid(cred.egid, name, f, ctx);
    }
    AUDIT_COMPARE_AUID_TO_OBJ_UID => {
    return audit_compare_uid(audit_get_loginuid(tsk), name, f, ctx);
    }
    AUDIT_COMPARE_SUID_TO_OBJ_UID => {
    return audit_compare_uid(cred.suid, name, f, ctx);
    }
    AUDIT_COMPARE_SGID_TO_OBJ_GID => {
    return audit_compare_gid(cred.sgid, name, f, ctx);
    }
    AUDIT_COMPARE_FSUID_TO_OBJ_UID => {
    return audit_compare_uid(cred.fsuid, name, f, ctx);
    }
    AUDIT_COMPARE_FSGID_TO_OBJ_GID => {
    return audit_compare_gid(cred.fsgid, name, f, ctx);
// uid comparisons
    }
    AUDIT_COMPARE_UID_TO_AUID => {
    return audit_uid_comparator(cred.uid, f.op,
    audit_get_loginuid(tsk));
    }
    AUDIT_COMPARE_UID_TO_EUID => {
    return audit_uid_comparator(cred.uid, f.op, cred.euid);
    }
    AUDIT_COMPARE_UID_TO_SUID => {
    return audit_uid_comparator(cred.uid, f.op, cred.suid);
    }
    AUDIT_COMPARE_UID_TO_FSUID => {
    return audit_uid_comparator(cred.uid, f.op, cred.fsuid);
// auid comparisons
    }
    AUDIT_COMPARE_AUID_TO_EUID => {
    return audit_uid_comparator(audit_get_loginuid(tsk), f.op,
    cred.euid);
    }
    AUDIT_COMPARE_AUID_TO_SUID => {
    return audit_uid_comparator(audit_get_loginuid(tsk), f.op,
    cred.suid);
    }
    AUDIT_COMPARE_AUID_TO_FSUID => {
    return audit_uid_comparator(audit_get_loginuid(tsk), f.op,
    cred.fsuid);
// euid comparisons
    }
    AUDIT_COMPARE_EUID_TO_SUID => {
    return audit_uid_comparator(cred.euid, f.op, cred.suid);
    }
    AUDIT_COMPARE_EUID_TO_FSUID => {
    return audit_uid_comparator(cred.euid, f.op, cred.fsuid);
// suid comparisons
    }
    AUDIT_COMPARE_SUID_TO_FSUID => {
    return audit_uid_comparator(cred.suid, f.op, cred.fsuid);
// gid comparisons
    }
    AUDIT_COMPARE_GID_TO_EGID => {
    return audit_gid_comparator(cred.gid, f.op, cred.egid);
    }
    AUDIT_COMPARE_GID_TO_SGID => {
    return audit_gid_comparator(cred.gid, f.op, cred.sgid);
    }
    AUDIT_COMPARE_GID_TO_FSGID => {
    return audit_gid_comparator(cred.gid, f.op, cred.fsgid);
// egid comparisons
    }
    AUDIT_COMPARE_EGID_TO_SGID => {
    return audit_gid_comparator(cred.egid, f.op, cred.sgid);
    }
    AUDIT_COMPARE_EGID_TO_FSGID => {
    return audit_gid_comparator(cred.egid, f.op, cred.fsgid);
// sgid comparison
    }
    AUDIT_COMPARE_SGID_TO_FSGID => {
    return audit_gid_comparator(cred.sgid, f.op, cred.fsgid);
    }
    _ => {
// WARN;
    return 0;
    }
    }
    return 0;
    }
// Determine if any context name data matches a rule's watch data
// Compare a task_struct with an audit_rule.  Return 1 on match, 0
// otherwise.
//
// If task_creation is true, this is an explicit indication that we are
// filtering a task rule at task creation time.  This and tsk == current are
// the only situations where tsk->cred may be accessed without an rcu read lock.
//
#[no_mangle]
pub unsafe extern "C" fn audit_filter_rules() {
    let mut cred = core::ptr::null_mut();
    int i, need_sid = 1;
pub static mut prop: lsm_prop = 0;
    let mut sessionid = 0;
    if (ctx && rule.prio <= ctx.prio) {
    return 0;
    }
    cred = rcu_dereference_check(tsk.cred, tsk == current || task_creation);
    while (i < rule.field_count) {
    let mut f = &rule.fields[i];
    let mut n = core::ptr::null_mut();
pub static mut result: c_int = 0;
    let mut pid = 0;
    match (f.type) {
    AUDIT_PID => {
    pid = task_tgid_nr(tsk);
    result = audit_comparator(pid, f.op, f.val);
    // break;
    }
    AUDIT_PPID => {
    if (ctx) {
    if (!ctx.ppid) {
    ctx.ppid = task_ppid_nr(tsk);
    }
    result = audit_comparator(ctx.ppid, f.op, f.val);
    }
    // break;
    }
    AUDIT_EXE => {
    result = audit_exe_compare(tsk, rule.exe);
    if (f.op == Audit_not_equal) {
    result = !result;
    }
    // break;
    }
    AUDIT_UID => {
    result = audit_uid_comparator(cred.uid, f.op, f.uid);
    // break;
    }
    AUDIT_EUID => {
    result = audit_uid_comparator(cred.euid, f.op, f.uid);
    // break;
    }
    AUDIT_SUID => {
    result = audit_uid_comparator(cred.suid, f.op, f.uid);
    // break;
    }
    AUDIT_FSUID => {
    result = audit_uid_comparator(cred.fsuid, f.op, f.uid);
    // break;
    }
    AUDIT_GID => {
    result = audit_gid_comparator(cred.gid, f.op, f.gid);
    if (f.op == Audit_equal) {
    if (!result) {
    result = groups_search(cred.group_info, f.gid);
    }
    } else if (f.op == Audit_not_equal) {
    if (result) {
    result = !groups_search(cred.group_info, f.gid);
    }
    }
    // break;
    }
    AUDIT_EGID => {
    result = audit_gid_comparator(cred.egid, f.op, f.gid);
    if (f.op == Audit_equal) {
    if (!result) {
    result = groups_search(cred.group_info, f.gid);
    }
    } else if (f.op == Audit_not_equal) {
    if (result) {
    result = !groups_search(cred.group_info, f.gid);
    }
    }
    // break;
    }
    AUDIT_SGID => {
    result = audit_gid_comparator(cred.sgid, f.op, f.gid);
    // break;
    }
    AUDIT_FSGID => {
    result = audit_gid_comparator(cred.fsgid, f.op, f.gid);
    // break;
    }
    AUDIT_SESSIONID => {
    sessionid = audit_get_sessionid(tsk);
    result = audit_comparator(sessionid, f.op, f.val);
    // break;
    }
    AUDIT_PERS => {
    result = audit_comparator(tsk.personality, f.op, f.val);
    // break;
    }
    AUDIT_ARCH => {
    if (ctx) {
    result = audit_comparator(ctx.arch, f.op, f.val);
    }
    // break;
    }
    AUDIT_EXIT => {
    if (ctx && ctx.return_valid != AUDITSC_INVALID) {
    result = audit_comparator(ctx.return_code, f.op, f.val);
    }
    // break;
    }
    AUDIT_SUCCESS => {
    if (ctx && ctx.return_valid != AUDITSC_INVALID) {
    if (f.val) {
    result = audit_comparator(ctx.return_valid, f.op, AUDITSC_SUCCESS);
    }
    else {
    result = audit_comparator(ctx.return_valid, f.op, AUDITSC_FAILURE);
    }
    }
    // break;
    }
    AUDIT_DEVMAJOR => {
    if (name) {
    if (audit_comparator(MAJOR(name.dev), f.op, f.val) ||
    audit_comparator(MAJOR(name.rdev), f.op, f.val)) {
    result += 1;
    }
    } else if (ctx) {
    list_for_each_entry(n, &ctx.names_list, list) {
    if (audit_comparator(MAJOR(n.dev), f.op, f.val) ||
    audit_comparator(MAJOR(n.rdev), f.op, f.val)) {
    result += 1;
    // break;
    }
    }
    }
    // break;
    }
    AUDIT_DEVMINOR => {
    if (name) {
    if (audit_comparator(MINOR(name.dev), f.op, f.val) ||
    audit_comparator(MINOR(name.rdev), f.op, f.val)) {
    result += 1;
    }
    } else if (ctx) {
    list_for_each_entry(n, &ctx.names_list, list) {
    if (audit_comparator(MINOR(n.dev), f.op, f.val) ||
    audit_comparator(MINOR(n.rdev), f.op, f.val)) {
    result += 1;
    // break;
    }
    }
    }
    // break;
    }
    AUDIT_INODE => {
    if (name) {
    result = audit_comparator(name.ino, f.op, f.val);
    }
if true {
    list_for_each_entry(n, &ctx.names_list, list) {
    if (audit_comparator(n.ino, f.op, f.val)) {
    result += 1;
    // break;
    }
    }
    }
    // break;
    }
    AUDIT_OBJ_UID => {
    if (name) {
    result = audit_uid_comparator(name.uid, f.op, f.uid);
    } else if (ctx) {
    list_for_each_entry(n, &ctx.names_list, list) {
    if (audit_uid_comparator(n.uid, f.op, f.uid)) {
    result += 1;
    // break;
    }
    }
    }
    // break;
    }
    AUDIT_OBJ_GID => {
    if (name) {
    result = audit_gid_comparator(name.gid, f.op, f.gid);
    } else if (ctx) {
    list_for_each_entry(n, &ctx.names_list, list) {
    if (audit_gid_comparator(n.gid, f.op, f.gid)) {
    result += 1;
    // break;
    }
    }
    }
    // break;
    }
    AUDIT_WATCH => {
    if (name) {
    result = audit_watch_compare(rule.watch,
    name.ino,
    name.dev);
    if (f.op == Audit_not_equal) {
    result = !result;
    }
    }
    // break;
    }
    AUDIT_DIR => {
    if (ctx) {
    result = match_tree_refs(ctx, rule.tree);
    if (f.op == Audit_not_equal) {
    result = !result;
    }
    }
    // break;
    }
    AUDIT_LOGINUID => {
    result = audit_uid_comparator(audit_get_loginuid(tsk),
    f.op, f.uid);
    // break;
    }
    AUDIT_LOGINUID_SET => {
    result = audit_comparator(audit_loginuid_set(tsk), f.op, f.val);
    // break;
    }
    AUDIT_SADDR_FAM => {
    if (ctx && ctx.sockaddr) {
    result = audit_comparator(ctx.sockaddr.ss_family,
    f.op, f.val);
    }
    // break;
    }
    AUDIT_SUBJ_USER => {
    }
    AUDIT_SUBJ_ROLE => {
    }
    AUDIT_SUBJ_TYPE => {
    }
    AUDIT_SUBJ_SEN => {
    }
    AUDIT_SUBJ_CLR => {
// NOTE: this may return negative values indicating
    a temporary error.  We simply treat this as a
    match for now to avoid losing information that
    may be wanted.   An error message will also be
    logged upon error */
    if (f.lsm_rule) {
    if (need_sid) {
// @tsk should always be equal to
// @current with the exception of
// fork()/copy_process() in which case
// the new @tsk creds are still a dup
// of @current's creds so we can still
// use
// security_current_getlsmprop_subj()
// here even though it always refs
// @current's creds
//
    security_current_getlsmprop_subj(&prop);
    need_sid = 0;
    }
    result = security_audit_rule_match(&prop,
    f.type,
    f.op,
    f.lsm_rule);
    }
    // break;
    }
    AUDIT_OBJ_USER => {
    }
    AUDIT_OBJ_ROLE => {
    }
    AUDIT_OBJ_TYPE => {
    }
    AUDIT_OBJ_LEV_LOW => {
    }
    AUDIT_OBJ_LEV_HIGH => {
// The above note for AUDIT_SUBJ_USER...AUDIT_SUBJ_CLR
    also applies here */
    if (f.lsm_rule) {
// Find files that match
    if (name) {
    result = security_audit_rule_match(
    &name.oprop,
    f.type,
    f.op,
    f.lsm_rule);
    } else if (ctx) {
    list_for_each_entry(n, &ctx.names_list, list) {
    if (security_audit_rule_match(
    &n.oprop,
    f.type,
    f.op,
    f.lsm_rule)) {
    result += 1;
    // break;
    }
    }
    }
// Find ipc objects that match
    if (!ctx || ctx.type != AUDIT_IPC) {
    // break;
    }
    if (security_audit_rule_match(&ctx.ipc.oprop,
    f.type, f.op,
    f.lsm_rule)) {
    result += 1;
    }
    }
    // break;
    }
    AUDIT_ARG0 => {
    }
    AUDIT_ARG1 => {
    }
    AUDIT_ARG2 => {
    }
    AUDIT_ARG3 => {
    if (ctx) {
    result = audit_comparator(ctx.argv[f.type-AUDIT_ARG0], f.op, f.val);
    }
    // break;
    }
    AUDIT_FILTERKEY => {
// ignore this field for filtering
    result = 1;
    // break;
    }
    AUDIT_PERM => {
    result = audit_match_perm(ctx, f.val);
    if (f.op == Audit_not_equal) {
    result = !result;
    }
    // break;
    }
    AUDIT_FILETYPE => {
    result = audit_match_filetype(ctx, f.val);
    if (f.op == Audit_not_equal) {
    result = !result;
    }
    // break;
    }
    AUDIT_FIELD_COMPARE => {
    result = audit_field_compare(tsk, cred, f, ctx, name);
    // break;
    }
    }
    if (!result) {
    return 0;
    }
    }
    if (ctx) {
    if (rule.filterkey) {
    kfree(ctx.filterkey);
    ctx.filterkey = kstrdup(rule.filterkey, GFP_ATOMIC);
    }
    ctx.prio = rule.prio;
    }
    match (rule.action) {
    AUDIT_NEVER => {
// state = AUDIT_STATE_DISABLED;
    // break;
    }
    AUDIT_ALWAYS => {
// state = AUDIT_STATE_RECORD;
    // break;
    }
    }
    return 1;
    }
// At process creation time, we can determine if system-call auditing is
// completely disabled for this task.  Since we only have the task
// structure at this point, we can only check uid and gid.
//
#[no_mangle]
unsafe extern "C" fn audit_filter_task(tsk: *mut task_struct, key: *mut c_char) -> enum audit_state {
    let mut e = core::ptr::null_mut();
    enum audit_state   state;
    rcu_read_lock();
    list_for_each_entry_rcu(e, &audit_filter_list[AUDIT_FILTER_TASK], list) {
    if (audit_filter_rules(tsk, &e.rule, core::ptr::null_mut(), core::ptr::null_mut(),
    &state, true)) {
    if (state == AUDIT_STATE_RECORD) {
// key = kstrdup(e->rule.filterkey, GFP_ATOMIC);
    }
    rcu_read_unlock();
    return state;
    }
    }
    rcu_read_unlock();
    return AUDIT_STATE_BUILD;
    }
#[no_mangle]
unsafe extern "C" fn audit_in_mask(rule: *const audit_krule, val: c_ulong) -> c_int {
    let mut word = 0;
    let mut bit = 0;
    if (val > 0xffffffff) {
    return false;
    }
    word = AUDIT_WORD(val);
    if (word >= AUDIT_BITMASK_SIZE) {
    return false;
    }
    bit = AUDIT_BIT(val);
    return rule.mask[word] & bit;
    }
//
// __audit_filter_op - common filter helper for operations (syscall/uring/etc)
// @tsk: associated task
// @ctx: audit context
// @list: audit filter list
// @name: audit_name (can be NULL)
// @op: current syscall/uring_op
//
// Run the udit filters specified in @list against @tsk using @ctx,
// @name, and @op, as necessary; the caller is responsible for ensuring
// that the call is made while the RCU read lock is held. The @name
// parameter can be NULL, but all others must be specified.
// Returns 1/true if the filter finds a match, 0/false if none are found.
//
#[no_mangle]
pub unsafe extern "C" fn __audit_filter_op() {
    let mut e = core::ptr::null_mut();
    enum audit_state state;
    list_for_each_entry_rcu(e, list, list) {
    if (audit_in_mask(&e.rule, op) &&
    audit_filter_rules(tsk, &e.rule, ctx, name,
    &state, false)) {
    ctx.current_state = state;
    return 1;
    }
    }
    return 0;
    }
//
// audit_filter_uring - apply filters to an io_uring operation
// @tsk: associated task
// @ctx: audit context
//
#[no_mangle]
pub unsafe extern "C" fn audit_filter_uring() {
    if (auditd_test_task(tsk)) {
    return;
    }
    rcu_read_lock();
    __audit_filter_op(tsk, ctx, &audit_filter_list[AUDIT_FILTER_URING_EXIT],
    core::ptr::null_mut(), ctx.uring_op);
    rcu_read_unlock();
    }
// At syscall exit time, this filter is called if the audit_state is
// not low enough that auditing cannot take place, but is also not
// high enough that we already know we have to write an audit record
// (i.e., the state is AUDIT_STATE_BUILD).
//
#[no_mangle]
pub unsafe extern "C" fn audit_filter_syscall() {
    if (auditd_test_task(tsk)) {
    return;
    }
    rcu_read_lock();
    __audit_filter_op(tsk, ctx, &audit_filter_list[AUDIT_FILTER_EXIT],
    core::ptr::null_mut(), ctx.major);
    rcu_read_unlock();
    }
//
// Given an audit_name check the inode hash table to see if they match.
// Called holding the rcu read lock to protect the use of audit_inode_hash
//
#[no_mangle]
pub unsafe extern "C" fn audit_filter_inode_name() {
pub static mut h: c_int = 0;
    let mut list = &audit_inode_hash[h];
    return __audit_filter_op(tsk, ctx, list, n, ctx.major);
    }
// At syscall exit time, this filter is called if any audit_names have been
// collected during syscall processing.  We only check rules in sublists at hash
// buckets applicable to the inode numbers in audit_names.
// Regarding audit_state, same rules apply as for audit_filter_syscall().
//
#[no_mangle]
pub unsafe extern "C" fn audit_filter_inodes(tsk: *mut task_struct, ctx: *mut audit_context) {
    let mut n = core::ptr::null_mut();
    if (auditd_test_task(tsk)) {
    return;
    }
    rcu_read_lock();
    list_for_each_entry(n, &ctx.names_list, list) {
    if (audit_filter_inode_name(tsk, n, ctx)) {
    break;
    }
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn audit_proctitle_free(context: *mut audit_context) {
    kfree(context.proctitle.value);
    context.proctitle.value = core::ptr::null_mut();
    context.proctitle.len = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_free_module(context: *mut audit_context) {
    if (context.type == AUDIT_KERN_MODULE) {
    kfree(context.module.name);
    context.module.name = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn audit_free_names(context: *mut audit_context) {
    let mut n = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(n, next, &context.names_list, list) {
    list_del(&n.list);
    if (n.name) {
    putname(n.name);
    }
    if (n.should_free) {
    kfree(n);
    }
    }
    context.name_count = 0;
    path_put(&context.pwd);
    context.pwd.dentry = core::ptr::null_mut();
    context.pwd.mnt = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn audit_free_aux(context: *mut audit_context) {
    let mut aux = core::ptr::null_mut();
    while ((aux = context.aux)) {
    context.aux = aux.next;
    kfree(aux);
    }
    context.aux = core::ptr::null_mut();
    while ((aux = context.aux_pids)) {
    context.aux_pids = aux.next;
    kfree(aux);
    }
    context.aux_pids = core::ptr::null_mut();
    }
//
// audit_reset_context - reset a audit_context structure
// @ctx: the audit_context to reset
//
// All fields in the audit_context will be reset to an initial state, all
// references held by fields will be dropped, and private memory will be
// released.  When this function returns the audit_context will be suitable
// for reuse, so long as the passed context is not NULL or a dummy context.
//
#[no_mangle]
unsafe extern "C" fn audit_reset_context(ctx: *mut audit_context) {
    if (!ctx) {
    return;
    }
// if ctx is non-null, reset the "ctx->context" regardless
    ctx.context = AUDIT_CTX_UNUSED;
    if (ctx.dummy) {
    return;
    }
//
// NOTE: It shouldn't matter in what order we release the fields, so
// release them in the order in which they appear in the struct;
// this gives us some hope of quickly making sure we are
// resetting the audit_context properly.
//
// Other things worth mentioning:
// - we don't reset "dummy"
// - we don't reset "state", we do reset "current_state"
// - we preserve "filterkey" if "state" is AUDIT_STATE_RECORD
// - much of this is likely overkill, but play it safe for now
// - we really need to work on improving the audit_context struct
//
    ctx.current_state = ctx.state;
    ctx.stamp.serial = 0;
    ctx.stamp.ctime = (timespec64){ .tv_sec = 0, .tv_nsec = 0 };
    ctx.major = 0;
    ctx.uring_op = 0;
    memset(ctx.argv, 0, sizeof!(ctx.argv));
    ctx.return_code = 0;
    ctx.prio = (ctx.state == AUDIT_STATE_RECORD ? ~0ULL : 0);
    ctx.return_valid = AUDITSC_INVALID;
    audit_free_names(ctx);
    if (ctx.state != AUDIT_STATE_RECORD) {
    kfree(ctx.filterkey);
    ctx.filterkey = core::ptr::null_mut();
    }
    audit_free_aux(ctx);
    kfree(ctx.sockaddr);
    ctx.sockaddr = core::ptr::null_mut();
    ctx.sockaddr_len = 0;
    ctx.ppid = 0;
    ctx.uid = ctx.euid = ctx.suid = ctx.fsuid = KUIDT_INIT(0);
    ctx.gid = ctx.egid = ctx.sgid = ctx.fsgid = KGIDT_INIT(0);
    ctx.personality = 0;
    ctx.arch = 0;
    ctx.target_pid = 0;
    ctx.target_auid = ctx.target_uid = KUIDT_INIT(0);
    ctx.target_sessionid = 0;
    lsmprop_init(&ctx.target_ref);
    ctx.target_comm[0] = '\0';
    unroll_tree_refs(ctx, core::ptr::null_mut(), 0);
// WARN_ON;
    audit_free_module(ctx);
    ctx.fds[0] = -1;
    ctx.type = 0; /* reset last for audit_free_*() */
    }
#[no_mangle]
pub unsafe extern "C" fn audit_alloc_context() {
    let mut context = core::ptr::null_mut();
    context = kzalloc_obj(*context);
    if (!context) {
    return core::ptr::null_mut();
    }
    context.context = AUDIT_CTX_UNUSED;
    context.state = state;
    context.prio = state == AUDIT_STATE_RECORD ? ~0ULL : 0;
// INIT_LIST_HEAD;
// INIT_LIST_HEAD;
    context.fds[0] = -1;
    context.return_valid = AUDITSC_INVALID;
    return context;
    }
//
// audit_alloc - allocate an audit context block for a task
// @tsk: task
//
// Filter on the task information and allocate a per-task audit context
// if necessary.  Doing so turns on system call auditing for the
// specified task.  This is called from copy_process, so no lock is
// needed.
//
#[no_mangle]
pub unsafe extern "C" fn audit_alloc(tsk: *mut task_struct) -> c_int {
    let mut context = core::ptr::null_mut();
    enum audit_state     state;
    let mut key = core::ptr::null_mut();
    if (likely(!audit_ever_enabled)) {
    return 0;
    }
    state = audit_filter_task(tsk, &key);
    if (state == AUDIT_STATE_DISABLED) {
    clear_task_syscall_work(tsk, SYSCALL_AUDIT);
    return 0;
    }
    context = audit_alloc_context(state);
    if (!context) {
    kfree(key);
    audit_log_lost("out of memory in audit_alloc");
    return -ENOMEM;
    }
    context.filterkey = key;
    audit_set_context(tsk, context);
    set_task_syscall_work(tsk, SYSCALL_AUDIT);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_free_context(context: *mut audit_context) {
// resetting is extra work, but it is likely just noise
    audit_reset_context(context);
    audit_proctitle_free(context);
    free_tree_refs(context);
    kfree(context.filterkey);
    kfree(context);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_log_pid_context() {
    let mut ab = core::ptr::null_mut();
pub static mut rc: c_int = 0;
    ab = audit_log_start(context, GFP_KERNEL, AUDIT_OBJ_PID);
    if (!ab) {
    return rc;
    }
    audit_log_format(ab, "opid=%d oauid=%d ouid=%d oses=%d", pid,
    from_kuid(&init_user_ns, auid),
    from_kuid(&init_user_ns, uid), sessionid);
    if (lsmprop_is_set(prop) && audit_log_obj_ctx(ab, prop)) {
    rc = 1;
    }
    audit_log_format(ab, " ocomm=");
    audit_log_untrustedstring(ab, comm);
    audit_log_end(ab);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_log_execve_info() {
    let mut len_max = 0;
    let mut len_rem = 0;
    let mut len_full = 0;
    let mut len_buf = 0;
pub static mut len_abuf: c_long = 0;
    let mut len_tmp = 0;
    let mut require_data = 0;
    let mut encode = 0;
    let mut iter = 0;
    let mut arg = 0;
    let mut buf_head = core::ptr::null_mut();
    let mut buf = core::ptr::null_mut();
    let mut p = current.mm.arg_start;
// NOTE: this buffer needs to be large enough to hold all the non-arg
// data we put in the audit record for this argument (see the
// code below) ... at this point in time 96 is plenty
    char abuf[96];
// NOTE: we set MAX_EXECVE_AUDIT_LEN to a rather arbitrary limit, the
// current value of 7500 is not as important as the fact that it
// is less than 8k, a setting of 7500 gives us plenty of wiggle
// room if we go over a little bit in the logging below
// WARN_ON_ONCE;
    len_max = MAX_EXECVE_AUDIT_LEN;
// scratch buffer to hold the userspace args
    buf_head = kmalloc(MAX_EXECVE_AUDIT_LEN + 1, GFP_KERNEL);
    if (!buf_head) {
    audit_panic("out of memory for argv string");
    return;
    }
    buf = buf_head;
    audit_log_format(*ab, "argc=%d", context.execve.argc);
    len_rem = len_max;
    len_buf = 0;
    len_full = 0;
    require_data = true;
    encode = false;
    iter = 0;
    arg = 0;
    do {
// NOTE: we don't ever want to trust this value for anything
// serious, but the audit record format insists we
// provide an argument length for really long arguments,
// e.g. > MAX_EXECVE_AUDIT_LEN, so we have no choice but
// to use strncpy_from_user() to obtain this value for
// recording in the log, although we don't use it
// anywhere here to avoid a double-fetch problem
    if (len_full == 0) {
    len_full = strnlen_user(p, MAX_ARG_STRLEN) - 1;
    }
// read more data from userspace
    if (require_data) {
// can we make more room in the buffer?
    if (buf != buf_head) {
    memmove(buf_head, buf, len_buf);
    buf = buf_head;
    }
// fetch as much as we can of the argument
    len_tmp = strncpy_from_user(&buf_head[len_buf], p,
    len_max - len_buf);
    if (len_tmp == -EFAULT) {
// unable to copy from userspace
    send_sig(SIGKILL, current, 0);
// goto;
    } else if (len_tmp == (len_max - len_buf)) {
// buffer is not large enough
    require_data = true;
// NOTE: if we are going to span multiple
// buffers force the encoding so we stand
// a chance at a sane len_full value and
// consistent record encoding
    encode = true;
    len_full = len_full * 2;
    p += len_tmp;
    } else {
    require_data = false;
    if (!encode) {
    encode = audit_string_contains_control(
    buf, len_tmp);
    }
// try to use a trusted value for len_full
    if (len_full < len_max) {
    len_full = (encode ?
    len_tmp * 2 : len_tmp);
    }
    p += len_tmp + 1;
    }
    len_buf += len_tmp;
    buf_head[len_buf] = '\0';
// length of the buffer in the audit record?
    len_abuf = (encode ? len_buf * 2 : len_buf + 2);
    }
// write as much as we can to the audit log
    if (len_buf >= 0) {
// NOTE: some magic numbers here - basically if we
// can't fit a reasonable amount of data into the
// existing audit buffer, flush it and start with
// a new buffer
    if ((sizeof!(abuf) + 8) > len_rem) {
    len_rem = len_max;
    audit_log_end(*ab);
// ab = audit_log_start(context,
    GFP_KERNEL, AUDIT_EXECVE);
    if (!*ab) {
// goto;
    }
    }
// create the non-arg portion of the arg record
    len_tmp = 0;
    if (require_data || (iter > 0) ||
    ((len_abuf + sizeof!(abuf)) > len_rem)) {
    if (iter == 0) {
    len_tmp += snprintf(&abuf[len_tmp],
    sizeof!(abuf) - len_tmp,
    " a%d_len=%lu",
    arg, len_full);
    }
    len_tmp += snprintf(&abuf[len_tmp],
    sizeof!(abuf) - len_tmp,
    " a%d[%d]=", arg, iter++);
    } else {
    len_tmp += snprintf(&abuf[len_tmp],
    sizeof!(abuf) - len_tmp,
    " a%d=", arg);
    }
// WARN_ON;
    abuf[sizeof!(abuf) - 1] = '\0';
// log the arg in the audit record
    audit_log_format(*ab, "%s", abuf);
    len_rem -= len_tmp;
    len_tmp = len_buf;
    if (encode) {
    if (len_abuf > len_rem) {
    len_tmp = len_rem / 2; /* encoding */
    }
    audit_log_n_hex(*ab, buf, len_tmp);
    len_rem -= len_tmp * 2;
    len_abuf -= len_tmp * 2;
    } else {
    if (len_abuf > len_rem) {
    len_tmp = len_rem - 2; /* quotes */
    }
    audit_log_n_string(*ab, buf, len_tmp);
    len_rem -= len_tmp + 2;
// don't subtract the "2" because we still need
// to add quotes to the remaining string
    len_abuf -= len_tmp;
    }
    len_buf -= len_tmp;
    buf += len_tmp;
    }
// ready to move to the next argument?
    if ((len_buf == 0) && !require_data) {
    arg += 1;
    iter = 0;
    len_full = 0;
    require_data = true;
    encode = false;
    }
    } while (arg < context.execve.argc);
// NOTE: the caller handles the final audit_log_end() call
// label;
    kfree(buf_head);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_log_cap() {
    if (cap_isclear(*cap)) {
    audit_log_format(ab, " %s=0", prefix);
    return;
    }
    audit_log_format(ab, " %s=%016llx", prefix, cap.val);
    }
#[no_mangle]
unsafe extern "C" fn audit_log_fcaps(ab: *mut audit_buffer, name: *mut audit_names) {
    if (name.fcap_ver == -1) {
    audit_log_format(ab, " cap_fe=? cap_fver=? cap_fp=? cap_fi=?");
    return;
    }
    audit_log_cap(ab, "cap_fp", &name.fcap.permitted);
    audit_log_cap(ab, "cap_fi", &name.fcap.inheritable);
    audit_log_format(ab, " cap_fe=%d cap_fver=%x cap_frootid=%d",
    name.fcap.fE, name.fcap_ver,
    from_kuid(&init_user_ns, name.fcap.rootid));
    }
#[no_mangle]
unsafe extern "C" fn audit_log_time(context: *mut audit_context, ab: *mut audit_buffer) {
    let mut ntp = &context.time.ntp_data;
    let mut tk = &context.time.tk_injoffset;
    static const char * const ntp_name[] = {
    "offset",
    "freq",
    "status",
    "tai",
    "tick",
    "adjust",
    };
    let mut type = 0;
    if (context.type == AUDIT_TIME_ADJNTPVAL) {
    while (type < AUDIT_NTP_NVALS) {
    if (ntp.vals[type].newval != ntp.vals[type].oldval) {
    if (!*ab) {
// ab = audit_log_start(context,
    GFP_KERNEL,
    AUDIT_TIME_ADJNTPVAL);
    if (!*ab) {
    return;
    }
    }
    audit_log_format(*ab, "op=%s old=%lli new=%lli",
    ntp_name[type],
    ntp.vals[type].oldval,
    ntp.vals[type].newval);
    audit_log_end(*ab);
// ab = NULL;
    }
    }
    }
    if (tk.tv_sec != 0 || tk.tv_nsec != 0) {
    if (!*ab) {
// ab = audit_log_start(context, GFP_KERNEL,
    AUDIT_TIME_INJOFFSET);
    if (!*ab) {
    return;
    }
    }
    audit_log_format(*ab, "sec=%lli nsec=%li",
    (long long)tk.tv_sec, tk.tv_nsec);
    audit_log_end(*ab);
// ab = NULL;
    }
    }
#[no_mangle]
unsafe extern "C" fn show_special(context: *mut audit_context, call_panic: *mut c_int) {
    let mut ab = core::ptr::null_mut();
    let mut i = 0;
    ab = audit_log_start(context, GFP_KERNEL, context.type);
    if (!ab) {
    return;
    }
    match (context.type) {
    AUDIT_SOCKETCALL => {
pub static mut nargs: c_int = 0;
    audit_log_format(ab, "nargs=%d", nargs);
    for (i = 0; i < nargs; i++) {
    audit_log_format(ab, " a%d=%lx", i,
    context.socketcall.args[i]);
    }
    }
    break; }
    AUDIT_IPC => {
    audit_log_format(ab, "ouid=%u ogid=%u mode=%#ho",
    from_kuid(&init_user_ns, context.ipc.uid),
    from_kgid(&init_user_ns, context.ipc.gid),
    context.ipc.mode);
    if (lsmprop_is_set(&context.ipc.oprop)) {
    if (audit_log_obj_ctx(ab, &context.ipc.oprop)) {
// call_panic = 1;
    }
    }
    if (context.ipc.has_perm) {
    audit_log_end(ab);
    ab = audit_log_start(context, GFP_KERNEL,
    AUDIT_IPC_SET_PERM);
    if (unlikely(!ab)) {
    return;
    }
    audit_log_format(ab,
    "qbytes=%lx ouid=%u ogid=%u mode=%#ho",
    context.ipc.qbytes,
    context.ipc.perm_uid,
    context.ipc.perm_gid,
    context.ipc.perm_mode);
    }
    break;
    AUDIT_MQ_OPEN => {
    audit_log_format(ab,
    "oflag=0x%x mode=%#ho mq_flags=0x%lx mq_maxmsg=%ld "
    "mq_msgsize=%ld mq_curmsgs=%ld",
    context.mq_open.oflag, context.mq_open.mode,
    context.mq_open.attr.mq_flags,
    context.mq_open.attr.mq_maxmsg,
    context.mq_open.attr.mq_msgsize,
    context.mq_open.attr.mq_curmsgs);
    break;
    AUDIT_MQ_SENDRECV => {
    audit_log_format(ab,
    "mqdes=%d msg_len=%zd msg_prio=%u "
    "abs_timeout_sec=%lld abs_timeout_nsec=%ld",
    context.mq_sendrecv.mqdes,
    context.mq_sendrecv.msg_len,
    context.mq_sendrecv.msg_prio,
    (long long) context.mq_sendrecv.abs_timeout.tv_sec,
    context.mq_sendrecv.abs_timeout.tv_nsec);
    break;
    AUDIT_MQ_NOTIFY => {
    audit_log_format(ab, "mqdes=%d sigev_signo=%d",
    context.mq_notify.mqdes,
    context.mq_notify.sigev_signo);
    break;
    AUDIT_MQ_GETSETATTR => { {
    let mut attr = &context.mq_getsetattr.mqstat;
    audit_log_format(ab,
    "mqdes=%d mq_flags=0x%lx mq_maxmsg=%ld mq_msgsize=%ld "
    "mq_curmsgs=%ld ",
    context.mq_getsetattr.mqdes,
    attr.mq_flags, attr.mq_maxmsg,
    attr.mq_msgsize, attr.mq_curmsgs);
    break; }
    AUDIT_CAPSET => {
    audit_log_format(ab, "pid=%d", context.capset.pid);
    audit_log_cap(ab, "cap_pi", &context.capset.cap.inheritable);
    audit_log_cap(ab, "cap_pp", &context.capset.cap.permitted);
    audit_log_cap(ab, "cap_pe", &context.capset.cap.effective);
    audit_log_cap(ab, "cap_pa", &context.capset.cap.ambient);
    break;
    AUDIT_MMAP => {
    audit_log_format(ab, "fd=%d flags=0x%x", context.mmap.fd,
    context.mmap.flags);
    break;
    AUDIT_OPENAT2 => {
    audit_log_format(ab, "oflag=0%llo mode=0%llo resolve=0x%llx",
    context.openat2.flags,
    context.openat2.mode,
    context.openat2.resolve);
    break;
    AUDIT_EXECVE => {
    audit_log_execve_info(context, &ab);
    break;
    AUDIT_KERN_MODULE => {
    audit_log_format(ab, "name=");
    if (context.module.name) {
    audit_log_untrustedstring(ab, context.module.name);
    } else {
    audit_log_format(ab, "(null)");
    }
    break;
    AUDIT_TIME_ADJNTPVAL => {
    AUDIT_TIME_INJOFFSET => {
// this call deviates from the rest, eating the buffer
    audit_log_time(context, &ab);
    break;
    }
    audit_log_end(ab);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_proctitle_rtrim(proctitle: *mut c_char, len: c_int) -> c_int {
    let mut end = proctitle + len - 1;
    while (end > proctitle && !isprint(*end)) {
    end -= 1;
    }
// catch the case where proctitle is only 1 non-print character
    len = end - proctitle + 1;
    len -= isprint(proctitle[len-1]) == 0;
    return len;
    }
//
// audit_log_name - produce AUDIT_PATH record from struct audit_names
// @context: audit_context for the task
// @n: audit_names structure with reportable details
// @path: optional path to report instead of audit_names->name
// @record_num: record number to report when handling a list of names
// @call_panic: optional pointer to int that will be updated if secid fails
//
#[no_mangle]
pub unsafe extern "C" fn audit_log_name() {
    let mut ab = core::ptr::null_mut();
    ab = audit_log_start(context, GFP_KERNEL, AUDIT_PATH);
    if (!ab) {
    return;
    }
    audit_log_format(ab, "item=%d", record_num);
    if (path) {
    audit_log_d_path(ab, " name=", path);
    }
if true {
    match (n.name_len) {
    AUDIT_NAME_FULL => {
// log the full path
    audit_log_format(ab, " name=");
    audit_log_untrustedstring(ab, n.name.name);
    // break;
    }
    0 => {
// name was specified as a relative path and the
// directory component is the cwd
//
    if (context.pwd.dentry && context.pwd.mnt) {
    audit_log_d_path(ab, " name=", &context.pwd);
    }
    else {
    audit_log_format(ab, " name=(null)");
    }
    // break;
    }
    _ => {
// log the name's directory component
    audit_log_format(ab, " name=");
    audit_log_n_untrustedstring(ab, n.name.name,
    n.name_len);
    }
    }
    } else {
    audit_log_format(ab, " name=(null)");
    }
    if (n.ino != AUDIT_INO_UNSET) {
    audit_log_format(ab, " inode=%llu dev=%02x:%02x mode=%#ho ouid=%u ogid=%u rdev=%02x:%02x",
    n.ino,
    MAJOR(n.dev),
    MINOR(n.dev),
    n.mode,
    from_kuid(&init_user_ns, n.uid),
    from_kgid(&init_user_ns, n.gid),
    MAJOR(n.rdev),
// MINOR;
    }
    if (lsmprop_is_set(&n.oprop) &&
    audit_log_obj_ctx(ab, &n.oprop)) {
// call_panic = 2;
    }
// log the audit_names record type
    match (n.type) {
    AUDIT_TYPE_NORMAL => {
    audit_log_format(ab, " nametype=NORMAL");
    // break;
    }
    AUDIT_TYPE_PARENT => {
    audit_log_format(ab, " nametype=PARENT");
    // break;
    }
    AUDIT_TYPE_CHILD_DELETE => {
    audit_log_format(ab, " nametype=DELETE");
    // break;
    }
    AUDIT_TYPE_CHILD_CREATE => {
    audit_log_format(ab, " nametype=CREATE");
    // break;
    }
    _ => {
    audit_log_format(ab, " nametype=UNKNOWN");
    // break;
    }
    }
    audit_log_fcaps(ab, n);
    audit_log_end(ab);
    }
#[no_mangle]
unsafe extern "C" fn audit_log_proctitle() {
    let mut res = 0;
    let mut buf = core::ptr::null_mut();
    let mut msg = "(null)";
pub static mut len: c_int = 0;
    let mut context = audit_context();
    let mut ab = core::ptr::null_mut();
    ab = audit_log_start(context, GFP_KERNEL, AUDIT_PROCTITLE);
    if (!ab) {
    return;	/* audit_panic or being filtered */
    }
    audit_log_format(ab, "proctitle=");
// Not  cached
    if (!context.proctitle.value) {
    buf = kmalloc(MAX_PROCTITLE_AUDIT_LEN, GFP_KERNEL);
    if (!buf) {
// goto;
    }
// Historically called this from procfs naming
    res = get_cmdline(current, buf, MAX_PROCTITLE_AUDIT_LEN);
    if (res == 0) {
    kfree(buf);
// goto;
    }
    res = audit_proctitle_rtrim(buf, res);
    if (res == 0) {
    kfree(buf);
// goto;
    }
    context.proctitle.value = buf;
    context.proctitle.len = res;
    }
    msg = context.proctitle.value;
    len = context.proctitle.len;
// label;
    audit_log_n_untrustedstring(ab, msg, len);
    audit_log_end(ab);
    }
//
// audit_log_uring - generate a AUDIT_URINGOP record
// @ctx: the audit context
//
#[no_mangle]
unsafe extern "C" fn audit_log_uring(ctx: *mut audit_context) {
    let mut ab = core::ptr::null_mut();
    let mut cred = core::ptr::null_mut();
    ab = audit_log_start(ctx, GFP_ATOMIC, AUDIT_URINGOP);
    if (!ab) {
    return;
    }
    cred = current_cred();
    audit_log_format(ab, "uring_op=%d", ctx.uring_op);
    if (ctx.return_valid != AUDITSC_INVALID) {
    audit_log_format(ab, " success=%s exit=%ld",
    str_yes_no(ctx.return_valid ==
    AUDITSC_SUCCESS),
    ctx.return_code);
    }
    audit_log_format(ab,
    " items=%d"
    " ppid=%d pid=%d uid=%u gid=%u euid=%u suid=%u"
    " fsuid=%u egid=%u sgid=%u fsgid=%u",
    ctx.name_count,
    task_ppid_nr(current), task_tgid_nr(current),
    from_kuid(&init_user_ns, cred.uid),
    from_kgid(&init_user_ns, cred.gid),
    from_kuid(&init_user_ns, cred.euid),
    from_kuid(&init_user_ns, cred.suid),
    from_kuid(&init_user_ns, cred.fsuid),
    from_kgid(&init_user_ns, cred.egid),
    from_kgid(&init_user_ns, cred.sgid),
    from_kgid(&init_user_ns, cred.fsgid));
    audit_log_task_context(ab);
    audit_log_key(ab, ctx.filterkey);
    audit_log_end(ab);
    }
#[no_mangle]
unsafe extern "C" fn audit_log_exit() {
    int i, call_panic = 0;
    let mut context = audit_context();
    let mut ab = core::ptr::null_mut();
    let mut aux = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    context.personality = current.personality;
    match (context.context) {
    AUDIT_CTX_SYSCALL => {
    ab = audit_log_start(context, GFP_KERNEL, AUDIT_SYSCALL);
    if (!ab) {
    return;
    }
    audit_log_format(ab, "arch=%x syscall=%d",
    context.arch, context.major);
    if (context.personality != PER_LINUX) {
    audit_log_format(ab, " per=%lx", context.personality);
    }
    if (context.return_valid != AUDITSC_INVALID) {
    audit_log_format(ab, " success=%s exit=%ld",
    str_yes_no(context.return_valid ==
    AUDITSC_SUCCESS),
    context.return_code);
    }
    audit_log_format(ab,
    " a0=%lx a1=%lx a2=%lx a3=%lx items=%d",
    context.argv[0],
    context.argv[1],
    context.argv[2],
    context.argv[3],
    context.name_count);
    audit_log_task_info(ab);
    audit_log_key(ab, context.filterkey);
    audit_log_end(ab);
    // break;
    }
    AUDIT_CTX_URING => {
    audit_log_uring(context);
    // break;
    }
    _ => {
// BUG;
    // break;
    }
    }
    while (aux) {
    ab = audit_log_start(context, GFP_KERNEL, aux.type);
    if (!ab) {
    continue; /* audit_panic has been called */
    }
    match (aux.type) {
    AUDIT_BPRM_FCAPS => {
    let mut axs = aux;
    audit_log_format(ab, "fver=%x", axs.fcap_ver);
    audit_log_cap(ab, "fp", &axs.fcap.permitted);
    audit_log_cap(ab, "fi", &axs.fcap.inheritable);
    audit_log_format(ab, " fe=%d", axs.fcap.fE);
    audit_log_cap(ab, "old_pp", &axs.old_pcap.permitted);
    audit_log_cap(ab, "old_pi", &axs.old_pcap.inheritable);
    audit_log_cap(ab, "old_pe", &axs.old_pcap.effective);
    audit_log_cap(ab, "old_pa", &axs.old_pcap.ambient);
    audit_log_cap(ab, "pp", &axs.new_pcap.permitted);
    audit_log_cap(ab, "pi", &axs.new_pcap.inheritable);
    audit_log_cap(ab, "pe", &axs.new_pcap.effective);
    audit_log_cap(ab, "pa", &axs.new_pcap.ambient);
    audit_log_format(ab, " frootid=%d",
    from_kuid(&init_user_ns,
    axs.fcap.rootid));
    }
    break; }
    }
    audit_log_end(ab);
    }
    if (context.type) {
    show_special(context, &call_panic);
    }
    if (context.fds[0] >= 0) {
    ab = audit_log_start(context, GFP_KERNEL, AUDIT_FD_PAIR);
    if (ab) {
    audit_log_format(ab, "fd0=%d fd1=%d",
    context.fds[0], context.fds[1]);
    audit_log_end(ab);
    }
    }
    if (context.sockaddr_len) {
    ab = audit_log_start(context, GFP_KERNEL, AUDIT_SOCKADDR);
    if (ab) {
    audit_log_format(ab, "saddr=");
    audit_log_n_hex(ab, context.sockaddr,
    context.sockaddr_len);
    audit_log_end(ab);
    }
    }
    while (aux) {
    let mut axs = aux;
    for (i = 0; i < axs.pid_count; i++) {
    if (audit_log_pid_context(context, axs.target_pid[i],
    axs.target_auid[i],
    axs.target_uid[i],
    axs.target_sessionid[i],
    &axs.target_ref[i],
    axs.target_comm[i]))
    call_panic = 1;
    }
    }
    if (context.target_pid &&
    audit_log_pid_context(context, context.target_pid,
    context.target_auid, context.target_uid,
    context.target_sessionid,
    &context.target_ref,
    context.target_comm)) {
    call_panic = 1;
    }
    if (context.pwd.dentry && context.pwd.mnt) {
    ab = audit_log_start(context, GFP_KERNEL, AUDIT_CWD);
    if (ab) {
    audit_log_d_path(ab, "cwd=", &context.pwd);
    audit_log_end(ab);
    }
    }
    i = 0;
    list_for_each_entry(n, &context.names_list, list) {
    if (n.hidden) {
    continue;
    }
    audit_log_name(context, n, core::ptr::null_mut(), i++, &call_panic);
    }
    if (context.context == AUDIT_CTX_SYSCALL) {
    audit_log_proctitle();
    }
// Send end of event record to help user space know we are finished
    ab = audit_log_start(context, GFP_KERNEL, AUDIT_EOE);
    if (ab) {
    audit_log_end(ab);
    }
    if (call_panic) {
    audit_panic("error in audit_log_exit()");
    }
    }
//
// __audit_free - free a per-task audit context
// @tsk: task whose audit context block to free
//
// Called from copy_process, do_exit, and the io_uring code
//
#[no_mangle]
pub unsafe extern "C" fn __audit_free(tsk: *mut task_struct) {
    let mut context = tsk.audit_context;
    if (!context) {
    return;
    }
// this may generate CONFIG_CHANGE records
    if (!list_empty(&context.killed_trees)) {
    audit_kill_trees(context);
    }
// We are called either by do_exit() or the fork() error handling code;
// in the former case tsk == current and in the latter tsk is a
// random task_struct that doesn't have any meaningful data we
// need to log via audit_log_exit().
//
    if (tsk == current && !context.dummy) {
    context.return_valid = AUDITSC_INVALID;
    context.return_code = 0;
    if (context.context == AUDIT_CTX_SYSCALL) {
    audit_filter_syscall(tsk, context);
    audit_filter_inodes(tsk, context);
    if (context.current_state == AUDIT_STATE_RECORD) {
    audit_log_exit();
    }
    } else if (context.context == AUDIT_CTX_URING) {
// TODO: verify this case is real and valid
    audit_filter_uring(tsk, context);
    audit_filter_inodes(tsk, context);
    if (context.current_state == AUDIT_STATE_RECORD) {
    audit_log_uring(context);
    }
    }
    }
    audit_set_context(tsk, core::ptr::null_mut());
    audit_free_context(context);
    }
//
// audit_return_fixup - fixup the return codes in the audit_context
// @ctx: the audit_context
// @success: true/false value to indicate if the operation succeeded or not
// @code: operation return code
//
// We need to fixup the return code in the audit logs if the actual return
// codes are later going to be fixed by the arch specific signal handlers.
//
#[no_mangle]
pub unsafe extern "C" fn audit_return_fixup() {
//
// This is actually a test for:
// (rc == ERESTARTSYS ) || (rc == ERESTARTNOINTR) ||
(rc == ERESTARTNOHAND) || (rc == ERESTART_RESTARTBLOCK)
//
// but is faster than a bunch of ||
//
    if (unlikely(code <= -ERESTARTSYS) &&
    (code >= -ERESTART_RESTARTBLOCK) &&
    (code != -ENOIOCTLCMD)) {
    ctx.return_code = -EINTR;
    }
    else {
    ctx.return_code  = code;
    }
    ctx.return_valid = (success ? AUDITSC_SUCCESS : AUDITSC_FAILURE);
    }
//
// __audit_uring_entry - prepare the kernel task's audit context for io_uring
// @op: the io_uring opcode
//
// This is similar to audit_syscall_entry() but is intended for use by io_uring
// operations.  This function should only ever be called from
// audit_uring_entry() as we rely on the audit context checking present in that
// function.
//
#[no_mangle]
pub unsafe extern "C" fn __audit_uring_entry(op: u8) {
    let mut ctx = audit_context();
    if (ctx.state == AUDIT_STATE_DISABLED) {
    return;
    }
//
// NOTE: It's possible that we can be called from the process' context
// before it returns to userspace, and before audit_syscall_exit()
// is called.  In this case there is not much to do, just record
// the io_uring details and return.
//
    ctx.uring_op = op;
    if (ctx.context == AUDIT_CTX_SYSCALL) {
    return;
    }
    ctx.dummy = !audit_n_rules;
    if (!ctx.dummy && ctx.state == AUDIT_STATE_BUILD) {
    ctx.prio = 0;
    }
    ctx.context = AUDIT_CTX_URING;
    ctx.current_state = ctx.state;
    ktime_get_coarse_real_ts64(&ctx.stamp.ctime);
    }
//
// __audit_uring_exit - wrap up the kernel task's audit context after io_uring
// @success: true/false value to indicate if the operation succeeded or not
// @code: operation return code
//
// This is similar to audit_syscall_exit() but is intended for use by io_uring
// operations.  This function should only ever be called from
// audit_uring_exit() as we rely on the audit context checking present in that
// function.
//
#[no_mangle]
pub unsafe extern "C" fn __audit_uring_exit(success: c_int, code: c_long) {
    let mut ctx = audit_context();
    if (ctx.dummy) {
    if (ctx.context != AUDIT_CTX_URING) {
    return;
    }
// goto;
    }
    audit_return_fixup(ctx, success, code);
    if (ctx.context == AUDIT_CTX_SYSCALL) {
//
// NOTE: See the note in __audit_uring_entry() about the case
// where we may be called from process context before we
// return to userspace via audit_syscall_exit().  In this
// case we simply emit a URINGOP record and bail, the
// normal syscall exit handling will take care of
// everything else.
// It is also worth mentioning that when we are called,
// the current process creds may differ from the creds
// used during the normal syscall processing; keep that
// in mind if/when we move the record generation code.
//
// We need to filter on the syscall info here to decide if we
// should emit a URINGOP record.  I know it seems odd but this
// solves the problem where users have a filter to block *all
// syscall records in the "exit" filter; we want to preserve
// the behavior here.
//
    audit_filter_syscall(current, ctx);
    if (ctx.current_state != AUDIT_STATE_RECORD) {
    audit_filter_uring(current, ctx);
    }
    audit_filter_inodes(current, ctx);
    if (ctx.current_state != AUDIT_STATE_RECORD) {
    return;
    }
    audit_log_uring(ctx);
    return;
    }
// this may generate CONFIG_CHANGE records
    if (!list_empty(&ctx.killed_trees)) {
    audit_kill_trees(ctx);
    }
// run through both filters to ensure we set the filterkey properly
    audit_filter_uring(current, ctx);
    audit_filter_inodes(current, ctx);
    if (ctx.current_state != AUDIT_STATE_RECORD) {
// goto;
    }
    audit_log_exit();
// label;
    audit_reset_context(ctx);
    }
//
// __audit_syscall_entry - fill in an audit record at syscall entry
// @major: major syscall type (function)
// @a1: additional syscall register 1
// @a2: additional syscall register 2
// @a3: additional syscall register 3
// @a4: additional syscall register 4
//
// Fill in audit context at syscall entry.  This only happens if the
// audit context was created when the task was created and the state or
// filters demand the audit context be built.  If the state from the
// per-task filter or from the per-syscall filter is AUDIT_STATE_RECORD,
// then the record will be written at syscall exit time (otherwise, it
// will only be written if another part of the kernel requests that it
// be written).
//
#[no_mangle]
pub unsafe extern "C" fn __audit_syscall_entry() {
    let mut context = audit_context();
    enum audit_state     state;
    if (!audit_enabled || !context) {
    return;
    }
// WARN_ON;
// WARN_ON;
    if (context.context != AUDIT_CTX_UNUSED || context.name_count) {
    audit_panic("unrecoverable error in audit_syscall_entry()");
    return;
    }
    state = context.state;
    if (state == AUDIT_STATE_DISABLED) {
    return;
    }
    context.dummy = !audit_n_rules;
    if (!context.dummy && state == AUDIT_STATE_BUILD) {
    context.prio = 0;
    if (auditd_test_task(current)) {
    return;
    }
    }
    context.arch	    = syscall_get_arch(current);
    context.major      = major;
    context.argv[0]    = a1;
    context.argv[1]    = a2;
    context.argv[2]    = a3;
    context.argv[3]    = a4;
    context.context = AUDIT_CTX_SYSCALL;
    context.current_state  = state;
    ktime_get_coarse_real_ts64(&context.stamp.ctime);
    }
//
// __audit_syscall_exit - deallocate audit context after a system call
// @success: success value of the syscall
// @return_code: return value of the syscall
//
// Tear down after system call.  If the audit context has been marked as
// auditable (either because of the AUDIT_STATE_RECORD state from
// filtering, or because some other part of the kernel wrote an audit
// message), then write out the syscall information.  In call cases,
// free the names stored from getname().
//
#[no_mangle]
pub unsafe extern "C" fn __audit_syscall_exit(success: c_int, return_code: c_long) {
    let mut context = audit_context();
    if (!context || context.dummy ||
    context.context != AUDIT_CTX_SYSCALL) {
// goto;
    }
// this may generate CONFIG_CHANGE records
    if (!list_empty(&context.killed_trees)) {
    audit_kill_trees(context);
    }
    audit_return_fixup(context, success, return_code);
// run through both filters to ensure we set the filterkey properly
    audit_filter_syscall(current, context);
    audit_filter_inodes(current, context);
    if (context.current_state != AUDIT_STATE_RECORD) {
// goto;
    }
    audit_log_exit();
// label;
    audit_reset_context(context);
    }
#[no_mangle]
pub unsafe extern "C" fn handle_one(inode: *const inode) {
    let mut context = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut chunk = core::ptr::null_mut();
    let mut count = 0;
    if (likely(!inode.i_fsnotify_marks)) {
    return;
    }
    context = audit_context();
    p = context.trees;
    count = context.tree_count;
    rcu_read_lock();
    chunk = audit_tree_lookup(inode);
    rcu_read_unlock();
    if (!chunk) {
    return;
    }
    if (likely(put_tree_ref(context, chunk))) {
    return;
    }
    if (unlikely(!grow_tree_refs(context))) {
    pr_warn!("out of memory, audit has lost a tree reference\n");
    audit_set_auditable(context);
    audit_put_chunk(chunk);
    unroll_tree_refs(context, p, count);
    return;
    }
    put_tree_ref(context, chunk);
    }
#[no_mangle]
unsafe extern "C" fn handle_path(dentry: *const dentry) {
    let mut context = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut d = core::ptr::null_mut();
    let mut parent = core::ptr::null_mut();
    let mut drop = core::ptr::null_mut();
    let mut seq = 0;
    let mut count = 0;
    context = audit_context();
    p = context.trees;
    count = context.tree_count;
// label;
    drop = core::ptr::null_mut();
    d = dentry;
    rcu_read_lock();
    seq = read_seqbegin(&rename_lock);
    for (;;) {
    let mut inode = d_backing_inode(d);
    if (inode && unlikely(inode.i_fsnotify_marks)) {
    let mut chunk = core::ptr::null_mut();
    chunk = audit_tree_lookup(inode);
    if (chunk) {
    if (unlikely(!put_tree_ref(context, chunk))) {
    drop = chunk;
    break;
    }
    }
    }
    parent = d.d_parent;
    if (parent == d) {
    break;
    }
    d = parent;
    }
    if (unlikely(read_seqretry(&rename_lock, seq) || drop)) {  /* in this order */ {
    rcu_read_unlock();
    }
    if (!drop) {
// just a race with rename
    unroll_tree_refs(context, p, count);
// goto;
    }
    audit_put_chunk(drop);
    if (grow_tree_refs(context)) {
// OK, got more space
    unroll_tree_refs(context, p, count);
// goto;
    }
// too bad
    pr_warn!("out of memory, audit has lost a tree reference\n");
    unroll_tree_refs(context, p, count);
    audit_set_auditable(context);
    return;
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn audit_alloc_name() {
    let mut aname = core::ptr::null_mut();
    if (context.name_count < AUDIT_NAMES) {
    aname = &context.preallocated_names[context.name_count];
    memset(aname, 0, sizeof!(*aname));
    } else {
    aname = kzalloc_obj(*aname, GFP_NOFS);
    if (!aname) {
    return core::ptr::null_mut();
    }
    aname.should_free = true;
    }
    aname.ino = AUDIT_INO_UNSET;
    aname.type = type;
    list_add_tail(&aname.list, &context.names_list);
    context.name_count += 1;
    if (!context.pwd.dentry) {
    get_fs_pwd(current.fs, &context.pwd);
    }
    return aname;
    }
//
// __audit_getname - add a name to the list
// @name: name to add
//
// Add a name to the list of audit names for this context.
// Called from fs/namei.c:getname().
//
#[no_mangle]
pub unsafe extern "C" fn __audit_getname(name: *mut filename) {
    let mut context = audit_context();
    let mut n = core::ptr::null_mut();
    if (context.context == AUDIT_CTX_UNUSED) {
    return;
    }
    n = audit_alloc_name(context, AUDIT_TYPE_UNKNOWN);
    if (!n) {
    return;
    }
    n.name = name;
    n.name_len = AUDIT_NAME_FULL;
    name.aname = n;
    name.refcnt += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_copy_fcaps() {
    let mut caps;
    let mut rc = 0;
    if (!dentry) {
    return 0;
    }
    rc = get_vfs_caps_from_disk(&nop_mnt_idmap, dentry, &caps);
    if (rc) {
    return rc;
    }
    name.fcap.permitted = caps.permitted;
    name.fcap.inheritable = caps.inheritable;
    name.fcap.fE = !!(caps.magic_etc & VFS_CAP_FLAGS_EFFECTIVE);
    name.fcap.rootid = caps.rootid;
    name.fcap_ver = (caps.magic_etc & VFS_CAP_REVISION_MASK) >>
    VFS_CAP_REVISION_SHIFT;
    return 0;
    }
// Copy inode data into an audit_names.
#[no_mangle]
pub unsafe extern "C" fn audit_copy_inode() {
    name.ino   = inode.i_ino;
    name.dev   = inode.i_sb.s_dev;
    name.mode  = inode.i_mode;
    name.uid   = inode.i_uid;
    name.gid   = inode.i_gid;
    name.rdev  = inode.i_rdev;
    security_inode_getlsmprop(inode, &name.oprop);
    if (flags & AUDIT_INODE_NOEVAL) {
    name.fcap_ver = -1;
    return;
    }
    audit_copy_fcaps(name, dentry);
    }
//
// __audit_inode - store the inode and device from a lookup
// @name: name being audited
// @dentry: dentry being audited
// @flags: attributes for this particular entry
//
#[no_mangle]
pub unsafe extern "C" fn __audit_inode() {
    let mut context = audit_context();
    let mut inode = d_backing_inode(dentry);
    let mut n = core::ptr::null_mut();
pub static mut parent: bool = false;
    let mut e = core::ptr::null_mut();
    let mut list = &audit_filter_list[AUDIT_FILTER_FS];
    let mut i = 0;
    if (context.context == AUDIT_CTX_UNUSED) {
    return;
    }
    rcu_read_lock();
    list_for_each_entry_rcu(e, list, list) {
    while (i < e.rule.field_count) {
    let mut f = &e.rule.fields[i];
    if (f.type == AUDIT_FSTYPE
    && audit_comparator(inode.i_sb.s_magic,
    f.op, f.val)
    && e.rule.action == AUDIT_NEVER) {
    rcu_read_unlock();
    return;
    }
    }
    }
    rcu_read_unlock();
    if (!name) {
// goto;
    }
//
// If we have a pointer to an audit_names entry already, then we can
// just use it directly if the type is correct.
//
    n = name.aname;
    if (n) {
    if (parent) {
    if (n.type == AUDIT_TYPE_PARENT ||
    n.type == AUDIT_TYPE_UNKNOWN) {
// goto;
    }
    } else {
    if (n.type != AUDIT_TYPE_PARENT) {
// goto;
    }
    }
    }
    list_for_each_entry_reverse(n, &context.names_list, list) {
    if (n.ino) {
// valid inode number, use that for the comparison
    if (n.ino != inode.i_ino ||
    n.dev != inode.i_sb.s_dev) {
    continue;
    }
    } else if (n.name) {
// inode number has not been set, check the name
    if (strcmp(n.name.name, name.name)) {
    continue;
    }
    } else {
// no inode and no name (?!) ... this is odd ...
    continue;
    }
// match the correct record type
    if (parent) {
    if (n.type == AUDIT_TYPE_PARENT ||
    n.type == AUDIT_TYPE_UNKNOWN) {
// goto;
    }
    } else {
    if (n.type != AUDIT_TYPE_PARENT) {
// goto;
    }
    }
    }
// label;
// unable to find an entry with both a matching name and type
    n = audit_alloc_name(context, AUDIT_TYPE_UNKNOWN);
    if (!n) {
    return;
    }
    if (name) {
    n.name = name;
    name.refcnt += 1;
    }
// label;
    if (parent) {
    n.name_len = n.name ? parent_len(n.name.name) : AUDIT_NAME_FULL;
    n.type = AUDIT_TYPE_PARENT;
    if (flags & AUDIT_INODE_HIDDEN) {
    n.hidden = true;
    }
    } else {
    n.name_len = AUDIT_NAME_FULL;
    n.type = AUDIT_TYPE_NORMAL;
    }
    handle_path(dentry);
    audit_copy_inode(n, dentry, inode, flags & AUDIT_INODE_NOEVAL);
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_file(file: *const file) {
    __audit_inode(core::ptr::null_mut(), file.f_path.dentry, 0);
    }
//
// __audit_inode_child - collect inode info for created/removed objects
// @parent: inode of dentry parent
// @dentry: dentry being audited
// @type:   AUDIT_TYPE_* value that we're looking for
//
// For syscalls that create or remove filesystem objects, audit_inode
// can only collect information for the filesystem object's parent.
// This call updates the audit context with the child's information.
// Syscalls that create a new filesystem object must be hooked after
// the object is created.  Syscalls that remove a filesystem object
// must be hooked prior, in order to capture the target inode during
// unsuccessful attempts.
//
#[no_mangle]
pub unsafe extern "C" fn __audit_inode_child() {
    let mut context = audit_context();
    let mut inode = d_backing_inode(dentry);
    let mut dname = &dentry.d_name;
    struct audit_names *n, *found_parent = core::ptr::null_mut(), *found_child = core::ptr::null_mut();
    let mut e = core::ptr::null_mut();
    let mut list = &audit_filter_list[AUDIT_FILTER_FS];
    let mut i = 0;
    if (context.context == AUDIT_CTX_UNUSED) {
    return;
    }
    rcu_read_lock();
    list_for_each_entry_rcu(e, list, list) {
    while (i < e.rule.field_count) {
    let mut f = &e.rule.fields[i];
    if (f.type == AUDIT_FSTYPE
    && audit_comparator(parent.i_sb.s_magic,
    f.op, f.val)
    && e.rule.action == AUDIT_NEVER) {
    rcu_read_unlock();
    return;
    }
    }
    }
    rcu_read_unlock();
    if (inode) {
    handle_one(inode);
    }
    list_for_each_entry(n, &context.names_list, list) {
// can only match entries that have a name
    if (!n.name) {
    continue;
    }
// look for a parent entry first
    if (!found_parent &&
    (n.type == AUDIT_TYPE_PARENT || n.type == AUDIT_TYPE_UNKNOWN) &&
    (n.ino == parent.i_ino && n.dev == parent.i_sb.s_dev &&
    !audit_compare_dname_path(dname, n.name.name, n.name_len))) {
    n.type = AUDIT_TYPE_PARENT;
    found_parent = n;
    if (found_child) {
    break;
    }
    continue;
    }
// is there a matching child entry?
    if (!found_child &&
    (n.type == type || n.type == AUDIT_TYPE_UNKNOWN) &&
    (!strcmp(dname.name, n.name.name) ||
    !audit_compare_dname_path(dname, n.name.name,
    found_parent ?
    found_parent.name_len :
    AUDIT_NAME_FULL))) {
    if (n.type == AUDIT_TYPE_UNKNOWN) {
    n.type = type;
    }
    found_child = n;
    if (found_parent) {
    break;
    }
    }
    }
    if (!found_parent) {
// create a new, "anonymous" parent record
    n = audit_alloc_name(context, AUDIT_TYPE_PARENT);
    if (!n) {
    return;
    }
    audit_copy_inode(n, core::ptr::null_mut(), parent, 0);
    }
    if (!found_child) {
    found_child = audit_alloc_name(context, type);
    if (!found_child) {
    return;
    }
// Re-use the name belonging to the slot for a matching parent
// directory. All names for this context are relinquished in
// audit_free_names()
    if (found_parent) {
    found_child.name = found_parent.name;
    found_child.name_len = AUDIT_NAME_FULL;
    found_child.name.refcnt += 1;
    }
    }
    if (inode) {
    audit_copy_inode(found_child, dentry, inode, 0);
    }
    else {
    found_child.ino = AUDIT_INO_UNSET;
    }
    }
// EXPORT_SYMBOL_GPL;
//
// auditsc_get_stamp - get local copies of audit_context values
// @ctx: audit_context for the task
// @stamp: timestamp to record
//
// Also sets the context as auditable.
//
#[no_mangle]
pub unsafe extern "C" fn auditsc_get_stamp(ctx: *mut audit_context, stamp: *mut audit_stamp) -> c_int {
    if (ctx.context == AUDIT_CTX_UNUSED) {
    return 0;
    }
    if (!ctx.stamp.serial) {
    ctx.stamp.serial = audit_serial();
    }
// stamp = ctx->stamp;
    if (!ctx.prio) {
    ctx.prio = 1;
    ctx.current_state = AUDIT_STATE_RECORD;
    }
    return 1;
    }
//
// __audit_mq_open - record audit data for a POSIX MQ open
// @oflag: open flag
// @mode: mode bits
// @attr: queue attributes
//
#[no_mangle]
pub unsafe extern "C" fn __audit_mq_open(oflag: c_int, mode: umode_t, attr: *mut mq_attr) {
    let mut context = audit_context();
    if (attr) {
    memcpy(&context.mq_open.attr, attr, sizeof!(mq_attr));
    }
    else {
    memset(&context.mq_open.attr, 0, sizeof!(mq_attr));
    }
    context.mq_open.oflag = oflag;
    context.mq_open.mode = mode;
    context.type = AUDIT_MQ_OPEN;
    }
//
// __audit_mq_sendrecv - record audit data for a POSIX MQ timed send/receive
// @mqdes: MQ descriptor
// @msg_len: Message length
// @msg_prio: Message priority
// @abs_timeout: Message timeout in absolute time
//
#[no_mangle]
pub unsafe extern "C" fn __audit_mq_sendrecv() {
    let mut context = audit_context();
    let mut p = &context.mq_sendrecv.abs_timeout;
    if (abs_timeout) {
    memcpy(p, abs_timeout, sizeof!(*p));
    }
    else {
    memset(p, 0, sizeof!(*p));
    }
    context.mq_sendrecv.mqdes = mqdes;
    context.mq_sendrecv.msg_len = msg_len;
    context.mq_sendrecv.msg_prio = msg_prio;
    context.type = AUDIT_MQ_SENDRECV;
    }
//
// __audit_mq_notify - record audit data for a POSIX MQ notify
// @mqdes: MQ descriptor
// @notification: Notification event
//
#[no_mangle]
pub unsafe extern "C" fn __audit_mq_notify(mqdes: mqd_t, notification: *const sigevent) {
    let mut context = audit_context();
    if (notification) {
    context.mq_notify.sigev_signo = notification.sigev_signo;
    }
    else {
    context.mq_notify.sigev_signo = 0;
    }
    context.mq_notify.mqdes = mqdes;
    context.type = AUDIT_MQ_NOTIFY;
    }
//
// __audit_mq_getsetattr - record audit data for a POSIX MQ get/set attribute
// @mqdes: MQ descriptor
// @mqstat: MQ flags
//
#[no_mangle]
pub unsafe extern "C" fn __audit_mq_getsetattr(mqdes: mqd_t, mqstat: *mut mq_attr) {
    let mut context = audit_context();
    context.mq_getsetattr.mqdes = mqdes;
    context.mq_getsetattr.mqstat = *mqstat;
    context.type = AUDIT_MQ_GETSETATTR;
    }
//
// __audit_ipc_obj - record audit data for ipc object
// @ipcp: ipc permissions
//
#[no_mangle]
pub unsafe extern "C" fn __audit_ipc_obj(ipcp: *mut kern_ipc_perm) {
    let mut context = audit_context();
    context.ipc.uid = ipcp.uid;
    context.ipc.gid = ipcp.gid;
    context.ipc.mode = ipcp.mode;
    context.ipc.has_perm = 0;
    security_ipc_getlsmprop(ipcp, &context.ipc.oprop);
    context.type = AUDIT_IPC;
    }
//
// __audit_ipc_set_perm - record audit data for new ipc permissions
// @qbytes: msgq bytes
// @uid: msgq user id
// @gid: msgq group id
// @mode: msgq mode (permissions)
//
// Called only after audit_ipc_obj().
//
#[no_mangle]
pub unsafe extern "C" fn __audit_ipc_set_perm(qbytes: c_ulong, uid: uid_t, gid: gid_t, mode: umode_t) {
    let mut context = audit_context();
    context.ipc.qbytes = qbytes;
    context.ipc.perm_uid = uid;
    context.ipc.perm_gid = gid;
    context.ipc.perm_mode = mode;
    context.ipc.has_perm = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_bprm(bprm: *mut linux_binprm) {
    let mut context = audit_context();
    context.type = AUDIT_EXECVE;
    context.execve.argc = bprm.argc;
    }
//
// __audit_socketcall - record audit data for sys_socketcall
// @nargs: number of args, which should not be more than AUDITSC_ARGS.
// @args: args array
//
#[no_mangle]
pub unsafe extern "C" fn __audit_socketcall(nargs: c_int, args: *mut c_ulong) -> c_int {
    let mut context = audit_context();
    if (nargs <= 0 || nargs > AUDITSC_ARGS || !args) {
    return -EINVAL;
    }
    context.type = AUDIT_SOCKETCALL;
    context.socketcall.nargs = nargs;
    memcpy(context.socketcall.args, args, nargs * sizeof!(unsigned long));
    return 0;
    }
//
// __audit_fd_pair - record audit data for pipe and socketpair
// @fd1: the first file descriptor
// @fd2: the second file descriptor
//
#[no_mangle]
pub unsafe extern "C" fn __audit_fd_pair(fd1: c_int, fd2: c_int) {
    let mut context = audit_context();
    context.fds[0] = fd1;
    context.fds[1] = fd2;
    }
//
// __audit_sockaddr - record audit data for sys_bind, sys_connect, sys_sendto
// @len: data length in user space
// @a: data address in kernel space
//
// Returns 0 for success or NULL context or < 0 on error.
//
#[no_mangle]
pub unsafe extern "C" fn __audit_sockaddr(len: c_int, a: *mut c_void) -> c_int {
    let mut context = audit_context();
    if (!context.sockaddr) {
    let mut p = kmalloc_obj(sockaddr_storage);
    if (!p) {
    return -ENOMEM;
    }
    context.sockaddr = p;
    }
    context.sockaddr_len = len;
    memcpy(context.sockaddr, a, len);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_ptrace(t: *mut task_struct) {
    let mut context = audit_context();
    context.target_pid = task_tgid_nr(t);
    context.target_auid = audit_get_loginuid(t);
    context.target_uid = task_uid(t);
    context.target_sessionid = audit_get_sessionid(t);
    strscpy(context.target_comm, t.comm);
    security_task_getlsmprop_obj(t, &context.target_ref);
    }
//
// audit_signal_info_syscall - record signal info for syscalls
// @t: task being signaled
//
// If the audit subsystem is being terminated, record the task (pid)
// and uid that is doing that.
//
#[no_mangle]
pub unsafe extern "C" fn audit_signal_info_syscall(t: *mut task_struct) -> c_int {
    let mut axp = core::ptr::null_mut();
    let mut ctx = audit_context();
pub static mut t_uid: kuid_t = 0;
    if (!audit_signals || audit_dummy_context()) {
    return 0;
    }
// optimize the common case by putting first signal recipient directly
// in audit_context
    if (!ctx.target_pid) {
    ctx.target_pid = task_tgid_nr(t);
    ctx.target_auid = audit_get_loginuid(t);
    ctx.target_uid = t_uid;
    ctx.target_sessionid = audit_get_sessionid(t);
    strscpy(ctx.target_comm, t.comm);
    security_task_getlsmprop_obj(t, &ctx.target_ref);
    return 0;
    }
    axp = ctx.aux_pids;
    if (!axp || axp.pid_count == AUDIT_AUX_PIDS) {
    axp = kzalloc_obj(*axp, GFP_ATOMIC);
    if (!axp) {
    return -ENOMEM;
    }
    axp.d.type = AUDIT_OBJ_PID;
    axp.d.next = ctx.aux_pids;
    ctx.aux_pids = axp;
    }
    if (WARN_ON_ONCE!(axp.pid_count >= AUDIT_AUX_PIDS)) {
    return -EINVAL;
    }
    axp.target_pid[axp.pid_count] = task_tgid_nr(t);
    axp.target_auid[axp.pid_count] = audit_get_loginuid(t);
    axp.target_uid[axp.pid_count] = t_uid;
    axp.target_sessionid[axp.pid_count] = audit_get_sessionid(t);
    security_task_getlsmprop_obj(t, &axp.target_ref[axp.pid_count]);
    strscpy(axp.target_comm[axp.pid_count], t.comm);
    axp.pid_count += 1;
    return 0;
    }
//
// __audit_log_bprm_fcaps - store information about a loading bprm and relevant fcaps
// @bprm: pointer to the bprm being processed
// @new: the proposed new credentials
// @old: the old credentials
//
// Simply check if the proc already has the caps given by the file and if not
// store the priv escalation info for later auditing at the end of the syscall
//
// -Eric
//
#[no_mangle]
pub unsafe extern "C" fn __audit_log_bprm_fcaps() {
    let mut ax = core::ptr::null_mut();
    let mut context = audit_context();
    let mut vcaps;
    ax = kmalloc_obj(*ax);
    if (!ax) {
    return -ENOMEM;
    }
    ax.d.type = AUDIT_BPRM_FCAPS;
    ax.d.next = context.aux;
    context.aux = ax;
    get_vfs_caps_from_disk(&nop_mnt_idmap,
    bprm.file.f_path.dentry, &vcaps);
    ax.fcap.permitted = vcaps.permitted;
    ax.fcap.inheritable = vcaps.inheritable;
    ax.fcap.fE = !!(vcaps.magic_etc & VFS_CAP_FLAGS_EFFECTIVE);
    ax.fcap.rootid = vcaps.rootid;
    ax.fcap_ver = (vcaps.magic_etc & VFS_CAP_REVISION_MASK) >> VFS_CAP_REVISION_SHIFT;
    ax.old_pcap.permitted   = old.cap_permitted;
    ax.old_pcap.inheritable = old.cap_inheritable;
    ax.old_pcap.effective   = old.cap_effective;
    ax.old_pcap.ambient     = old.cap_ambient;
    ax.new_pcap.permitted   = new.cap_permitted;
    ax.new_pcap.inheritable = new.cap_inheritable;
    ax.new_pcap.effective   = new.cap_effective;
    ax.new_pcap.ambient     = new.cap_ambient;
    return 0;
    }
//
// __audit_log_capset - store information about the arguments to the capset syscall
// @new: the new credentials
// @old: the old (current) credentials
//
// Record the arguments userspace sent to sys_capset for later printing by the
// audit system if applicable
//
#[no_mangle]
pub unsafe extern "C" fn __audit_log_capset(new: *const cred, old: *const cred) {
    let mut context = audit_context();
    context.capset.pid = task_tgid_nr(current);
    context.capset.cap.effective   = new.cap_effective;
    context.capset.cap.inheritable = new.cap_inheritable;
    context.capset.cap.permitted   = new.cap_permitted;
    context.capset.cap.ambient     = new.cap_ambient;
    context.type = AUDIT_CAPSET;
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_mmap_fd(fd: c_int, flags: c_int) {
    let mut context = audit_context();
    context.mmap.fd = fd;
    context.mmap.flags = flags;
    context.type = AUDIT_MMAP;
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_openat2_how(how: *mut open_how) {
    let mut context = audit_context();
    context.openat2.flags = how.flags;
    context.openat2.mode = how.mode;
    context.openat2.resolve = how.resolve;
    context.type = AUDIT_OPENAT2;
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_log_kern_module(name: *const c_char) {
    let mut context = audit_context();
    context.module.name = kstrdup(name, GFP_KERNEL);
    if (!context.module.name) {
    audit_log_lost("out of memory in __audit_log_kern_module");
    }
    context.type = AUDIT_KERN_MODULE;
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_fanotify(response: u32, friar: *mut fanotify_response_info_audit_rule) {
// {subj,obj}_trust values are {0,1,2}: no,yes,unknown
    match (friar.hdr.type) {
    FAN_RESPONSE_INFO_NONE => {
    audit_log(audit_context(), GFP_KERNEL, AUDIT_FANOTIFY,
    "resp=%u fan_type=%u fan_info=0 subj_trust=2 obj_trust=2",
    response, FAN_RESPONSE_INFO_NONE);
    // break;
    }
    FAN_RESPONSE_INFO_AUDIT_RULE => {
    audit_log(audit_context(), GFP_KERNEL, AUDIT_FANOTIFY,
    "resp=%u fan_type=%u fan_info=%X subj_trust=%u obj_trust=%u",
    response, friar.hdr.type, friar.rule_number,
    friar.subj_trust, friar.obj_trust);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_tk_injoffset(offset: timespec64) {
    let mut context = audit_context();
// only set type if not already set by NTP
    if (!context.type) {
    context.type = AUDIT_TIME_INJOFFSET;
    }
    memcpy(&context.time.tk_injoffset, &offset, sizeof!(offset));
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_ntp_log(ad: *const audit_ntp_data) {
    let mut context = audit_context();
    let mut type = 0;
    for (type = 0; type < AUDIT_NTP_NVALS; type++) {
    if (ad.vals[type].newval != ad.vals[type].oldval) {
    }
// unconditionally set type, overwriting TK
    context.type = AUDIT_TIME_ADJNTPVAL;
    memcpy(&context.time.ntp_data, ad, sizeof!(*ad));
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __audit_log_nfcfg() {
    let mut ab = core::ptr::null_mut();
    char comm[sizeof!(current.comm)];
    ab = audit_log_start(audit_context(), gfp, AUDIT_NETFILTER_CFG);
    if (!ab) {
    return;
    }
    audit_log_format(ab, "table=%s family=%u entries=%u op=%s",
    name, af, nentries, audit_nfcfgs[op].s);
    audit_log_format(ab, " pid=%u", task_tgid_nr(current));
    audit_log_task_context(ab); /* subj= */
    audit_log_format(ab, " comm=");
    audit_log_untrustedstring(ab, get_task_comm(comm, current));
    audit_log_end(ab);
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
unsafe extern "C" fn audit_log_task(ab: *mut audit_buffer) {
    kuid_t auid, uid;
    let mut gid;
    let mut sessionid = 0;
    char comm[sizeof!(current.comm)];
    auid = audit_get_loginuid(current);
    sessionid = audit_get_sessionid(current);
    current_uid_gid(&uid, &gid);
    audit_log_format(ab, "auid=%u uid=%u gid=%u ses=%u",
    from_kuid(&init_user_ns, auid),
    from_kuid(&init_user_ns, uid),
    from_kgid(&init_user_ns, gid),
    sessionid);
    audit_log_task_context(ab);
    audit_log_format(ab, " pid=%d comm=", task_tgid_nr(current));
    audit_log_untrustedstring(ab, get_task_comm(comm, current));
    audit_log_d_path_exe(ab, current.mm);
    }
//
// audit_core_dumps - record information about processes that end abnormally
// @signr: signal value
//
// If a process ends with a core dump, something fishy is going on and we
// should record the event for investigation.
//
#[no_mangle]
pub unsafe extern "C" fn audit_core_dumps(signr: c_long) {
    let mut ab = core::ptr::null_mut();
    if (!audit_enabled) {
    return;
    }
    if (signr == SIGQUIT)	/* don't care for those */ {
    return;
    }
    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_ANOM_ABEND);
    if (unlikely(!ab)) {
    return;
    }
    audit_log_task(ab);
    audit_log_format(ab, " sig=%ld res=1", signr);
    audit_log_end(ab);
    }
//
// audit_seccomp - record information about a seccomp action
// @syscall: syscall number
// @signr: signal value
// @code: the seccomp action
//
// Record the information associated with a seccomp action. Event filtering for
// seccomp actions that are not to be logged is done in seccomp_log().
// Therefore, this function forces auditing independent of the audit_enabled
// and dummy context state because seccomp actions should be logged even when
// audit is not in use.
//
#[no_mangle]
pub unsafe extern "C" fn audit_seccomp(syscall: c_ulong, signr: c_long, code: c_int) {
    let mut ab = core::ptr::null_mut();
    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_SECCOMP);
    if (unlikely(!ab)) {
    return;
    }
    audit_log_task(ab);
    audit_log_format(ab, " sig=%ld arch=%x syscall=%ld compat=%d ip=0x%lx code=0x%x",
    signr, syscall_get_arch(current), syscall,
    in_compat_syscall(), KSTK_EIP(current), code);
    audit_log_end(ab);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_seccomp_actions_logged() {
    let mut ab = core::ptr::null_mut();
    if (!audit_enabled) {
    return;
    }
    ab = audit_log_start(audit_context(), GFP_KERNEL,
    AUDIT_CONFIG_CHANGE);
    if (unlikely(!ab)) {
    return;
    }
    audit_log_format(ab,
    "op=seccomp-logging actions=%s old-actions=%s res=%d",
    names, old_names, res);
    audit_log_end(ab);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_killed_trees() {
    let mut ctx = audit_context();
    if (likely(!ctx || ctx.context == AUDIT_CTX_UNUSED)) {
    return core::ptr::null_mut();
    }
    return &ctx.killed_trees;
    }
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}