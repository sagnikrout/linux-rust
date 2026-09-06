//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/newbt.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrep_newbt_resv {
// Link to list of extents that we've reserved.
    pub list: list_head,
    pub pag: *mut xfs_perag,
// Auto-freeing this reservation if we don't commit.
    pub autoreap: xfs_alloc_autoreap,
// AG block of the extent we reserved.
    pub agbno: xfs_agblock_t,
// Length of the reservation.
    pub len: xfs_extlen_t,
// How much of this reservation has been used.
    pub used: xfs_extlen_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrep_newbt {
    pub sc: *mut xfs_scrub,
// Custom allocation function, or NULL for xfs_alloc_vextent
    pub alloc_hint): xfs_fsblock_t,
// List of extents that we've reserved.
    pub resv_list: list_head,
// Fake root for new btree.
    pub afake: xbtree_afakeroot,
    pub ifake: xbtree_ifakeroot,
}

// rmap owner of these blocks
// btree geometry for the bulk loader
// Allocation hint
// per-ag reservation type
extern "C" {
    pub fn xrep_newbt_init_bare(xnr: *mut xrep_newbt, sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xrep_newbt_init_metadir_inode(xnr: *mut xrep_newbt, sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_newbt_alloc_blocks(xnr: *mut xrep_newbt, nr_blocks: u64) -> c_int;
}
extern "C" {
    pub fn xrep_newbt_cancel(xnr: *mut xrep_newbt);
}
extern "C" {
    pub fn xrep_newbt_commit(xnr: *mut xrep_newbt) -> c_int;
}
extern "C" {
    pub fn xrep_newbt_unused_blocks(xnr: *mut xrep_newbt) -> c_uint;
}
