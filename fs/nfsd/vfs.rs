//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/vfs.h
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
// Copyright (C) 1995-1997 Olaf Kirch <okir@monad.swb.de>
//

//
// Flags for nfsd_permission
//
pub const NFSD_MAY_NOP: c_int = 0;
pub const NFSD_MAY_EXEC: c_uint = 0x001 /* == MAY_EXEC */;
pub const NFSD_MAY_WRITE: c_uint = 0x002 /* == MAY_WRITE */;
pub const NFSD_MAY_READ: c_uint = 0x004 /* == MAY_READ */;
pub const NFSD_MAY_SATTR: c_uint = 0x008;
pub const NFSD_MAY_TRUNC: c_uint = 0x010;
pub const NFSD_MAY_NLM: c_uint = 0x020 /* request is from lockd */;
pub const NFSD_MAY_MASK: c_uint = 0x03f;
// extra hints to permission and open routines:
pub const NFSD_MAY_OWNER_OVERRIDE: c_uint = 0x040;
pub const NFSD_MAY_LOCAL_ACCESS: c_uint = 0x080 /* for device special files */;
pub const NFSD_MAY_BYPASS_GSS_ON_ROOT: c_uint = 0x100;
pub const NFSD_MAY_NOT_BREAK_LEASE: c_uint = 0x200;
pub const NFSD_MAY_BYPASS_GSS: c_uint = 0x400;
pub const NFSD_MAY_READ_IF_EXEC: c_uint = 0x800;
pub const NFSD_MAY_64BIT_COOKIE: c_uint = 0x1000 /* 64 bit readdir cookies for >= NFSv3 */;
pub const NFSD_MAY_LOCALIO: c_uint = 0x2000 /* for tracing, reflects when localio used */;

//
// Callback function for readdir
//
extern "C" {
    pub fn int(: *mut *mut nfsd_filldir_t)(void, : *const c_char, _arg: c_int, _arg: loff_t, _arg: u64, _arg: unsigned) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct readdir_cd {
    pub /: *mut *mut __be32 err; / nfs_ok, nfserr, or nfserr_eof,
}

// nfsd/vfs.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_attrs {
    pub /: *mut *mut *mut iattr na_iattr; / input,
    pub /: *mut *mut *mut xdr_netobj na_seclabel; / input,
    pub /: *mut *mut *mut posix_acl na_pacl; / input,
    pub /: *mut *mut *mut posix_acl na_dpacl; / input,
    pub /: *mut *mut int na_labelerr; / output,
    pub /: *mut *mut int na_dpaclerr; / output,
    pub /: *mut *mut int na_paclerr; / output,
}

extern "C" {
    pub fn nfserrno(errno: c_int) -> __be32;
}
extern "C" {
    pub fn nfsd_mountpoint(: *mut dentry, : *mut svc_export) -> c_int;
}

extern "C" {
    pub fn nfsd_access(: *mut svc_rqst, : *mut svc_fh, : *mut u32, : *mut u32) -> __be32;
}

extern "C" {
    pub fn nfsd_open_break_lease(: *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nfsd_read_splice_ok(rqstp: *mut svc_rqst) -> bool;
}
extern "C" {
    pub fn nfsd_filp_close(fp: *mut file);
}
