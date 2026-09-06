//! Automatically rewritten from C Header to Rust Module
//! Source: fs/notify/fsnotify.h
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
// fsnotify_connp_t is what we embed in objects which connector can be attached
// to.
//
extern "C" {
    pub fn real_mount(_arg: conn->obj) -> return;
}
extern "C" {
    pub fn fsnotify_object_sb(_arg: conn->obj, _arg: conn->type) -> return;
}
// destroy all events sitting in this groups notification queue
extern "C" {
    pub fn fsnotify_flush_notify(group: *mut fsnotify_group);
}
// protects reads of inode and vfsmount marks list
// compare two groups for sorting of marks lists
// Destroy all inode marks for given superblock
extern "C" {
    pub fn fsnotify_unmount_inodes(sbinfo: *mut fsnotify_sb_info);
}
// Destroy all marks attached to an object via connector
extern "C" {
    pub fn fsnotify_destroy_marks(connp: *mut fsnotify_connp_t);
}
// run the list of all marks associated with inode and destroy them
// run the list of all marks associated with vfsmount and destroy them
// run the list of all marks associated with sb and destroy them
//
// update the dentry->d_flags of all of inode's children to indicate if inode cares
// about events that happen to its children.
//
extern "C" {
    pub fn fsnotify_set_children_dentry_flags(inode: *mut inode);
}
extern "C" {
    pub fn fsnotify_init_connector_caches();
}
