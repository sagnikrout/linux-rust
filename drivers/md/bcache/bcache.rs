//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/bcache.h
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
// SOME HIGH LEVEL CODE DOCUMENTATION:
//
// Bcache mostly works with cache sets, cache devices, and backing devices.
//
// Support for multiple cache devices hasn't quite been finished off yet, but
// it's about 95% plumbed through. A cache set and its cache devices is sort of
// like a md raid array and its component devices. Most of the code doesn't care
// about individual cache devices, the main abstraction is the cache set.
//
// Multiple cache devices is intended to give us the ability to mirror dirty
// cached data and metadata, without mirroring clean cached data.
//
// Backing devices are different, in that they have a lifetime independent of a
// cache set. When you register a newly formatted backing device it'll come up
// in passthrough mode, and then you can attach and detach a backing device from
// a cache set at runtime - while it's mounted and in use. Detaching implicitly
// invalidates any cached data for that backing device.
//
// A cache set can have multiple (many) backing devices attached to it.
//
// There's also flash only volumes - this is the reason for the distinction
// between struct cached_dev and struct bcache_device. A flash only volume
// works much like a bcache device that has a backing device, except the
// "cached" data is always dirty. The end result is that we get thin
// provisioning with very little additional code.
//
// Flash only volumes work but they're not production ready because the moving
// garbage collector needs more work. More on that later.
//
// BUCKETS/ALLOCATION:
//
// Bcache is primarily designed for caching, which means that in normal
// operation all of our available space will be allocated. Thus, we need an
// efficient way of deleting things from the cache so we can write new things to
// it.
//
// To do this, we first divide the cache device up into buckets. A bucket is the
// unit of allocation; they're typically around 1 mb - anywhere from 128k to 2M+
// works efficiently.
//
// Each bucket has a 16 bit priority, and an 8 bit generation associated with
// it. The gens and priorities for all the buckets are stored contiguously and
// packed on disk (in a linked list of buckets - aside from the superblock, all
// of bcache's metadata is stored in buckets).
//
// The priority is used to implement an LRU. We reset a bucket's priority when
// we allocate it or on cache it, and every so often we decrement the priority
// of each bucket. It could be used to implement something more sophisticated,
// if anyone ever gets around to it.
//
// The generation is used for invalidating buckets. Each pointer also has an 8
// bit generation embedded in it; for a pointer to be considered valid, its gen
// must match the gen of the bucket it points into.  Thus, to reuse a bucket all
// we have to do is increment its gen (and write its new gen to disk; we batch
// this up).
//
// Bcache is entirely COW - we never write twice to a bucket, even buckets that
// contain metadata (including btree nodes).
//
// THE BTREE:
//
// Bcache is in large part design around the btree.
//
// At a high level, the btree is just an index of key -> ptr tuples.
//
// Keys represent extents, and thus have a size field. Keys also have a variable
// number of pointers attached to them (potentially zero, which is handy for
// invalidating the cache).
//
// The key itself is an inode:offset pair. The inode number corresponds to a
// backing device or a flash only volume. The offset is the ending offset of the
// extent within the inode - not the starting offset; this makes lookups
// slightly more convenient.
//
// Pointers contain the cache device id, the offset on that device, and an 8 bit
// generation number. More on the gen later.
//
// Index lookups are not fully abstracted - cache lookups in particular are
// still somewhat mixed in with the btree code, but things are headed in that
// direction.
//
// Updates are fairly well abstracted, though. There are two different ways of
// updating the btree; insert and replace.
//
// BTREE_INSERT will just take a list of keys and insert them into the btree -
// overwriting (possibly only partially) any extents they overlap with. This is
// used to update the index after a write.
//
// BTREE_REPLACE is really cmpxchg(); it inserts a key into the btree iff it is
// overwriting a key that matches another given key. This is used for inserting
// data into the cache after a cache miss, and for background writeback, and for
// the moving garbage collector.
//
// There is no "delete" operation; deleting things from the index is
// accomplished by either by invalidating pointers (by incrementing a bucket's
// gen) or by inserting a key with 0 pointers - which will overwrite anything
// previously present at that location in the index.
//
// This means that there are always stale/invalid keys in the btree. They're
// filtered out by the code that iterates through a btree node, and removed when
// a btree node is rewritten.
//
// BTREE NODES:
//
// Our unit of allocation is a bucket, and we can't arbitrarily allocate and
// free smaller than a bucket - so, that's how big our btree nodes are.
//
// (If buckets are really big we'll only use part of the bucket for a btree node
// - no less than 1/4th - but a bucket still contains no more than a single
// btree node. I'd actually like to change this, but for now we rely on the
// bucket's gen for deleting btree nodes when we rewrite/split a node.)
//
// Anyways, btree nodes are big - big enough to be inefficient with a textbook
// btree implementation.
//
// The way this is solved is that btree nodes are internally log structured; we
// can append new keys to an existing btree node without rewriting it. This
// means each set of keys we write is sorted, but the node is not.
//
// We maintain this log structure in memory - keeping 1Mb of keys sorted would
// be expensive, and we have to distinguish between the keys we have written and
// the keys we haven't. So to do a lookup in a btree node, we have to search
// each sorted set. But we do merge written sets together lazily, so the cost of
// these extra searches is quite low (normally most of the keys in a btree node
// will be in one big set, and then there'll be one or two sets that are much
// smaller).
//
// This log structure makes bcache's btree more of a hybrid between a
// conventional btree and a compacting data structure, with some of the
// advantages of both.
//
// GARBAGE COLLECTION:
//
// We can't just invalidate any bucket - it might contain dirty data or
// metadata. If it once contained dirty data, other writes might overwrite it
// later, leaving no valid pointers into that bucket in the index.
//
// Thus, the primary purpose of garbage collection is to find buckets to reuse.
// It also counts how much valid data it each bucket currently contains, so that
// allocation can reuse buckets sooner when they've been mostly overwritten.
//
// It also does some things that are really internal to the btree
// implementation. If a btree node contains pointers that are stale by more than
// some threshold, it rewrites the btree node to avoid the bucket's generation
// wrapping around. It also merges adjacent btree nodes if they're empty enough.
//
// THE JOURNAL:
//
// Bcache's journal is not necessary for consistency; we always strictly
// order metadata writes so that the btree and everything else is consistent on
// disk in the event of an unclean shutdown, and in fact bcache had writeback
// caching (with recovery from unclean shutdown) before journalling was
// implemented.
//
// Rather, the journal is purely a performance optimization; we can't complete a
// write until we've updated the index on disk, otherwise the cache would be
// inconsistent in the event of an unclean shutdown. This means that without the
// journal, on random write workloads we constantly have to update all the leaf
// nodes in the btree, and those writes will be mostly empty (appending at most
// a few keys each) - highly inefficient in terms of amount of metadata writes,
// and it puts more strain on the various btree resorting/compacting code.
//
// The journal is just a log of keys we've inserted; on startup we just reinsert
// all the keys in the open journal entries. That means that when we're updating
// a node in the btree, we can wait until a 4k block of keys fills up before
// writing them out.
//
// For simplicity, we only journal updates to leaf nodes; updates to parent
// nodes are rare enough (since our leaf nodes are huge) that it wasn't worth
// the complexity to deal with journalling them (in particular, journal replay)
// - updates to non leaf nodes just happen synchronously (see btree_split()).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bucket {
    pub pin: core::sync::atomic::AtomicI32,
    pub prio: u16,
    pub gen: u8,
    pub /: *mut *mut uint8_t last_gc; / Most out of date gen in the btree,
    pub /: *mut *mut uint16_t gc_mark; / Bitfield used by GC. See below for field,
    pub reclaimable_in_gc:1: u16,
}

//
// I'd use bitfields for these, but I don't trust the compiler not to screw me
// as multiple threads touch struct bucket without locking
//
pub const GC_MARK_RECLAIMABLE: c_int = 1;
pub const GC_MARK_DIRTY: c_int = 2;
pub const GC_MARK_METADATA: c_int = 3;
pub const GC_SECTORS_USED_SIZE: c_int = 13;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keybuf_key {
    pub node: rb_node,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keybuf {
    pub last_scanned: bkey,
    pub lock: spinlock_t,
//
// Beginning and end of range in rb tree - so that we can skip taking
// lock and checking the rb tree when we need to check for overlapping
// keys.
//
    pub start: bkey,
    pub end: bkey,
    pub keys: rb_root,
pub const KEYBUF_NR: c_int = 500;
    pub KEYBUF_NR): DECLARE_ARRAY_ALLOCATOR(struct keybuf_key, freelist,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcache_device {
    pub cl: closure,
    pub kobj: kobject,
    pub c: *mut cache_set,
    pub id: c_uint,
pub const BCACHEDEVNAME_SIZE: c_int = 12;
    pub name: [c_char; BCACHEDEVNAME_SIZE],
    pub disk: *mut gendisk,
    pub flags: c_ulong,
pub const BCACHE_DEV_CLOSING: c_int = 0;
pub const BCACHE_DEV_DETACHING: c_int = 1;
pub const BCACHE_DEV_UNLINK_DONE: c_int = 2;
pub const BCACHE_DEV_WB_RUNNING: c_int = 3;
pub const BCACHE_DEV_RATE_DW_RUNNING: c_int = 4;
    pub nr_stripes: c_int,

    pub stripe_size: c_uint,
    pub stripe_sectors_dirty: *mut core::sync::atomic::AtomicI32,
    pub full_dirty_stripes: *mut c_ulong,
    pub bio_split: bio_set,
    pub bio_detached: bio_set,
    pub data_csum:1: c_uint,
    pub sectors): *mut *mut bio bio, unsigned int,
    pub arg): unsigned int cmd, unsigned long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io {
// Used to track sequential IO so it can be skipped
    pub hash: hlist_node,
    pub lru: list_head,
    pub jiffies: c_ulong,
    pub sequential: c_uint,
    pub last: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stop_on_failure {
    BCH_CACHED_DEV_STOP_AUTO = 0,
    BCH_CACHED_DEV_STOP_ALWAYS,
    BCH_CACHED_DEV_STOP_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cached_dev {
    pub list: list_head,
    pub disk: bcache_device,
    pub bdev: *mut block_device,
    pub bdev_file: *mut file,
    pub sb: cache_sb,
    pub sb_disk: *mut cache_sb_disk,
    pub sb_bio: bio,
    pub sb_bv: [bio_vec; 1],
    pub sb_write: closure,
    pub sb_write_mutex: semaphore,
// Refcount on the cache set. Always nonzero when we're caching.
    pub count: refcount_t,
    pub detach: work_struct,
//
// Device might not be running if it's dirty and the cache set hasn't
// showed up yet.
//
    pub running: core::sync::atomic::AtomicI32,
//
// Writes take a shared lock from start to finish; scanning for dirty
// data to refill the rb tree requires an exclusive lock.
//
    pub writeback_lock: rw_semaphore,
//
// Nonzero, and writeback has a refcount (d->count), iff there is dirty
// data in the cache. Protected by writeback_lock; must have an
// shared lock to set and exclusive lock to clear.
//
    pub has_dirty: core::sync::atomic::AtomicI32,
pub const BCH_CACHE_READA_ALL: c_int = 0;
pub const BCH_CACHE_READA_META_ONLY: c_int = 1;
    pub cache_readahead_policy: c_uint,
    pub writeback_rate: bch_ratelimit,
    pub writeback_rate_update: delayed_work,
// Limit number of writeback bios in flight
    pub in_flight: semaphore,
    pub writeback_thread: *mut task_struct,
    pub writeback_write_wq: *mut workqueue_struct,
    pub writeback_keys: keybuf,
    pub status_update_thread: *mut task_struct,
//
// Order the write-half of writeback operations strongly in dispatch
// order.  (Maintain LBA order; don't allow reads completing out of
// order to re-order the writes...)
//
    pub writeback_ordering_wait: closure_waitlist,
    pub writeback_sequence_next: core::sync::atomic::AtomicI32,
// For tracking sequential IO
pub const RECENT_IO_BITS: c_int = 7;
    pub io: [io; RECENT_IO],
    pub 1]: hlist_head io_hash[RECENT_IO +,
    pub io_lru: list_head,
    pub io_lock: spinlock_t,
    pub accounting: cache_accounting,
// The rest of this all shows up in sysfs
    pub sequential_cutoff: c_uint,
    pub io_disable:1: c_uint,
    pub verify:1: c_uint,
    pub bypass_torture_test:1: c_uint,
    pub partial_stripes_expensive:1: c_uint,
    pub writeback_metadata:1: c_uint,
    pub writeback_running:1: c_uint,
    pub writeback_consider_fragment:1: c_uint,
    pub writeback_percent: c_uchar,
    pub writeback_delay: c_uint,
    pub writeback_rate_target: u64,
    pub writeback_rate_proportional: i64,
    pub writeback_rate_integral: i64,
    pub writeback_rate_integral_scaled: i64,
    pub writeback_rate_change: i32,
    pub writeback_rate_update_seconds: c_uint,
    pub writeback_rate_i_term_inverse: c_uint,
    pub writeback_rate_p_term_inverse: c_uint,
    pub writeback_rate_fp_term_low: c_uint,
    pub writeback_rate_fp_term_mid: c_uint,
    pub writeback_rate_fp_term_high: c_uint,
    pub writeback_rate_minimum: c_uint,
    pub stop_when_cache_set_failed: stop_on_failure,
pub const DEFAULT_CACHED_DEV_ERROR_LIMIT: c_int = 64;
    pub io_errors: core::sync::atomic::AtomicI32,
    pub error_limit: c_uint,
    pub offline_seconds: c_uint,
//
// Retry to update writeback_rate if contention happens for
// down_read(dc->writeback_lock) in update_writeback_rate()
//
pub const BCH_WBRATE_UPDATE_MAX_SKIPS: c_int = 15;
    pub rate_update_retry: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alloc_reserve {
    RESERVE_BTREE,
    RESERVE_PRIO,
    RESERVE_MOVINGGC,
    RESERVE_NONE,
    RESERVE_NR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache {
    pub set: *mut cache_set,
    pub sb: cache_sb,
    pub sb_disk: *mut cache_sb_disk,
    pub sb_bio: bio,
    pub sb_bv: [bio_vec; 1],
    pub kobj: kobject,
    pub bdev: *mut block_device,
    pub bdev_file: *mut file,
    pub alloc_thread: *mut task_struct,
    pub prio: closure,
    pub disk_buckets: *mut prio_set,
//
// When allocating new buckets, prio_write() gets first dibs - since we
// may not be allocate at all without writing priorities and gens.
// prio_last_buckets[] contains the last buckets we wrote priorities to
// (so gc can mark them as metadata), prio_buckets[] contains the
// buckets allocated for the next prio write.
//
    pub prio_buckets: *mut u64,
    pub prio_last_buckets: *mut u64,
//
// free: Buckets that are ready to be used
//
// free_inc: Incoming buckets - these are buckets that currently have
// cached data in them, and we can't reuse them until after we write
// their new gen to disk. After prio_write() finishes writing the new
// gens/prios, they'll be moved to the free list.
//
    pub free)[RESERVE_NR]: DECLARE_FIFO(long,,
    pub free_inc): DECLARE_FIFO(long,,
    pub fifo_last_bucket: usize,
// Allocation stuff:
    pub buckets: *mut bucket,
    pub heap): *mut *mut DECLARE_HEAP(struct bucket ,,
//
// If nonzero, we know we aren't going to find any buckets to invalidate
// until a gc finishes - otherwise we could pointlessly burn a ton of
// cpu
//
    pub invalidate_needs_gc: c_uint,
    pub journal: journal_device,
// The rest of this all shows up in sysfs
pub const IO_ERROR_SHIFT: c_int = 20;
    pub io_errors: core::sync::atomic::AtomicI32,
    pub io_count: core::sync::atomic::AtomicI32,
    pub meta_sectors_written: atomic_long_t,
    pub btree_sectors_written: atomic_long_t,
    pub sectors_written: atomic_long_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gc_stat {
    pub nodes: usize,
    pub nodes_pre: usize,
    pub key_bytes: usize,
    pub nkeys: usize,
    pub /: *mut *mut uint64_t data; / sectors,
    pub /: *mut *mut unsigned int in_use; / percent,
}

//
// Flag bits, for how the cache set is shutting down, and what phase it's at:
//
// CACHE_SET_UNREGISTERING means we're not just shutting down, we're detaching
// all the backing devices first (their cached data gets invalidated, and they
// won't automatically reattach).
//
// CACHE_SET_STOPPING always gets set first when we're closing down a cache set;
// we'll continue to run normally for awhile with CACHE_SET_STOPPING set (i.e.
// flushing dirty data).
//
// CACHE_SET_RUNNING means all cache devices have been registered and journal
// replay is complete.
//
// CACHE_SET_IO_DISABLE is set when bcache is stopping the whold cache set, all
// external and internal I/O should be denied when this flag is set.
//
pub const CACHE_SET_UNREGISTERING: c_int = 0;
pub const CACHE_SET_STOPPING: c_int = 1;
pub const CACHE_SET_RUNNING: c_int = 2;
pub const CACHE_SET_IO_DISABLE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_set {
    pub cl: closure,
    pub list: list_head,
    pub kobj: kobject,
    pub internal: kobject,
    pub debug: *mut dentry,
    pub accounting: cache_accounting,
    pub flags: c_ulong,
    pub idle_counter: core::sync::atomic::AtomicI32,
    pub at_max_writeback_rate: core::sync::atomic::AtomicI32,
    pub cache: *mut cache,
    pub devices: *mut bcache_device,
    pub devices_max_used: c_uint,
    pub attached_dev_nr: core::sync::atomic::AtomicI32,
    pub cached_devs: list_head,
    pub cached_dev_sectors: u64,
    pub flash_dev_dirty_sectors: atomic_long_t,
    pub caching: closure,
    pub sb_write: closure,
    pub sb_write_mutex: semaphore,
    pub search: mempool_t,
    pub bio_meta: mempool_t,
    pub bio_split: bio_set,
// For the btree cache
    pub shrink: *mut shrinker,
// For the btree cache and anything allocation related
    pub bucket_lock: mutex,
// log2(bucket_size), in sectors
    pub bucket_bits: c_ushort,
// log2(block_size), in sectors
    pub block_bits: c_ushort,
//
// Default number of pages for a new btree node - may be less than a
// full bucket
//
    pub btree_pages: c_uint,
//
// Lists of struct btrees; lru is the list for structs that have memory
// allocated for actual btree node, freed is for structs that do not.
//
// We never free a struct btree, except on shutdown - we just put it on
// the btree_cache_freed list and reuse it later. This simplifies the
// code, and it doesn't cost us much memory as the memory usage is
// dominated by buffers that hold the actual btree node data and those
// can be freed - and the number of struct btrees allocated is
// effectively bounded.
//
// btree_cache_freeable effectively is a small cache - we use it because
// high order page allocations can be rather expensive, and it's quite
// common to delete and allocate btree nodes in quick succession. It
// should never grow past ~2-3 nodes in practice.
//
    pub btree_cache: list_head,
    pub btree_cache_freeable: list_head,
    pub btree_cache_freed: list_head,
// Number of elements in btree_cache + btree_cache_freeable lists
    pub btree_cache_used: c_uint,
//
// If we need to allocate memory for a new btree node and that
// allocation fails, we can cannibalize another node in the btree cache
// to satisfy the allocation - lock to guarantee only one thread does
// this at a time:
//
    pub btree_cache_wait: wait_queue_head_t,
    pub btree_cache_alloc_lock: *mut task_struct,
    pub btree_cannibalize_lock: spinlock_t,
//
// When we free a btree node, we increment the gen of the bucket the
// node is in - but we can't rewrite the prios and gens until we
// finished whatever it is we were doing, otherwise after a crash the
// btree node would be freed but for say a split, we might not have the
// pointers to the new nodes inserted into the btree yet.
//
// This is a refcount that blocks prio_write() until the new keys are
// written.
//
    pub prio_blocked: core::sync::atomic::AtomicI32,
    pub bucket_wait: wait_queue_head_t,
    pub bucket_wait_cnt: core::sync::atomic::AtomicI32,
//
// For any bio we don't skip we subtract the number of sectors from
// rescale; when it hits 0 we rescale all the bucket priorities.
//
    pub rescale: core::sync::atomic::AtomicI32,
//
// used for GC, identify if any front side I/Os is inflight
//
    pub search_inflight: core::sync::atomic::AtomicI32,
//
// When we invalidate buckets, we use both the priority and the amount
// of good data to determine which buckets to reuse first - to weight
// those together consistently we keep track of the smallest nonzero
// priority of any bucket.
//
    pub min_prio: u16,
//
// max(gen - last_gc) for all buckets. When it gets too big we have to
// gc to keep gens from wrapping around.
//
    pub need_gc: u8,
    pub gc_stats: gc_stat,
    pub nbuckets: usize,
    pub avail_nbuckets: usize,
    pub gc_thread: *mut task_struct,
// Where in the btree gc currently is
    pub gc_done: bkey,
//
// For automatical garbage collection after writeback completed, this
// varialbe is used as bit fields,
// - 0000 0001b (BCH_ENABLE_AUTO_GC): enable gc after writeback
// - 0000 0010b (BCH_DO_AUTO_GC):     do gc after writeback
// This is an optimization for following write request after writeback
// finished, but read hit rate dropped due to clean data on cache is
// discarded. Unless user explicitly sets it via sysfs, it won't be
// enabled.
//
pub const BCH_ENABLE_AUTO_GC: c_int = 1;
pub const BCH_DO_AUTO_GC: c_int = 2;
    pub gc_after_writeback: u8,
//
// The allocation code needs gc_mark in struct bucket to be correct, but
// it's not while a gc is in progress. Protected by bucket_lock.
//
    pub gc_mark_valid: c_int,
// Counts how many sectors bio_insert has added to the cache
    pub sectors_to_gc: core::sync::atomic::AtomicI32,
    pub gc_wait: wait_queue_head_t,
    pub moving_gc_keys: keybuf,
// Number of moving GC bios in flight
    pub moving_in_flight: semaphore,
    pub moving_gc_wq: *mut workqueue_struct,
    pub root: *mut btree,

    pub verify_data: *mut btree,
    pub verify_ondisk: *mut bset,
    pub verify_lock: mutex,
    pub set_uuid: [u8; 16],
    pub nr_uuids: c_uint,
    pub uuids: *mut uuid_entry,
    pub uuid_write: closure,
    pub uuid_write_mutex: semaphore,
//
// A btree node on disk could have too many bsets for an iterator to fit
// on the stack - have to dynamically allocate them.
// bch_cache_set_alloc() will make sure the pool can allocate iterators
// equipped with enough room that can host
// (sb.bucket_size / sb.block_size)
// btree_iter_sets, which is more than static MAX_BSETS.
//
    pub fill_iter: mempool_t,
    pub sort: bset_sort_state,
// List of buckets we're currently writing data to
    pub data_buckets: list_head,
    pub data_bucket_lock: spinlock_t,
    pub journal: journal,
pub const CONGESTED_MAX: c_int = 1024;
    pub congested_last_us: c_uint,
    pub congested: core::sync::atomic::AtomicI32,
// The rest of this all shows up in sysfs
    pub congested_read_threshold_us: c_uint,
    pub congested_write_threshold_us: c_uint,
    pub btree_gc_time: time_stats,
    pub btree_split_time: time_stats,
    pub btree_read_time: time_stats,
    pub cache_read_races: atomic_long_t,
    pub writeback_keys_done: atomic_long_t,
    pub writeback_keys_failed: atomic_long_t,
    pub reclaim: atomic_long_t,
    pub reclaimed_journal_buckets: atomic_long_t,
    pub flush_write: atomic_long_t,
    pub on_error: },
pub const DEFAULT_IO_ERROR_LIMIT: c_int = 8;
    pub error_limit: c_uint,
    pub error_decay: c_uint,
    pub journal_delay_ms: c_ushort,
    pub expensive_debug_checks: bool,
    pub verify:1: c_uint,
    pub key_merging_disabled:1: c_uint,
    pub gc_always_rewrite:1: c_uint,
    pub shrinker_disabled:1: c_uint,
    pub copy_gc_enabled:1: c_uint,
    pub idle_max_writeback_rate_enabled:1: c_uint,
pub const BUCKET_HASH_BITS: c_int = 12;
    pub BUCKET_HASH_BITS]: hlist_head bucket_hash[1 <<,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bbio {
    pub submit_time_us: c_uint,
    pub key: bkey,
    pub _pad: [u64; 3],
//
// We only need pad = 3 here because we only ever carry around a
// single pointer - i.e. the pointer we're doing io to/from.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct detached_dev_io_private {
    pub d: *mut bcache_device,
    pub start_time: c_ulong,
    pub orig_bio: *mut bio,
    pub bio: bio,
}

extern "C" {
    pub fn sector_to_bucket(_arg: c, _arg: PTR_OFFSET(k, _arg: ptr)) -> return;
}
extern "C" {
    pub fn gen_after(_arg: PTR_BUCKET(c, _arg: k, _arg: i)->gen, _arg: PTR_GEN(k, _arg: i)) -> return;
}
// Btree key macros
//
// This is used for various on disk data structures - cache_sb, prio_set, bset,
// jset: The checksum is _always_ the first 8 bytes of these structs
//

// Error handling macros

// Looping macros

// Paired with the mb in cached_dev_attach
//
// bucket_gc_gen() returns the difference between the bucket's current gen and
// the oldest gen of any pointer into that bucket in the btree (last_gc).
//

//
// Prevent the kthread exits directly, and make sure when kthread_stop()
// is called to stop a kthread, it is still alive. If a kthread might be
// stopped by CACHE_SET_IO_DISABLE bit set, wait_for_kthread_stop() is
// necessary before the kthread returns.
//
// Forward declarations
extern "C" {
    pub fn bch_count_backing_io_errors(dc: *mut cached_dev, bio: *mut bio);
}
extern "C" {
    pub fn bch_bbio_free(bio: *mut bio, c: *mut cache_set);
}
extern "C" {
    pub fn __bch_submit_bbio(bio: *mut bio, c: *mut cache_set);
}
extern "C" {
    pub fn bch_inc_gen(ca: *mut cache, b: *mut bucket) -> u8;
}
extern "C" {
    pub fn bch_rescale_priorities(c: *mut cache_set, sectors: c_int);
}
extern "C" {
    pub fn bch_can_invalidate_bucket(ca: *mut cache, b: *mut bucket) -> bool;
}
extern "C" {
    pub fn __bch_invalidate_one_bucket(ca: *mut cache, b: *mut bucket);
}
extern "C" {
    pub fn __bch_bucket_free(ca: *mut cache, b: *mut bucket);
}
extern "C" {
    pub fn bch_bucket_free(c: *mut cache_set, k: *mut bkey);
}
extern "C" {
    pub fn bch_bucket_alloc(ca: *mut cache, reserve: c_uint, wait: bool) -> c_long;
}
extern "C" {
    pub fn bch_cached_dev_error(dc: *mut cached_dev) -> bool;
}
extern "C" {
    pub fn bch_cache_set_error(c: *mut cache_set, fmt: *const c_char, ...) -> bool;
}
extern "C" {
    pub fn bch_prio_write(ca: *mut cache, wait: bool) -> c_int;
}
extern "C" {
    pub fn bch_write_bdev_super(dc: *mut cached_dev, parent: *mut closure);
}
extern "C" {
    pub fn bch_cached_dev_release(kobj: *mut kobject);
}
extern "C" {
    pub fn bch_flash_dev_release(kobj: *mut kobject);
}
extern "C" {
    pub fn bch_cache_set_release(kobj: *mut kobject);
}
extern "C" {
    pub fn bch_cache_release(kobj: *mut kobject);
}
extern "C" {
    pub fn bch_uuid_write(c: *mut cache_set) -> c_int;
}
extern "C" {
    pub fn bcache_write_super(c: *mut cache_set);
}
extern "C" {
    pub fn bch_flash_dev_create(c: *mut cache_set, size: u64) -> c_int;
}
extern "C" {
    pub fn bch_cached_dev_detach(dc: *mut cached_dev);
}
extern "C" {
    pub fn bch_cached_dev_run(dc: *mut cached_dev) -> c_int;
}
extern "C" {
    pub fn bcache_device_stop(d: *mut bcache_device);
}
extern "C" {
    pub fn bch_cache_set_unregister(c: *mut cache_set);
}
extern "C" {
    pub fn bch_cache_set_stop(c: *mut cache_set);
}
extern "C" {
    pub fn bch_btree_cache_free(c: *mut cache_set);
}
extern "C" {
    pub fn bch_btree_cache_alloc(c: *mut cache_set) -> c_int;
}
extern "C" {
    pub fn bch_moving_init_cache_set(c: *mut cache_set);
}
extern "C" {
    pub fn bch_open_buckets_alloc(c: *mut cache_set) -> c_int;
}
extern "C" {
    pub fn bch_open_buckets_free(c: *mut cache_set);
}
extern "C" {
    pub fn bch_cache_allocator_start(ca: *mut cache) -> c_int;
}
extern "C" {
    pub fn bch_debug_exit();
}
extern "C" {
    pub fn bch_debug_init();
}
extern "C" {
    pub fn bch_request_exit();
}
extern "C" {
    pub fn bch_request_init() -> c_int;
}
extern "C" {
    pub fn bch_btree_exit();
}
extern "C" {
    pub fn bch_btree_init() -> c_int;
}
