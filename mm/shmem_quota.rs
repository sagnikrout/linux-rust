//! Automatically rewritten from C to Rust
//! Source: mm/shmem_quota.c
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
// In memory quota format relies on quota infrastructure to store dquot
// information for us. While conventional quota formats for file systems
// with persistent storage can load quota information into dquot from the
// storage on-demand and hence quota dquot shrinker can free any dquot
// that is not currently being used, it must be avoided here. Otherwise we
// can lose valuable information, user provided limits, because there is
// no persistent storage to load the information from afterwards.
//
// One information that in-memory quota format needs to keep track of is
// a sorted list of ids for each quota type. This is done by utilizing
// an rb tree which root is stored in mem_dqinfo->dqi_priv for each quota
// type.
//
// This format can be used to support quota on file system without persistent
// storage such as tmpfs.
//
// Author:	Lukas Czerner <lczerner@redhat.com>
// Carlos Maiolino <cmaiolino@redhat.com>
//
// Copyright (C) 2023 Red Hat, Inc.
//

//
// The following constants define the amount of time given a user
// before the soft limits are treated as hard limits (usually resulting
// in an allocation failure). The timer is started when the user crosses
// their soft limit, it is reset when they go below their soft limit.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct quota_id {
    pub node: rb_node,
    pub id: qid_t,
    pub bhardlimit: qsize_t,
    pub bsoftlimit: qsize_t,
    pub ihardlimit: qsize_t,
    pub isoftlimit: qsize_t,
}

#[no_mangle]
unsafe extern "C" fn shmem_check_quota_file(sb: *mut super_block, type: c_int) -> c_int {
// There is no real quota file, nothing to do
    return 1;
    }
//
// There is no real quota file. Just allocate rb_root for quota ids and
// set limits
//
#[no_mangle]
unsafe extern "C" fn shmem_read_file_info(sb: *mut super_block, type: c_int) -> c_int {
    let mut dqopt = sb_dqopt(sb);
    let mut info = &dqopt.info[type];
    info.dqi_priv = kzalloc_obj(rb_root, GFP_NOFS);
    if (!info.dqi_priv) {
    return -ENOMEM;
    }
    info.dqi_max_spc_limit = SHMEM_QUOTA_MAX_SPC_LIMIT;
    info.dqi_max_ino_limit = SHMEM_QUOTA_MAX_INO_LIMIT;
    info.dqi_bgrace = SHMEM_MAX_DQ_TIME;
    info.dqi_igrace = SHMEM_MAX_IQ_TIME;
    info.dqi_flags = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shmem_write_file_info(sb: *mut super_block, type: c_int) -> c_int {
// There is no real quota file, nothing to do
    return 0;
    }
//
// Free all the quota_id entries in the rb tree and rb_root.
//
#[no_mangle]
unsafe extern "C" fn shmem_free_file_info(sb: *mut super_block, type: c_int) -> c_int {
    let mut info = &sb_dqopt(sb).info[type];
    let mut root = info.dqi_priv;
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
    info.dqi_priv = core::ptr::null_mut();
    node = rb_first(root);
    while (node) {
    entry = rb_entry(node, quota_id, node);
    node = rb_next(&entry.node);
    rb_erase(&entry.node, root);
    kfree(entry);
    }
    kfree(root);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shmem_get_next_id(sb: *mut super_block, qid: *mut kqid) -> c_int {
    let mut info = sb_dqinfo(sb, qid.type);
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut id: qid_t = 0;
    let mut dqopt = sb_dqopt(sb);
    let mut entry = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (!sb_has_quota_active(sb, qid.type)) {
    return -ESRCH;
    }
    down_read(&dqopt.dqio_sem);
    node = (info.dqi_priv).rb_node;
    while (node) {
    entry = rb_entry(node, quota_id, node);
    if (id < entry.id) {
    node = node.rb_left;
    }

    else if (id > entry.id) {
    node = node.rb_right;
    }
    else {
// goto;
    }
    }
    if (!entry) {
    ret = -ENOENT;
// goto;
    }
    if (id > entry.id) {
    node = rb_next(&entry.node);
    if (!node) {
    ret = -ENOENT;
// goto;
    }
    entry = rb_entry(node, quota_id, node);
    }
// label;
// qid = make_kqid(&init_user_ns, qid->type, entry->id);
// label;
    up_read(&dqopt.dqio_sem);
    return ret;
    }
//
// Load dquot with limits from existing entry, or create the new entry if
// it does not exist.
//
#[no_mangle]
unsafe extern "C" fn shmem_acquire_dquot(dquot: *mut dquot) -> c_int {
    let mut info = sb_dqinfo(dquot.dq_sb, dquot.dq_id.type);
pub static mut n: *mut c_void = core::ptr::null_mut();
    let mut sbinfo = dquot.dq_sb.s_fs_info;
    let mut parent = core::ptr::null_mut(), *new_node = core::ptr::null_mut();
    let mut new_entry = core::ptr::null_mut();
    let mut entry = core::ptr::null_mut();
pub static mut id: qid_t = 0;
    let mut dqopt = sb_dqopt(dquot.dq_sb);
pub static mut ret: c_int = 0;
    mutex_lock(&dquot.dq_lock);
    down_write(&dqopt.dqio_sem);
    n = &(info.dqi_priv).rb_node;
    while (*n) {
    parent = *n;
    entry = rb_entry(parent, quota_id, node);
    if (id < entry.id) {
    n = &(*n).rb_left;
    }

    else if (id > entry.id) {
    n = &(*n).rb_right;
    }
    else {
// goto;
    }
    }
// We don't have entry for this id yet, create it
    new_entry = kzalloc_obj(quota_id, GFP_NOFS);
    if (!new_entry) {
    ret = -ENOMEM;
// goto;
    }
    new_entry.id = id;
    if (dquot.dq_id.type == USRQUOTA) {
    new_entry.bhardlimit = sbinfo.qlimits.usrquota_bhardlimit;
    new_entry.ihardlimit = sbinfo.qlimits.usrquota_ihardlimit;
    } else if (dquot.dq_id.type == GRPQUOTA) {
    new_entry.bhardlimit = sbinfo.qlimits.grpquota_bhardlimit;
    new_entry.ihardlimit = sbinfo.qlimits.grpquota_ihardlimit;
    }
    new_node = &new_entry.node;
    rb_link_node(new_node, parent, n);
    rb_insert_color(new_node, info.dqi_priv);
    entry = new_entry;
// label;
// Load the stored limits from the tree
    spin_lock(&dquot.dq_dqb_lock);
    dquot.dq_dqb.dqb_bhardlimit = entry.bhardlimit;
    dquot.dq_dqb.dqb_bsoftlimit = entry.bsoftlimit;
    dquot.dq_dqb.dqb_ihardlimit = entry.ihardlimit;
    dquot.dq_dqb.dqb_isoftlimit = entry.isoftlimit;
    if (!dquot.dq_dqb.dqb_bhardlimit &&
    !dquot.dq_dqb.dqb_bsoftlimit &&
    !dquot.dq_dqb.dqb_ihardlimit &&
    !dquot.dq_dqb.dqb_isoftlimit) {
    set_bit(DQ_FAKE_B, &dquot.dq_flags);
    }
    spin_unlock(&dquot.dq_dqb_lock);
// Make sure flags update is visible after dquot has been filled
    smp_mb__before_atomic();
    set_bit(DQ_ACTIVE_B, &dquot.dq_flags);
// label;
    up_write(&dqopt.dqio_sem);
    mutex_unlock(&dquot.dq_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn shmem_is_empty_dquot(dquot: *mut dquot) -> bool {
    let mut sbinfo = dquot.dq_sb.s_fs_info;
    let mut bhardlimit;
    let mut ihardlimit;
    if (dquot.dq_id.type == USRQUOTA) {
    bhardlimit = sbinfo.qlimits.usrquota_bhardlimit;
    ihardlimit = sbinfo.qlimits.usrquota_ihardlimit;
    } else if (dquot.dq_id.type == GRPQUOTA) {
    bhardlimit = sbinfo.qlimits.grpquota_bhardlimit;
    ihardlimit = sbinfo.qlimits.grpquota_ihardlimit;
    }
    if (test_bit(DQ_FAKE_B, &dquot.dq_flags) ||
    (dquot.dq_dqb.dqb_curspace == 0 &&
    dquot.dq_dqb.dqb_curinodes == 0 &&
    dquot.dq_dqb.dqb_bhardlimit == bhardlimit &&
    dquot.dq_dqb.dqb_ihardlimit == ihardlimit)) {
    return true;
    }
    return false;
    }
//
// Store limits from dquot in the tree unless it's fake. If it is fake
// remove the id from the tree since there is no useful information in
// there.
//
#[no_mangle]
unsafe extern "C" fn shmem_release_dquot(dquot: *mut dquot) -> c_int {
    let mut info = sb_dqinfo(dquot.dq_sb, dquot.dq_id.type);
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut id: qid_t = 0;
    let mut dqopt = sb_dqopt(dquot.dq_sb);
    let mut entry = core::ptr::null_mut();
    mutex_lock(&dquot.dq_lock);
// Check whether we are not racing with some other dqget()
    if (dquot_is_busy(dquot)) {
// goto;
    }
    down_write(&dqopt.dqio_sem);
    node = (info.dqi_priv).rb_node;
    while (node) {
    entry = rb_entry(node, quota_id, node);
    if (id < entry.id) {
    node = node.rb_left;
    }

    else if (id > entry.id) {
    node = node.rb_right;
    }
    else {
// goto;
    }
    }
// We should always find the entry in the rb tree
    WARN_ONCE(1, "quota id %u from dquot %p, not in rb tree!\n", id, dquot);
    up_write(&dqopt.dqio_sem);
    mutex_unlock(&dquot.dq_lock);
    return -ENOENT;
// label;
    if (shmem_is_empty_dquot(dquot)) {
// Remove entry from the tree
    rb_erase(&entry.node, info.dqi_priv);
    kfree(entry);
    } else {
// Store the limits in the tree
    spin_lock(&dquot.dq_dqb_lock);
    entry.bhardlimit = dquot.dq_dqb.dqb_bhardlimit;
    entry.bsoftlimit = dquot.dq_dqb.dqb_bsoftlimit;
    entry.ihardlimit = dquot.dq_dqb.dqb_ihardlimit;
    entry.isoftlimit = dquot.dq_dqb.dqb_isoftlimit;
    spin_unlock(&dquot.dq_dqb_lock);
    }
    clear_bit(DQ_ACTIVE_B, &dquot.dq_flags);
    up_write(&dqopt.dqio_sem);
// label;
    mutex_unlock(&dquot.dq_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shmem_mark_dquot_dirty(dquot: *mut dquot) -> c_int {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shmem_dquot_write_info(sb: *mut super_block, type: c_int) -> c_int {
    return 0;
    }
pub static mut quota_format_ops: usize = 0;
pub static mut quota_format_type: usize = 0;
pub static mut dquot_operations: usize = 0;