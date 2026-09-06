//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fs_context.h
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
// Filesystem superblock creation and reconfiguration context.
//
// Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fs_context_purpose {
    FS_CONTEXT_FOR_MOUNT,		/* New superblock for explicit mount */
    FS_CONTEXT_FOR_SUBMOUNT,	/* New superblock for automatic submount */
    FS_CONTEXT_FOR_RECONFIGURE,	/* Superblock reconfiguration (remount) */
}

//
// Userspace usage phase for fsopen/fspick.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fs_context_phase {
    FS_CONTEXT_CREATE_PARAMS,	/* Loading params for sb creation */
    FS_CONTEXT_CREATING,		/* A superblock is being created */
    FS_CONTEXT_AWAITING_MOUNT,	/* Superblock created, awaiting fsmount() */
    FS_CONTEXT_AWAITING_RECONF,	/* Awaiting initialisation for reconfiguration */
    FS_CONTEXT_RECONF_PARAMS,	/* Loading params for reconfiguration */
    FS_CONTEXT_RECONFIGURING,	/* Reconfiguring the superblock */
    FS_CONTEXT_FAILED,		/* Failed to correctly transition a context */
}

//
// Type of parameter value.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fs_value_type {
    fs_value_is_undefined,
    fs_value_is_flag,		/* Value not given a value */
    fs_value_is_string,		/* Value is a string */
    fs_value_is_blob,		/* Value is a binary blob */
    fs_value_is_filename,		/* Value is a filename* + dirfd */
    fs_value_is_file,		/* Value is a file* */
}

//
// Configuration parameter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_parameter {
    pub /: *const *const *const char key; / Parameter name,
    pub /: *mut *mut fs_value_type type:8; / The type of value here,
    pub string: *mut c_char,
    pub blob: *mut c_void,
    pub name: *mut filename,
    pub file: *mut file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_log {
    pub prefix: *const c_char,
    pub log: *mut fc_log,
}

//
// Filesystem context for holding the parameters used in the creation or
// reconfiguration of a superblock.
//
// Superblock creation fills in ->root whereas reconfiguration begins with this
// already set.
//
// See Documentation/filesystems/mount_api.rst
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_context {
    pub ops: *const fs_context_operations,
    pub /: *mut *mut mutex uapi_mutex; / Userspace access mutex,
    pub fs_type: *mut file_system_type,
    pub /: *mut *mut *mut void fs_private; / The filesystem's context,
    pub sget_key: *mut c_void,
    pub /: *mut *mut *mut dentry root; / The root and superblock,
    pub /: *mut *mut *mut user_namespace user_ns; / The user namespace for this mount,
    pub /: *mut *mut *mut net net_ns; / The network namespace for this mount,
    pub /: *const *const *const cred cred; / The mounter's credentials,
    pub /: *mut *mut p_log log; / Logging buffer,
    pub /: *const *const *const char source; / The source name (eg. dev path),
    pub /: *mut *mut *mut void security; / LSM options,
    pub /: *mut *mut *mut void s_fs_info; / Proposed s_fs_info,
    pub /: *mut *mut *mut unsigned int sb_flags; / Proposed superblock flags (SB_),
    pub /: *mut *mut unsigned int sb_flags_mask; / Superblock flags that were changed,
    pub /: *mut *mut unsigned int s_iflags; / OR'd with sb->s_iflags,
    pub purpose:8: fs_context_purpose,
    pub /: *mut *mut fs_context_phase phase:8; / The phase the context is in,
    pub /: *mut *mut bool need_free:1; / Need to call ops->free(),
    pub /: *mut *mut bool global:1; / Goes into &init_user_ns,
    pub /: *mut *mut bool oldapi:1; / Coming from mount(2),
    pub /: *mut *mut bool exclusive:1; / create new superblock, reject existing one,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_context_operations {
    pub fc): *mut *mut void (free)(struct fs_context,
    pub src_fc): *mut *mut *mut int (dup)(struct fs_context fc, struct fs_context,
    pub param): *mut *mut *mut int (parse_param)(struct fs_context fc, struct fs_parameter,
    pub data): *mut *mut *mut int (parse_monolithic)(struct fs_context fc, void,
    pub fc): *mut *mut int (get_tree)(struct fs_context,
    pub fc): *mut *mut int (reconfigure)(struct fs_context,
}

//
// fs_context manipulation functions.
//
extern "C" {
    pub fn vfs_parse_fs_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
}
extern "C" {
    pub fn vfs_parse_fs_qstr(_arg: fc, _arg: key, NULL: value ? &QSTR(value) :) -> return;
}
extern "C" {
    pub fn generic_parse_monolithic(fc: *mut fs_context, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn vfs_get_tree(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn put_fs_context(fc: *mut fs_context);
}
extern "C" {
    pub fn fc_drop_locked(fc: *mut fs_context);
}
pub const GET_TREE_BDEV_QUIET_LOOKUP: c_uint = 0x0001;
//
// Mount error, warning and informational message logging.  This structure is
// shareable between a mount and a subordinate mount.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_log {
    pub usage: refcount_t,
    pub /: *mut *mut u8 head; / Insertion index in buffer[],
    pub /: *mut *mut u8 tail; / Removal index in buffer[],
    pub /: *mut *mut u8 need_free; / Mask of kfree'able items in buffer[],
    pub /: *mut *mut *mut module owner; / Owner module for strings that don't then need freeing,
    pub buffer: [*mut c_char; 8],
}

extern "C" {
    pub fn logfc(log: *mut fc_log, prefix: *const c_char, level: c_char, fmt: *const c_char, ...);
}

//
// infof - Store supplementary informational message
// @fc: The context in which to log the informational message
// @fmt: The format string
//
// Store the supplementary informational message for the process if the process
// has enabled the facility.
//

//
// warnf - Store supplementary warning message
// @fc: The context in which to log the error message
// @fmt: The format string
//
// Store the supplementary warning message for the process if the process has
// enabled the facility.
//

//
// errorf - Store supplementary error message
// @fc: The context in which to log the error message
// @fmt: The format string
//
// Store the supplementary error message for the process if the process has
// enabled the facility.
//

//
// invalf - Store supplementary invalid argument error message
// @fc: The context in which to log the error message
// @fmt: The format string
//
// Store the supplementary error message for the process if the process has
// enabled the facility and return -EINVAL.
//

