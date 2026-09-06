//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sed-opal.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright © 2016 Intel Corporation
//
// Authors:
// Rafael Antognolli <rafael.antognolli@intel.com>
// Scott  Bauer      <scott.bauer@intel.com>
//

pub const OPAL_KEY_MAX: c_int = 256;
pub const OPAL_MAX_LRS: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_mbr {
    OPAL_MBR_ENABLE = 0x0,
    OPAL_MBR_DISABLE = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_mbr_done_flag {
    OPAL_MBR_NOT_DONE = 0x0,
    OPAL_MBR_DONE = 0x01
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_user {
    OPAL_ADMIN1 = 0x0,
    OPAL_USER1 = 0x01,
    OPAL_USER2 = 0x02,
    OPAL_USER3 = 0x03,
    OPAL_USER4 = 0x04,
    OPAL_USER5 = 0x05,
    OPAL_USER6 = 0x06,
    OPAL_USER7 = 0x07,
    OPAL_USER8 = 0x08,
    OPAL_USER9 = 0x09,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_lock_state {
    OPAL_RO = 0x01, /* 0001 */
    OPAL_RW = 0x02, /* 0010 */
    OPAL_LK = 0x04, /* 0100 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_lock_flags {
// IOC_OPAL_SAVE will also store the provided key for locking
    OPAL_SAVE_FOR_LOCK = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_key_type {
    OPAL_INCLUDED = 0,	/* key[] is the key */
    OPAL_KEYRING,		/* key is in keyring */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_key {
    pub lr: __u8,
    pub key_len: __u8,
    pub key_type: __u8,
    pub __align: [__u8; 5],
    pub key: [__u8; OPAL_KEY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_revert_lsp_opts {
    OPAL_PRESERVE = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_lr_act {
    pub key: opal_key,
    pub sum: __u32,
    pub num_lrs: __u8,
    pub lr: [__u8; OPAL_MAX_LRS],
    pub /: *mut *mut __u8 align[2]; / Align to 8 byte boundary,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_lr_react {
    pub key: opal_key,
    pub /: *mut *mut opal_key new_admin_key; / Set new Admin1 PIN if key_len is > 0,
    pub /*: *mut __u8 num_lrs;,
// Configure selected ranges (from lr[]) in SUM.
// If num_lrs > 0 the 'entire_table' must be 0
//
    pub lr: [__u8; OPAL_MAX_LRS],
    pub /: *mut *mut __u8 range_policy; / Set RangeStartRangeLengthPolicy parameter,
    pub /: *mut *mut __u8 entire_table; / Set all locking objects in SUM,
    pub /: *mut *mut __u8 align[4]; / Align to 8 byte boundary,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_session_info {
    pub sum: __u32,
    pub who: __u32,
    pub opal_key: opal_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_user_lr_setup {
    pub range_start: __u64,
    pub range_length: __u64,
    pub /: *mut *mut __u32 RLE; / Read Lock enabled,
    pub /: *mut *mut __u32 WLE; / Write Lock Enabled,
    pub session: opal_session_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_lr_status {
    pub session: opal_session_info,
    pub range_start: __u64,
    pub range_length: __u64,
    pub /: *mut *mut __u32 RLE; / Read Lock enabled,
    pub /: *mut *mut __u32 WLE; / Write Lock Enabled,
    pub l_state: __u32,
    pub align: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_sum_ranges {
//
// Initiate Admin1 session if key_len > 0,
// use Anybody session otherwise.
//
    pub key: opal_key,
    pub num_lrs: __u8,
    pub lr: [__u8; OPAL_MAX_LRS],
    pub range_policy: __u8,
    pub /: *mut *mut __u8 align[5]; / Align to 8 byte boundary,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_lock_unlock {
    pub session: opal_session_info,
    pub l_state: __u32,
    pub flags: __u16,
    pub __align: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_new_pw {
    pub session: opal_session_info,
// When we're not operating in sum, and we first set
// passwords we need to set them via ADMIN authority.
// After passwords are changed, we can set them via,
// User authorities.
// Because of this restriction we need to know about
// Two different users. One in 'session' which we will use
// to start the session and new_userr_pw as the user we're
// chaning the pw for.
//
    pub new_user_pw: opal_session_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_mbr_data {
    pub key: opal_key,
    pub enable_disable: __u8,
    pub __align: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_mbr_done {
    pub key: opal_key,
    pub done_flag: __u8,
    pub __align: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_shadow_mbr {
    pub key: opal_key,
    pub data: __u64,
    pub offset: __u64,
    pub size: __u64,
}

// Opal table operations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_table_ops {
    OPAL_READ_TABLE,
    OPAL_WRITE_TABLE,
}

pub const OPAL_UID_LENGTH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_read_write_table {
    pub key: opal_key,
    pub data: __u64,
    pub table_uid: [__u8; OPAL_UID_LENGTH],
    pub offset: __u64,
    pub size: __u64,

    pub flags: __u64,
    pub priv: __u64,
}

pub const OPAL_FL_SUPPORTED: c_uint = 0x00000001;
pub const OPAL_FL_LOCKING_SUPPORTED: c_uint = 0x00000002;
pub const OPAL_FL_LOCKING_ENABLED: c_uint = 0x00000004;
pub const OPAL_FL_LOCKED: c_uint = 0x00000008;
pub const OPAL_FL_MBR_ENABLED: c_uint = 0x00000010;
pub const OPAL_FL_MBR_DONE: c_uint = 0x00000020;
pub const OPAL_FL_SUM_SUPPORTED: c_uint = 0x00000040;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_status {
    pub flags: __u32,
    pub reserved: __u32,
}

//
// Geometry Reporting per TCG Storage OPAL SSC
// section 3.1.1.4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_geometry {
    pub align: __u8,
    pub logical_block_size: __u32,
    pub alignment_granularity: __u64,
    pub lowest_aligned_lba: __u64,
    pub __align: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_discovery {
    pub data: __u64,
    pub size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_revert_lsp {
    pub key: opal_key,
    pub options: __u32,
    pub __pad: __u32,
}

