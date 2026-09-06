//! Automatically rewritten from C Header to Rust Module
//! Source: fs/tracefs/internal.h
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
pub struct tracefs_inode {
    pub vfs_inode: inode,
// The below gets initialized with memset_after(ti, 0, vfs_inode)
    pub list: list_head,
    pub flags: c_ulong,
    pub private: *mut c_void,
}

//
// struct eventfs_attr - cache the mode and ownership of a eventfs entry
// @mode:	saved mode plus flags of what is saved
// @uid:	saved uid if changed
// @gid:	saved gid if changed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eventfs_attr {
    pub mode: c_int,
    pub uid: kuid_t,
    pub gid: kgid_t,
}

//
// struct eventfs_inode - hold the properties of the eventfs directories.
// @list:	link list into the parent directory
// @rcu:	Union with @list for freeing
// @children:	link list into the child eventfs_inode
// @entries:	the array of entries representing the files in the directory
// @name:	the name of the directory to create
// @entry_attrs: Saved mode and ownership of the @d_children
// @data:	The private data to pass to the callbacks
// @attr:	Saved mode and ownership of eventfs_inode itself
// @is_freed:	Flag set if the eventfs is on its way to be freed
// Note if is_freed is set, then dentry is corrupted.
// @is_events:	Flag set for only the top level "events" directory
// @nr_entries: The number of items in @entries
// @ino:	The saved inode number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eventfs_inode {
    pub list: list_head,
    pub children: list_head,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn container_of(_arg: inode, tracefs_inode: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn eventfs_remount(ti: *mut tracefs_inode, update_uid: bool, update_gid: bool);
}
extern "C" {
    pub fn eventfs_d_release(dentry: *mut dentry);
}
extern "C" {
    pub fn eventfs_remount_lock() -> c_int;
}
extern "C" {
    pub fn eventfs_remount_unlock(srcu_idx: c_int);
}
