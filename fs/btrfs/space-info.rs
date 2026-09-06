//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/space-info.h
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
// Different levels for to flush space when doing space reservations.
//
// The higher the level, the more methods we try to reclaim space.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_reserve_flush_enum {
//
// Used when we can't flush or don't need:
//
// 1) We are holding a transaction handle open, so we can't flush as
// that could deadlock.
//
// 2) For a nowait write we don't want to block when reserving delalloc.
//
// 3) Joining a transaction or attaching a transaction, we don't want
// to wait and we don't need to reserve anything (any needed space
// was reserved before in a dedicated block reserve, or we rely on
// the global block reserve, see btrfs_init_root_block_rsv()).
//
// 4) Starting a transaction when we don't need to reserve space, as
// we don't need it because we previously reserved in a dedicated
// block reserve or rely on the global block reserve, like the above
// case.
//
    BTRFS_RESERVE_NO_FLUSH,

//
// Flush space by:
// - Running delayed inode items
// - Allocating a new chunk
//
    BTRFS_RESERVE_FLUSH_LIMIT,

//
// Flush space by:
// - Running delayed inode items
// - Running delayed refs
// - Running delalloc and waiting for ordered extents
// - Allocating a new chunk
// - Committing transaction
//
    BTRFS_RESERVE_FLUSH_EVICT,

//
// Flush space by above mentioned methods and by:
// - Running delayed iputs
// - Committing transaction
//
// Can be interrupted by a fatal signal.
//
    BTRFS_RESERVE_FLUSH_DATA,
    BTRFS_RESERVE_FLUSH_FREE_SPACE_INODE,
    BTRFS_RESERVE_FLUSH_ALL,

//
// Pretty much the same as FLUSH_ALL, but can also steal space from
// global rsv.
//
// Can be interrupted by a fatal signal.
//
    BTRFS_RESERVE_FLUSH_ALL_STEAL,

//
// This is for relocation on zoned filesystems only. We need to use
// priority flushing for this, because otherwise we can deadlock on
// waiting for a ticket, that cannot be granted, because we cannot do
// any allocations.
//
// Apart from being specific to zoned relocation, it is equal to
// BTRFS_FLUSH_FREE_SPACE_INODE.
//
    BTRFS_RESERVE_FLUSH_ZONED_RELOCATION,

//
// This is for btrfs_use_block_rsv only.  We have exhausted our block
// rsv and our global block rsv.  This can happen for things like
// delalloc where we are overwriting a lot of extents with a single
// extent and didn't reserve enough space.  Alternatively it can happen
// with delalloc where we reserve 1 extents worth for a large extent but
// fragmentation leads to multiple extents being created.  This will
// give us the reservation in the case of
//
// if (num_bytes < (space_info->total_bytes -
// btrfs_space_info_used(space_info, false))
//
// Which ignores bytes_may_use.  This is potentially dangerous, but our
// reservation system is generally pessimistic so is able to absorb this
// style of mistake.
//
    BTRFS_RESERVE_FLUSH_EMERGENCY,
}

//
// Please be aware that the order of enum values will be the order of the reclaim
// process in btrfs_async_reclaim_metadata_space().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_flush_state {
    FLUSH_DELAYED_ITEMS_NR	= 1,
    FLUSH_DELAYED_ITEMS	= 2,
    FLUSH_DELAYED_REFS_NR	= 3,
    FLUSH_DELAYED_REFS	= 4,
    FLUSH_DELALLOC		= 5,
    FLUSH_DELALLOC_WAIT	= 6,
    FLUSH_DELALLOC_FULL	= 7,
    ALLOC_CHUNK		= 8,
    ALLOC_CHUNK_FORCE	= 9,
    RUN_DELAYED_IPUTS	= 10,
    COMMIT_TRANS		= 11,
    RESET_ZONES		= 12,
    RECLAIM_ZONES		= 13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_space_info_sub_group {
    BTRFS_SUB_GROUP_PRIMARY,
    BTRFS_SUB_GROUP_DATA_RELOC,
    BTRFS_SUB_GROUP_TREELOG,
}

pub const BTRFS_SPACE_INFO_SUB_GROUP_MAX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_space_info {
    pub fs_info: *mut btrfs_fs_info,
    pub parent: *mut btrfs_space_info,
    pub sub_group: [*mut btrfs_space_info; BTRFS_SPACE_INFO_SUB_GROUP_MAX],
    pub subgroup_id: c_int,
    pub lock: spinlock_t,
    pub space,: *mut *mut u64 total_bytes; / total bytes in the,
    pub used,: *mut *mut u64 bytes_used; / total bytes,
    pub the: *mut *mut u64 bytes_pinned; / total bytes pinned, will be freed when,
    pub for: *mut *mut u64 bytes_reserved; / total bytes the allocator has reserved,
    pub for: *mut *mut u64 bytes_may_use; / number of bytes that may be used,
    pub /: *mut *mut u64 bytes_readonly; / total bytes that are read only,
    pub until: *mut *mut u64 bytes_zone_unusable; / total bytes that are unusable,
    pub of: *mut *mut u64 max_extent_size; / This will hold the maximum extent size,
// Chunk size in bytes
    pub chunk_size: u64,
//
// Once a block group drops below this threshold (percents) we'll
// schedule it for reclaim.
//
    pub bg_reclaim_threshold: c_int,
    pub preemptive: *mut *mut int clamp; / Used to scale our threshold for,
    pub more: *mut *mut bool full; / indicates that we cannot allocate any,
    pub /: *mut *mut bool chunk_alloc; / set if we are allocating a chunk,
    pub /: *mut *mut bool flush; / set if we are trying to make space,
    pub chunk: *mut *mut unsigned int force_alloc; / set if we need to force a,
    pub /: *mut *mut u64 disk_used; / total bytes used on disk,
    pub into: *mut *mut u64 disk_total; / total bytes on disk, takes mirrors,
    pub flags: u64,
    pub list: list_head,
// Protected by the spinlock 'lock'.
    pub ro_bgs: list_head,
    pub priority_tickets: list_head,
    pub tickets: list_head,
//
// Size of space that needs to be reclaimed in order to satisfy pending
// tickets
//
    pub reclaim_size: u64,
//
// tickets_id just indicates the next ticket will be handled, so note
// it's not stored per ticket.
//
    pub tickets_id: u64,
    pub groups_sem: rw_semaphore,
// for block groups in our same type
    pub block_groups: [list_head; BTRFS_NR_RAID_TYPES],
    pub kobj: kobject,
    pub block_group_kobjs: [*mut kobject; BTRFS_NR_RAID_TYPES],
//
// Monotonically increasing counter of block group reclaim attempts
// Exposed in /sys/fs/<uuid>/allocation/<type>/reclaim_count
//
    pub reclaim_count: u64,
//
// Monotonically increasing counter of reclaimed bytes
// Exposed in /sys/fs/<uuid>/allocation/<type>/reclaim_bytes
//
    pub reclaim_bytes: u64,
//
// Monotonically increasing counter of reclaim errors
// Exposed in /sys/fs/<uuid>/allocation/<type>/reclaim_errors
//
    pub reclaim_errors: u64,
//
// If true, use the dynamic relocation threshold, instead of the
// fixed bg_reclaim_threshold.
//
    pub dynamic_reclaim: bool,
//
// Periodically check all block groups against the reclaim
// threshold in the cleaner thread.
//
    pub periodic_reclaim: bool,
//
// Periodic reclaim should be a no-op if a space_info hasn't
// freed any space since the last time we tried.
//
    pub periodic_reclaim_ready: bool,
//
// Net bytes freed or allocated since the last reclaim pass.
//
    pub reclaimable_bytes: i64,
}

//
// Declare a helper function to detect underflow of various space info members
//

extern "C" {
    pub fn btrfs_init_space_info(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_clear_space_info_full(info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_try_granting_tickets(space_info: *mut btrfs_space_info);
}
extern "C" {
    pub fn btrfs_dump_space_info_for_trans_abort(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_init_async_reclaim_work(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_account_ro_block_groups_free_space(sinfo: *mut btrfs_space_info) -> u64;
}
extern "C" {
    pub fn btrfs_space_info_update_reclaimable(space_info: *mut btrfs_space_info, bytes: i64);
}
extern "C" {
    pub fn btrfs_set_periodic_reclaim_ready(space_info: *mut btrfs_space_info, ready: bool);
}
extern "C" {
    pub fn btrfs_calc_reclaim_threshold(space_info: *const btrfs_space_info) -> c_int;
}
extern "C" {
    pub fn btrfs_reclaim_sweep(fs_info: *const btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_return_free_space(space_info: *mut btrfs_space_info, len: u64);
}
