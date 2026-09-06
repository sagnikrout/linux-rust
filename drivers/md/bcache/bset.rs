//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/bset.h
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
// BKEYS:
//
// A bkey contains a key, a size field, a variable number of pointers, and some
// ancillary flag bits.
//
// We use two different functions for validating bkeys, bch_ptr_invalid and
// bch_ptr_bad().
//
// bch_ptr_invalid() primarily filters out keys and pointers that would be
// invalid due to some sort of bug, whereas bch_ptr_bad() filters out keys and
// pointer that occur in normal practice but don't point to real data.
//
// The one exception to the rule that ptr_invalid() filters out invalid keys is
// that it also filters out keys of size 0 - these are keys that have been
// completely overwritten. It'd be safe to delete these in memory while leaving
// them on disk, just unnecessary work - so we filter them out when resorting
// instead.
//
// We can't filter out stale keys when we're resorting, because garbage
// collection needs to find them to ensure bucket gens don't wrap around -
// unless we're rewriting the btree node those stale keys still exist on disk.
//
// We also implement functions here for removing some number of sectors from the
// front or the back of a bkey - this is mainly used for fixing overlapping
// extents, by removing the overlapping sectors from the older key.
//
// BSETS:
//
// A bset is an array of bkeys laid out contiguously in memory in sorted order,
// along with a header. A btree node is made up of a number of these, written at
// different times.
//
// There could be many of them on disk, but we never allow there to be more than
// 4 in memory - we lazily resort as needed.
//
// We implement code here for creating and maintaining auxiliary search trees
// (described below) for searching an individial bset, and on top of that we
// implement a btree iterator.
//
// BTREE ITERATOR:
//
// Most of the code in bcache doesn't care about an individual bset - it needs
// to search entire btree nodes and iterate over them in sorted order.
//
// The btree iterator code serves both functions; it iterates through the keys
// in a btree node in sorted order, starting from either keys after a specific
// point (if you pass it a search key) or the start of the btree node.
//
// AUXILIARY SEARCH TREES:
//
// Since keys are variable length, we can't use a binary search on a bset - we
// wouldn't be able to find the start of the next key. But binary searches are
// slow anyways, due to terrible cache behaviour; bcache originally used binary
// searches and that code topped out at under 50k lookups/second.
//
// So we need to construct some sort of lookup table. Since we only insert keys
// into the last (unwritten) set, most of the keys within a given btree node are
// usually in sets that are mostly constant. We use two different types of
// lookup tables to take advantage of this.
//
// Both lookup tables share in common that they don't index every key in the
// set; they index one key every BSET_CACHELINE bytes, and then a linear search
// is used for the rest.
//
// For sets that have been written to disk and are no longer being inserted
// into, we construct a binary search tree in an array - traversing a binary
// search tree in an array gives excellent locality of reference and is very
// fast, since both children of any node are adjacent to each other in memory
// (and their grandchildren, and great grandchildren...) - this means
// prefetching can be used to great effect.
//
// It's quite useful performance wise to keep these nodes small - not just
// because they're more likely to be in L2, but also because we can prefetch
// more nodes on a single cacheline and thus prefetch more iterations in advance
// when traversing this tree.
//
// Nodes in the auxiliary search tree must contain both a key to compare against
// (we don't want to fetch the key from the set, that would defeat the purpose),
// and a pointer to the key. We use a few tricks to compress both of these.
//
// To compress the pointer, we take advantage of the fact that one node in the
// search tree corresponds to precisely BSET_CACHELINE bytes in the set. We have
// a function (to_inorder()) that takes the index of a node in a binary tree and
// returns what its index would be in an inorder traversal, so we only have to
// store the low bits of the offset.
//
// The key is 84 bits (KEY_DEV + key->key, the offset on the device). To
// compress that,  we take advantage of the fact that when we're traversing the
// search tree at every iteration we know that both our search key and the key
// we're looking for lie within some range - bounded by our previous
// comparisons. (We special case the start of a search so that this is true even
// at the root of the tree).
//
// So we know the key we're looking for is between a and b, and a and b don't
// differ higher than bit 50, we don't need to check anything higher than bit
// 50.
//
// We don't usually need the rest of the bits, either; we only need enough bits
// to partition the key range we're currently checking.  Consider key n - the
// key our auxiliary search tree node corresponds to, and key p, the key
// immediately preceding n.  The lowest bit we need to store in the auxiliary
// search tree is the highest bit that differs between n and p.
//
// Note that this could be bit 0 - we might sometimes need all 80 bits to do the
// comparison. But we'd really like our nodes in the auxiliary search tree to be
// of fixed size.
//
// The solution is to make them fixed size, and when we're constructing a node
// check if p and n differed in the bits we needed them to. If they don't we
// flag that node, and when doing lookups we fallback to comparing against the
// real key. As long as this doesn't happen to often (and it seems to reliably
// happen a bit less than 1% of the time), we win - even on failures, that key
// is then more likely to be in cache than if we were doing binary searches all
// the way, since we're touching so much less memory.
//
// The keys in the auxiliary search tree are stored in (software) floating
// point, with an exponent and a mantissa. The exponent needs to be big enough
// to address all the bits in the original key, but the number of bits in the
// mantissa is somewhat arbitrary; more bits just gets us fewer failures.
//
// We need 7 bits for the exponent and 3 bits for the key's offset (since keys
// are 8 byte aligned); using 22 bits for the mantissa means a node is 4 bytes.
// We need one node per 128 bytes in the btree node, which means the auxiliary
// search trees take up 3% as much memory as the btree itself.
//
// Constructing these auxiliary search trees is moderately expensive, and we
// don't want to be constantly rebuilding the search tree for the last set
// whenever we insert another key into it. For the unwritten set, we use a much
// simpler lookup table - it's just a flat array, so index i in the lookup table
// corresponds to the i range of BSET_CACHELINE bytes in the set. Indexing
// within each byte range works the same as with the auxiliary search trees.
//
// These are much easier to keep up to date when we insert a key - we do it
// somewhat lazily; when we shift a key up we usually just increment the pointer
// to it, only when it would overflow do we go to the trouble of finding the
// first key in that range of bytes again.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bset_tree {
//
// We construct a binary tree in an array as if the array
// started at 1, so that things line up on the same cachelines
// better: see comments in bset.c at cacheline_to_bkey() for
// details
//
// size of the binary tree and prev array
    pub size: c_uint,
// function of size - precalculated for to_inorder()
    pub extra: c_uint,
// copy of the last key in the set
    pub end: bkey,
    pub tree: *mut bkey_float,
//
// The nodes in the bset tree point to specific keys - this
// array holds the sizes of the previous key.
//
// Conceptually it's a member of struct bkey_float, but we want
// to keep bkey_float to 4 bytes and prev isn't used in the fast
// path.
//
    pub prev: *mut u8,
// The actual btree node, with pointers to each sorted set
    pub data: *mut bset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_keys_ops {
    pub r): btree_iter_set,
    pub tmp): *mut bkey,
    pub replace_key): *mut bkey,
    pub k): *const bkey,
    pub k): *const bkey,
    pub r): *mut *mut bkey l, bkey,
    pub k): *const bkey,
    pub k): *const bkey,
//
// Only used for deciding whether to use START_KEY(k) or just the key
// itself in a couple places
//
    pub is_extents: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_keys {
    pub ops: *const btree_keys_ops,
    pub page_order: u8,
    pub nsets: u8,
    pub last_set_unwritten:1: c_uint,
    pub expensive_debug_checks: *mut bool,
//
// Sets of sorted keys - the real btree node - plus a binary search tree
//
// set[0] is special; set[0]->tree, set[0]->prev and set[0]->data point
// to the memory we have allocated for this btree node. Additionally,
// set[0]->data points to the entire btree node as it exists on disk.
//
    pub set: [bset_tree; MAX_BSETS],
}

extern "C" {
    pub fn bch_btree_keys_free(b: *mut btree_keys);
}
extern "C" {
    pub fn bch_bset_init_next(b: *mut btree_keys, i: *mut bset, magic: u64);
}
extern "C" {
    pub fn bch_bset_build_written_tree(b: *mut btree_keys);
}
extern "C" {
    pub fn bch_bset_fix_invalidated_key(b: *mut btree_keys, k: *mut bkey);
}
extern "C" {
    pub fn bch_bkey_try_merge(b: *mut btree_keys, l: *mut bkey, r: *mut bkey) -> bool;
}
// Btree key iteration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_iter {
    pub used: size_t size,,

    pub b: *mut btree_keys,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_iter_set {
    pub end: *mut *mut bkey k,,
    pub data: [}; ],
}

// Fixed-size btree_iter that can be allocated on the stack
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_iter_stack {
// Must be last as it ends in a flexible-array member.
    pub stack_data: [btree_iter_set; MAX_BSETS],
}

extern "C" {
    pub fn bool(b: *mut *mut ptr_filter_fn)(struct btree_keys, k: *const bkey) -> typedef;
}
//
// Returns the first key that is strictly greater than search
//

// Sorting
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bset_sort_state {
    pub pool: mempool_t,
    pub page_order: c_uint,
    pub crit_factor: c_uint,
    pub time: time_stats,
}

extern "C" {
    pub fn bch_bset_sort_state_free(state: *mut bset_sort_state);
}
extern "C" {
    pub fn bch_btree_sort_lazy(b: *mut btree_keys, state: *mut bset_sort_state);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bset_stats {
    pub sets_unwritten: size_t sets_written,,
    pub bytes_unwritten: size_t bytes_written,,
    pub failed: size_t floats,,
}

extern "C" {
    pub fn bch_btree_keys_stats(b: *mut btree_keys, state: *mut bset_stats);
}
// Bkey utility code

extern "C" {
    pub fn bkey_idx(_arg: i->start, _arg: idx) -> return;
}
// k = ZERO_KEY;
extern "C" {
    pub fn __bch_cut_front(where: *const bkey, k: *mut bkey) -> bool;
}
extern "C" {
    pub fn __bch_cut_back(where: *const bkey, k: *mut bkey) -> bool;
}
extern "C" {
    pub fn __bch_cut_front(_arg: where, _arg: k) -> return;
}
extern "C" {
    pub fn __bch_cut_back(_arg: where, _arg: k) -> return;
}
//
// Pointer '*preceding_key_p' points to a memory object to store preceding
// key of k. If the preceding key does not exist, set '*preceding_key_p' to
// NULL. So the caller of preceding_key() needs to take care of memory
// which '*preceding_key_p' pointed to before calling preceding_key().
// Currently the only caller of preceding_key() is bch_btree_insert_key(),
// and it points to an on-stack variable, so the memory release is handled
// by stackframe itself.
//
// Keylists
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keylist {
    pub keys: *mut bkey,
    pub keys_p: *mut u64,
}

// Enough room for btree_split's keys without realloc
pub const KEYLIST_INLINE: c_int = 16;
extern "C" {
    pub fn bch_keylist_nkeys(sizeof(uint64_t: *mut *mut l)) -> return;
}
extern "C" {
    pub fn bch_keylist_pop_front(l: *mut keylist);
}
extern "C" {
    pub fn __bch_keylist_realloc(l: *mut keylist, u64s: c_uint) -> c_int;
}
// Debug stuff

extern "C" {
    pub fn __bch_count_data(b: *mut btree_keys) -> c_int;
}
extern "C" {
    pub fn bch_dump_bset(b: *mut btree_keys, i: *mut bset, set: c_uint);
}
extern "C" {
    pub fn bch_dump_bucket(b: *mut btree_keys);
}

extern "C" {
    pub fn bch_dump_bset(b: *mut btree_keys, i: *mut bset, set: c_uint);
}

