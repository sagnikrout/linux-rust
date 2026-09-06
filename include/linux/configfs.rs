//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/configfs.h
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
// configfs.h - definitions for the device driver filesystem
//
// Based on sysfs:
// sysfs is Copyright (C) 2001, 2002, 2003 Patrick Mochel
//
// Based on kobject.h:
// Copyright (c) 2002-2003	Patrick Mochel
// Copyright (c) 2002-2003	Open Source Development Labs
//
// configfs Copyright (C) 2005 Oracle.  All rights reserved.
//
// Please read Documentation/filesystems/configfs.rst before using
// the configfs interface, ESPECIALLY the parts about reference counts and
// item destructors.
//

pub const CONFIGFS_ITEM_NAME_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_item {
    pub ci_name: *mut c_char,
    pub ci_namebuf: [c_char; CONFIGFS_ITEM_NAME_LEN],
    pub ci_kref: kref,
    pub ci_entry: list_head,
    pub ci_parent: *mut config_item,
    pub ci_group: *mut config_group,
    pub ci_type: *const config_item_type,
    pub ci_dentry: *mut dentry,
}

extern "C" {
    pub fn config_item_set_name(: *mut config_item, : *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn config_item_put(: *mut config_item);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_item_type {
    pub ct_owner: *mut module,
    pub ct_item_ops: *const configfs_item_operations,
    pub ct_group_ops: *const configfs_group_operations,
    pub ct_attrs: *mut configfs_attribute,
    pub ct_bin_attrs: *mut configfs_bin_attribute,
}

//
// group - a group of config_items of a specific type, belonging
// to a specific subsystem.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_group {
    pub cg_item: config_item,
    pub cg_children: list_head,
    pub cg_subsys: *mut configfs_subsystem,
    pub default_groups: list_head,
    pub group_entry: list_head,
}

extern "C" {
    pub fn config_group_init(group: *mut config_group);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct configfs_attribute {
    pub ca_name: *const c_char,
    pub ca_owner: *mut module,
    pub ca_mode: umode_t,
    pub ): *mut *mut *mut ssize_t (show)(struct config_item , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct config_item , char ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct configfs_bin_attribute {
    pub /: *mut *mut configfs_attribute cb_attr; / std. attribute,
    pub /: *mut *mut *mut void cb_private; / for user,
    pub /: *mut *mut size_t cb_max_size; / max core size,
    pub size_t): *mut *mut *mut *mut ssize_t (read)(struct config_item , void ,,
    pub size_t): *const *const *const *const ssize_t (write)(struct config_item , void ,,
}

//
// If allow_link() exists, the item can symlink(2) out to other
// items.  If the item is a group, it may support mkdir(2).
// Groups supply one of make_group() and make_item().  If the
// group supports make_group(), one can create group children.  If it
// supports make_item(), one can create config_item children.  make_group()
// and make_item() return ERR_PTR() on errors.  If it has
// default_groups on group->default_groups, it has automatically created
// group children.  default_groups may coexist alongsize make_group() or
// make_item(), but if the group wishes to have only default_groups
// children (disallowing mkdir(2)), it need not provide either function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct configfs_item_operations {
    pub ): *mut *mut void (release)(struct config_item,
    pub target): *mut *mut *mut int (allow_link)(struct config_item src, struct config_item,
    pub target): *mut *mut *mut void (drop_link)(struct config_item src, struct config_item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct configfs_group_operations {
    pub name): *const *const *const *const config_item (make_item)(config_group group, char,
    pub name): *const *const *const *const config_group (make_group)(config_group group, char,
    pub item): *mut *mut *mut void (disconnect_notify)(struct config_group group, struct config_item,
    pub item): *mut *mut *mut void (drop_item)(struct config_group group, struct config_item,
    pub n): *mut *mut *mut *mut bool (is_visible)(struct config_item item, struct configfs_attribute attr, int,
    pub n): c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct configfs_subsystem {
    pub su_group: config_group,
    pub su_mutex: mutex,
}

extern "C" {
    pub fn configfs_register_subsystem(subsys: *mut configfs_subsystem) -> c_int;
}
extern "C" {
    pub fn configfs_unregister_subsystem(subsys: *mut configfs_subsystem);
}
extern "C" {
    pub fn configfs_unregister_group(group: *mut config_group);
}
extern "C" {
    pub fn configfs_remove_default_groups(group: *mut config_group);
}
extern "C" {
    pub fn configfs_unregister_default_group(group: *mut config_group);
}
// These functions can sleep and can alloc with GFP_KERNEL
// WARNING: These cannot be called underneath configfs callbacks!!
extern "C" {
    pub fn configfs_undepend_item(target: *mut config_item);
}
//
// These functions can sleep and can alloc with GFP_KERNEL
// NOTE: These should be called only underneath configfs callbacks.
// NOTE: First parameter is a caller's subsystem, not target's.
// WARNING: These cannot be called on newly created item
// (in make_group()/make_item() callback)
//
