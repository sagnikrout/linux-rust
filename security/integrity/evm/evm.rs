//! Automatically rewritten from C Header to Rust Module
//! Source: security/integrity/evm/evm.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2005-2010 IBM Corporation
//
// Authors:
// Mimi Zohar <zohar@us.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
//
// File: evm.h
//

pub const EVM_INIT_HMAC: c_uint = 0x0001;
pub const EVM_INIT_X509: c_uint = 0x0002;
pub const EVM_ALLOW_METADATA_WRITES: c_uint = 0x0004;
pub const EVM_SIGV3_REQUIRED: c_uint = 0x0008;
pub const EVM_SETUP_COMPLETE: c_uint = 0x80000000 /* userland has signaled key load */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xattr_list {
    pub list: list_head,
    pub name: *mut c_char,
    pub enabled: bool,
}

pub const EVM_NEW_FILE: c_uint = 0x00000001;
pub const EVM_IMMUTABLE_DIGSIG: c_uint = 0x00000002;
// EVM integrity metadata associated with an inode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evm_iint_cache {
    pub flags: c_ulong,
    pub evm_status:4: integrity_status,
    pub metadata_inode: integrity_inode_attributes,
}

pub const EVM_ATTR_FSUUID: c_uint = 0x0001;
// List of EVM protected security xattrs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evm_digest {
    pub hdr: ima_digest_data_hdr,
    pub digest: [c_char; IMA_MAX_DIGEST_SIZE],
    pub __packed: },
    pub req_xattr_name): *const int evm_protected_xattr(char,
    pub evm_init_key(void): c_int,
    pub req_xattr_value_len): usize,
    pub iint): *mut evm_iint_cache,
    pub iint): *mut *mut evm_digest data, evm_iint_cache,
    pub hmac_val): *mut c_char,
    pub evm_init_secfs(void): c_int,
