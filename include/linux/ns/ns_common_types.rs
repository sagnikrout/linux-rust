//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ns/ns_common_types.h
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

//
// Namespace lifetimes are managed via a two-tier reference counting model:
//
// (1) __ns_ref (refcount_t): Main reference count tracking memory
// lifetime. Controls when the namespace structure itself is freed.
// It also pins the namespace on the namespace trees whereas (2)
// only regulates their visibility to userspace.
//
// (2) __ns_ref_active (atomic_t): Reference count tracking active users.
// Controls visibility of the namespace in the namespace trees.
// Any live task that uses the namespace (via nsproxy or cred) holds
// an active reference. Any open file descriptor or bind-mount of
// the namespace holds an active reference. Once all tasks have
// called exited their namespaces and all file descriptors and
// bind-mounts have been released the active reference count drops
// to zero and the namespace becomes inactive. IOW, the namespace
// cannot be listed or opened via file handles anymore.
//
// Note that it is valid to transition from active to inactive and
// back from inactive to active e.g., when resurrecting an inactive
// namespace tree via the SIOCGSKNS ioctl().
//
// Relationship and lifecycle states:
//
// - Active (__ns_ref_active > 0):
// Namespace is actively used and visible to userspace. The namespace
// can be reopened via /proc/<pid>/ns/<ns_type>, via namespace file
// handles, or discovered via listns().
//
// - Inactive (__ns_ref_active == 0, __ns_ref > 0):
// No tasks are actively using the namespace and it isn't pinned by
// any bind-mounts or open file descriptors anymore. But the namespace
// is still kept alive by internal references. For example, the user
// namespace could be pinned by an open file through file->f_cred
// references when one of the now defunct tasks had opened a file and
// handed the file descriptor off to another process via a UNIX
// sockets. Such references keep the namespace structure alive through
// __ns_ref but will not hold an active reference.
//
// - Destroyed (__ns_ref == 0):
// No references remain. The namespace is removed from the tree and freed.
//
// State transitions:
//
// Active -> Inactive:
// When the last task using the namespace exits it drops its active
// references to all namespaces. However, user and pid namespaces
// remain accessible until the task has been reaped.
//
// Inactive -> Active:
// An inactive namespace tree might be resurrected due to e.g., the
// SIOCGSKNS ioctl() on a socket.
//
// Inactive -> Destroyed:
// When __ns_ref drops to zero the namespace is removed from the
// namespaces trees and the memory is freed (after RCU grace period).
//
// Initial namespaces:
// Boot-time namespaces (init_net, init_pid_ns, etc.) start with
// __ns_ref_active = 1 and remain active forever.
//
// @ns_type: type of namespace (e.g., CLONE_NEWNET)
// @stashed: cached dentry to be used by the vfs
// @ops: namespace operations
// @inum: namespace inode number (quickly recycled for non-initial namespaces)
// @__ns_ref: main reference count (do not use directly)
// @ns_tree: namespace tree nodes and active reference count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns_common {
    pub /: *mut *mut refcount_t __ns_ref; / do not use directly,
    pub ____cacheline_aligned_in_smp: },
    pub ns_type: u32,
    pub stashed: *mut dentry,
    pub ops: *const proc_ns_operations,
    pub inum: c_uint,
    pub ns_tree: struct,
    pub ns_rcu: rcu_head,
}

//
// FOR_EACH_NS_TYPE - Canonical list of namespace types
//
// Enumerates all (struct type, CLONE_NEW* flag) pairs.  This is the
// single source of truth used to derive ns_common_type() and
// CLONE_NS_ALL.  When adding a new namespace type, add a single entry
// here; all consumers update automatically.
//
// @X: Callback macro taking (struct_name, clone_flag) as arguments.
//

// Bitmask of all known CLONE_NEW* flags.

//
// ns_common_type - Map a namespace struct pointer to its CLONE_NEW* flag
//
// Uses a leading-comma pattern so the FOR_EACH_NS_TYPE expansion
// produces ", struct foo *: FLAG" entries without a trailing comma.
//

