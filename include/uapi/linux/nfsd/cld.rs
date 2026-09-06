//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfsd/cld.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Upcall description for nfsdcld communication
//
// Copyright (c) 2012 Red Hat, Inc.
// Author(s): Jeff Layton <jlayton@redhat.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
//

// latest upcall version available
pub const CLD_UPCALL_VERSION: c_int = 2;
// defined by RFC3530
pub const NFS4_OPAQUE_LIMIT: c_int = 1024;

pub const SHA256_DIGEST_SIZE: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cld_command {
    Cld_Create,		/* create a record for this cm_id */
    Cld_Remove,		/* remove record of this cm_id */
    Cld_Check,		/* is this cm_id allowed? */
    Cld_GraceDone,		/* grace period is complete */
    Cld_GraceStart,		/* grace start (upload client records) */
    Cld_GetVersion,		/* query max supported upcall version */
}

// representation of long-form NFSv4 client ID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cld_name {
    pub /: *mut *mut __u16 cn_len; / length of cm_id,
    pub /: *mut *mut unsigned char cn_id[NFS4_OPAQUE_LIMIT]; / client-provided,
    pub __attribute__((packed)): },
// sha256 hash of the kerberos principal
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cld_princhash {
    pub /: *mut *mut __u8 cp_len; / length of cp_data,
    pub /: *mut *mut unsigned char cp_data[SHA256_DIGEST_SIZE]; / hash of principal,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cld_clntinfo {
    pub cc_name: cld_name,
    pub cc_princhash: cld_princhash,
    pub __attribute__((packed)): },
// message struct for communication with userspace
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cld_msg {
    pub /: *mut *mut __u8 cm_vers; / upcall version,
    pub /: *mut *mut __u8 cm_cmd; / upcall command,
    pub /: *mut *mut __s16 cm_status; / return code,
    pub /: *mut *mut __u32 cm_xid; / transaction id,
    pub /: *mut *mut __s64 cm_gracetime; / grace period start time,
    pub cm_name: cld_name,
    pub /: *mut *mut __u8 cm_version; / for getting max version,
// C attribute field omitted
    pub __attribute__((packed)): },
// version 2 message can include hash of kerberos principal
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cld_msg_v2 {
    pub /: *mut *mut __u8 cm_vers; / upcall version,
    pub /: *mut *mut __u8 cm_cmd; / upcall command,
    pub /: *mut *mut __s16 cm_status; / return code,
    pub /: *mut *mut __u32 cm_xid; / transaction id,
    pub cm_name: cld_name,
    pub /: *mut *mut __u8 cm_version; / for getting max version,
    pub /: *mut *mut cld_clntinfo cm_clntinfo; / name & princ hash,
// C attribute field omitted
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cld_msg_hdr {
    pub /: *mut *mut __u8 cm_vers; / upcall version,
    pub /: *mut *mut __u8 cm_cmd; / upcall command,
    pub /: *mut *mut __s16 cm_status; / return code,
    pub /: *mut *mut __u32 cm_xid; / transaction id,
    pub __attribute__((packed)): },
