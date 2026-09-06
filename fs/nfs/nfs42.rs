//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/nfs42.h
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
// Copyright (c) 2014 Anna Schumaker <Anna.Schumaker@Netapp.com>
//

//
// FIXME:  four LAYOUTSTATS calls per compound at most! Do we need to support
// more? Need to consider not to pre-alloc too much for a compound.
//

// nfs4.2proc.c

extern "C" {
    pub fn nfs42_proc_allocate(: *mut file, _arg: loff_t, _arg: loff_t) -> c_int;
}
extern "C" {
    pub fn nfs42_proc_deallocate(: *mut file, _arg: loff_t, _arg: loff_t) -> c_int;
}
extern "C" {
    pub fn nfs42_proc_zero_range(: *mut file, _arg: loff_t, _arg: loff_t) -> c_int;
}
extern "C" {
    pub fn nfs42_proc_llseek(: *mut file, _arg: loff_t, _arg: c_int) -> loff_t;
}
extern "C" {
    pub fn nfs42_proc_clone(: *mut file, : *mut file, _arg: loff_t, _arg: loff_t, _arg: loff_t) -> c_int;
}
extern "C" {
    pub fn nfs42_proc_removexattr(inode: *mut inode, name: *const c_char) -> c_int;
}
//
// Maximum XDR buffer size needed for a listxattr buffer of buflen size.
//
// The upper boundary is a buffer with all 1-byte sized attribute names.
// They would be 7 bytes long in the eventual buffer ("user.x\0"), and
// 8 bytes long XDR-encoded.
//
// Include the trailing eof word as well and make the result a multiple
// of 4 bytes.
//

