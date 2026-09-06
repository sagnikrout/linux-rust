//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-snap.c
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
// Copyright (C) 2001-2002 Sistina Software (UK) Limited.
//
// This file is released under the GPL.
//

    static const char dm_snapshot_merge_target_name[] = "snapshot-merge";

    ((ti).type.name == dm_snapshot_merge_target_name)
//
// The size of the mempool used to track chunks in use.
//
pub const MIN_IOS: c_int = 256;
pub const DM_TRACKED_CHUNK_HASH_SIZE: c_int = 16;

    (DM_TRACKED_CHUNK_HASH_SIZE - 1))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_hlist_head {
    pub head: hlist_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_exception_table {
    pub hash_mask: u32,
    pub hash_shift: c_uint,
    pub table: *mut dm_hlist_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_snapshot {
    pub lock: rw_semaphore,
    pub origin: *mut dm_dev,
    pub cow: *mut dm_dev,
    pub ti: *mut dm_target,
// List of snapshots per Origin
    pub list: list_head,
//
// You can't use a snapshot if this is 0 (e.g. if full).
// A snapshot-merge target never clears this.
//
    pub valid: c_int,
//
// The snapshot overflowed because of a write to the snapshot device.
// We don't have to invalidate the snapshot in this case, but we need
// to prevent further writes.
//
    pub snapshot_overflowed: c_int,
// Origin writes don't trigger exceptions until this is set
    pub active: c_int,
    pub pending_exceptions_count: core::sync::atomic::AtomicI32,
    pub pe_allocation_lock: spinlock_t,
// Protected by "pe_allocation_lock"
    pub exception_start_sequence: sector_t,
// Protected by kcopyd single-threaded callback
    pub exception_complete_sequence: sector_t,
//
// A list of pending exceptions that completed out of order.
// Protected by kcopyd single-threaded callback.
//
    pub out_of_order_tree: rb_root,
    pub pending_pool: mempool_t,
    pub pending: dm_exception_table,
    pub complete: dm_exception_table,
//
// pe_lock protects all pending_exception operations and access
// as well as the snapshot_bios list.
//
    pub pe_lock: spinlock_t,
// Chunks with outstanding reads
    pub tracked_chunk_lock: spinlock_t,
    pub tracked_chunk_hash: [hlist_head; DM_TRACKED_CHUNK_HASH_SIZE],
// The on disk metadata handler
    pub store: *mut dm_exception_store,
    pub in_progress: c_uint,
    pub in_progress_wait: wait_queue_head,
    pub kcopyd_client: *mut dm_kcopyd_client,
// Wait for events based on state_bits
    pub state_bits: c_ulong,
// Range of chunks currently being merged.
    pub first_merging_chunk: chunk_t,
    pub num_merging_chunks: c_int,
//
// The merge operation failed if this flag is set.
// Failure modes are handled as follows:
// - I/O error reading the header
// => don't load the target; abort.
// - Header does not have "valid" flag set
// => use the origin; forget about the snapshot.
// - I/O error when reading exceptions
// => don't load the target; abort.
// (We can't use the intermediate origin state.)
// - I/O error while merging
// => stop merging; set merge_failed; process I/O normally.
//
    pub merge_failed:1: bool,
    pub discard_zeroes_cow:1: bool,
    pub discard_passdown_origin:1: bool,
//
// Incoming bios that overlap with chunks being merged must wait
// for them to be committed.
//
    pub bios_queued_during_merge: bio_list,
}

//
// state_bits:
// RUNNING_MERGE  - Merge operation is in progress.
// SHUTDOWN_MERGE - Set to signal that merge needs to be stopped;
// cleared afterwards.
//
pub const RUNNING_MERGE: c_int = 0;
pub const SHUTDOWN_MERGE: c_int = 1;
//
// Maximum number of chunks being copied on write.
//
// The value was decided experimentally as a trade-off between memory
// consumption, stalling the kernel's workqueues and maintaining a high enough
// throughput.
//
pub const DEFAULT_COW_THRESHOLD: c_int = 2048;
    let mut cow_threshold: static unsigned int = DEFAULT_COW_THRESHOLD;
    module_param_named(snapshot_cow_threshold, cow_threshold, uint, 0644);
    MODULE_PARM_DESC(snapshot_cow_threshold, "Maximum number of chunks being copied on write");
    DECLARE_DM_KCOPYD_THROTTLE_WITH_MODULE_PARM(snapshot_copy_throttle,
    "A percentage of time allocated for copy on write");
    struct dm_dev *dm_snap_origin(struct dm_snapshot *s)
    {
    return s.origin;
    }
    EXPORT_SYMBOL(dm_snap_origin);
    struct dm_dev *dm_snap_cow(struct dm_snapshot *s)
    {
    return s.cow;
    }
    EXPORT_SYMBOL(dm_snap_cow);
    static sector_t chunk_to_sector(struct dm_exception_store *store,
    chunk_t chunk)
    {
    return chunk << store.chunk_shift;
    }
#[no_mangle]
unsafe extern "C" fn bdev_equal(lhs: *mut block_device, rhs: *mut block_device) -> c_int {
    static int bdev_equal(struct block_device *lhs, struct block_device *rhs)
    {
//
// There is only ever one instance of a particular block
// device so we can compare pointers safely.
//
    let mut lhs: return = = rhs;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_snap_pending_exception {
    pub e: dm_exception,
//
// Origin buffers waiting for this to complete are held
// in a bio list
//
    pub origin_bios: bio_list,
    pub snapshot_bios: bio_list,
// Pointer back to snapshot context
    pub snap: *mut dm_snapshot,
//
// 1 indicates the exception has already been sent to
// kcopyd.
//
    pub started: c_int,
// There was copying error.
    pub copy_error: c_int,
// A sequence number, it is used for in-order completion.
    pub exception_sequence: sector_t,
    pub out_of_order_node: rb_node,
//
// For writing a complete chunk, bypassing the copy.
//
    pub full_bio: *mut bio,
    pub full_bio_end_io: *mut bio_end_io_t,
}

//
// Hash table mapping origin volumes to lists of snapshots and
// a lock to protect it
//
    static struct kmem_cache *exception_cache;
    static struct kmem_cache *pending_cache;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_snap_tracked_chunk {
    pub node: hlist_node,
    pub chunk: chunk_t,
}

#[no_mangle]
unsafe extern "C" fn init_tracked_chunk(bio: *mut bio) {
    static void init_tracked_chunk(struct bio *bio)
    {
    struct dm_snap_tracked_chunk *c = dm_per_bio_data(bio, sizeof(struct dm_snap_tracked_chunk));
    INIT_HLIST_NODE(&c.node);
    }
#[no_mangle]
unsafe extern "C" fn is_bio_tracked(bio: *mut bio) -> bool {
    static bool is_bio_tracked(struct bio *bio)
    {
    struct dm_snap_tracked_chunk *c = dm_per_bio_data(bio, sizeof(struct dm_snap_tracked_chunk));
    return !hlist_unhashed(&c.node);
    }
#[no_mangle]
unsafe extern "C" fn track_chunk(s: *mut dm_snapshot, bio: *mut bio, chunk: chunk_t) {
    static void track_chunk(struct dm_snapshot *s, struct bio *bio, chunk_t chunk)
    {
    struct dm_snap_tracked_chunk *c = dm_per_bio_data(bio, sizeof(struct dm_snap_tracked_chunk));
    c.chunk = chunk;
    spin_lock_irq(&s.tracked_chunk_lock);
    hlist_add_head(&c.node,
    &s.tracked_chunk_hash[DM_TRACKED_CHUNK_HASH(chunk)]);
    spin_unlock_irq(&s.tracked_chunk_lock);
    }
#[no_mangle]
unsafe extern "C" fn stop_tracking_chunk(s: *mut dm_snapshot, bio: *mut bio) {
    static void stop_tracking_chunk(struct dm_snapshot *s, struct bio *bio)
    {
    struct dm_snap_tracked_chunk *c = dm_per_bio_data(bio, sizeof(struct dm_snap_tracked_chunk));
    unsigned long flags;
    spin_lock_irqsave(&s.tracked_chunk_lock, flags);
    hlist_del(&c.node);
    spin_unlock_irqrestore(&s.tracked_chunk_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn __chunk_is_tracked(s: *mut dm_snapshot, chunk: chunk_t) -> c_int {
    static int __chunk_is_tracked(struct dm_snapshot *s, chunk_t chunk)
    {
    struct dm_snap_tracked_chunk *c;
    let mut found: c_int = 0;
    spin_lock_irq(&s.tracked_chunk_lock);
    hlist_for_each_entry(c,
    &s.tracked_chunk_hash[DM_TRACKED_CHUNK_HASH(chunk)], node) {
    if (c.chunk == chunk) {
    found = 1;
    break;
    }
    }
    spin_unlock_irq(&s.tracked_chunk_lock);
    return found;
    }
//
// This conflicting I/O is extremely improbable in the caller,
// so fsleep(1000) is sufficient and there is no need for a wait queue.
//
#[no_mangle]
unsafe extern "C" fn __check_for_conflicting_io(s: *mut dm_snapshot, chunk: chunk_t) {
    static void __check_for_conflicting_io(struct dm_snapshot *s, chunk_t chunk)
    {
    while (__chunk_is_tracked(s, chunk))
    fsleep(1000);
    }
//
// One of these per registered origin, held in the snapshot_origins hash
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct origin {
// The origin device
    pub bdev: *mut block_device,
    pub hash_list: list_head,
// List of snapshots for this origin
    pub snapshots: list_head,
}

//
// This structure is allocated for each origin target
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_origin {
    pub dev: *mut dm_dev,
    pub ti: *mut dm_target,
    pub split_boundary: c_uint,
    pub hash_list: list_head,
}

//
// Size of the hash table for origin volumes. If we make this
// the size of the minors list then it should be nearly perfect
//
pub const ORIGIN_HASH_SIZE: c_int = 256;
pub const ORIGIN_MASK: c_uint = 0xFF;
    static struct list_head *_origins;
    static struct list_head *_dm_origins;
    static struct rw_semaphore _origins_lock;
    static DECLARE_WAIT_QUEUE_HEAD(_pending_exceptions_done);
    static DEFINE_SPINLOCK(_pending_exceptions_done_spinlock);
    static uint64_t _pending_exceptions_done_count;
#[no_mangle]
unsafe extern "C" fn init_origin_hash() -> c_int {
    static int init_origin_hash(void)
    {
    int i;
    _origins = kmalloc_objs(struct list_head, ORIGIN_HASH_SIZE);
    if (!_origins) {
    DMERR("unable to allocate memory for _origins");
    return -ENOMEM;
    }
    for (i = 0; i < ORIGIN_HASH_SIZE; i++)
    INIT_LIST_HEAD(_origins + i);
    _dm_origins = kmalloc_objs(struct list_head, ORIGIN_HASH_SIZE);
    if (!_dm_origins) {
    DMERR("unable to allocate memory for _dm_origins");
    kfree(_origins);
    return -ENOMEM;
    }
    for (i = 0; i < ORIGIN_HASH_SIZE; i++)
    INIT_LIST_HEAD(_dm_origins + i);
    init_rwsem(&_origins_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exit_origin_hash() {
    static void exit_origin_hash(void)
    {
    kfree(_origins);
    kfree(_dm_origins);
    }
#[no_mangle]
unsafe extern "C" fn origin_hash(bdev: *mut block_device) -> c_uint {
    static unsigned int origin_hash(struct block_device *bdev)
    {
    return bdev.bd_dev & ORIGIN_MASK;
    }
    static struct origin *__lookup_origin(struct block_device *origin)
    {
    struct list_head *ol;
    struct origin *o;
    ol = &_origins[origin_hash(origin)];
    list_for_each_entry(o, ol, hash_list)
    if (bdev_equal(o.bdev, origin))
    return o;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __insert_origin(o: *mut origin) {
    static void __insert_origin(struct origin *o)
    {
    struct list_head *sl = &_origins[origin_hash(o.bdev)];
    list_add_tail(&o.hash_list, sl);
    }
    static struct dm_origin *__lookup_dm_origin(struct block_device *origin)
    {
    struct list_head *ol;
    struct dm_origin *o;
    ol = &_dm_origins[origin_hash(origin)];
    list_for_each_entry(o, ol, hash_list)
    if (bdev_equal(o.dev.bdev, origin))
    return o;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __insert_dm_origin(o: *mut dm_origin) {
    static void __insert_dm_origin(struct dm_origin *o)
    {
    struct list_head *sl = &_dm_origins[origin_hash(o.dev.bdev)];
    list_add_tail(&o.hash_list, sl);
    }
#[no_mangle]
unsafe extern "C" fn __remove_dm_origin(o: *mut dm_origin) {
    static void __remove_dm_origin(struct dm_origin *o)
    {
    list_del(&o.hash_list);
    }
//
// _origins_lock must be held when calling this function.
// Returns number of snapshots registered using the supplied cow device, plus:
// snap_src - a snapshot suitable for use as a source of exception handover
// snap_dest - a snapshot capable of receiving exception handover.
// snap_merge - an existing snapshot-merge target linked to the same origin.
// There can be at most one snapshot-merge target. The parameter is optional.
//
// Possible return values and states of snap_src and snap_dest.
// 0: NULL, NULL  - first new snapshot
// 1: snap_src, NULL - normal snapshot
// 2: snap_src, snap_dest  - waiting for handover
// 2: snap_src, NULL - handed over, waiting for old to be deleted
// 1: NULL, snap_dest - source got destroyed without handover
//
    static int __find_snapshots_sharing_cow(struct dm_snapshot *snap,
    struct dm_snapshot **snap_src,
    struct dm_snapshot **snap_dest,
    struct dm_snapshot **snap_merge)
    {
    struct dm_snapshot *s;
    struct origin *o;
    let mut count: c_int = 0;
    int active;
    o = __lookup_origin(snap.origin.bdev);
    if (!o)
    goto out;
    list_for_each_entry(s, &o.snapshots, list) {
    if (dm_target_is_snapshot_merge(s.ti) && snap_merge)
// snap_merge = s;
    if (!bdev_equal(s.cow.bdev, snap.cow.bdev))
    continue;
    down_read(&s.lock);
    active = s.active;
    up_read(&s.lock);
    if (active) {
    if (snap_src)
// snap_src = s;
    } else if (snap_dest)
// snap_dest = s;
    count++;
    }
    out:
    return count;
    }
//
// On success, returns 1 if this snapshot is a handover destination,
// otherwise returns 0.
//
#[no_mangle]
unsafe extern "C" fn __validate_exception_handover(snap: *mut dm_snapshot) -> c_int {
    static int __validate_exception_handover(struct dm_snapshot *snap)
    {
    struct dm_snapshot *snap_src = core::ptr::null_mut(), *snap_dest = core::ptr::null_mut();
    struct dm_snapshot *snap_merge = core::ptr::null_mut();
// Does snapshot need exceptions handed over to it?
    if ((__find_snapshots_sharing_cow(snap, &snap_src, &snap_dest,
    &snap_merge) == 2) ||
    snap_dest) {
    snap.ti.error = "Snapshot cow pairing for exception table handover failed";
    return -EINVAL;
    }
//
// If no snap_src was found, snap cannot become a handover
// destination.
//
    if (!snap_src)
    return 0;
//
// Non-snapshot-merge handover?
//
    if (!dm_target_is_snapshot_merge(snap.ti))
    return 1;
//
// Do not allow more than one merging snapshot.
//
    if (snap_merge) {
    snap.ti.error = "A snapshot is already merging.";
    return -EINVAL;
    }
    if (!snap_src.store.type.prepare_merge ||
    !snap_src.store.type.commit_merge) {
    snap.ti.error = "Snapshot exception store does not support snapshot-merge.";
    return -EINVAL;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn __insert_snapshot(o: *mut origin, s: *mut dm_snapshot) {
    static void __insert_snapshot(struct origin *o, struct dm_snapshot *s)
    {
    struct dm_snapshot *l;
// Sort the list according to chunk size, largest-first smallest-last
    list_for_each_entry(l, &o.snapshots, list)
    if (l.store.chunk_size < s.store.chunk_size)
    break;
    list_add_tail(&s.list, &l.list);
    }
//
// Make a note of the snapshot and its origin so we can look it
// up when the origin has a write on it.
//
// Also validate snapshot exception store handovers.
// On success, returns 1 if this registration is a handover destination,
// otherwise returns 0.
//
#[no_mangle]
unsafe extern "C" fn register_snapshot(snap: *mut dm_snapshot) -> c_int {
    static int register_snapshot(struct dm_snapshot *snap)
    {
    struct origin *o, *new_o = core::ptr::null_mut();
    struct block_device *bdev = snap.origin.bdev;
    let mut r: c_int = 0;
    new_o = kmalloc_obj(*new_o);
    if (!new_o)
    return -ENOMEM;
    down_write(&_origins_lock);
    r = __validate_exception_handover(snap);
    if (r < 0) {
    kfree(new_o);
    goto out;
    }
    o = __lookup_origin(bdev);
    if (o)
    kfree(new_o);
    else {
// New origin
    o = new_o;
// Initialise the struct
    INIT_LIST_HEAD(&o.snapshots);
    o.bdev = bdev;
    __insert_origin(o);
    }
    __insert_snapshot(o, snap);
    out:
    up_write(&_origins_lock);
    return r;
    }
//
// Move snapshot to correct place in list according to chunk size.
//
#[no_mangle]
unsafe extern "C" fn reregister_snapshot(s: *mut dm_snapshot) {
    static void reregister_snapshot(struct dm_snapshot *s)
    {
    struct block_device *bdev = s.origin.bdev;
    down_write(&_origins_lock);
    list_del(&s.list);
    __insert_snapshot(__lookup_origin(bdev), s);
    up_write(&_origins_lock);
    }
#[no_mangle]
unsafe extern "C" fn unregister_snapshot(s: *mut dm_snapshot) {
    static void unregister_snapshot(struct dm_snapshot *s)
    {
    struct origin *o;
    down_write(&_origins_lock);
    o = __lookup_origin(s.origin.bdev);
    list_del(&s.list);
    if (o && list_empty(&o.snapshots)) {
    list_del(&o.hash_list);
    kfree(o);
    }
    up_write(&_origins_lock);
    }
//
// Implementation of the exception hash tables.
// The lowest hash_shift bits of the chunk number are ignored, allowing
// some consecutive chunks to be grouped together.
//
    static uint32_t exception_hash(struct dm_exception_table *et, chunk_t chunk);
// Lock to protect access to the completed and pending exception hash tables.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_exception_table_lock {
    pub complete_slot: *mut spinlock_t,
    pub pending_slot: *mut spinlock_t,
}

    static void dm_exception_table_lock_init(struct dm_snapshot *s, chunk_t chunk,
    struct dm_exception_table_lock *lock)
    {
    struct dm_exception_table *complete = &s.complete;
    struct dm_exception_table *pending = &s.pending;
    lock.complete_slot = &complete.table[exception_hash(complete, chunk)].lock;
    lock.pending_slot = &pending.table[exception_hash(pending, chunk)].lock;
    }
#[no_mangle]
unsafe extern "C" fn dm_exception_table_lock(lock: *mut dm_exception_table_lock) {
    static void dm_exception_table_lock(struct dm_exception_table_lock *lock)
    {
    spin_lock_nested(lock.complete_slot, 1);
    spin_lock_nested(lock.pending_slot, 2);
    }
#[no_mangle]
unsafe extern "C" fn dm_exception_table_unlock(lock: *mut dm_exception_table_lock) {
    static void dm_exception_table_unlock(struct dm_exception_table_lock *lock)
    {
    spin_unlock(lock.pending_slot);
    spin_unlock(lock.complete_slot);
    }
    static int dm_exception_table_init(struct dm_exception_table *et,
    uint32_t size, unsigned int hash_shift)
    {
    unsigned int i;
    et.hash_shift = hash_shift;
    et.hash_mask = size - 1;
    et.table = kvmalloc_objs(struct dm_hlist_head, size);
    if (!et.table)
    return -ENOMEM;
    for (i = 0; i < size; i++) {
    INIT_HLIST_HEAD(&et.table[i].head);
    spin_lock_init(&et.table[i].lock);
    }
    return 0;
    }
    static void dm_exception_table_exit(struct dm_exception_table *et,
    struct kmem_cache *mem)
    {
    struct dm_hlist_head *slot;
    struct dm_exception *ex;
    struct hlist_node *pos;
    int i, size;
    size = et.hash_mask + 1;
    for (i = 0; i < size; i++) {
    slot = et.table + i;
    hlist_for_each_entry_safe(ex, pos, &slot.head, hash_list) {
    hlist_del(&ex.hash_list);
    kmem_cache_free(mem, ex);
    cond_resched();
    }
    }
    kvfree(et.table);
    }
#[no_mangle]
unsafe extern "C" fn exception_hash(et: *mut dm_exception_table, chunk: chunk_t) -> u32 {
    static uint32_t exception_hash(struct dm_exception_table *et, chunk_t chunk)
    {
    return (chunk >> et.hash_shift) & et.hash_mask;
    }
#[no_mangle]
unsafe extern "C" fn dm_remove_exception(e: *mut dm_exception) {
    static void dm_remove_exception(struct dm_exception *e)
    {
    hlist_del(&e.hash_list);
    }
//
// Return the exception data for a sector, or NULL if not
// remapped.
//
    static struct dm_exception *dm_lookup_exception(struct dm_exception_table *et,
    chunk_t chunk)
    {
    struct hlist_head *slot;
    struct dm_exception *e;
    slot = &et.table[exception_hash(et, chunk)].head;
    hlist_for_each_entry(e, slot, hash_list)
    if (chunk >= e.old_chunk &&
    chunk <= e.old_chunk + dm_consecutive_chunk_count(e))
    return e;
    return core::ptr::null_mut();
    }
    static struct dm_exception *alloc_completed_exception(gfp_t gfp)
    {
    struct dm_exception *e;
    e = kmem_cache_alloc(exception_cache, gfp);
    if (!e && gfp == GFP_NOIO)
    e = kmem_cache_alloc(exception_cache, GFP_ATOMIC);
    return e;
    }
#[no_mangle]
unsafe extern "C" fn free_completed_exception(e: *mut dm_exception) {
    static void free_completed_exception(struct dm_exception *e)
    {
    kmem_cache_free(exception_cache, e);
    }
    static struct dm_snap_pending_exception *alloc_pending_exception(struct dm_snapshot *s)
    {
    struct dm_snap_pending_exception *pe = mempool_alloc(&s.pending_pool,
    GFP_NOIO);
    atomic_inc(&s.pending_exceptions_count);
    pe.snap = s;
    return pe;
    }
#[no_mangle]
unsafe extern "C" fn free_pending_exception(pe: *mut dm_snap_pending_exception) {
    static void free_pending_exception(struct dm_snap_pending_exception *pe)
    {
    struct dm_snapshot *s = pe.snap;
    mempool_free(pe, &s.pending_pool);
    smp_mb__before_atomic();
    atomic_dec(&s.pending_exceptions_count);
    }
    static void dm_insert_exception(struct dm_exception_table *eh,
    struct dm_exception *new_e)
    {
    struct hlist_head *l;
    struct dm_exception *e = core::ptr::null_mut();
    l = &eh.table[exception_hash(eh, new_e.old_chunk)].head;
// Add immediately if this table doesn't support consecutive chunks
    if (!eh.hash_shift)
    goto out;
// List is ordered by old_chunk
    hlist_for_each_entry(e, l, hash_list) {
// Insert after an existing chunk?
    if (new_e.old_chunk == (e.old_chunk +
    dm_consecutive_chunk_count(e) + 1) &&
    new_e.new_chunk == (dm_chunk_number(e.new_chunk) +
    dm_consecutive_chunk_count(e) + 1)) {
    dm_consecutive_chunk_count_inc(e);
    free_completed_exception(new_e);
    return;
    }
// Insert before an existing chunk?
    if (new_e.old_chunk == (e.old_chunk - 1) &&
    new_e.new_chunk == (dm_chunk_number(e.new_chunk) - 1)) {
    dm_consecutive_chunk_count_inc(e);
    e.old_chunk--;
    e.new_chunk--;
    free_completed_exception(new_e);
    return;
    }
    if (new_e.old_chunk < e.old_chunk)
    break;
    }
    out:
    if (!e) {
//
// Either the table doesn't support consecutive chunks or slot
// l is empty.
//
    hlist_add_head(&new_e.hash_list, l);
    } else if (new_e.old_chunk < e.old_chunk) {
// Add before an existing exception
    hlist_add_before(&new_e.hash_list, &e.hash_list);
    } else {
// Add to l's tail: e is the last exception in this slot
    hlist_add_behind(&new_e.hash_list, &e.hash_list);
    }
    }
//
// Callback used by the exception stores to load exceptions when
// initialising.
//
#[no_mangle]
unsafe extern "C" fn dm_add_exception(context: *mut c_void, old: chunk_t, new: chunk_t) -> c_int {
    static int dm_add_exception(void *context, chunk_t old, chunk_t new)
    {
    struct dm_snapshot *s = context;
    struct dm_exception *e;
    e = alloc_completed_exception(GFP_KERNEL);
    if (!e)
    return -ENOMEM;
    e.old_chunk = old;
// Consecutive_count is implicitly initialised to zero
    e.new_chunk = new;
    dm_insert_exception(&s.complete, e);
    return 0;
    }
//
// Return a minimum chunk size of all snapshots that have the specified origin.
// Return zero if the origin has no snapshots.
//
#[no_mangle]
unsafe extern "C" fn __minimum_chunk_size(o: *mut origin) -> u32 {
    static uint32_t __minimum_chunk_size(struct origin *o)
    {
    struct dm_snapshot *snap;
    let mut chunk_size: c_uint = rounddown_pow_of_two(UINT_MAX);
    if (o)
    list_for_each_entry(snap, &o.snapshots, list)
    chunk_size = min_not_zero(chunk_size,
    snap.store.chunk_size);
    return (uint32_t) chunk_size;
    }
//
// Hard coded magic.
//
#[no_mangle]
unsafe extern "C" fn calc_max_buckets() -> c_int {
    static int calc_max_buckets(void)
    {
// use a fixed size of 2MB
    let mut mem: c_ulong = 2 * 1024 * 1024;
    mem /= sizeof(struct dm_hlist_head);
    return mem;
    }
//
// Allocate room for a suitable hash table.
//
#[no_mangle]
unsafe extern "C" fn init_hash_tables(s: *mut dm_snapshot) -> c_int {
    static int init_hash_tables(struct dm_snapshot *s)
    {
    sector_t hash_size, cow_dev_size, max_buckets;
//
// Calculate based on the size of the original volume or
// the COW volume...
//
    cow_dev_size = get_dev_size(s.cow.bdev);
    max_buckets = calc_max_buckets();
    hash_size = cow_dev_size >> s.store.chunk_shift;
    hash_size = min(hash_size, max_buckets);
    if (hash_size < 64)
    hash_size = 64;
    hash_size = rounddown_pow_of_two(hash_size);
    if (dm_exception_table_init(&s.complete, hash_size,
    DM_CHUNK_CONSECUTIVE_BITS))
    return -ENOMEM;
//
// Allocate hash table for in-flight exceptions
// Make this smaller than the real hash table
//
    hash_size >>= 3;
    if (hash_size < 64)
    hash_size = 64;
    if (dm_exception_table_init(&s.pending, hash_size, 0)) {
    dm_exception_table_exit(&s.complete, exception_cache);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn merge_shutdown(s: *mut dm_snapshot) {
    static void merge_shutdown(struct dm_snapshot *s)
    {
    clear_and_wake_up_bit(RUNNING_MERGE, &s.state_bits);
    }
    static struct bio *__release_queued_bios_after_merge(struct dm_snapshot *s)
    {
    s.first_merging_chunk = 0;
    s.num_merging_chunks = 0;
    return bio_list_get(&s.bios_queued_during_merge);
    }
//
// Remove one chunk from the index of completed exceptions.
//
    static int __remove_single_exception_chunk(struct dm_snapshot *s,
    chunk_t old_chunk)
    {
    struct dm_exception *e;
    e = dm_lookup_exception(&s.complete, old_chunk);
    if (!e) {
    DMERR("Corruption detected: exception for block %llu is on disk but not in memory",
    (unsigned long long)old_chunk);
    return -EINVAL;
    }
//
// If this is the only chunk using this exception, remove exception.
//
    if (!dm_consecutive_chunk_count(e)) {
    dm_remove_exception(e);
    free_completed_exception(e);
    return 0;
    }
//
// The chunk may be either at the beginning or the end of a
// group of consecutive chunks - never in the middle.  We are
// removing chunks in the opposite order to that in which they
// were added, so this should always be true.
// Decrement the consecutive chunk counter and adjust the
// starting point if necessary.
//
    if (old_chunk == e.old_chunk) {
    e.old_chunk++;
    e.new_chunk++;
    } else if (old_chunk != e.old_chunk +
    dm_consecutive_chunk_count(e)) {
    DMERR("Attempt to merge block %llu from the middle of a chunk range [%llu - %llu]",
    (unsigned long long)old_chunk,
    (unsigned long long)e.old_chunk,
    (unsigned long long)
    e.old_chunk + dm_consecutive_chunk_count(e));
    return -EINVAL;
    }
    dm_consecutive_chunk_count_dec(e);
    return 0;
    }
    static void flush_bios(struct bio *bio);
#[no_mangle]
unsafe extern "C" fn remove_single_exception_chunk(s: *mut dm_snapshot) -> c_int {
    static int remove_single_exception_chunk(struct dm_snapshot *s)
    {
    struct bio *b = core::ptr::null_mut();
    int r;
    let mut old_chunk: chunk_t = s.first_merging_chunk + s.num_merging_chunks - 1;
    down_write(&s.lock);
//
// Process chunks (and associated exceptions) in reverse order
// so that dm_consecutive_chunk_count_dec() accounting works.
//
    do {
    r = __remove_single_exception_chunk(s, old_chunk);
    if (r)
    goto out;
    } while (old_chunk-- > s.first_merging_chunk);
    b = __release_queued_bios_after_merge(s);
    out:
    up_write(&s.lock);
    if (b)
    flush_bios(b);
    return r;
    }
    static int origin_write_extent(struct dm_snapshot *merging_snap,
    sector_t sector, unsigned int chunk_size);
    static void merge_callback(int read_err, unsigned long write_err,
    void *context);
#[no_mangle]
unsafe extern "C" fn read_pending_exceptions_done_count() -> u64 {
    static uint64_t read_pending_exceptions_done_count(void)
    {
    uint64_t pending_exceptions_done;
    spin_lock(&_pending_exceptions_done_spinlock);
    pending_exceptions_done = _pending_exceptions_done_count;
    spin_unlock(&_pending_exceptions_done_spinlock);
    return pending_exceptions_done;
    }
#[no_mangle]
unsafe extern "C" fn increment_pending_exceptions_done_count() {
    static void increment_pending_exceptions_done_count(void)
    {
    spin_lock(&_pending_exceptions_done_spinlock);
    _pending_exceptions_done_count++;
    spin_unlock(&_pending_exceptions_done_spinlock);
    wake_up_all(&_pending_exceptions_done);
    }
#[no_mangle]
unsafe extern "C" fn snapshot_merge_next_chunks(s: *mut dm_snapshot) {
    static void snapshot_merge_next_chunks(struct dm_snapshot *s)
    {
    int i, linear_chunks;
    chunk_t old_chunk, new_chunk;
    struct dm_io_region src, dest;
    sector_t io_size;
    uint64_t previous_count;
    BUG_ON(!test_bit(RUNNING_MERGE, &s.state_bits));
    if (unlikely(test_bit(SHUTDOWN_MERGE, &s.state_bits)))
    goto shut;
//
// valid flag never changes during merge, so no lock required.
//
    if (!s.valid) {
    DMERR("Snapshot is invalid: can't merge");
    goto shut;
    }
    linear_chunks = s.store.type.prepare_merge(s.store, &old_chunk,
    &new_chunk);
    if (linear_chunks <= 0) {
    if (linear_chunks < 0) {
    DMERR("Read error in exception store: shutting down merge");
    down_write(&s.lock);
    s.merge_failed = true;
    up_write(&s.lock);
    }
    goto shut;
    }
// Adjust old_chunk and new_chunk to reflect start of linear region
    old_chunk = old_chunk + 1 - linear_chunks;
    new_chunk = new_chunk + 1 - linear_chunks;
//
// Use one (potentially large) I/O to copy all 'linear_chunks'
// from the exception store to the origin
//
    io_size = linear_chunks * s.store.chunk_size;
    dest.bdev = s.origin.bdev;
    dest.sector = chunk_to_sector(s.store, old_chunk);
    dest.count = min(io_size, get_dev_size(dest.bdev) - dest.sector);
    src.bdev = s.cow.bdev;
    src.sector = chunk_to_sector(s.store, new_chunk);
    src.count = dest.count;
//
// Reallocate any exceptions needed in other snapshots then
// wait for the pending exceptions to complete.
// Each time any pending exception (globally on the system)
// completes we are woken and repeat the process to find out
// if we can proceed.  While this may not seem a particularly
// efficient algorithm, it is not expected to have any
// significant impact on performance.
//
    previous_count = read_pending_exceptions_done_count();
    while (origin_write_extent(s, dest.sector, io_size)) {
    wait_event(_pending_exceptions_done,
    (read_pending_exceptions_done_count() !=
    previous_count));
// Retry after the wait, until all exceptions are done.
    previous_count = read_pending_exceptions_done_count();
    }
    down_write(&s.lock);
    s.first_merging_chunk = old_chunk;
    s.num_merging_chunks = linear_chunks;
    up_write(&s.lock);
// Wait until writes to all 'linear_chunks' drain
    for (i = 0; i < linear_chunks; i++)
    __check_for_conflicting_io(s, old_chunk + i);
    dm_kcopyd_copy(s.kcopyd_client, &src, 1, &dest, 0, merge_callback, s);
    return;
    shut:
    merge_shutdown(s);
    }
    static void error_bios(struct bio *bio);
#[no_mangle]
unsafe extern "C" fn merge_callback(read_err: c_int, write_err: c_ulong, context: *mut c_void) {
    static void merge_callback(int read_err, unsigned long write_err, void *context)
    {
    struct dm_snapshot *s = context;
    struct bio *b = core::ptr::null_mut();
    if (read_err || write_err) {
    if (read_err)
    DMERR("Read error: shutting down merge.");
    else
    DMERR("Write error: shutting down merge.");
    goto shut;
    }
    if (blkdev_issue_flush(s.origin.bdev) < 0) {
    DMERR("Flush after merge failed: shutting down merge");
    goto shut;
    }
    if (s.store.type.commit_merge(s.store,
    s.num_merging_chunks) < 0) {
    DMERR("Write error in exception store: shutting down merge");
    goto shut;
    }
    if (remove_single_exception_chunk(s) < 0)
    goto shut;
    snapshot_merge_next_chunks(s);
    return;
    shut:
    down_write(&s.lock);
    s.merge_failed = true;
    b = __release_queued_bios_after_merge(s);
    up_write(&s.lock);
    error_bios(b);
    merge_shutdown(s);
    }
#[no_mangle]
unsafe extern "C" fn start_merge(s: *mut dm_snapshot) {
    static void start_merge(struct dm_snapshot *s)
    {
    if (!test_and_set_bit(RUNNING_MERGE, &s.state_bits))
    snapshot_merge_next_chunks(s);
    }
//
// Stop the merging process and wait until it finishes.
//
#[no_mangle]
unsafe extern "C" fn stop_merge(s: *mut dm_snapshot) {
    static void stop_merge(struct dm_snapshot *s)
    {
    set_bit(SHUTDOWN_MERGE, &s.state_bits);
    wait_on_bit(&s.state_bits, RUNNING_MERGE, TASK_UNINTERRUPTIBLE);
    clear_bit(SHUTDOWN_MERGE, &s.state_bits);
    }
    static int parse_snapshot_features(struct dm_arg_set *as, struct dm_snapshot *s,
    struct dm_target *ti)
    {
    int r;
    unsigned int argc;
    const char *arg_name;
    static const struct dm_arg _args[] = {
    {0, 2, "Invalid number of feature arguments"},
    };
//
// No feature arguments supplied.
//
    if (!as.argc)
    return 0;
    r = dm_read_arg_group(_args, as, &argc, &ti.error);
    if (r)
    return -EINVAL;
    while (argc && !r) {
    arg_name = dm_shift_arg(as);
    argc--;
    if (!strcasecmp(arg_name, "discard_zeroes_cow"))
    s.discard_zeroes_cow = true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(arg_name, _arg: "discard_passdown_origin")) -> else {
    else if (!strcasecmp(arg_name, "discard_passdown_origin"))
    s.discard_passdown_origin = true;
    else {
    ti.error = "Unrecognised feature requested";
    r = -EINVAL;
    break;
    }
    }
    if (!s.discard_zeroes_cow && s.discard_passdown_origin) {
//
// TODO: really these are disjoint.. but ti->num_discard_bios
// and dm_bio_get_target_bio_nr() require rigid constraints.
//
    ti.error = "discard_passdown_origin feature depends on discard_zeroes_cow";
    r = -EINVAL;
    }
    return r;
    }
//
// Construct a snapshot mapping:
// <origin_dev> <COW-dev> <p|po|n> <chunk-size> [<# feature args> [<arg>]*]
//
#[no_mangle]
unsafe extern "C" fn snapshot_ctr(ti: *mut dm_target, argc: c_uint, argv: *mut c_char) -> c_int {
    static int snapshot_ctr(struct dm_target *ti, unsigned int argc, char **argv)
    {
    struct dm_snapshot *s;
    struct dm_arg_set as;
    int i;
    let mut r: c_int = -EINVAL;
    char *origin_path, *cow_path;
    unsigned int args_used, num_flush_bios = 1;
    let mut origin_mode: blk_mode_t = BLK_OPEN_READ;
    if (argc < 4) {
    ti.error = "requires 4 or more arguments";
    r = -EINVAL;
    goto bad;
    }
    if (dm_target_is_snapshot_merge(ti)) {
    num_flush_bios = 2;
    origin_mode = BLK_OPEN_WRITE;
    }
    s = kzalloc_obj(*s);
    if (!s) {
    ti.error = "Cannot allocate private snapshot structure";
    r = -ENOMEM;
    goto bad;
    }
    as.argc = argc;
    as.argv = argv;
    dm_consume_args(&as, 4);
    r = parse_snapshot_features(&as, s, ti);
    if (r)
    goto bad_features;
    origin_path = argv[0];
    argv++;
    argc--;
    r = dm_get_device(ti, origin_path, origin_mode, &s.origin);
    if (r) {
    ti.error = "Cannot get origin device";
    goto bad_origin;
    }
    cow_path = argv[0];
    argv++;
    argc--;
    r = dm_get_device(ti, cow_path, dm_table_get_mode(ti.table), &s.cow);
    if (r) {
    ti.error = "Cannot get COW device";
    goto bad_cow;
    }
    if (s.cow.bdev && s.cow.bdev == s.origin.bdev) {
    ti.error = "COW device cannot be the same as origin device";
    r = -EINVAL;
    goto bad_store;
    }
    r = dm_exception_store_create(ti, argc, argv, s, &args_used, &s.store);
    if (r) {
    ti.error = "Couldn't create exception store";
    r = -EINVAL;
    goto bad_store;
    }
    argv += args_used;
    argc -= args_used;
    s.ti = ti;
    s.valid = 1;
    s.snapshot_overflowed = 0;
    s.active = 0;
    atomic_set(&s.pending_exceptions_count, 0);
    spin_lock_init(&s.pe_allocation_lock);
    s.exception_start_sequence = 0;
    s.exception_complete_sequence = 0;
    s.out_of_order_tree = RB_ROOT;
    init_rwsem(&s.lock);
    INIT_LIST_HEAD(&s.list);
    spin_lock_init(&s.pe_lock);
    s.state_bits = 0;
    s.merge_failed = false;
    s.first_merging_chunk = 0;
    s.num_merging_chunks = 0;
    bio_list_init(&s.bios_queued_during_merge);
// Allocate hash table for COW data
    if (init_hash_tables(s)) {
    ti.error = "Unable to allocate hash table space";
    r = -ENOMEM;
    goto bad_hash_tables;
    }
    init_waitqueue_head(&s.in_progress_wait);
    s.kcopyd_client = dm_kcopyd_client_create(&dm_kcopyd_throttle);
    if (IS_ERR(s.kcopyd_client)) {
    r = PTR_ERR(s.kcopyd_client);
    ti.error = "Could not create kcopyd client";
    goto bad_kcopyd;
    }
    r = mempool_init_slab_pool(&s.pending_pool, MIN_IOS, pending_cache);
    if (r) {
    ti.error = "Could not allocate mempool for pending exceptions";
    goto bad_pending_pool;
    }
    for (i = 0; i < DM_TRACKED_CHUNK_HASH_SIZE; i++)
    INIT_HLIST_HEAD(&s.tracked_chunk_hash[i]);
    spin_lock_init(&s.tracked_chunk_lock);
    ti.private = s;
    ti.num_flush_bios = num_flush_bios;
    if (s.discard_zeroes_cow)
    ti.num_discard_bios = (s.discard_passdown_origin ? 2 : 1);
    ti.per_io_data_size = sizeof(struct dm_snap_tracked_chunk);
// Add snapshot to the list of snapshots for this origin
// Exceptions aren't triggered till snapshot_resume() is called
    r = register_snapshot(s);
    if (r == -ENOMEM) {
    ti.error = "Snapshot origin struct allocation failed";
    goto bad_load_and_register;
    } else if (r < 0) {
// invalid handover, register_snapshot has set ti->error
    goto bad_load_and_register;
    }
//
// Metadata must only be loaded into one table at once, so skip this
// if metadata will be handed over during resume.
// Chunk size will be set during the handover - set it to zero to
// ensure it's ignored.
//
    if (r > 0) {
    s.store.chunk_size = 0;
    return 0;
    }
    r = s.store.type.read_metadata(s.store, dm_add_exception,
    (void *)s);
    if (r < 0) {
    ti.error = "Failed to read snapshot metadata";
    goto bad_read_metadata;
    } else if (r > 0) {
    s.valid = 0;
    DMWARN("Snapshot is marked invalid.");
    }
    if (!s.store.chunk_size) {
    ti.error = "Chunk size not set";
    r = -EINVAL;
    goto bad_read_metadata;
    }
    r = dm_set_target_max_io_len(ti, s.store.chunk_size);
    if (r)
    goto bad_read_metadata;
    return 0;
    bad_read_metadata:
    unregister_snapshot(s);
    bad_load_and_register:
    mempool_exit(&s.pending_pool);
    bad_pending_pool:
    dm_kcopyd_client_destroy(s.kcopyd_client);
    bad_kcopyd:
    dm_exception_table_exit(&s.pending, pending_cache);
    dm_exception_table_exit(&s.complete, exception_cache);
    bad_hash_tables:
    dm_exception_store_destroy(s.store);
    bad_store:
    dm_put_device(ti, s.cow);
    bad_cow:
    dm_put_device(ti, s.origin);
    bad_origin:
    bad_features:
    kfree(s);
    bad:
    return r;
    }
#[no_mangle]
unsafe extern "C" fn __free_exceptions(s: *mut dm_snapshot) {
    static void __free_exceptions(struct dm_snapshot *s)
    {
    dm_kcopyd_client_destroy(s.kcopyd_client);
    s.kcopyd_client = core::ptr::null_mut();
    dm_exception_table_exit(&s.pending, pending_cache);
    dm_exception_table_exit(&s.complete, exception_cache);
    }
    static void __handover_exceptions(struct dm_snapshot *snap_src,
    struct dm_snapshot *snap_dest)
    {
    union {
    struct dm_exception_table table_swap;
    struct dm_exception_store *store_swap;
    } u;
//
// Swap all snapshot context information between the two instances.
//
    u.table_swap = snap_dest.complete;
    snap_dest.complete = snap_src.complete;
    snap_src.complete = u.table_swap;
    u.store_swap = snap_dest.store;
    snap_dest.store = snap_src.store;
    snap_dest.store.userspace_supports_overflow = u.store_swap.userspace_supports_overflow;
    snap_src.store = u.store_swap;
    snap_dest.store.snap = snap_dest;
    snap_src.store.snap = snap_src;
    snap_dest.ti.max_io_len = snap_dest.store.chunk_size;
    snap_dest.valid = snap_src.valid;
    snap_dest.snapshot_overflowed = snap_src.snapshot_overflowed;
//
// Set source invalid to ensure it receives no further I/O.
//
    snap_src.valid = 0;
    }
#[no_mangle]
unsafe extern "C" fn snapshot_dtr(ti: *mut dm_target) {
    static void snapshot_dtr(struct dm_target *ti)
    {

    int i;

    struct dm_snapshot *s = ti.private;
    struct dm_snapshot *snap_src = core::ptr::null_mut(), *snap_dest = core::ptr::null_mut();
    down_read(&_origins_lock);
// Check whether exception handover must be cancelled
    (void) __find_snapshots_sharing_cow(s, &snap_src, &snap_dest, core::ptr::null_mut());
    if (snap_src && snap_dest && (s == snap_src)) {
    down_write(&snap_dest.lock);
    snap_dest.valid = 0;
    up_write(&snap_dest.lock);
    DMERR("Cancelling snapshot handover.");
    }
    up_read(&_origins_lock);
    if (dm_target_is_snapshot_merge(ti))
    stop_merge(s);
// Prevent further origin writes from using this snapshot.
// After this returns there can be no new kcopyd jobs.
    unregister_snapshot(s);
    while (atomic_read(&s.pending_exceptions_count))
    fsleep(1000);
//
// Ensure instructions in mempool_exit aren't reordered
// before atomic_read.
//
    smp_mb();

    for (i = 0; i < DM_TRACKED_CHUNK_HASH_SIZE; i++)
    BUG_ON(!hlist_empty(&s.tracked_chunk_hash[i]));

    __free_exceptions(s);
    mempool_exit(&s.pending_pool);
    dm_exception_store_destroy(s.store);
    dm_put_device(ti, s.cow);
    dm_put_device(ti, s.origin);
    WARN_ON(s.in_progress);
    kfree(s);
    }
#[no_mangle]
unsafe extern "C" fn account_start_copy(s: *mut dm_snapshot) {
    static void account_start_copy(struct dm_snapshot *s)
    {
    spin_lock(&s.in_progress_wait.lock);
    s.in_progress++;
    spin_unlock(&s.in_progress_wait.lock);
    }
#[no_mangle]
unsafe extern "C" fn account_end_copy(s: *mut dm_snapshot) {
    static void account_end_copy(struct dm_snapshot *s)
    {
    spin_lock(&s.in_progress_wait.lock);
    BUG_ON(!s.in_progress);
    s.in_progress--;
    if (likely(s.in_progress <= cow_threshold) &&
    unlikely(waitqueue_active(&s.in_progress_wait)))
    wake_up_locked(&s.in_progress_wait);
    spin_unlock(&s.in_progress_wait.lock);
    }
#[no_mangle]
unsafe extern "C" fn wait_for_in_progress(s: *mut dm_snapshot, unlock_origins: bool) -> bool {
    static bool wait_for_in_progress(struct dm_snapshot *s, bool unlock_origins)
    {
    if (unlikely(s.in_progress > cow_threshold)) {
    spin_lock(&s.in_progress_wait.lock);
    if (likely(s.in_progress > cow_threshold)) {
//
// NOTE: this throttle doesn't account for whether
// the caller is servicing an IO that will trigger a COW
// so excess throttling may result for chunks not required
// to be COW'd.  But if cow_threshold was reached, extra
// throttling is unlikely to negatively impact performance.
//
    DECLARE_WAITQUEUE(wait, current);
    __add_wait_queue(&s.in_progress_wait, &wait);
    __set_current_state(TASK_UNINTERRUPTIBLE);
    spin_unlock(&s.in_progress_wait.lock);
    if (unlock_origins)
    up_read(&_origins_lock);
    io_schedule();
    remove_wait_queue(&s.in_progress_wait, &wait);
    return false;
    }
    spin_unlock(&s.in_progress_wait.lock);
    }
    return true;
    }
//
// Flush a list of buffers.
//
#[no_mangle]
unsafe extern "C" fn flush_bios(bio: *mut bio) {
    static void flush_bios(struct bio *bio)
    {
    struct bio *n;
    while (bio) {
    n = bio.bi_next;
    bio.bi_next = core::ptr::null_mut();
    submit_bio_noacct(bio);
    bio = n;
    }
    }
    static int do_origin(struct dm_dev *origin, struct bio *bio, bool limit);
//
// Flush a list of buffers.
//
#[no_mangle]
unsafe extern "C" fn retry_origin_bios(s: *mut dm_snapshot, bio: *mut bio) {
    static void retry_origin_bios(struct dm_snapshot *s, struct bio *bio)
    {
    struct bio *n;
    int r;
    while (bio) {
    n = bio.bi_next;
    bio.bi_next = core::ptr::null_mut();
    r = do_origin(s.origin, bio, false);
    if (r == DM_MAPIO_REMAPPED)
    submit_bio_noacct(bio);
    bio = n;
    }
    }
//
// Error a list of buffers.
//
#[no_mangle]
unsafe extern "C" fn error_bios(bio: *mut bio) {
    static void error_bios(struct bio *bio)
    {
    struct bio *n;
    while (bio) {
    n = bio.bi_next;
    bio.bi_next = core::ptr::null_mut();
    bio_io_error(bio);
    bio = n;
    }
    }
#[no_mangle]
unsafe extern "C" fn __invalidate_snapshot(s: *mut dm_snapshot, err: c_int) {
    static void __invalidate_snapshot(struct dm_snapshot *s, int err)
    {
    if (!s.valid)
    return;
    if (err == -EIO)
    DMERR("Invalidating snapshot: Error reading/writing.");
#[no_mangle]
pub unsafe extern "C" fn if(-ENOMEM: err ==) -> else {
    else if (err == -ENOMEM)
    DMERR("Invalidating snapshot: Unable to allocate exception.");
    if (s.store.type.drop_snapshot)
    s.store.type.drop_snapshot(s.store);
    s.valid = 0;
    dm_table_event(s.ti.table);
    }
#[no_mangle]
unsafe extern "C" fn invalidate_snapshot(s: *mut dm_snapshot, err: c_int) {
    static void invalidate_snapshot(struct dm_snapshot *s, int err)
    {
    down_write(&s.lock);
    __invalidate_snapshot(s, err);
    up_write(&s.lock);
    }
#[no_mangle]
unsafe extern "C" fn pending_complete(context: *mut c_void, success: c_int) {
    static void pending_complete(void *context, int success)
    {
    struct dm_snap_pending_exception *pe = context;
    struct dm_exception *e;
    struct dm_snapshot *s = pe.snap;
    struct bio *origin_bios = core::ptr::null_mut();
    struct bio *snapshot_bios = core::ptr::null_mut();
    struct bio *full_bio = core::ptr::null_mut();
    struct dm_exception_table_lock lock;
    let mut error: c_int = 0;
    dm_exception_table_lock_init(s, pe.e.old_chunk, &lock);
    if (!success) {
// Read/write error - snapshot is unusable
    invalidate_snapshot(s, -EIO);
    error = 1;
    dm_exception_table_lock(&lock);
    goto out;
    }
    e = alloc_completed_exception(GFP_NOIO);
    if (!e) {
    invalidate_snapshot(s, -ENOMEM);
    error = 1;
    dm_exception_table_lock(&lock);
    goto out;
    }
// e = pe->e;
    down_read(&s.lock);
    dm_exception_table_lock(&lock);
    if (!s.valid) {
    up_read(&s.lock);
    free_completed_exception(e);
    error = 1;
    goto out;
    }
//
// Add a proper exception. After inserting the completed exception all
// subsequent snapshot reads to this chunk will be redirected to the
// COW device.  This ensures that we do not starve. Moreover, as long
// as the pending exception exists, neither origin writes nor snapshot
// merging can overwrite the chunk in origin.
//
    dm_insert_exception(&s.complete, e);
    up_read(&s.lock);
// Wait for conflicting reads to drain
    if (__chunk_is_tracked(s, pe.e.old_chunk)) {
    dm_exception_table_unlock(&lock);
    __check_for_conflicting_io(s, pe.e.old_chunk);
    dm_exception_table_lock(&lock);
    }
    out:
// Remove the in-flight exception from the list
    dm_remove_exception(&pe.e);
    dm_exception_table_unlock(&lock);
    snapshot_bios = bio_list_get(&pe.snapshot_bios);
    origin_bios = bio_list_get(&pe.origin_bios);
    full_bio = pe.full_bio;
    if (full_bio)
    full_bio.bi_end_io = pe.full_bio_end_io;
    increment_pending_exceptions_done_count();
// Submit any pending write bios
    if (error) {
    if (full_bio)
    bio_io_error(full_bio);
    error_bios(snapshot_bios);
    } else {
    if (full_bio)
    bio_endio(full_bio);
    flush_bios(snapshot_bios);
    }
    retry_origin_bios(s, origin_bios);
    free_pending_exception(pe);
    }
#[no_mangle]
unsafe extern "C" fn complete_exception(pe: *mut dm_snap_pending_exception) {
    static void complete_exception(struct dm_snap_pending_exception *pe)
    {
    struct dm_snapshot *s = pe.snap;
// Update the metadata if we are persistent
    s.store.type.commit_exception(s.store, &pe.e, !pe.copy_error,
    pending_complete, pe);
    }
//
// Called when the copy I/O has finished.  kcopyd actually runs
// this code so don't block.
//
#[no_mangle]
unsafe extern "C" fn copy_callback(read_err: c_int, write_err: c_ulong, context: *mut c_void) {
    static void copy_callback(int read_err, unsigned long write_err, void *context)
    {
    struct dm_snap_pending_exception *pe = context;
    struct dm_snapshot *s = pe.snap;
    pe.copy_error = read_err || write_err;
    if (pe.exception_sequence == s.exception_complete_sequence) {
    struct rb_node *next;
    s.exception_complete_sequence++;
    complete_exception(pe);
    next = rb_first(&s.out_of_order_tree);
    while (next) {
    pe = rb_entry(next, struct dm_snap_pending_exception,
    out_of_order_node);
    if (pe.exception_sequence != s.exception_complete_sequence)
    break;
    next = rb_next(next);
    s.exception_complete_sequence++;
    rb_erase(&pe.out_of_order_node, &s.out_of_order_tree);
    complete_exception(pe);
    cond_resched();
    }
    } else {
    struct rb_node *parent = core::ptr::null_mut();
    struct rb_node **p = &s.out_of_order_tree.rb_node;
    struct dm_snap_pending_exception *pe2;
    while (*p) {
    pe2 = rb_entry(*p, struct dm_snap_pending_exception, out_of_order_node);
    parent = *p;
    BUG_ON(pe.exception_sequence == pe2.exception_sequence);
    if (pe.exception_sequence < pe2.exception_sequence)
    p = &((*p).rb_left);
    else
    p = &((*p).rb_right);
    }
    rb_link_node(&pe.out_of_order_node, parent, p);
    rb_insert_color(&pe.out_of_order_node, &s.out_of_order_tree);
    }
    account_end_copy(s);
    }
//
// Dispatches the copy operation to kcopyd.
//
#[no_mangle]
unsafe extern "C" fn start_copy(pe: *mut dm_snap_pending_exception) {
    static void start_copy(struct dm_snap_pending_exception *pe)
    {
    struct dm_snapshot *s = pe.snap;
    struct dm_io_region src, dest;
    struct block_device *bdev = s.origin.bdev;
    sector_t dev_size;
    dev_size = get_dev_size(bdev);
    src.bdev = bdev;
    src.sector = chunk_to_sector(s.store, pe.e.old_chunk);
    src.count = min((sector_t)s.store.chunk_size, dev_size - src.sector);
    dest.bdev = s.cow.bdev;
    dest.sector = chunk_to_sector(s.store, pe.e.new_chunk);
    dest.count = src.count;
// Hand over to kcopyd
    account_start_copy(s);
    dm_kcopyd_copy(s.kcopyd_client, &src, 1, &dest, 0, copy_callback, pe);
    }
#[no_mangle]
unsafe extern "C" fn full_bio_end_io(bio: *mut bio) {
    static void full_bio_end_io(struct bio *bio)
    {
    void *callback_data = bio.bi_private;
    dm_kcopyd_do_callback(callback_data, 0, bio.bi_status ? 1 : 0);
    }
    static void start_full_bio(struct dm_snap_pending_exception *pe,
    struct bio *bio)
    {
    struct dm_snapshot *s = pe.snap;
    void *callback_data;
    pe.full_bio = bio;
    pe.full_bio_end_io = bio.bi_end_io;
    account_start_copy(s);
    callback_data = dm_kcopyd_prepare_callback(s.kcopyd_client,
    copy_callback, pe);
    bio.bi_end_io = full_bio_end_io;
    bio.bi_private = callback_data;
    submit_bio_noacct(bio);
    }
    static struct dm_snap_pending_exception *
    __lookup_pending_exception(struct dm_snapshot *s, chunk_t chunk)
    {
    struct dm_exception *e = dm_lookup_exception(&s.pending, chunk);
    if (!e)
    return core::ptr::null_mut();
    return container_of(e, struct dm_snap_pending_exception, e);
    }
//
// Inserts a pending exception into the pending table.
//
// NOTE: a write lock must be held on the chunk's pending exception table slot
// before calling this.
//
    static struct dm_snap_pending_exception *
    __insert_pending_exception(struct dm_snapshot *s,
    struct dm_snap_pending_exception *pe, chunk_t chunk)
    {
    pe.e.old_chunk = chunk;
    bio_list_init(&pe.origin_bios);
    bio_list_init(&pe.snapshot_bios);
    pe.started = 0;
    pe.full_bio = core::ptr::null_mut();
    spin_lock(&s.pe_allocation_lock);
    if (s.store.type.prepare_exception(s.store, &pe.e)) {
    spin_unlock(&s.pe_allocation_lock);
    free_pending_exception(pe);
    return core::ptr::null_mut();
    }
    pe.exception_sequence = s.exception_start_sequence++;
    spin_unlock(&s.pe_allocation_lock);
    dm_insert_exception(&s.pending, &pe.e);
    return pe;
    }
//
// Looks to see if this snapshot already has a pending exception
// for this chunk, otherwise it allocates a new one and inserts
// it into the pending table.
//
// NOTE: a write lock must be held on the chunk's pending exception table slot
// before calling this.
//
    static struct dm_snap_pending_exception *
    __find_pending_exception(struct dm_snapshot *s,
    struct dm_snap_pending_exception *pe, chunk_t chunk)
    {
    struct dm_snap_pending_exception *pe2;
    pe2 = __lookup_pending_exception(s, chunk);
    if (pe2) {
    free_pending_exception(pe);
    return pe2;
    }
    return __insert_pending_exception(s, pe, chunk);
    }
    static void remap_exception(struct dm_snapshot *s, struct dm_exception *e,
    struct bio *bio, chunk_t chunk)
    {
    bio_set_dev(bio, s.cow.bdev);
    bio.bi_iter.bi_sector =
    chunk_to_sector(s.store, dm_chunk_number(e.new_chunk) +
    (chunk - e.old_chunk)) +
    (bio.bi_iter.bi_sector & s.store.chunk_mask);
    }
#[no_mangle]
unsafe extern "C" fn zero_callback(read_err: c_int, write_err: c_ulong, context: *mut c_void) {
    static void zero_callback(int read_err, unsigned long write_err, void *context)
    {
    struct bio *bio = context;
    struct dm_snapshot *s = bio.bi_private;
    account_end_copy(s);
    bio.bi_status = write_err ? BLK_STS_IOERR : 0;
    bio_endio(bio);
    }
    static void zero_exception(struct dm_snapshot *s, struct dm_exception *e,
    struct bio *bio, chunk_t chunk)
    {
    struct dm_io_region dest;
    dest.bdev = s.cow.bdev;
    dest.sector = bio.bi_iter.bi_sector;
    dest.count = s.store.chunk_size;
    account_start_copy(s);
    WARN_ON_ONCE(bio.bi_private);
    bio.bi_private = s;
    dm_kcopyd_zero(s.kcopyd_client, 1, &dest, 0, zero_callback, bio);
    }
#[no_mangle]
unsafe extern "C" fn io_overlaps_chunk(s: *mut dm_snapshot, bio: *mut bio) -> bool {
    static bool io_overlaps_chunk(struct dm_snapshot *s, struct bio *bio)
    {
    return bio.bi_iter.bi_size ==
    (s.store.chunk_size << SECTOR_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn snapshot_map(ti: *mut dm_target, bio: *mut bio) -> c_int {
    static int snapshot_map(struct dm_target *ti, struct bio *bio)
    {
    struct dm_exception *e;
    struct dm_snapshot *s = ti.private;
    let mut r: c_int = DM_MAPIO_REMAPPED;
    chunk_t chunk;
    struct dm_snap_pending_exception *pe = core::ptr::null_mut();
    struct dm_exception_table_lock lock;
    init_tracked_chunk(bio);
    if (bio.bi_opf & REQ_PREFLUSH) {
    bio_set_dev(bio, s.cow.bdev);
    return DM_MAPIO_REMAPPED;
    }
    chunk = sector_to_chunk(s.store, bio.bi_iter.bi_sector);
    dm_exception_table_lock_init(s, chunk, &lock);
// Full snapshots are not usable
// To get here the table must be live so s->active is always set.
    if (!s.valid)
    return DM_MAPIO_KILL;
    if (bio_data_dir(bio) == WRITE) {
    while (unlikely(!wait_for_in_progress(s, false)))
    ; /* wait_for_in_progress() has slept */
    }
    down_read(&s.lock);
    dm_exception_table_lock(&lock);
    if (!s.valid || (unlikely(s.snapshot_overflowed) &&
    bio_data_dir(bio) == WRITE)) {
    r = DM_MAPIO_KILL;
    goto out_unlock;
    }
    if (unlikely(bio_op(bio) == REQ_OP_DISCARD)) {
    if (s.discard_passdown_origin && dm_bio_get_target_bio_nr(bio)) {
//
// passdown discard to origin (without triggering
// snapshot exceptions via do_origin; doing so would
// defeat the goal of freeing space in origin that is
// implied by the "discard_passdown_origin" feature)
//
    bio_set_dev(bio, s.origin.bdev);
    track_chunk(s, bio, chunk);
    goto out_unlock;
    }
// discard to snapshot (target_bio_nr == 0) zeroes exceptions
    }
// If the block is already remapped - use that, else remap it
    e = dm_lookup_exception(&s.complete, chunk);
    if (e) {
    remap_exception(s, e, bio, chunk);
    if (unlikely(bio_op(bio) == REQ_OP_DISCARD) &&
    io_overlaps_chunk(s, bio)) {
    dm_exception_table_unlock(&lock);
    up_read(&s.lock);
    zero_exception(s, e, bio, chunk);
    r = DM_MAPIO_SUBMITTED; /* discard is not issued */
    goto out;
    }
    goto out_unlock;
    }
    if (unlikely(bio_op(bio) == REQ_OP_DISCARD)) {
//
// If no exception exists, complete discard immediately
// otherwise it'll trigger copy-out.
//
    bio_endio(bio);
    r = DM_MAPIO_SUBMITTED;
    goto out_unlock;
    }
//
// Write to snapshot - higher level takes care of RW/RO
// flags so we should only get this if we are
// writable.
//
    if (bio_data_dir(bio) == WRITE) {
    pe = __lookup_pending_exception(s, chunk);
    if (!pe) {
    dm_exception_table_unlock(&lock);
    pe = alloc_pending_exception(s);
    dm_exception_table_lock(&lock);
    e = dm_lookup_exception(&s.complete, chunk);
    if (e) {
    free_pending_exception(pe);
    remap_exception(s, e, bio, chunk);
    goto out_unlock;
    }
    pe = __find_pending_exception(s, pe, chunk);
    if (!pe) {
    dm_exception_table_unlock(&lock);
    up_read(&s.lock);
    down_write(&s.lock);
    if (s.store.userspace_supports_overflow) {
    if (s.valid && !s.snapshot_overflowed) {
    s.snapshot_overflowed = 1;
    DMERR("Snapshot overflowed: Unable to allocate exception.");
    }
    } else
    __invalidate_snapshot(s, -ENOMEM);
    up_write(&s.lock);
    r = DM_MAPIO_KILL;
    goto out;
    }
    }
    remap_exception(s, &pe.e, bio, chunk);
    r = DM_MAPIO_SUBMITTED;
    if (!pe.started && io_overlaps_chunk(s, bio)) {
    pe.started = 1;
    dm_exception_table_unlock(&lock);
    up_read(&s.lock);
    start_full_bio(pe, bio);
    goto out;
    }
    bio_list_add(&pe.snapshot_bios, bio);
    if (!pe.started) {
// this is protected by the exception table lock
    pe.started = 1;
    dm_exception_table_unlock(&lock);
    up_read(&s.lock);
    start_copy(pe);
    goto out;
    }
    } else {
    bio_set_dev(bio, s.origin.bdev);
    track_chunk(s, bio, chunk);
    }
    out_unlock:
    dm_exception_table_unlock(&lock);
    up_read(&s.lock);
    out:
    return r;
    }
//
// A snapshot-merge target behaves like a combination of a snapshot
// target and a snapshot-origin target.  It only generates new
// exceptions in other snapshots and not in the one that is being
// merged.
//
// For each chunk, if there is an existing exception, it is used to
// redirect I/O to the cow device.  Otherwise I/O is sent to the origin,
// which in turn might generate exceptions in other snapshots.
// If merging is currently taking place on the chunk in question, the
// I/O is deferred by adding it to s->bios_queued_during_merge.
//
#[no_mangle]
unsafe extern "C" fn snapshot_merge_map(ti: *mut dm_target, bio: *mut bio) -> c_int {
    static int snapshot_merge_map(struct dm_target *ti, struct bio *bio)
    {
    struct dm_exception *e;
    struct dm_snapshot *s = ti.private;
    let mut r: c_int = DM_MAPIO_REMAPPED;
    chunk_t chunk;
    init_tracked_chunk(bio);
    if (bio.bi_opf & REQ_PREFLUSH) {
    if (!dm_bio_get_target_bio_nr(bio))
    bio_set_dev(bio, s.origin.bdev);
    else
    bio_set_dev(bio, s.cow.bdev);
    return DM_MAPIO_REMAPPED;
    }
    if (unlikely(bio_op(bio) == REQ_OP_DISCARD)) {
// Once merging, discards no longer effect change
    bio_endio(bio);
    return DM_MAPIO_SUBMITTED;
    }
    chunk = sector_to_chunk(s.store, bio.bi_iter.bi_sector);
    down_write(&s.lock);
// Full merging snapshots are redirected to the origin
    if (!s.valid)
    goto redirect_to_origin;
// If the block is already remapped - use that
    e = dm_lookup_exception(&s.complete, chunk);
    if (e) {
// Queue writes overlapping with chunks being merged
    if (bio_data_dir(bio) == WRITE &&
    chunk >= s.first_merging_chunk &&
    chunk < (s.first_merging_chunk +
    s.num_merging_chunks)) {
    bio_set_dev(bio, s.origin.bdev);
    bio_list_add(&s.bios_queued_during_merge, bio);
    r = DM_MAPIO_SUBMITTED;
    goto out_unlock;
    }
    remap_exception(s, e, bio, chunk);
    if (bio_data_dir(bio) == WRITE)
    track_chunk(s, bio, chunk);
    goto out_unlock;
    }
    redirect_to_origin:
    bio_set_dev(bio, s.origin.bdev);
    if (bio_data_dir(bio) == WRITE) {
    up_write(&s.lock);
    return do_origin(s.origin, bio, false);
    }
    out_unlock:
    up_write(&s.lock);
    return r;
    }
    static int snapshot_end_io(struct dm_target *ti, struct bio *bio,
    blk_status_t *error)
    {
    struct dm_snapshot *s = ti.private;
    if (is_bio_tracked(bio))
    stop_tracking_chunk(s, bio);
    return DM_ENDIO_DONE;
    }
#[no_mangle]
unsafe extern "C" fn snapshot_merge_presuspend(ti: *mut dm_target) {
    static void snapshot_merge_presuspend(struct dm_target *ti)
    {
    struct dm_snapshot *s = ti.private;
    stop_merge(s);
    }
#[no_mangle]
unsafe extern "C" fn snapshot_preresume(ti: *mut dm_target) -> c_int {
    static int snapshot_preresume(struct dm_target *ti)
    {
    let mut r: c_int = 0;
    struct dm_snapshot *s = ti.private;
    struct dm_snapshot *snap_src = core::ptr::null_mut(), *snap_dest = core::ptr::null_mut();
    down_read(&_origins_lock);
    (void) __find_snapshots_sharing_cow(s, &snap_src, &snap_dest, core::ptr::null_mut());
    if (snap_src && snap_dest) {
    down_read(&snap_src.lock);
    if (s == snap_src) {
    DMERR("Unable to resume snapshot source until handover completes.");
    r = -EINVAL;
    } else if (!dm_suspended(snap_src.ti)) {
    DMERR("Unable to perform snapshot handover until source is suspended.");
    r = -EINVAL;
    }
    up_read(&snap_src.lock);
    }
    up_read(&_origins_lock);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn snapshot_resume(ti: *mut dm_target) {
    static void snapshot_resume(struct dm_target *ti)
    {
    struct dm_snapshot *s = ti.private;
    struct dm_snapshot *snap_src = core::ptr::null_mut(), *snap_dest = core::ptr::null_mut(), *snap_merging = core::ptr::null_mut();
    struct dm_origin *o;
    struct mapped_device *origin_md = core::ptr::null_mut();
    let mut must_restart_merging: bool = false;
    down_read(&_origins_lock);
    o = __lookup_dm_origin(s.origin.bdev);
    if (o)
    origin_md = dm_table_get_md(o.ti.table);
    if (!origin_md) {
    (void) __find_snapshots_sharing_cow(s, core::ptr::null_mut(), core::ptr::null_mut(), &snap_merging);
    if (snap_merging)
    origin_md = dm_table_get_md(snap_merging.ti.table);
    }
    if (origin_md == dm_table_get_md(ti.table))
    origin_md = core::ptr::null_mut();
    if (origin_md) {
    if (dm_hold(origin_md))
    origin_md = core::ptr::null_mut();
    }
    up_read(&_origins_lock);
    if (origin_md) {
    dm_internal_suspend_fast(origin_md);
    if (snap_merging && test_bit(RUNNING_MERGE, &snap_merging.state_bits)) {
    must_restart_merging = true;
    stop_merge(snap_merging);
    }
    }
    down_read(&_origins_lock);
    (void) __find_snapshots_sharing_cow(s, &snap_src, &snap_dest, core::ptr::null_mut());
    if (snap_src && snap_dest) {
    down_write(&snap_src.lock);
    down_write_nested(&snap_dest.lock, SINGLE_DEPTH_NESTING);
    __handover_exceptions(snap_src, snap_dest);
    up_write(&snap_dest.lock);
    up_write(&snap_src.lock);
    }
    up_read(&_origins_lock);
    if (origin_md) {
    if (must_restart_merging)
    start_merge(snap_merging);
    dm_internal_resume_fast(origin_md);
    dm_put(origin_md);
    }
// Now we have correct chunk size, reregister
    reregister_snapshot(s);
    down_write(&s.lock);
    s.active = 1;
    up_write(&s.lock);
    }
#[no_mangle]
unsafe extern "C" fn get_origin_minimum_chunksize(bdev: *mut block_device) -> u32 {
    static uint32_t get_origin_minimum_chunksize(struct block_device *bdev)
    {
    uint32_t min_chunksize;
    down_read(&_origins_lock);
    min_chunksize = __minimum_chunk_size(__lookup_origin(bdev));
    up_read(&_origins_lock);
    return min_chunksize;
    }
#[no_mangle]
unsafe extern "C" fn snapshot_merge_resume(ti: *mut dm_target) {
    static void snapshot_merge_resume(struct dm_target *ti)
    {
    struct dm_snapshot *s = ti.private;
//
// Handover exceptions from existing snapshot.
//
    snapshot_resume(ti);
//
// snapshot-merge acts as an origin, so set ti->max_io_len
//
    ti.max_io_len = get_origin_minimum_chunksize(s.origin.bdev);
    start_merge(s);
    }
    static void snapshot_status(struct dm_target *ti, status_type_t type,
    unsigned int status_flags, char *result, unsigned int maxlen)
    {
    let mut sz: c_uint = 0;
    struct dm_snapshot *snap = ti.private;
    unsigned int num_features;
    switch (type) {
    case STATUSTYPE_INFO:
    down_write(&snap.lock);
    if (!snap.valid)
    DMEMIT("Invalid");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: snap->merge_failed) -> else {
    else if (snap.merge_failed)
    DMEMIT("Merge failed");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: snap->snapshot_overflowed) -> else {
    else if (snap.snapshot_overflowed)
    DMEMIT("Overflow");
    else {
    if (snap.store.type.usage) {
    sector_t total_sectors, sectors_allocated,
    metadata_sectors;
    snap.store.type.usage(snap.store,
    &total_sectors,
    &sectors_allocated,
    &metadata_sectors);
    DMEMIT("%llu/%llu %llu",
    (unsigned long long)sectors_allocated,
    (unsigned long long)total_sectors,
    (unsigned long long)metadata_sectors);
    } else
    DMEMIT("Unknown");
    }
    up_write(&snap.lock);
    break;
    case STATUSTYPE_TABLE:
//
// kdevname returns a static pointer so we need
// to make private copies if the output is to
// make sense.
//
    DMEMIT("%s %s", snap.origin.name, snap.cow.name);
    sz += snap.store.type.status(snap.store, type, result + sz,
    maxlen - sz);
    num_features = snap.discard_zeroes_cow + snap.discard_passdown_origin;
    if (num_features) {
    DMEMIT(" %u", num_features);
    if (snap.discard_zeroes_cow)
    DMEMIT(" discard_zeroes_cow");
    if (snap.discard_passdown_origin)
    DMEMIT(" discard_passdown_origin");
    }
    break;
    case STATUSTYPE_IMA:
    DMEMIT_TARGET_NAME_VERSION(ti.type);
    DMEMIT(",snap_origin_name=%s", snap.origin.name);
    DMEMIT(",snap_cow_name=%s", snap.cow.name);
    DMEMIT(",snap_valid=%c", snap.valid ? 'y' : 'n');
    DMEMIT(",snap_merge_failed=%c", snap.merge_failed ? 'y' : 'n');
    DMEMIT(",snapshot_overflowed=%c", snap.snapshot_overflowed ? 'y' : 'n');
    DMEMIT(";");
    break;
    }
    }
    static int snapshot_iterate_devices(struct dm_target *ti,
    iterate_devices_callout_fn fn, void *data)
    {
    struct dm_snapshot *snap = ti.private;
    int r;
    r = fn(ti, snap.origin, 0, ti.len, data);
    if (!r)
    r = fn(ti, snap.cow, 0, get_dev_size(snap.cow.bdev), data);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn snapshot_io_hints(ti: *mut dm_target, limits: *mut queue_limits) {
    static void snapshot_io_hints(struct dm_target *ti, struct queue_limits *limits)
    {
    struct dm_snapshot *snap = ti.private;
    if (snap.discard_zeroes_cow) {
    struct dm_snapshot *snap_src = core::ptr::null_mut(), *snap_dest = core::ptr::null_mut();
    down_read(&_origins_lock);
    (void) __find_snapshots_sharing_cow(snap, &snap_src, &snap_dest, core::ptr::null_mut());
    if (snap_src && snap_dest)
    snap = snap_src;
// All discards are split on chunk_size boundary
    limits.discard_granularity = snap.store.chunk_size;
    limits.max_hw_discard_sectors = snap.store.chunk_size;
    up_read(&_origins_lock);
    }
    }
//
// ---------------------------------------------------------------
// Origin methods
// ---------------------------------------------------------------
//
// If no exceptions need creating, DM_MAPIO_REMAPPED is returned and any
// supplied bio was ignored.  The caller may submit it immediately.
// (No remapping actually occurs as the origin is always a direct linear
// map.)
//
// If further exceptions are required, DM_MAPIO_SUBMITTED is returned
// and any supplied bio is added to a list to be submitted once all
// the necessary exceptions exist.
//
    static int __origin_write(struct list_head *snapshots, sector_t sector,
    struct bio *bio)
    {
    let mut r: c_int = DM_MAPIO_REMAPPED;
    struct dm_snapshot *snap;
    struct dm_exception *e;
    struct dm_snap_pending_exception *pe, *pe2;
    struct dm_snap_pending_exception *pe_to_start_now = core::ptr::null_mut();
    struct dm_snap_pending_exception *pe_to_start_last = core::ptr::null_mut();
    struct dm_exception_table_lock lock;
    chunk_t chunk;
// Do all the snapshots on this origin
    list_for_each_entry(snap, snapshots, list) {
//
// Don't make new exceptions in a merging snapshot
// because it has effectively been deleted
//
    if (dm_target_is_snapshot_merge(snap.ti))
    continue;
// Nothing to do if writing beyond end of snapshot
    if (sector >= dm_table_get_size(snap.ti.table))
    continue;
//
// Remember, different snapshots can have
// different chunk sizes.
//
    chunk = sector_to_chunk(snap.store, sector);
    dm_exception_table_lock_init(snap, chunk, &lock);
    down_read(&snap.lock);
    dm_exception_table_lock(&lock);
// Only deal with valid and active snapshots
    if (!snap.valid || !snap.active)
    goto next_snapshot;
    pe = __lookup_pending_exception(snap, chunk);
    if (!pe) {
//
// Check exception table to see if block is already
// remapped in this snapshot and trigger an exception
// if not.
//
    e = dm_lookup_exception(&snap.complete, chunk);
    if (e)
    goto next_snapshot;
    dm_exception_table_unlock(&lock);
    pe = alloc_pending_exception(snap);
    dm_exception_table_lock(&lock);
    pe2 = __lookup_pending_exception(snap, chunk);
    if (!pe2) {
    e = dm_lookup_exception(&snap.complete, chunk);
    if (e) {
    free_pending_exception(pe);
    goto next_snapshot;
    }
    pe = __insert_pending_exception(snap, pe, chunk);
    if (!pe) {
    dm_exception_table_unlock(&lock);
    up_read(&snap.lock);
    invalidate_snapshot(snap, -ENOMEM);
    continue;
    }
    } else {
    free_pending_exception(pe);
    pe = pe2;
    }
    }
    r = DM_MAPIO_SUBMITTED;
//
// If an origin bio was supplied, queue it to wait for the
// completion of this exception, and start this one last,
// at the end of the function.
//
    if (bio) {
    bio_list_add(&pe.origin_bios, bio);
    bio = core::ptr::null_mut();
    if (!pe.started) {
    pe.started = 1;
    pe_to_start_last = pe;
    }
    }
    if (!pe.started) {
    pe.started = 1;
    pe_to_start_now = pe;
    }
    next_snapshot:
    dm_exception_table_unlock(&lock);
    up_read(&snap.lock);
    if (pe_to_start_now) {
    start_copy(pe_to_start_now);
    pe_to_start_now = core::ptr::null_mut();
    }
    }
//
// Submit the exception against which the bio is queued last,
// to give the other exceptions a head start.
//
    if (pe_to_start_last)
    start_copy(pe_to_start_last);
    return r;
    }
//
// Called on a write from the origin driver.
//
#[no_mangle]
unsafe extern "C" fn do_origin(origin: *mut dm_dev, bio: *mut bio, limit: bool) -> c_int {
    static int do_origin(struct dm_dev *origin, struct bio *bio, bool limit)
    {
    struct origin *o;
    let mut r: c_int = DM_MAPIO_REMAPPED;
    again:
    down_read(&_origins_lock);
    o = __lookup_origin(origin.bdev);
    if (o) {
    if (limit) {
    struct dm_snapshot *s;
    list_for_each_entry(s, &o.snapshots, list)
    if (unlikely(!wait_for_in_progress(s, true)))
    goto again;
    }
    r = __origin_write(&o.snapshots, bio.bi_iter.bi_sector, bio);
    }
    up_read(&_origins_lock);
    return r;
    }
//
// Trigger exceptions in all non-merging snapshots.
//
// The chunk size of the merging snapshot may be larger than the chunk
// size of some other snapshot so we may need to reallocate multiple
// chunks in other snapshots.
//
// We scan all the overlapping exceptions in the other snapshots.
// Returns 1 if anything was reallocated and must be waited for,
// otherwise returns 0.
//
// size must be a multiple of merging_snap's chunk_size.
//
    static int origin_write_extent(struct dm_snapshot *merging_snap,
    sector_t sector, unsigned int size)
    {
    let mut must_wait: c_int = 0;
    sector_t n;
    struct origin *o;
//
// The origin's __minimum_chunk_size() got stored in max_io_len
// by snapshot_merge_resume().
//
    down_read(&_origins_lock);
    o = __lookup_origin(merging_snap.origin.bdev);
    for (n = 0; n < size; n += merging_snap.ti.max_io_len)
    if (__origin_write(&o.snapshots, sector + n, core::ptr::null_mut()) ==
    DM_MAPIO_SUBMITTED)
    must_wait = 1;
    up_read(&_origins_lock);
    return must_wait;
    }
//
// Origin: maps a linear range of a device, with hooks for snapshotting.
//
// Construct an origin mapping: <dev_path>
// The context for an origin is merely a 'struct dm_dev *'
// pointing to the real device.
//
#[no_mangle]
unsafe extern "C" fn origin_ctr(ti: *mut dm_target, argc: c_uint, argv: *mut c_char) -> c_int {
    static int origin_ctr(struct dm_target *ti, unsigned int argc, char **argv)
    {
    int r;
    struct dm_origin *o;
    if (argc != 1) {
    ti.error = "origin: incorrect number of arguments";
    return -EINVAL;
    }
    o = kmalloc_obj(struct dm_origin);
    if (!o) {
    ti.error = "Cannot allocate private origin structure";
    r = -ENOMEM;
    goto bad_alloc;
    }
    r = dm_get_device(ti, argv[0], dm_table_get_mode(ti.table), &o.dev);
    if (r) {
    ti.error = "Cannot get target device";
    goto bad_open;
    }
    o.ti = ti;
    ti.private = o;
    ti.num_flush_bios = 1;
    return 0;
    bad_open:
    kfree(o);
    bad_alloc:
    return r;
    }
#[no_mangle]
unsafe extern "C" fn origin_dtr(ti: *mut dm_target) {
    static void origin_dtr(struct dm_target *ti)
    {
    struct dm_origin *o = ti.private;
    dm_put_device(ti, o.dev);
    kfree(o);
    }
#[no_mangle]
unsafe extern "C" fn origin_map(ti: *mut dm_target, bio: *mut bio) -> c_int {
    static int origin_map(struct dm_target *ti, struct bio *bio)
    {
    struct dm_origin *o = ti.private;
    unsigned int available_sectors;
    bio_set_dev(bio, o.dev.bdev);
    if (unlikely(bio.bi_opf & REQ_PREFLUSH))
    return DM_MAPIO_REMAPPED;
    if (bio_data_dir(bio) != WRITE)
    return DM_MAPIO_REMAPPED;
    available_sectors = o.split_boundary -
    ((unsigned int)bio.bi_iter.bi_sector & (o.split_boundary - 1));
    if (bio_sectors(bio) > available_sectors)
    dm_accept_partial_bio(bio, available_sectors);
// Only tell snapshots if this is a write
    return do_origin(o.dev, bio, true);
    }
//
// Set the target "max_io_len" field to the minimum of all the snapshots'
// chunk sizes.
//
#[no_mangle]
unsafe extern "C" fn origin_resume(ti: *mut dm_target) {
    static void origin_resume(struct dm_target *ti)
    {
    struct dm_origin *o = ti.private;
    o.split_boundary = get_origin_minimum_chunksize(o.dev.bdev);
    down_write(&_origins_lock);
    __insert_dm_origin(o);
    up_write(&_origins_lock);
    }
#[no_mangle]
unsafe extern "C" fn origin_postsuspend(ti: *mut dm_target) {
    static void origin_postsuspend(struct dm_target *ti)
    {
    struct dm_origin *o = ti.private;
    down_write(&_origins_lock);
    __remove_dm_origin(o);
    up_write(&_origins_lock);
    }
    static void origin_status(struct dm_target *ti, status_type_t type,
    unsigned int status_flags, char *result, unsigned int maxlen)
    {
    struct dm_origin *o = ti.private;
    switch (type) {
    case STATUSTYPE_INFO:
    result[0] = '\0';
    break;
    case STATUSTYPE_TABLE:
    snprintf(result, maxlen, "%s", o.dev.name);
    break;
    case STATUSTYPE_IMA:
    result[0] = '\0';
    break;
    }
    }
    static int origin_iterate_devices(struct dm_target *ti,
    iterate_devices_callout_fn fn, void *data)
    {
    struct dm_origin *o = ti.private;
    return fn(ti, o.dev, 0, ti.len, data);
    }
    static struct target_type origin_target = {
    .name    = "snapshot-origin",
    .version = {1, 9, 0},
    .module  = THIS_MODULE,
    .ctr     = origin_ctr,
    .dtr     = origin_dtr,
    .map     = origin_map,
    .resume  = origin_resume,
    .postsuspend = origin_postsuspend,
    .status  = origin_status,
    .iterate_devices = origin_iterate_devices,
    };
    static struct target_type snapshot_target = {
    .name    = "snapshot",
    .version = {1, 16, 0},
    .module  = THIS_MODULE,
    .ctr     = snapshot_ctr,
    .dtr     = snapshot_dtr,
    .map     = snapshot_map,
    .end_io  = snapshot_end_io,
    .preresume  = snapshot_preresume,
    .resume  = snapshot_resume,
    .status  = snapshot_status,
    .iterate_devices = snapshot_iterate_devices,
    .io_hints = snapshot_io_hints,
    };
    static struct target_type merge_target = {
    .name    = dm_snapshot_merge_target_name,
    .version = {1, 5, 0},
    .module  = THIS_MODULE,
    .ctr     = snapshot_ctr,
    .dtr     = snapshot_dtr,
    .map     = snapshot_merge_map,
    .end_io  = snapshot_end_io,
    .presuspend = snapshot_merge_presuspend,
    .preresume  = snapshot_preresume,
    .resume  = snapshot_merge_resume,
    .status  = snapshot_status,
    .iterate_devices = snapshot_iterate_devices,
    .io_hints = snapshot_io_hints,
    };
#[no_mangle]
unsafe extern "C" fn dm_snapshot_init() -> int __init {
    static int __init dm_snapshot_init(void)
    {
    int r;
    r = dm_exception_store_init();
    if (r) {
    DMERR("Failed to initialize exception stores");
    return r;
    }
    r = init_origin_hash();
    if (r) {
    DMERR("init_origin_hash failed.");
    goto bad_origin_hash;
    }
    exception_cache = KMEM_CACHE(dm_exception, 0);
    if (!exception_cache) {
    DMERR("Couldn't create exception cache.");
    r = -ENOMEM;
    goto bad_exception_cache;
    }
    pending_cache = KMEM_CACHE(dm_snap_pending_exception, 0);
    if (!pending_cache) {
    DMERR("Couldn't create pending cache.");
    r = -ENOMEM;
    goto bad_pending_cache;
    }
    r = dm_register_target(&snapshot_target);
    if (r < 0)
    goto bad_register_snapshot_target;
    r = dm_register_target(&origin_target);
    if (r < 0)
    goto bad_register_origin_target;
    r = dm_register_target(&merge_target);
    if (r < 0)
    goto bad_register_merge_target;
    return 0;
    bad_register_merge_target:
    dm_unregister_target(&origin_target);
    bad_register_origin_target:
    dm_unregister_target(&snapshot_target);
    bad_register_snapshot_target:
    kmem_cache_destroy(pending_cache);
    bad_pending_cache:
    kmem_cache_destroy(exception_cache);
    bad_exception_cache:
    exit_origin_hash();
    bad_origin_hash:
    dm_exception_store_exit();
    return r;
    }
#[no_mangle]
unsafe extern "C" fn dm_snapshot_exit() -> void __exit {
    static void __exit dm_snapshot_exit(void)
    {
    dm_unregister_target(&snapshot_target);
    dm_unregister_target(&origin_target);
    dm_unregister_target(&merge_target);
    exit_origin_hash();
    kmem_cache_destroy(pending_cache);
    kmem_cache_destroy(exception_cache);
    dm_exception_store_exit();
    }
// Module hooks
    module_init(dm_snapshot_init);
    module_exit(dm_snapshot_exit);
    MODULE_DESCRIPTION(DM_NAME " snapshot target");
    MODULE_AUTHOR("Joe Thornber");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("dm-snapshot-origin");
    MODULE_ALIAS("dm-snapshot-merge");
