//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/common/smbglob.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright (C) International Business Machines  Corp., 2002,2008
// 2018 Samsung Electronics Co., Ltd.
// Author(s): Steve French (sfrench@us.ibm.com)
// Jeremy Allison (jra@samba.org)
// Namjae Jeon (linkinjeon@kernel.org)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_version_values {
    pub version_string: *mut c_char,
    pub protocol_id: __u16,
    pub lock_cmd: __le16,
    pub req_capabilities: __u32,
    pub max_read_size: __u32,
    pub max_write_size: __u32,
    pub max_trans_size: __u32,
    pub max_credits: __u32,
    pub large_lock_type: __u32,
    pub exclusive_lock_type: __u32,
    pub shared_lock_type: __u32,
    pub unlock_lock_type: __u32,
    pub header_size: usize,
    pub max_header_size: usize,
    pub read_rsp_size: usize,
    pub cap_unix: c_uint,
    pub cap_nt_find: c_uint,
    pub cap_large_files: c_uint,
    pub cap_unicode: c_uint,
    pub signing_enabled: __u16,
    pub signing_required: __u16,
    pub create_lease_size: usize,
    pub create_durable_size: usize,
    pub create_durable_v2_size: usize,
    pub create_mxac_size: usize,
    pub create_disk_id_size: usize,
    pub create_posix_size: usize,
    pub create_aapl_size: usize,
}

