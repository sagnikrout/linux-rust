//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/nfs.h
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
// Copyright (c) 2012 Netapp, Inc. All rights reserved.
//
// Function and structures exported by the NFS module
// for use by NFS version-specific modules.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_subversion {
    pub /: *mut *mut *mut module owner; / THIS_MODULE pointer,
    pub /: *mut *mut *mut file_system_type nfs_fs; / NFS filesystem type,
    pub /: *const *const *const rpc_version rpc_vers; / NFS version information,
    pub /: *const *const *const nfs_rpc_ops rpc_ops; / NFS operations,
    pub /: *const *const *const super_operations sops; / NFS Super operations,
    pub /: *const *const *const *const xattr_handler  xattr; / NFS xattr handlers,
}

extern "C" {
    pub fn get_nfs_version(: *mut nfs_subversion) -> c_int;
}
extern "C" {
    pub fn put_nfs_version(: *mut nfs_subversion);
}
extern "C" {
    pub fn register_nfs_version(: *mut nfs_subversion);
}
extern "C" {
    pub fn unregister_nfs_version(: *mut nfs_subversion);
}
