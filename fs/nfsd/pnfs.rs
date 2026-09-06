//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/pnfs.h
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
pub const _FS_NFSD_PNFS_H: c_int = 1;

// Cap exponential backoff between fence retries at 3 minutes

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_deviceid_map {
    pub hash: list_head,
    pub idx: u64,
    pub fsid_type: c_int,
    pub fsid: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_layout_ops {
    pub notify_types: u32,
    pub disable_recalls: bool,
    pub gdevp): *mut nfsd4_getdeviceinfo,
    pub gdevp): *const nfsd4_getdeviceinfo,
    pub lgp): *const *const svc_fh fhp, nfsd4_layoutget,
    pub lgp): *const nfsd4_layoutget,
    pub lcp): *mut nfsd4_layoutcommit,
    pub file): *mut nfsd_file,
}

extern "C" {
    pub fn nfsd4_setup_layout_type(exp: *mut svc_export);
}
extern "C" {
    pub fn nfsd4_return_all_client_layouts(: *mut nfs4_client);
}
extern "C" {
    pub fn nfsd4_close_layout(ls: *mut nfs4_layout_stateid);
}
extern "C" {
    pub fn nfsd4_init_pnfs() -> c_int;
}
extern "C" {
    pub fn nfsd4_exit_pnfs();
}

