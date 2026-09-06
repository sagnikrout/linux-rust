//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/block.h
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

pub const RWBS_LEN: c_int = 10;

//
// block_touch_buffer - mark a buffer accessed
// @bh: buffer_head being touched
//
// Called from touch_buffer().
//
// block_dirty_buffer - mark a buffer dirty
// @bh: buffer_head being dirtied
//
// Called from mark_buffer_dirty().
//

//
// block_rq_requeue - place block IO request back on a queue
// @rq: block IO operation request
//
// The block operation request @rq is being placed back into queue
// @q.  For some reason the request was not completed and needs to be
// put back in the queue.
//
// block_rq_complete - block IO operation completed by device driver
// @rq: block operations request
// @error: status code
// @nr_bytes: number of completed bytes
//
// The block_rq_complete tracepoint event indicates that some portion
// of operation request has been completed by the device driver.  If
// the @rq->bio is %NULL, then there is absolutely no additional work to
// do for the request. If @rq->bio is non-NULL then there is
// additional work required to complete the request.
//
// block_rq_error - block IO operation error reported by device driver
// @rq: block operations request
// @error: status code
// @nr_bytes: number of completed bytes
//
// The block_rq_error tracepoint event indicates that some portion
// of operation request has failed as reported by the device driver.
//
// block_rq_tag_wait - triggered when a request is starved of a tag
// @q: request queue of the target device
// @hctx: hardware context of the request experiencing starvation
// @is_sched_tag: indicates whether the starved pool is the software scheduler
// @alloc_flags: allocation flags dictating the specific tag pool
//
// Called immediately before the submitting context is forced to block due
// to the exhaustion of available tags (i.e., physical hardware driver
// tags, software scheduler tags, or reserved tags). This trace point
// indicates that the context will be placed into an uninterruptible state
// via sbitmap_prepare_to_wait(). If a tag is not acquired in the final
// lockless retry, the context will yield the CPU via io_schedule() until
// an active request completes and relinquishes its assigned tag.
//
// block_rq_insert - insert block operation request into queue
// @rq: block IO operation request
//
// Called immediately before block operation request @rq is inserted
// into queue @q.  The fields in the operation request @rq struct can
// be examined to determine which device and sectors the pending
// operation would access.
//
// block_rq_issue - issue pending block IO request operation to device driver
// @rq: block IO operation request
//
// Called when block operation request @rq from queue @q is sent to a
// device driver for processing.
//
// block_rq_merge - merge request with another one in the elevator
// @rq: block IO operation request
//
// Called when block operation request @rq from queue @q is merged to another
// request queued in the elevator.
//
// block_io_start - insert a request for execution
// @rq: block IO operation request
//
// Called when block operation request @rq is queued for execution
//
// block_io_done - block IO operation request completed
// @rq: block IO operation request
//
// Called when block operation request @rq is completed
//
// block_bio_complete - completed all work on the block operation
// @q: queue holding the block operation
// @bio: block operation completed
//
// This tracepoint indicates there is no further work to do on this
// block IO operation @bio.
//
// block_bio_backmerge - merging block operation to the end of an existing operation
// @bio: new block operation to merge
//
// Merging block request @bio to the end of an existing block request.
//
// block_bio_frontmerge - merging block operation to the beginning of an existing operation
// @bio: new block operation to merge
//
// Merging block IO operation @bio to the beginning of an existing block request.
//
// block_bio_queue - putting new block IO operation in queue
// @bio: new block operation
//
// About to place the block IO operation @bio into queue @q.
//
// block_getrq - get a free request entry in queue for block IO operations
// @bio: pending block IO operation (can be %NULL)
//
// A request struct has been allocated to handle the block IO operation @bio.
//
// blk_zone_append_update_request_bio - update bio sector after zone append
// @rq: the completed request that sets the bio sector
//
// Update the bio's bi_sector after a zone append command has been completed.
//
// block_plug - keep operations requests in request queue
// @q: request queue to plug
//
// Plug the request queue @q.  Do not allow block operation requests
// to be sent to the device driver. Instead, accumulate requests in
// the queue to improve throughput performance of the block device.
//
// block_unplug - release of operations requests in request queue
// @q: request queue to unplug
// @depth: number of requests just added to the queue
// @explicit: whether this was an explicit unplug, or one from schedule()
//
// Unplug request queue @q because device driver is scheduled to work
// on elements in the request queue.
//
// block_split - split a single bio struct into two bio structs
// @bio: block operation being split
// @new_sector: The starting sector for the new bio
//
// The bio request @bio needs to be split into two bio requests.  The newly
// created @bio request starts at @new_sector. This split may be required due to
// hardware limitations such as operation crossing device boundaries in a RAID
// system.
//
// block_bio_remap - map request for a logical device to the raw device
// @bio: revised operation
// @dev: original device for the operation
// @from: original sector for the operation
//
// An operation for a logical device has been mapped to the
// raw block device.
//
// block_rq_remap - map request for a block operation request
// @rq: block IO operation request
// @dev: device for the operation
// @from: original sector for the operation
//
// The block operation request @rq in @q has been remapped.  The block
// operation request @rq holds the current information and @from hold
// the original sector.
//
// blkdev_zone_mgmt - Execute a zone management operation on a range of zones
// @bio: The block IO operation sent down to the device
// @nr_sectors: The number of sectors affected by this operation
//
// Execute a zone management operation on a specified range of zones. This
// range is encoded in %nr_sectors, which has to be a multiple of the zone
// size.
//

// This part must be outside protection
