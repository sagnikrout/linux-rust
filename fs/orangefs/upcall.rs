//! Automatically rewritten from C Header to Rust Module
//! Source: fs/orangefs/upcall.h
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
// Sanitized this header file to fix
// 32-64 bit interaction issues between
// client-core and device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_io_request_s {
    pub __pad1: __s32,
    pub buf_index: __s32,
    pub count: __s32,
    pub __pad2: __s32,
    pub offset: __s64,
    pub refn: orangefs_object_kref,
    pub io_type: ORANGEFS_io_type,
    pub readahead_size: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_lookup_request_s {
    pub sym_follow: __s32,
    pub __pad1: __s32,
    pub parent_refn: orangefs_object_kref,
    pub d_name: [c_char; ORANGEFS_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_create_request_s {
    pub parent_refn: orangefs_object_kref,
    pub attributes: ORANGEFS_sys_attr_s,
    pub d_name: [c_char; ORANGEFS_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_symlink_request_s {
    pub parent_refn: orangefs_object_kref,
    pub attributes: ORANGEFS_sys_attr_s,
    pub entry_name: [c_char; ORANGEFS_NAME_MAX],
    pub target: [c_char; ORANGEFS_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_getattr_request_s {
    pub refn: orangefs_object_kref,
    pub mask: __u32,
    pub __pad1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_setattr_request_s {
    pub refn: orangefs_object_kref,
    pub attributes: ORANGEFS_sys_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_remove_request_s {
    pub parent_refn: orangefs_object_kref,
    pub d_name: [c_char; ORANGEFS_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_mkdir_request_s {
    pub parent_refn: orangefs_object_kref,
    pub attributes: ORANGEFS_sys_attr_s,
    pub d_name: [c_char; ORANGEFS_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_readdir_request_s {
    pub refn: orangefs_object_kref,
    pub token: __u64,
    pub max_dirent_count: __s32,
    pub buf_index: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_readdirplus_request_s {
    pub refn: orangefs_object_kref,
    pub token: __u64,
    pub max_dirent_count: __s32,
    pub mask: __u32,
    pub buf_index: __s32,
    pub __pad1: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_rename_request_s {
    pub old_parent_refn: orangefs_object_kref,
    pub new_parent_refn: orangefs_object_kref,
    pub d_old_name: [c_char; ORANGEFS_NAME_MAX],
    pub d_new_name: [c_char; ORANGEFS_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_statfs_request_s {
    pub fs_id: __s32,
    pub __pad1: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_truncate_request_s {
    pub refn: orangefs_object_kref,
    pub size: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_ra_cache_flush_request_s {
    pub refn: orangefs_object_kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_fs_mount_request_s {
    pub orangefs_config_server: [c_char; ORANGEFS_MAX_SERVER_ADDR_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_fs_umount_request_s {
    pub id: __s32,
    pub fs_id: __s32,
    pub orangefs_config_server: [c_char; ORANGEFS_MAX_SERVER_ADDR_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_getxattr_request_s {
    pub refn: orangefs_object_kref,
    pub key_sz: __s32,
    pub __pad1: __s32,
    pub key: [c_char; ORANGEFS_MAX_XATTR_NAMELEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_setxattr_request_s {
    pub refn: orangefs_object_kref,
    pub keyval: ORANGEFS_keyval_pair,
    pub flags: __s32,
    pub __pad1: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_listxattr_request_s {
    pub refn: orangefs_object_kref,
    pub requested_count: __s32,
    pub __pad1: __s32,
    pub token: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_removexattr_request_s {
    pub refn: orangefs_object_kref,
    pub key_sz: __s32,
    pub __pad1: __s32,
    pub key: [c_char; ORANGEFS_MAX_XATTR_NAMELEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_op_cancel_s {
    pub op_tag: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_fsync_request_s {
    pub refn: orangefs_object_kref,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum orangefs_param_request_type {
    ORANGEFS_PARAM_REQUEST_SET = 1,
    ORANGEFS_PARAM_REQUEST_GET = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum orangefs_param_request_op {
    ORANGEFS_PARAM_REQUEST_OP_ACACHE_TIMEOUT_MSECS = 1,
    ORANGEFS_PARAM_REQUEST_OP_ACACHE_HARD_LIMIT = 2,
    ORANGEFS_PARAM_REQUEST_OP_ACACHE_SOFT_LIMIT = 3,
    ORANGEFS_PARAM_REQUEST_OP_ACACHE_RECLAIM_PERCENTAGE = 4,
    ORANGEFS_PARAM_REQUEST_OP_PERF_TIME_INTERVAL_SECS = 5,
    ORANGEFS_PARAM_REQUEST_OP_PERF_HISTORY_SIZE = 6,
    ORANGEFS_PARAM_REQUEST_OP_PERF_RESET = 7,
    ORANGEFS_PARAM_REQUEST_OP_NCACHE_TIMEOUT_MSECS = 8,
    ORANGEFS_PARAM_REQUEST_OP_NCACHE_HARD_LIMIT = 9,
    ORANGEFS_PARAM_REQUEST_OP_NCACHE_SOFT_LIMIT = 10,
    ORANGEFS_PARAM_REQUEST_OP_NCACHE_RECLAIM_PERCENTAGE = 11,
    ORANGEFS_PARAM_REQUEST_OP_STATIC_ACACHE_TIMEOUT_MSECS = 12,
    ORANGEFS_PARAM_REQUEST_OP_STATIC_ACACHE_HARD_LIMIT = 13,
    ORANGEFS_PARAM_REQUEST_OP_STATIC_ACACHE_SOFT_LIMIT = 14,
    ORANGEFS_PARAM_REQUEST_OP_STATIC_ACACHE_RECLAIM_PERCENTAGE = 15,
    ORANGEFS_PARAM_REQUEST_OP_CLIENT_DEBUG = 16,
    ORANGEFS_PARAM_REQUEST_OP_CCACHE_TIMEOUT_SECS = 17,
    ORANGEFS_PARAM_REQUEST_OP_CCACHE_HARD_LIMIT = 18,
    ORANGEFS_PARAM_REQUEST_OP_CCACHE_SOFT_LIMIT = 19,
    ORANGEFS_PARAM_REQUEST_OP_CCACHE_RECLAIM_PERCENTAGE = 20,
    ORANGEFS_PARAM_REQUEST_OP_CAPCACHE_TIMEOUT_SECS = 21,
    ORANGEFS_PARAM_REQUEST_OP_CAPCACHE_HARD_LIMIT = 22,
    ORANGEFS_PARAM_REQUEST_OP_CAPCACHE_SOFT_LIMIT = 23,
    ORANGEFS_PARAM_REQUEST_OP_CAPCACHE_RECLAIM_PERCENTAGE = 24,
    ORANGEFS_PARAM_REQUEST_OP_TWO_MASK_VALUES = 25,
    ORANGEFS_PARAM_REQUEST_OP_READAHEAD_SIZE = 26,
    ORANGEFS_PARAM_REQUEST_OP_READAHEAD_COUNT = 27,
    ORANGEFS_PARAM_REQUEST_OP_READAHEAD_COUNT_SIZE = 28,
    ORANGEFS_PARAM_REQUEST_OP_READAHEAD_READCNT = 29,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_param_request_s {
    pub type: orangefs_param_request_type,
    pub op: orangefs_param_request_op,
    pub value64: __s64,
    pub value32: [__s32; 2],
    pub u: },
    pub s_value: [c_char; ORANGEFS_MAX_DEBUG_STRING_LEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum orangefs_perf_count_request_type {
    ORANGEFS_PERF_COUNT_REQUEST_ACACHE = 1,
    ORANGEFS_PERF_COUNT_REQUEST_NCACHE = 2,
    ORANGEFS_PERF_COUNT_REQUEST_CAPCACHE = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_perf_count_request_s {
    pub type: orangefs_perf_count_request_type,
    pub __pad1: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_fs_key_request_s {
    pub fsid: __s32,
    pub __pad1: __s32,
}

// 2.9.6
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_features_request_s {
    pub features: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_upcall_s {
    pub type: __s32,
    pub uid: __u32,
    pub gid: __u32,
    pub pid: c_int,
    pub tgid: c_int,
// Trailers unused but must be retained for protocol compatibility.
    pub trailer_size: __s64,
    pub trailer_buf: *mut c_char,
    pub io: orangefs_io_request_s,
    pub lookup: orangefs_lookup_request_s,
    pub create: orangefs_create_request_s,
    pub sym: orangefs_symlink_request_s,
    pub getattr: orangefs_getattr_request_s,
    pub setattr: orangefs_setattr_request_s,
    pub remove: orangefs_remove_request_s,
    pub mkdir: orangefs_mkdir_request_s,
    pub readdir: orangefs_readdir_request_s,
    pub readdirplus: orangefs_readdirplus_request_s,
    pub rename: orangefs_rename_request_s,
    pub statfs: orangefs_statfs_request_s,
    pub truncate: orangefs_truncate_request_s,
    pub ra_cache_flush: orangefs_ra_cache_flush_request_s,
    pub fs_mount: orangefs_fs_mount_request_s,
    pub fs_umount: orangefs_fs_umount_request_s,
    pub getxattr: orangefs_getxattr_request_s,
    pub setxattr: orangefs_setxattr_request_s,
    pub listxattr: orangefs_listxattr_request_s,
    pub removexattr: orangefs_removexattr_request_s,
    pub cancel: orangefs_op_cancel_s,
    pub fsync: orangefs_fsync_request_s,
    pub param: orangefs_param_request_s,
    pub perf_count: orangefs_perf_count_request_s,
    pub fs_key: orangefs_fs_key_request_s,
    pub features: orangefs_features_request_s,
    pub req: },
}
