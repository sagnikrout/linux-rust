//! Automatically rewritten from C to Rust
//! Source: kernel/audit_fsnotify.c
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
// audit_fsnotify.c -- tracking inodes
//
// Copyright 2003-2009,2014-2015 Red Hat, Inc.
// Copyright 2005 Hewlett-Packard Development Company, L.P.
// Copyright 2005 IBM Corporation
//

//
// this mark lives on the parent directory of the inode in question.
// but dev, ino, and path are about the child
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_fsnotify_mark {
//     pub /: *mut *mut dev_t dev; / associated superblock device,
//     pub /: *mut *mut u64 ino; / associated inode number,
//     pub /: *mut *mut *mut char path; / insertion path,
//     pub /: *mut *mut fsnotify_mark mark; / fsnotify mark on the inode,
    pub rule: *mut audit_krule,
}

// fsnotify handle.
pub static mut audit_fsnotify_group: *mut c_void = core::ptr::null_mut();
// fsnotify events we care about.

    FS_MOVE_SELF)
#[no_mangle]
unsafe extern "C" fn audit_fsnotify_mark_free(audit_mark: *mut audit_fsnotify_mark) {
    kfree(audit_mark.path);
    kfree(audit_mark);
    }
#[no_mangle]
unsafe extern "C" fn audit_fsnotify_free_mark(mark: *mut fsnotify_mark) {
    let mut audit_mark = core::ptr::null_mut();
    audit_mark = container_of!(mark, audit_fsnotify_mark, mark);
    audit_fsnotify_mark_free(audit_mark);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_mark_path() {
    return mark.path;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_mark_compare(mark: *mut audit_fsnotify_mark, ino: u64, dev: dev_t) -> c_int {
    if (mark.ino == AUDIT_INO_UNSET) {
    return 0;
    }
    return (mark.ino == ino) && (mark.dev == dev);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_update_mark() {
    audit_mark.dev = inode ? inode.i_sb.s_dev : AUDIT_DEV_UNSET;
    audit_mark.ino = inode ? inode.i_ino : AUDIT_INO_UNSET;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_alloc_mark() {
    let mut audit_mark = core::ptr::null_mut();
    let mut path;
    let mut dentry = core::ptr::null_mut();
    let mut dir = core::ptr::null_mut();
    let mut child = core::ptr::null_mut();
    let mut ret = 0;
    let mut allow_dups = 0;
    if (pathname[0] != '/' || pathname[len-1] == '/') {
    return ERR_PTR(-EINVAL);
    }
    if (!ctx) {
    dentry = kern_path_parent(pathname, &path);
    if (IS_ERR(dentry)) {
    return ERR_CAST(dentry); /* returning an error */
    }
    dir = d_inode(path.dentry);
    child = d_inode(dentry);
    allow_dups = 0;
    } else {
    dir = ctx.dir;
    child = ctx.child;
    allow_dups = 1;
    }
    audit_mark = kzalloc_obj(*audit_mark);
    if (unlikely(!audit_mark)) {
    audit_mark = ERR_PTR(-ENOMEM);
// goto;
    }
    fsnotify_init_mark(&audit_mark.mark, audit_fsnotify_group);
    audit_mark.mark.mask = AUDIT_FS_EVENTS;
    audit_mark.path = pathname;
    audit_mark.rule = krule;
    audit_update_mark(audit_mark, child);
    ret = fsnotify_add_inode_mark(&audit_mark.mark, dir, allow_dups);
    if (ret < 0) {
    audit_mark.path = core::ptr::null_mut();
    fsnotify_put_mark(&audit_mark.mark);
    audit_mark = ERR_PTR(ret);
    }
// label;
    if (!ctx) {
    dput(dentry);
    path_put(&path);
    }
    return audit_mark;
    }
#[no_mangle]
unsafe extern "C" fn audit_mark_log_rule_change(audit_mark: *mut audit_fsnotify_mark, op: *mut c_char) {
    let mut ab = core::ptr::null_mut();
    let mut rule = audit_mark.rule;
    if (!audit_enabled) {
    return;
    }
    ab = audit_log_start(audit_context(), GFP_NOFS, AUDIT_CONFIG_CHANGE);
    if (unlikely(!ab)) {
    return;
    }
    audit_log_session_info(ab);
    audit_log_format(ab, " op=%s path=", op);
    audit_log_untrustedstring(ab, audit_mark.path);
    audit_log_key(ab, rule.filterkey);
    audit_log_format(ab, " list=%d res=1", rule.listnr);
    audit_log_end(ab);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_remove_mark(audit_mark: *mut audit_fsnotify_mark) {
    fsnotify_destroy_mark(&audit_mark.mark, audit_fsnotify_group);
    fsnotify_put_mark(&audit_mark.mark);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_remove_mark_rule(krule: *mut audit_krule) {
    let mut mark = krule.exe;
    audit_remove_mark(mark);
    }
#[no_mangle]
unsafe extern "C" fn audit_autoremove_mark_rule(audit_mark: *mut audit_fsnotify_mark) {
    let mut rule = audit_mark.rule;
    let mut entry = container_of!(rule, audit_entry, rule);
    audit_mark_log_rule_change(audit_mark, "autoremove_rule");
    audit_del_rule(entry);
    }
// Update mark data in audit rules based on fsnotify events.
#[no_mangle]
pub unsafe extern "C" fn audit_mark_handle_event() {
    let mut audit_mark = core::ptr::null_mut();
    audit_mark = container_of!(inode_mark, audit_fsnotify_mark, mark);
    if (WARN_ON_ONCE!(inode_mark.group != audit_fsnotify_group)) {
    return 0;
    }
    if (mask & (FS_CREATE|FS_MOVED_TO|FS_DELETE|FS_MOVED_FROM)) {
    if (audit_compare_dname_path(dname, audit_mark.path, AUDIT_NAME_FULL)) {
    return 0;
    }
    audit_update_mark(audit_mark, inode);
    } else if (mask & (FS_DELETE_SELF|FS_UNMOUNT|FS_MOVE_SELF)) {
    audit_autoremove_mark_rule(audit_mark);
    }
    return 0;
    }
pub static mut fsnotify_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn audit_fsnotify_init() -> c_int {
    audit_fsnotify_group = fsnotify_alloc_group(&audit_mark_fsnotify_ops,
    FSNOTIFY_GROUP_DUPS);
    if (IS_ERR(audit_fsnotify_group)) {
    audit_fsnotify_group = core::ptr::null_mut();
    audit_panic("cannot create audit fsnotify group");
    }
    return 0;
    }
// device_initcall;