//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/log.h
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
// The minimum amount of log space required for a log flush is one block for
// revokes and one block for the log header.  Log flushes other than
// GFS2_LOG_HEAD_FLUSH_NORMAL may write one or two more log headers.
//
pub const GFS2_LOG_FLUSH_MIN_BLOCKS: c_int = 4;
extern "C" {
    pub fn gfs2_ordered_del_inode(ip: *mut gfs2_inode);
}
extern "C" {
    pub fn gfs2_struct2blk(sdp: *mut gfs2_sbd, nstruct: c_uint) -> c_uint;
}
extern "C" {
    pub fn gfs2_log_is_empty(sdp: *mut gfs2_sbd) -> bool;
}
extern "C" {
    pub fn gfs2_log_release_revokes(sdp: *mut gfs2_sbd, revokes: c_uint);
}
extern "C" {
    pub fn gfs2_log_release(sdp: *mut gfs2_sbd, blks: c_uint);
}
extern "C" {
    pub fn gfs2_remove_from_journal(bh: *mut buffer_head, meta: c_int);
}
extern "C" {
    pub fn gfs2_log_commit(sdp: *mut gfs2_sbd, trans: *mut gfs2_trans);
}
extern "C" {
    pub fn gfs2_ail1_flush(sdp: *mut gfs2_sbd, wbc: *mut writeback_control);
}
extern "C" {
    pub fn log_flush_wait(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_logd(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn gfs2_add_revoke(sdp: *mut gfs2_sbd, bd: *mut gfs2_bufdata);
}
extern "C" {
    pub fn gfs2_glock_remove_revoke(gl: *mut gfs2_glock);
}
extern "C" {
    pub fn gfs2_flush_revokes(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_ail_drain(sdp: *mut gfs2_sbd);
}
