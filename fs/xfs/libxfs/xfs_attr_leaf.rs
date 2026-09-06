//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_attr_leaf.h
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
// Copyright (c) 2000,2002-2003,2005 Silicon Graphics, Inc.
// Copyright (c) 2013 Red Hat, Inc.
// All Rights Reserved.
//
// Incore version of the attribute leaf header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attr3_icleaf_hdr {
    pub forw: u32,
    pub back: u32,
    pub magic: u16,
    pub count: u16,
    pub usedbytes: u16,
//
// Firstused is 32-bit here instead of 16-bit like the on-disk variant
// to support maximum fsb size of 64k without overflow issues throughout
// the attr code. Instead, the overflow condition is handled on
// conversion to/from disk.
//
    pub firstused: u32,
    pub holes: __u8,
    pub base: u16,
    pub size: u16,
    pub freemap: [}; XFS_ATTR_LEAF_MAPSIZE],
}

// ========================================================================
// Function prototypes for the kernel.
// ========================================================================
//
// Internal routines when attribute fork size < XFS_LITINO(mp).
//
extern "C" {
    pub fn xfs_attr_shortform_create(args: *mut xfs_da_args);
}
extern "C" {
    pub fn xfs_attr_shortform_replace(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_shortform_add(args: *mut xfs_da_args, forkoff: c_int);
}
extern "C" {
    pub fn xfs_attr_shortform_getvalue(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_shortform_to_leaf(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_sf_removename(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_shortform_allfit(bp: *mut xfs_buf, dp: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_attr_shortform_bytesfit(dp: *mut xfs_inode, bytes: c_int) -> c_int;
}
extern "C" {
    pub fn xfs_attr_fork_remove(ip: *mut xfs_inode, tp: *mut xfs_trans);
}
//
// Internal routines when attribute fork size == XFS_LBSIZE(mp).
//
extern "C" {
    pub fn xfs_attr3_leaf_to_node(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr3_leaf_clearflag(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr3_leaf_setflag(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr3_leaf_flipflags(args: *mut xfs_da_args) -> c_int;
}
//
// Routines used for growing the Btree.
//
extern "C" {
    pub fn xfs_attr3_leaf_getvalue(bp: *mut xfs_buf, args: *mut xfs_da_args) -> c_int;
}
//
// Routines used for shrinking the Btree.
//
extern "C" {
    pub fn xfs_attr3_leaf_toosmall(state: *mut xfs_da_state, retval: *mut c_int) -> c_int;
}
//
// Utility routines.
//
extern "C" {
    pub fn xfs_attr_leaf_lasthash(bp: *mut xfs_buf, count: *mut c_int) -> xfs_dahash_t;
}
extern "C" {
    pub fn xfs_attr_leaf_newentsize(args: *mut xfs_da_args, local: *mut c_int) -> c_int;
}
