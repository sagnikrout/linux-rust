//! Automatically rewritten from C Header to Rust Module
//! Source: fs/coda/coda_linux.h
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
// Coda File System, Linux Kernel module
//
// Original version, adapted from cfs_mach.c, (C) Carnegie Mellon University
// Linux modifications (C) 1996, Peter J. Braam
// Rewritten for Linux 2.1 (C) 1997 Carnegie Mellon University
//
// Carnegie Mellon University encourages users of this software to
// contribute improvements to the Coda project.
//

// operations
// operations shared over more than one file
extern "C" {
    pub fn coda_open(i: *mut inode, f: *mut file) -> c_int;
}
extern "C" {
    pub fn coda_release(i: *mut inode, f: *mut file) -> c_int;
}
extern "C" {
    pub fn coda_revalidate_inode(: *mut inode) -> c_int;
}
extern "C" {
    pub fn coda_setattr(: *mut mnt_idmap, : *mut dentry, : *mut iattr) -> c_int;
}
// this file:  helpers
extern "C" {
    pub fn coda_iscontrol(name: *const c_char, length: usize) -> c_int;
}
extern "C" {
    pub fn coda_inode_type(attr: *mut coda_vattr) -> umode_t;
}
extern "C" {
    pub fn coda_vattr_to_iattr(: *mut inode, : *mut coda_vattr);
}
extern "C" {
    pub fn coda_iattr_to_vattr(: *mut iattr, : *mut coda_vattr);
}
extern "C" {
    pub fn coda_flags_to_cflags(short: unsigned) -> c_ushort;
}
// inode to cnode access functions
extern "C" {
    pub fn container_of(_arg: inode, coda_inode_info: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn coda_f2s(_arg: &(ITOC(inode)->c_fid)) -> return;
}
// this will not zap the inode away
