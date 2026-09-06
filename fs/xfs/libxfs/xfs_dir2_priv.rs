//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_dir2_priv.h
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
// Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// In-core version of the leaf and free block headers to abstract the
// differences in the v2 and v3 disk format of the headers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir3_icleaf_hdr {
    pub forw: u32,
    pub back: u32,
    pub magic: u16,
    pub count: u16,
    pub stale: u16,
//
// Pointer to the on-disk format entries, which are behind the
// variable size (v4 vs v5) header in the on-disk block.
//
    pub ents: *mut xfs_dir2_leaf_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir3_icfree_hdr {
    pub magic: u32,
    pub firstdb: u32,
    pub nvalid: u32,
    pub nused: u32,
//
// Pointer to the on-disk format entries, which are behind the
// variable size (v4 vs v5) header in the on-disk block.
//
    pub bests: *mut __be16,
}

// xfs_dir2.c
extern "C" {
    pub fn xfs_ascii_ci_hashname(name: *const xfs_name) -> xfs_dahash_t;
}
// xfs_dir2_block.c
extern "C" {
    pub fn xfs_dir2_block_addname(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_block_lookup(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_block_removename(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_block_replace(args: *mut xfs_da_args) -> c_int;
}
// xfs_dir2_data.c

extern "C" {
    pub fn xfs_dir3_data_check(dp: *mut xfs_inode, bp: *mut xfs_buf);
}

// Macro flag: #define	xfs_dir3_data_check(dp,bp)

// xfs_dir2_leaf.c
extern "C" {
    pub fn xfs_dir2_leaf_addname(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_leaf_lookup(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_leaf_removename(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_leaf_replace(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_node_to_leaf(state: *mut xfs_da_state) -> c_int;
}
// xfs_dir2_node.c
extern "C" {
    pub fn xfs_dir2_leafn_toosmall(state: *mut xfs_da_state, action: *mut c_int) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_node_addname(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_node_lookup(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_node_removename(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_node_replace(args: *mut xfs_da_args) -> c_int;
}
// xfs_dir2_sf.c
extern "C" {
    pub fn xfs_dir2_sf_get_parent_ino(hdr: *mut xfs_dir2_sf_hdr) -> xfs_ino_t;
}
extern "C" {
    pub fn xfs_dir2_sf_put_parent_ino(hdr: *mut xfs_dir2_sf_hdr, ino: xfs_ino_t);
}
extern "C" {
    pub fn xfs_dir2_sf_addname(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_sf_create(args: *mut xfs_da_args, pino: xfs_ino_t) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_sf_lookup(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_sf_removename(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir2_sf_replace(args: *mut xfs_da_args) -> c_int;
}
// xfs_dir2_readdir.c
extern "C" {
    pub fn round_up(_arg: len, _arg: XFS_DIR2_DATA_ALIGN) -> return;
}
extern "C" {
    pub fn round_up(_arg: len, _arg: XFS_DIR2_DATA_ALIGN) -> return;
}
