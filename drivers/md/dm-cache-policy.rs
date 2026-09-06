//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-cache-policy.h
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
// Copyright (C) 2012 Red Hat. All rights reserved.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
//
// The cache policy makes the important decisions about which blocks get to
// live on the faster cache device.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum policy_operation {
    POLICY_PROMOTE,
    POLICY_DEMOTE,
    POLICY_WRITEBACK
}

//
// This is the instruction passed back to the core target.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct policy_work {
    pub op: policy_operation,
    pub oblock: dm_oblock_t,
    pub cblock: dm_cblock_t,
}

//
// The cache policy object.  It is envisaged that this structure will be
// embedded in a bigger, policy specific structure (ie. use container_of()).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_cache_policy {
//
// Destroys this object.
//
    pub p): *mut *mut void (destroy)(struct dm_cache_policy,
//
// Find the location of a block.
//
// Must not block.
//
// Returns 0 if in cache (cblock will be set), -ENOENT if not, < 0 for
// other errors (-EWOULDBLOCK would be typical).  data_dir should be
// READ or WRITE. fast_copy should be set if migrating this block would
// be 'cheap' somehow (eg, discarded data). background_queued will be set
// if a migration has just been queued.
//
    pub background_queued): *mut int data_dir, bool fast_copy, bool,
//
// Sometimes the core target can optimise a migration, eg, the
// block may be discarded, or the bio may cover an entire block.
// In order to optimise it needs the migration immediately though
// so it knows to do something different with the bio.
//
// This method is optional (policy-internal will fallback to using
// lookup).
//
    pub work): *mut policy_work,
//
// Retrieves background work.  Returns -ENODATA when there's no
// background work.
//
    pub result): *mut policy_work,
//
// You must pass in the same work pointer that you were given, not
// a copy.
//
    pub success): bool,
    pub cblock): *mut *mut *mut void (set_dirty)(struct dm_cache_policy p, dm_cblock_t,
    pub cblock): *mut *mut *mut void (clear_dirty)(struct dm_cache_policy p, dm_cblock_t,
//
// Called when a cache target is first created.  Used to load a
// mapping from the metadata device into the policy.
//
    pub hint_valid): uint32_t hint, bool,
//
// Drops the mapping, irrespective of whether it's clean or dirty.
// Returns -ENODATA if cblock is not mapped.
//
    pub cblock): *mut *mut *mut int (invalidate_mapping)(struct dm_cache_policy p, dm_cblock_t,
//
// Gets the hint for a given cblock.  Called in a single threaded
// context.  So no locking required.
//
    pub cblock): *mut *mut *mut uint32_t (get_hint)(struct dm_cache_policy p, dm_cblock_t,
//
// How full is the cache?
//
    pub p): *mut *mut dm_cblock_t (residency)(struct dm_cache_policy,
//
// Because of where we sit in the block layer, we can be asked to
// map a lot of little bios that are all in the same block (no
// queue merging has occurred).  To stop the policy being fooled by
// these, the core target sends regular tick() calls to the policy.
// The policy should only count an entry as hit once per tick.
//
// This method is optional.
//
    pub can_block): *mut *mut *mut void (tick)(struct dm_cache_policy p, bool,
//
// Configuration.
//
    pub sz_ptr): *mut unsigned int maxlen, ssize_t,
    pub value): *const *const char key, char,
    pub allow): *mut *mut *mut void (allow_migrations)(struct dm_cache_policy p, bool,
//
// Book keeping ptr for the policy register, not for general use.
//
    pub private: *mut c_void,
}

// ----------------------------------------------------------------
//
// We maintain a little register of the different policy types.
//
pub const CACHE_POLICY_NAME_SIZE: c_int = 16;
pub const CACHE_POLICY_VERSION_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_cache_policy_type {
// For use by the register code only.
    pub list: list_head,
//
// Policy writers should fill in these fields.  The name field is
// what gets passed on the target line to select your policy.
//
    pub name: [c_char; CACHE_POLICY_NAME_SIZE],
    pub version: [c_uint; CACHE_POLICY_VERSION_SIZE],
//
// For use by an alias dm_cache_policy_type to point to the
// real dm_cache_policy_type.
//
    pub real: *mut dm_cache_policy_type,
//
// Policies may store a hint for each cache block.
// Currently the size of this hint must be 0 or 4 bytes but we
// expect to relax this in future.
//
    pub hint_size: usize,
    pub owner: *mut module,
    pub block_size): sector_t,
}

extern "C" {
    pub fn dm_cache_policy_register(type: *mut dm_cache_policy_type) -> c_int;
}
extern "C" {
    pub fn dm_cache_policy_unregister(type: *mut dm_cache_policy_type);
}
// ----------------------------------------------------------------
