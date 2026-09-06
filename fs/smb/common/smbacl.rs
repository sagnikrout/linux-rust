//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/common/smbacl.h
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


// SPDX-License-Identifier: LGPL-2.1+
//
// Copyright (c) International Business Machines  Corp., 2007
// Author(s): Steve French (sfrench@us.ibm.com)
// Modified by Namjae Jeon (linkinjeon@kernel.org)
//

// ACE types - see MS-DTYP 2.4.4.1
pub const ACCESS_ALLOWED_ACE_TYPE: c_uint = 0x00;
pub const ACCESS_DENIED_ACE_TYPE: c_uint = 0x01;
pub const SYSTEM_AUDIT_ACE_TYPE: c_uint = 0x02;
pub const SYSTEM_ALARM_ACE_TYPE: c_uint = 0x03;
pub const ACCESS_ALLOWED_COMPOUND_ACE_TYPE: c_uint = 0x04;
pub const ACCESS_ALLOWED_OBJECT_ACE_TYPE: c_uint = 0x05;
pub const ACCESS_DENIED_OBJECT_ACE_TYPE: c_uint = 0x06;
pub const SYSTEM_AUDIT_OBJECT_ACE_TYPE: c_uint = 0x07;
pub const SYSTEM_ALARM_OBJECT_ACE_TYPE: c_uint = 0x08;
pub const ACCESS_ALLOWED_CALLBACK_ACE_TYPE: c_uint = 0x09;
pub const ACCESS_DENIED_CALLBACK_ACE_TYPE: c_uint = 0x0A;
pub const ACCESS_ALLOWED_CALLBACK_OBJECT_ACE_TYPE: c_uint = 0x0B;
pub const ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE: c_uint = 0x0C;
pub const SYSTEM_AUDIT_CALLBACK_ACE_TYPE: c_uint = 0x0D;
pub const SYSTEM_ALARM_CALLBACK_ACE_TYPE: c_uint = 0x0E /* Reserved */;
pub const SYSTEM_AUDIT_CALLBACK_OBJECT_ACE_TYPE: c_uint = 0x0F;
pub const SYSTEM_ALARM_CALLBACK_OBJECT_ACE_TYPE: c_uint = 0x10 /* reserved */;
pub const SYSTEM_MANDATORY_LABEL_ACE_TYPE: c_uint = 0x11;
pub const SYSTEM_RESOURCE_ATTRIBUTE_ACE_TYPE: c_uint = 0x12;
pub const SYSTEM_SCOPED_POLICY_ID_ACE_TYPE: c_uint = 0x13;
// ACE flags
pub const OBJECT_INHERIT_ACE: c_uint = 0x01;
pub const CONTAINER_INHERIT_ACE: c_uint = 0x02;
pub const NO_PROPAGATE_INHERIT_ACE: c_uint = 0x04;
pub const INHERIT_ONLY_ACE: c_uint = 0x08;
pub const INHERITED_ACE: c_uint = 0x10;
pub const SUCCESSFUL_ACCESS_ACE_FLAG: c_uint = 0x40;
pub const FAILED_ACCESS_ACE_FLAG: c_uint = 0x80;
//
// Maximum size of a string representation of a SID:
//
// The fields are unsigned values in decimal. So:
//
// u8:  max 3 bytes in decimal
// u32: max 10 bytes in decimal
//
// "S-" + 3 bytes for version field + 15 for authority field + NULL terminator
//
// For authority field, max is when all 6 values are non-zero and it must be
// represented in hex. So "-0x" + 12 hex digits.
//
// Add 11 bytes for each subauthority field (10 bytes each + 1 for '-')
//

//
// ACE types - see MS-DTYP 2.4.4.1
//
// Security ID types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_ntsd {
    pub /: *mut *mut __le16 revision; / revision level,
    pub type: __le16,
    pub osidoffset: __le32,
    pub gsidoffset: __le32,
    pub sacloffset: __le32,
    pub dacloffset: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_sid {
    pub /: *mut *mut __u8 revision; / revision level,
    pub num_subauth: __u8,
    pub authority: [__u8; NUM_AUTHS],
    pub /: *mut *mut __le32 sub_auth[SID_MAX_SUB_AUTHORITIES]; / sub_auth[num_subauth],
    pub __packed: },
// size of a struct smb_sid, sans sub_auth array

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_acl {
    pub /: *mut *mut __le16 revision; / revision level,
    pub size: __le16,
    pub num_aces: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_ace {
    pub /: *mut *mut __u8 type; / see above and MS-DTYP 2.4.4.1,
    pub flags: __u8,
    pub size: __le16,
    pub access_req: __le32,
    pub /: *mut *mut smb_sid sid; / ie UUID of user or group who gets these perms,
    pub __packed: },
