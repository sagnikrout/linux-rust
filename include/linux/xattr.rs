//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/xattr.h
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

// List of all open_how "versions".

//
// struct xattr_handler: When @name is set, match attributes with exactly that
// name.  When @prefix is set instead, match attributes with that prefix and
// with a non-empty suffix.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xattr_handler {
    pub name: *const c_char,
    pub prefix: *const c_char,
    pub /: *mut *mut int flags; / fs private flags,
    pub dentry): *mut *mut bool (list)(struct dentry,
    pub size): usize,
    pub flags): size_t size, int,
}

//
// xattr_handler_can_list - check whether xattr can be listed
// @handler: handler for this type of xattr
// @dentry: dentry whose inode xattr to list
//
// Determine whether the xattr associated with @dentry can be listed given
// @handler.
//
// Return: true if xattr can be listed, false if not.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xattr {
    pub name: *const c_char,
    pub value: *mut c_void,
    pub value_len: usize,
}

extern "C" {
    pub fn __vfs_getxattr(: *mut dentry, : *mut inode, : *const c_char, : *mut c_void, _arg: usize) -> isize;
}
extern "C" {
    pub fn vfs_listxattr(d: *mut dentry, list: *mut c_char, size: usize) -> isize;
}
extern "C" {
    pub fn __vfs_removexattr(: *mut mnt_idmap, : *mut dentry, : *const c_char) -> c_int;
}
extern "C" {
    pub fn vfs_removexattr(: *mut mnt_idmap, : *mut dentry, : *const c_char) -> c_int;
}
extern "C" {
    pub fn generic_listxattr(dentry: *mut dentry, buffer: *mut c_char, buffer_size: usize) -> isize;
}
extern "C" {
    pub fn xattr_supports_user_prefix(inode: *mut inode) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_xattr_cache {
    pub ht: *mut rhashtable,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_xattr {
    pub hash_node: rhash_head,
    pub parent: *mut list_head,
    pub node: list_head,
    pub rcu: rcu_head,
    pub name: *mut c_char,
    pub size: usize,
    pub __counted_by(size): char value[],
}

pub const SIMPLE_XATTR_MAX_NR: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_xattr_limits {
    pub /: *mut *mut *mut atomic_t nr_xattrs; / current user. xattr count,
    pub /: *mut *mut *mut atomic_t xattr_size; / current total user. value bytes,
}

extern "C" {
    pub fn simple_xattr_space(name: *const c_char, size: usize) -> usize;
}
extern "C" {
    pub fn simple_xattr_free(xattr: *mut simple_xattr);
}
extern "C" {
    pub fn simple_xattr_free_rcu(xattr: *mut simple_xattr);
}
extern "C" {
    pub fn xattr_list_one(buffer: *mut c_char, remaining_size: *mut isize, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn simple_xattr_cache_cleanup(cache: *mut simple_xattr_cache);
}
