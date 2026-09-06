//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/rgrp.h
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
// Copyright (C) 2004-2008 Red Hat, Inc.  All rights reserved.
//

// Since each block in the file system is represented by two bits in the
// bitmap, one 64-bit word in the bitmap will represent 32 blocks.
// By reserving 32 blocks at a time, we can optimize / shortcut how we search
// through the bitmaps by looking a word at a time.
//
pub const RGRP_RSRV_MINBLKS: c_int = 32;
pub const RGRP_RSRV_ADDBLKS: c_int = 64;
extern "C" {
    pub fn gfs2_rgrp_verify(rgd: *mut gfs2_rgrpd);
}
extern "C" {
    pub fn gfs2_clear_rgrpd(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_rindex_update(sdp: *mut gfs2_sbd) -> c_int;
}
extern "C" {
    pub fn gfs2_free_clones(rgd: *mut gfs2_rgrpd);
}
extern "C" {
    pub fn gfs2_rgrp_go_instantiate(gl: *mut gfs2_glock) -> c_int;
}
extern "C" {
    pub fn gfs2_rgrp_brelse(rgd: *mut gfs2_rgrpd);
}
pub const GFS2_AF_ORLOV: c_int = 1;
extern "C" {
    pub fn gfs2_inplace_release(ip: *mut gfs2_inode);
}
extern "C" {
    pub fn gfs2_rs_deltree(rs: *mut gfs2_blkreserv);
}
extern "C" {
    pub fn gfs2_rs_delete(ip: *mut gfs2_inode);
}
extern "C" {
    pub fn gfs2_free_di(rgd: *mut gfs2_rgrpd, ip: *mut gfs2_inode);
}
extern "C" {
    pub fn gfs2_unlink_di(inode: *mut inode);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_rgrp_list {
    pub rl_rgrps: c_uint,
    pub rl_space: c_uint,
    pub rl_rgd: *mut gfs2_rgrpd,
    pub rl_ghs: *mut gfs2_holder,
}

extern "C" {
    pub fn gfs2_rlist_free(rlist: *mut gfs2_rgrp_list);
}
extern "C" {
    pub fn gfs2_ri_total(sdp: *mut gfs2_sbd) -> u64;
}
extern "C" {
    pub fn gfs2_fitrim(filp: *mut file, argp: *mut void __user) -> c_int;
}
// This is how to tell if a reservation is in the rgrp tree:
extern "C" {
    pub fn check_and_update_goal(ip: *mut gfs2_inode);
}
extern "C" {
    pub fn rgrp_lock_local(rgd: *mut gfs2_rgrpd);
}
extern "C" {
    pub fn rgrp_unlock_local(rgd: *mut gfs2_rgrpd);
}
