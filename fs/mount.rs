//! Automatically rewritten from C Header to Rust Module
//! Source: fs/mount.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnt_namespace {
    pub ns: ns_common,
    pub root: *mut *mut mount,
    pub /: *mut *mut rb_root mounts; / Protected by namespace_sem,
    pub /: *mut *mut *mut rb_node mnt_last_node; / last (rightmost) mount in the rbtree,
    pub /: *mut *mut *mut rb_node mnt_first_node; / first (leftmost) mount in the rbtree,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnt_pcp {
    pub mnt_count: c_int,
    pub mnt_writers: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mountpoint {
    pub m_hash: hlist_node,
    pub m_dentry: *mut dentry,
    pub m_list: hlist_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mount {
    pub mnt_hash: hlist_node,
    pub mnt_parent: *mut mount,
    pub mnt_mountpoint: *mut dentry,
    pub mnt: vfsmount,
    pub /: *mut *mut rb_node mnt_node; / node in the ns->mounts rbtree,
    pub mnt_rcu: rcu_head,
    pub mnt_llist: llist_node,
}

// except that LSB of pprev is stolen

//
// Containing namespace (active or deactivating, non-refcounted).
// Normally protected by namespace_sem.
// Can also be accessed locklessly under RCU. RCU readers can't rely on
// the namespace still being active, but implicitly hold a passive
// reference (because an RCU delay happens between a namespace being
// deactivated and the corresponding passive refcount drop).
//

//
// T_SHARED_MASK is the set of flags that should be cleared when a
// mount becomes shared.  Currently, this is only the flag that says a
// mount cannot be bind mounted, since this is how we create a mount
// that shares events with another mount.  If you add a new T_
// flag, consider how it interacts with shared mounts.
//

extern "C" {
    pub fn container_of(_arg: mnt, mount: struct, _arg: mnt) -> return;
}
// neither detached nor internal?
extern "C" {
    pub fn __legitimize_mnt(: *mut vfsmount, _arg: unsigned) -> c_int;
}
extern "C" {
    pub fn __detach_mounts(dentry: *mut dentry);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_mounts {
    pub ns: *mut mnt_namespace,
    pub root: path,
    pub ): *mut *mut *mut int (show)(struct seq_file , struct vfsmount,
}

extern "C" {
    pub fn __is_local_mountpoint(dentry: *const dentry) -> bool;
}
extern "C" {
    pub fn __is_local_mountpoint(_arg: dentry) -> return;
}
extern "C" {
    pub fn RB_EMPTY_ROOT(_arg: &ns->mounts) -> return;
}
extern "C" {
    pub fn has_locked_children(mnt: *mut mount, dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn container_of(_arg: ns, mnt_namespace: struct, _arg: ns) -> return;
}

// Optimize the case where there are no watches

extern "C" {
    pub fn __test_write_hold(_arg: m->mnt_pprev_for_sb) -> return;
}
