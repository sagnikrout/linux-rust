//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_group.h
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
// Copyright (c) 2018 Red Hat, Inc.
//
pub const __LIBXFS_GROUP_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_group {
    pub xg_mount: *mut xfs_mount,
    pub xg_gno: u32,
    pub xg_type: xfs_group_type,
    pub /: *mut *mut atomic_t xg_ref; / passive reference count,
    pub /: *mut *mut atomic_t xg_active_ref; / active reference count,
// Precalculated geometry info
    pub /: *mut *mut uint32_t xg_block_count; / max usable gbno,
    pub /: *mut *mut uint32_t xg_min_gbno; / min usable gbno,

// -- kernel only structures below this line --
//
// For perags and non-zoned RT groups:
// Track freed but not yet committed extents.
//
    pub xg_busy_extents: *mut xfs_extent_busy_tree,
//
// For zoned RT groups:
// List of groups that need a zone reset.
//
// The zonegc code forces a log flush of the rtrmap inode before
// resetting the write pointer, so there is no need for
// individual busy extent tracking.
//
    pub xg_next_reset: *mut xfs_group,
}

//
// Bitsets of per-ag metadata that have been checked and/or are sick.
// Callers should hold xg_state_lock before accessing this field.
//
// We use xfs_drain to track the number of deferred log intent items
// that have been queued (but not yet processed) so that waiters (e.g.
// scrub) will not lock resources when other threads are in the middle
// of processing a chain of intent items only to find momentary
// inconsistencies.
//
// Hook to feed rmapbt updates to an active online repair.
//

extern "C" {
    pub fn xfs_group_put(xg: *mut xfs_group);
}
extern "C" {
    pub fn xfs_group_rele(xg: *mut xfs_group);
}

extern "C" {
    pub fn XFS_FSB_TO_BB(_arg: mp, fsbno: g->start_fsb +) -> return;
}
