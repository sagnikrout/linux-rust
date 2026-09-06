//! Automatically rewritten from C to Rust
//! Source: kernel/groups.c
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


























// SPDX-License-Identifier: GPL-2.0
//
// Supplementary group IDs
//

#[no_mangle]
pub unsafe extern "C" fn groups_alloc() {
    let mut gi = core::ptr::null_mut();
    gi = kvmalloc_flex(*gi, gid, gidsetsize, GFP_KERNEL_ACCOUNT);
    if (!gi) {
    return core::ptr::null_mut();
    }
    refcount_set(&gi.usage, 1);
    gi.ngroups = gidsetsize;
    return gi;
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn groups_free(group_info: *mut group_info) {
    kvfree(group_info);
    }
// EXPORT_SYMBOL;
// export the group_info to a user-space array
#[no_mangle]
pub unsafe extern "C" fn groups_to_user() {
    let mut user_ns = current_user_ns();
    let mut i = 0;
pub static mut count: c_uint = 0;
    while (i < count) {
    let mut gid = 0;
    gid = from_kgid_munged(user_ns, group_info.gid[i]);
    if (put_user(gid, grouplist+i)) {
    return -EFAULT;
    }
    }
    return 0;
    }
// fill a group_info from a user-space array - it must be allocated already
#[no_mangle]
pub unsafe extern "C" fn groups_from_user() {
    let mut user_ns = current_user_ns();
    let mut i = 0;
pub static mut count: c_uint = 0;
    while (i < count) {
    let mut gid = 0;
    let mut kgid;
    if (get_user(gid, grouplist+i)) {
    return -EFAULT;
    }
    kgid = make_kgid(user_ns, gid);
    if (!gid_valid(kgid)) {
    return -EINVAL;
    }
    group_info.gid[i] = kgid;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gid_cmp(_a: *const c_void, _b: *const c_void) -> c_int {
pub static mut a: kgid_t = 0;
pub static mut b: kgid_t = 0;
    return gid_gt(a, b) - gid_lt(a, b);
    }
#[no_mangle]
pub unsafe extern "C" fn groups_sort(group_info: *mut group_info) {
    sort(group_info.gid, group_info.ngroups, sizeof!(*group_info.gid),
    gid_cmp, core::ptr::null_mut());
    }
// EXPORT_SYMBOL;
// a simple bsearch
#[no_mangle]
pub unsafe extern "C" fn groups_search(group_info: *const group_info, grp: kgid_t) -> c_int {
    let mut left = 0;
    let mut right = 0;
    if (!group_info) {
    return 0;
    }
    left = 0;
    right = group_info.ngroups;
    while (left < right) {
pub static mut mid: c_uint = 0;
    if (gid_gt(grp, group_info.gid[mid])) {
    left = mid + 1;
    }

    else if (gid_lt(grp, group_info.gid[mid])) {
    right = mid;
    }
    else {
    return 1;
    }
    }
    return 0;
    }
//
// set_groups - Change a group subscription in a set of credentials
// @new: The newly prepared set of credentials to alter
// @group_info: The group list to install
//
#[no_mangle]
pub unsafe extern "C" fn set_groups(new: *mut cred, group_info: *mut group_info) {
    put_group_info(new.group_info);
    get_group_info(group_info);
    new.group_info = group_info;
    }
// EXPORT_SYMBOL;
//
// set_current_groups - Change current's group subscription
// @group_info: The group list to impose
//
// Validate a group subscription and, if valid, impose it upon current's task
// security record.
//
#[no_mangle]
pub unsafe extern "C" fn set_current_groups(group_info: *mut group_info) -> c_int {
    let mut new = core::ptr::null_mut();
    let mut old = core::ptr::null_mut();
    let mut retval = 0;
    new = prepare_creds();
    if (!new) {
    return -ENOMEM;
    }
    old = current_cred();
    set_groups(new, group_info);
    retval = security_task_fix_setgroups(new, old);
    if (retval < 0) {
// goto;
    }
    return commit_creds(new);
// label;
    abort_creds(new);
    return retval;
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn sys_getgroups() {
    let mut cred = current_cred();
    let mut i = 0;
    if (gidsetsize < 0) {
    return -EINVAL;
    }
// no need to grab task_lock here; it cannot change
    i = cred.group_info.ngroups;
    if (gidsetsize) {
    if (i > gidsetsize) {
    i = -EINVAL;
// goto;
    }
    if (groups_to_user(grouplist, cred.group_info)) {
    i = -EFAULT;
// goto;
    }
    }
// label;
    return i;
    }
#[no_mangle]
pub unsafe extern "C" fn may_setgroups() -> bool {
    let mut user_ns = current_user_ns();
    return ns_capable_setid(user_ns, CAP_SETGID) &&
    userns_may_setgroups(user_ns);
    }
//
// SMP: Our groups are copy-on-write. We can set them safely
// without another task interfering.
//
#[no_mangle]
pub unsafe extern "C" fn sys_setgroups() {
    let mut group_info = core::ptr::null_mut();
    let mut retval = 0;
    if (!may_setgroups()) {
    return -EPERM;
    }
    if ((unsigned)gidsetsize > NGROUPS_MAX) {
    return -EINVAL;
    }
    group_info = groups_alloc(gidsetsize);
    if (!group_info) {
    return -ENOMEM;
    }
    retval = groups_from_user(group_info, grouplist);
    if (retval) {
    put_group_info(group_info);
    return retval;
    }
    groups_sort(group_info);
    retval = set_current_groups(group_info);
    put_group_info(group_info);
    return retval;
    }
//
// Check whether we're fsgid/egid or in the supplemental group..
//
#[no_mangle]
pub unsafe extern "C" fn in_group_p(grp: kgid_t) -> c_int {
    let mut cred = current_cred();
pub static mut retval: c_int = 1;
    if (!gid_eq(grp, cred.fsgid)) {
    retval = groups_search(cred.group_info, grp);
    }
    return retval;
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn in_egroup_p(grp: kgid_t) -> c_int {
    let mut cred = current_cred();
pub static mut retval: c_int = 1;
    if (!gid_eq(grp, cred.egid)) {
    retval = groups_search(cred.group_info, grp);
    }
    return retval;
    }
// EXPORT_SYMBOL;