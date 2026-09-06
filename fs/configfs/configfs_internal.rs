//! Automatically rewritten from C Header to Rust Module
//! Source: fs/configfs/configfs_internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// configfs_internal.h - Internal stuff for configfs
//
// Based on sysfs:
// sysfs is Copyright (C) 2001, 2002, 2003 Patrick Mochel
//
// configfs Copyright (C) 2005 Oracle.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct configfs_fragment {
    pub frag_count: core::sync::atomic::AtomicI32,
    pub frag_sem: rw_semaphore,
    pub frag_dead: bool,
}

extern "C" {
    pub fn put_fragment(: *mut configfs_fragment);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct configfs_dirent {
    pub s_count: core::sync::atomic::AtomicI32,
    pub s_dependent_count: c_int,
    pub s_sibling: list_head,
    pub s_children: list_head,
    pub s_links: c_int,
    pub s_element: *mut *mut c_void,
    pub s_type: c_int,
    pub s_mode: umode_t,
    pub s_dentry: *mut *mut dentry,
    pub s_iattr: *mut *mut iattr,

    pub s_depth: c_int,

    pub s_frag: *mut configfs_fragment,
}

pub const CONFIGFS_ROOT: c_uint = 0x0001;
pub const CONFIGFS_DIR: c_uint = 0x0002;
pub const CONFIGFS_ITEM_ATTR: c_uint = 0x0004;
pub const CONFIGFS_ITEM_BIN_ATTR: c_uint = 0x0008;
pub const CONFIGFS_ITEM_LINK: c_uint = 0x0020;
pub const CONFIGFS_USET_DIR: c_uint = 0x0040;
pub const CONFIGFS_USET_DEFAULT: c_uint = 0x0080;
pub const CONFIGFS_USET_DROPPING: c_uint = 0x0100;
pub const CONFIGFS_USET_IN_MKDIR: c_uint = 0x0200;
pub const CONFIGFS_USET_CREATING: c_uint = 0x0400;

extern "C" {
    pub fn configfs_is_root(item: *mut config_item) -> c_int;
}
extern "C" {
    pub fn configfs_new_inode(mode: umode_t, : *mut configfs_dirent, : *mut super_block) -> *mut inode;
}
extern "C" {
    pub fn configfs_create_file(: *mut config_item, : *const configfs_attribute) -> c_int;
}
extern "C" {
    pub fn configfs_dirent_is_ready(: *mut configfs_dirent) -> c_int;
}
extern "C" {
    pub fn configfs_get_name(sd: *mut configfs_dirent) -> *const c_uchar;
}
extern "C" {
    pub fn configfs_release_fs();
}
extern "C" {
    pub fn configfs_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: attr, configfs_bin_attribute: struct, _arg: cb_attr) -> return;
}
