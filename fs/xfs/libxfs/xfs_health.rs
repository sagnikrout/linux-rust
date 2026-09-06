//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_health.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2019 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//
// In-Core Filesystem Health Assessments
// =====================================
//
// We'd like to be able to summarize the current health status of the
// filesystem so that the administrator knows when it's necessary to schedule
// some downtime for repairs.  Until then, we would also like to avoid abrupt
// shutdowns due to corrupt metadata.
//
// The online scrub feature evaluates the health of all filesystem metadata.
// When scrub detects corruption in a piece of metadata it will set the
// corresponding sickness flag, and repair will clear it if successful.  If
// problems remain at unmount time, we can also request manual intervention by
// logging a notice to run xfs_repair.
//
// Each health tracking group uses a pair of fields for reporting.  The
// "checked" field tell us if a given piece of metadata has ever been examined,
// and the "sick" field tells us if that piece was found to need repairs.
// Therefore we can conclude that for a given sick flag value:
//
// - checked && sick   => metadata needs repair
// - checked && !sick  => metadata is ok
// - !checked && sick  => errors have been observed during normal operation,
// but the metadata has not been checked thoroughly
// - !checked && !sick => has not been examined since mount
//
// Evidence of health problems can be sorted into three basic categories:
//
// a) Primary evidence, which signals that something is defective within the
// general grouping of metadata.
//
// b) Secondary evidence, which are side effects of primary problem but are
// not themselves problems.  These can be forgotten when the primary
// health problems are addressed.
//
// c) Indirect evidence, which points to something being wrong in another
// group, but we had to release resources and this is all that's left of
// that state.
//
// Observable health issues for metadata spanning the entire filesystem.

// Observable health issues for realtime group metadata.

// Observable health issues for AG metadata.

// Observable health issues for inode metadata.

// Don't propagate sick status to ag health summary during inactivation

// Primary evidence of health problems in a given group.

// Secondary state related to (but not primary evidence of) health problems.

// Evidence of health problems elsewhere.

// All health masks.

//
// These functions must be provided by the xfs implementation.  Function
// behavior with respect to the first argument should be as follows:
//
// xfs_*_mark_sick:        Set the sick flags and do not set checked flags.
// Runtime code should call this upon encountering
// a corruption.
//
// xfs_*_mark_corrupt:     Set the sick and checked flags simultaneously.
// Fsck tools should call this when corruption is
// found.
//
// xfs_*_mark_healthy:     Clear the sick flags and set the checked flags.
// Fsck tools should call this after correcting errors.
//
// xfs_*_measure_sickness: Return the sick and check status in the provided
// out parameters.
//
extern "C" {
    pub fn xfs_fs_mark_sick(mp: *mut xfs_mount, mask: c_uint);
}
extern "C" {
    pub fn xfs_fs_mark_corrupt(mp: *mut xfs_mount, mask: c_uint);
}
extern "C" {
    pub fn xfs_fs_mark_healthy(mp: *mut xfs_mount, mask: c_uint);
}
extern "C" {
    pub fn xfs_group_mark_sick(xg: *mut xfs_group, mask: c_uint);
}

extern "C" {
    pub fn xfs_group_mark_corrupt(xg: *mut xfs_group, mask: c_uint);
}
extern "C" {
    pub fn xfs_group_mark_healthy(xg: *mut xfs_group, mask: c_uint);
}
extern "C" {
    pub fn xfs_inode_mark_sick(ip: *mut xfs_inode, mask: c_uint);
}
extern "C" {
    pub fn xfs_inode_mark_corrupt(ip: *mut xfs_inode, mask: c_uint);
}
extern "C" {
    pub fn xfs_inode_mark_healthy(ip: *mut xfs_inode, mask: c_uint);
}
extern "C" {
    pub fn xfs_health_unmount(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_bmap_mark_sick(ip: *mut xfs_inode, whichfork: c_int);
}
extern "C" {
    pub fn xfs_btree_mark_sick(cur: *mut xfs_btree_cur);
}
extern "C" {
    pub fn xfs_dirattr_mark_sick(ip: *mut xfs_inode, whichfork: c_int);
}
extern "C" {
    pub fn xfs_da_mark_sick(args: *mut xfs_da_args);
}
// Now some helpers.

extern "C" {
    pub fn xfs_fsop_geom_health(mp: *mut xfs_mount, geo: *mut xfs_fsop_geom);
}
extern "C" {
    pub fn xfs_ag_geom_health(pag: *mut xfs_perag, ageo: *mut xfs_ag_geometry);
}
extern "C" {
    pub fn xfs_bulkstat_health(ip: *mut xfs_inode, bs: *mut xfs_bulkstat);
}

extern "C" {
    pub fn xfs_healthmon_inode_mask(sick_mask: c_uint) -> c_uint;
}
extern "C" {
    pub fn xfs_healthmon_rtgroup_mask(sick_mask: c_uint) -> c_uint;
}
extern "C" {
    pub fn xfs_healthmon_perag_mask(sick_mask: c_uint) -> c_uint;
}
extern "C" {
    pub fn xfs_healthmon_fs_mask(sick_mask: c_uint) -> c_uint;
}
