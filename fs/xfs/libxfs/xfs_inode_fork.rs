//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_inode_fork.h
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
// Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// File incore extent information, present for each of data & attr forks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_ifork {
    pub /: *mut *mut int64_t if_bytes; / bytes in if_data,
    pub /: *mut *mut *mut xfs_btree_block if_broot; / file's incore btree root,
    pub /: *mut *mut unsigned int if_seq; / fork mod counter,
    pub /: *mut *mut int if_height; / height of the extent tree,
    pub or: *mut *mut *mut void if_data; / extent tree root,
    pub /: *mut *mut xfs_extnum_t if_nextents; / # of extents in this fork,
    pub /: *mut *mut short if_broot_bytes; / bytes allocated for root,
    pub /: *mut *mut int8_t if_format; / format of this fork,
    pub /: *mut *mut uint8_t if_needextents; / extents have not been read,
}

//
// Worst-case increase in the fork extent count when we're adding a single
// extent to a fork and there's no possibility of splitting an existing mapping.
//

//
// Punching out an extent from the middle of an existing extent can cause the
// extent count to increase by 1.
// i.e. | Old extent | Hole | Old extent |
//

//
// Adding/removing an xattr can cause XFS_DA_NODE_MAXDEPTH extents to
// be added. One extra extent for dabtree in case a local attr is
// large enough to cause a double split.  It can also cause extent
// count to increase proportional to the size of a remote xattr's
// value.
//

//
// A write to a sub-interval of an existing unwritten extent causes the original
// extent to be split into 3 extents
// i.e. | Unwritten | Real | Unwritten |
// Hence extent count can increase by 2.
//

//
// Moving an extent to data fork can cause a sub-interval of an existing extent
// to be unmapped. This will increase extent count by 1. Mapping in the new
// extent can increase the extent count by 1 again i.e.
// | Old extent | New extent | Old extent |
// Hence number of extents increases by 2.
//

//
// Removing an initial range of source/donor file's extent and adding a new
// extent (from donor/source file) in its place will cause extent count to
// increase by 1.
//

//
// Fork handling.
//

extern "C" {
    pub fn be64_to_cpu(_arg: dip->di_big_nextents) -> return;
}
extern "C" {
    pub fn be32_to_cpu(_arg: dip->di_nextents) -> return;
}
extern "C" {
    pub fn be32_to_cpu(_arg: dip->di_big_anextents) -> return;
}
extern "C" {
    pub fn be16_to_cpu(_arg: dip->di_anextents) -> return;
}
extern "C" {
    pub fn xfs_dfork_data_extents(_arg: dip) -> return;
}
extern "C" {
    pub fn xfs_dfork_attr_extents(_arg: dip) -> return;
}
extern "C" {
    pub fn xfs_ifork_zap_attr(ip: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_iformat_data_fork(: *mut xfs_inode, : *mut xfs_dinode) -> c_int;
}
extern "C" {
    pub fn xfs_iformat_attr_fork(: *mut xfs_inode, : *mut xfs_dinode) -> c_int;
}
extern "C" {
    pub fn xfs_idestroy_fork(ifp: *mut xfs_ifork);
}
extern "C" {
    pub fn xfs_iread_extents(: *mut xfs_trans, : *mut xfs_inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn xfs_iext_count(ifp: *mut xfs_ifork) -> xfs_extnum_t;
}
extern "C" {
    pub fn xfs_iext_destroy(: *mut xfs_ifork);
}
extern "C" {
    pub fn xfs_iext_first(: *mut xfs_ifork, : *mut xfs_iext_cursor);
}
extern "C" {
    pub fn xfs_iext_last(: *mut xfs_ifork, : *mut xfs_iext_cursor);
}
extern "C" {
    pub fn xfs_iext_next(: *mut xfs_ifork, : *mut xfs_iext_cursor);
}
extern "C" {
    pub fn xfs_iext_prev(: *mut xfs_ifork, : *mut xfs_iext_cursor);
}
extern "C" {
    pub fn xfs_iext_get_extent(_arg: ifp, _arg: cur, _arg: gotp) -> return;
}
extern "C" {
    pub fn xfs_iext_get_extent(_arg: ifp, _arg: cur, _arg: gotp) -> return;
}
//
// Return the extent after cur in gotp without updating the cursor.
//
extern "C" {
    pub fn xfs_iext_get_extent(_arg: ifp, _arg: &ncur, _arg: gotp) -> return;
}
//
// Return the extent before cur in gotp without updating the cursor.
//
extern "C" {
    pub fn xfs_iext_get_extent(_arg: ifp, _arg: &ncur, _arg: gotp) -> return;
}

extern "C" {
    pub fn xfs_ifork_init_cow(ip: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_ifork_verify_local_data(ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_ifork_verify_local_attr(ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_ifork_is_realtime(ip: *mut xfs_inode, whichfork: c_int) -> bool;
}
// returns true if the fork has extents but they are not read in yet.
// see xfs_iformat_{data,attr}_fork() for needextents semantics
