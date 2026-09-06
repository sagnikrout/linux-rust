//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blk_types.h
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
// Block data types and constants.  Directly include this file only to
// break include dependency loop.
//

extern "C" {
    pub fn void(: *mut bio_end_io_t) (struct bio) -> typedef;
}
//
// The basic unit of block I/O is a sector. It is used in a number of contexts
// in Linux (blk, bio, genhd). The size of one sector is 512 = 2**9
// bytes. Variables of type sector_t represent an offset or size that is a
// multiple of 512 bytes. Hence these two constants.
//

pub const SECTOR_SHIFT: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_device {
    pub bd_start_sect: sector_t,
    pub bd_nr_sectors: sector_t,
    pub bd_disk: *mut *mut gendisk,
    pub bd_queue: *mut *mut request_queue,
    pub bd_stats: *mut disk_stats __percpu,
    pub bd_stamp: c_ulong,
    pub flags: atomic_t __bd_flags; // partition number +,

    pub bd_dev: dev_t,
    pub /: *mut *mut *mut address_space bd_mapping; / page cache,
    pub bd_openers: core::sync::atomic::AtomicI32,
    pub /: *mut *mut spinlock_t bd_size_lock; / for bd_inode->i_size updates,
    pub bd_claiming: *mut *mut c_void,
    pub bd_holder: *mut *mut c_void,
    pub bd_holder_ops: *const blk_holder_ops,
    pub bd_holder_lock: mutex,
    pub bd_holders: c_int,
    pub bd_holder_dir: *mut kobject,
    pub /: *mut *mut atomic_t bd_fsfreeze_count; / >0 freeze requests, <0 freeze deniers,
    pub /: *mut *mut mutex bd_fsfreeze_mutex; / serialize freeze/thaw,
    pub bd_meta_info: *mut partition_meta_info,
    pub bd_writers: c_int,

    pub bd_security: *mut c_void,

//
// keep this out-of-line as it's both big and not needed in the fast
// path
//
    pub bd_device: device,
    pub __randomize_layout: },

//
// Block error status values.  See block/blk-core:blk_errors for the details.
//
pub type blk_status_t = u8 ;
pub type blk_short_t = u16;
pub const BLK_STS_OK: c_int = 0;

// hack for device mapper, don't use elsewhere:

//
// BLK_STS_AGAIN should only be returned if RQF_NOWAIT is set
// and the bio would block (cf bio_wouldblock_error())
//

//
// BLK_STS_DEV_RESOURCE is returned from the driver to the block layer if
// device related resources are unavailable, but the driver can guarantee
// that the queue will be rerun in the future once resources become
// available again. This is typically the case for device specific
// resources that are consumed for IO. If the driver fails allocating these
// resources, we know that inflight (or pending) IO will free these
// resource upon completion.
//
// This is different from BLK_STS_RESOURCE in that it explicitly references
// a device specific resource. For resources of wider scope, allocation
// failure can happen without having pending IO. This means that we can't
// rely on request completions freeing these resources, as IO may not be in
// flight. Examples of that are kernel memory allocations, DMA mappings, or
// any other system wide resources.
//

//
// BLK_STS_ZONE_OPEN_RESOURCE is returned from the driver in the completion
// path if the device returns a status indicating that too many zone resources
// are currently open. The same command should be successful if resubmitted
// after the number of open zones decreases below the device's limits, which is
// reported in the request_queue's max_open_zones.
//

//
// BLK_STS_ZONE_ACTIVE_RESOURCE is returned from the driver in the completion
// path if the device returns a status indicating that too many zone resources
// are currently active. The same command should be successful if resubmitted
// after the number of active zones decreases below the device's limits, which
// is reported in the request_queue's max_active_zones.
//

//
// BLK_STS_OFFLINE is returned from the driver when the target device is offline
// or is being taken offline. This could help differentiate the case where a
// device is intentionally being shut down from a real I/O error.
//

//
// BLK_STS_DURATION_LIMIT is returned from the driver when the target device
// aborted the command because it exceeded one of its Command Duration Limits.
//

//
// Invalid size or alignment.
//

//
// blk_path_error - returns true if error may be path related
// @error: status the request was completed with
//
// Description:
// This classifies block error status into non-retryable errors and ones
// that may be successful if retried on a failover path.
//
// Return:
// %false - retrying failover path will not help
// %true  - may succeed if retried
//
    pub false: return,
// Anything else could be a path failure, so should be retried
    pub true: return,
pub type blk_opf_t = __u32 ;
pub type blk_qc_t = c_uint;

//
// main unit of I/O for the block layer and lower layers (ie drivers and
// stacking drivers)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio {
    pub /: *mut *mut *mut bio bi_next; / request queue link,
    pub bi_bdev: *mut block_device,
    pub bits: *mut *mut blk_opf_t bi_opf; / bottom bits REQ_OP, top,
// req_flags.
//
    pub /: *mut *mut *mut unsigned short bi_flags; / BIO_ below,
    pub bi_ioprio: c_ushort,
    pub bi_write_hint: rw_hint,
    pub bi_write_stream: u8,
    pub bi_status: blk_status_t,
//
// The bvec gap bit indicates the lowest set bit in any address offset
// between all bi_io_vecs. This field is initialized only after the bio
// is split to the hardware limits (see bio_split_io_at()). The value
// may be used to consider DMA optimization when performing that
// mapping. The value is compared to a power of two mask where the
// result depends on any bit set within the mask, so saving the lowest
// bit is sufficient to know if any segment gap collides with the mask.
//
    pub bi_bvec_gap_bit: u8,
    pub __bi_remaining: core::sync::atomic::AtomicI32,
// The actual vec list, preserved by bio_reset()
    pub bi_io_vec: *mut bio_vec,
    pub bi_iter: bvec_iter,
// for polled bios:
    pub bi_cookie: blk_qc_t,
// for plugged zoned writes only:
    pub __bi_nr_segments: c_uint,
}

//
// Represents the association of the css and request_queue for the bio.
// If a bio goes direct to device, it will not have a blkg as it will
// not have a request_queue associated with it.  The reference is put
// on release of the bio.
//
// Time that this bio was issued.

//
// Everything starting with bi_max_vecs will be preserved by bio_reset()
//
// Number of elements in `bi_io_vec` that were allocated for this bio.
// Only used by the bio submitter to make `bio_add_page` fail once full
// and to free the `bi_io_vec` allocation. Must not be used in drivers
// and does not hold a useful value for cloned bios.
//

//
// bio flags
//
// throttling rules. Don't do it again.
// of this bio.
//
// This bio has completed bps throttling at the single tg granularity,
// which is different from BIO_BPS_THROTTLED. When the bio is enqueued
// into the sq->queued of the upper tg, or is about to be dispatched,
// this flag needs to be cleared. Since blk-throttle and rq_qos are not
// on the same hierarchical level, reuse the value.
//
pub type blk_mq_req_flags_t = __u32 ;
pub const REQ_OP_BITS: c_int = 8;

pub const REQ_FLAG_BITS: c_int = 24;
//
// enum req_op - Operations common to the bio and request structures.
// We use 8 bits for encoding the operation, and the remaining 24 for flags.
//
// The least significant bit of the operation number indicates the data
// transfer direction:
//
// - if the least significant bit is set transfers are TO the device
// - if the least significant bit is not set transfers are FROM the device
//
// If a operation does not transfer data the least significant bit has no
// meaning.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum req_op {
// @REQ_OP_READ: read sectors from the device
    REQ_OP_READ		= ( blk_opf_t)0,
// @REQ_OP_WRITE: write sectors to the device
    REQ_OP_WRITE		= ( blk_opf_t)1,
// @REQ_OP_FLUSH: flush the volatile write cache
    REQ_OP_FLUSH		= ( blk_opf_t)2,
// @REQ_OP_DISCARD: discard sectors
    REQ_OP_DISCARD		= ( blk_opf_t)3,
// @REQ_OP_SECURE_ERASE: securely erase sectors
    REQ_OP_SECURE_ERASE	= ( blk_opf_t)5,
// @REQ_OP_ZONE_APPEND: write data at the current zone write pointer
    REQ_OP_ZONE_APPEND	= ( blk_opf_t)7,
// @REQ_OP_WRITE_ZEROES: write the zero filled sector many times
    REQ_OP_WRITE_ZEROES	= ( blk_opf_t)9,
// @REQ_OP_ZONE_OPEN: Open a zone
    REQ_OP_ZONE_OPEN	= ( blk_opf_t)11,
// @REQ_OP_ZONE_CLOSE: Close a zone
    REQ_OP_ZONE_CLOSE	= ( blk_opf_t)13,
// @REQ_OP_ZONE_FINISH: Transition a zone to full
    REQ_OP_ZONE_FINISH	= ( blk_opf_t)15,
// @REQ_OP_ZONE_RESET: reset a zone write pointer
    REQ_OP_ZONE_RESET	= ( blk_opf_t)17,
// @REQ_OP_ZONE_RESET_ALL: reset all the zone present on the device
    REQ_OP_ZONE_RESET_ALL	= ( blk_opf_t)19,

// Driver private requests
// private:
    REQ_OP_DRV_IN		= ( blk_opf_t)34,
    REQ_OP_DRV_OUT		= ( blk_opf_t)35,

    REQ_OP_LAST		= ( blk_opf_t)36,
}

// Keep cmd_flag_name[] in sync with the definitions below
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum req_flag_bits {
    __REQ_FAILFAST_DEV =	/* no driver retries of device errors */
    REQ_OP_BITS,
    __REQ_FAILFAST_TRANSPORT, /* no driver retries of transport errors */
    __REQ_FAILFAST_DRIVER,	/* no driver retries of driver errors */
    __REQ_SYNC,		/* request is sync (sync write or read) */
    __REQ_META,		/* metadata io request */
    __REQ_PRIO,		/* boost priority in cfq */
    __REQ_NOMERGE,		/* don't touch this for merging */
    __REQ_IDLE,		/* anticipate more IO after this one */
    __REQ_INTEGRITY,	/* I/O includes block integrity payload */
    __REQ_FUA,		/* forced unit access */
    __REQ_PREFLUSH,		/* request for cache flush */
    __REQ_RAHEAD,		/* read ahead, can fail anytime */
    __REQ_BACKGROUND,	/* background IO */
    __REQ_NOWAIT,           /* Don't wait if request will block */
    __REQ_POLLED,		/* caller polls for completion using bio_poll */
    __REQ_ALLOC_CACHE,	/* allocate IO from cache if available */
    __REQ_SWAP,		/* swap I/O */
    __REQ_DRV,		/* for driver use */
    __REQ_FS_PRIVATE,	/* for file system (submitter) use */
    __REQ_ATOMIC,		/* for atomic write operations */
//
// Command specific flags, keep last:
//
// for REQ_OP_WRITE_ZEROES:
    __REQ_NOUNMAP,		/* do not free blocks when zeroing */

    __REQ_NR_BITS,		/* stops here */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stat_group {
    STAT_READ,
    STAT_WRITE,
    STAT_DISCARD,
    STAT_FLUSH,

    NR_STAT_GROUPS
}

//
// Check if the bio or request is one that needs special treatment in the
// flush state machine.
//
// Reads are always treated as synchronous, as are requests with the FUA or
// PREFLUSH flag.  Other operations may be marked as synchronous using the
// REQ_SYNC flag.
//
// Check if a bio or request operation is a zone management operation.
//
extern "C" {
    pub fn op_is_write(_arg: op) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_rq_stat {
    pub mean: u64,
    pub min: u64,
    pub max: u64,
    pub nr_samples: u32,
    pub batch: u64,
}
