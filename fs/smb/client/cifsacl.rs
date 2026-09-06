//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cifsacl.h
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
// Copyright (c) International Business Machines  Corp., 2007
// Author(s): Steve French (sfrench@us.ibm.com)
//

pub const READ_BIT: c_uint = 0x4;
pub const WRITE_BIT: c_uint = 0x2;
pub const EXEC_BIT: c_uint = 0x1;
pub const ACL_OWNER_MASK: c_int = 0700;
pub const ACL_GROUP_MASK: c_int = 0070;
pub const ACL_EVERYONE_MASK: c_int = 0007;
pub const UBITSHIFT: c_int = 6;
pub const GBITSHIFT: c_int = 3;
//
// Security Descriptor length containing DACL with 3 ACEs (one each for
// owner, group and world).
//

//
// The current SMB3 form of security descriptor is similar to what was used for
// cifs (see above) but some fields are split, and fields in the struct below
// matches names of fields to the spec, MS-DTYP (see sections 2.4.5 and
// 2.4.6). Note that "CamelCase" fields are used in this struct in order to
// match the MS-DTYP and MS-SMB2 specs which define the wire format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_sd {
    pub /: *mut *mut __u8 Revision; / revision level, MUST be one,
    pub /: *mut *mut __u8 Sbz1; / only meaningful if 'RM' flag set below,
    pub Control: __le16,
    pub OffsetOwner: __le32,
    pub OffsetGroup: __le32,
    pub OffsetSacl: __le32,
    pub OffsetDacl: __le32,
    pub __packed: },
// Meaning of 'Control' field flags
pub const ACL_CONTROL_SR: c_uint = 0x8000	/* Self relative */;
pub const ACL_CONTROL_RM: c_uint = 0x4000	/* Resource manager control bits */;
pub const ACL_CONTROL_PS: c_uint = 0x2000	/* SACL protected from inherits */;
pub const ACL_CONTROL_PD: c_uint = 0x1000	/* DACL protected from inherits */;
pub const ACL_CONTROL_SI: c_uint = 0x0800	/* SACL Auto-Inherited */;
pub const ACL_CONTROL_DI: c_uint = 0x0400	/* DACL Auto-Inherited */;
pub const ACL_CONTROL_SC: c_uint = 0x0200	/* SACL computed through inheritance */;
pub const ACL_CONTROL_DC: c_uint = 0x0100	/* DACL computed through inheritance */;
pub const ACL_CONTROL_SS: c_uint = 0x0080	/* Create server ACL */;
pub const ACL_CONTROL_DT: c_uint = 0x0040	/* DACL provided by trusted source */;
pub const ACL_CONTROL_SD: c_uint = 0x0020	/* SACL defaulted */;
pub const ACL_CONTROL_SP: c_uint = 0x0010	/* SACL is present on object */;
pub const ACL_CONTROL_DD: c_uint = 0x0008	/* DACL defaulted */;
pub const ACL_CONTROL_DP: c_uint = 0x0004	/* DACL is present on object */;
pub const ACL_CONTROL_GD: c_uint = 0x0002	/* Group was defaulted */;
pub const ACL_CONTROL_OD: c_uint = 0x0001	/* User was defaulted */;
// Meaning of AclRevision flags
pub const ACL_REVISION: c_uint = 0x02 /* See section 2.4.4.1 of MS-DTYP */;
pub const ACL_REVISION_DS: c_uint = 0x04 /* Additional AceTypes allowed */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_acl {
    pub /: *mut *mut u8 AclRevision; / revision level,
    pub /: *mut *mut u8 Sbz1; / MBZ,
    pub AclSize: __le16,
    pub AceCount: __le16,
    pub /: *mut *mut __le16 Sbz2; / MBZ,
    pub __packed: },
//
// Used to store the special 'NFS SIDs' used to persist the POSIX uid and gid
// See http://technet.microsoft.com/en-us/library/hh509017(v=ws.10).aspx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owner_sid {
    pub Revision: u8,
    pub NumAuth: u8,
    pub Authority: [u8; 6],
    pub SubAuthorities: [__le32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owner_group_sids {
    pub owner: owner_sid,
    pub group: owner_sid,
    pub __packed: },
//
// Minimum security identifier can be one for system defined Users
// and Groups such as NULL SID and World or Built-in accounts such
// as Administrator and Guest and consists of
// Revision + Num (Sub)Auths + Authority + Domain (one Subauthority)
//

//
// Minimum security descriptor can be one without any SACL and DACL and can
// consist of revision, type, and two sids of minimum size for owner and group
//

