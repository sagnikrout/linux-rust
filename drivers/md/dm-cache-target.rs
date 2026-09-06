//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-cache-target.c
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

    DECLARE_DM_KCOPYD_THROTTLE_WITH_MODULE_PARM(cache_copy_throttle,
    "A percentage of time allocated for copying to and/or from cache");
// ----------------------------------------------------------------
//
// Glossary:
//
// oblock: index of an origin block
// cblock: index of a cache block
// promotion: movement of a block from origin to cache
// demotion: movement of a block from cache to origin
// migration: movement of a block between the origin and cache device,
// either direction
//
// ----------------------------------------------------------------
//
// Represents a chunk of future work.  'input' allows continuations to pass
// values between themselves, typically error values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct continuation {
    pub ws: work_struct,
    pub input: blk_status_t,
}

    static inline void init_continuation(struct continuation *k,
    void (*fn)(struct work_struct *))
    {
    INIT_WORK(&k.ws, fn);
    k.input = 0;
    }
    static inline void queue_continuation(struct workqueue_struct *wq,
    struct continuation *k)
    {
    queue_work(wq, &k.ws);
    }
// ----------------------------------------------------------------
//
// The batcher collects together pieces of work that need a particular
// operation to occur before they can proceed (typically a commit).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct batcher {
//
// The operation that everyone is waiting for.
//
    pub context): *mut *mut blk_status_t (commit_op)(void,
    pub commit_context: *mut c_void,
//
// This is how bios should be issued once the commit op is complete
// (accounted_request).
//
    pub context): *mut *mut *mut void (issue_op)(struct bio bio, void,
    pub issue_context: *mut c_void,
//
// Queued work gets put on here after commit.
//
    pub wq: *mut workqueue_struct,
    pub lock: spinlock_t,
    pub work_items: list_head,
    pub bios: bio_list,
    pub commit_work: work_struct,
    pub commit_scheduled: bool,
}

#[no_mangle]
unsafe extern "C" fn __commit(_ws: *mut work_struct) {
    static void __commit(struct work_struct *_ws)
    {
    struct batcher *b = container_of(_ws, struct batcher, commit_work);
    blk_status_t r;
    struct list_head work_items;
    struct work_struct *ws, *tmp;
    struct continuation *k;
    struct bio *bio;
    struct bio_list bios;
    INIT_LIST_HEAD(&work_items);
    bio_list_init(&bios);
//
// We have to grab these before the commit_op to avoid a race
// condition.
//
    spin_lock_irq(&b.lock);
    list_splice_init(&b.work_items, &work_items);
    bio_list_merge_init(&bios, &b.bios);
    b.commit_scheduled = false;
    spin_unlock_irq(&b.lock);
    r = b.commit_op(b.commit_context);
    list_for_each_entry_safe(ws, tmp, &work_items, entry) {
    k = container_of(ws, struct continuation, ws);
    k.input = r;
    INIT_LIST_HEAD(&ws.entry); /* to avoid a WARN_ON */
    queue_work(b.wq, ws);
    }
    while ((bio = bio_list_pop(&bios))) {
    if (r) {
    bio.bi_status = r;
    bio_endio(bio);
    } else
    b.issue_op(bio, b.issue_context);
    }
    }
    static void batcher_init(struct batcher *b,
    blk_status_t (*commit_op)(void *),
    void *commit_context,
    void (*issue_op)(struct bio *bio, void *),
    void *issue_context,
    struct workqueue_struct *wq)
    {
    b.commit_op = commit_op;
    b.commit_context = commit_context;
    b.issue_op = issue_op;
    b.issue_context = issue_context;
    b.wq = wq;
    spin_lock_init(&b.lock);
    INIT_LIST_HEAD(&b.work_items);
    bio_list_init(&b.bios);
    INIT_WORK(&b.commit_work, __commit);
    b.commit_scheduled = false;
    }
#[no_mangle]
unsafe extern "C" fn async_commit(b: *mut batcher) {
    static void async_commit(struct batcher *b)
    {
    queue_work(b.wq, &b.commit_work);
    }
#[no_mangle]
unsafe extern "C" fn continue_after_commit(b: *mut batcher, k: *mut continuation) {
    static void continue_after_commit(struct batcher *b, struct continuation *k)
    {
    bool commit_scheduled;
    spin_lock_irq(&b.lock);
    commit_scheduled = b.commit_scheduled;
    list_add_tail(&k.ws.entry, &b.work_items);
    spin_unlock_irq(&b.lock);
    if (commit_scheduled)
    async_commit(b);
    }
//
// Bios are errored if commit failed.
//
#[no_mangle]
unsafe extern "C" fn issue_after_commit(b: *mut batcher, bio: *mut bio) {
    static void issue_after_commit(struct batcher *b, struct bio *bio)
    {
    bool commit_scheduled;
    spin_lock_irq(&b.lock);
    commit_scheduled = b.commit_scheduled;
    bio_list_add(&b.bios, bio);
    spin_unlock_irq(&b.lock);
    if (commit_scheduled)
    async_commit(b);
    }
//
// Call this if some urgent work is waiting for the commit to complete.
//
#[no_mangle]
unsafe extern "C" fn schedule_commit(b: *mut batcher) {
    static void schedule_commit(struct batcher *b)
    {
    bool immediate;
    spin_lock_irq(&b.lock);
    immediate = !list_empty(&b.work_items) || !bio_list_empty(&b.bios);
    b.commit_scheduled = true;
    spin_unlock_irq(&b.lock);
    if (immediate)
    async_commit(b);
    }
//
// There are a couple of places where we let a bio run, but want to do some
// work before calling its endio function.  We do this by temporarily
// changing the endio fn.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_hook_info {
    pub bi_end_io: *mut bio_end_io_t,
}

    static void dm_hook_bio(struct dm_hook_info *h, struct bio *bio,
    bio_end_io_t *bi_end_io, void *bi_private)
    {
    h.bi_end_io = bio.bi_end_io;
    bio.bi_end_io = bi_end_io;
    bio.bi_private = bi_private;
    }
#[no_mangle]
unsafe extern "C" fn dm_unhook_bio(h: *mut dm_hook_info, bio: *mut bio) {
    static void dm_unhook_bio(struct dm_hook_info *h, struct bio *bio)
    {
    bio.bi_end_io = h.bi_end_io;
    }
// ----------------------------------------------------------------
pub const MIGRATION_POOL_SIZE: c_int = 128;

pub const MIGRATION_COUNT_WINDOW: c_int = 10;
//
// The block size of the device holding cache data must be
// between 32KB and 1GB.
//

    enum cache_metadata_mode {
    CM_WRITE,		/* metadata may be changed */
    CM_READ_ONLY,		/* metadata may not be changed */
    CM_FAIL
    };
    enum cache_io_mode {
//
// Data is written to cached blocks only.  These blocks are marked
// dirty.  If you lose the cache device you will lose data.
// Potential performance increase for both reads and writes.
//
    CM_IO_WRITEBACK,
//
// Data is written to both cache and origin.  Blocks are never
// dirty.  Potential performance benfit for reads only.
//
    CM_IO_WRITETHROUGH,
//
// A degraded mode useful for various cache coherency situations
// (eg, rolling back snapshots).  Reads and writes always go to the
// origin.  If a write goes to a cached oblock, then the cache
// block is invalidated.
//
    CM_IO_PASSTHROUGH
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_features {
    pub mode: enum cache_metadata_mode,
    pub io_mode: enum cache_io_mode,
    pub metadata_version: c_uint,
    pub discard_passdown:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_stats {
    pub read_hit: core::sync::atomic::AtomicI32,
    pub read_miss: core::sync::atomic::AtomicI32,
    pub write_hit: core::sync::atomic::AtomicI32,
    pub write_miss: core::sync::atomic::AtomicI32,
    pub demotion: core::sync::atomic::AtomicI32,
    pub promotion: core::sync::atomic::AtomicI32,
    pub writeback: core::sync::atomic::AtomicI32,
    pub copies_avoided: core::sync::atomic::AtomicI32,
    pub cache_cell_clash: core::sync::atomic::AtomicI32,
    pub commit_count: core::sync::atomic::AtomicI32,
    pub discard_count: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache {
    pub ti: *mut dm_target,
    pub lock: spinlock_t,
//
// Fields for converting from sectors to blocks.
//
    pub sectors_per_block_shift: c_int,
    pub sectors_per_block: sector_t,
    pub cmd: *mut dm_cache_metadata,
//
// Metadata is written to this device.
//
    pub metadata_dev: *mut dm_dev,
//
// The slower of the two data devices.  Typically a spindle.
//
    pub origin_dev: *mut dm_dev,
//
// The faster of the two data devices.  Typically an SSD.
//
    pub cache_dev: *mut dm_dev,
//
// Size of the origin device in _complete_ blocks and native sectors.
//
    pub origin_blocks: dm_oblock_t,
    pub origin_sectors: sector_t,
//
// Size of the cache device in blocks.
//
    pub cache_size: dm_cblock_t,
//
// Invalidation fields.
//
    pub invalidation_lock: spinlock_t,
    pub invalidation_requests: list_head,
    pub migration_threshold: sector_t,
//
// The number of in flight migrations that are performing
// background io. eg, promotion, writeback.
//
    pub nr_io_migrations: core::sync::atomic::AtomicI32,
    pub deferred_bios: bio_list,
    pub quiesce_lock: rw_semaphore,
//
// origin_blocks entries, discarded if set.
//
    pub discard_nr_blocks: dm_dblock_t,
    pub discard_bitset: *mut c_ulong,
    pub /: *mut *mut uint32_t discard_block_size; / a power of 2 times sectors per block,
//
// Rather than reconstructing the table line for the status we just
// save it and regurgitate.
//
    pub nr_ctr_args: c_uint,
    pub ctr_args: *const c_char,
    pub copier: *mut dm_kcopyd_client,
    pub deferred_bio_worker: work_struct,
    pub migration_worker: work_struct,
    pub wq: *mut workqueue_struct,
    pub waker: delayed_work,
    pub prison: *mut dm_bio_prison_v2,
//
// cache_size entries, dirty if set
//
    pub dirty_bitset: *mut c_ulong,
    pub nr_dirty: core::sync::atomic::AtomicI32,
    pub policy_nr_args: c_uint,
    pub policy: *mut dm_cache_policy,
//
// Cache features such as write-through.
//
    pub features: cache_features,
    pub stats: cache_stats,
    pub need_tick_bio:1: bool,
    pub sized:1: bool,
    pub invalidate:1: bool,
    pub commit_requested:1: bool,
    pub loaded_mappings:1: bool,
    pub loaded_discards:1: bool,
// background work management
    pub background_work_allowed: bool,
    pub background_work_nr: unsigned,
    pub background_work_lock: spinlock_t,
    pub background_work_wait: wait_queue_head_t,
    pub committer: batcher,
    pub commit_ws: work_struct,
    pub tracker: dm_io_tracker,
    pub migration_pool: mempool_t,
    pub bs: bio_set,
//
// Cache_size entries. Set bits indicate blocks mapped beyond the
// target length, which are marked for invalidation.
//
    pub invalid_bitset: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_bio_data {
    pub tick:1: bool,
    pub req_nr:2: c_uint,
    pub cell: *mut dm_bio_prison_cell_v2,
    pub hook_info: dm_hook_info,
    pub len: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_cache_migration {
    pub k: continuation,
    pub cache: *mut cache,
    pub op: *mut policy_work,
    pub overwrite_bio: *mut bio,
    pub cell: *mut dm_bio_prison_cell_v2,
    pub invalidate_cblock: dm_cblock_t,
    pub invalidate_oblock: dm_oblock_t,
}

// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn writethrough_mode(cache: *mut cache) -> bool {
    static bool writethrough_mode(struct cache *cache)
    {
    return cache.features.io_mode == CM_IO_WRITETHROUGH;
    }
#[no_mangle]
unsafe extern "C" fn writeback_mode(cache: *mut cache) -> bool {
    static bool writeback_mode(struct cache *cache)
    {
    return cache.features.io_mode == CM_IO_WRITEBACK;
    }
#[no_mangle]
pub unsafe extern "C" fn passthrough_mode(cache: *mut cache) -> bool {
    static inline bool passthrough_mode(struct cache *cache)
    {
    return unlikely(cache.features.io_mode == CM_IO_PASSTHROUGH);
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn wake_deferred_bio_worker(cache: *mut cache) {
    static void wake_deferred_bio_worker(struct cache *cache)
    {
    queue_work(cache.wq, &cache.deferred_bio_worker);
    }
#[no_mangle]
unsafe extern "C" fn wake_migration_worker(cache: *mut cache) {
    static void wake_migration_worker(struct cache *cache)
    {
    if (passthrough_mode(cache))
    return;
    queue_work(cache.wq, &cache.migration_worker);
    }
// ----------------------------------------------------------------
    static struct dm_bio_prison_cell_v2 *alloc_prison_cell(struct cache *cache)
    {
    return dm_bio_prison_alloc_cell_v2(cache.prison, GFP_NOIO);
    }
#[no_mangle]
unsafe extern "C" fn free_prison_cell(cache: *mut cache, cell: *mut dm_bio_prison_cell_v2) {
    static void free_prison_cell(struct cache *cache, struct dm_bio_prison_cell_v2 *cell)
    {
    dm_bio_prison_free_cell_v2(cache.prison, cell);
    }
    static struct dm_cache_migration *alloc_migration(struct cache *cache)
    {
    struct dm_cache_migration *mg;
    mg = mempool_alloc(&cache.migration_pool, GFP_NOIO);
    memset(mg, 0, sizeof(*mg));
    mg.cache = cache;
    return mg;
    }
#[no_mangle]
unsafe extern "C" fn free_migration(mg: *mut dm_cache_migration) {
    static void free_migration(struct dm_cache_migration *mg)
    {
    mempool_free(mg, &mg.cache.migration_pool);
    }
// ----------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn oblock_succ(b: dm_oblock_t) -> dm_oblock_t {
    static inline dm_oblock_t oblock_succ(dm_oblock_t b)
    {
    return to_oblock(from_oblock(b) + 1ull);
    }
#[no_mangle]
unsafe extern "C" fn build_key(begin: dm_oblock_t, end: dm_oblock_t, key: *mut dm_cell_key_v2) {
    static void build_key(dm_oblock_t begin, dm_oblock_t end, struct dm_cell_key_v2 *key)
    {
    key.virtual = 0;
    key.dev = 0;
    key.block_begin = from_oblock(begin);
    key.block_end = from_oblock(end);
    }
//
// We have two lock levels.  Level 0, which is used to prevent WRITEs, and
// level 1 which prevents *both* READs and WRITEs.
//
pub const WRITE_LOCK_LEVEL: c_int = 0;
pub const READ_WRITE_LOCK_LEVEL: c_int = 1;
#[no_mangle]
unsafe extern "C" fn lock_level(bio: *mut bio) -> c_uint {
    static unsigned int lock_level(struct bio *bio)
    {
    return bio_data_dir(bio) == WRITE ?
    WRITE_LOCK_LEVEL :
    READ_WRITE_LOCK_LEVEL;
    }
//
// --------------------------------------------------------------
// Per bio data
// --------------------------------------------------------------
//
    static struct per_bio_data *get_per_bio_data(struct bio *bio)
    {
    struct per_bio_data *pb = dm_per_bio_data(bio, sizeof(struct per_bio_data));
    BUG_ON(!pb);
    return pb;
    }
    static struct per_bio_data *init_per_bio_data(struct bio *bio)
    {
    struct per_bio_data *pb = get_per_bio_data(bio);
    pb.tick = false;
    pb.req_nr = dm_bio_get_target_bio_nr(bio);
    pb.cell = core::ptr::null_mut();
    pb.len = 0;
    return pb;
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn defer_bio(cache: *mut cache, bio: *mut bio) {
    static void defer_bio(struct cache *cache, struct bio *bio)
    {
    spin_lock_irq(&cache.lock);
    bio_list_add(&cache.deferred_bios, bio);
    spin_unlock_irq(&cache.lock);
    wake_deferred_bio_worker(cache);
    }
#[no_mangle]
unsafe extern "C" fn defer_bios(cache: *mut cache, bios: *mut bio_list) {
    static void defer_bios(struct cache *cache, struct bio_list *bios)
    {
    spin_lock_irq(&cache.lock);
    bio_list_merge_init(&cache.deferred_bios, bios);
    spin_unlock_irq(&cache.lock);
    wake_deferred_bio_worker(cache);
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn bio_detain_shared(cache: *mut cache, oblock: dm_oblock_t, bio: *mut bio) -> bool {
    static bool bio_detain_shared(struct cache *cache, dm_oblock_t oblock, struct bio *bio)
    {
    bool r;
    struct per_bio_data *pb;
    struct dm_cell_key_v2 key;
    let mut end: dm_oblock_t = to_oblock(from_oblock(oblock) + 1ULL);
    struct dm_bio_prison_cell_v2 *cell_prealloc, *cell;
    cell_prealloc = alloc_prison_cell(cache); /* FIXME: allow wait if calling from worker */
    build_key(oblock, end, &key);
    r = dm_cell_get_v2(cache.prison, &key, lock_level(bio), bio, cell_prealloc, &cell);
    if (!r) {
//
// Failed to get the lock.
//
    free_prison_cell(cache, cell_prealloc);
    return r;
    }
    if (cell != cell_prealloc)
    free_prison_cell(cache, cell_prealloc);
    pb = get_per_bio_data(bio);
    pb.cell = cell;
    return r;
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn is_dirty(cache: *mut cache, b: dm_cblock_t) -> bool {
    static bool is_dirty(struct cache *cache, dm_cblock_t b)
    {
    return test_bit(from_cblock(b), cache.dirty_bitset);
    }
#[no_mangle]
unsafe extern "C" fn set_dirty(cache: *mut cache, cblock: dm_cblock_t) {
    static void set_dirty(struct cache *cache, dm_cblock_t cblock)
    {
    if (!test_and_set_bit(from_cblock(cblock), cache.dirty_bitset)) {
    atomic_inc(&cache.nr_dirty);
    policy_set_dirty(cache.policy, cblock);
    }
    }
//
// These two are called when setting after migrations to force the policy
// and dirty bitset to be in sync.
//
#[no_mangle]
unsafe extern "C" fn force_set_dirty(cache: *mut cache, cblock: dm_cblock_t) {
    static void force_set_dirty(struct cache *cache, dm_cblock_t cblock)
    {
    if (!test_and_set_bit(from_cblock(cblock), cache.dirty_bitset))
    atomic_inc(&cache.nr_dirty);
    policy_set_dirty(cache.policy, cblock);
    }
#[no_mangle]
unsafe extern "C" fn force_clear_dirty(cache: *mut cache, cblock: dm_cblock_t) {
    static void force_clear_dirty(struct cache *cache, dm_cblock_t cblock)
    {
    if (test_and_clear_bit(from_cblock(cblock), cache.dirty_bitset)) {
    if (atomic_dec_return(&cache.nr_dirty) == 0)
    dm_table_event(cache.ti.table);
    }
    policy_clear_dirty(cache.policy, cblock);
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn block_size_is_power_of_two(cache: *mut cache) -> bool {
    static bool block_size_is_power_of_two(struct cache *cache)
    {
    return cache.sectors_per_block_shift >= 0;
    }
#[no_mangle]
unsafe extern "C" fn block_div(b: dm_block_t, n: u32) -> dm_block_t {
    static dm_block_t block_div(dm_block_t b, uint32_t n)
    {
    do_div(b, n);
    return b;
    }
#[no_mangle]
unsafe extern "C" fn oblocks_per_dblock(cache: *mut cache) -> dm_block_t {
    static dm_block_t oblocks_per_dblock(struct cache *cache)
    {
    let mut oblocks: dm_block_t = cache.discard_block_size;
    if (block_size_is_power_of_two(cache))
    oblocks >>= cache.sectors_per_block_shift;
    else
    oblocks = block_div(oblocks, cache.sectors_per_block);
    return oblocks;
    }
#[no_mangle]
unsafe extern "C" fn oblock_to_dblock(cache: *mut cache, oblock: dm_oblock_t) -> dm_dblock_t {
    static dm_dblock_t oblock_to_dblock(struct cache *cache, dm_oblock_t oblock)
    {
    return to_dblock(block_div(from_oblock(oblock),
    oblocks_per_dblock(cache)));
    }
#[no_mangle]
unsafe extern "C" fn set_discard(cache: *mut cache, b: dm_dblock_t) {
    static void set_discard(struct cache *cache, dm_dblock_t b)
    {
    BUG_ON(from_dblock(b) >= from_dblock(cache.discard_nr_blocks));
    atomic_inc(&cache.stats.discard_count);
    spin_lock_irq(&cache.lock);
    set_bit(from_dblock(b), cache.discard_bitset);
    spin_unlock_irq(&cache.lock);
    }
#[no_mangle]
unsafe extern "C" fn clear_discard(cache: *mut cache, b: dm_dblock_t) {
    static void clear_discard(struct cache *cache, dm_dblock_t b)
    {
    spin_lock_irq(&cache.lock);
    clear_bit(from_dblock(b), cache.discard_bitset);
    spin_unlock_irq(&cache.lock);
    }
#[no_mangle]
unsafe extern "C" fn is_discarded(cache: *mut cache, b: dm_dblock_t) -> bool {
    static bool is_discarded(struct cache *cache, dm_dblock_t b)
    {
    int r;
    spin_lock_irq(&cache.lock);
    r = test_bit(from_dblock(b), cache.discard_bitset);
    spin_unlock_irq(&cache.lock);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn is_discarded_oblock(cache: *mut cache, b: dm_oblock_t) -> bool {
    static bool is_discarded_oblock(struct cache *cache, dm_oblock_t b)
    {
    int r;
    spin_lock_irq(&cache.lock);
    r = test_bit(from_dblock(oblock_to_dblock(cache, b)),
    cache.discard_bitset);
    spin_unlock_irq(&cache.lock);
    return r;
    }
//
// -------------------------------------------------------------
// Remapping
// --------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn remap_to_origin(cache: *mut cache, bio: *mut bio) {
    static void remap_to_origin(struct cache *cache, struct bio *bio)
    {
    bio_set_dev(bio, cache.origin_dev.bdev);
    }
    static void remap_to_cache(struct cache *cache, struct bio *bio,
    dm_cblock_t cblock)
    {
    let mut bi_sector: sector_t = bio.bi_iter.bi_sector;
    let mut block: sector_t = from_cblock(cblock);
    bio_set_dev(bio, cache.cache_dev.bdev);
    if (!block_size_is_power_of_two(cache))
    bio.bi_iter.bi_sector =
    (block * cache.sectors_per_block) +
    sector_div(bi_sector, cache.sectors_per_block);
    else
    bio.bi_iter.bi_sector =
    (block << cache.sectors_per_block_shift) |
    (bi_sector & (cache.sectors_per_block - 1));
    }
#[no_mangle]
unsafe extern "C" fn check_if_tick_bio_needed(cache: *mut cache, bio: *mut bio) {
    static void check_if_tick_bio_needed(struct cache *cache, struct bio *bio)
    {
    struct per_bio_data *pb;
    spin_lock_irq(&cache.lock);
    if (cache.need_tick_bio && !op_is_flush(bio.bi_opf) &&
    bio_op(bio) != REQ_OP_DISCARD) {
    pb = get_per_bio_data(bio);
    pb.tick = true;
    cache.need_tick_bio = false;
    }
    spin_unlock_irq(&cache.lock);
    }
    static void remap_to_origin_clear_discard(struct cache *cache, struct bio *bio,
    dm_oblock_t oblock)
    {
// FIXME: check_if_tick_bio_needed() is called way too much through this interface
    check_if_tick_bio_needed(cache, bio);
    remap_to_origin(cache, bio);
    if (bio_data_dir(bio) == WRITE)
    clear_discard(cache, oblock_to_dblock(cache, oblock));
    }
    static void remap_to_cache_dirty(struct cache *cache, struct bio *bio,
    dm_oblock_t oblock, dm_cblock_t cblock)
    {
    check_if_tick_bio_needed(cache, bio);
    remap_to_cache(cache, bio, cblock);
    if (bio_data_dir(bio) == WRITE) {
    set_dirty(cache, cblock);
    clear_discard(cache, oblock_to_dblock(cache, oblock));
    }
    }
#[no_mangle]
unsafe extern "C" fn get_bio_block(cache: *mut cache, bio: *mut bio) -> dm_oblock_t {
    static dm_oblock_t get_bio_block(struct cache *cache, struct bio *bio)
    {
    let mut block_nr: sector_t = bio.bi_iter.bi_sector;
    if (!block_size_is_power_of_two(cache))
    (void) sector_div(block_nr, cache.sectors_per_block);
    else
    block_nr >>= cache.sectors_per_block_shift;
    return to_oblock(block_nr);
    }
#[no_mangle]
unsafe extern "C" fn accountable_bio(cache: *mut cache, bio: *mut bio) -> bool {
    static bool accountable_bio(struct cache *cache, struct bio *bio)
    {
    return bio_op(bio) != REQ_OP_DISCARD;
    }
#[no_mangle]
unsafe extern "C" fn accounted_begin(cache: *mut cache, bio: *mut bio) {
    static void accounted_begin(struct cache *cache, struct bio *bio)
    {
    struct per_bio_data *pb;
    if (accountable_bio(cache, bio)) {
    pb = get_per_bio_data(bio);
    pb.len = bio_sectors(bio);
    dm_iot_io_begin(&cache.tracker, pb.len);
    }
    }
#[no_mangle]
unsafe extern "C" fn accounted_complete(cache: *mut cache, bio: *mut bio) {
    static void accounted_complete(struct cache *cache, struct bio *bio)
    {
    struct per_bio_data *pb = get_per_bio_data(bio);
    dm_iot_io_end(&cache.tracker, pb.len);
    }
#[no_mangle]
unsafe extern "C" fn accounted_request(cache: *mut cache, bio: *mut bio) {
    static void accounted_request(struct cache *cache, struct bio *bio)
    {
    accounted_begin(cache, bio);
    dm_submit_bio_remap(bio, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn issue_op(bio: *mut bio, context: *mut c_void) {
    static void issue_op(struct bio *bio, void *context)
    {
    struct cache *cache = context;
    accounted_request(cache, bio);
    }
//
// When running in writethrough mode we need to send writes to clean blocks
// to both the cache and origin devices.  Clone the bio and send them in parallel.
//
    static void remap_to_origin_and_cache(struct cache *cache, struct bio *bio,
    dm_oblock_t oblock, dm_cblock_t cblock)
    {
    struct bio *origin_bio = bio_alloc_clone(cache.origin_dev.bdev, bio,
    GFP_NOIO, &cache.bs);
    BUG_ON(!origin_bio);
    bio_chain(origin_bio, bio);
    if (bio_data_dir(origin_bio) == WRITE)
    clear_discard(cache, oblock_to_dblock(cache, oblock));
    submit_bio(origin_bio);
    remap_to_cache(cache, bio, cblock);
    }
//
// --------------------------------------------------------------
// Failure modes
// --------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn get_cache_mode(cache: *mut cache) -> enum cache_metadata_mode {
    static enum cache_metadata_mode get_cache_mode(struct cache *cache)
    {
    return cache.features.mode;
    }
    static const char *cache_device_name(struct cache *cache)
    {
    return dm_table_device_name(cache.ti.table);
    }
#[no_mangle]
unsafe extern "C" fn notify_mode_switch(cache: *mut cache, mode: enum cache_metadata_mode) {
    static void notify_mode_switch(struct cache *cache, enum cache_metadata_mode mode)
    {
    static const char *descs[] = {
    "write",
    "read-only",
    "fail"
    };
    dm_table_event(cache.ti.table);
    DMINFO("%s: switching cache to %s mode",
    cache_device_name(cache), descs[(int)mode]);
    }
#[no_mangle]
unsafe extern "C" fn set_cache_mode(cache: *mut cache, new_mode: enum cache_metadata_mode) {
    static void set_cache_mode(struct cache *cache, enum cache_metadata_mode new_mode)
    {
    bool needs_check;
    let mut old_mode: enum cache_metadata_mode = get_cache_mode(cache);
    if (dm_cache_metadata_needs_check(cache.cmd, &needs_check)) {
    DMERR("%s: unable to read needs_check flag, setting failure mode.",
    cache_device_name(cache));
    new_mode = CM_FAIL;
    }
    if (new_mode == CM_WRITE && needs_check) {
    DMERR("%s: unable to switch cache to write mode until repaired.",
    cache_device_name(cache));
    if (old_mode != new_mode)
    new_mode = old_mode;
    else
    new_mode = CM_READ_ONLY;
    }
// Never move out of fail mode
    if (old_mode == CM_FAIL)
    new_mode = CM_FAIL;
    switch (new_mode) {
    case CM_FAIL:
    case CM_READ_ONLY:
    dm_cache_metadata_set_read_only(cache.cmd);
    break;
    case CM_WRITE:
    dm_cache_metadata_set_read_write(cache.cmd);
    break;
    }
    cache.features.mode = new_mode;
    if (new_mode != old_mode)
    notify_mode_switch(cache, new_mode);
    }
#[no_mangle]
unsafe extern "C" fn abort_transaction(cache: *mut cache) {
    static void abort_transaction(struct cache *cache)
    {
    const char *dev_name = cache_device_name(cache);
    if (get_cache_mode(cache) >= CM_READ_ONLY)
    return;
    DMERR_LIMIT("%s: aborting current metadata transaction", dev_name);
    if (dm_cache_metadata_abort(cache.cmd)) {
    DMERR("%s: failed to abort metadata transaction", dev_name);
    set_cache_mode(cache, CM_FAIL);
    }
    if (dm_cache_metadata_set_needs_check(cache.cmd)) {
    DMERR("%s: failed to set 'needs_check' flag in metadata", dev_name);
    set_cache_mode(cache, CM_FAIL);
    }
    }
#[no_mangle]
unsafe extern "C" fn metadata_operation_failed(cache: *mut cache, op: *const c_char, r: c_int) {
    static void metadata_operation_failed(struct cache *cache, const char *op, int r)
    {
    DMERR_LIMIT("%s: metadata operation '%s' failed: error = %d",
    cache_device_name(cache), op, r);
    abort_transaction(cache);
    set_cache_mode(cache, CM_READ_ONLY);
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn load_stats(cache: *mut cache) {
    static void load_stats(struct cache *cache)
    {
    struct dm_cache_statistics stats;
    dm_cache_metadata_get_stats(cache.cmd, &stats);
    atomic_set(&cache.stats.read_hit, stats.read_hits);
    atomic_set(&cache.stats.read_miss, stats.read_misses);
    atomic_set(&cache.stats.write_hit, stats.write_hits);
    atomic_set(&cache.stats.write_miss, stats.write_misses);
    }
#[no_mangle]
unsafe extern "C" fn save_stats(cache: *mut cache) {
    static void save_stats(struct cache *cache)
    {
    struct dm_cache_statistics stats;
    if (get_cache_mode(cache) >= CM_READ_ONLY)
    return;
    stats.read_hits = atomic_read(&cache.stats.read_hit);
    stats.read_misses = atomic_read(&cache.stats.read_miss);
    stats.write_hits = atomic_read(&cache.stats.write_hit);
    stats.write_misses = atomic_read(&cache.stats.write_miss);
    dm_cache_metadata_set_stats(cache.cmd, &stats);
    }
#[no_mangle]
unsafe extern "C" fn update_stats(stats: *mut cache_stats, op: enum policy_operation) {
    static void update_stats(struct cache_stats *stats, enum policy_operation op)
    {
    switch (op) {
    case POLICY_PROMOTE:
    atomic_inc(&stats.promotion);
    break;
    case POLICY_DEMOTE:
    atomic_inc(&stats.demotion);
    break;
    case POLICY_WRITEBACK:
    atomic_inc(&stats.writeback);
    break;
    }
    }
//
// ---------------------------------------------------------------------
// Migration processing
//
// Migration covers moving data from the origin device to the cache, or
// vice versa.
// ---------------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn inc_io_migrations(cache: *mut cache) {
    static void inc_io_migrations(struct cache *cache)
    {
    atomic_inc(&cache.nr_io_migrations);
    }
#[no_mangle]
unsafe extern "C" fn dec_io_migrations(cache: *mut cache) {
    static void dec_io_migrations(struct cache *cache)
    {
    atomic_dec(&cache.nr_io_migrations);
    }
#[no_mangle]
unsafe extern "C" fn discard_or_flush(bio: *mut bio) -> bool {
    static bool discard_or_flush(struct bio *bio)
    {
    return bio_op(bio) == REQ_OP_DISCARD || op_is_flush(bio.bi_opf);
    }
    static void calc_discard_block_range(struct cache *cache, struct bio *bio,
    dm_dblock_t *b, dm_dblock_t *e)
    {
    let mut sb: sector_t = bio.bi_iter.bi_sector;
    let mut se: sector_t = bio_end_sector(bio);
// b = to_dblock(dm_sector_div_up(sb, cache->discard_block_size));
    if (se - sb < cache.discard_block_size)
// e = *b;
    else
// e = to_dblock(block_div(se, cache->discard_block_size));
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn prevent_background_work(cache: *mut cache) {
    static void prevent_background_work(struct cache *cache)
    {
    spin_lock_irq(&cache.background_work_lock);
    cache.background_work_allowed = false;
    wait_event_lock_irq(cache.background_work_wait,
    cache.background_work_nr == 0,
    cache.background_work_lock);
    spin_unlock_irq(&cache.background_work_lock);
    }
#[no_mangle]
unsafe extern "C" fn allow_background_work(cache: *mut cache) {
    static void allow_background_work(struct cache *cache)
    {
    spin_lock_irq(&cache.background_work_lock);
    cache.background_work_allowed = true;
    spin_unlock_irq(&cache.background_work_lock);
    }
#[no_mangle]
unsafe extern "C" fn background_work_begin(cache: *mut cache) -> bool {
    static bool background_work_begin(struct cache *cache)
    {
    bool r;
    spin_lock_irq(&cache.background_work_lock);
    r = cache.background_work_allowed;
    if (r)
    cache.background_work_nr++;
    spin_unlock_irq(&cache.background_work_lock);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn background_work_end(cache: *mut cache) {
    static void background_work_end(struct cache *cache)
    {
    spin_lock_irq(&cache.background_work_lock);
    if (--cache.background_work_nr == 0)
    wake_up(&cache.background_work_wait);
    spin_unlock_irq(&cache.background_work_lock);
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn bio_writes_complete_block(cache: *mut cache, bio: *mut bio) -> bool {
    static bool bio_writes_complete_block(struct cache *cache, struct bio *bio)
    {
    return (bio_data_dir(bio) == WRITE) &&
    (bio.bi_iter.bi_size == (cache.sectors_per_block << SECTOR_SHIFT));
    }
#[no_mangle]
unsafe extern "C" fn optimisable_bio(cache: *mut cache, bio: *mut bio, block: dm_oblock_t) -> bool {
    static bool optimisable_bio(struct cache *cache, struct bio *bio, dm_oblock_t block)
    {
    return writeback_mode(cache) &&
    (is_discarded_oblock(cache, block) || bio_writes_complete_block(cache, bio));
    }
    static void quiesce(struct dm_cache_migration *mg,
    void (*continuation)(struct work_struct *))
    {
    init_continuation(&mg.k, continuation);
    dm_cell_quiesce_v2(mg.cache.prison, mg.cell, &mg.k.ws);
    }
    static struct dm_cache_migration *ws_to_mg(struct work_struct *ws)
    {
    struct continuation *k = container_of(ws, struct continuation, ws);
    return container_of(k, struct dm_cache_migration, k);
    }
#[no_mangle]
unsafe extern "C" fn copy_complete(read_err: c_int, write_err: c_ulong, context: *mut c_void) {
    static void copy_complete(int read_err, unsigned long write_err, void *context)
    {
    struct dm_cache_migration *mg = container_of(context, struct dm_cache_migration, k);
    if (read_err || write_err)
    mg.k.input = BLK_STS_IOERR;
    queue_continuation(mg.cache.wq, &mg.k);
    }
#[no_mangle]
unsafe extern "C" fn copy(mg: *mut dm_cache_migration, promote: bool) {
    static void copy(struct dm_cache_migration *mg, bool promote)
    {
    struct dm_io_region o_region, c_region;
    struct cache *cache = mg.cache;
    o_region.bdev = cache.origin_dev.bdev;
    o_region.sector = from_oblock(mg.op.oblock) * cache.sectors_per_block;
    o_region.count = cache.sectors_per_block;
    c_region.bdev = cache.cache_dev.bdev;
    c_region.sector = from_cblock(mg.op.cblock) * cache.sectors_per_block;
    c_region.count = cache.sectors_per_block;
    if (promote)
    dm_kcopyd_copy(cache.copier, &o_region, 1, &c_region, 0, copy_complete, &mg.k);
    else
    dm_kcopyd_copy(cache.copier, &c_region, 1, &o_region, 0, copy_complete, &mg.k);
    }
#[no_mangle]
unsafe extern "C" fn bio_drop_shared_lock(cache: *mut cache, bio: *mut bio) {
    static void bio_drop_shared_lock(struct cache *cache, struct bio *bio)
    {
    struct per_bio_data *pb = get_per_bio_data(bio);
    if (pb.cell && dm_cell_put_v2(cache.prison, pb.cell))
    free_prison_cell(cache, pb.cell);
    pb.cell = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn overwrite_endio(bio: *mut bio) {
    static void overwrite_endio(struct bio *bio)
    {
    struct dm_cache_migration *mg = bio.bi_private;
    struct cache *cache = mg.cache;
    struct per_bio_data *pb = get_per_bio_data(bio);
    dm_unhook_bio(&pb.hook_info, bio);
    if (bio.bi_status)
    mg.k.input = bio.bi_status;
    queue_continuation(cache.wq, &mg.k);
    }
    static void overwrite(struct dm_cache_migration *mg,
    void (*continuation)(struct work_struct *))
    {
    struct bio *bio = mg.overwrite_bio;
    struct per_bio_data *pb = get_per_bio_data(bio);
    dm_hook_bio(&pb.hook_info, bio, overwrite_endio, mg);
//
// The overwrite bio is part of the copy operation, as such it does
// not set/clear discard or dirty flags.
//
    if (mg.op.op == POLICY_PROMOTE)
    remap_to_cache(mg.cache, bio, mg.op.cblock);
    else
    remap_to_origin(mg.cache, bio);
    init_continuation(&mg.k, continuation);
    accounted_request(mg.cache, bio);
    }
//
// Migration steps:
//
// 1) exclusive lock preventing WRITEs
// 2) quiesce
// 3) copy or issue overwrite bio
// 4) upgrade to exclusive lock preventing READs and WRITEs
// 5) quiesce
// 6) update metadata and commit
// 7) unlock
//
#[no_mangle]
unsafe extern "C" fn mg_complete(mg: *mut dm_cache_migration, success: bool) {
    static void mg_complete(struct dm_cache_migration *mg, bool success)
    {
    struct bio_list bios;
    struct cache *cache = mg.cache;
    struct policy_work *op = mg.op;
    let mut cblock: dm_cblock_t = op.cblock;
    if (success)
    update_stats(&cache.stats, op.op);
    switch (op.op) {
    case POLICY_PROMOTE:
    clear_discard(cache, oblock_to_dblock(cache, op.oblock));
    policy_complete_background_work(cache.policy, op, success);
    if (mg.overwrite_bio) {
    if (success)
    force_set_dirty(cache, cblock);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: mg->k.input) -> else {
    else if (mg.k.input)
    mg.overwrite_bio.bi_status = mg.k.input;
    else
    mg.overwrite_bio.bi_status = BLK_STS_IOERR;
    bio_endio(mg.overwrite_bio);
    } else {
    if (success)
    force_clear_dirty(cache, cblock);
    dec_io_migrations(cache);
    }
    break;
    case POLICY_DEMOTE:
//
// We clear dirty here to update the nr_dirty counter.
//
    if (success)
    force_clear_dirty(cache, cblock);
    policy_complete_background_work(cache.policy, op, success);
    dec_io_migrations(cache);
    break;
    case POLICY_WRITEBACK:
    if (success)
    force_clear_dirty(cache, cblock);
    policy_complete_background_work(cache.policy, op, success);
    dec_io_migrations(cache);
    break;
    }
    bio_list_init(&bios);
    if (mg.cell) {
    if (dm_cell_unlock_v2(cache.prison, mg.cell, &bios))
    free_prison_cell(cache, mg.cell);
    }
    free_migration(mg);
    defer_bios(cache, &bios);
    wake_migration_worker(cache);
    background_work_end(cache);
    }
#[no_mangle]
unsafe extern "C" fn mg_success(ws: *mut work_struct) {
    static void mg_success(struct work_struct *ws)
    {
    struct dm_cache_migration *mg = ws_to_mg(ws);
    mg_complete(mg, mg.k.input == 0);
    }
#[no_mangle]
unsafe extern "C" fn mg_update_metadata(ws: *mut work_struct) {
    static void mg_update_metadata(struct work_struct *ws)
    {
    int r;
    struct dm_cache_migration *mg = ws_to_mg(ws);
    struct cache *cache = mg.cache;
    struct policy_work *op = mg.op;
    switch (op.op) {
    case POLICY_PROMOTE:
    r = dm_cache_insert_mapping(cache.cmd, op.cblock, op.oblock);
    if (r) {
    DMERR_LIMIT("%s: migration failed; couldn't insert mapping",
    cache_device_name(cache));
    metadata_operation_failed(cache, "dm_cache_insert_mapping", r);
    mg_complete(mg, false);
    return;
    }
    mg_complete(mg, true);
    break;
    case POLICY_DEMOTE:
    r = dm_cache_remove_mapping(cache.cmd, op.cblock);
    if (r) {
    DMERR_LIMIT("%s: migration failed; couldn't update on disk metadata",
    cache_device_name(cache));
    metadata_operation_failed(cache, "dm_cache_remove_mapping", r);
    mg_complete(mg, false);
    return;
    }
//
// It would be nice if we only had to commit when a REQ_FLUSH
// comes through.  But there's one scenario that we have to
// look out for:
//
// - vblock x in a cache block
// - domotion occurs
// - cache block gets reallocated and over written
// - crash
//
// When we recover, because there was no commit the cache will
// rollback to having the data for vblock x in the cache block.
// But the cache block has since been overwritten, so it'll end
// up pointing to data that was never in 'x' during the history
// of the device.
//
// To avoid this issue we require a commit as part of the
// demotion operation.
//
    init_continuation(&mg.k, mg_success);
    continue_after_commit(&cache.committer, &mg.k);
    schedule_commit(&cache.committer);
    break;
    case POLICY_WRITEBACK:
    mg_complete(mg, true);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mg_update_metadata_after_copy(ws: *mut work_struct) {
    static void mg_update_metadata_after_copy(struct work_struct *ws)
    {
    struct dm_cache_migration *mg = ws_to_mg(ws);
//
// Did the copy succeed?
//
    if (mg.k.input)
    mg_complete(mg, false);
    else
    mg_update_metadata(ws);
    }
#[no_mangle]
unsafe extern "C" fn mg_upgrade_lock(ws: *mut work_struct) {
    static void mg_upgrade_lock(struct work_struct *ws)
    {
    int r;
    struct dm_cache_migration *mg = ws_to_mg(ws);
//
// Did the copy succeed?
//
    if (mg.k.input)
    mg_complete(mg, false);
    else {
//
// Now we want the lock to prevent both reads and writes.
//
    r = dm_cell_lock_promote_v2(mg.cache.prison, mg.cell,
    READ_WRITE_LOCK_LEVEL);
    if (r < 0)
    mg_complete(mg, false);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: r) -> else {
    else if (r)
    quiesce(mg, mg_update_metadata);
    else
    mg_update_metadata(ws);
    }
    }
#[no_mangle]
unsafe extern "C" fn mg_full_copy(ws: *mut work_struct) {
    static void mg_full_copy(struct work_struct *ws)
    {
    struct dm_cache_migration *mg = ws_to_mg(ws);
    struct cache *cache = mg.cache;
    struct policy_work *op = mg.op;
    let mut is_policy_promote: bool = (op.op == POLICY_PROMOTE);
    if ((!is_policy_promote && !is_dirty(cache, op.cblock)) ||
    is_discarded_oblock(cache, op.oblock)) {
    mg_upgrade_lock(ws);
    return;
    }
    init_continuation(&mg.k, mg_upgrade_lock);
    copy(mg, is_policy_promote);
    }
#[no_mangle]
unsafe extern "C" fn mg_copy(ws: *mut work_struct) {
    static void mg_copy(struct work_struct *ws)
    {
    struct dm_cache_migration *mg = ws_to_mg(ws);
    if (mg.overwrite_bio) {
//
// No exclusive lock was held when we last checked if the bio
// was optimisable.  So we have to check again in case things
// have changed (eg, the block may no longer be discarded).
//
    if (!optimisable_bio(mg.cache, mg.overwrite_bio, mg.op.oblock)) {
//
// Fallback to a real full copy after doing some tidying up.
//
    let mut rb: bool = bio_detain_shared(mg.cache, mg.op.oblock, mg.overwrite_bio);
    BUG_ON(rb); /* An exclusive lock must _not_ be held for this block */
    mg.overwrite_bio = core::ptr::null_mut();
    inc_io_migrations(mg.cache);
    mg_full_copy(ws);
    return;
    }
//
// It's safe to do this here, even though it's new data
// because all IO has been locked out of the block.
//
// mg_lock_writes() already took READ_WRITE_LOCK_LEVEL
// so _not_ using mg_upgrade_lock() as continutation.
//
    overwrite(mg, mg_update_metadata_after_copy);
    } else
    mg_full_copy(ws);
    }
#[no_mangle]
unsafe extern "C" fn mg_lock_writes(mg: *mut dm_cache_migration) -> c_int {
    static int mg_lock_writes(struct dm_cache_migration *mg)
    {
    int r;
    struct dm_cell_key_v2 key;
    struct cache *cache = mg.cache;
    struct dm_bio_prison_cell_v2 *prealloc;
    prealloc = alloc_prison_cell(cache);
//
// Prevent writes to the block, but allow reads to continue.
// Unless we're using an overwrite bio, in which case we lock
// everything.
//
    build_key(mg.op.oblock, oblock_succ(mg.op.oblock), &key);
    r = dm_cell_lock_v2(cache.prison, &key,
    mg.overwrite_bio ?  READ_WRITE_LOCK_LEVEL : WRITE_LOCK_LEVEL,
    prealloc, &mg.cell);
    if (r < 0) {
    free_prison_cell(cache, prealloc);
    mg_complete(mg, false);
    return r;
    }
    if (mg.cell != prealloc)
    free_prison_cell(cache, prealloc);
    if (r == 0)
    mg_copy(&mg.k.ws);
    else
    quiesce(mg, mg_copy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mg_start(cache: *mut cache, op: *mut policy_work, bio: *mut bio) -> c_int {
    static int mg_start(struct cache *cache, struct policy_work *op, struct bio *bio)
    {
    struct dm_cache_migration *mg;
    if (!background_work_begin(cache)) {
    policy_complete_background_work(cache.policy, op, false);
    return -EPERM;
    }
    mg = alloc_migration(cache);
    mg.op = op;
    mg.overwrite_bio = bio;
    if (!bio)
    inc_io_migrations(cache);
    return mg_lock_writes(mg);
    }
//
// --------------------------------------------------------------
// invalidation processing
// --------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn invalidate_complete(mg: *mut dm_cache_migration, success: bool) {
    static void invalidate_complete(struct dm_cache_migration *mg, bool success)
    {
    struct bio_list bios;
    struct cache *cache = mg.cache;
    if (success)
    atomic_inc(&cache.stats.demotion);
    bio_list_init(&bios);
    if (mg.cell) {
    if (dm_cell_unlock_v2(cache.prison, mg.cell, &bios))
    free_prison_cell(cache, mg.cell);
    }
    if (mg.overwrite_bio) {
// Set generic error if the bio hasn't been issued yet,
// e.g., invalidation or metadata commit failed before bio
// submission. Otherwise preserve the bio's own error status.
    if (!success && !mg.overwrite_bio.bi_status)
    mg.overwrite_bio.bi_status = BLK_STS_IOERR;
    bio_endio(mg.overwrite_bio);
    }
    free_migration(mg);
    defer_bios(cache, &bios);
    background_work_end(cache);
    }
#[no_mangle]
unsafe extern "C" fn invalidate_completed(ws: *mut work_struct) {
    static void invalidate_completed(struct work_struct *ws)
    {
    struct dm_cache_migration *mg = ws_to_mg(ws);
    invalidate_complete(mg, !mg.k.input);
    }
#[no_mangle]
unsafe extern "C" fn invalidate_cblock(cache: *mut cache, cblock: dm_cblock_t) -> c_int {
    static int invalidate_cblock(struct cache *cache, dm_cblock_t cblock)
    {
    int r;
    r = policy_invalidate_mapping(cache.policy, cblock);
    if (!r) {
    r = dm_cache_remove_mapping(cache.cmd, cblock);
    if (r) {
    DMERR_LIMIT("%s: invalidation failed; couldn't update on disk metadata",
    cache_device_name(cache));
    metadata_operation_failed(cache, "dm_cache_remove_mapping", r);
    }
    } else if (r == -ENODATA) {
//
// Harmless, already unmapped.
//
    r = 0;
    } else
    DMERR("%s: policy_invalidate_mapping failed", cache_device_name(cache));
    return r;
    }
#[no_mangle]
unsafe extern "C" fn invalidate_committed(ws: *mut work_struct) {
    static void invalidate_committed(struct work_struct *ws)
    {
    struct dm_cache_migration *mg = ws_to_mg(ws);
    struct cache *cache = mg.cache;
    struct bio *bio = mg.overwrite_bio;
    struct per_bio_data *pb = get_per_bio_data(bio);
    if (mg.k.input) {
    invalidate_complete(mg, false);
    return;
    }
    init_continuation(&mg.k, invalidate_completed);
    remap_to_origin_clear_discard(cache, bio, mg.invalidate_oblock);
    dm_hook_bio(&pb.hook_info, bio, overwrite_endio, mg);
    dm_submit_bio_remap(bio, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn invalidate_remove(ws: *mut work_struct) {
    static void invalidate_remove(struct work_struct *ws)
    {
    int r;
    struct dm_cache_migration *mg = ws_to_mg(ws);
    struct cache *cache = mg.cache;
    r = invalidate_cblock(cache, mg.invalidate_cblock);
    if (r) {
    invalidate_complete(mg, false);
    return;
    }
    init_continuation(&mg.k, invalidate_committed);
    continue_after_commit(&cache.committer, &mg.k);
    schedule_commit(&cache.committer);
    }
#[no_mangle]
unsafe extern "C" fn invalidate_lock(mg: *mut dm_cache_migration) -> c_int {
    static int invalidate_lock(struct dm_cache_migration *mg)
    {
    int r;
    struct dm_cell_key_v2 key;
    struct cache *cache = mg.cache;
    struct dm_bio_prison_cell_v2 *prealloc;
    prealloc = alloc_prison_cell(cache);
    build_key(mg.invalidate_oblock, oblock_succ(mg.invalidate_oblock), &key);
    r = dm_cell_lock_v2(cache.prison, &key,
    READ_WRITE_LOCK_LEVEL, prealloc, &mg.cell);
    if (r < 0) {
    free_prison_cell(cache, prealloc);
// Defer the bio for retrying the cell lock
    if (mg.overwrite_bio) {
    struct bio *bio = mg.overwrite_bio;
    mg.overwrite_bio = core::ptr::null_mut();
    defer_bio(cache, bio);
    }
    invalidate_complete(mg, false);
    return r;
    }
    if (mg.cell != prealloc)
    free_prison_cell(cache, prealloc);
    if (r)
    quiesce(mg, invalidate_remove);
    else {
//
// We can't call invalidate_remove() directly here because we
// might still be in request context.
//
    init_continuation(&mg.k, invalidate_remove);
    queue_work(cache.wq, &mg.k.ws);
    }
    return 0;
    }
    static int invalidate_start(struct cache *cache, dm_cblock_t cblock,
    dm_oblock_t oblock, struct bio *bio)
    {
    struct dm_cache_migration *mg;
    if (!background_work_begin(cache))
    return -EPERM;
    mg = alloc_migration(cache);
    mg.overwrite_bio = bio;
    mg.invalidate_cblock = cblock;
    mg.invalidate_oblock = oblock;
    return invalidate_lock(mg);
    }
//
// --------------------------------------------------------------
// bio processing
// --------------------------------------------------------------
//
    enum busy {
    IDLE,
    BUSY
    };
#[no_mangle]
unsafe extern "C" fn spare_migration_bandwidth(cache: *mut cache) -> enum busy {
    static enum busy spare_migration_bandwidth(struct cache *cache)
    {
    let mut idle: bool = dm_iot_idle_for(&cache.tracker, HZ);
    sector_t current_volume = (atomic_read(&cache.nr_io_migrations) + 1) *
    cache.sectors_per_block;
    if (idle && current_volume <= cache.migration_threshold)
    return IDLE;
    else
    return BUSY;
    }
#[no_mangle]
unsafe extern "C" fn inc_hit_counter(cache: *mut cache, bio: *mut bio) {
    static void inc_hit_counter(struct cache *cache, struct bio *bio)
    {
    atomic_inc(bio_data_dir(bio) == READ ?
    &cache.stats.read_hit : &cache.stats.write_hit);
    }
#[no_mangle]
unsafe extern "C" fn inc_miss_counter(cache: *mut cache, bio: *mut bio) {
    static void inc_miss_counter(struct cache *cache, struct bio *bio)
    {
    atomic_inc(bio_data_dir(bio) == READ ?
    &cache.stats.read_miss : &cache.stats.write_miss);
    }
// ----------------------------------------------------------------
    static int map_bio(struct cache *cache, struct bio *bio, dm_oblock_t block,
    bool *commit_needed)
    {
    int r, data_dir;
    bool rb, background_queued;
    dm_cblock_t cblock;
// commit_needed = false;
    rb = bio_detain_shared(cache, block, bio);
    if (!rb) {
//
// An exclusive lock is held for this block, so we have to
// wait.  We set the commit_needed flag so the current
// transaction will be committed asap, allowing this lock
// to be dropped.
//
// commit_needed = true;
    return DM_MAPIO_SUBMITTED;
    }
    data_dir = bio_data_dir(bio);
    if (optimisable_bio(cache, bio, block)) {
    struct policy_work *op = core::ptr::null_mut();
    r = policy_lookup_with_work(cache.policy, block, &cblock, data_dir, true, &op);
    if (unlikely(r && r != -ENOENT)) {
    DMERR_LIMIT("%s: policy_lookup_with_work() failed with r = %d",
    cache_device_name(cache), r);
    bio_io_error(bio);
    return DM_MAPIO_SUBMITTED;
    }
    if (r == -ENOENT && op) {
    bio_drop_shared_lock(cache, bio);
    BUG_ON(op.op != POLICY_PROMOTE);
    mg_start(cache, op, bio);
    return DM_MAPIO_SUBMITTED;
    }
    } else {
    r = policy_lookup(cache.policy, block, &cblock, data_dir, false, &background_queued);
    if (unlikely(r && r != -ENOENT)) {
    DMERR_LIMIT("%s: policy_lookup() failed with r = %d",
    cache_device_name(cache), r);
    bio_io_error(bio);
    return DM_MAPIO_SUBMITTED;
    }
    if (background_queued)
    wake_migration_worker(cache);
    }
    if (r == -ENOENT) {
    struct per_bio_data *pb = get_per_bio_data(bio);
//
// Miss.
//
    inc_miss_counter(cache, bio);
    if (pb.req_nr == 0) {
    accounted_begin(cache, bio);
    remap_to_origin_clear_discard(cache, bio, block);
    } else {
//
// This is a duplicate writethrough io that is no
// longer needed because the block has been demoted.
//
    bio_endio(bio);
    return DM_MAPIO_SUBMITTED;
    }
    } else {
//
// Hit.
//
    inc_hit_counter(cache, bio);
//
// Passthrough always maps to the origin, invalidating any
// cache blocks that are written to.
//
    if (passthrough_mode(cache)) {
    if (bio_data_dir(bio) == WRITE) {
    bio_drop_shared_lock(cache, bio);
    invalidate_start(cache, cblock, block, bio);
    return DM_MAPIO_SUBMITTED;
    } else
    remap_to_origin_clear_discard(cache, bio, block);
    } else {
    if (bio_data_dir(bio) == WRITE && writethrough_mode(cache) &&
    !is_dirty(cache, cblock)) {
    remap_to_origin_and_cache(cache, bio, block, cblock);
    accounted_begin(cache, bio);
    } else
    remap_to_cache_dirty(cache, bio, block, cblock);
    }
    }
//
// dm core turns FUA requests into a separate payload and FLUSH req.
//
    if (bio.bi_opf & REQ_FUA) {
//
// issue_after_commit will call accounted_begin a second time.  So
// we call accounted_complete() to avoid double accounting.
//
    accounted_complete(cache, bio);
    issue_after_commit(&cache.committer, bio);
// commit_needed = true;
    return DM_MAPIO_SUBMITTED;
    }
    return DM_MAPIO_REMAPPED;
    }
#[no_mangle]
unsafe extern "C" fn process_bio(cache: *mut cache, bio: *mut bio) -> bool {
    static bool process_bio(struct cache *cache, struct bio *bio)
    {
    bool commit_needed;
    if (map_bio(cache, bio, get_bio_block(cache, bio), &commit_needed) == DM_MAPIO_REMAPPED)
    dm_submit_bio_remap(bio, core::ptr::null_mut());
    return commit_needed;
    }
//
// A non-zero return indicates read_only or fail_io mode.
//
#[no_mangle]
unsafe extern "C" fn commit(cache: *mut cache, clean_shutdown: bool) -> c_int {
    static int commit(struct cache *cache, bool clean_shutdown)
    {
    int r;
    if (get_cache_mode(cache) >= CM_READ_ONLY)
    return -EINVAL;
    atomic_inc(&cache.stats.commit_count);
    r = dm_cache_commit(cache.cmd, clean_shutdown);
    if (r)
    metadata_operation_failed(cache, "dm_cache_commit", r);
    return r;
    }
//
// Used by the batcher.
//
#[no_mangle]
unsafe extern "C" fn commit_op(context: *mut c_void) -> blk_status_t {
    static blk_status_t commit_op(void *context)
    {
    struct cache *cache = context;
    if (dm_cache_changed_this_transaction(cache.cmd))
    return errno_to_blk_status(commit(cache, false));
    return 0;
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn process_flush_bio(cache: *mut cache, bio: *mut bio) -> bool {
    static bool process_flush_bio(struct cache *cache, struct bio *bio)
    {
    struct per_bio_data *pb = get_per_bio_data(bio);
    if (!pb.req_nr)
    remap_to_origin(cache, bio);
    else
    remap_to_cache(cache, bio, 0);
    issue_after_commit(&cache.committer, bio);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn process_discard_bio(cache: *mut cache, bio: *mut bio) -> bool {
    static bool process_discard_bio(struct cache *cache, struct bio *bio)
    {
    dm_dblock_t b, e;
//
// FIXME: do we need to lock the region?  Or can we just assume the
// user wont be so foolish as to issue discard concurrently with
// other IO?
//
    calc_discard_block_range(cache, bio, &b, &e);
    while (b != e) {
    set_discard(cache, b);
    b = to_dblock(from_dblock(b) + 1);
    }
    if (cache.features.discard_passdown) {
    remap_to_origin(cache, bio);
    dm_submit_bio_remap(bio, core::ptr::null_mut());
    } else
    bio_endio(bio);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn process_deferred_bios(ws: *mut work_struct) {
    static void process_deferred_bios(struct work_struct *ws)
    {
    struct cache *cache = container_of(ws, struct cache, deferred_bio_worker);
    let mut commit_needed: bool = false;
    struct bio_list bios;
    struct bio *bio;
    bio_list_init(&bios);
    spin_lock_irq(&cache.lock);
    bio_list_merge_init(&bios, &cache.deferred_bios);
    spin_unlock_irq(&cache.lock);
    while ((bio = bio_list_pop(&bios))) {
    if (bio.bi_opf & REQ_PREFLUSH)
    commit_needed = process_flush_bio(cache, bio) || commit_needed;
#[no_mangle]
pub unsafe extern "C" fn if(REQ_OP_DISCARD: bio_op(bio) ==) -> else {
    else if (bio_op(bio) == REQ_OP_DISCARD)
    commit_needed = process_discard_bio(cache, bio) || commit_needed;
    else
    commit_needed = process_bio(cache, bio) || commit_needed;
    cond_resched();
    }
    if (commit_needed)
    schedule_commit(&cache.committer);
    }
//
// --------------------------------------------------------------
// Main worker loop
// --------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn requeue_deferred_bios(cache: *mut cache) {
    static void requeue_deferred_bios(struct cache *cache)
    {
    struct bio *bio;
    struct bio_list bios;
    bio_list_init(&bios);
    bio_list_merge_init(&bios, &cache.deferred_bios);
    while ((bio = bio_list_pop(&bios))) {
    bio.bi_status = BLK_STS_DM_REQUEUE;
    bio_endio(bio);
    cond_resched();
    }
    }
//
// We want to commit periodically so that not too much
// unwritten metadata builds up.
//
#[no_mangle]
unsafe extern "C" fn do_waker(ws: *mut work_struct) {
    static void do_waker(struct work_struct *ws)
    {
    struct cache *cache = container_of(to_delayed_work(ws), struct cache, waker);
    policy_tick(cache.policy, true);
    wake_migration_worker(cache);
    schedule_commit(&cache.committer);
    queue_delayed_work(cache.wq, &cache.waker, COMMIT_PERIOD);
    }
#[no_mangle]
unsafe extern "C" fn check_migrations(ws: *mut work_struct) {
    static void check_migrations(struct work_struct *ws)
    {
    int r;
    struct policy_work *op;
    struct cache *cache = container_of(ws, struct cache, migration_worker);
    enum busy b;
    for (;;) {
    b = spare_migration_bandwidth(cache);
    r = policy_get_background_work(cache.policy, b == IDLE, &op);
    if (r == -ENODATA)
    break;
    if (r) {
    DMERR_LIMIT("%s: policy_background_work failed",
    cache_device_name(cache));
    break;
    }
    r = mg_start(cache, op, core::ptr::null_mut());
    if (r)
    break;
    cond_resched();
    }
    }
//
// --------------------------------------------------------------
// Target methods
// --------------------------------------------------------------
//
// This function gets called on the error paths of the constructor, so we
// have to cope with a partially initialised struct.
//
#[no_mangle]
unsafe extern "C" fn __destroy(cache: *mut cache) {
    static void __destroy(struct cache *cache)
    {
    mempool_exit(&cache.migration_pool);
    if (cache.prison)
    dm_bio_prison_destroy_v2(cache.prison);
    if (cache.wq)
    destroy_workqueue(cache.wq);
    if (cache.dirty_bitset)
    free_bitset(cache.dirty_bitset);
    if (cache.discard_bitset)
    free_bitset(cache.discard_bitset);
    if (cache.invalid_bitset)
    free_bitset(cache.invalid_bitset);
    if (cache.copier)
    dm_kcopyd_client_destroy(cache.copier);
    if (cache.cmd)
    dm_cache_metadata_close(cache.cmd);
    if (cache.metadata_dev)
    dm_put_device(cache.ti, cache.metadata_dev);
    if (cache.origin_dev)
    dm_put_device(cache.ti, cache.origin_dev);
    if (cache.cache_dev)
    dm_put_device(cache.ti, cache.cache_dev);
    if (cache.policy)
    dm_cache_policy_destroy(cache.policy);
    bioset_exit(&cache.bs);
    kfree(cache);
    }
#[no_mangle]
unsafe extern "C" fn destroy(cache: *mut cache) {
    static void destroy(struct cache *cache)
    {
    unsigned int i;
    cancel_delayed_work_sync(&cache.waker);
    for (i = 0; i < cache.nr_ctr_args ; i++)
    kfree(cache.ctr_args[i]);
    kfree(cache.ctr_args);
    __destroy(cache);
    }
#[no_mangle]
unsafe extern "C" fn cache_dtr(ti: *mut dm_target) {
    static void cache_dtr(struct dm_target *ti)
    {
    struct cache *cache = ti.private;
    destroy(cache);
    }
#[no_mangle]
unsafe extern "C" fn get_dev_size(dev: *mut dm_dev) -> sector_t {
    static sector_t get_dev_size(struct dm_dev *dev)
    {
    return bdev_nr_sectors(dev.bdev);
    }
// ----------------------------------------------------------------
//
// Construct a cache device mapping.
//
// cache <metadata dev> <cache dev> <origin dev> <block size>
// <#feature args> [<feature arg>]
// <policy> <#policy args> [<policy arg>]
//
// metadata dev    : fast device holding the persistent metadata
// cache dev	   : fast device holding cached data blocks
// origin dev	   : slow device holding original data blocks
// block size	   : cache unit size in sectors
//
// #feature args   : number of feature arguments passed
// feature args    : writethrough.  (The default is writeback.)
//
// policy	   : the replacement policy to use
// #policy args    : an even number of policy arguments corresponding
// to key/value pairs passed to the policy
// policy args	   : key/value pairs passed to the policy
// E.g. 'sequential_threshold 1024'
// See cache-policies.txt for details.
//
// Optional feature arguments are:
// writethrough  : write through caching that prohibits cache block
// content from being different from origin block content.
// Without this argument, the default behaviour is to write
// back cache block contents later for performance reasons,
// so they may differ from the corresponding origin blocks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_args {
    pub ti: *mut dm_target,
    pub metadata_dev: *mut dm_dev,
    pub cache_dev: *mut dm_dev,
    pub cache_sectors: sector_t,
    pub origin_dev: *mut dm_dev,
    pub block_size: u32,
    pub policy_name: *const c_char,
    pub policy_argc: c_int,
    pub policy_argv: *const c_char,
    pub features: cache_features,
}

#[no_mangle]
unsafe extern "C" fn destroy_cache_args(ca: *mut cache_args) {
    static void destroy_cache_args(struct cache_args *ca)
    {
    if (ca.metadata_dev)
    dm_put_device(ca.ti, ca.metadata_dev);
    if (ca.cache_dev)
    dm_put_device(ca.ti, ca.cache_dev);
    if (ca.origin_dev)
    dm_put_device(ca.ti, ca.origin_dev);
    kfree(ca);
    }
#[no_mangle]
unsafe extern "C" fn at_least_one_arg(as: *mut dm_arg_set, error: *mut c_char) -> bool {
    static bool at_least_one_arg(struct dm_arg_set *as, char **error)
    {
    if (!as.argc) {
// error = "Insufficient args";
    return false;
    }
    return true;
    }
    static int parse_metadata_dev(struct cache_args *ca, struct dm_arg_set *as,
    char **error)
    {
    int r;
    sector_t metadata_dev_size;
    if (!at_least_one_arg(as, error))
    return -EINVAL;
    r = dm_get_device(ca.ti, dm_shift_arg(as),
    BLK_OPEN_READ | BLK_OPEN_WRITE, &ca.metadata_dev);
    if (r) {
// error = "Error opening metadata device";
    return r;
    }
    metadata_dev_size = get_dev_size(ca.metadata_dev);
    if (metadata_dev_size > DM_CACHE_METADATA_MAX_SECTORS_WARNING)
    DMWARN("Metadata device %pg is larger than %u sectors: excess space will not be used.",
    ca.metadata_dev.bdev, THIN_METADATA_MAX_SECTORS);
    return 0;
    }
    static int parse_cache_dev(struct cache_args *ca, struct dm_arg_set *as,
    char **error)
    {
    int r;
    if (!at_least_one_arg(as, error))
    return -EINVAL;
    r = dm_get_device(ca.ti, dm_shift_arg(as),
    BLK_OPEN_READ | BLK_OPEN_WRITE, &ca.cache_dev);
    if (r) {
// error = "Error opening cache device";
    return r;
    }
    ca.cache_sectors = get_dev_size(ca.cache_dev);
    return 0;
    }
    static int parse_origin_dev(struct cache_args *ca, struct dm_arg_set *as,
    char **error)
    {
    int r;
    if (!at_least_one_arg(as, error))
    return -EINVAL;
    r = dm_get_device(ca.ti, dm_shift_arg(as),
    BLK_OPEN_READ | BLK_OPEN_WRITE, &ca.origin_dev);
    if (r) {
// error = "Error opening origin device";
    return r;
    }
    return 0;
    }
    static int parse_block_size(struct cache_args *ca, struct dm_arg_set *as,
    char **error)
    {
    unsigned long block_size;
    if (!at_least_one_arg(as, error))
    return -EINVAL;
    if (kstrtoul(dm_shift_arg(as), 10, &block_size) || !block_size ||
    block_size < DATA_DEV_BLOCK_SIZE_MIN_SECTORS ||
    block_size > DATA_DEV_BLOCK_SIZE_MAX_SECTORS ||
    block_size & (DATA_DEV_BLOCK_SIZE_MIN_SECTORS - 1)) {
// error = "Invalid data block size";
    return -EINVAL;
    }
    if (block_size > ca.cache_sectors) {
// error = "Data block size is larger than the cache device";
    return -EINVAL;
    }
    ca.block_size = block_size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_features(cf: *mut cache_features) {
    static void init_features(struct cache_features *cf)
    {
    cf.mode = CM_WRITE;
    cf.io_mode = CM_IO_WRITEBACK;
    cf.metadata_version = 1;
    cf.discard_passdown = true;
    }
    static int parse_features(struct cache_args *ca, struct dm_arg_set *as,
    char **error)
    {
    static const struct dm_arg _args[] = {
    {0, 3, "Invalid number of cache feature arguments"},
    };
    int r, mode_ctr = 0;
    unsigned int argc;
    const char *arg;
    struct cache_features *cf = &ca.features;
    init_features(cf);
    r = dm_read_arg_group(_args, as, &argc, error);
    if (r)
    return -EINVAL;
    while (argc--) {
    arg = dm_shift_arg(as);
    if (!strcasecmp(arg, "writeback")) {
    cf.io_mode = CM_IO_WRITEBACK;
    mode_ctr++;
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(arg, _arg: "writethrough")) -> else {
    cf.io_mode = CM_IO_WRITETHROUGH;
    mode_ctr++;
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(arg, _arg: "passthrough")) -> else {
    cf.io_mode = CM_IO_PASSTHROUGH;
    mode_ctr++;
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(arg, _arg: "metadata2")) -> else {
    else if (!strcasecmp(arg, "metadata2"))
    cf.metadata_version = 2;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(arg, _arg: "no_discard_passdown")) -> else {
    else if (!strcasecmp(arg, "no_discard_passdown"))
    cf.discard_passdown = false;
    else {
// error = "Unrecognised cache feature requested";
    return -EINVAL;
    }
    }
    if (mode_ctr > 1) {
// error = "Duplicate cache io_mode features requested";
    return -EINVAL;
    }
    return 0;
    }
    static int parse_policy(struct cache_args *ca, struct dm_arg_set *as,
    char **error)
    {
    static const struct dm_arg _args[] = {
    {0, 1024, "Invalid number of policy arguments"},
    };
    int r;
    if (!at_least_one_arg(as, error))
    return -EINVAL;
    ca.policy_name = dm_shift_arg(as);
    r = dm_read_arg_group(_args, as, &ca.policy_argc, error);
    if (r)
    return -EINVAL;
    ca.policy_argv = (const char **)as.argv;
    dm_consume_args(as, ca.policy_argc);
    return 0;
    }
    static int parse_cache_args(struct cache_args *ca, int argc, char **argv,
    char **error)
    {
    int r;
    struct dm_arg_set as;
    as.argc = argc;
    as.argv = argv;
    r = parse_metadata_dev(ca, &as, error);
    if (r)
    return r;
    r = parse_cache_dev(ca, &as, error);
    if (r)
    return r;
    r = parse_origin_dev(ca, &as, error);
    if (r)
    return r;
    r = parse_block_size(ca, &as, error);
    if (r)
    return r;
    r = parse_features(ca, &as, error);
    if (r)
    return r;
    r = parse_policy(ca, &as, error);
    if (r)
    return r;
    return 0;
    }
// ----------------------------------------------------------------
    static struct kmem_cache *migration_cache = core::ptr::null_mut();
pub const NOT_CORE_OPTION: c_int = 1;
#[no_mangle]
unsafe extern "C" fn process_config_option(cache: *mut cache, key: *const c_char, value: *const c_char) -> c_int {
    static int process_config_option(struct cache *cache, const char *key, const char *value)
    {
    unsigned long tmp;
    if (!strcasecmp(key, "migration_threshold")) {
    if (kstrtoul(value, 10, &tmp))
    return -EINVAL;
    cache.migration_threshold = tmp;
    return 0;
    }
    return NOT_CORE_OPTION;
    }
#[no_mangle]
unsafe extern "C" fn set_config_value(cache: *mut cache, key: *const c_char, value: *const c_char) -> c_int {
    static int set_config_value(struct cache *cache, const char *key, const char *value)
    {
    let mut r: c_int = process_config_option(cache, key, value);
    if (r == NOT_CORE_OPTION)
    r = policy_set_config_value(cache.policy, key, value);
    if (r)
    DMWARN("bad config value for %s: %s", key, value);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn set_config_values(cache: *mut cache, argc: c_int, argv: *const c_char) -> c_int {
    static int set_config_values(struct cache *cache, int argc, const char **argv)
    {
    let mut r: c_int = 0;
    if (argc & 1) {
    DMWARN("Odd number of policy arguments given but they should be <key> <value> pairs.");
    return -EINVAL;
    }
    while (argc) {
    r = set_config_value(cache, argv[0], argv[1]);
    if (r)
    break;
    argc -= 2;
    argv += 2;
    }
    return r;
    }
    static int create_cache_policy(struct cache *cache, struct cache_args *ca,
    char **error)
    {
    struct dm_cache_policy *p = dm_cache_policy_create(ca.policy_name,
    cache.cache_size,
    cache.origin_sectors,
    cache.sectors_per_block);
    if (IS_ERR(p)) {
// error = "Error creating cache's policy";
    return PTR_ERR(p);
    }
    cache.policy = p;
    BUG_ON(!cache.policy);
    return 0;
    }
//
// We want the discard block size to be at least the size of the cache
// block size and have no more than 2^14 discard blocks across the origin.
//

    static bool too_many_discard_blocks(sector_t discard_block_size,
    sector_t origin_size)
    {
    (void) sector_div(origin_size, discard_block_size);
    return origin_size > MAX_DISCARD_BLOCKS;
    }
    static sector_t calculate_discard_block_size(sector_t cache_block_size,
    sector_t origin_size)
    {
    let mut discard_block_size: sector_t = cache_block_size;
    if (origin_size)
    while (too_many_discard_blocks(discard_block_size, origin_size))
    discard_block_size *= 2;
    return discard_block_size;
    }
#[no_mangle]
unsafe extern "C" fn set_cache_size(cache: *mut cache, size: dm_cblock_t) {
    static void set_cache_size(struct cache *cache, dm_cblock_t size)
    {
    let mut nr_blocks: dm_block_t = from_cblock(size);
    if (nr_blocks > (1 << 20) && cache.cache_size != size)
    DMWARN_LIMIT("You have created a cache device with a lot of individual cache blocks (%llu)\n"
    "All these mappings can consume a lot of kernel memory, and take some time to read/write.\n"
    "Please consider increasing the cache block size to reduce the overall cache block count.",
    (unsigned long long) nr_blocks);
    cache.cache_size = size;
    }
pub const DEFAULT_MIGRATION_THRESHOLD: c_int = 2048;
#[no_mangle]
unsafe extern "C" fn cache_create(ca: *mut cache_args, result: *mut cache) -> c_int {
    static int cache_create(struct cache_args *ca, struct cache **result)
    {
    let mut r: c_int = 0;
    char **error = &ca.ti.error;
    struct cache *cache;
    struct dm_target *ti = ca.ti;
    dm_block_t origin_blocks;
    struct dm_cache_metadata *cmd;
    let mut may_format: bool = ca.features.mode == CM_WRITE;
    cache = kzalloc_obj(*cache);
    if (!cache)
    return -ENOMEM;
    cache.ti = ca.ti;
    ti.private = cache;
    ti.accounts_remapped_io = true;
    ti.num_flush_bios = 2;
    ti.flush_supported = true;
    ti.num_discard_bios = 1;
    ti.discards_supported = true;
    ti.per_io_data_size = sizeof(struct per_bio_data);
    cache.features = ca.features;
    if (writethrough_mode(cache)) {
// Create bioset for writethrough bios issued to origin
    r = bioset_init(&cache.bs, BIO_POOL_SIZE, 0, 0);
    if (r)
    goto bad;
    }
    cache.metadata_dev = ca.metadata_dev;
    cache.origin_dev = ca.origin_dev;
    cache.cache_dev = ca.cache_dev;
    ca.metadata_dev = ca.origin_dev = ca.cache_dev = core::ptr::null_mut();
    origin_blocks = cache.origin_sectors = ti.len;
    origin_blocks = block_div(origin_blocks, ca.block_size);
    cache.origin_blocks = to_oblock(origin_blocks);
    cache.sectors_per_block = ca.block_size;
    if (dm_set_target_max_io_len(ti, cache.sectors_per_block)) {
    r = -EINVAL;
    goto bad;
    }
    if (ca.block_size & (ca.block_size - 1)) {
    let mut cache_size: dm_block_t = ca.cache_sectors;
    cache.sectors_per_block_shift = -1;
    cache_size = block_div(cache_size, ca.block_size);
    set_cache_size(cache, to_cblock(cache_size));
    } else {
    cache.sectors_per_block_shift = __ffs(ca.block_size);
    set_cache_size(cache, to_cblock(ca.cache_sectors >> cache.sectors_per_block_shift));
    }
    r = create_cache_policy(cache, ca, error);
    if (r)
    goto bad;
    cache.policy_nr_args = ca.policy_argc;
    cache.migration_threshold = DEFAULT_MIGRATION_THRESHOLD;
    r = set_config_values(cache, ca.policy_argc, ca.policy_argv);
    if (r) {
// error = "Error setting cache policy's config values";
    goto bad;
    }
    cmd = dm_cache_metadata_open(cache.metadata_dev.bdev,
    ca.block_size, may_format,
    dm_cache_policy_get_hint_size(cache.policy),
    ca.features.metadata_version);
    if (IS_ERR(cmd)) {
// error = "Error creating metadata object";
    r = PTR_ERR(cmd);
    goto bad;
    }
    cache.cmd = cmd;
    set_cache_mode(cache, CM_WRITE);
    if (get_cache_mode(cache) != CM_WRITE) {
// error = "Unable to get write access to metadata, please check/repair metadata.";
    r = -EINVAL;
    goto bad;
    }
    if (passthrough_mode(cache))
    policy_allow_migrations(cache.policy, false);
    spin_lock_init(&cache.lock);
    bio_list_init(&cache.deferred_bios);
    atomic_set(&cache.nr_io_migrations, 0);
    r = -ENOMEM;
    atomic_set(&cache.nr_dirty, 0);
    cache.dirty_bitset = alloc_bitset(from_cblock(cache.cache_size));
    if (!cache.dirty_bitset) {
// error = "could not allocate dirty bitset";
    goto bad;
    }
    clear_bitset(cache.dirty_bitset, from_cblock(cache.cache_size));
    cache.discard_block_size =
    calculate_discard_block_size(cache.sectors_per_block,
    cache.origin_sectors);
    cache.discard_nr_blocks = to_dblock(dm_sector_div_up(cache.origin_sectors,
    cache.discard_block_size));
    cache.discard_bitset = alloc_bitset(from_dblock(cache.discard_nr_blocks));
    if (!cache.discard_bitset) {
// error = "could not allocate discard bitset";
    goto bad;
    }
    clear_bitset(cache.discard_bitset, from_dblock(cache.discard_nr_blocks));
    cache.invalid_bitset = alloc_bitset(from_cblock(cache.cache_size));
    if (!cache.invalid_bitset) {
// error = "could not allocate bitset for invalid blocks";
    goto bad;
    }
    clear_bitset(cache.invalid_bitset, from_cblock(cache.cache_size));
    cache.copier = dm_kcopyd_client_create(&dm_kcopyd_throttle);
    if (IS_ERR(cache.copier)) {
// error = "could not create kcopyd client";
    r = PTR_ERR(cache.copier);
    goto bad;
    }
    cache.wq = alloc_workqueue("dm-" DM_MSG_PREFIX,
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if (!cache.wq) {
// error = "could not create workqueue for metadata object";
    goto bad;
    }
    INIT_WORK(&cache.deferred_bio_worker, process_deferred_bios);
    INIT_WORK(&cache.migration_worker, check_migrations);
    INIT_DELAYED_WORK(&cache.waker, do_waker);
    cache.prison = dm_bio_prison_create_v2(cache.wq);
    if (!cache.prison) {
// error = "could not create bio prison";
    goto bad;
    }
    r = mempool_init_slab_pool(&cache.migration_pool, MIGRATION_POOL_SIZE,
    migration_cache);
    if (r) {
// error = "Error creating cache's migration mempool";
    goto bad;
    }
    cache.need_tick_bio = true;
    cache.sized = false;
    cache.invalidate = false;
    cache.commit_requested = false;
    cache.loaded_mappings = false;
    cache.loaded_discards = false;
    load_stats(cache);
    atomic_set(&cache.stats.demotion, 0);
    atomic_set(&cache.stats.promotion, 0);
    atomic_set(&cache.stats.copies_avoided, 0);
    atomic_set(&cache.stats.cache_cell_clash, 0);
    atomic_set(&cache.stats.commit_count, 0);
    atomic_set(&cache.stats.discard_count, 0);
    spin_lock_init(&cache.invalidation_lock);
    INIT_LIST_HEAD(&cache.invalidation_requests);
    batcher_init(&cache.committer, commit_op, cache,
    issue_op, cache, cache.wq);
    dm_iot_init(&cache.tracker);
    init_waitqueue_head(&cache.background_work_wait);
    spin_lock_init(&cache.background_work_lock);
    cache.background_work_allowed = false;
    cache.background_work_nr = 0;
// result = cache;
    return 0;
    bad:
    __destroy(cache);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn copy_ctr_args(cache: *mut cache, argc: c_int, argv: *const c_char) -> c_int {
    static int copy_ctr_args(struct cache *cache, int argc, const char **argv)
    {
    unsigned int i;
    const char **copy;
    copy = kcalloc(argc, sizeof(*copy), GFP_KERNEL);
    if (!copy)
    return -ENOMEM;
    for (i = 0; i < argc; i++) {
    copy[i] = kstrdup(argv[i], GFP_KERNEL);
    if (!copy[i]) {
    while (i--)
    kfree(copy[i]);
    kfree(copy);
    return -ENOMEM;
    }
    }
    cache.nr_ctr_args = argc;
    cache.ctr_args = copy;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cache_ctr(ti: *mut dm_target, argc: c_uint, argv: *mut c_char) -> c_int {
    static int cache_ctr(struct dm_target *ti, unsigned int argc, char **argv)
    {
    let mut r: c_int = -EINVAL;
    struct cache_args *ca;
    struct cache *cache = core::ptr::null_mut();
    ca = kzalloc_obj(*ca);
    if (!ca) {
    ti.error = "Error allocating memory for cache";
    return -ENOMEM;
    }
    ca.ti = ti;
    r = parse_cache_args(ca, argc, argv, &ti.error);
    if (r)
    goto out;
    r = cache_create(ca, &cache);
    if (r)
    goto out;
    r = copy_ctr_args(cache, argc - 3, (const char **)argv + 3);
    if (r) {
    __destroy(cache);
    goto out;
    }
    ti.private = cache;
    out:
    destroy_cache_args(ca);
    return r;
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn cache_map(ti: *mut dm_target, bio: *mut bio) -> c_int {
    static int cache_map(struct dm_target *ti, struct bio *bio)
    {
    struct cache *cache = ti.private;
    int r;
    bool commit_needed;
    let mut block: dm_oblock_t = get_bio_block(cache, bio);
    init_per_bio_data(bio);
    if (unlikely(from_oblock(block) >= from_oblock(cache.origin_blocks))) {
//
// This can only occur if the io goes to a partial block at
// the end of the origin device.  We don't cache these.
// Just remap to the origin and carry on.
//
    remap_to_origin(cache, bio);
    accounted_begin(cache, bio);
    return DM_MAPIO_REMAPPED;
    }
    if (discard_or_flush(bio)) {
    defer_bio(cache, bio);
    return DM_MAPIO_SUBMITTED;
    }
    r = map_bio(cache, bio, block, &commit_needed);
    if (commit_needed)
    schedule_commit(&cache.committer);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn cache_end_io(ti: *mut dm_target, bio: *mut bio, error: *mut blk_status_t) -> c_int {
    static int cache_end_io(struct dm_target *ti, struct bio *bio, blk_status_t *error)
    {
    struct cache *cache = ti.private;
    unsigned long flags;
    struct per_bio_data *pb = get_per_bio_data(bio);
    if (pb.tick) {
    policy_tick(cache.policy, false);
    spin_lock_irqsave(&cache.lock, flags);
    cache.need_tick_bio = true;
    spin_unlock_irqrestore(&cache.lock, flags);
    }
    bio_drop_shared_lock(cache, bio);
    accounted_complete(cache, bio);
    return DM_ENDIO_DONE;
    }
#[no_mangle]
unsafe extern "C" fn write_dirty_bitset(cache: *mut cache) -> c_int {
    static int write_dirty_bitset(struct cache *cache)
    {
    int r;
    if (get_cache_mode(cache) >= CM_READ_ONLY)
    return -EINVAL;
    r = dm_cache_set_dirty_bits(cache.cmd, from_cblock(cache.cache_size), cache.dirty_bitset);
    if (r)
    metadata_operation_failed(cache, "dm_cache_set_dirty_bits", r);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn write_discard_bitset(cache: *mut cache) -> c_int {
    static int write_discard_bitset(struct cache *cache)
    {
    unsigned int i, r;
    if (get_cache_mode(cache) >= CM_READ_ONLY)
    return -EINVAL;
    r = dm_cache_discard_bitset_resize(cache.cmd, cache.discard_block_size,
    cache.discard_nr_blocks);
    if (r) {
    DMERR("%s: could not resize on-disk discard bitset", cache_device_name(cache));
    metadata_operation_failed(cache, "dm_cache_discard_bitset_resize", r);
    return r;
    }
    for (i = 0; i < from_dblock(cache.discard_nr_blocks); i++) {
    r = dm_cache_set_discard(cache.cmd, to_dblock(i),
    is_discarded(cache, to_dblock(i)));
    if (r) {
    metadata_operation_failed(cache, "dm_cache_set_discard", r);
    return r;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn write_hints(cache: *mut cache) -> c_int {
    static int write_hints(struct cache *cache)
    {
    int r;
    if (get_cache_mode(cache) >= CM_READ_ONLY)
    return -EINVAL;
    r = dm_cache_write_hints(cache.cmd, cache.policy);
    if (r) {
    metadata_operation_failed(cache, "dm_cache_write_hints", r);
    return r;
    }
    return 0;
    }
//
// returns true on success
//
#[no_mangle]
unsafe extern "C" fn sync_metadata(cache: *mut cache) -> bool {
    static bool sync_metadata(struct cache *cache)
    {
    int r1, r2, r3, r4;
    r1 = write_dirty_bitset(cache);
    if (r1)
    DMERR("%s: could not write dirty bitset", cache_device_name(cache));
    r2 = write_discard_bitset(cache);
    if (r2)
    DMERR("%s: could not write discard bitset", cache_device_name(cache));
    save_stats(cache);
    r3 = write_hints(cache);
    if (r3)
    DMERR("%s: could not write hints", cache_device_name(cache));
//
// If writing the above metadata failed, we still commit, but don't
// set the clean shutdown flag.  This will effectively force every
// dirty bit to be set on reload.
//
    r4 = commit(cache, !r1 && !r2 && !r3);
    if (r4)
    DMERR("%s: could not write cache metadata", cache_device_name(cache));
    return !r1 && !r2 && !r3 && !r4;
    }
#[no_mangle]
unsafe extern "C" fn cache_postsuspend(ti: *mut dm_target) {
    static void cache_postsuspend(struct dm_target *ti)
    {
    struct cache *cache = ti.private;
    prevent_background_work(cache);
    BUG_ON(atomic_read(&cache.nr_io_migrations));
    cancel_delayed_work_sync(&cache.waker);
    drain_workqueue(cache.wq);
    WARN_ON(cache.tracker.in_flight);
//
// If it's a flush suspend there won't be any deferred bios, so this
// call is harmless.
//
    requeue_deferred_bios(cache);
    if (get_cache_mode(cache) == CM_WRITE)
    (void) sync_metadata(cache);
    }
    static int load_mapping(void *context, dm_oblock_t oblock, dm_cblock_t cblock,
    bool dirty, uint32_t hint, bool hint_valid)
    {
    struct cache *cache = context;
    if (dirty) {
    if (passthrough_mode(cache)) {
    DMERR("%s: cannot enter passthrough mode unless all blocks are clean",
    cache_device_name(cache));
    return -EBUSY;
    }
    set_bit(from_cblock(cblock), cache.dirty_bitset);
    atomic_inc(&cache.nr_dirty);
    } else
    clear_bit(from_cblock(cblock), cache.dirty_bitset);
    return policy_load_mapping(cache.policy, oblock, cblock, dirty, hint, hint_valid);
    }
    static int load_filtered_mapping(void *context, dm_oblock_t oblock, dm_cblock_t cblock,
    bool dirty, uint32_t hint, bool hint_valid)
    {
    struct cache *cache = context;
    if (from_oblock(oblock) >= from_oblock(cache.origin_blocks)) {
    if (dirty) {
    DMERR("%s: unable to shrink origin; cache block %u is dirty",
    cache_device_name(cache), from_cblock(cblock));
    return -EFBIG;
    }
    set_bit(from_cblock(cblock), cache.invalid_bitset);
    return 0;
    }
    return load_mapping(context, oblock, cblock, dirty, hint, hint_valid);
    }
//
// The discard block size in the on disk metadata is not
// necessarily the same as we're currently using.  So we have to
// be careful to only set the discarded attribute if we know it
// covers a complete block of the new size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct discard_load_info {
    pub cache: *mut cache,
//
// These blocks are sized using the on disk dblock size, rather
// than the current one.
//
    pub block_size: dm_block_t,
    pub discard_end: dm_block_t discard_begin,,
}

    static void discard_load_info_init(struct cache *cache,
    struct discard_load_info *li)
    {
    li.cache = cache;
    li.discard_begin = li.discard_end = 0;
    }
#[no_mangle]
unsafe extern "C" fn set_discard_range(li: *mut discard_load_info) {
    static void set_discard_range(struct discard_load_info *li)
    {
    sector_t b, e;
    if (li.discard_begin == li.discard_end)
    return;
//
// Convert to sectors.
//
    b = li.discard_begin * li.block_size;
    e = li.discard_end * li.block_size;
//
// Then convert back to the current dblock size.
//
    b = dm_sector_div_up(b, li.cache.discard_block_size);
    sector_div(e, li.cache.discard_block_size);
//
// The origin may have shrunk, so we need to check we're still in
// bounds.
//
    if (e > from_dblock(li.cache.discard_nr_blocks))
    e = from_dblock(li.cache.discard_nr_blocks);
    for (; b < e; b++)
    set_discard(li.cache, to_dblock(b));
    }
    static int load_discard(void *context, sector_t discard_block_size,
    dm_dblock_t dblock, bool discard)
    {
    struct discard_load_info *li = context;
    li.block_size = discard_block_size;
    if (discard) {
    if (from_dblock(dblock) == li.discard_end)
//
// We're already in a discard range, just extend it.
//
    li.discard_end = li.discard_end + 1ULL;
    else {
//
// Emit the old range and start a new one.
//
    set_discard_range(li);
    li.discard_begin = from_dblock(dblock);
    li.discard_end = li.discard_begin + 1ULL;
    }
    } else {
    set_discard_range(li);
    li.discard_begin = li.discard_end = 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_cache_dev_size(cache: *mut cache) -> dm_cblock_t {
    static dm_cblock_t get_cache_dev_size(struct cache *cache)
    {
    let mut size: sector_t = get_dev_size(cache.cache_dev);
    (void) sector_div(size, cache.sectors_per_block);
    return to_cblock(size);
    }
#[no_mangle]
unsafe extern "C" fn can_resume(cache: *mut cache) -> bool {
    static bool can_resume(struct cache *cache)
    {
    bool clean_when_opened;
    int r;
//
// Disallow retrying the resume operation for devices that failed the
// first resume attempt, as the failure leaves the policy object partially
// initialized. Retrying could trigger BUG_ON when loading cache mappings
// into the incomplete policy object.
//
    if (cache.sized && !cache.loaded_mappings) {
    if (get_cache_mode(cache) != CM_WRITE)
    DMERR("%s: unable to resume a failed-loaded cache, please check metadata.",
    cache_device_name(cache));
    else
    DMERR("%s: unable to resume cache due to missing proper cache table reload",
    cache_device_name(cache));
    return false;
    }
    if (passthrough_mode(cache)) {
    r = dm_cache_metadata_clean_when_opened(cache.cmd, &clean_when_opened);
    if (r) {
    DMERR("%s: failed to query metadata flags", cache_device_name(cache));
    return false;
    }
    if (!clean_when_opened) {
    DMERR("%s: unable to resume into passthrough mode after unclean shutdown",
    cache_device_name(cache));
    return false;
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn can_resize(cache: *mut cache, new_size: dm_cblock_t) -> bool {
    static bool can_resize(struct cache *cache, dm_cblock_t new_size)
    {
    if (from_cblock(new_size) > from_cblock(cache.cache_size)) {
    DMERR("%s: unable to extend cache due to missing cache table reload",
    cache_device_name(cache));
    return false;
    }
//
// We can't drop a dirty block when shrinking the cache.
//
    if (cache.loaded_mappings) {
    new_size = to_cblock(find_next_bit(cache.dirty_bitset,
    from_cblock(cache.cache_size),
    from_cblock(new_size)));
    if (new_size != cache.cache_size) {
    DMERR("%s: unable to shrink cache; cache block %llu is dirty",
    cache_device_name(cache),
    (unsigned long long) from_cblock(new_size));
    return false;
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn resize_cache_dev(cache: *mut cache, new_size: dm_cblock_t) -> c_int {
    static int resize_cache_dev(struct cache *cache, dm_cblock_t new_size)
    {
    int r;
    r = dm_cache_resize(cache.cmd, new_size);
    if (r) {
    DMERR("%s: could not resize cache metadata", cache_device_name(cache));
    metadata_operation_failed(cache, "dm_cache_resize", r);
    return r;
    }
    set_cache_size(cache, new_size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn truncate_oblocks(cache: *mut cache) -> c_int {
    static int truncate_oblocks(struct cache *cache)
    {
    let mut nr_blocks: u32 = from_cblock(cache.cache_size);
    uint32_t i;
    int r;
    for_each_set_bit(i, cache.invalid_bitset, nr_blocks) {
    r = dm_cache_remove_mapping(cache.cmd, to_cblock(i));
    if (r) {
    DMERR_LIMIT("%s: invalidation failed; couldn't update on disk metadata",
    cache_device_name(cache));
    return r;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cache_preresume(ti: *mut dm_target) -> c_int {
    static int cache_preresume(struct dm_target *ti)
    {
    let mut r: c_int = 0;
    struct cache *cache = ti.private;
    let mut csize: dm_cblock_t = get_cache_dev_size(cache);
    if (!can_resume(cache))
    return -EINVAL;
//
// Check to see if the cache has resized.
//
    if (!cache.sized || csize != cache.cache_size) {
    if (!can_resize(cache, csize))
    return -EINVAL;
    r = resize_cache_dev(cache, csize);
    if (r)
    return r;
    cache.sized = true;
    }
    if (!cache.loaded_mappings) {
//
// The fast device could have been resized since the last
// failed preresume attempt.  To be safe we start by a blank
// bitset for cache blocks.
//
    clear_bitset(cache.invalid_bitset, from_cblock(cache.cache_size));
    r = dm_cache_load_mappings(cache.cmd, cache.policy,
    load_filtered_mapping, cache);
    if (r) {
    DMERR("%s: could not load cache mappings", cache_device_name(cache));
    if (r != -EFBIG && r != -EBUSY)
    metadata_operation_failed(cache, "dm_cache_load_mappings", r);
    return r;
    }
    r = truncate_oblocks(cache);
    if (r) {
    metadata_operation_failed(cache, "dm_cache_remove_mapping", r);
    return r;
    }
    cache.loaded_mappings = true;
    }
    if (!cache.loaded_discards) {
    struct discard_load_info li;
//
// The discard bitset could have been resized, or the
// discard block size changed.  To be safe we start by
// setting every dblock to not discarded.
//
    clear_bitset(cache.discard_bitset, from_dblock(cache.discard_nr_blocks));
    discard_load_info_init(cache, &li);
    r = dm_cache_load_discards(cache.cmd, load_discard, &li);
    if (r) {
    DMERR("%s: could not load origin discards", cache_device_name(cache));
    metadata_operation_failed(cache, "dm_cache_load_discards", r);
    return r;
    }
    set_discard_range(&li);
    cache.loaded_discards = true;
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn cache_resume(ti: *mut dm_target) {
    static void cache_resume(struct dm_target *ti)
    {
    struct cache *cache = ti.private;
    cache.need_tick_bio = true;
    allow_background_work(cache);
    do_waker(&cache.waker.work);
    }
    static void emit_flags(struct cache *cache, char *result,
    unsigned int maxlen, ssize_t *sz_ptr)
    {
    let mut sz: isize = *sz_ptr;
    struct cache_features *cf = &cache.features;
    let mut count: c_uint = (cf.metadata_version == 2) + !cf.discard_passdown + 1;
    DMEMIT("%u ", count);
    if (cf.metadata_version == 2)
    DMEMIT("metadata2 ");
    if (writethrough_mode(cache))
    DMEMIT("writethrough ");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: passthrough_mode(cache)) -> else {
    else if (passthrough_mode(cache))
    DMEMIT("passthrough ");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: writeback_mode(cache)) -> else {
    else if (writeback_mode(cache))
    DMEMIT("writeback ");
    else {
    DMEMIT("unknown ");
    DMERR("%s: internal error: unknown io mode: %d",
    cache_device_name(cache), (int) cf.io_mode);
    }
    if (!cf.discard_passdown)
    DMEMIT("no_discard_passdown ");
// sz_ptr = sz;
    }
//
// Status format:
//
// <metadata block size> <#used metadata blocks>/<#total metadata blocks>
// <cache block size> <#used cache blocks>/<#total cache blocks>
// <#read hits> <#read misses> <#write hits> <#write misses>
// <#demotions> <#promotions> <#dirty>
// <#features> <features>
// <#core args> <core args>
// <policy name> <#policy args> <policy args>* <cache metadata mode> <needs_check>
//
    static void cache_status(struct dm_target *ti, status_type_t type,
    unsigned int status_flags, char *result, unsigned int maxlen)
    {
    let mut r: c_int = 0;
    unsigned int i;
    let mut sz: isize = 0;
    let mut nr_free_blocks_metadata: dm_block_t = 0;
    let mut nr_blocks_metadata: dm_block_t = 0;
    char buf[BDEVNAME_SIZE];
    struct cache *cache = ti.private;
    dm_cblock_t residency;
    bool needs_check;
    switch (type) {
    case STATUSTYPE_INFO:
    if (get_cache_mode(cache) == CM_FAIL) {
    DMEMIT("Fail");
    break;
    }
// Commit to ensure statistics aren't out-of-date
    if (!(status_flags & DM_STATUS_NOFLUSH_FLAG) && !dm_suspended(ti))
    (void) commit(cache, false);
    r = dm_cache_get_free_metadata_block_count(cache.cmd, &nr_free_blocks_metadata);
    if (r) {
    DMERR("%s: dm_cache_get_free_metadata_block_count returned %d",
    cache_device_name(cache), r);
    goto err;
    }
    r = dm_cache_get_metadata_dev_size(cache.cmd, &nr_blocks_metadata);
    if (r) {
    DMERR("%s: dm_cache_get_metadata_dev_size returned %d",
    cache_device_name(cache), r);
    goto err;
    }
    residency = policy_residency(cache.policy);
    DMEMIT("%u %llu/%llu %llu %llu/%llu %u %u %u %u %u %u %lu ",
    (unsigned int)DM_CACHE_METADATA_BLOCK_SIZE,
    (unsigned long long)(nr_blocks_metadata - nr_free_blocks_metadata),
    (unsigned long long)nr_blocks_metadata,
    (unsigned long long)cache.sectors_per_block,
    (unsigned long long) from_cblock(residency),
    (unsigned long long) from_cblock(cache.cache_size),
    (unsigned int) atomic_read(&cache.stats.read_hit),
    (unsigned int) atomic_read(&cache.stats.read_miss),
    (unsigned int) atomic_read(&cache.stats.write_hit),
    (unsigned int) atomic_read(&cache.stats.write_miss),
    (unsigned int) atomic_read(&cache.stats.demotion),
    (unsigned int) atomic_read(&cache.stats.promotion),
    (unsigned long) atomic_read(&cache.nr_dirty));
    emit_flags(cache, result, maxlen, &sz);
    DMEMIT("2 migration_threshold %llu ", (unsigned long long) cache.migration_threshold);
    DMEMIT("%s ", dm_cache_policy_get_name(cache.policy));
    if (sz < maxlen) {
    r = policy_emit_config_values(cache.policy, result, maxlen, &sz);
    if (r)
    DMERR("%s: policy_emit_config_values returned %d",
    cache_device_name(cache), r);
    }
    if (get_cache_mode(cache) == CM_READ_ONLY)
    DMEMIT("ro ");
    else
    DMEMIT("rw ");
    r = dm_cache_metadata_needs_check(cache.cmd, &needs_check);
    if (r || needs_check)
    DMEMIT("needs_check ");
    else
    DMEMIT("- ");
    break;
    case STATUSTYPE_TABLE:
    format_dev_t(buf, cache.metadata_dev.bdev.bd_dev);
    DMEMIT("%s ", buf);
    format_dev_t(buf, cache.cache_dev.bdev.bd_dev);
    DMEMIT("%s ", buf);
    format_dev_t(buf, cache.origin_dev.bdev.bd_dev);
    DMEMIT("%s", buf);
    for (i = 0; i < cache.nr_ctr_args - 1; i++)
    DMEMIT(" %s", cache.ctr_args[i]);
    if (cache.nr_ctr_args)
    DMEMIT(" %s", cache.ctr_args[cache.nr_ctr_args - 1]);
    break;
    case STATUSTYPE_IMA:
    DMEMIT_TARGET_NAME_VERSION(ti.type);
    if (get_cache_mode(cache) == CM_FAIL)
    DMEMIT(",metadata_mode=fail");
#[no_mangle]
pub unsafe extern "C" fn if(CM_READ_ONLY: get_cache_mode(cache) ==) -> else {
    else if (get_cache_mode(cache) == CM_READ_ONLY)
    DMEMIT(",metadata_mode=ro");
    else
    DMEMIT(",metadata_mode=rw");
    format_dev_t(buf, cache.metadata_dev.bdev.bd_dev);
    DMEMIT(",cache_metadata_device=%s", buf);
    format_dev_t(buf, cache.cache_dev.bdev.bd_dev);
    DMEMIT(",cache_device=%s", buf);
    format_dev_t(buf, cache.origin_dev.bdev.bd_dev);
    DMEMIT(",cache_origin_device=%s", buf);
    DMEMIT(",writethrough=%c", writethrough_mode(cache) ? 'y' : 'n');
    DMEMIT(",writeback=%c", writeback_mode(cache) ? 'y' : 'n');
    DMEMIT(",passthrough=%c", passthrough_mode(cache) ? 'y' : 'n');
    DMEMIT(",metadata2=%c", cache.features.metadata_version == 2 ? 'y' : 'n');
    DMEMIT(",no_discard_passdown=%c", cache.features.discard_passdown ? 'n' : 'y');
    DMEMIT(";");
    break;
    }
    return;
    err:
    DMEMIT("Error");
    }
//
// Defines a range of cblocks, begin to (end - 1) are in the range.  end is
// the one-past-the-end value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cblock_range {
    pub begin: dm_cblock_t,
    pub end: dm_cblock_t,
}

#[no_mangle]
pub unsafe extern "C" fn cblock_succ(b: dm_cblock_t) -> dm_cblock_t {
    static inline dm_cblock_t cblock_succ(dm_cblock_t b)
    {
    return to_cblock(from_cblock(b) + 1);
    }
//
// A cache block range can take two forms:
//
// i) A single cblock, eg. '3456'
// ii) A begin and end cblock with a dash between, eg. 123-234
//
    static int parse_cblock_range(struct cache *cache, char *str,
    struct cblock_range *result)
    {
    char *blocknr = strsep(&str, "-");
    unsigned int b, e;
    int r;
    r = kstrtouint(blocknr, 10, &b);
    if (r)
    goto bad;
    result.begin = to_cblock(b);
    if (str) {
    blocknr = str;
    r = kstrtouint(blocknr, 10, &e);
    if (r)
    goto bad;
    result.end = to_cblock(e);
    } else {
    result.end = cblock_succ(result.begin);
    }
    return 0;
    bad:
    DMERR("%s: invalid cblock range '%s'", cache_device_name(cache), blocknr);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn validate_cblock_range(cache: *mut cache, range: *mut cblock_range) -> c_int {
    static int validate_cblock_range(struct cache *cache, struct cblock_range *range)
    {
    let mut b: u64 = from_cblock(range.begin);
    let mut e: u64 = from_cblock(range.end);
    let mut n: u64 = from_cblock(cache.cache_size);
    if (b >= n) {
    DMERR("%s: begin cblock out of range: %llu >= %llu",
    cache_device_name(cache), b, n);
    return -EINVAL;
    }
    if (e > n) {
    DMERR("%s: end cblock out of range: %llu > %llu",
    cache_device_name(cache), e, n);
    return -EINVAL;
    }
    if (b >= e) {
    DMERR("%s: invalid cblock range: %llu >= %llu",
    cache_device_name(cache), b, e);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn request_invalidation(cache: *mut cache, range: *mut cblock_range) -> c_int {
    static int request_invalidation(struct cache *cache, struct cblock_range *range)
    {
    let mut r: c_int = 0;
//
// We don't need to do any locking here because we know we're in
// passthrough mode.  There's is potential for a race between an
// invalidation triggered by an io and an invalidation message.  This
// is harmless, we must not worry if the policy call fails.
//
    while (range.begin != range.end) {
    r = invalidate_cblock(cache, range.begin);
    if (r)
    return r;
    range.begin = cblock_succ(range.begin);
    }
    cache.commit_requested = true;
    return r;
    }
    static int process_invalidate_cblocks_message(struct cache *cache, unsigned int count,
    char **cblock_ranges)
    {
    let mut r: c_int = 0;
    unsigned int i;
    struct cblock_range range;
    if (!passthrough_mode(cache)) {
    DMERR("%s: cache has to be in passthrough mode for invalidation",
    cache_device_name(cache));
    return -EPERM;
    }
    for (i = 0; i < count; i++) {
    r = parse_cblock_range(cache, cblock_ranges[i], &range);
    if (r)
    break;
    r = validate_cblock_range(cache, &range);
    if (r)
    break;
//
// Pass begin and end origin blocks to the worker and wake it.
//
    r = request_invalidation(cache, &range);
    if (r)
    break;
    }
    return r;
    }
//
// Supports
// "<key> <value>"
// and
// "invalidate_cblocks [(<begin>)|(<begin>-<end>)]
//
// The key migration_threshold is supported by the cache target core.
//
    static int cache_message(struct dm_target *ti, unsigned int argc, char **argv,
    char *result, unsigned int maxlen)
    {
    struct cache *cache = ti.private;
    if (!argc)
    return -EINVAL;
    if (get_cache_mode(cache) >= CM_READ_ONLY) {
    DMERR("%s: unable to service cache target messages in READ_ONLY or FAIL mode",
    cache_device_name(cache));
    return -EOPNOTSUPP;
    }
    if (!strcasecmp(argv[0], "invalidate_cblocks"))
    return process_invalidate_cblocks_message(cache, argc - 1, argv + 1);
    if (argc != 2)
    return -EINVAL;
    return set_config_value(cache, argv[0], argv[1]);
    }
    static int cache_iterate_devices(struct dm_target *ti,
    iterate_devices_callout_fn fn, void *data)
    {
    let mut r: c_int = 0;
    struct cache *cache = ti.private;
    r = fn(ti, cache.cache_dev, 0, get_dev_size(cache.cache_dev), data);
    if (!r)
    r = fn(ti, cache.origin_dev, 0, ti.len, data);
    return r;
    }
//
// If discard_passdown was enabled verify that the origin device
// supports discards.  Disable discard_passdown if not.
//
#[no_mangle]
unsafe extern "C" fn disable_passdown_if_not_supported(cache: *mut cache) {
    static void disable_passdown_if_not_supported(struct cache *cache)
    {
    struct block_device *origin_bdev = cache.origin_dev.bdev;
    struct queue_limits *origin_limits = bdev_limits(origin_bdev);
    const char *reason = core::ptr::null_mut();
    if (!cache.features.discard_passdown)
    return;
    if (!bdev_max_discard_sectors(origin_bdev))
    reason = "discard unsupported";
#[no_mangle]
pub unsafe extern "C" fn if(cache->sectors_per_block: origin_limits->max_discard_sectors <) -> else {
    else if (origin_limits.max_discard_sectors < cache.sectors_per_block)
    reason = "max discard sectors smaller than a block";
    if (reason) {
    DMWARN("Origin device (%pg) %s: Disabling discard passdown.",
    origin_bdev, reason);
    cache.features.discard_passdown = false;
    }
    }
#[no_mangle]
unsafe extern "C" fn set_discard_limits(cache: *mut cache, limits: *mut queue_limits) {
    static void set_discard_limits(struct cache *cache, struct queue_limits *limits)
    {
    struct block_device *origin_bdev = cache.origin_dev.bdev;
    struct queue_limits *origin_limits = bdev_limits(origin_bdev);
    if (!cache.features.discard_passdown) {
// No passdown is done so setting own virtual limits
    limits.max_hw_discard_sectors = min_t(sector_t, cache.discard_block_size * 1024,
    cache.origin_sectors);
    limits.discard_granularity = cache.discard_block_size << SECTOR_SHIFT;
    return;
    }
//
// cache_iterate_devices() is stacking both origin and fast device limits
// but discards aren't passed to fast device, so inherit origin's limits.
//
    limits.max_hw_discard_sectors = origin_limits.max_hw_discard_sectors;
    limits.discard_granularity = origin_limits.discard_granularity;
    limits.discard_alignment = origin_limits.discard_alignment;
    }
#[no_mangle]
unsafe extern "C" fn cache_io_hints(ti: *mut dm_target, limits: *mut queue_limits) {
    static void cache_io_hints(struct dm_target *ti, struct queue_limits *limits)
    {
    struct cache *cache = ti.private;
    let mut io_opt_sectors: u64 = limits.io_opt >> SECTOR_SHIFT;
//
// If the system-determined stacked limits are compatible with the
// cache's blocksize (io_opt is a factor) do not override them.
//
    if (io_opt_sectors < cache.sectors_per_block ||
    do_div(io_opt_sectors, cache.sectors_per_block)) {
    limits.io_min = cache.sectors_per_block << SECTOR_SHIFT;
    limits.io_opt = cache.sectors_per_block << SECTOR_SHIFT;
    }
    disable_passdown_if_not_supported(cache);
    set_discard_limits(cache, limits);
    }
// ----------------------------------------------------------------
    static struct target_type cache_target = {
    .name = "cache",
    .version = {2, 4, 0},
    .module = THIS_MODULE,
    .ctr = cache_ctr,
    .dtr = cache_dtr,
    .map = cache_map,
    .end_io = cache_end_io,
    .postsuspend = cache_postsuspend,
    .preresume = cache_preresume,
    .resume = cache_resume,
    .status = cache_status,
    .message = cache_message,
    .iterate_devices = cache_iterate_devices,
    .io_hints = cache_io_hints,
    };
#[no_mangle]
unsafe extern "C" fn dm_cache_init() -> int __init {
    static int __init dm_cache_init(void)
    {
    int r;
    migration_cache = KMEM_CACHE(dm_cache_migration, 0);
    if (!migration_cache) {
    r = -ENOMEM;
    goto err;
    }
    btracker_work_cache = kmem_cache_create("dm_cache_bt_work",
    sizeof(struct bt_work), __alignof__(struct bt_work), 0, core::ptr::null_mut());
    if (!btracker_work_cache) {
    r = -ENOMEM;
    goto err;
    }
    r = dm_register_target(&cache_target);
    if (r) {
    goto err;
    }
    return 0;
    err:
    kmem_cache_destroy(migration_cache);
    kmem_cache_destroy(btracker_work_cache);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn dm_cache_exit() -> void __exit {
    static void __exit dm_cache_exit(void)
    {
    dm_unregister_target(&cache_target);
    kmem_cache_destroy(migration_cache);
    kmem_cache_destroy(btracker_work_cache);
    }
    module_init(dm_cache_init);
    module_exit(dm_cache_exit);
    MODULE_DESCRIPTION(DM_NAME " cache target");
    MODULE_AUTHOR("Joe Thornber <ejt@redhat.com>");
    MODULE_LICENSE("GPL");
