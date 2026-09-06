//! Automatically rewritten from C to Rust
//! Source: kernel/audit_watch.c
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
// audit_watch.c -- watching inodes
//
// Copyright 2003-2009 Red Hat, Inc.
// Copyright 2005 Hewlett-Packard Development Company, L.P.
// Copyright 2005 IBM Corporation
//

//
// Reference counting:
//
// audit_parent: lifetime is from audit_init_parent() to receipt of an FS_IGNORED
// event.  Each audit_watch holds a reference to its associated parent.
//
// audit_watch: if added to lists, lifetime is from audit_init_watch() to
// audit_remove_watch().  Additionally, an audit_watch may exist
// temporarily to assist in searching existing filter data.  Each
// audit_krule holds a reference to its associated watch.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_watch {
//     pub /: *mut *mut refcount_t count; / reference count,
//     pub /: *mut *mut dev_t dev; / associated superblock device,
//     pub /: *mut *mut *mut char path; / insertion path,
//     pub /: *mut *mut u64 ino; / associated inode number,
//     pub /: *mut *mut *mut audit_parent parent; / associated parent,
//     pub /: *mut *mut list_head wlist; / entry in parent->watches list,
//     pub /: *mut *mut list_head rules; / anchor for krule->rlist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_parent {
//     pub /: *mut *mut list_head watches; / anchor for audit_watch->wlist,
//     pub /: *mut *mut fsnotify_mark mark; / fsnotify mark on the inode,
}

// fsnotify handle.
pub static mut audit_watch_group: *mut c_void = core::ptr::null_mut();
// fsnotify events we care about.

    FS_MOVE_SELF | FS_UNMOUNT)
#[no_mangle]
unsafe extern "C" fn audit_free_parent(parent: *mut audit_parent) {
// WARN_ON;
    kfree(parent);
    }
#[no_mangle]
unsafe extern "C" fn audit_watch_free_mark(entry: *mut fsnotify_mark) {
    let mut parent = core::ptr::null_mut();
    parent = container_of!(entry, audit_parent, mark);
    audit_free_parent(parent);
    }
#[no_mangle]
unsafe extern "C" fn audit_get_parent(parent: *mut audit_parent) {
    if (likely(parent)) {
    fsnotify_get_mark(&parent.mark);
    }
    }
#[no_mangle]
unsafe extern "C" fn audit_put_parent(parent: *mut audit_parent) {
    if (likely(parent)) {
    fsnotify_put_mark(&parent.mark);
    }
    }
//
// Find and return the audit_parent on the given inode.  If found a reference
// is taken on this parent.
//
#[no_mangle]
pub unsafe extern "C" fn audit_find_parent() {
    let mut parent = core::ptr::null_mut();
    let mut entry = core::ptr::null_mut();
    entry = fsnotify_find_inode_mark(inode, audit_watch_group);
    if (entry) {
    parent = container_of!(entry, audit_parent, mark);
    }
    return parent;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_get_watch(watch: *mut audit_watch) {
    refcount_inc(&watch.count);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_put_watch(watch: *mut audit_watch) {
    if (refcount_dec_and_test(&watch.count)) {
// WARN_ON;
// WARN_ON;
    kfree(watch.path);
    kfree(watch);
    }
    }
#[no_mangle]
unsafe extern "C" fn audit_remove_watch(watch: *mut audit_watch) {
    list_del(&watch.wlist);
    audit_put_parent(watch.parent);
    watch.parent = core::ptr::null_mut();
    audit_put_watch(watch); /* match initial get */
    }
#[no_mangle]
pub unsafe extern "C" fn audit_watch_path() {
    return watch.path;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_watch_compare(watch: *mut audit_watch, ino: u64, dev: dev_t) -> c_int {
    return (watch.ino != AUDIT_INO_UNSET) &&
    (watch.ino == ino) &&
    (watch.dev == dev);
    }
// Initialize a parent watch entry.
#[no_mangle]
pub unsafe extern "C" fn audit_init_parent() {
    let mut inode = d_backing_inode(path.dentry);
    let mut parent = core::ptr::null_mut();
    let mut ret = 0;
    parent = kzalloc_obj(*parent);
    if (unlikely(!parent)) {
    return ERR_PTR(-ENOMEM);
    }
// INIT_LIST_HEAD;
    fsnotify_init_mark(&parent.mark, audit_watch_group);
    parent.mark.mask = AUDIT_FS_WATCH;
    ret = fsnotify_add_inode_mark(&parent.mark, inode, 0);
    if (ret < 0) {
    audit_free_parent(parent);
    return ERR_PTR(ret);
    }
    return parent;
    }
// Initialize a watch entry.
#[no_mangle]
pub unsafe extern "C" fn audit_init_watch() {
    let mut watch = core::ptr::null_mut();
    watch = kzalloc_obj(*watch);
    if (unlikely(!watch)) {
    return ERR_PTR(-ENOMEM);
    }
// INIT_LIST_HEAD;
    refcount_set(&watch.count, 1);
    watch.path = path;
    watch.dev = AUDIT_DEV_UNSET;
    watch.ino = AUDIT_INO_UNSET;
    return watch;
    }
// Translate a watch string to kernel representation.
#[no_mangle]
pub unsafe extern "C" fn audit_to_watch(krule: *mut audit_krule, path: *mut c_char, len: c_int, op: u32) -> c_int {
    let mut watch = core::ptr::null_mut();
    if (!audit_watch_group) {
    return -EOPNOTSUPP;
    }
    if (path[0] != '/' || path[len-1] == '/' ||
    (krule.listnr != AUDIT_FILTER_EXIT &&
    krule.listnr != AUDIT_FILTER_URING_EXIT) ||
    op != Audit_equal ||
    krule.inode_f || krule.watch || krule.tree) {
    return -EINVAL;
    }
    watch = audit_init_watch(path);
    if (IS_ERR(watch)) {
    return PTR_ERR(watch);
    }
    krule.watch = watch;
    return 0;
    }
// Duplicate the given audit watch.  The new watch's rules list is initialized
// to an empty list and wlist is undefined.
#[no_mangle]
pub unsafe extern "C" fn audit_dupe_watch() {
    let mut path = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    path = kstrdup(old.path, GFP_KERNEL);
    if (unlikely(!path)) {
    return ERR_PTR(-ENOMEM);
    }
    new = audit_init_watch(path);
    if (IS_ERR(new)) {
    kfree(path);
// goto;
    }
    new.dev = old.dev;
    new.ino = old.ino;
    audit_get_parent(old.parent);
    new.parent = old.parent;
// label;
    return new;
    }
#[no_mangle]
unsafe extern "C" fn audit_watch_log_rule_change(r: *mut audit_krule, w: *mut audit_watch, op: *mut c_char) {
    let mut ab = core::ptr::null_mut();
    if (!audit_enabled) {
    return;
    }
    ab = audit_log_start(audit_context(), GFP_NOFS, AUDIT_CONFIG_CHANGE);
    if (!ab) {
    return;
    }
    audit_log_session_info(ab);
    audit_log_format(ab, "op=%s path=", op);
    audit_log_untrustedstring(ab, w.path);
    audit_log_key(ab, r.filterkey);
    audit_log_format(ab, " list=%d res=1", r.listnr);
    audit_log_end(ab);
    }
// Update inode info in audit rules based on filesystem event.
#[no_mangle]
pub unsafe extern "C" fn audit_update_watch() {
    let mut owatch = core::ptr::null_mut();
    let mut nwatch = core::ptr::null_mut();
    let mut nextw = core::ptr::null_mut();
    let mut r = core::ptr::null_mut();
    let mut nextr = core::ptr::null_mut();
    let mut oentry = core::ptr::null_mut();
    let mut nentry = core::ptr::null_mut();
    mutex_lock(&audit_filter_mutex);
// Run all of the watches on this parent looking for the one that
// matches the given dname
    list_for_each_entry_safe(owatch, nextw, &parent.watches, wlist) {
    if (audit_compare_dname_path(dname, owatch.path,
    AUDIT_NAME_FULL)) {
    continue;
    }
// If the update involves invalidating rules, do the inode-based
// filtering now, so we don't omit records.
    if (invalidating && !audit_dummy_context()) {
    audit_filter_inodes(current, audit_context());
    }
// updating ino will likely change which audit_hash_list we
// are on so we need a new watch for the new list
    nwatch = audit_dupe_watch(owatch);
    if (IS_ERR(nwatch)) {
    mutex_unlock(&audit_filter_mutex);
    audit_panic("error updating watch, skipping");
    return;
    }
    nwatch.dev = dev;
    nwatch.ino = ino;
    list_for_each_entry_safe(r, nextr, &owatch.rules, rlist) {
    oentry = container_of!(r, audit_entry, rule);
    list_del(&oentry.rule.rlist);
    list_del_rcu(&oentry.list);
    nentry = audit_dupe_rule(&oentry.rule, ctx);
    if (IS_ERR(nentry)) {
    list_del(&oentry.rule.list);
    audit_panic("error updating watch, removing");
    } else {
pub static mut h: c_int = 0;
//
// nentry->rule.watch == oentry->rule.watch so
// we must drop that reference and set it to our
// new watch.
//
    audit_put_watch(nentry.rule.watch);
    audit_get_watch(nwatch);
    nentry.rule.watch = nwatch;
    list_add(&nentry.rule.rlist, &nwatch.rules);
    list_add_rcu(&nentry.list, &audit_inode_hash[h]);
    list_replace(&oentry.rule.list,
    &nentry.rule.list);
    }
    if (oentry.rule.exe) {
    audit_remove_mark(oentry.rule.exe);
    }
    call_rcu(&oentry.rcu, audit_free_rule_rcu);
    }
    audit_remove_watch(owatch);
// goto; /* event applies to a single watch */
    }
    mutex_unlock(&audit_filter_mutex);
    return;
// label;
    list_add(&nwatch.wlist, &parent.watches);
    mutex_unlock(&audit_filter_mutex);
    return;
    }
// Remove all watches & rules associated with a parent that is going away.
#[no_mangle]
unsafe extern "C" fn audit_remove_parent_watches(parent: *mut audit_parent) {
    let mut w = core::ptr::null_mut();
    let mut nextw = core::ptr::null_mut();
    let mut r = core::ptr::null_mut();
    let mut nextr = core::ptr::null_mut();
    let mut e = core::ptr::null_mut();
    mutex_lock(&audit_filter_mutex);
    list_for_each_entry_safe(w, nextw, &parent.watches, wlist) {
    list_for_each_entry_safe(r, nextr, &w.rules, rlist) {
    e = container_of!(r, audit_entry, rule);
    audit_watch_log_rule_change(r, w, "remove_rule");
    if (e.rule.exe) {
    audit_remove_mark(e.rule.exe);
    }
    list_del(&r.rlist);
    list_del(&r.list);
    list_del_rcu(&e.list);
    call_rcu(&e.rcu, audit_free_rule_rcu);
    }
    audit_remove_watch(w);
    }
    mutex_unlock(&audit_filter_mutex);
    fsnotify_destroy_mark(&parent.mark, audit_watch_group);
    }
// Get path information necessary for adding watches.
#[no_mangle]
unsafe extern "C" fn audit_get_nd(watch: *mut audit_watch, parent: *mut path) -> c_int {
    let mut d = core::ptr::null_mut();
    d = kern_path_parent(watch.path, parent);
    if (IS_ERR(d)) {
    return PTR_ERR(d);
    }
    if (d_is_positive(d)) {
// update watch filter fields
    watch.dev = d.d_sb.s_dev;
    watch.ino = d_backing_inode(d).i_ino;
    }
    dput(d);
    return 0;
    }
// Associate the given rule with an existing parent.
// Caller must hold audit_filter_mutex.
#[no_mangle]
pub unsafe extern "C" fn audit_add_to_parent() {
    struct audit_watch *w, *watch = krule.watch;
pub static mut watch_found: c_int = 0;
    lockdep_assert_held(&audit_filter_mutex);
    list_for_each_entry(w, &parent.watches, wlist) {
    if (strcmp(watch.path, w.path)) {
    continue;
    }
    watch_found = 1;
// put krule's ref to temporary watch
    audit_put_watch(watch);
    audit_get_watch(w);
    krule.watch = watch = w;
    audit_put_parent(parent);
    break;
    }
    if (!watch_found) {
    watch.parent = parent;
    audit_get_watch(watch);
    list_add(&watch.wlist, &parent.watches);
    }
    list_add(&krule.rlist, &watch.rules);
    }
// Find a matching watch entry, or add this one.
// Caller must hold audit_filter_mutex.
#[no_mangle]
pub unsafe extern "C" fn audit_add_watch(krule: *mut audit_krule, list: *mut list_head) -> c_int {
    let mut watch = krule.watch;
    let mut parent = core::ptr::null_mut();
    let mut parent_path;
    int h, ret = 0;
//
// When we will be calling audit_add_to_parent, krule->watch might have
// been updated and watch might have been freed.
// So we need to keep a reference of watch.
//
    audit_get_watch(watch);
    mutex_unlock(&audit_filter_mutex);
// Avoid calling path_lookup under audit_filter_mutex.
    ret = audit_get_nd(watch, &parent_path);
// caller expects mutex locked
    mutex_lock(&audit_filter_mutex);
    if (ret) {
    audit_put_watch(watch);
    return ret;
    }
// either find an old parent or attach a new one
    parent = audit_find_parent(d_backing_inode(parent_path.dentry));
    if (!parent) {
    parent = audit_init_parent(&parent_path);
    if (IS_ERR(parent)) {
    ret = PTR_ERR(parent);
// goto;
    }
    }
    audit_add_to_parent(krule, parent);
    h = audit_hash_ino(watch.ino);
// list = &audit_inode_hash[h];
// label;
    path_put(&parent_path);
    audit_put_watch(watch);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_remove_watch_rule(krule: *mut audit_krule) {
    let mut watch = krule.watch;
    let mut parent = watch.parent;
    list_del(&krule.rlist);
    if (list_empty(&watch.rules)) {
//
// audit_remove_watch() drops our reference to 'parent' which
// can get freed. Grab our own reference to be safe.
//
    audit_get_parent(parent);
    audit_remove_watch(watch);
    if (list_empty(&parent.watches)) {
    fsnotify_destroy_mark(&parent.mark, audit_watch_group);
    }
    audit_put_parent(parent);
    }
    }
// Update watch data in audit rules based on fsnotify events.
#[no_mangle]
pub unsafe extern "C" fn audit_watch_handle_event() {
    let mut parent = core::ptr::null_mut();
    parent = container_of!(inode_mark, audit_parent, mark);
    if (WARN_ON_ONCE!(inode_mark.group != audit_watch_group)) {
    return 0;
    }
    if (mask & (FS_CREATE|FS_MOVED_TO) && inode) {
pub static mut ctx: audit_watch_ctx = 0;
    audit_update_watch(parent, dname, inode.i_sb.s_dev, inode.i_ino, 0,
    &ctx);
    } else if (mask & (FS_DELETE|FS_MOVED_FROM)) {
pub static mut ctx: audit_watch_ctx = 0;
    audit_update_watch(parent, dname, AUDIT_DEV_UNSET, AUDIT_INO_UNSET, 1,
    &ctx);
    }

    else if (mask & (FS_DELETE_SELF|FS_UNMOUNT|FS_MOVE_SELF)) {
    audit_remove_parent_watches(parent);
    }
    return 0;
    }
pub static mut fsnotify_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn audit_watch_init() -> c_int {
    audit_watch_group = fsnotify_alloc_group(&audit_watch_fsnotify_ops, 0);
    if (IS_ERR(audit_watch_group)) {
    audit_watch_group = core::ptr::null_mut();
    audit_panic("cannot create audit fsnotify group");
    }
    return 0;
    }
// device_initcall;
#[no_mangle]
pub unsafe extern "C" fn audit_dupe_exe() {
    let mut audit_mark = core::ptr::null_mut();
    let mut pathname = core::ptr::null_mut();
    pathname = kstrdup(audit_mark_path(old.exe), GFP_KERNEL);
    if (!pathname) {
    return -ENOMEM;
    }
    audit_mark = audit_alloc_mark(new, pathname, strlen(pathname), ctx);
    if (IS_ERR(audit_mark)) {
    kfree(pathname);
    return PTR_ERR(audit_mark);
    }
    new.exe = audit_mark;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_exe_compare(tsk: *mut task_struct, mark: *mut audit_fsnotify_mark) -> c_int {
    let mut exe_file = core::ptr::null_mut();
    let mut ino = 0;
    let mut dev;
// only do exe filtering if we are recording @current events/records
    if (tsk != current) {
    return 0;
    }
    if (!current.mm) {
    return 0;
    }
    exe_file = get_mm_exe_file(current.mm);
    if (!exe_file) {
    return 0;
    }
    ino = file_inode(exe_file).i_ino;
    dev = file_inode(exe_file).i_sb.s_dev;
    fput(exe_file);
    return audit_mark_compare(mark, ino, dev);
    }