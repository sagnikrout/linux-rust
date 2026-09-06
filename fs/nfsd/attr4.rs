//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/attr4.h
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
// NFSv4 file attributes supported by this implementation
//

//
// The following attributes are not implemented by NFSD:
// ARCHIVE       (deprecated anyway)
// HIDDEN        (unlikely to be supported any time soon)
// MIMETYPE      (unlikely to be supported any time soon)
// QUOTA_*       (unlikely to be supported any time soon)
// SYSTEM        (unlikely to be supported any time soon)
// TIME_BACKUP   (unlikely to be supported any time soon)
//

pub const NFSD4_SUPPORTED_ATTRS_WORD2: c_int = 0;
// 4.1

pub const PNFSD_SUPPORTED_ATTRS_WORD1: c_int = 0;
pub const PNFSD_SUPPORTED_ATTRS_WORD2: c_int = 0;

// 4.2

pub const NFSD4_2_SECURITY_ATTRS: c_int = 0;

pub const NFSD4_2_POSIX_ACL_ATTRS: c_int = 0;

// These will return ERR_INVAL if specified in GETATTR or READDIR.

//
// These are the only attrs allowed in CREATE/OPEN/SETATTR. Don't add
// a writeable attribute here without also adding code to parse it to
// nfsd4_decode_fattr4().
//

pub const MAYBE_FATTR4_WORD2_SECURITY_LABEL: c_int = 0;

pub const MAYBE_FATTR4_WORD2_POSIX_ACL_ATTRS: c_int = 0;

//
// we currently store the exclusive create verifier in the v_{a,m}time
// attributes so the client can't set these at create time using EXCLUSIVE4_1
//

//
// The FATTR4_WORD2_TIME_DELEG attributes are not to be allowed for
// OPEN(create) with EXCLUSIVE4_1. It doesn't make sense to set a
// delegated timestamp on a new file.
//
// This mask includes NFSv4.2-only attributes (e.g., POSIX ACLs).
// Version filtering occurs via nfsd_suppattrs[] before this mask
// is applied, so pre-4.2 clients never see unsupported attributes.
//

extern "C" {
    pub fn bmval_is_subset(_arg: bmval, _arg: nfsd_suppattrs[minorversion]) -> return;
}
