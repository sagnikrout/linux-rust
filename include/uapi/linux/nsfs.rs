//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nsfs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const NSIO: c_uint = 0xb7;
// Returns a file descriptor that refers to an owning user namespace

// Returns a file descriptor that refers to a parent namespace

// Returns the type of namespace (CLONE_NEW* value) referred to by

// Get owner UID (in the caller's user namespace) for a user namespace

// Translate pid from target pid namespace into the caller's pid namespace.

// Return thread-group leader id of pid in the callers pid namespace.

// Translate pid from caller's pid namespace into a target pid namespace.

// Return thread-group leader id of pid in the target pid namespace.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnt_ns_info {
    pub size: __u32,
    pub nr_mounts: __u32,
    pub mnt_ns_id: __u64,
}

// Get information about namespace.

// Get next namespace.

// Get previous namespace.

// Retrieve namespace identifiers.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_ns_ino {
    IPC_NS_INIT_INO		= 0xEFFFFFFFU,
    UTS_NS_INIT_INO		= 0xEFFFFFFEU,
    USER_NS_INIT_INO	= 0xEFFFFFFDU,
    PID_NS_INIT_INO		= 0xEFFFFFFCU,
    CGROUP_NS_INIT_INO	= 0xEFFFFFFBU,
    TIME_NS_INIT_INO	= 0xEFFFFFFAU,
    NET_NS_INIT_INO		= 0xEFFFFFF9U,
    MNT_NS_INIT_INO		= 0xEFFFFFF8U,

    MNT_NS_ANON_INO		= 0xEFFFFFF7U,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsfs_file_handle {
    pub ns_id: __u64,
    pub ns_type: __u32,
    pub ns_inum: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_ns_id {
    IPC_NS_INIT_ID		= 1ULL,
    UTS_NS_INIT_ID		= 2ULL,
    USER_NS_INIT_ID		= 3ULL,
    PID_NS_INIT_ID		= 4ULL,
    CGROUP_NS_INIT_ID	= 5ULL,
    TIME_NS_INIT_ID		= 6ULL,
    NET_NS_INIT_ID		= 7ULL,
    MNT_NS_INIT_ID		= 8ULL,

    NS_LAST_INIT_ID		= MNT_NS_INIT_ID,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ns_type {
    TIME_NS    = (1ULL << 7),  /* CLONE_NEWTIME */
    MNT_NS     = (1ULL << 17), /* CLONE_NEWNS */
    CGROUP_NS  = (1ULL << 25), /* CLONE_NEWCGROUP */
    UTS_NS     = (1ULL << 26), /* CLONE_NEWUTS */
    IPC_NS     = (1ULL << 27), /* CLONE_NEWIPC */
    USER_NS    = (1ULL << 28), /* CLONE_NEWUSER */
    PID_NS     = (1ULL << 29), /* CLONE_NEWPID */
    NET_NS     = (1ULL << 30), /* CLONE_NEWNET */
}

//
// struct ns_id_req - namespace ID request structure
// @size: size of this structure
// @spare: reserved for future use
// @ns_id: last namespace ID
// @ns_type: bit mask of namespace types to include
// @spare2: reserved for future use
// @user_ns_id: filter on this user namespace ID (or 0)
//
// Structure for passing namespace ID and miscellaneous parameters to
// statns(2) and listns(2).
//
// For statns(2) @param represents the request mask.
// For listns(2) @param represents the last listed mount id (or zero).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns_id_req {
    pub size: __u32,
    pub spare: __u32,
    pub ns_id: __u64,
    pub ns_type: __u32,
    pub spare2: __u32,
    pub user_ns_id: __u64,
}

//
// Special @user_ns_id value that can be passed to listns()
//
pub const LISTNS_CURRENT_USER: c_uint = 0xffffffffffffffff /* Caller's userns */;
// List of all ns_id_req versions.

