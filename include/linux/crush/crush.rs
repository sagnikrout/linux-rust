//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crush/crush.h
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
// CRUSH is a pseudo-random data distribution algorithm that
// efficiently distributes input values (typically, data objects)
// across a heterogeneous, structured storage cluster.
//
// The algorithm was originally described in detail in this paper
// (although the algorithm has evolved somewhat since then):
//
// https://www.ssrc.ucsc.edu/Papers/weil-sc06.pdf
//
// LGPL2
//
pub const CRUSH_MAGIC: c_uint = 0x00010000ul   /* for detecting algorithm revisions */;

pub const CRUSH_ITEM_UNDEF: c_uint = 0x7ffffffe  /* undefined result (internal use only) */;
pub const CRUSH_ITEM_NONE: c_uint = 0x7fffffff  /* no result */;
//
// CRUSH uses user-defined "rules" to describe how inputs should be
// mapped to devices.  A rule consists of sequence of steps to perform
// to generate the set of output devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_rule_step {
    pub op: __u32,
    pub arg1: __s32,
    pub arg2: __s32,
}

// step op codes
// arg2 = type
//
// for specifying choose num (arg1) relative to the max parameter
// passed to do_rule
//
pub const CRUSH_CHOOSE_N: c_int = 0;

//
// The rule mask is used to describe what the rule is intended for.
// Given a ruleset and size of output set, we search through the
// rule list for a matching rule_mask.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_rule_mask {
    pub ruleset: __u8,
    pub type: __u8,
    pub min_size: __u8,
    pub max_size: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_rule {
    pub len: __u32,
    pub mask: crush_rule_mask,
    pub steps: [crush_rule_step; ],
}

//
// A bucket is a named container of other items (either devices or
// other buckets).  Items within a bucket are chosen using one of a
// few different algorithms.  The table summarizes how the speed of
// each option measures up against mapping stability when items are
// added or removed.
//
// Bucket Alg     Speed       Additions    Removals
// ------------------------------------------------
// uniform         O(1)       poor         poor
// list            O(n)       optimal      poor
// tree            O(log n)   good         good
// straw           O(n)       better       better
// straw2          O(n)       optimal      optimal
//
// although tree was a legacy algorithm, it has been buggy, so
// exclude it.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_bucket {
    pub /: *mut *mut __s32 id; / this'll be negative,
    pub /: *mut *mut __u16 type; / non-zero; type=0 is reserved for devices,
    pub /: *mut *mut *mut __u8 alg; / one of CRUSH_BUCKET_,
    pub /: *mut *mut *mut __u8 hash; / which hash function to use, CRUSH_HASH_,
    pub /: *mut *mut __u32 weight; / 16-bit fixed point,
    pub /: *mut *mut __u32 size; / num items,
    pub items: *mut __s32,
}

// @ingroup API
//
// Replacement weights for each item in a bucket. The size of the
// array must be exactly the size of the straw2 bucket, just as the
// item_weights array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_weight_set {
    pub weights: *mut *mut *mut __u32 weights; /!< 16.16 fixed point,
    pub /: *mut *mut __u32 size; /!< size of the __weights__ array,
}

// @ingroup API
//
// Replacement weights and ids for a given straw2 bucket, for
// placement purposes.
//
// When crush_do_rule() chooses the Nth item from a straw2 bucket, the
// replacement weights found at __weight_set[N]__ are used instead of
// the weights from __item_weights__. If __N__ is greater than
// __weight_set_size__, the weights found at __weight_set_size-1__ are
// used instead. For instance if __weight_set__ is:
//
// [ [ 0x10000, 0x20000 ],   // position 0
// [ 0x20000, 0x40000 ] ]  // position 1
//
// choosing the 0th item will use position 0 weights [ 0x10000, 0x20000 ]
// choosing the 1th item will use position 1 weights [ 0x20000, 0x40000 ]
// choosing the 2th item will use position 1 weights [ 0x20000, 0x40000 ]
// etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_choose_arg {
    pub /: *mut *mut *mut __s32 ids; /!< values to use instead of items,
    pub /: *mut *mut __u32 ids_size; /!< size of the __ids__ array,
    pub for: *mut *mut *mut crush_weight_set weight_set; /!< weight replacements,
    pub /: *mut *mut __u32 weight_set_size; /!< size of the __weight_set__ array,
}

// @ingroup API
//
// Replacement weights and ids for each bucket in the crushmap. The
// __size__ of the __args__ array must be exactly the same as the
// __map->max_buckets__.
//
// The __crush_choose_arg__ at index N will be used when choosing
// an item from the bucket __map->buckets[N]__ bucket, provided it
// is a straw2 bucket.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_choose_arg_map {

    pub node: rb_node,
    pub choose_args_index: i64,

    pub bucket: *mut *mut *mut crush_choose_arg args; /!< replacement for each,
    pub /: *mut *mut __u32 size; /!< size of the __args__ array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_bucket_uniform {
    pub h: crush_bucket,
    pub /: *mut *mut __u32 item_weight; / 16-bit fixed point; all items equally weighted,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_bucket_list {
    pub h: crush_bucket,
    pub /: *mut *mut *mut __u32 item_weights; / 16-bit fixed point,
    pub sum: *mut *mut *mut __u32 sum_weights; / 16-bit fixed point. element i is,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_bucket_tree {
    pub of: *mut *mut crush_bucket h; / note: h.size is _tree_ size, not number,
    pub num_nodes: __u8,
    pub node_weights: *mut __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_bucket_straw {
    pub h: crush_bucket,
    pub /: *mut *mut *mut __u32 item_weights; / 16-bit fixed point,
    pub /: *mut *mut *mut __u32 straws; / 16-bit fixed point,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_bucket_straw2 {
    pub h: crush_bucket,
    pub /: *mut *mut *mut __u32 item_weights; / 16-bit fixed point,
}

//
// CRUSH map includes all buckets, rules, etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_map {
    pub buckets: *mut crush_bucket,
    pub rules: *mut crush_rule,
    pub max_buckets: __s32,
    pub max_rules: __u32,
    pub max_devices: __s32,
// choose local retries before re-descent
    pub choose_local_tries: __u32,
// choose local attempts using a fallback permutation before
// re-descent
    pub choose_local_fallback_tries: __u32,
// choose attempts before giving up
    pub choose_total_tries: __u32,
// attempt chooseleaf inner descent once for firstn mode; on
// reject retry outer descent.  Note that this does *not
// apply to a collision: in that case we will retry as we used
// to.
    pub chooseleaf_descend_once: __u32,
// if non-zero, feed r into chooseleaf, bit-shifted right by (r-1)
// bits.  a value of 1 is best for new clusters.  for legacy clusters
// that want to limit reshuffling, a value of 3 or 4 will make the
// mappings line up a bit better with previous mappings.
    pub chooseleaf_vary_r: __u8,
// if true, it makes chooseleaf firstn to return stable results (if
// no local retry) so that data migrations would be optimal when some
// device fails.
    pub chooseleaf_stable: __u8,
//
// This value is calculated after decode or construction by
// the builder. It is exposed here (rather than having a
// 'build CRUSH working space' function) so that callers can
// reserve a static buffer, allocate space on the stack, or
// otherwise avoid calling into the heap allocator if they
// want to. The size of the working space depends on the map,
// while the size of the scratch vector passed to the mapper
// depends on the size of the desired result set.
//
// Nothing stops the caller from allocating both in one swell
// foop and passing in two points, though.
//
    pub working_size: usize,
//
// version 0 (original) of straw_calc has various flaws.  version 1
// fixes a few of them.
//
    pub straw_calc_version: __u8,
//
// allowed bucket algs is a bitmask, here the bit positions
// are CRUSH_BUCKET_*.  note that these are *bits* and
// CRUSH_BUCKET_* values are not, so we need to or together (1
// << CRUSH_BUCKET_WHATEVER).  The 0th bit is not used to
// minimize confusion (bucket type values start at 1).
//
    pub allowed_bucket_algs: __u32,
    pub choose_tries: *mut __u32,

// device/bucket type id -> type name (CrushWrapper::type_map)
    pub type_names: rb_root,
// device/bucket id -> name (CrushWrapper::name_map)
    pub names: rb_root,
// CrushWrapper::choose_args
    pub choose_args: rb_root,

}

// crush.c
extern "C" {
    pub fn crush_get_bucket_item_weight(b: *const crush_bucket, pos: c_int) -> c_int;
}
extern "C" {
    pub fn crush_destroy_bucket_uniform(b: *mut crush_bucket_uniform);
}
extern "C" {
    pub fn crush_destroy_bucket_list(b: *mut crush_bucket_list);
}
extern "C" {
    pub fn crush_destroy_bucket_tree(b: *mut crush_bucket_tree);
}
extern "C" {
    pub fn crush_destroy_bucket_straw(b: *mut crush_bucket_straw);
}
extern "C" {
    pub fn crush_destroy_bucket_straw2(b: *mut crush_bucket_straw2);
}
extern "C" {
    pub fn crush_destroy_bucket(b: *mut crush_bucket);
}
extern "C" {
    pub fn crush_destroy_rule(r: *mut crush_rule);
}
extern "C" {
    pub fn crush_destroy(map: *mut crush_map);
}
//
// These data structures are private to the CRUSH implementation. They
// are exposed in this header file because builder needs their
// definitions to calculate the total working size.
//
// Moving this out of the crush map allow us to treat the CRUSH map as
// immutable within the mapper and removes the requirement for a CRUSH
// map lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_work_bucket {
    pub /: *mut *mut *mut __u32 perm_x; / @x for which perm is defined,
    pub /: *mut *mut *mut __u32 perm_n; / num elements of perm that are permuted/defined,
    pub /: *mut *mut *mut __u32 perm; / Permutation of the bucket's items,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_work {
    pub /: *mut *mut *mut *mut crush_work_bucket work; / Per-bucket working store,

    pub item: list_head,

}

// osdmap.c
extern "C" {
    pub fn clear_crush_names(root: *mut rb_root);
}
extern "C" {
    pub fn clear_choose_args(c: *mut crush_map);
}

