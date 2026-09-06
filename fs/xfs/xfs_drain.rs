//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_drain.h
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

//
// Passive drain mechanism.  This data structure tracks a count of some items
// and contains a waitqueue for callers who would like to wake up when the
// count hits zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_defer_drain {
// Number of items pending in some part of the filesystem.
    pub dr_count: core::sync::atomic::AtomicI32,
// Queue to wait for dri_count to go to zero
    pub dr_waiters: wait_queue_head,
}

extern "C" {
    pub fn xfs_defer_drain_init(dr: *mut xfs_defer_drain);
}
extern "C" {
    pub fn xfs_defer_drain_free(dr: *mut xfs_defer_drain);
}
extern "C" {
    pub fn xfs_defer_drain_wait_disable();
}
extern "C" {
    pub fn xfs_defer_drain_wait_enable();
}
//
// Deferred Work Intent Drains
// ===========================
//
// When a writer thread executes a chain of log intent items, the AG header
// buffer locks will cycle during a transaction roll to get from one intent
// item to the next in a chain.  Although scrub takes all AG header buffer
// locks, this isn't sufficient to guard against scrub checking an AG while
// that writer thread is in the middle of finishing a chain because there's no
// higher level locking primitive guarding allocation groups.
//
// When there's a collision, cross-referencing between data structures (e.g.
// rmapbt and refcountbt) yields false corruption events; if repair is running,
// this results in incorrect repairs, which is catastrophic.
//
// The solution is to the perag structure the count of active intents and make
// scrub wait until it has both AG header buffer locks and the intent counter
// reaches zero.  It is therefore critical that deferred work threads hold the
// AGI or AGF buffers when decrementing the intent counter.
//
// Given a list of deferred work items, the deferred work manager will complete
// a work item and all the sub-items that the parent item creates before moving
// on to the next work item in the list.  This is also true for all levels of
// sub-items.  Writer threads are permitted to queue multiple work items
// targetting the same AG, so a deferred work item (such as a BUI) that creates
// sub-items (such as RUIs) must bump the intent counter and maintain it until
// the sub-items can themselves bump the intent counter.
//
// Therefore, the intent count tracks entire lifetimes of deferred work items.
// All functions that create work items must increment the intent counter as
// soon as the item is added to the transaction and cannot drop the counter
// until the item is finished or cancelled.
//
// The same principles apply to realtime groups because the rt metadata inode
// ILOCKs are not held across transaction rolls.
//
extern "C" {
    pub fn xfs_group_intent_put(rtg: *mut xfs_group);
}
extern "C" {
    pub fn xfs_group_intent_drain(xg: *mut xfs_group) -> c_int;
}
extern "C" {
    pub fn xfs_group_intent_busy(xg: *mut xfs_group) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_defer_drain {

