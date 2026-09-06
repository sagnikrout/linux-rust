//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/raid5.h
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
// Each stripe contains one buffer per device.  Each buffer can be in
// one of a number of states stored in "flags".  Changes between
// these states happen *almost* exclusively under the protection of the
// STRIPE_ACTIVE flag.  Some very specific changes can happen in bi_end_io, and
// these are not protected by STRIPE_ACTIVE.
//
// The flag bits that are used to represent these states are:
// R5_UPTODATE and R5_LOCKED
//
// State Empty == !UPTODATE, !LOCK
// We have no data, and there is no active request
// State Want == !UPTODATE, LOCK
// A read request is being submitted for this block
// State Dirty == UPTODATE, LOCK
// Some new data is in this buffer, and it is being written out
// State Clean == UPTODATE, !LOCK
// We have valid data which is the same as on disc
//
// The possible state transitions are:
//
// Empty -> Want   - on read or write to get old data for  parity calc
// Empty -> Dirty  - on compute_parity to satisfy write/sync request.
// Empty -> Clean  - on compute_block when computing a block for failed drive
// Want  -> Empty  - on failed read
// Want  -> Clean  - on successful completion of read request
// Dirty -> Clean  - on successful completion of write request
// Dirty -> Clean  - on failed write
// Clean -> Dirty  - on compute_parity to satisfy write/sync (RECONSTRUCT or RMW)
//
// The Want->Empty, Want->Clean, Dirty->Clean, transitions
// all happen in end_io at interrupt time.
// Each sets the Uptodate bit before releasing the Lock bit.
// This leaves one multi-stage transition:
// Want->Dirty->Clean
// This is safe because thinking that a Clean buffer is actually dirty
// will at worst delay some action, and the stripe will be scheduled
// for attention after the transition is complete.
//
// There is one possibility that is not covered by these states.  That
// is if one drive has failed and there is a spare being rebuilt.  We
// can't distinguish between a clean block that has been generated
// from parity calculations, and a clean block that has been
// successfully written to the spare ( or to parity when resyncing).
// To distinguish these states we have a stripe bit STRIPE_INSYNC that
// is set whenever a write is scheduled to the spare, or to the parity
// disc if there is no spare.  A sync request clears this bit, and
// when we find it set with no buffers locked, we know the sync is
// complete.
//
// Buffers for the md device that arrive via make_request are attached
// to the appropriate stripe in one of two lists linked on b_reqnext.
// One list (bh_read) for read requests, one (bh_write) for write.
// There should never be more than one buffer on the two lists
// together, but we are not guaranteed of that so we allow for more.
//
// If a buffer is on the read list when the associated cache buffer is
// Uptodate, the data is copied into the read buffer and it's end_io
// routine is called.  This may happen in the end_request routine only
// if the buffer has just successfully been read.  end_request should
// remove the buffers from the list and then set the Uptodate bit on
// the buffer.  Other threads may do this only if they first check
// that the Uptodate bit is set.  Once they have checked that they may
// take buffers off the read queue.
//
// When a buffer on the write list is committed for write it is copied
// into the cache buffer, which is then marked dirty, and moved onto a
// third list, the written list (bh_written).  Once both the parity
// block and the cached buffer are successfully written, any buffer on
// a written list can be returned with end_io.
//
// The write list and read list both act as fifos.  The read list,
// write list and written list are protected by the device_lock.
// The device_lock is only for list manipulations and will only be
// held for a very short time.  It can be claimed from interrupts.
//
// Stripes in the stripe cache can be on one of two lists (or on
// neither).  The "inactive_list" contains stripes which are not
// currently being used for any request.  They can freely be reused
// for another stripe.  The "handle_list" contains stripes that need
// to be handled in some way.  Both of these are fifo queues.  Each
// stripe is also (potentially) linked to a hash bucket in the hash
// table so that it can be found by sector number.  Stripes that are
// not hashed must be on the inactive_list, and will normally be at
// the front.  All stripes start life this way.
//
// The inactive_list, handle_list and hash bucket lists are all protected by the
// device_lock.
// - stripes have a reference counter. If count==0, they are on a list.
// - If a stripe might need handling, STRIPE_HANDLE is set.
// - When refcount reaches zero, then if STRIPE_HANDLE it is put on
// handle_list else inactive_list
//
// This, combined with the fact that STRIPE_HANDLE is only ever
// cleared while a stripe has a non-zero count means that if the
// refcount is 0 and STRIPE_HANDLE is set, then it is on the
// handle_list and if recount is 0 and STRIPE_HANDLE is not set, then
// the stripe is on inactive_list.
//
// The possible transitions are:
// activate an unhashed/inactive stripe (get_active_stripe())
// lockdev check-hash unlink-stripe cnt++ clean-stripe hash-stripe unlockdev
// activate a hashed, possibly active stripe (get_active_stripe())
// lockdev check-hash if(!cnt++)unlink-stripe unlockdev
// attach a request to an active stripe (add_stripe_bh())
// lockdev attach-buffer unlockdev
// handle a stripe (handle_stripe())
// setSTRIPE_ACTIVE,  clrSTRIPE_HANDLE ...
// (lockdev check-buffers unlockdev) ..
// change-state ..
// record io/ops needed clearSTRIPE_ACTIVE schedule io/ops
// release an active stripe (release_stripe())
// lockdev if (!--cnt) { if  STRIPE_HANDLE, add to handle_list else add to inactive-list } unlockdev
//
// The refcount counts each thread that have activated the stripe,
// plus raid5d if it is handling it, plus one for each active request
// on a cached buffer, and plus one if the stripe is undergoing stripe
// operations.
//
// The stripe operations are:
// -copying data between the stripe cache and user application buffers
// -computing blocks to save a disk access, or to recover a missing block
// -updating the parity on a write operation (reconstruct write and
// read-modify-write)
// -checking parity correctness
// -running i/o to disk
// These operations are carried out by raid5_run_ops which uses the async_tx
// api to (optionally) offload operations to dedicated hardware engines.
// When requesting an operation handle_stripe sets the pending bit for the
// operation and increments the count.  raid5_run_ops is then run whenever
// the count is non-zero.
// There are some critical dependencies between the operations that prevent some
// from being requested while another is in flight.
// 1/ Parity check operations destroy the in cache version of the parity block,
// so we prevent parity dependent operations like writes and compute_blocks
// from starting while a check is in progress.  Some dma engines can perform
// the check without damaging the parity block, in these cases the parity
// block is re-marked up to date (assuming the check was successful) and is
// not re-read from disk.
// 2/ When a write operation is requested we immediately lock the affected
// blocks, and mark them as not up to date.  This causes new read requests
// to be held off, as well as parity checks and compute block operations.
// 3/ Once a compute block operation has been requested handle_stripe treats
// that block as if it is up to date.  raid5_run_ops guaruntees that any
// operation that is dependent on the compute block result is initiated after
// the compute block completes.
//
// Operations state - intermediate states that are visible outside of
// STRIPE_ACTIVE.
// In general _idle indicates nothing is running, _run indicates a data
// processing operation is active, and _result means the data processing result
// is stable and can be acted upon.  For simple operations like biofill and
// compute that only have an _idle and _run state they are indicated with
// sh->state flags (STRIPE_BIOFILL_RUN and STRIPE_COMPUTE_RUN)
//
// enum check_states - handles syncing / repairing a stripe
// @check_state_idle - check operations are quiesced
// @check_state_run - check operation is running
// @check_state_result - set outside lock when check result is valid
// @check_state_compute_run - check failed and we are repairing
// @check_state_compute_result - set outside lock when compute result is valid
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum check_states {
    check_state_idle = 0,
    check_state_run, /* xor parity check */
    check_state_run_q, /* q-parity check */
    check_state_run_pq, /* pq dual parity check */
    check_state_check_result,
    check_state_compute_run, /* parity repair */
    check_state_compute_result,
}

//
// enum reconstruct_states - handles writing or expanding a stripe
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reconstruct_states {
    reconstruct_state_idle = 0,
    reconstruct_state_prexor_drain_run,	/* prexor-write */
    reconstruct_state_drain_run,		/* write */
    reconstruct_state_run,			/* expand */
    reconstruct_state_prexor_drain_result,
    reconstruct_state_drain_result,
    reconstruct_state_result,
}

pub const DEFAULT_STRIPE_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stripe_head {
    pub hash: hlist_node,
    pub /: *mut *mut list_head lru; / inactive_list or handle_list,
    pub release_list: llist_node,
    pub raid_conf: *mut r5conf,
    pub every: *mut *mut short generation; / increments with,
// reshape
    pub /: *mut *mut sector_t sector; / sector of this row,
    pub /: *mut *mut short pd_idx; / parity disk index,
    pub /: *mut *mut short qd_idx; / 'Q' disk index for raid6,
    pub /: *mut *mut short ddf_layout;/ use DDF ordering to calculate Q,
    pub hash_lock_index: c_short,
    pub /: *mut *mut unsigned long state; / state flags,
    pub /: *mut *mut atomic_t count; / nr of active thread/requests,
    pub /: *mut *mut int bm_seq; / sequence number for bitmap flushes,
    pub /: *mut *mut int disks; / disks in stripe,
    pub stripe,: *mut *mut int overwrite_disks; / total overwrite disks in,
// this is only checked when stripe
// has STRIPE_BATCH_READY
//
    pub check_state: check_states,
    pub reconstruct_state: reconstruct_states,
    pub stripe_lock: spinlock_t,
    pub cpu: c_int,
    pub group: *mut r5worker_group,
    pub /: *mut *mut *mut stripe_head batch_head; / protected by stripe lock,
    pub /: *mut *mut spinlock_t batch_lock; / only header's lock is useful,
    pub lock*/: *mut *mut list_head batch_list; / protected by head's batch,
    pub log_io: *mut r5l_io_unit,
    pub ppl_io: *mut ppl_io_unit,
}

//
// struct stripe_operations
// @target - STRIPE_OP_COMPUTE_BLK target
// @target2 - 2nd compute target in the raid6 case
// @zero_sum_result - P and Q verification flags
// @request - async service request flags for raid_run_ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stripe_operations {
    pub target2: int target,,
    pub zero_sum_result: sum_check_flags,
    pub ops: },

// These pages will be used by bios in dev[i]
    pub pages: *mut page,
    pub /: *mut *mut int nr_pages; / page array size,
    pub stripes_per_page: c_int,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r5dev {
// rreq and rvec are used for the replacement device when
// writing data to both devices.
//
    pub rreq: bio req,,
    pub rvec: bio_vec vec,,
    pub orig_page: *mut *mut page page,,
    pub /: *mut *mut unsigned int offset; / offset of the page,
    pub written: *mut *mut *mut *mut bio toread, read, towrite,,
    pub /: *mut *mut sector_t sector; / sector of this page,
    pub flags: c_ulong,
    pub log_checksum: u32,
    pub write_hint: c_ushort,
    pub /: *mut *mut } dev[]; / allocated depending of RAID geometry ("disks" member),
}

// stripe_head_state - collects and tracks the dynamic state of a stripe_head
// for handle_stripe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stripe_head_state {
// 'syncing' means that we need to read all devices, either
// to check/correct parity, or to reconstruct a missing device.
// 'replacing' means we are replacing one or more drives and
// the source is valid at this point so we don't need to
// read all devices, just the replacement targets.
//
    pub replacing: int syncing, expanding, expanded,,
    pub written: int locked, uptodate, to_read, to_write, failed,,
    pub non_overwrite: int to_fill, compute, req_compute,,
    pub just_cached: int injournal,,
    pub failed_num: [c_int; 2],
    pub q_failed: int p_failed,,
    pub dec_preread_active: c_int,
    pub ops_request: c_ulong,
    pub blocked_rdev: *mut md_rdev,
    pub handle_bad_blocks: c_int,
    pub log_failed: c_int,
    pub waiting_extra_page: c_int,
}

// Flags for struct r5dev.flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r5dev_flags {
    R5_UPTODATE,	/* page contains current data */
    R5_LOCKED,	/* IO has been submitted on "req" */
    R5_DOUBLE_LOCKED,/* Cannot clear R5_LOCKED until 2 writes complete */
    R5_OVERWRITE,	/* towrite covers whole page */
// and some that are internal to handle_stripe
    R5_Insync,	/* rdev && rdev->in_sync at start */
    R5_Wantread,	/* want to schedule a read */
    R5_Wantwrite,
    R5_Overlap,	/* There is a pending overlapping request
// on this block
    R5_ReadNoMerge, /* prevent bio from merging in block-layer */
    R5_ReadError,	/* seen a read error here recently */
    R5_ReWrite,	/* have tried to over-write the readerror */

    R5_Expanded,	/* This block now has post-expand data */
    R5_Wantcompute,	/* compute_block in progress treat as
// uptodate
//
    R5_Wantfill,	/* dev->toread contains a bio that needs
// filling
//
    R5_Wantdrain,	/* dev->towrite needs to be drained */
    R5_WantFUA,	/* Write should be FUA */
    R5_SyncIO,	/* The IO is sync */
    R5_WriteError,	/* got a write error - need to record it */
    R5_MadeGood,	/* A bad block has been fixed by writing to it */
    R5_ReadRepl,	/* Will/did read from replacement rather than orig */
    R5_MadeGoodRepl,/* A bad block on the replacement device has been
// fixed by writing to it
    R5_NeedReplace,	/* This device has a replacement which is not
// up-to-date at this stripe.
    R5_WantReplace, /* We need to update the replacement, we have read
// data in, and now is a good time to write it out.
//
    R5_Discard,	/* Discard the stripe */
    R5_SkipCopy,	/* Don't copy data from bio to stripe cache */
    R5_InJournal,	/* data being written is in the journal device.
// if R5_InJournal is set for parity pd_idx, all the
// data and parity being written are in the journal
// device
//
    R5_OrigPageUPTDODATE,	/* with write back cache, we read old data into
// dev->orig_page for prexor. When this flag is
// set, orig_page contains latest data in the
// raid disk.
//
}

//
// Stripe state
//
// this bit is used in two scenarios:
//
// 1. write-out phase
// set in first entry of r5l_write_stripe
// clear in second entry of r5l_write_stripe
// used to bypass logic in handle_stripe
//
// 2. caching phase
// set in r5c_try_caching_write()
// clear when journal write is done
// used to initiate r5c_cache_data()
// also used to bypass logic in handle_stripe
//
// see more detail in the raid5-cache.c
//
// in conf->r5c_partial_stripe_list)
//
// in conf->r5c_full_stripe_list)
//

//
// Operation request flags
//
// RAID parity calculation preferences
//
// Pages requested from set_syndrome_sources()
//
// Plugging:
//
// To improve write throughput, we need to delay the handling of some
// stripes until there has been a chance that several write requests
// for the one stripe have all been collected.
// In particular, any write request that would require pre-reading
// is put on a "delayed" queue until there are no stripes currently
// in a pre-read phase.  Further, if the "delayed" queue is empty when
// a stripe is put on it then we "plug" the queue and do not process it
// until an unplug call is made. (the unplug_io_fn() is called).
//
// When preread is initiated on a stripe, we set PREREAD_ACTIVE and add
// it to the count of prereading stripes.
// When write is initiated, or the stripe refcnt == 0 (just in case) we
// clear the PREREAD_ACTIVE flag and decrement the count
// Whenever the 'handle' queue is empty and the device is not plugged, we
// move any strips from delayed to handle and clear the DELAYED flag and set
// PREREAD_ACTIVE.
// In stripe_handle, if we find pre-reading is necessary, we do it if
// PREREAD_ACTIVE is set, else we set DELAYED which will send it to the delayed queue.
// HANDLE gets cleared if stripe_handle leaves nothing locked.
//
// Note: disk_info.rdev can be set to NULL asynchronously by raid5_remove_disk.
// There are three safe ways to access disk_info.rdev.
// 1/ when holding mddev->reconfig_mutex
// 2/ when resync/recovery/reshape is known to be happening - i.e. in code that
// is called as part of performing resync/recovery/reshape.
// 3/ while holding rcu_read_lock(), use rcu_dereference to get the pointer
// and if it is non-NULL, increment rdev->nr_pending before dropping the RCU
// lock.
// When .rdev is set to NULL, the nr_pending count checked again and if
// it has been incremented, the pointer is put back in .rdev.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct disk_info {
    pub rdev: *mut md_rdev,
    pub replacement: *mut md_rdev,
    pub /: *mut *mut *mut page extra_page; / extra page to use in prexor,
}

//
// Stripe cache
//
pub const NR_STRIPES: c_int = 256;

pub const IO_THRESHOLD: c_int = 1;
pub const BYPASS_THRESHOLD: c_int = 1;

pub const MAX_STRIPE_BATCH: c_int = 8;
// NOTE NR_STRIPE_HASH_LOCKS must remain below 64.
// This is because we sometimes take all the spinlocks
// and creating that much locking depth can cause
// problems.
//
pub const NR_STRIPE_HASH_LOCKS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r5worker {
    pub work: work_struct,
    pub group: *mut r5worker_group,
    pub temp_inactive_list: [list_head; NR_STRIPE_HASH_LOCKS],
    pub working: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r5worker_group {
    pub handle_list: list_head,
    pub loprio_list: list_head,
    pub conf: *mut r5conf,
    pub workers: *mut r5worker,
    pub stripes_cnt: c_int,
}

//
// r5c journal modes of the array: write-back or write-through.
// write-through mode has identical behavior as existing log only
// implementation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r5c_journal_mode {
    R5C_JOURNAL_MODE_WRITE_THROUGH = 0,
    R5C_JOURNAL_MODE_WRITE_BACK = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r5_cache_state {
    R5_INACTIVE_BLOCKED,	/* release of inactive stripes blocked,
// waiting for 25% to be free
//
    R5_ALLOC_MORE,		/* It might help to allocate another
// stripe.
//
    R5_DID_ALLOC,		/* A stripe was allocated, don't allocate
// more until at least one has been
// released.  This avoids flooding
// the cache.
//
    R5C_LOG_TIGHT,		/* log device space tight, need to
// prioritize stripes at last_checkpoint
//
    R5C_LOG_CRITICAL,	/* log device is running out of space,
// only process stripes that are already
// occupying the log
//
    R5C_EXTRA_PAGE_IN_USE,	/* a stripe is using disk_info.extra_page
// for prexor
//
}

pub const PENDING_IO_MAX: c_int = 512;
pub const PENDING_IO_ONE_FLUSH: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r5pending_data {
    pub sibling: list_head,
    pub /: *mut *mut sector_t sector; / stripe sector,
    pub bios: bio_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid5_percpu {
    pub /: *mut *mut *mut page spare_page; / Used when checking P/Q in raid6,
    pub buffer: *mut *mut *mut void scribble; / space for constructing,
// lists and performing address
// conversions
//
    pub scribble_obj_size: c_int,
    pub lock: local_lock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r5conf {
    pub stripe_hashtbl: *mut hlist_head,
// only protect corresponding hash list and inactive_list
    pub hash_locks: [spinlock_t; NR_STRIPE_HASH_LOCKS],
    pub mddev: *mut mddev,
    pub chunk_sectors: c_int,
    pub rmw_level: int level, algorithm,,
    pub max_degraded: c_int,
    pub raid_disks: c_int,
    pub max_nr_stripes: c_int,
    pub min_nr_stripes: c_int,

    pub stripe_size: c_ulong,
    pub stripe_shift: c_uint,
    pub stripe_sectors: c_ulong,

// reshape_progress is the leading edge of a 'reshape'
// It has value MaxSector when no reshape is happening
// If delta_disks < 0, it is the last sector we started work on,
// else is it the next sector to work on.
//
    pub reshape_progress: sector_t,
// reshape_safe is the trailing edge of a reshape.  We know that
// before (or after) this address, all reshape has completed.
//
    pub reshape_safe: sector_t,
    pub previous_raid_disks: c_int,
    pub prev_chunk_sectors: c_int,
    pub prev_algo: c_int,
    pub /: *mut *mut short generation; / increments with every reshape,
    pub /: *mut *mut seqcount_spinlock_t gen_lock; / lock against generation changes,
    pub updated: *mut *mut unsigned long reshape_checkpoint; / Time we last,
// metadata
    pub between: *mut *mut long long min_offset_diff; / minimum difference,
// data_offset and
// new_data_offset across all
// devices.  May be negative,
// but is closest to zero.
//
    pub /: *mut *mut list_head handle_list; / stripes needing handling,
    pub /: *mut *mut list_head loprio_list; / low priority stripes,
    pub /: *mut *mut list_head hold_list; / preread ready stripes,
    pub /: *mut *mut list_head delayed_list; / stripes that have plugged requests,
    pub /: *mut *mut list_head bitmap_list; / stripes delaying awaiting bitmap update,
    pub /: *mut *mut *mut bio retry_read_aligned; / currently retrying aligned bios,
    pub /: *mut *mut unsigned int retry_read_offset; / sector offset into retry_read_aligned,
    pub /: *mut *mut *mut bio retry_read_aligned_list; / aligned bios retry list,
    pub /: *mut *mut atomic_t preread_active_stripes; / stripes with scheduled io,
    pub active_aligned_reads: core::sync::atomic::AtomicI32,
    pub /: *mut *mut atomic_t pending_full_writes; / full write backlog,
    pub /: *mut *mut int bypass_count; / bypassed prereads,
    pub /: *mut *mut int bypass_threshold; / preread nice,
    pub /: *mut *mut int skip_copy; / Don't copy data from bio to stripe cache,
    pub /: *mut *mut *mut list_head last_hold; / detect hold_list promotions,
    pub /: *mut *mut atomic_t reshape_stripes; / stripes with pending writes for reshape,
// unfortunately we need two cache names as we temporarily have
// two caches.
//
    pub active_name: c_int,
    pub cache_name: [c_char; 2][48],
    pub /: *mut *mut *mut kmem_cache slab_cache; / for allocating stripes,
    pub /: *mut *mut mutex cache_size_mutex; / Protect changes to cache size,
    pub seq_write: int seq_flush,,
    pub quiesce: c_int,
    pub needed,: *mut *mut int fullsync; / set to 1 if a full sync is,
// (fresh device added).
// Cleared when a sync completes.
//
// per cpu variables
    pub percpu: *mut raid5_percpu __percpu,
    pub scribble_disks: c_int,
    pub scribble_sectors: c_int,
    pub node: hlist_node,
//
// Free stripes pool
//
    pub active_stripes: core::sync::atomic::AtomicI32,
    pub inactive_list: [list_head; NR_STRIPE_HASH_LOCKS],
    pub r5c_cached_full_stripes: core::sync::atomic::AtomicI32,
    pub r5c_full_stripe_list: list_head,
    pub r5c_cached_partial_stripes: core::sync::atomic::AtomicI32,
    pub r5c_partial_stripe_list: list_head,
    pub r5c_flushing_full_stripes: core::sync::atomic::AtomicI32,
    pub r5c_flushing_partial_stripes: core::sync::atomic::AtomicI32,
    pub empty_inactive_list_nr: core::sync::atomic::AtomicI32,
    pub released_stripes: llist_head,
    pub wait_for_quiescent: wait_queue_head_t,
    pub wait_for_stripe: wait_queue_head_t,
    pub wait_for_reshape: wait_queue_head_t,
    pub cache_state: c_ulong,
    pub shrinker: *mut shrinker,
    pub /: *mut *mut int pool_size; / number of disks in stripeheads in pool,
    pub device_lock: spinlock_t,
    pub disks: *mut disk_info,
    pub bio_split: bio_set,
// When taking over an array from a different personality, we store
// the new thread here until we fully activate the array.
//
    pub thread: *mut md_thread __rcu,
    pub temp_inactive_list: [list_head; NR_STRIPE_HASH_LOCKS],
    pub worker_groups: *mut r5worker_group,
    pub group_cnt: c_int,
    pub worker_cnt_per_group: c_int,
    pub log: *mut r5l_log,
    pub log_private: *mut c_void,
    pub pending_bios_lock: spinlock_t,
    pub batch_bio_dispatch: bool,
    pub pending_data: *mut r5pending_data,
    pub free_list: list_head,
    pub pending_list: list_head,
    pub pending_data_cnt: c_int,
    pub next_pending_data: *mut r5pending_data,
    pub raid5_discard_unsupported: bool,
    pub ctx_pool: *mut mempool_t,
    pub ctx_size: c_int,
}

// bio's attached to a stripe+device for I/O are linked together in bi_sector
// order without overlap.  There may be several bio's per stripe+device, and
// a bio could span several devices.
// When walking this list for a particular stripe+device, we must never proceed
// beyond a bio that extends past this device, as the next bio might no longer
// be valid.
// This function is used to determine the 'next' bio in the list, given the
// sector of the current stripe+device
//
// Our supported algorithms
//

// Define non-rotating (raid4) algorithms.  These allow
// conversion of raid4 to raid5.
//

// DDF RAID6 layouts differ from md/raid6 layouts in two ways.
// Firstly, the exact positioning of the parity block is slightly
// different between the 'LEFT_*' modes of md and the "_N_*" modes
// of DDF.
// Secondly, or order of datablocks over which the Q syndrome is computed
// is different.
// Consequently we have different layouts for DDF/raid6 than md/raid6.
// These layouts are from the DDFv1.2 spec.
// Interestingly DDFv1.2-Errata-A does not specify N_CONTINUE but
// leaves RLQ=3 as 'Vendor Specific'
//

// For every RAID5 algorithm we define a RAID6 algorithm
// with exactly the same layout for data and parity, and
// with the Q block always on the last device (N-1).
// This allows trivial conversion from RAID5 to RAID6
//
pub const ALGORITHM_LEFT_ASYMMETRIC_6: c_int = 16;
pub const ALGORITHM_RIGHT_ASYMMETRIC_6: c_int = 17;
pub const ALGORITHM_LEFT_SYMMETRIC_6: c_int = 18;
pub const ALGORITHM_RIGHT_SYMMETRIC_6: c_int = 19;
pub const ALGORITHM_PARITY_0_6: c_int = 20;

//
// Return offset of the corresponding page for r5dev.
//
// Return corresponding page address for r5dev.
//

extern "C" {
    pub fn raid5_set_cache_size(mddev: *mut mddev, size: c_int) -> c_int;
}
extern "C" {
    pub fn raid5_compute_blocknr(sh: *mut stripe_head, i: c_int, previous: c_int) -> sector_t;
}
extern "C" {
    pub fn raid5_release_stripe(sh: *mut stripe_head);
}
// get stripe from previous generation (when reshaping)

// do not block waiting for a free stripe

// do not block waiting for quiesce to be released

extern "C" {
    pub fn raid5_calc_degraded(conf: *mut r5conf) -> c_int;
}
extern "C" {
    pub fn r5c_journal_mode_set(mddev: *mut mddev, journal_mode: c_int) -> c_int;
}
