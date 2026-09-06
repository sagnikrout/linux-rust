//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/user_namespace.h
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


// SPDX-License-Identifier: GPL-2.0

pub const UID_GID_MAP_MAX_BASE_EXTENTS: c_int = 5;
pub const UID_GID_MAP_MAX_EXTENTS: c_int = 340;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uid_gid_extent {
    pub first: u32,
    pub lower_first: u32,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uid_gid_map {
    pub extent: [uid_gid_extent; UID_GID_MAP_MAX_BASE_EXTENTS],
    pub nr_extents: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucount_type {
    UCOUNT_USER_NAMESPACES,
    UCOUNT_PID_NAMESPACES,
    UCOUNT_UTS_NAMESPACES,
    UCOUNT_IPC_NAMESPACES,
    UCOUNT_NET_NAMESPACES,
    UCOUNT_MNT_NAMESPACES,
    UCOUNT_CGROUP_NAMESPACES,
    UCOUNT_TIME_NAMESPACES,

    UCOUNT_INOTIFY_INSTANCES,
    UCOUNT_INOTIFY_WATCHES,

    UCOUNT_FANOTIFY_GROUPS,
    UCOUNT_FANOTIFY_MARKS,

    UCOUNT_BINFMT_MISC_INTERPRETERS,

    UCOUNT_COUNTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rlimit_type {
    UCOUNT_RLIMIT_NPROC,
    UCOUNT_RLIMIT_MSGQUEUE,
    UCOUNT_RLIMIT_SIGPENDING,
    UCOUNT_RLIMIT_MEMLOCK,
    UCOUNT_RLIMIT_COUNTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace {
    pub uid_map: uid_gid_map,
    pub gid_map: uid_gid_map,
    pub projid_map: uid_gid_map,
    pub parent: *mut user_namespace,
    pub level: c_int,
    pub owner: kuid_t,
    pub group: kgid_t,
    pub ns: ns_common,
    pub flags: c_ulong,
// parent_could_setfcap: true if the creator if this ns had CAP_SETFCAP
// in its effective capability set at the child ns creation time.
    pub parent_could_setfcap: bool,

// List of joinable keyrings in this namespace.  Modification access of
// these pointers is controlled by keyring_sem.  Once
// user_keyring_register is set, it won't be changed, so it can be
// accessed directly with READ_ONCE().
//
    pub keyring_name_list: list_head,
    pub user_keyring_register: *mut key,
    pub keyring_sem: rw_semaphore,

// Register of per-UID persistent keyrings for this namespace

    pub persistent_keyring_register: *mut key,

    pub work: work_struct,

    pub set: ctl_table_set,
    pub sysctls: *mut ctl_table_header,

    pub ucounts: *mut ucounts,
    pub ucount_max: [c_long; UCOUNT_COUNTS],
    pub rlimit_max: [c_long; UCOUNT_RLIMIT_COUNTS],
    pub binfmt_misc: *mut binfmt_misc,

    pub __randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucounts {
    pub node: hlist_nulls_node,
    pub ns: *mut user_namespace,
    pub uid: kuid_t,
    pub rcu: rcu_head,
    pub count: rcuref_t,
    pub ucount: [atomic_long_t; UCOUNT_COUNTS],
    pub rlimit: [atomic_long_t; UCOUNT_RLIMIT_COUNTS],
}

extern "C" {
    pub fn setup_userns_sysctls(ns: *mut user_namespace) -> bool;
}
extern "C" {
    pub fn retire_userns_sysctls(ns: *mut user_namespace);
}
extern "C" {
    pub fn dec_ucount(ucounts: *mut ucounts, type: ucount_type);
}
extern "C" {
    pub fn put_ucounts(ucounts: *mut ucounts);
}
extern "C" {
    pub fn atomic_long_read(_arg: &ucounts->rlimit[type]) -> return;
}
extern "C" {
    pub fn inc_rlimit_ucounts(ucounts: *mut ucounts, type: rlimit_type, v: c_long) -> c_long;
}
extern "C" {
    pub fn dec_rlimit_ucounts(ucounts: *mut ucounts, type: rlimit_type, v: c_long) -> bool;
}
extern "C" {
    pub fn dec_rlimit_put_ucounts(ucounts: *mut ucounts, type: rlimit_type);
}
extern "C" {
    pub fn is_rlimit_overlimit(ucounts: *mut ucounts, type: rlimit_type, max: c_ulong) -> bool;
}
extern "C" {
    pub fn READ_ONCE(_arg: ns->rlimit_max[type]) -> return;
}
extern "C" {
    pub fn container_of(_arg: ns, user_namespace: struct, _arg: ns) -> return;
}

extern "C" {
    pub fn create_user_ns(new: *mut cred) -> c_int;
}
extern "C" {
    pub fn unshare_userns(unshare_flags: c_ulong, new_cred: *mut cred) -> c_int;
}
extern "C" {
    pub fn __put_user_ns(ns: *mut user_namespace);
}
extern "C" {
    pub fn proc_uid_map_write(: *mut file, : *const char __user, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn proc_gid_map_write(: *mut file, : *const char __user, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn proc_projid_map_write(: *mut file, : *const char __user, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn proc_setgroups_write(: *mut file, : *const char __user, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn proc_setgroups_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn userns_may_setgroups(ns: *const user_namespace) -> bool;
}
extern "C" {
    pub fn current_in_userns(target_ns: *const user_namespace) -> bool;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EPERM) -> return;
}

