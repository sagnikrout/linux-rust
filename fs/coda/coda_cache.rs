//! Automatically rewritten from C Header to Rust Module
//! Source: fs/coda/coda_cache.h
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
// Coda filesystem -- Linux Minicache
//
// Copyright (C) 1989 - 1997 Carnegie Mellon University
//
// Carnegie Mellon University encourages users of this software to
// contribute improvements to the Coda project. Contact Peter Braam
// <coda@cs.cmu.edu>
//
// credential cache
extern "C" {
    pub fn coda_cache_enter(inode: *mut inode, mask: c_int);
}
extern "C" {
    pub fn coda_cache_clear_inode(: *mut inode);
}
extern "C" {
    pub fn coda_cache_clear_all(sb: *mut super_block);
}
extern "C" {
    pub fn coda_cache_check(inode: *mut inode, mask: c_int) -> c_int;
}
// for downcalls and attributes and lookups
extern "C" {
    pub fn coda_flag_inode_children(inode: *mut inode, flag: c_int);
}
