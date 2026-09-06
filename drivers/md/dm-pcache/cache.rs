//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-pcache/cache.h
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

// Garbage collection thresholds

pub const PCACHE_CACHE_SUBTREE_SIZE_MASK: c_uint = 0x3FFFFF            /* Mask for tree size */;

// Maximum number of keys per key set
pub const PCACHE_KSET_KEYS_MAX: c_int = 128;

// Maximum number of keys to clean in one round of clean_work
pub const PCACHE_CLEAN_KEYS_MAX: c_int = 10;
// Writeback and garbage collection intervals in jiffies

// Macro to get the cache key structure from an rb_node pointer

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_pos_onmedia {
    pub header: pcache_meta_header,
    pub cache_seg_id: __u32,
    pub seg_off: __u32,
}

// Offset and size definitions for cache segment control

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_seg_gen {
    pub header: pcache_meta_header,
    pub gen: __u64,
}

// Control structure for cache segments
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_seg_ctrl {
    pub gen: [pcache_cache_seg_gen; PCACHE_META_INDEX_MAX],
    pub res: [__u64; 64],
}

pub const PCACHE_CACHE_MODE_WRITEBACK: c_int = 0;
pub const PCACHE_CACHE_MODE_WRITETHROUGH: c_int = 1;
pub const PCACHE_CACHE_MODE_WRITEAROUND: c_int = 2;
pub const PCACHE_CACHE_MODE_WRITEONLY: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_info {
    pub header: pcache_meta_header,
    pub seg_id: __u32,
    pub n_segs: __u32,
    pub flags: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_pos {
    pub cache_seg: *mut pcache_cache_segment,
    pub seg_off: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_segment {
    pub cache: *mut pcache_cache,
    pub /: *mut *mut u32 cache_seg_id; / Index in cache->segments,
    pub segment: pcache_segment,
    pub refs: core::sync::atomic::AtomicI32,
    pub cache_seg_info: pcache_segment_info,
    pub info_lock: mutex,
    pub info_index: u32,
    pub gen_lock: spinlock_t,
    pub gen: u64,
    pub gen_seq: u64,
    pub gen_index: u32,
    pub cache_seg_ctrl: *mut pcache_cache_seg_ctrl,
}

// rbtree for cache entries
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_subtree {
    pub root: rb_root,
    pub tree_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_tree {
    pub cache: *mut pcache_cache,
    pub n_subtrees: u32,
    pub key_pool: mempool_t,
    pub subtrees: *mut pcache_cache_subtree,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_key {
    pub cache_tree: *mut pcache_cache_tree,
    pub cache_subtree: *mut pcache_cache_subtree,
    pub ref: kref,
    pub rb_node: rb_node,
    pub list_node: list_head,
    pub off: u64,
    pub len: u32,
    pub flags: u32,
    pub cache_pos: pcache_cache_pos,
    pub seg_gen: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_key_onmedia {
    pub off: __u64,
    pub len: __u32,
    pub flags: __u32,
    pub cache_seg_id: __u32,
    pub cache_seg_off: __u32,
    pub seg_gen: __u64,
    pub data_crc: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_kset_onmedia {
    pub crc: __u32,
    pub key_num: __u32,
    pub next_cache_seg_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache {
    pub backing_dev: *mut pcache_backing_dev,
    pub cache_dev: *mut pcache_cache_dev,
    pub cache_ctrl: *mut pcache_cache_ctrl,
    pub dev_size: u64,
    pub data_heads: *mut pcache_cache_data_head __percpu,
    pub key_head_lock: spinlock_t,
    pub key_head: pcache_cache_pos,
    pub n_ksets: u32,
    pub ksets: *mut pcache_cache_kset,
    pub key_tail_lock: mutex,
    pub key_tail: pcache_cache_pos,
    pub key_tail_seq: u64,
    pub key_tail_index: u32,
    pub dirty_tail_lock: mutex,
    pub dirty_tail: pcache_cache_pos,
    pub dirty_tail_seq: u64,
    pub dirty_tail_index: u32,
    pub req_key_tree: pcache_cache_tree,
    pub clean_work: work_struct,
    pub writeback_lock: mutex,
    pub wb_kset_onmedia_buf: [c_char; PCACHE_KSET_ONMEDIA_SIZE_MAX],
    pub writeback_key_tree: pcache_cache_tree,
    pub writeback_work: delayed_work,
    pub pending: core::sync::atomic::AtomicI32,
    pub advance: u32,
    pub ret: c_int,
    pub writeback_ctx: },
    pub writeback_errors: core::sync::atomic::AtomicI32,
    pub gc_kset_onmedia_buf: [c_char; PCACHE_KSET_ONMEDIA_SIZE_MAX],
    pub gc_work: delayed_work,
    pub gc_errors: core::sync::atomic::AtomicI32,
    pub cache_info_lock: mutex,
    pub cache_info: pcache_cache_info,
    pub cache_info_addr: *mut pcache_cache_info,
    pub info_index: u32,
    pub n_segs: u32,
    pub seg_map: *mut c_ulong,
    pub last_cache_seg: u32,
    pub cache_full: bool,
    pub seg_map_lock: spinlock_t,
    pub segments: *mut pcache_cache_segment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_options {
    pub cache_mode:4: u32,
    pub data_crc:1: u32,
}

extern "C" {
    pub fn pcache_cache_start(pcache: *mut dm_pcache) -> c_int;
}
extern "C" {
    pub fn pcache_cache_stop(pcache: *mut dm_pcache);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_ctrl {
// Updated by gc_thread
    pub key_tail_pos: [pcache_cache_pos_onmedia; PCACHE_META_INDEX_MAX],
// Updated by writeback_thread
    pub dirty_tail_pos: [pcache_cache_pos_onmedia; PCACHE_META_INDEX_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_data_head {
    pub head_pos: pcache_cache_pos,
}

extern "C" {
    pub fn FIELD_GET(_arg: PCACHE_CACHE_FLAGS_GC_PERCENT_MASK, _arg: cache->cache_info.flags) -> return;
}
extern "C" {
    pub fn pcache_cache_set_gc_percent(cache: *mut pcache_cache, percent: u8) -> c_int;
}
// cache key
extern "C" {
    pub fn cache_key_init(cache_tree: *mut pcache_cache_tree, key: *mut pcache_cache_key);
}
extern "C" {
    pub fn cache_key_get(key: *mut pcache_cache_key);
}
extern "C" {
    pub fn cache_key_put(key: *mut pcache_cache_key);
}
extern "C" {
    pub fn cache_key_append(cache: *mut pcache_cache, key: *mut pcache_cache_key, force_close: bool) -> c_int;
}
extern "C" {
    pub fn cache_key_insert(cache_tree: *mut pcache_cache_tree, key: *mut pcache_cache_key, fixup: bool);
}
extern "C" {
    pub fn cache_pos_advance(pos: *mut pcache_cache_pos, len: u32);
}

pub const PCACHE_KSET_MAGIC: c_uint = 0x676894a64e164f1aULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_kset {
    pub cache: *mut pcache_cache,
    pub kset_lock: spinlock_t,
    pub flush_work: delayed_work,
    pub kset_onmedia: pcache_cache_kset_onmedia,
}

pub const SUBTREE_WALK_RET_OK: c_int = 0;
pub const SUBTREE_WALK_RET_ERR: c_int = 1;
pub const SUBTREE_WALK_RET_NEED_KEY: c_int = 2;
pub const SUBTREE_WALK_RET_NEED_REQ: c_int = 3;
pub const SUBTREE_WALK_RET_RESEARCH: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_subtree_walk_ctx {
    pub cache_tree: *mut pcache_cache_tree,
    pub start_node: *mut rb_node,
    pub pcache_req: *mut pcache_request,
    pub key: *mut pcache_cache_key,
    pub req_done: u32,
    pub ret: c_int,
// pre-allocated key and backing_dev_req
    pub pre_alloc_key: *mut pcache_cache_key,
    pub pre_alloc_req: *mut pcache_backing_dev_req,
    pub delete_key_list: *mut list_head,
    pub submit_req_list: *mut list_head,
//
// |--------|		key_tmp
// |====|			key
//
    pub ctx): *mut pcache_cache_subtree_walk_ctx,
//
// |----------|			key_tmp
// |=====|		key
//
    pub ctx): *mut pcache_cache_subtree_walk_ctx,
//
// |----------------|	key_tmp
// |===========|		key
//
    pub ctx): *mut pcache_cache_subtree_walk_ctx,
//
// |--------|			key_tmp
// |==========|		key
//
    pub ctx): *mut pcache_cache_subtree_walk_ctx,
//
// |----|			key_tmp
// |==========|			key
//
    pub ctx): *mut pcache_cache_subtree_walk_ctx,
//
// |-----------|		key_tmp
// |====|			key
//
    pub ctx): *mut pcache_cache_subtree_walk_ctx,
    pub ret): *mut *mut *mut int (walk_finally)(struct pcache_cache_subtree_walk_ctx ctx, int,
    pub ctx): *mut *mut bool (walk_done)(struct pcache_cache_subtree_walk_ctx,
}

extern "C" {
    pub fn cache_subtree_walk(ctx: *mut pcache_cache_subtree_walk_ctx) -> c_int;
}
extern "C" {
    pub fn cache_kset_close(cache: *mut pcache_cache, kset: *mut pcache_cache_kset) -> c_int;
}
extern "C" {
    pub fn clean_fn(work: *mut work_struct);
}
extern "C" {
    pub fn kset_flush_fn(work: *mut work_struct);
}
extern "C" {
    pub fn cache_replay(cache: *mut pcache_cache) -> c_int;
}
extern "C" {
    pub fn cache_tree_init(cache: *mut pcache_cache, cache_tree: *mut pcache_cache_tree, n_subtrees: u32) -> c_int;
}
extern "C" {
    pub fn cache_tree_clear(cache_tree: *mut pcache_cache_tree);
}
extern "C" {
    pub fn cache_tree_exit(cache_tree: *mut pcache_cache_tree);
}
// cache segments
extern "C" {
    pub fn cache_seg_get(cache_seg: *mut pcache_cache_segment);
}
extern "C" {
    pub fn cache_seg_put(cache_seg: *mut pcache_cache_segment);
}
extern "C" {
    pub fn cache_seg_set_next_seg(cache_seg: *mut pcache_cache_segment, seg_id: u32);
}
// cache request
extern "C" {
    pub fn pcache_cache_flush(cache: *mut pcache_cache) -> c_int;
}
extern "C" {
    pub fn pcache_cache_handle_req(cache: *mut pcache_cache, pcache_req: *mut pcache_request) -> c_int;
}
// gc
extern "C" {
    pub fn pcache_cache_gc_fn(work: *mut work_struct);
}
// writeback
extern "C" {
    pub fn cache_writeback_exit(cache: *mut pcache_cache);
}
extern "C" {
    pub fn cache_writeback_init(cache: *mut pcache_cache) -> c_int;
}
extern "C" {
    pub fn cache_writeback_fn(work: *mut work_struct);
}
// inline functions
extern "C" {
    pub fn cache_pos_addr(_arg: &cache->key_head) -> return;
}
extern "C" {
    pub fn this_cpu_ptr(_arg: cache->data_heads) -> return;
}
//
// cache_seg_is_ctrl_seg - Checks if a cache segment is a cache ctrl segment.
// @cache_seg_id: ID of the cache segment.
//
// Returns true if the cache segment ID corresponds to a cache ctrl segment.
//
// Note: We extend the segment control of the first cache segment
// (cache segment ID 0) to serve as the cache control (pcache_cache_ctrl)
// for the entire PCACHE cache. This function determines whether the given
// cache segment is the one storing the pcache_cache_ctrl information.
//
// cache_seg_id_valid - Validate a cache segment id read from the cache device.
// @cache: Pointer to the pcache_cache structure.
// @cache_seg_id: Segment id decoded from on-media metadata.
//
// On-media segment ids are only protected by a CRC, which an attacker who can
// format the cache device computes over their chosen value. Reject any id that
// would index cache->segments[] out of bounds before it is dereferenced.
//
// cache_key_cutfront - Cuts a specified length from the front of a cache key.
// @key: Pointer to pcache_cache_key structure.
// @cut_len: Length to cut from the front.
//
// Advances the cache key position by cut_len and adjusts offset and length accordingly.
//
// cache_key_cutback - Cuts a specified length from the back of a cache key.
// @key: Pointer to pcache_cache_key structure.
// @cut_len: Length to cut from the back.
//
// Reduces the length of the cache key by cut_len.
//
extern "C" {
    pub fn FIELD_GET(_arg: PCACHE_CACHE_FLAGS_CACHE_MODE_MASK, _arg: cache->cache_info.flags) -> return;
}
//
// cache_key_data_crc - Calculates CRC for data in a cache key.
// @key: Pointer to the pcache_cache_key structure.
//
// Returns the CRC-32 checksum of the data within the cache key's position.
//
extern "C" {
    pub fn crc32c(_arg: PCACHE_CRC_SEED, _arg: data, _arg: key->len) -> return;
}
//
// kset_onmedia_valid - Validate a kset header read from the cache device.
// @kset_onmedia: Pointer to the kset copied from on-media metadata.
//
// The magic and CRC are attacker-computable (fixed public seed). A non-last
// kset stores key_num keys inline, and cache_kset_crc() and the replay loop
// read struct_size(.., data, key_num) bytes from a buffer sized for
// PCACHE_KSET_KEYS_MAX keys, so key_num must be bounded before any such use.
//
extern "C" {
    pub fn crc32c(_arg: PCACHE_CRC_SEED, 4: *mut *mut (void )kset_onmedia +, _arg: crc_size) -> return;
}
extern "C" {
    pub fn struct_size_t(pcache_cache_kset_onmedia: struct, _arg: data, _arg: kset_onmedia->key_num) -> return;
}
//
// cache_seg_remain - Computes remaining space in a cache segment.
// @pos: Pointer to pcache_cache_pos structure.
//
// Returns the amount of remaining space in the segment data starting from
// the current position offset.
//
// cache_key_invalid - Checks if a cache key is invalid.
// @key: Pointer to pcache_cache_key structure.
//
// Returns true if the cache key is invalid due to its generation being
// less than the generation of its segment; otherwise returns false.
//
// When the GC (garbage collection) thread identifies a segment
// as reclaimable, it increments the segment's generation (gen). However,
// it does not immediately remove all related cache keys. When accessing
// such a cache key, this function can be used to determine if the cache
// key has already become invalid.
//
// cache_key_lstart - Retrieves the logical start offset of a cache key.
// @key: Pointer to pcache_cache_key structure.
//
// Returns the logical start offset for the cache key.
//
// cache_key_lend - Retrieves the logical end offset of a cache key.
// @key: Pointer to pcache_cache_key structure.
//
// Returns the logical end offset for the cache key.
//
// cache_pos_onmedia_crc - Calculates the CRC for an on-media cache position.
// @pos_om: Pointer to pcache_cache_pos_onmedia structure.
//
// Calculates the CRC-32 checksum of the position, excluding the first 4 bytes.
// Returns the computed CRC value.
//
extern "C" {
    pub fn pcache_meta_crc(_arg: &pos_om->header, pcache_cache_pos_onmedia): sizeof(struct) -> return;
}
extern "C" {
    pub fn cache_verify_dirty_tail(cache: *mut pcache_cache) -> c_int;
}
extern "C" {
    pub fn pcache_cache_init() -> c_int;
}
extern "C" {
    pub fn pcache_cache_exit();
}
