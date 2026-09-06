//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_rtbitmap.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rtalloc_args {
    pub rtg: *mut xfs_rtgroup,
    pub mp: *mut xfs_mount,
    pub tp: *mut xfs_trans,
    pub /: *mut *mut *mut xfs_buf rbmbp; / bitmap block buffer,
    pub /: *mut *mut *mut xfs_buf sumbp; / summary block buffer,
    pub /: *mut *mut xfs_fileoff_t rbmoff; / bitmap block number,
    pub /: *mut *mut xfs_fileoff_t sumoff; / summary block number,
}

// Convert an rgbno into an rt extent number.
// Compute the misalignment between an extent length and a realtime extent .
// Convert an rt block count into an rt extent count.
extern "C" {
    pub fn div_u64(_arg: blen, _arg: mp->m_sb.sb_rextsize) -> return;
}
// Return the offset of a file block length within an rt extent.
extern "C" {
    pub fn do_div(_arg: blen, _arg: mp->m_sb.sb_rextsize) -> return;
}
// Round this block count up to the nearest rt extent size.
extern "C" {
    pub fn roundup_64(_arg: blen, _arg: mp->m_sb.sb_rextsize) -> return;
}
// Convert an rt block number into an rt extent number.
// open-coded 64-bit masking operation
extern "C" {
    pub fn div_u64(_arg: rtbno, _arg: mp->m_sb.sb_rextsize) -> return;
}
// Return the offset of a rtgroup block number within an rt extent.
// Return the offset of an rt block number within an rt extent.
// open-coded 64-bit masking operation
extern "C" {
    pub fn do_div(_arg: rtbno, _arg: mp->m_sb.sb_rextsize) -> return;
}
// Round this file block offset up to the nearest rt extent size.
extern "C" {
    pub fn roundup_64(_arg: off, _arg: mp->m_sb.sb_rextsize) -> return;
}
// Round this file block offset down to the nearest rt extent size.
extern "C" {
    pub fn rounddown_64(_arg: off, _arg: mp->m_sb.sb_rextsize) -> return;
}
// Convert an rt extent number to a file block offset in the rt bitmap file.
extern "C" {
    pub fn div_u64(_arg: rtx, _arg: mp->m_rtx_per_rbmblock) -> return;
}
// Convert an rt extent number to a word offset within an rt bitmap block.
// Convert a file block offset in the rt bitmap file to an rt extent number.
// Return a pointer to a bitmap word within a rt bitmap block.
// Convert an ondisk bitmap word to its incore representation.
extern "C" {
    pub fn be32_to_cpu(_arg: word->rtg) -> return;
}
// Set an ondisk bitmap word from an incore representation.
//
// Convert a rt extent length and rt bitmap block number to a xfs_suminfo_t
// offset within the rt summary file.
//
// Convert an xfs_suminfo_t offset to a file block offset within the rt summary
// file.
//
extern "C" {
    pub fn XFS_B_TO_FSBT(_arg: mp, sizeof(xfs_suminfo_t): *mut *mut rsumoff) -> return;
}
//
// Convert an xfs_suminfo_t offset to an info word offset within an rt summary
// block.
//
// Return a pointer to a summary info word within a rt summary block.
// Get the current value of a summary counter.
extern "C" {
    pub fn be32_to_cpu(_arg: info->rtg) -> return;
}
// Add to the current value of a summary counter and return the new value.
extern "C" {
    pub fn be32_to_cpu(_arg: info->rtg) -> return;
}
//
// Functions for walking free space rtextents in the realtime bitmap.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rtalloc_rec {
    pub ar_startext: xfs_rtxnum_t,
    pub ar_extcount: xfs_rtbxlen_t,
}

extern "C" {
    pub fn xfs_rtbuf_cache_relse(args: *mut xfs_rtalloc_args);
}
extern "C" {
    pub fn xfs_rtbitmap_read_buf(args: *mut xfs_rtalloc_args, block: xfs_fileoff_t) -> c_int;
}
extern "C" {
    pub fn xfs_rtsummary_read_buf(args: *mut xfs_rtalloc_args, block: xfs_fileoff_t) -> c_int;
}
// Same as above, but in units of rt blocks.
extern "C" {
    pub fn xfs_rtbitmap_rtx_per_rbmblock(mp: *mut xfs_mount) -> xfs_rtxnum_t;
}
extern "C" {
    pub fn xfs_rtbitmap_blockcount(mp: *mut xfs_mount) -> xfs_filblks_t;
}

// shut up gcc

