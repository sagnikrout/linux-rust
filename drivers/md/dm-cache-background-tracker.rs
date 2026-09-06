//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-cache-background-tracker.h
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
// Copyright (C) 2017 Red Hat. All rights reserved.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
//
// The cache policy decides what background work should be performed,
// such as promotions, demotions and writebacks. The core cache target
// is in charge of performing the work, and does so when it sees fit.
//
// The background_tracker acts as a go between. Keeping track of future
// work that the policy has decided upon, and handing (issuing) it to
// the core target when requested.
//
// There is no locking in this, so calls will probably need to be
// protected with a spinlock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_work {
    pub list: list_head,
    pub node: rb_node,
    pub work: policy_work,
}

//
// Create a new tracker, it will not be able to queue more than
// 'max_work' entries.
//
// Destroy the tracker. No issued, but not complete, work should
// exist when this is called. It is fine to have queued but unissued
// work.
//
extern "C" {
    pub fn btracker_destroy(b: *mut background_tracker);
}
extern "C" {
    pub fn btracker_nr_demotions_queued(b: *mut background_tracker) -> c_uint;
}
//
// Queue some work within the tracker. 'work' should point to the work
// to queue, this will be copied (ownership doesn't pass).  If pwork
// is not NULL then it will be set to point to the tracker's internal
// copy of the work.
//
// returns -EINVAL iff the work is already queued.  -ENOMEM if the work
// couldn't be queued for another reason.
//
// Hands out the next piece of work to be performed.
// Returns -ENODATA if there's no work.
//
extern "C" {
    pub fn btracker_issue(b: *mut background_tracker, work: *mut policy_work) -> c_int;
}
//
// Informs the tracker that the work has been completed and it may forget
// about it.
//
extern "C" {
    pub fn btracker_complete(b: *mut background_tracker, op: *mut policy_work);
}
//
// Predicate to see if an origin block is already scheduled for promotion.
//
// ----------------------------------------------------------------
