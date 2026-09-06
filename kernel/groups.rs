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
    struct user_namespace *user_ns = current_user_ns();
    let mut i = 0;
pub static mut count: c_uint = group_info.ngroups;
    for (i = 0; i < count; i++) {
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
    struct user_namespace *user_ns = current_user_ns();
    let mut i = 0;
pub static mut count: c_uint = group_info.ngroups;
    for (i = 0; i < count; i++) {
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
pub static mut a: kgid_t = *_a;
pub static mut b: kgid_t = *_b;
    return gid_gt(a, b) - gid_lt(a, b);
    }
#[no_mangle]
pub unsafe extern "C" fn groups_sort(group_info: *mut group_info) {
    sort(group_info.gid, group_info.ngroups, sizeof(*group_info.gid),
    gid_cmp, core::ptr::null_mut());
    }
// EXPORT_SYMBOL;
// a simple bsearch
#[no_mangle]
pub unsafe extern "C" fn groups_search(group_info: *const group_info, grp: kgid_t) -> c_int {
    unsigned int left, right;
    if (!group_info) {
    return 0;
    }
    left = 0;
    right = group_info.ngroups;
    while (left < right) {
pub static mut mid: c_uint = (left+right)/2;
    if (gid_gt(grp, group_info.gid[mid])) {
    left = mid + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: gid_lt(grp, _arg: group_info->gid[mid])) -> else {
    else if (gid_lt(grp, group_info.gid[mid]))
    right = mid;
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
    goto error;
    }
    return commit_creds(new);
    error:
    abort_creds(new);
    return retval;
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn sys_getgroups() {
    const struct cred *cred = current_cred();
    let mut i = 0;
    if (gidsetsize < 0) {
    return -EINVAL;
    }
// no need to grab task_lock here; it cannot change
    i = cred.group_info.ngroups;
    if (gidsetsize) {
    if (i > gidsetsize) {
    i = -EINVAL;
    goto out;
    }
    if (groups_to_user(grouplist, cred.group_info)) {
    i = -EFAULT;
    goto out;
    }
    }
    out:
    return i;
    }
#[no_mangle]
pub unsafe extern "C" fn may_setgroups() -> bool {
    struct user_namespace *user_ns = current_user_ns();
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
    const struct cred *cred = current_cred();
pub static mut retval: c_int = 1;
    if (!gid_eq(grp, cred.fsgid)) {
    retval = groups_search(cred.group_info, grp);
    }
    return retval;
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn in_egroup_p(grp: kgid_t) -> c_int {
    const struct cred *cred = current_cred();
pub static mut retval: c_int = 1;
    if (!gid_eq(grp, cred.egid)) {
    retval = groups_search(cred.group_info, grp);
    }
    return retval;
    }
// EXPORT_SYMBOL;

}