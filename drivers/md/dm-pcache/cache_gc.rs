//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-pcache/cache_gc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// cache_key_gc - Releases the reference of a cache key segment.
// @key: Pointer to the cache key to be garbage collected.
//
// This function decrements the reference count of the cache segment
// associated with the given key. If the reference count drops to zero,
// the segment may be invalidated and reused.
//
#[no_mangle]
unsafe extern "C" fn cache_key_gc(key: *mut pcache_cache_key) {
    static void cache_key_gc(struct pcache_cache_key *key)
    {
    cache_seg_put(key.cache_pos.cache_seg);
    }
#[no_mangle]
unsafe extern "C" fn need_gc(cache: *mut pcache_cache, dirty_tail: *mut pcache_cache_pos, key_tail: *mut pcache_cache_pos) -> bool {
    static bool need_gc(struct pcache_cache *cache, struct pcache_cache_pos *dirty_tail, struct pcache_cache_pos *key_tail)
    {
    struct dm_pcache *pcache = CACHE_TO_PCACHE(cache);
    struct pcache_cache_kset_onmedia *kset_onmedia;
    void *dirty_addr, *key_addr;
    u32 segs_used, segs_gc_threshold, to_copy;
    int ret;
    dirty_addr = cache_pos_addr(dirty_tail);
    key_addr = cache_pos_addr(key_tail);
    if (dirty_addr == key_addr) {
    pcache_dev_debug(pcache, "key tail is equal to dirty tail: %u:%u\n",
    dirty_tail.cache_seg.cache_seg_id,
    dirty_tail.seg_off);
    return false;
    }
    kset_onmedia = (struct pcache_cache_kset_onmedia *)cache.gc_kset_onmedia_buf;
    to_copy = min(PCACHE_KSET_ONMEDIA_SIZE_MAX, cache_seg_remain(key_tail));
    ret = copy_mc_to_kernel(kset_onmedia, key_addr, to_copy);
    if (ret) {
    pcache_dev_err(pcache, "error to read kset: %d", ret);
    return false;
    }
// Reject a corrupted or out-of-bounds kset before reading its keys
    if (!kset_onmedia_valid(kset_onmedia)) {
    pcache_dev_debug(pcache, "gc error: invalid kset. key_tail: %u:%u magic: %llx, key_num: %u\n",
    key_tail.cache_seg.cache_seg_id, key_tail.seg_off,
    kset_onmedia.magic, kset_onmedia.key_num);
    return false;
    }
// Verify the CRC of the kset_onmedia
    if (kset_onmedia.crc != cache_kset_crc(kset_onmedia)) {
    pcache_dev_debug(pcache, "gc error: crc is not as expected. crc: %x, expected: %x\n",
    cache_kset_crc(kset_onmedia), kset_onmedia.crc);
    return false;
    }
    segs_used = bitmap_weight(cache.seg_map, cache.n_segs);
    segs_gc_threshold = cache.n_segs * pcache_cache_get_gc_percent(cache) / 100;
    if (segs_used < segs_gc_threshold) {
    pcache_dev_debug(pcache, "segs_used: %u, segs_gc_threshold: %u\n", segs_used, segs_gc_threshold);
    return false;
    }
    return true;
    }
//
// last_kset_gc - Advances the garbage collection for the last kset.
// @cache: Pointer to the pcache_cache structure.
// @kset_onmedia: Pointer to the kset_onmedia structure for the last kset.
//
#[no_mangle]
unsafe extern "C" fn last_kset_gc(cache: *mut pcache_cache, kset_onmedia: *mut pcache_cache_kset_onmedia) -> c_int {
    static int last_kset_gc(struct pcache_cache *cache, struct pcache_cache_kset_onmedia *kset_onmedia)
    {
    struct dm_pcache *pcache = CACHE_TO_PCACHE(cache);
    struct pcache_cache_segment *cur_seg, *next_seg;
    if (!cache_seg_id_valid(cache, kset_onmedia.next_cache_seg_id)) {
    pcache_dev_err(pcache, "invalid next_cache_seg_id %u in gc (n_segs %u)\n",
    kset_onmedia.next_cache_seg_id, cache.n_segs);
    return -EIO;
    }
    cur_seg = cache.key_tail.cache_seg;
    next_seg = &cache.segments[kset_onmedia.next_cache_seg_id];
    mutex_lock(&cache.key_tail_lock);
    cache.key_tail.cache_seg = next_seg;
    cache.key_tail.seg_off = 0;
    cache_encode_key_tail(cache);
    mutex_unlock(&cache.key_tail_lock);
    pcache_dev_debug(pcache, "gc advance kset seg: %u\n", cur_seg.cache_seg_id);
    spin_lock(&cache.seg_map_lock);
    __clear_bit(cur_seg.cache_seg_id, cache.seg_map);
    spin_unlock(&cache.seg_map_lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pcache_cache_gc_fn(work: *mut work_struct) {
    void pcache_cache_gc_fn(struct work_struct *work)
    {
    struct pcache_cache *cache = container_of(work, struct pcache_cache, gc_work.work);
    struct dm_pcache *pcache = CACHE_TO_PCACHE(cache);
    struct pcache_cache_pos dirty_tail, key_tail;
    struct pcache_cache_kset_onmedia *kset_onmedia;
    struct pcache_cache_key_onmedia *key_onmedia;
    struct pcache_cache_key *key;
    int ret;
    int i;
    kset_onmedia = (struct pcache_cache_kset_onmedia *)cache.gc_kset_onmedia_buf;
    while (true) {
    if (pcache_is_stopping(pcache) || atomic_read(&cache.gc_errors))
    return;
// Get new tail positions
    mutex_lock(&cache.dirty_tail_lock);
    cache_pos_copy(&dirty_tail, &cache.dirty_tail);
    mutex_unlock(&cache.dirty_tail_lock);
    mutex_lock(&cache.key_tail_lock);
    cache_pos_copy(&key_tail, &cache.key_tail);
    mutex_unlock(&cache.key_tail_lock);
    if (!need_gc(cache, &dirty_tail, &key_tail))
    break;
    if (kset_onmedia.flags & PCACHE_KSET_FLAGS_LAST) {
// Don't move to the next segment if dirty_tail has not moved
    if (dirty_tail.cache_seg == key_tail.cache_seg)
    break;
    ret = last_kset_gc(cache, kset_onmedia);
    if (ret) {
    atomic_inc(&cache.gc_errors);
    return;
    }
    continue;
    }
    if (get_kset_onmedia_size(kset_onmedia) > cache_seg_remain(&key_tail)) {
    atomic_inc(&cache.gc_errors);
    return;
    }
    for (i = 0; i < kset_onmedia.key_num; i++) {
    let mut key_tmp: pcache_cache_key = { 0 };
    key_onmedia = &kset_onmedia.data[i];
    key = &key_tmp;
    cache_key_init(&cache.req_key_tree, key);
    ret = cache_key_decode(cache, key_onmedia, key);
    if (ret) {
// return without re-arm gc work, and prevent future
// gc, because we can't retry the partial-gc-ed kset
//
    atomic_inc(&cache.gc_errors);
    pcache_dev_err(pcache, "failed to decode cache key in gc\n");
    return;
    }
    cache_key_gc(key);
    }
    pcache_dev_debug(pcache, "gc advance: %u:%u %u\n",
    key_tail.cache_seg.cache_seg_id,
    key_tail.seg_off,
    get_kset_onmedia_size(kset_onmedia));
    mutex_lock(&cache.key_tail_lock);
    cache_pos_advance(&cache.key_tail, get_kset_onmedia_size(kset_onmedia));
    cache_encode_key_tail(cache);
    mutex_unlock(&cache.key_tail_lock);
    }
    queue_delayed_work(cache_get_wq(cache), &cache.gc_work, PCACHE_CACHE_GC_INTERVAL);
    }
