//! Automatically rewritten from C Header to Rust Module
//! Source: fs/kernfs/kernfs-internal.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// fs/kernfs/kernfs-internal.h - kernfs internal header file
//
// Copyright (c) 2001-3 Patrick Mochel
// Copyright (c) 2007 SUSE Linux Products GmbH
// Copyright (c) 2007, 2013 Tejun Heo <teheo@suse.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_iattrs {
    pub ia_uid: kuid_t,
    pub ia_gid: kgid_t,
    pub ia_atime: timespec64,
    pub ia_mtime: timespec64,
    pub ia_ctime: timespec64,
    pub xattrs: list_head,
    pub xattr_limits: simple_xattr_limits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_root {
// published fields
    pub kn: *mut kernfs_node,
    pub /: *mut *mut *mut unsigned int flags; / KERNFS_ROOT_ flags,
// private fields, do not use outside kernfs proper
    pub ino_idr: idr,
    pub /: *mut *mut spinlock_t kernfs_idr_lock; / root->ino_idr,
    pub last_id_lowbits: u32,
    pub id_highbits: u32,
    pub syscall_ops: *mut kernfs_syscall_ops,
// list of kernfs_super_info of this root, protected by kernfs_rwsem
    pub supers: list_head,
    pub deactivate_waitq: wait_queue_head_t,
    pub kernfs_rwsem: rw_semaphore,
    pub kernfs_iattr_rwsem: rw_semaphore,
    pub kernfs_supers_rwsem: rw_semaphore,
// kn->parent and kn->name
    pub kernfs_rename_lock: rwlock_t,
    pub rcu: rcu_head,
    pub xa_cache: simple_xattr_cache,
}

// +1 to avoid triggering overflow warning when negating it

// KERNFS_TYPE_MASK and types are defined in include/linux/kernfs.h
//
// kernfs_root - find out the kernfs_root a kernfs_node belongs to
// @kn: kernfs_node of interest
//
// Return: the kernfs_root @kn belongs to.
//
// if parent exists, it's always a dir; otherwise, @sd is a dir
//
// mount.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_super_info {
    pub sb: *mut super_block,
//
// The root associated with this super_block.  Each super_block is
// identified by the root and ns it's associated with.
//
    pub root: *mut kernfs_root,
//
// Each sb is associated with one namespace tag, currently the
// network namespace of the task which mounted this kernfs
// instance.  If multiple tags become necessary, make the following
// an array and compare kernfs_node tag against every entry.
//
    pub ns: *const ns_common,
// anchored at kernfs_root->supers, protected by kernfs_rwsem
    pub node: list_head,
}

extern "C" {
    pub fn lockdep_is_held(_arg: &kernfs_root(kn)->kernfs_rwsem) -> return;
}
extern "C" {
    pub fn lockdep_is_held(_arg: &kernfs_root(kn)->kernfs_rename_lock) -> return;
}
extern "C" {
    pub fn rcu_dereference_check(_arg: kn->name, _arg: kernfs_root_is_locked(kn)) -> return;
}
//
// The kernfs_node::__parent remains valid within a RCU section. The kn
// can be reparented (and renamed) which changes the entry. This can be
// avoided by locking kernfs_root::kernfs_rwsem or
// kernfs_root::kernfs_rename_lock.
// Both locks can be used to obtain a reference on __parent. Once the
// reference count reaches 0 then the node is about to be freed
// and can not be renamed (or become a different parent) anymore.
//
// inode.c
//
extern "C" {
    pub fn kernfs_evict_inode(inode: *mut inode);
}
extern "C" {
    pub fn kernfs_iop_listxattr(dentry: *mut dentry, buf: *mut c_char, size: usize) -> isize;
}
extern "C" {
    pub fn __kernfs_setattr(kn: *mut kernfs_node, iattr: *const iattr) -> c_int;
}
//
// dir.c
//
extern "C" {
    pub fn kernfs_put_active(kn: *mut kernfs_node);
}
extern "C" {
    pub fn kernfs_add_one(kn: *mut kernfs_node) -> c_int;
}
//
// file.c
//
extern "C" {
    pub fn kernfs_should_drain_open_files(kn: *mut kernfs_node) -> bool;
}
extern "C" {
    pub fn kernfs_drain_open_files(kn: *mut kernfs_node);
}
//
// symlink.c
//
// kernfs locks
//
// Hashed mutex helpers - protect per-node data structures
