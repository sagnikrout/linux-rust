//! Automatically rewritten from C Header to Rust Module
//! Source: fs/orangefs/downcall.h
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
// (C) 2001 Clemson University and The University of Chicago
//
// See COPYING in top-level directory.
//
// Definitions of downcalls used in Linux kernel module.
//
// Sanitized the device-client core interaction
// for clean 32-64 bit usage
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_io_response {
    pub amt_complete: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_lookup_response {
    pub refn: orangefs_object_kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_create_response {
    pub refn: orangefs_object_kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_symlink_response {
    pub refn: orangefs_object_kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_getattr_response {
    pub attributes: ORANGEFS_sys_attr_s,
    pub link_target: [c_char; ORANGEFS_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_mkdir_response {
    pub refn: orangefs_object_kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_statfs_response {
    pub block_size: __s64,
    pub blocks_total: __s64,
    pub blocks_avail: __s64,
    pub files_total: __s64,
    pub files_avail: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_fs_mount_response {
    pub fs_id: __s32,
    pub id: __s32,
    pub root_khandle: orangefs_khandle,
}

// the getxattr response is the attribute value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_getxattr_response {
    pub val_sz: __s32,
    pub __pad1: __s32,
    pub val: [c_char; ORANGEFS_MAX_XATTR_VALUELEN],
}

// the listxattr response is an array of attribute names
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_listxattr_response {
    pub returned_count: __s32,
    pub __pad1: __s32,
    pub token: __u64,
    pub ORANGEFS_MAX_XATTR_NAMELEN]: *mut *mut char key[ORANGEFS_MAX_XATTR_LISTLEN,
    pub keylen: __s32,
    pub __pad2: __s32,
    pub lengths: [__s32; ORANGEFS_MAX_XATTR_LISTLEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_param_response {
    pub value64: __s64,
    pub value32: [__s32; 2],
    pub u: },
}

pub const PERF_COUNT_BUF_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_perf_count_response {
    pub buffer: [c_char; PERF_COUNT_BUF_SIZE],
}

pub const FS_KEY_BUF_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_fs_key_response {
    pub fs_keylen: __s32,
    pub __pad1: __s32,
    pub fs_key: [c_char; FS_KEY_BUF_SIZE],
}

// 2.9.6
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_features_response {
    pub features: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_downcall_s {
    pub type: __s32,
    pub status: __s32,
// currently trailer is used only by readdir
    pub trailer_size: __s64,
    pub trailer_buf: *mut c_char,
    pub io: orangefs_io_response,
    pub lookup: orangefs_lookup_response,
    pub create: orangefs_create_response,
    pub sym: orangefs_symlink_response,
    pub getattr: orangefs_getattr_response,
    pub mkdir: orangefs_mkdir_response,
    pub statfs: orangefs_statfs_response,
    pub fs_mount: orangefs_fs_mount_response,
    pub getxattr: orangefs_getxattr_response,
    pub listxattr: orangefs_listxattr_response,
    pub param: orangefs_param_response,
    pub perf_count: orangefs_perf_count_response,
    pub fs_key: orangefs_fs_key_response,
    pub features: orangefs_features_response,
    pub resp: },
}

//
// The readdir response comes in the trailer.  It is followed by the
// directory entries as described in dir.c.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_readdir_response_s {
    pub token: __u64,
    pub directory_version: __u64,
    pub __pad2: __u32,
    pub orangefs_dirent_outcount: __u32,
}
