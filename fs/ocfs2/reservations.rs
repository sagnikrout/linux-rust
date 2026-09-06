//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/reservations.h
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
// reservations.h
//
// Allocation reservations function prototypes and structures.
//
// Copyright (C) 2010 Novell.  All rights reserved.
//

pub const OCFS2_DEFAULT_RESV_LEVEL: c_int = 2;
pub const OCFS2_MAX_RESV_LEVEL: c_int = 9;
pub const OCFS2_MIN_RESV_LEVEL: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_alloc_reservation {
    pub r_node: rb_node,
    pub /: *mut *mut unsigned int r_start; / Beginning of current window,
    pub /: *mut *mut unsigned int r_len; / Length of the window,
    pub /: *mut *mut unsigned int r_last_len; / Length of most recent alloc,
    pub /: *mut *mut unsigned int r_last_start; / Start of most recent alloc,
    pub /: *mut *mut list_head r_lru; / LRU list head,
    pub r_flags: c_uint,
}

pub const OCFS2_RESV_FLAG_INUSE: c_uint = 0x01	/* Set when r_node is part of a btree */;
pub const OCFS2_RESV_FLAG_TMP: c_uint = 0x02	/* Temporary reservation, will be;
// destroyed immediately after use
pub const OCFS2_RESV_FLAG_DIR: c_uint = 0x04	/* Reservation is for an unindexed;
// directory btree
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_reservation_map {
    pub m_reservations: rb_root,
    pub m_disk_bitmap: *mut c_char,
    pub m_osb: *mut ocfs2_super,
// The following are not initialized to meaningful values until a disk
// bitmap is provided.
    pub valid: *mut *mut u32 m_bitmap_len; / Number of,
// bits available
    pub reservations: *mut *mut list_head m_lru; / LRU of,
// structures.
}

extern "C" {
    pub fn ocfs2_resv_init_once(resv: *mut ocfs2_alloc_reservation);
}

extern "C" {
    pub fn ocfs2_dir_resv_allowed(osb: *mut ocfs2_super) -> c_int;
}
//
// ocfs2_resv_discard() - truncate a reservation
// @resmap:
// @resv: the reservation to truncate.
//
// After this function is called, the reservation will be empty, and
// unlinked from the rbtree.
//
// ocfs2_resmap_init() - Initialize fields of a reservations bitmap
// @osb: struct ocfs2_super to be saved in resmap
// @resmap: struct ocfs2_reservation_map to initialize
//
// ocfs2_resmap_restart() - "restart" a reservation bitmap
// @resmap: reservations bitmap
// @clen: Number of valid bits in the bitmap
// @disk_bitmap: the disk bitmap this resmap should refer to.
//
// Re-initialize the parameters of a reservation bitmap. This is
// useful for local alloc window slides.
//
// This function will call ocfs2_trunc_resv against all existing
// reservations. A future version will recalculate existing
// reservations based on the new bitmap.
//
// ocfs2_resmap_uninit() - uninitialize a reservation bitmap structure
// @resmap: the struct ocfs2_reservation_map to uninitialize
//
extern "C" {
    pub fn ocfs2_resmap_uninit(resmap: *mut ocfs2_reservation_map);
}
//
// ocfs2_resmap_resv_bits() - Return still-valid reservation bits
// @resmap: reservations bitmap
// @resv: reservation to base search from
// @cstart: start of proposed allocation
// @clen: length (in clusters) of proposed allocation
//
// Using the reservation data from resv, this function will compare
// resmap and resmap->m_disk_bitmap to determine what part (if any) of
// the reservation window is still clear to use. If resv is empty,
// this function will try to allocate a window for it.
//
// On success, zero is returned and the valid allocation area is set in cstart
// and clen.
//
// Returns -ENOSPC if reservations are disabled.
//
// ocfs2_resmap_claimed_bits() - Tell the reservation code that bits were used.
// @resmap: reservations bitmap
// @resv: optional reservation to recalculate based on new bitmap
// @cstart: start of allocation in clusters
// @clen: end of allocation in clusters.
//
// Tell the reservation code that bits were used to fulfill allocation in
// resmap. The bits don't have to have been part of any existing
// reservation. But we must always call this function when bits are claimed.
// Internally, the reservations code will use this information to mark the
// reservations bitmap. If resv is passed, it's next allocation window will be
// calculated. It also expects that 'cstart' is the same as we passed back
// from ocfs2_resmap_resv_bits().
//
