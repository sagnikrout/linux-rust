//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/extent_map.h
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
// extent_map.h
//
// In-memory file extent mappings for OCFS2.
//
// Copyright (C) 2004 Oracle.  All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_extent_map_item {
    pub ei_cpos: c_uint,
    pub ei_phys: c_uint,
    pub ei_clusters: c_uint,
    pub ei_flags: c_uint,
    pub ei_list: list_head,
}

pub const OCFS2_MAX_EXTENT_MAP_ITEMS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_extent_map {
    pub em_num_items: c_uint,
    pub em_list: list_head,
}

extern "C" {
    pub fn ocfs2_extent_map_init(inode: *mut inode);
}
extern "C" {
    pub fn ocfs2_extent_map_trunc(inode: *mut inode, cluster: c_uint);
}
extern "C" {
    pub fn ocfs2_seek_data_hole_offset(file: *mut file, offset: *mut loff_t, origin: c_int) -> c_int;
}
