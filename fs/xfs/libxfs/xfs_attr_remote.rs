//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_attr_remote.h
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
// Copyright (c) 2013 Red Hat, Inc.
// All Rights Reserved.
//
extern "C" {
    pub fn xfs_attr3_rmt_blocks(mp: *mut xfs_mount, attrlen: c_uint) -> c_uint;
}
// Number of rmt blocks needed to store the maximally sized attr value
extern "C" {
    pub fn xfs_attr3_rmt_blocks(_arg: mp, _arg: XFS_XATTR_SIZE_MAX) -> return;
}
extern "C" {
    pub fn xfs_attr_rmtval_get(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_rmtval_invalidate(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_rmtval_remove(attr: *mut xfs_attr_intent) -> c_int;
}
extern "C" {
    pub fn xfs_attr_rmt_find_hole(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_rmtval_set_value(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_rmtval_set_blk(attr: *mut xfs_attr_intent) -> c_int;
}
extern "C" {
    pub fn xfs_attr_rmtval_find_space(attr: *mut xfs_attr_intent) -> c_int;
}
