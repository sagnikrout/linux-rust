//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/xattr.h
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
// Copyright (C) 2021 Samsung Electronics Co., Ltd.
//
// These are on-disk structures to store additional metadata into xattr to
// reproduce windows filesystem semantics. And they are encoded with NDR to
// compatible with samba's xattr meta format. The compatibility with samba
// is important because it can lose the information(file attribute,
// creation time, acls) about the existing files when switching between
// ksmbd and samba.
//
// Dos attribute flags used for what variable is valid.
//
// Dos attribute structure which is compatible with samba's one.
// Storing it into the xattr named "DOSATTRIB" separately from inode
// allows ksmbd to faithfully reproduce windows filesystem semantics
// on top of a POSIX filesystem.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xattr_dos_attrib {
    pub /: *mut *mut __u16 version; / version 3 or version 4,
    pub /: *mut *mut __u32 flags; / valid flags,
    pub /: *mut *mut __u32 attr; / Dos attribute,
    pub /: *mut *mut __u32 ea_size; / EA size,
    pub size: __u64,
    pub alloc_size: __u64,
    pub /: *mut *mut __u64 create_time; / File creation time,
    pub /: *mut *mut __u64 change_time; / File change time,
    pub /: *mut *mut __u64 itime; / Invented/Initial time,
}

//
// Enumeration is used for computing posix acl hash.
//
pub const SMB_ACL_READ: c_int = 4;
pub const SMB_ACL_WRITE: c_int = 2;
pub const SMB_ACL_EXECUTE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xattr_acl_entry {
    pub type: c_int,
    pub uid: uid_t,
    pub gid: gid_t,
    pub perm: mode_t,
}

//
// xattr_smb_acl structure is used for computing posix acl hash.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xattr_smb_acl {
    pub count: c_int,
    pub next: c_int,
    pub __counted_by(count): xattr_acl_entry entries[],
}

// 64bytes hash in xattr_ntacl is computed with sha256
pub const XATTR_SD_HASH_TYPE_SHA256: c_uint = 0x1;
pub const XATTR_SD_HASH_SIZE: c_int = 64;
//
// xattr_ntacl is used for storing ntacl and hashes.
// Hash is used for checking valid posix acl and ntacl in xattr.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xattr_ntacl {
    pub 4*/: *mut *mut __u16 version; / version,
    pub sd_buf: *mut c_void,
    pub sd_size: __u32,
    pub /: *mut *mut __u16 hash_type; / hash type,
    pub /: *mut *mut __u8 desc[10]; / posix_acl description,
    pub desc_len: __u16,
    pub current_time: __u64,
    pub /: *mut *mut __u8 hash[XATTR_SD_HASH_SIZE]; / 64bytes hash for ntacl,
    pub /: *mut *mut __u8 posix_acl_hash[XATTR_SD_HASH_SIZE]; / 64bytes hash for posix acl,
}

// DOS ATTRIBUTE XATTR PREFIX

// STREAM XATTR PREFIX

// SECURITY DESCRIPTOR(NTACL) XATTR PREFIX

