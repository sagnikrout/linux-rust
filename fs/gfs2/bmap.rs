//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/bmap.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
//

//
// gfs2_write_calc_reserv - calculate number of blocks needed to write to a file
// @ip: the file
// @len: the number of bytes to be written to the file
// @data_blocks: returns the number of data blocks required
// @ind_blocks: returns the number of indirect blocks required
//
// data_blocks = (len >> sdp->sd_sb.sb_bsize_shift) + 3;
// ind_blocks = 3 * (sdp->sd_max_height - 1);
// ind_blocks += tmp;
extern "C" {
    pub fn gfs2_unstuff_dinode(ip: *mut gfs2_inode) -> c_int;
}
extern "C" {
    pub fn gfs2_clear_beyond_eof(inode: *mut inode, end: loff_t) -> c_int;
}
extern "C" {
    pub fn gfs2_setattr_size(inode: *mut inode, size: u64) -> c_int;
}
extern "C" {
    pub fn gfs2_truncatei_resume(ip: *mut gfs2_inode) -> c_int;
}
extern "C" {
    pub fn gfs2_file_dealloc(ip: *mut gfs2_inode) -> c_int;
}
extern "C" {
    pub fn gfs2_map_journal_extents(sdp: *mut gfs2_sbd, jd: *mut gfs2_jdesc) -> c_int;
}
extern "C" {
    pub fn gfs2_free_journal_extents(jd: *mut gfs2_jdesc);
}
extern "C" {
    pub fn __gfs2_punch_hole(file: *mut file, offset: loff_t, length: loff_t) -> c_int;
}
