//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blk-integrity.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blk_integrity_flags {
    BLK_INTEGRITY_NOVERIFY		= 1 << 0,
    BLK_INTEGRITY_NOGENERATE	= 1 << 1,
    BLK_INTEGRITY_DEVICE_CAPABLE	= 1 << 2,
    BLK_INTEGRITY_REF_TAG		= 1 << 3,
    BLK_INTEGRITY_STACKED		= 1 << 4,
    BLK_SPLIT_INTERVAL_CAPABLE	= 1 << 5,
}

extern "C" {
    pub fn queue_limits_stack_integrity(_arg: t, _arg: &bdev->bd_disk->queue->limits) -> return;
}

extern "C" {
    pub fn blk_rq_map_integrity_sg(: *mut request, : *mut scatterlist) -> c_int;
}
extern "C" {
    pub fn blk_rq_count_integrity_sg(: *mut request_queue, : *mut bio) -> c_int;
}
extern "C" {
    pub fn blk_get_integrity(_arg: bdev->bd_disk) -> return;
}
//
// bio_integrity_intervals - Return number of integrity intervals for a bio
// @bi:		blk_integrity profile for device
// @sectors:	Size of the bio in 512-byte sectors
//
// Description: The block layer calculates everything in 512 byte
// sectors but integrity metadata is done in terms of the data integrity
// interval size of the storage device.  Convert the block layer sectors
// to the appropriate number of integrity intervals.
//
// Return the current bvec that contains the integrity data. bip_iter may be
// advanced to iterate over the integrity data.
//

// the optimizer will remove all calls to this function

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bio_integrity_action {
    BI_ACT_BUFFER		= (1u << 0),	/* allocate buffer */
    BI_ACT_CHECK		= (1u << 1),	/* generate / verify PI */
    BI_ACT_ZERO		= (1u << 2),	/* zero buffer */
}

//
// bio_integrity_action - return the integrity action needed for a bio
// @bio:	bio to operate on
//
// Returns the mask of integrity actions (BI_ACT_*) that need to be performed
// for @bio.
//
extern "C" {
    pub fn __bio_integrity_action(bio: *mut bio) -> c_uint;
}
extern "C" {
    pub fn __bio_integrity_action(_arg: bio) -> return;
}
