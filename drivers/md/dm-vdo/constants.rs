//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/constants.h
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
// Copyright 2023 Red Hat
//

//
// The maximum number of contiguous PBNs which will go to a single bio submission queue,
// assuming there is more than one queue.
//
// The number of entries on a block map page
// The origin of the flat portion of the block map
//
// The height of a block map tree. Assuming a root count of 60 and 812 entries per page,
// this is big enough to represent almost 95 PB of logical space.
//
// The default number of bio submission queues.
// The number of contiguous PBNs to be submitted to a single bio queue.
// The number of trees in the arboreal block map
// The default size of the recovery journal, in blocks
// The default size of each slab journal, in blocks
// The recovery journal starting sequence number set at format time
//
// The initial size of lbn_operations and pbn_operations, which is based upon the expected
// maximum number of outstanding VIOs. This value was chosen to make it highly unlikely
// that the maps would need to be resized.
//
// The maximum number of logical zones
// The maximum number of physical zones
// The default blocks in one slab
// The minimum blocks in one slab
// The maximum blocks in one slab
// The maximum number of slabs the slab depot supports
//
// The maximum number of block map pages to load simultaneously during recovery or rebuild.
//
// The maximum number of entries in the slab summary
// The maximum number of total threads in a VDO thread configuration.
// The maximum number of VIOs in the system at once
// The only physical block size supported by VDO
// The number of sectors per block
// The size of a sector that will not be torn
// The physical block number reserved for storing the zero block
