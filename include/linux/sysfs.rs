//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sysfs.h
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
// sysfs.h - definitions for the device driver filesystem
//
// Copyright (c) 2001,2002 Patrick Mochel
// Copyright (c) 2004 Silicon Graphics, Inc.
// Copyright (c) 2007 SUSE Linux Products GmbH
// Copyright (c) 2007 Tejun Heo <teheo@suse.de>
//
// Please see Documentation/filesystems/sysfs.rst for more information.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct attribute {
    pub name: *const c_char,
    pub mode: umode_t,

    pub ignore_lockdep:1: bool,
    pub key: *mut lock_class_key,
    pub skey: lock_class_key,

}

//
// sysfs_attr_init - initialize a dynamically allocated sysfs attribute
// @attr: struct attribute to initialize
//
// Initialize a dynamically allocated struct attribute so we can
// make lockdep happy.  This is a new requirement for attributes
// and initially this is only needed when lockdep is enabled.
// Lockdep gives a nice error when your attribute is added to
// sysfs if you don't have this.
//

//
// struct attribute_group - data structure used to declare an attribute group.
// @name:	Optional: Attribute group name
// If specified, the attribute group will be created in a
// new subdirectory with this name. Additionally when a
// group is named, @is_visible and @is_bin_visible may
// return SYSFS_GROUP_INVISIBLE to control visibility of
// the directory itself.
// @is_visible:	Optional: Function to return permissions associated with an
// attribute of the group. Will be called repeatedly for
// each non-binary attribute in the group. Only read/write
// permissions as well as SYSFS_PREALLOC are accepted. Must
// return 0 if an attribute is not visible. The returned
// value will replace static permissions defined in struct
// attribute. Use SYSFS_GROUP_VISIBLE() when assigning this
// callback to specify separate _group_visible() and
// _attr_visible() handlers.
// @is_bin_visible:
// Optional: Function to return permissions associated with a
// binary attribute of the group. Will be called repeatedly
// for each binary attribute in the group. Only read/write
// permissions as well as SYSFS_PREALLOC (and the
// visibility flags for named groups) are accepted. Must
// return 0 if a binary attribute is not visible. The
// returned value will replace static permissions defined
// in struct bin_attribute. If @is_visible is not set, Use
// SYSFS_GROUP_VISIBLE() when assigning this callback to
// specify separate _group_visible() and _attr_visible()
// handlers.
// @bin_size:
// Optional: Function to return the size of a binary attribute
// of the group. Will be called repeatedly for each binary
// attribute in the group. Overwrites the size field embedded
// inside the attribute itself.
// @attrs:	Pointer to NULL terminated list of attributes.
// @bin_attrs:	Pointer to NULL terminated list of binary attributes.
// Either attrs or bin_attrs or both must be provided.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attribute_group {
    pub name: *const c_char,
    pub int): *mut *mut attribute ,,
    pub int): *const *const attribute ,,
    pub int): *const *const bin_attribute ,,
    pub attrs: *mut attribute,
    pub attrs_const: *const *const attribute,
}

pub const SYSFS_PREALLOC: c_int = 010000;
pub const SYSFS_GROUP_INVISIBLE: c_int = 020000;
//
// DEFINE_SYSFS_GROUP_VISIBLE(name):
// A helper macro to pair with the assignment of ".is_visible =
// SYSFS_GROUP_VISIBLE(name)", that arranges for the directory
// associated with a named attribute_group to optionally be hidden.
// This allows for static declaration of attribute_groups, and the
// simplification of attribute visibility lifetime that implies,
// without polluting sysfs with empty attribute directories.
// Ex.
//
// static umode_t example_attr_visible(struct kobject *kobj,
// struct attribute *attr, int n)
// {
// if (example_attr_condition)
// return 0;
// else if (ro_attr_condition)
// return 0444;
// return a->mode;
// }
//
// static bool example_group_visible(struct kobject *kobj)
// {
// if (example_group_condition)
// return false;
// return true;
// }
//
// DEFINE_SYSFS_GROUP_VISIBLE(example);
//
// static struct attribute_group example_group = {
// .name = "example",
// .is_visible = SYSFS_GROUP_VISIBLE(example),
// .attrs = &example_attrs,
// };
//
// Note that it expects <name>_attr_visible and <name>_group_visible to
// be defined. For cases where individual attributes do not need
// separate visibility consideration, only entire group visibility at
// once, see DEFINE_SIMPLE_SYSFS_GROUP_VISIBLE().
//

//
// DEFINE_SIMPLE_SYSFS_GROUP_VISIBLE(name):
// A helper macro to pair with SYSFS_GROUP_VISIBLE() that like
// DEFINE_SYSFS_GROUP_VISIBLE() controls group visibility, but does
// not require the implementation of a per-attribute visibility
// callback.
// Ex.
//
// static bool example_group_visible(struct kobject *kobj)
// {
// if (example_group_condition)
// return false;
// return true;
// }
//
// DEFINE_SIMPLE_SYSFS_GROUP_VISIBLE(example);
//
// static struct attribute_group example_group = {
// .name = "example",
// .is_visible = SYSFS_GROUP_VISIBLE(example),
// .attrs = &example_attrs,
// };
//

//
// Same as DEFINE_SYSFS_GROUP_VISIBLE, but for groups with only binary
// attributes. If an attribute_group defines both text and binary
// attributes, the group visibility is determined by the function
// specified to is_visible() not is_bin_visible()
//

//
// Use these macros to make defining attributes easier.
// See include/linux/device.h for examples..
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bin_attribute {
    pub attr: attribute,
    pub size: usize,
    pub private: *mut c_void,
    pub (*f_mapping)(void): *mut address_space,
    pub size_t): *mut *mut char , loff_t,,
    pub size_t): *mut *mut char , loff_t,,
    pub int): loff_t,,
    pub vma): *mut vm_area_struct,
}

//
// sysfs_bin_attr_init - initialize a dynamically allocated bin_attribute
// @attr: struct bin_attribute to initialize
//
// Initialize a dynamically allocated struct bin_attribute so we
// can make lockdep happy.  This is a new requirement for
// attributes and initially this is only needed when lockdep is
// enabled.  Lockdep gives a nice error when your attribute is
// added to sysfs if you don't have this.
//

// macros to create static binary attributes easier

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysfs_ops {
    pub ): *mut *mut *mut *mut ssize_t (show)(struct kobject , struct attribute , char,
    pub size_t): *const *const *const *const *const ssize_t (store)(struct kobject , struct attribute , char ,,
}

extern "C" {
    pub fn sysfs_create_dir_ns(kobj: *mut kobject, ns: *const ns_common) -> int __must_check;
}
extern "C" {
    pub fn sysfs_remove_dir(kobj: *mut kobject);
}
extern "C" {
    pub fn sysfs_unbreak_active_protection(kn: *mut kernfs_node);
}
extern "C" {
    pub fn sysfs_remove_file_self(kobj: *mut kobject, attr: *const attribute) -> bool;
}
extern "C" {
    pub fn sysfs_remove_files(kobj: *mut kobject, attr: *const *const attribute);
}
extern "C" {
    pub fn sysfs_remove_link(kobj: *mut kobject, name: *const c_char);
}
extern "C" {
    pub fn sysfs_notify(kobj: *mut kobject, dir: *const c_char, attr: *const c_char);
}
extern "C" {
    pub fn sysfs_init() -> int __must_check;
}
extern "C" {
    pub fn kernfs_enable_ns(_arg: kn) -> return;
}
extern "C" {
    pub fn sysfs_change_owner(kobj: *mut kobject, kuid: kuid_t, kgid: kgid_t) -> c_int;
}
extern "C" {
    pub fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn sysfs_emit_at(buf: *mut c_char, at: c_int, fmt: *const c_char, ...) -> c_int;
}

extern "C" {
    pub fn sysfs_create_file_ns(_arg: kobj, _arg: attr, _arg: NULL) -> return;
}
extern "C" {
    pub fn sysfs_rename_link_ns(_arg: kobj, _arg: target, _arg: old_name, _arg: new_name, _arg: NULL) -> return;
}
extern "C" {
    pub fn kernfs_find_and_get(_arg: parent, _arg: name) -> return;
}
// Permissions on a sysfs file: you didn't miss the 0 prefix did you?

// USER_READABLE >= GROUP_READABLE >= OTHER_READABLE */		\
// USER_WRITABLE >= GROUP_WRITABLE */					\
// OTHER_WRITABLE?  Generally considered a bad idea. */		\
