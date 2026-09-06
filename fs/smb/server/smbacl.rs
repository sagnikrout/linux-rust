//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/smbacl.h
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

// Revision for ACLs
pub const SD_REVISION: c_int = 1;
// Control flags for Security Descriptor
pub const OWNER_DEFAULTED: c_uint = 0x0001;
pub const GROUP_DEFAULTED: c_uint = 0x0002;
pub const DACL_PRESENT: c_uint = 0x0004;
pub const DACL_DEFAULTED: c_uint = 0x0008;
pub const SACL_PRESENT: c_uint = 0x0010;
pub const SACL_DEFAULTED: c_uint = 0x0020;
pub const DACL_TRUSTED: c_uint = 0x0040;
pub const SERVER_SECURITY: c_uint = 0x0080;
pub const DACL_AUTO_INHERIT_REQ: c_uint = 0x0100;
pub const SACL_AUTO_INHERIT_REQ: c_uint = 0x0200;
pub const DACL_AUTO_INHERITED: c_uint = 0x0400;
pub const SACL_AUTO_INHERITED: c_uint = 0x0800;
pub const DACL_PROTECTED: c_uint = 0x1000;
pub const SACL_PROTECTED: c_uint = 0x2000;
pub const RM_CONTROL_VALID: c_uint = 0x4000;
pub const SELF_RELATIVE: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_fattr {
    pub cf_uid: kuid_t,
    pub cf_gid: kgid_t,
    pub cf_mode: umode_t,
    pub daccess: __le32,
    pub cf_acls: *mut posix_acl,
    pub cf_dacls: *mut posix_acl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_ace_state {
    pub allow: u32,
    pub deny: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_user_ace_state {
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_ace_state_array {
    pub n: c_int,
    pub aces: [posix_user_ace_state; ],
}

//
// while processing the nfsv4 ace, this maintains the partial permissions
// calculated so far:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_acl_state {
    pub owner: posix_ace_state,
    pub group: posix_ace_state,
    pub other: posix_ace_state,
    pub everyone: posix_ace_state,
    pub /: *mut *mut posix_ace_state mask; / deny unused in this case,
    pub users: *mut posix_ace_state_array,
    pub groups: *mut posix_ace_state_array,
}

extern "C" {
    pub fn init_acl_state(state: *mut posix_acl_state, cnt: u16) -> c_int;
}
extern "C" {
    pub fn free_acl_state(state: *mut posix_acl_state);
}
extern "C" {
    pub fn compare_sids(ctsid: *const smb_sid, cwsid: *const smb_sid) -> c_int;
}
extern "C" {
    pub fn smb_inherit_flags(flags: c_int, is_dir: bool) -> bool;
}
extern "C" {
    pub fn id_to_sid(cid: c_uint, sidtype: c_uint, ssid: *mut smb_sid);
}
extern "C" {
    pub fn ksmbd_init_domain(sub_auth: *mut u32);
}
// If this is an idmapped mount, apply the idmapping.
// Translate the kuid into a userspace id ksmbd would see.
extern "C" {
    pub fn from_kuid(_arg: &init_user_ns, _arg: vfsuid_into_kuid(vfsuid)) -> return;
}
// If this is an idmapped mount, apply the idmapping.
// Translate the kgid into a userspace id ksmbd would see.
extern "C" {
    pub fn from_kgid(_arg: &init_user_ns, _arg: vfsgid_into_kgid(vfsgid)) -> return;
}
