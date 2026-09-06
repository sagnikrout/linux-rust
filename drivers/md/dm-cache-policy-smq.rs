//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-cache-policy-smq.c
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
// Copyright (C) 2015 Red Hat. All rights reserved.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
//
// Maximum number of concurrent background work items (promotions,
// demotions, writebacks) that can be queued in the background tracker.
// Tuneable via the module parameter smq_max_background_work.
// Only affects newly created cache devices.
//
    let mut smq_max_background_work: static unsigned int = 4096;
    module_param(smq_max_background_work, uint, 0644);
    MODULE_PARM_DESC(smq_max_background_work, "Max concurrent background work items");
//
// Safe division functions that return zero on divide by zero.
//
#[no_mangle]
unsafe extern "C" fn safe_div(n: c_uint, d: c_uint) -> c_uint {
    static unsigned int safe_div(unsigned int n, unsigned int d)
    {
    return d ? n / d : 0u;
    }
#[no_mangle]
unsafe extern "C" fn safe_mod(n: c_uint, d: c_uint) -> c_uint {
    static unsigned int safe_mod(unsigned int n, unsigned int d)
    {
    return d ? n % d : 0u;
    }
// ----------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct entry {
    pub hash_next:28: c_uint,
    pub prev:28: c_uint,
    pub next:28: c_uint,
    pub level:6: c_uint,
    pub dirty:1: bool,
    pub allocated:1: bool,
    pub sentinel:1: bool,
    pub pending_work:1: bool,
    pub oblock: dm_oblock_t,
}

// ----------------------------------------------------------------

//
// An entry_space manages a set of entries that we use for the queues.
// The clean and dirty queues share entries, so this object is separate
// from the queue itself.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct entry_space {
    pub begin: *mut entry,
    pub end: *mut entry,
}

#[no_mangle]
unsafe extern "C" fn space_init(es: *mut entry_space, nr_entries: c_uint) -> c_int {
    static int space_init(struct entry_space *es, unsigned int nr_entries)
    {
    if (!nr_entries) {
    es.begin = es.end = core::ptr::null_mut();
    return 0;
    }
    es.begin = vzalloc(array_size(nr_entries, sizeof(struct entry)));
    if (!es.begin)
    return -ENOMEM;
    es.end = es.begin + nr_entries;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn space_exit(es: *mut entry_space) {
    static void space_exit(struct entry_space *es)
    {
    vfree(es.begin);
    }
    static struct entry *__get_entry(struct entry_space *es, unsigned int block)
    {
    struct entry *e;
    e = es.begin + block;
    BUG_ON(e >= es.end);
    return e;
    }
#[no_mangle]
unsafe extern "C" fn to_index(es: *mut entry_space, e: *mut entry) -> c_uint {
    static unsigned int to_index(struct entry_space *es, struct entry *e)
    {
    BUG_ON(e < es.begin || e >= es.end);
    return e - es.begin;
    }
    static struct entry *to_entry(struct entry_space *es, unsigned int block)
    {
    if (block == INDEXER_NULL)
    return core::ptr::null_mut();
    return __get_entry(es, block);
    }
// ----------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ilist {
    pub /: *mut *mut unsigned int nr_elts; / excluding sentinel entries,
    pub tail: unsigned int head,,
}

#[no_mangle]
unsafe extern "C" fn l_init(l: *mut ilist) {
    static void l_init(struct ilist *l)
    {
    l.nr_elts = 0;
    l.head = l.tail = INDEXER_NULL;
    }
    static struct entry *l_head(struct entry_space *es, struct ilist *l)
    {
    return to_entry(es, l.head);
    }
    static struct entry *l_tail(struct entry_space *es, struct ilist *l)
    {
    return to_entry(es, l.tail);
    }
    static struct entry *l_next(struct entry_space *es, struct entry *e)
    {
    return to_entry(es, e.next);
    }
    static struct entry *l_prev(struct entry_space *es, struct entry *e)
    {
    return to_entry(es, e.prev);
    }
#[no_mangle]
unsafe extern "C" fn l_empty(l: *mut ilist) -> bool {
    static bool l_empty(struct ilist *l)
    {
    return l.head == INDEXER_NULL;
    }
#[no_mangle]
unsafe extern "C" fn l_add_head(es: *mut entry_space, l: *mut ilist, e: *mut entry) {
    static void l_add_head(struct entry_space *es, struct ilist *l, struct entry *e)
    {
    struct entry *head = l_head(es, l);
    e.next = l.head;
    e.prev = INDEXER_NULL;
    if (head)
    head.prev = l.head = to_index(es, e);
    else
    l.head = l.tail = to_index(es, e);
    if (!e.sentinel)
    l.nr_elts++;
    }
#[no_mangle]
unsafe extern "C" fn l_add_tail(es: *mut entry_space, l: *mut ilist, e: *mut entry) {
    static void l_add_tail(struct entry_space *es, struct ilist *l, struct entry *e)
    {
    struct entry *tail = l_tail(es, l);
    e.next = INDEXER_NULL;
    e.prev = l.tail;
    if (tail)
    tail.next = l.tail = to_index(es, e);
    else
    l.head = l.tail = to_index(es, e);
    if (!e.sentinel)
    l.nr_elts++;
    }
    static void l_add_before(struct entry_space *es, struct ilist *l,
    struct entry *old, struct entry *e)
    {
    struct entry *prev = l_prev(es, old);
    if (!prev)
    l_add_head(es, l, e);
    else {
    e.prev = old.prev;
    e.next = to_index(es, old);
    prev.next = old.prev = to_index(es, e);
    if (!e.sentinel)
    l.nr_elts++;
    }
    }
#[no_mangle]
unsafe extern "C" fn l_del(es: *mut entry_space, l: *mut ilist, e: *mut entry) {
    static void l_del(struct entry_space *es, struct ilist *l, struct entry *e)
    {
    struct entry *prev = l_prev(es, e);
    struct entry *next = l_next(es, e);
    if (prev)
    prev.next = e.next;
    else
    l.head = e.next;
    if (next)
    next.prev = e.prev;
    else
    l.tail = e.prev;
    if (!e.sentinel)
    l.nr_elts--;
    }
    static struct entry *l_pop_head(struct entry_space *es, struct ilist *l)
    {
    struct entry *e;
    for (e = l_head(es, l); e; e = l_next(es, e))
    if (!e.sentinel) {
    l_del(es, l, e);
    return e;
    }
    return core::ptr::null_mut();
    }
    static struct entry *l_pop_tail(struct entry_space *es, struct ilist *l)
    {
    struct entry *e;
    for (e = l_tail(es, l); e; e = l_prev(es, e))
    if (!e.sentinel) {
    l_del(es, l, e);
    return e;
    }
    return core::ptr::null_mut();
    }
// ----------------------------------------------------------------
//
// The stochastic-multi-queue is a set of lru lists stacked into levels.
// Entries are moved up levels when they are used, which loosely orders the
// most accessed entries in the top levels and least in the bottom.  This
// structure is *much* better than a single lru list.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue {
    pub es: *mut entry_space,
    pub nr_elts: c_uint,
    pub nr_levels: c_uint,
    pub qs: [ilist; MAX_LEVELS],
//
// We maintain a count of the number of entries we would like in each
// level.
//
    pub last_target_nr_elts: c_uint,
    pub nr_top_levels: c_uint,
    pub nr_in_top_levels: c_uint,
    pub target_count: [c_uint; MAX_LEVELS],
}

#[no_mangle]
unsafe extern "C" fn q_init(q: *mut queue, es: *mut entry_space, nr_levels: c_uint) {
    static void q_init(struct queue *q, struct entry_space *es, unsigned int nr_levels)
    {
    unsigned int i;
    q.es = es;
    q.nr_elts = 0;
    q.nr_levels = nr_levels;
    for (i = 0; i < q.nr_levels; i++) {
    l_init(q.qs + i);
    q.target_count[i] = 0u;
    }
    q.last_target_nr_elts = 0u;
    q.nr_top_levels = 0u;
    q.nr_in_top_levels = 0u;
    }
#[no_mangle]
unsafe extern "C" fn q_size(q: *mut queue) -> c_uint {
    static unsigned int q_size(struct queue *q)
    {
    return q.nr_elts;
    }
//
// Insert an entry to the back of the given level.
//
#[no_mangle]
unsafe extern "C" fn q_push(q: *mut queue, e: *mut entry) {
    static void q_push(struct queue *q, struct entry *e)
    {
    BUG_ON(e.pending_work);
    if (!e.sentinel)
    q.nr_elts++;
    l_add_tail(q.es, q.qs + e.level, e);
    }
#[no_mangle]
unsafe extern "C" fn q_push_front(q: *mut queue, e: *mut entry) {
    static void q_push_front(struct queue *q, struct entry *e)
    {
    BUG_ON(e.pending_work);
    if (!e.sentinel)
    q.nr_elts++;
    l_add_head(q.es, q.qs + e.level, e);
    }
#[no_mangle]
unsafe extern "C" fn q_push_before(q: *mut queue, old: *mut entry, e: *mut entry) {
    static void q_push_before(struct queue *q, struct entry *old, struct entry *e)
    {
    BUG_ON(e.pending_work);
    if (!e.sentinel)
    q.nr_elts++;
    l_add_before(q.es, q.qs + e.level, old, e);
    }
#[no_mangle]
unsafe extern "C" fn q_del(q: *mut queue, e: *mut entry) {
    static void q_del(struct queue *q, struct entry *e)
    {
    l_del(q.es, q.qs + e.level, e);
    if (!e.sentinel)
    q.nr_elts--;
    }
//
// Return the oldest entry of the lowest populated level.
//
    static struct entry *q_peek(struct queue *q, unsigned int max_level, bool can_cross_sentinel)
    {
    unsigned int level;
    struct entry *e;
    max_level = min(max_level, q.nr_levels);
    for (level = 0; level < max_level; level++)
    for (e = l_head(q.es, q.qs + level); e; e = l_next(q.es, e)) {
    if (e.sentinel) {
    if (can_cross_sentinel)
    continue;
    else
    break;
    }
    return e;
    }
    return core::ptr::null_mut();
    }
    static struct entry *q_pop(struct queue *q)
    {
    struct entry *e = q_peek(q, q.nr_levels, true);
    if (e)
    q_del(q, e);
    return e;
    }
//
// This function assumes there is a non-sentinel entry to pop.  It's only
// used by redistribute, so we know this is true.  It also doesn't adjust
// the q->nr_elts count.
//
    static struct entry *__redist_pop_from(struct queue *q, unsigned int level)
    {
    struct entry *e;
    for (; level < q.nr_levels; level++)
    for (e = l_head(q.es, q.qs + level); e; e = l_next(q.es, e))
    if (!e.sentinel) {
    l_del(q.es, q.qs + e.level, e);
    return e;
    }
    return core::ptr::null_mut();
    }
    static void q_set_targets_subrange_(struct queue *q, unsigned int nr_elts,
    unsigned int lbegin, unsigned int lend)
    {
    unsigned int level, nr_levels, entries_per_level, remainder;
    BUG_ON(lbegin > lend);
    BUG_ON(lend > q.nr_levels);
    nr_levels = lend - lbegin;
    entries_per_level = safe_div(nr_elts, nr_levels);
    remainder = safe_mod(nr_elts, nr_levels);
    for (level = lbegin; level < lend; level++)
    q.target_count[level] =
    (level < (lbegin + remainder)) ? entries_per_level + 1u : entries_per_level;
    }
//
// Typically we have fewer elements in the top few levels which allows us
// to adjust the promote threshold nicely.
//
#[no_mangle]
unsafe extern "C" fn q_set_targets(q: *mut queue) {
    static void q_set_targets(struct queue *q)
    {
    if (q.last_target_nr_elts == q.nr_elts)
    return;
    q.last_target_nr_elts = q.nr_elts;
    if (q.nr_top_levels > q.nr_levels)
    q_set_targets_subrange_(q, q.nr_elts, 0, q.nr_levels);
    else {
    q_set_targets_subrange_(q, q.nr_in_top_levels,
    q.nr_levels - q.nr_top_levels, q.nr_levels);
    if (q.nr_in_top_levels < q.nr_elts)
    q_set_targets_subrange_(q, q.nr_elts - q.nr_in_top_levels,
    0, q.nr_levels - q.nr_top_levels);
    else
    q_set_targets_subrange_(q, 0, 0, q.nr_levels - q.nr_top_levels);
    }
    }
#[no_mangle]
unsafe extern "C" fn q_redistribute(q: *mut queue) {
    static void q_redistribute(struct queue *q)
    {
    unsigned int target, level;
    struct ilist *l, *l_above;
    struct entry *e;
    q_set_targets(q);
    for (level = 0u; level < q.nr_levels - 1u; level++) {
    l = q.qs + level;
    target = q.target_count[level];
//
// Pull down some entries from the level above.
//
    while (l.nr_elts < target) {
    e = __redist_pop_from(q, level + 1u);
    if (!e) {
// bug in nr_elts
    break;
    }
    e.level = level;
    l_add_tail(q.es, l, e);
    }
//
// Push some entries up.
//
    l_above = q.qs + level + 1u;
    while (l.nr_elts > target) {
    e = l_pop_tail(q.es, l);
    if (!e)
// bug in nr_elts
    break;
    e.level = level + 1u;
    l_add_tail(q.es, l_above, e);
    }
    }
    }
    static void q_requeue(struct queue *q, struct entry *e, unsigned int extra_levels,
    struct entry *s1, struct entry *s2)
    {
    struct entry *de;
    let mut sentinels_passed: c_uint = 0;
    let mut new_level: c_uint = min(q.nr_levels - 1u, e.level + extra_levels);
// try and find an entry to swap with
    if (extra_levels && (e.level < q.nr_levels - 1u)) {
    for (de = l_head(q.es, q.qs + new_level); de && de.sentinel; de = l_next(q.es, de))
    sentinels_passed++;
    if (de) {
    q_del(q, de);
    de.level = e.level;
    if (s1) {
    switch (sentinels_passed) {
    case 0:
    q_push_before(q, s1, de);
    break;
    case 1:
    q_push_before(q, s2, de);
    break;
    default:
    q_push(q, de);
    }
    } else
    q_push(q, de);
    }
    }
    q_del(q, e);
    e.level = new_level;
    q_push(q, e);
    }
// ----------------------------------------------------------------
pub const FP_SHIFT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats {
    pub hit_threshold: c_uint,
    pub hits: c_uint,
    pub misses: c_uint,
}

    enum performance {
    Q_POOR,
    Q_FAIR,
    Q_WELL
    };
#[no_mangle]
unsafe extern "C" fn stats_init(s: *mut stats, nr_levels: c_uint) {
    static void stats_init(struct stats *s, unsigned int nr_levels)
    {
    s.hit_threshold = (nr_levels * 3u) / 4u;
    s.hits = 0u;
    s.misses = 0u;
    }
#[no_mangle]
unsafe extern "C" fn stats_reset(s: *mut stats) {
    static void stats_reset(struct stats *s)
    {
    s.hits = s.misses = 0u;
    }
#[no_mangle]
unsafe extern "C" fn stats_level_accessed(s: *mut stats, level: c_uint) {
    static void stats_level_accessed(struct stats *s, unsigned int level)
    {
    if (level >= s.hit_threshold)
    s.hits++;
    else
    s.misses++;
    }
#[no_mangle]
unsafe extern "C" fn stats_miss(s: *mut stats) {
    static void stats_miss(struct stats *s)
    {
    s.misses++;
    }
//
// There are times when we don't have any confidence in the hotspot queue.
// Such as when a fresh cache is created and the blocks have been spread
// out across the levels, or if an io load changes.  We detect this by
// seeing how often a lookup is in the top levels of the hotspot queue.
//
#[no_mangle]
unsafe extern "C" fn stats_assess(s: *mut stats) -> enum performance {
    static enum performance stats_assess(struct stats *s)
    {
    let mut confidence: c_uint = safe_div(s.hits << FP_SHIFT, s.hits + s.misses);
    if (confidence < SIXTEENTH)
    return Q_POOR;
#[no_mangle]
pub unsafe extern "C" fn if(EIGHTH: confidence <) -> else {
    else if (confidence < EIGHTH)
    return Q_FAIR;
    else
    return Q_WELL;
    }
// ----------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smq_hash_table {
    pub es: *mut entry_space,
    pub hash_bits: c_ulonglong,
    pub buckets: *mut c_uint,
}

//
// All cache entries are stored in a chained hash table.  To save space we
// use indexing again, and only store indexes to the next entry.
//
#[no_mangle]
unsafe extern "C" fn h_init(ht: *mut smq_hash_table, es: *mut entry_space, nr_entries: c_uint) -> c_int {
    static int h_init(struct smq_hash_table *ht, struct entry_space *es, unsigned int nr_entries)
    {
    unsigned int i, nr_buckets;
    ht.es = es;
    nr_buckets = roundup_pow_of_two(max(nr_entries / 4u, 16u));
    ht.hash_bits = __ffs(nr_buckets);
    ht.buckets = vmalloc_array(nr_buckets, sizeof(*ht.buckets));
    if (!ht.buckets)
    return -ENOMEM;
    for (i = 0; i < nr_buckets; i++)
    ht.buckets[i] = INDEXER_NULL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn h_exit(ht: *mut smq_hash_table) {
    static void h_exit(struct smq_hash_table *ht)
    {
    vfree(ht.buckets);
    }
    static struct entry *h_head(struct smq_hash_table *ht, unsigned int bucket)
    {
    return to_entry(ht.es, ht.buckets[bucket]);
    }
    static struct entry *h_next(struct smq_hash_table *ht, struct entry *e)
    {
    return to_entry(ht.es, e.hash_next);
    }
#[no_mangle]
unsafe extern "C" fn __h_insert(ht: *mut smq_hash_table, bucket: c_uint, e: *mut entry) {
    static void __h_insert(struct smq_hash_table *ht, unsigned int bucket, struct entry *e)
    {
    e.hash_next = ht.buckets[bucket];
    ht.buckets[bucket] = to_index(ht.es, e);
    }
#[no_mangle]
unsafe extern "C" fn h_insert(ht: *mut smq_hash_table, e: *mut entry) {
    static void h_insert(struct smq_hash_table *ht, struct entry *e)
    {
    let mut h: c_uint = hash_64(from_oblock(e.oblock), ht.hash_bits);
    __h_insert(ht, h, e);
    }
    static struct entry *__h_lookup(struct smq_hash_table *ht, unsigned int h, dm_oblock_t oblock,
    struct entry **prev)
    {
    struct entry *e;
// prev = NULL;
    for (e = h_head(ht, h); e; e = h_next(ht, e)) {
    if (e.oblock == oblock)
    return e;
// prev = e;
    }
    return core::ptr::null_mut();
    }
    static void __h_unlink(struct smq_hash_table *ht, unsigned int h,
    struct entry *e, struct entry *prev)
    {
    if (prev)
    prev.hash_next = e.hash_next;
    else
    ht.buckets[h] = e.hash_next;
    }
//
// Also moves each entry to the front of the bucket.
//
    static struct entry *h_lookup(struct smq_hash_table *ht, dm_oblock_t oblock)
    {
    struct entry *e, *prev;
    let mut h: c_uint = hash_64(from_oblock(oblock), ht.hash_bits);
    e = __h_lookup(ht, h, oblock, &prev);
    if (e && prev) {
//
// Move to the front because this entry is likely
// to be hit again.
//
    __h_unlink(ht, h, e, prev);
    __h_insert(ht, h, e);
    }
    return e;
    }
#[no_mangle]
unsafe extern "C" fn h_remove(ht: *mut smq_hash_table, e: *mut entry) {
    static void h_remove(struct smq_hash_table *ht, struct entry *e)
    {
    let mut h: c_uint = hash_64(from_oblock(e.oblock), ht.hash_bits);
    struct entry *prev;
//
// The down side of using a singly linked list is we have to
// iterate the bucket to remove an item.
//
    e = __h_lookup(ht, h, e.oblock, &prev);
    if (e)
    __h_unlink(ht, h, e, prev);
    }
// ----------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct entry_alloc {
    pub es: *mut entry_space,
    pub begin: c_uint,
    pub nr_allocated: c_uint,
    pub free: ilist,
}

    static void init_allocator(struct entry_alloc *ea, struct entry_space *es,
    unsigned int begin, unsigned int end)
    {
    unsigned int i;
    ea.es = es;
    ea.nr_allocated = 0u;
    ea.begin = begin;
    l_init(&ea.free);
    for (i = begin; i != end; i++)
    l_add_tail(ea.es, &ea.free, __get_entry(ea.es, i));
    }
#[no_mangle]
unsafe extern "C" fn init_entry(e: *mut entry) {
    static void init_entry(struct entry *e)
    {
//
// We can't memset because that would clear the hotspot and
// sentinel bits which remain constant.
//
    e.hash_next = INDEXER_NULL;
    e.next = INDEXER_NULL;
    e.prev = INDEXER_NULL;
    e.level = 0u;
    e.dirty = true;	/* FIXME: audit */
    e.allocated = true;
    e.sentinel = false;
    e.pending_work = false;
    }
    static struct entry *alloc_entry(struct entry_alloc *ea)
    {
    struct entry *e;
    if (l_empty(&ea.free))
    return core::ptr::null_mut();
    e = l_pop_head(ea.es, &ea.free);
    init_entry(e);
    ea.nr_allocated++;
    return e;
    }
//
// This assumes the cblock hasn't already been allocated.
//
    static struct entry *alloc_particular_entry(struct entry_alloc *ea, unsigned int i)
    {
    struct entry *e = __get_entry(ea.es, ea.begin + i);
    BUG_ON(e.allocated);
    l_del(ea.es, &ea.free, e);
    init_entry(e);
    ea.nr_allocated++;
    return e;
    }
#[no_mangle]
unsafe extern "C" fn free_entry(ea: *mut entry_alloc, e: *mut entry) {
    static void free_entry(struct entry_alloc *ea, struct entry *e)
    {
    BUG_ON(!ea.nr_allocated);
    BUG_ON(!e.allocated);
    ea.nr_allocated--;
    e.allocated = false;
    l_add_tail(ea.es, &ea.free, e);
    }
#[no_mangle]
unsafe extern "C" fn allocator_empty(ea: *mut entry_alloc) -> bool {
    static bool allocator_empty(struct entry_alloc *ea)
    {
    return l_empty(&ea.free);
    }
#[no_mangle]
unsafe extern "C" fn get_index(ea: *mut entry_alloc, e: *mut entry) -> c_uint {
    static unsigned int get_index(struct entry_alloc *ea, struct entry *e)
    {
    return to_index(ea.es, e) - ea.begin;
    }
    static struct entry *get_entry(struct entry_alloc *ea, unsigned int index)
    {
    return __get_entry(ea.es, ea.begin + index);
    }
// ----------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smq_policy {
    pub policy: dm_cache_policy,
// protects everything
    pub lock: spinlock_t,
    pub cache_size: dm_cblock_t,
    pub cache_block_size: sector_t,
    pub hotspot_block_size: sector_t,
    pub nr_hotspot_blocks: c_uint,
    pub cache_blocks_per_hotspot_block: c_uint,
    pub hotspot_level_jump: c_uint,
    pub es: entry_space,
    pub writeback_sentinel_alloc: entry_alloc,
    pub demote_sentinel_alloc: entry_alloc,
    pub hotspot_alloc: entry_alloc,
    pub cache_alloc: entry_alloc,
    pub hotspot_hit_bits: *mut c_ulong,
    pub cache_hit_bits: *mut c_ulong,
//
// We maintain three queues of entries.  The cache proper,
// consisting of a clean and dirty queue, containing the currently
// active mappings.  The hotspot queue uses a larger block size to
// track blocks that are being hit frequently and potential
// candidates for promotion to the cache.
//
    pub hotspot: queue,
    pub clean: queue,
    pub dirty: queue,
    pub hotspot_stats: stats,
    pub cache_stats: stats,
//
// Keeps track of time, incremented by the core.  We use this to
// avoid attributing multiple hits within the same tick.
//
    pub tick: c_uint,
//
// The hash tables allows us to quickly find an entry by origin
// block.
//
    pub table: smq_hash_table,
    pub hotspot_table: smq_hash_table,
    pub current_writeback_sentinels: bool,
    pub next_writeback_period: c_ulong,
    pub current_demote_sentinels: bool,
    pub next_demote_period: c_ulong,
    pub write_promote_level: c_uint,
    pub read_promote_level: c_uint,
    pub next_hotspot_period: c_ulong,
    pub next_cache_period: c_ulong,
    pub bg_work: *mut background_tracker,
    pub migrations_allowed:1: bool,
//
// If this is set the policy will try and clean the whole cache
// even if the device is not idle.
//
    pub cleaner:1: bool,
}

// ----------------------------------------------------------------
    static struct entry *get_sentinel(struct entry_alloc *ea, unsigned int level, bool which)
    {
    return get_entry(ea, which ? level : NR_CACHE_LEVELS + level);
    }
    static struct entry *writeback_sentinel(struct smq_policy *mq, unsigned int level)
    {
    return get_sentinel(&mq.writeback_sentinel_alloc, level, mq.current_writeback_sentinels);
    }
    static struct entry *demote_sentinel(struct smq_policy *mq, unsigned int level)
    {
    return get_sentinel(&mq.demote_sentinel_alloc, level, mq.current_demote_sentinels);
    }
#[no_mangle]
unsafe extern "C" fn __update_writeback_sentinels(mq: *mut smq_policy) {
    static void __update_writeback_sentinels(struct smq_policy *mq)
    {
    unsigned int level;
    struct queue *q = &mq.dirty;
    struct entry *sentinel;
    for (level = 0; level < q.nr_levels; level++) {
    sentinel = writeback_sentinel(mq, level);
    q_del(q, sentinel);
    q_push(q, sentinel);
    }
    }
#[no_mangle]
unsafe extern "C" fn __update_demote_sentinels(mq: *mut smq_policy) {
    static void __update_demote_sentinels(struct smq_policy *mq)
    {
    unsigned int level;
    struct queue *q = &mq.clean;
    struct entry *sentinel;
    for (level = 0; level < q.nr_levels; level++) {
    sentinel = demote_sentinel(mq, level);
    q_del(q, sentinel);
    q_push(q, sentinel);
    }
    }
#[no_mangle]
unsafe extern "C" fn update_sentinels(mq: *mut smq_policy) {
    static void update_sentinels(struct smq_policy *mq)
    {
    if (time_after(jiffies, mq.next_writeback_period)) {
    mq.next_writeback_period = jiffies + WRITEBACK_PERIOD;
    mq.current_writeback_sentinels = !mq.current_writeback_sentinels;
    __update_writeback_sentinels(mq);
    }
    if (time_after(jiffies, mq.next_demote_period)) {
    mq.next_demote_period = jiffies + DEMOTE_PERIOD;
    mq.current_demote_sentinels = !mq.current_demote_sentinels;
    __update_demote_sentinels(mq);
    }
    }
#[no_mangle]
unsafe extern "C" fn __sentinels_init(mq: *mut smq_policy) {
    static void __sentinels_init(struct smq_policy *mq)
    {
    unsigned int level;
    struct entry *sentinel;
    for (level = 0; level < NR_CACHE_LEVELS; level++) {
    sentinel = writeback_sentinel(mq, level);
    sentinel.level = level;
    q_push(&mq.dirty, sentinel);
    sentinel = demote_sentinel(mq, level);
    sentinel.level = level;
    q_push(&mq.clean, sentinel);
    }
    }
#[no_mangle]
unsafe extern "C" fn sentinels_init(mq: *mut smq_policy) {
    static void sentinels_init(struct smq_policy *mq)
    {
    mq.next_writeback_period = jiffies + WRITEBACK_PERIOD;
    mq.next_demote_period = jiffies + DEMOTE_PERIOD;
    mq.current_writeback_sentinels = false;
    mq.current_demote_sentinels = false;
    __sentinels_init(mq);
    mq.current_writeback_sentinels = !mq.current_writeback_sentinels;
    mq.current_demote_sentinels = !mq.current_demote_sentinels;
    __sentinels_init(mq);
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn del_queue(mq: *mut smq_policy, e: *mut entry) {
    static void del_queue(struct smq_policy *mq, struct entry *e)
    {
    q_del(e.dirty ? &mq.dirty : &mq.clean, e);
    }
#[no_mangle]
unsafe extern "C" fn push_queue(mq: *mut smq_policy, e: *mut entry) {
    static void push_queue(struct smq_policy *mq, struct entry *e)
    {
    if (e.dirty)
    q_push(&mq.dirty, e);
    else
    q_push(&mq.clean, e);
    }
// !h, !q, a -> h, q, a
#[no_mangle]
unsafe extern "C" fn push(mq: *mut smq_policy, e: *mut entry) {
    static void push(struct smq_policy *mq, struct entry *e)
    {
    h_insert(&mq.table, e);
    if (!e.pending_work)
    push_queue(mq, e);
    }
#[no_mangle]
unsafe extern "C" fn push_queue_front(mq: *mut smq_policy, e: *mut entry) {
    static void push_queue_front(struct smq_policy *mq, struct entry *e)
    {
    if (e.dirty)
    q_push_front(&mq.dirty, e);
    else
    q_push_front(&mq.clean, e);
    }
#[no_mangle]
unsafe extern "C" fn push_front(mq: *mut smq_policy, e: *mut entry) {
    static void push_front(struct smq_policy *mq, struct entry *e)
    {
    h_insert(&mq.table, e);
    if (!e.pending_work)
    push_queue_front(mq, e);
    }
#[no_mangle]
unsafe extern "C" fn infer_cblock(mq: *mut smq_policy, e: *mut entry) -> dm_cblock_t {
    static dm_cblock_t infer_cblock(struct smq_policy *mq, struct entry *e)
    {
    return to_cblock(get_index(&mq.cache_alloc, e));
    }
#[no_mangle]
unsafe extern "C" fn requeue(mq: *mut smq_policy, e: *mut entry) {
    static void requeue(struct smq_policy *mq, struct entry *e)
    {
//
// Pending work has temporarily been taken out of the queues.
//
    if (e.pending_work)
    return;
    if (!test_and_set_bit(from_cblock(infer_cblock(mq, e)), mq.cache_hit_bits)) {
    if (!e.dirty) {
    q_requeue(&mq.clean, e, 1u, core::ptr::null_mut(), core::ptr::null_mut());
    return;
    }
    q_requeue(&mq.dirty, e, 1u,
    get_sentinel(&mq.writeback_sentinel_alloc, e.level, !mq.current_writeback_sentinels),
    get_sentinel(&mq.writeback_sentinel_alloc, e.level, mq.current_writeback_sentinels));
    }
    }
#[no_mangle]
unsafe extern "C" fn default_promote_level(mq: *mut smq_policy) -> c_uint {
    static unsigned int default_promote_level(struct smq_policy *mq)
    {
//
// The promote level depends on the current performance of the
// cache.
//
// If the cache is performing badly, then we can't afford
// to promote much without causing performance to drop below that
// of the origin device.
//
// If the cache is performing well, then we don't need to promote
// much.  If it isn't broken, don't fix it.
//
// If the cache is middling then we promote more.
//
// This scheme reminds me of a graph of entropy vs probability of a
// binary variable.
//
    static const unsigned int table[] = {
    1, 1, 1, 2, 4, 6, 7, 8, 7, 6, 4, 4, 3, 3, 2, 2, 1
    };
    let mut hits: c_uint = mq.cache_stats.hits;
    let mut misses: c_uint = mq.cache_stats.misses;
    let mut index: c_uint = safe_div(hits << 4u, hits + misses);
    return table[index];
    }
#[no_mangle]
unsafe extern "C" fn update_promote_levels(mq: *mut smq_policy) {
    static void update_promote_levels(struct smq_policy *mq)
    {
//
// If there are unused cache entries then we want to be really
// eager to promote.
//
    unsigned int threshold_level = allocator_empty(&mq.cache_alloc) ?
    default_promote_level(mq) : (NR_HOTSPOT_LEVELS / 2u);
    threshold_level = max(threshold_level, NR_HOTSPOT_LEVELS);
//
// If the hotspot queue is performing badly then we have little
// confidence that we know which blocks to promote.  So we cut down
// the amount of promotions.
//
    switch (stats_assess(&mq.hotspot_stats)) {
    case Q_POOR:
    threshold_level /= 4u;
    break;
    case Q_FAIR:
    threshold_level /= 2u;
    break;
    case Q_WELL:
    break;
    }
    mq.read_promote_level = NR_HOTSPOT_LEVELS - threshold_level;
    mq.write_promote_level = (NR_HOTSPOT_LEVELS - threshold_level);
    }
//
// If the hotspot queue is performing badly, then we try and move entries
// around more quickly.
//
#[no_mangle]
unsafe extern "C" fn update_level_jump(mq: *mut smq_policy) {
    static void update_level_jump(struct smq_policy *mq)
    {
    switch (stats_assess(&mq.hotspot_stats)) {
    case Q_POOR:
    mq.hotspot_level_jump = 4u;
    break;
    case Q_FAIR:
    mq.hotspot_level_jump = 2u;
    break;
    case Q_WELL:
    mq.hotspot_level_jump = 1u;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn end_hotspot_period(mq: *mut smq_policy) {
    static void end_hotspot_period(struct smq_policy *mq)
    {
    clear_bitset(mq.hotspot_hit_bits, mq.nr_hotspot_blocks);
    update_promote_levels(mq);
    if (time_after(jiffies, mq.next_hotspot_period)) {
    update_level_jump(mq);
    q_redistribute(&mq.hotspot);
    stats_reset(&mq.hotspot_stats);
    mq.next_hotspot_period = jiffies + HOTSPOT_UPDATE_PERIOD;
    }
    }
#[no_mangle]
unsafe extern "C" fn end_cache_period(mq: *mut smq_policy) {
    static void end_cache_period(struct smq_policy *mq)
    {
    if (time_after(jiffies, mq.next_cache_period)) {
    clear_bitset(mq.cache_hit_bits, from_cblock(mq.cache_size));
    q_redistribute(&mq.dirty);
    q_redistribute(&mq.clean);
    stats_reset(&mq.cache_stats);
    mq.next_cache_period = jiffies + CACHE_UPDATE_PERIOD;
    }
    }
// ----------------------------------------------------------------
//
// Targets are given as a percentage.
//

#[no_mangle]
unsafe extern "C" fn percent_to_target(mq: *mut smq_policy, p: c_uint) -> c_uint {
    static unsigned int percent_to_target(struct smq_policy *mq, unsigned int p)
    {
    return from_cblock(mq.cache_size) * p / 100u;
    }
#[no_mangle]
unsafe extern "C" fn clean_target_met(mq: *mut smq_policy, idle: bool) -> bool {
    static bool clean_target_met(struct smq_policy *mq, bool idle)
    {
//
// Cache entries may not be populated.  So we cannot rely on the
// size of the clean queue.
//
    if (idle || mq.cleaner) {
//
// We'd like to clean everything.
//
    return q_size(&mq.dirty) == 0u;
    }
//
// If we're busy we don't worry about cleaning at all.
//
    return true;
    }
#[no_mangle]
unsafe extern "C" fn free_target_met(mq: *mut smq_policy) -> bool {
    static bool free_target_met(struct smq_policy *mq)
    {
    unsigned int nr_free;
    nr_free = from_cblock(mq.cache_size) - mq.cache_alloc.nr_allocated;
    return (nr_free + btracker_nr_demotions_queued(mq.bg_work)) >=
    percent_to_target(mq, FREE_TARGET);
    }
// ----------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn mark_pending(mq: *mut smq_policy, e: *mut entry) {
    static void mark_pending(struct smq_policy *mq, struct entry *e)
    {
    BUG_ON(e.sentinel);
    BUG_ON(!e.allocated);
    BUG_ON(e.pending_work);
    e.pending_work = true;
    }
#[no_mangle]
unsafe extern "C" fn clear_pending(mq: *mut smq_policy, e: *mut entry) {
    static void clear_pending(struct smq_policy *mq, struct entry *e)
    {
    BUG_ON(!e.pending_work);
    e.pending_work = false;
    }
#[no_mangle]
unsafe extern "C" fn queue_writeback(mq: *mut smq_policy, idle: bool) {
    static void queue_writeback(struct smq_policy *mq, bool idle)
    {
    int r;
    struct policy_work work;
    struct entry *e;
    e = q_peek(&mq.dirty, mq.dirty.nr_levels, idle);
    if (e) {
    mark_pending(mq, e);
    q_del(&mq.dirty, e);
    work.op = POLICY_WRITEBACK;
    work.oblock = e.oblock;
    work.cblock = infer_cblock(mq, e);
    r = btracker_queue(mq.bg_work, &work, core::ptr::null_mut());
    if (r) {
    clear_pending(mq, e);
    q_push_front(&mq.dirty, e);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn queue_demotion(mq: *mut smq_policy) {
    static void queue_demotion(struct smq_policy *mq)
    {
    int r;
    struct policy_work work;
    struct entry *e;
    if (WARN_ON_ONCE(!mq.migrations_allowed))
    return;
    e = q_peek(&mq.clean, mq.clean.nr_levels / 2, true);
    if (!e) {
    if (!clean_target_met(mq, true))
    queue_writeback(mq, false);
    return;
    }
    mark_pending(mq, e);
    q_del(&mq.clean, e);
    work.op = POLICY_DEMOTE;
    work.oblock = e.oblock;
    work.cblock = infer_cblock(mq, e);
    r = btracker_queue(mq.bg_work, &work, core::ptr::null_mut());
    if (r) {
    clear_pending(mq, e);
    q_push_front(&mq.clean, e);
    }
    }
    static void queue_promotion(struct smq_policy *mq, dm_oblock_t oblock,
    struct policy_work **workp)
    {
    int r;
    struct entry *e;
    struct policy_work work;
    if (!mq.migrations_allowed)
    return;
    if (allocator_empty(&mq.cache_alloc)) {
//
// We always claim to be 'idle' to ensure some demotions happen
// with continuous loads.
//
    if (!free_target_met(mq))
    queue_demotion(mq);
    return;
    }
    if (btracker_promotion_already_present(mq.bg_work, oblock))
    return;
//
// We allocate the entry now to reserve the cblock.  If the
// background work is aborted we must remember to free it.
//
    e = alloc_entry(&mq.cache_alloc);
    BUG_ON(!e);
    e.pending_work = true;
    work.op = POLICY_PROMOTE;
    work.oblock = oblock;
    work.cblock = infer_cblock(mq, e);
    r = btracker_queue(mq.bg_work, &work, workp);
    if (r)
    free_entry(&mq.cache_alloc, e);
    }
// ----------------------------------------------------------------
    enum promote_result {
    PROMOTE_NOT,
    PROMOTE_TEMPORARY,
    PROMOTE_PERMANENT
    };
//
// Converts a boolean into a promote result.
//
#[no_mangle]
unsafe extern "C" fn maybe_promote(promote: bool) -> enum promote_result {
    static enum promote_result maybe_promote(bool promote)
    {
    return promote ? PROMOTE_PERMANENT : PROMOTE_NOT;
    }
    static enum promote_result should_promote(struct smq_policy *mq, struct entry *hs_e,
    int data_dir, bool fast_promote)
    {
    if (data_dir == WRITE) {
    if (!allocator_empty(&mq.cache_alloc) && fast_promote)
    return PROMOTE_TEMPORARY;
    return maybe_promote(hs_e.level >= mq.write_promote_level);
    } else
    return maybe_promote(hs_e.level >= mq.read_promote_level);
    }
#[no_mangle]
unsafe extern "C" fn to_hblock(mq: *mut smq_policy, b: dm_oblock_t) -> dm_oblock_t {
    static dm_oblock_t to_hblock(struct smq_policy *mq, dm_oblock_t b)
    {
    let mut r: sector_t = from_oblock(b);
    (void) sector_div(r, mq.cache_blocks_per_hotspot_block);
    return to_oblock(r);
    }
    static struct entry *update_hotspot_queue(struct smq_policy *mq, dm_oblock_t b)
    {
    unsigned int hi;
    let mut hb: dm_oblock_t = to_hblock(mq, b);
    struct entry *e = h_lookup(&mq.hotspot_table, hb);
    if (e) {
    stats_level_accessed(&mq.hotspot_stats, e.level);
    hi = get_index(&mq.hotspot_alloc, e);
    q_requeue(&mq.hotspot, e,
    test_and_set_bit(hi, mq.hotspot_hit_bits) ?
    0u : mq.hotspot_level_jump,
    core::ptr::null_mut(), core::ptr::null_mut());
    } else {
    stats_miss(&mq.hotspot_stats);
    e = alloc_entry(&mq.hotspot_alloc);
    if (!e) {
    e = q_pop(&mq.hotspot);
    if (e) {
    h_remove(&mq.hotspot_table, e);
    hi = get_index(&mq.hotspot_alloc, e);
    clear_bit(hi, mq.hotspot_hit_bits);
    }
    }
    if (e) {
    e.oblock = hb;
    q_push(&mq.hotspot, e);
    h_insert(&mq.hotspot_table, e);
    }
    }
    return e;
    }
// ----------------------------------------------------------------
//
// Public interface, via the policy struct.  See dm-cache-policy.h for a
// description of these.
//
    static struct smq_policy *to_smq_policy(struct dm_cache_policy *p)
    {
    return container_of(p, struct smq_policy, policy);
    }
#[no_mangle]
unsafe extern "C" fn smq_destroy(p: *mut dm_cache_policy) {
    static void smq_destroy(struct dm_cache_policy *p)
    {
    struct smq_policy *mq = to_smq_policy(p);
    btracker_destroy(mq.bg_work);
    h_exit(&mq.hotspot_table);
    h_exit(&mq.table);
    free_bitset(mq.hotspot_hit_bits);
    free_bitset(mq.cache_hit_bits);
    space_exit(&mq.es);
    kfree(mq);
    }
// ----------------------------------------------------------------
    static int __lookup(struct smq_policy *mq, dm_oblock_t oblock, dm_cblock_t *cblock,
    int data_dir, bool fast_copy,
    struct policy_work **work, bool *background_work)
    {
    struct entry *e, *hs_e;
    enum promote_result pr;
// background_work = false;
    e = h_lookup(&mq.table, oblock);
    if (e) {
    stats_level_accessed(&mq.cache_stats, e.level);
    requeue(mq, e);
// cblock = infer_cblock(mq, e);
    return 0;
    } else {
    stats_miss(&mq.cache_stats);
//
// The hotspot queue only gets updated with misses.
//
    hs_e = update_hotspot_queue(mq, oblock);
    pr = should_promote(mq, hs_e, data_dir, fast_copy);
    if (pr != PROMOTE_NOT) {
    queue_promotion(mq, oblock, work);
// background_work = true;
    }
    return -ENOENT;
    }
    }
    static int smq_lookup(struct dm_cache_policy *p, dm_oblock_t oblock, dm_cblock_t *cblock,
    int data_dir, bool fast_copy,
    bool *background_work)
    {
    int r;
    unsigned long flags;
    struct smq_policy *mq = to_smq_policy(p);
    spin_lock_irqsave(&mq.lock, flags);
    r = __lookup(mq, oblock, cblock,
    data_dir, fast_copy,
    core::ptr::null_mut(), background_work);
    spin_unlock_irqrestore(&mq.lock, flags);
    return r;
    }
    static int smq_lookup_with_work(struct dm_cache_policy *p,
    dm_oblock_t oblock, dm_cblock_t *cblock,
    int data_dir, bool fast_copy,
    struct policy_work **work)
    {
    int r;
    bool background_queued;
    unsigned long flags;
    struct smq_policy *mq = to_smq_policy(p);
    spin_lock_irqsave(&mq.lock, flags);
    r = __lookup(mq, oblock, cblock, data_dir, fast_copy, work, &background_queued);
    spin_unlock_irqrestore(&mq.lock, flags);
    return r;
    }
    static int smq_get_background_work(struct dm_cache_policy *p, bool idle,
    struct policy_work **result)
    {
    int r;
    unsigned long flags;
    struct smq_policy *mq = to_smq_policy(p);
    spin_lock_irqsave(&mq.lock, flags);
    r = btracker_issue(mq.bg_work, result);
    if (r == -ENODATA) {
    if (!clean_target_met(mq, idle)) {
    queue_writeback(mq, idle);
    r = btracker_issue(mq.bg_work, result);
    }
    }
    spin_unlock_irqrestore(&mq.lock, flags);
    return r;
    }
//
// We need to clear any pending work flags that have been set, and in the
// case of promotion free the entry for the destination cblock.
//
    static void __complete_background_work(struct smq_policy *mq,
    struct policy_work *work,
    bool success)
    {
    struct entry *e = get_entry(&mq.cache_alloc,
    from_cblock(work.cblock));
    switch (work.op) {
    case POLICY_PROMOTE:
// !h, !q, a
    clear_pending(mq, e);
    if (success) {
    e.oblock = work.oblock;
    e.level = NR_CACHE_LEVELS - 1;
    push(mq, e);
// h, q, a
    } else {
    free_entry(&mq.cache_alloc, e);
// !h, !q, !a
    }
    break;
    case POLICY_DEMOTE:
// h, !q, a
    if (success) {
    h_remove(&mq.table, e);
    free_entry(&mq.cache_alloc, e);
// !h, !q, !a
    } else {
    clear_pending(mq, e);
    push_queue(mq, e);
// h, q, a
    }
    break;
    case POLICY_WRITEBACK:
// h, !q, a
    clear_pending(mq, e);
    push_queue(mq, e);
// h, q, a
    break;
    }
    btracker_complete(mq.bg_work, work);
    }
    static void smq_complete_background_work(struct dm_cache_policy *p,
    struct policy_work *work,
    bool success)
    {
    unsigned long flags;
    struct smq_policy *mq = to_smq_policy(p);
    spin_lock_irqsave(&mq.lock, flags);
    __complete_background_work(mq, work, success);
    spin_unlock_irqrestore(&mq.lock, flags);
    }
// in_hash(oblock) -> in_hash(oblock)
#[no_mangle]
unsafe extern "C" fn __smq_set_clear_dirty(mq: *mut smq_policy, cblock: dm_cblock_t, set: bool) {
    static void __smq_set_clear_dirty(struct smq_policy *mq, dm_cblock_t cblock, bool set)
    {
    struct entry *e = get_entry(&mq.cache_alloc, from_cblock(cblock));
    if (e.pending_work)
    e.dirty = set;
    else {
    del_queue(mq, e);
    e.dirty = set;
    push_queue(mq, e);
    }
    }
#[no_mangle]
unsafe extern "C" fn smq_set_dirty(p: *mut dm_cache_policy, cblock: dm_cblock_t) {
    static void smq_set_dirty(struct dm_cache_policy *p, dm_cblock_t cblock)
    {
    unsigned long flags;
    struct smq_policy *mq = to_smq_policy(p);
    spin_lock_irqsave(&mq.lock, flags);
    __smq_set_clear_dirty(mq, cblock, true);
    spin_unlock_irqrestore(&mq.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn smq_clear_dirty(p: *mut dm_cache_policy, cblock: dm_cblock_t) {
    static void smq_clear_dirty(struct dm_cache_policy *p, dm_cblock_t cblock)
    {
    struct smq_policy *mq = to_smq_policy(p);
    unsigned long flags;
    spin_lock_irqsave(&mq.lock, flags);
    __smq_set_clear_dirty(mq, cblock, false);
    spin_unlock_irqrestore(&mq.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn random_level(cblock: dm_cblock_t) -> c_uint {
    static unsigned int random_level(dm_cblock_t cblock)
    {
    return hash_32(from_cblock(cblock), 9) & (NR_CACHE_LEVELS - 1);
    }
    static int smq_load_mapping(struct dm_cache_policy *p,
    dm_oblock_t oblock, dm_cblock_t cblock,
    bool dirty, uint32_t hint, bool hint_valid)
    {
    struct smq_policy *mq = to_smq_policy(p);
    struct entry *e;
    e = alloc_particular_entry(&mq.cache_alloc, from_cblock(cblock));
    e.oblock = oblock;
    e.dirty = dirty;
    e.level = hint_valid ? min(hint, NR_CACHE_LEVELS - 1) : random_level(cblock);
    e.pending_work = false;
//
// When we load mappings we push ahead of both sentinels in order to
// allow demotions and cleaning to occur immediately.
//
    push_front(mq, e);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smq_invalidate_mapping(p: *mut dm_cache_policy, cblock: dm_cblock_t) -> c_int {
    static int smq_invalidate_mapping(struct dm_cache_policy *p, dm_cblock_t cblock)
    {
    struct smq_policy *mq = to_smq_policy(p);
    struct entry *e = get_entry(&mq.cache_alloc, from_cblock(cblock));
    unsigned long flags;
    let mut r: c_int = 0;
    spin_lock_irqsave(&mq.lock, flags);
    if (!e.allocated) {
    r = -ENODATA;
    goto out;
    }
// FIXME: what if this block has pending background work?
    del_queue(mq, e);
    h_remove(&mq.table, e);
    free_entry(&mq.cache_alloc, e);
    out:
    spin_unlock_irqrestore(&mq.lock, flags);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn smq_get_hint(p: *mut dm_cache_policy, cblock: dm_cblock_t) -> u32 {
    static uint32_t smq_get_hint(struct dm_cache_policy *p, dm_cblock_t cblock)
    {
    struct smq_policy *mq = to_smq_policy(p);
    struct entry *e = get_entry(&mq.cache_alloc, from_cblock(cblock));
    if (!e.allocated)
    return 0;
    return e.level;
    }
#[no_mangle]
unsafe extern "C" fn smq_residency(p: *mut dm_cache_policy) -> dm_cblock_t {
    static dm_cblock_t smq_residency(struct dm_cache_policy *p)
    {
    dm_cblock_t r;
    unsigned long flags;
    struct smq_policy *mq = to_smq_policy(p);
    spin_lock_irqsave(&mq.lock, flags);
    r = to_cblock(mq.cache_alloc.nr_allocated);
    spin_unlock_irqrestore(&mq.lock, flags);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn smq_tick(p: *mut dm_cache_policy, can_block: bool) {
    static void smq_tick(struct dm_cache_policy *p, bool can_block)
    {
    struct smq_policy *mq = to_smq_policy(p);
    unsigned long flags;
    spin_lock_irqsave(&mq.lock, flags);
    mq.tick++;
    update_sentinels(mq);
    end_hotspot_period(mq);
    end_cache_period(mq);
    spin_unlock_irqrestore(&mq.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn smq_allow_migrations(p: *mut dm_cache_policy, allow: bool) {
    static void smq_allow_migrations(struct dm_cache_policy *p, bool allow)
    {
    struct smq_policy *mq = to_smq_policy(p);
    mq.migrations_allowed = allow;
    }
//
// smq has no config values, but the old mq policy did.  To avoid breaking
// software we continue to accept these configurables for the mq policy,
// but they have no effect.
//
    static int mq_set_config_value(struct dm_cache_policy *p,
    const char *key, const char *value)
    {
    unsigned long tmp;
    if (kstrtoul(value, 10, &tmp))
    return -EINVAL;
    if (!strcasecmp(key, "random_threshold") ||
    !strcasecmp(key, "sequential_threshold") ||
    !strcasecmp(key, "discard_promote_adjustment") ||
    !strcasecmp(key, "read_promote_adjustment") ||
    !strcasecmp(key, "write_promote_adjustment")) {
    DMWARN("tunable '%s' no longer has any effect, mq policy is now an alias for smq", key);
    return 0;
    }
    return -EINVAL;
    }
    static int mq_emit_config_values(struct dm_cache_policy *p, char *result,
    unsigned int maxlen, ssize_t *sz_ptr)
    {
    let mut sz: isize = *sz_ptr;
    DMEMIT("10 random_threshold 0 "
    "sequential_threshold 0 "
    "discard_promote_adjustment 0 "
    "read_promote_adjustment 0 "
    "write_promote_adjustment 0 ");
// sz_ptr = sz;
    return 0;
    }
// Init the policy plugin interface function pointers.
#[no_mangle]
unsafe extern "C" fn init_policy_functions(mq: *mut smq_policy, mimic_mq: bool) {
    static void init_policy_functions(struct smq_policy *mq, bool mimic_mq)
    {
    mq.policy.destroy = smq_destroy;
    mq.policy.lookup = smq_lookup;
    mq.policy.lookup_with_work = smq_lookup_with_work;
    mq.policy.get_background_work = smq_get_background_work;
    mq.policy.complete_background_work = smq_complete_background_work;
    mq.policy.set_dirty = smq_set_dirty;
    mq.policy.clear_dirty = smq_clear_dirty;
    mq.policy.load_mapping = smq_load_mapping;
    mq.policy.invalidate_mapping = smq_invalidate_mapping;
    mq.policy.get_hint = smq_get_hint;
    mq.policy.residency = smq_residency;
    mq.policy.tick = smq_tick;
    mq.policy.allow_migrations = smq_allow_migrations;
    if (mimic_mq) {
    mq.policy.set_config_value = mq_set_config_value;
    mq.policy.emit_config_values = mq_emit_config_values;
    }
    }
    static bool too_many_hotspot_blocks(sector_t origin_size,
    sector_t hotspot_block_size,
    unsigned int nr_hotspot_blocks)
    {
    return (hotspot_block_size * nr_hotspot_blocks) > origin_size;
    }
    static void calc_hotspot_params(sector_t origin_size,
    sector_t cache_block_size,
    unsigned int nr_cache_blocks,
    sector_t *hotspot_block_size,
    unsigned int *nr_hotspot_blocks)
    {
// hotspot_block_size = cache_block_size * 16u;
// nr_hotspot_blocks = max(nr_cache_blocks / 4u, 1024u);
    while ((*hotspot_block_size > cache_block_size) &&
    too_many_hotspot_blocks(origin_size, *hotspot_block_size, *nr_hotspot_blocks))
// hotspot_block_size /= 2u;
    }
    static struct dm_cache_policy *
    __smq_create(dm_cblock_t cache_size, sector_t origin_size, sector_t cache_block_size,
    bool mimic_mq, bool migrations_allowed, bool cleaner)
    {
    unsigned int i;
    let mut nr_sentinels_per_queue: c_uint = 2u * NR_CACHE_LEVELS;
    let mut total_sentinels: c_uint = 2u * nr_sentinels_per_queue;
    struct smq_policy *mq = kzalloc_obj(*mq);
    if (!mq)
    return core::ptr::null_mut();
    init_policy_functions(mq, mimic_mq);
    mq.cache_size = cache_size;
    mq.cache_block_size = cache_block_size;
    calc_hotspot_params(origin_size, cache_block_size, from_cblock(cache_size),
    &mq.hotspot_block_size, &mq.nr_hotspot_blocks);
    mq.cache_blocks_per_hotspot_block = div64_u64(mq.hotspot_block_size, mq.cache_block_size);
    mq.hotspot_level_jump = 1u;
    if (space_init(&mq.es, total_sentinels + mq.nr_hotspot_blocks + from_cblock(cache_size))) {
    DMERR("couldn't initialize entry space");
    goto bad_pool_init;
    }
    init_allocator(&mq.writeback_sentinel_alloc, &mq.es, 0, nr_sentinels_per_queue);
    for (i = 0; i < nr_sentinels_per_queue; i++)
    get_entry(&mq.writeback_sentinel_alloc, i).sentinel = true;
    init_allocator(&mq.demote_sentinel_alloc, &mq.es, nr_sentinels_per_queue, total_sentinels);
    for (i = 0; i < nr_sentinels_per_queue; i++)
    get_entry(&mq.demote_sentinel_alloc, i).sentinel = true;
    init_allocator(&mq.hotspot_alloc, &mq.es, total_sentinels,
    total_sentinels + mq.nr_hotspot_blocks);
    init_allocator(&mq.cache_alloc, &mq.es,
    total_sentinels + mq.nr_hotspot_blocks,
    total_sentinels + mq.nr_hotspot_blocks + from_cblock(cache_size));
    mq.hotspot_hit_bits = alloc_bitset(mq.nr_hotspot_blocks);
    if (!mq.hotspot_hit_bits) {
    DMERR("couldn't allocate hotspot hit bitset");
    goto bad_hotspot_hit_bits;
    }
    clear_bitset(mq.hotspot_hit_bits, mq.nr_hotspot_blocks);
    if (from_cblock(cache_size)) {
    mq.cache_hit_bits = alloc_bitset(from_cblock(cache_size));
    if (!mq.cache_hit_bits) {
    DMERR("couldn't allocate cache hit bitset");
    goto bad_cache_hit_bits;
    }
    clear_bitset(mq.cache_hit_bits, from_cblock(mq.cache_size));
    } else
    mq.cache_hit_bits = core::ptr::null_mut();
    mq.tick = 0;
    spin_lock_init(&mq.lock);
    q_init(&mq.hotspot, &mq.es, NR_HOTSPOT_LEVELS);
    mq.hotspot.nr_top_levels = 8;
    mq.hotspot.nr_in_top_levels = min(mq.nr_hotspot_blocks / NR_HOTSPOT_LEVELS,
    from_cblock(mq.cache_size) / mq.cache_blocks_per_hotspot_block);
    q_init(&mq.clean, &mq.es, NR_CACHE_LEVELS);
    q_init(&mq.dirty, &mq.es, NR_CACHE_LEVELS);
    stats_init(&mq.hotspot_stats, NR_HOTSPOT_LEVELS);
    stats_init(&mq.cache_stats, NR_CACHE_LEVELS);
    if (h_init(&mq.table, &mq.es, from_cblock(cache_size)))
    goto bad_alloc_table;
    if (h_init(&mq.hotspot_table, &mq.es, mq.nr_hotspot_blocks))
    goto bad_alloc_hotspot_table;
    sentinels_init(mq);
    mq.write_promote_level = mq.read_promote_level = NR_HOTSPOT_LEVELS;
    mq.next_hotspot_period = jiffies;
    mq.next_cache_period = jiffies;
    mq.bg_work = btracker_create(max(1u, smq_max_background_work));
    if (!mq.bg_work)
    goto bad_btracker;
    mq.migrations_allowed = migrations_allowed;
    mq.cleaner = cleaner;
    return &mq.policy;
    bad_btracker:
    h_exit(&mq.hotspot_table);
    bad_alloc_hotspot_table:
    h_exit(&mq.table);
    bad_alloc_table:
    free_bitset(mq.cache_hit_bits);
    bad_cache_hit_bits:
    free_bitset(mq.hotspot_hit_bits);
    bad_hotspot_hit_bits:
    space_exit(&mq.es);
    bad_pool_init:
    kfree(mq);
    return core::ptr::null_mut();
    }
    static struct dm_cache_policy *smq_create(dm_cblock_t cache_size,
    sector_t origin_size,
    sector_t cache_block_size)
    {
    return __smq_create(cache_size, origin_size, cache_block_size,
    false, true, false);
    }
    static struct dm_cache_policy *mq_create(dm_cblock_t cache_size,
    sector_t origin_size,
    sector_t cache_block_size)
    {
    return __smq_create(cache_size, origin_size, cache_block_size,
    true, true, false);
    }
    static struct dm_cache_policy *cleaner_create(dm_cblock_t cache_size,
    sector_t origin_size,
    sector_t cache_block_size)
    {
    return __smq_create(cache_size, origin_size, cache_block_size,
    false, false, true);
    }
// ----------------------------------------------------------------
    static struct dm_cache_policy_type smq_policy_type = {
    .name = "smq",
    .version = {2, 0, 0},
    .hint_size = 4,
    .owner = THIS_MODULE,
    .create = smq_create
    };
    static struct dm_cache_policy_type mq_policy_type = {
    .name = "mq",
    .version = {2, 0, 0},
    .hint_size = 4,
    .owner = THIS_MODULE,
    .create = mq_create,
    };
    static struct dm_cache_policy_type cleaner_policy_type = {
    .name = "cleaner",
    .version = {2, 0, 0},
    .hint_size = 4,
    .owner = THIS_MODULE,
    .create = cleaner_create,
    };
    static struct dm_cache_policy_type default_policy_type = {
    .name = "default",
    .version = {2, 0, 0},
    .hint_size = 4,
    .owner = THIS_MODULE,
    .create = smq_create,
    .real = &smq_policy_type
    };
#[no_mangle]
unsafe extern "C" fn smq_init() -> int __init {
    static int __init smq_init(void)
    {
    int r;
    r = dm_cache_policy_register(&smq_policy_type);
    if (r) {
    DMERR("register failed %d", r);
    return -ENOMEM;
    }
    r = dm_cache_policy_register(&mq_policy_type);
    if (r) {
    DMERR("register failed (as mq) %d", r);
    goto out_mq;
    }
    r = dm_cache_policy_register(&cleaner_policy_type);
    if (r) {
    DMERR("register failed (as cleaner) %d", r);
    goto out_cleaner;
    }
    r = dm_cache_policy_register(&default_policy_type);
    if (r) {
    DMERR("register failed (as default) %d", r);
    goto out_default;
    }
    return 0;
    out_default:
    dm_cache_policy_unregister(&cleaner_policy_type);
    out_cleaner:
    dm_cache_policy_unregister(&mq_policy_type);
    out_mq:
    dm_cache_policy_unregister(&smq_policy_type);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn smq_exit() -> void __exit {
    static void __exit smq_exit(void)
    {
    dm_cache_policy_unregister(&cleaner_policy_type);
    dm_cache_policy_unregister(&smq_policy_type);
    dm_cache_policy_unregister(&mq_policy_type);
    dm_cache_policy_unregister(&default_policy_type);
    }
    module_init(smq_init);
    module_exit(smq_exit);
    MODULE_AUTHOR("Joe Thornber <dm-devel@lists.linux.dev>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("smq cache policy");
    MODULE_ALIAS("dm-cache-default");
    MODULE_ALIAS("dm-cache-mq");
    MODULE_ALIAS("dm-cache-cleaner");
