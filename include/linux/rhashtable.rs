//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rhashtable.h
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
// Resizable, Scalable, Concurrent Hash Table
//
// Copyright (c) 2015-2016 Herbert Xu <herbert@gondor.apana.org.au>
// Copyright (c) 2014-2015 Thomas Graf <tgraf@suug.ch>
// Copyright (c) 2008-2014 Patrick McHardy <kaber@trash.net>
//
// Code partially derived from nft_hash
// Rewritten with rehash code from br_multicast plus single list
// pointer as suggested by Josh Triplett
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

//
// Objects in an rhashtable have an embedded struct rhash_head
// which is linked into as hash chain from the hash table - or one
// of two or more hash tables when the rhashtable is being resized.
// The end of the chain is marked with a special nulls marks which has
// the least significant bit set but otherwise stores the address of
// the hash bucket.  This allows us to be sure we've found the end
// of the right list.
// The value stored in the hash bucket has BIT(0) used as a lock bit.
// This bit must be atomically set before any changes are made to
// the chain.  To avoid dereferencing this pointer without clearing
// the bit first, we use an opaque 'struct rhash_lock_head *' for the
// pointer stored in the bucket.  This struct needs to be defined so
// that rcu_dereference() works on it, but it has no content so a
// cast is needed for it to be useful.  This ensures it isn't
// used by mistake with clearing the lock bit first.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhash_lock_head {
// Maximum chain length before rehash
//
// The maximum (not average) chain length grows with the size of the hash
// table, at a rate of (log N)/(log log N).
//
// The value of 16 is selected so that even if the hash table grew to
// 2^32 you would not expect the maximum chain length to exceed it
// unless we are under attack (or extremely unlucky).
//
// As this limit is only to detect attacks, we don't need to set it to a
// lower value as you'd need the chain length to vastly exceed 16 to have
// any real effect on the system.
//

//
// struct bucket_table - Table of hash buckets
// @size: Number of hash buckets
// @nest: Number of bits of first-level nested table.
// @rehash: Current bucket being rehashed
// @hash_rnd: Random seed to fold into hash
// @walkers: List of active walkers
// @rcu: RCU structure for freeing the table
// @future_tbl: Table under construction during rehashing
// @ntbl: Nested table used when out of memory.
// @buckets: size * hash buckets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bucket_table {
    pub size: c_uint,
    pub nest: c_uint,
    pub hash_rnd: u32,
    pub walkers: list_head,
    pub rcu: rcu_head,
    pub future_tbl: *mut bucket_table __rcu,
    pub dep_map: lockdep_map,
    pub ____cacheline_aligned_in_smp: *mut *mut rhash_lock_head __rcu buckets[],
}

//
// NULLS_MARKER() expects a hash value with the low
// bits mostly likely to be significant, and it discards
// the msb.
// We give it an address, in which the bottom bit is
// always 0, and the msb might be significant.
// So we shift the address down one bit to align with
// expectations and avoid losing a significant bit.
//
// We never store the NULLS_MARKER in the hash table
// itself as we need the lsb for locking.
// Instead we store a NULL
//

// params must be equal to ht->p if it isn't constant.
extern "C" {
    pub fn rht_bucket_index(_arg: tbl, _arg: hash) -> return;
}
//
// rht_grow_above_75 - returns true if nelems > 0.75 * table-size
// @ht:		hash table
// @tbl:	current table
//
// Expand table when exceeding 75% load
//
// rht_shrink_below_30 - returns true if nelems < 0.3 * table-size
// @ht:		hash table
// @tbl:	current table
//
// Shrink table beneath 30% load
//
// rht_grow_above_100 - returns true if nelems > table-size
// @ht:		hash table
// @tbl:	current table
//
// rht_grow_above_max - returns true if table is above maximum
// @ht:		hash table
// @tbl:	current table
//

extern "C" {
    pub fn lockdep_rht_mutex_is_held(ht: *mut rhashtable) -> c_int;
}
extern "C" {
    pub fn lockdep_rht_bucket_is_held(tbl: *const bucket_table, hash: u32) -> c_int;
}

extern "C" {
    pub fn rhashtable_walk_exit(iter: *mut rhashtable_iter);
}
extern "C" {
    pub fn rhashtable_walk_start_check(__acquires_shared(RCU: *mut *mut rhashtable_iter iter)) -> c_int;
}
extern "C" {
    pub fn rhashtable_walk_stop(__releases_shared(RCU: *mut *mut rhashtable_iter iter));
}
extern "C" {
    pub fn rhashtable_destroy(ht: *mut rhashtable);
}

//
// We lock a bucket by setting BIT(0) in the pointer - this is always
// zero in real pointers.  The NULLS mark is never stored in the bucket,
// rather we store NULL if the bucket is empty.
// bit_spin_locks do not handle contention well, but the whole point
// of the hashtable design is to achieve minimum per-bucket contention.
// A nested hash table might not have a bucket pointer.  In that case
// we cannot get a lock.  For remove and replace the bucket cannot be
// interesting and doesn't need locking.
// For insert we allocate the bucket if this is the last bucket_table,
// and then take the lock.
// Sometimes we unlock a bucket by writing a new pointer there.  In that
// case we don't need to unlock, but we do need to reset state such as
// local_bh. For that we have rht_assign_unlock().  As rcu_assign_pointer()
// provides the same release semantics that bit_spin_unlock() provides,
// this is safe.
// When we write to a bucket without unlocking, we use rht_assign_locked().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rht_lookup_freq {
    RHT_LOOKUP_NORMAL,
    RHT_LOOKUP_LIKELY,
}

//
// Where 'bkt' is a bucket and might be locked:
// rht_ptr_rcu() dereferences that pointer and clears the lock bit.
// rht_ptr() dereferences in a context where the bucket is locked.
// rht_ptr_exclusive() dereferences in a context where exclusive
// access is guaranteed, such as when destroying the table.
//
extern "C" {
    pub fn __rht_ptr(_arg: *mut rcu_dereference_all(bkt), _arg: bkt, _arg: freq) -> return;
}
extern "C" {
    pub fn __rht_ptr_rcu(_arg: bkt, _arg: RHT_LOOKUP_NORMAL) -> return;
}
//
// rht_for_each_from - iterate over hash chain from given head
// @pos:	the &struct rhash_head to use as a loop cursor.
// @head:	the &struct rhash_head to start from
// @tbl:	the &struct bucket_table
// @hash:	the hash value / bucket index
//

//
// rht_for_each - iterate over hash chain
// @pos:	the &struct rhash_head to use as a loop cursor.
// @tbl:	the &struct bucket_table
// @hash:	the hash value / bucket index
//

//
// rht_for_each_entry_from - iterate over hash chain from given head
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct rhash_head to use as a loop cursor.
// @head:	the &struct rhash_head to start from
// @tbl:	the &struct bucket_table
// @hash:	the hash value / bucket index
// @member:	name of the &struct rhash_head within the hashable struct.
//

//
// rht_for_each_entry - iterate over hash chain of given type
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct rhash_head to use as a loop cursor.
// @tbl:	the &struct bucket_table
// @hash:	the hash value / bucket index
// @member:	name of the &struct rhash_head within the hashable struct.
//

//
// rht_for_each_entry_safe - safely iterate over hash chain of given type
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct rhash_head to use as a loop cursor.
// @next:	the &struct rhash_head to use as next in loop cursor.
// @tbl:	the &struct bucket_table
// @hash:	the hash value / bucket index
// @member:	name of the &struct rhash_head within the hashable struct.
//
// This hash chain list-traversal primitive allows for the looped code to
// remove the loop cursor from the list.
//

//
// rht_for_each_rcu_from - iterate over rcu hash chain from given head
// @pos:	the &struct rhash_head to use as a loop cursor.
// @head:	the &struct rhash_head to start from
// @tbl:	the &struct bucket_table
// @hash:	the hash value / bucket index
//
// This hash chain list-traversal primitive may safely run concurrently with
// the _rcu mutation primitives such as rhashtable_insert() as long as the
// traversal is guarded by rcu_read_lock().
//

//
// rht_for_each_rcu - iterate over rcu hash chain
// @pos:	the &struct rhash_head to use as a loop cursor.
// @tbl:	the &struct bucket_table
// @hash:	the hash value / bucket index
//
// This hash chain list-traversal primitive may safely run concurrently with
// the _rcu mutation primitives such as rhashtable_insert() as long as the
// traversal is guarded by rcu_read_lock().
//

//
// rht_for_each_entry_rcu_from - iterated over rcu hash chain from given head
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct rhash_head to use as a loop cursor.
// @head:	the &struct rhash_head to start from
// @tbl:	the &struct bucket_table
// @hash:	the hash value / bucket index
// @member:	name of the &struct rhash_head within the hashable struct.
//
// This hash chain list-traversal primitive may safely run concurrently with
// the _rcu mutation primitives such as rhashtable_insert() as long as the
// traversal is guarded by rcu_read_lock().
//

//
// rht_for_each_entry_rcu - iterate over rcu hash chain of given type
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct rhash_head to use as a loop cursor.
// @tbl:	the &struct bucket_table
// @hash:	the hash value / bucket index
// @member:	name of the &struct rhash_head within the hashable struct.
//
// This hash chain list-traversal primitive may safely run concurrently with
// the _rcu mutation primitives such as rhashtable_insert() as long as the
// traversal is guarded by rcu_read_lock().
//

//
// rhl_for_each_rcu - iterate over rcu hash table list
// @pos:	the &struct rlist_head to use as a loop cursor.
// @list:	the head of the list
//
// This hash chain list-traversal primitive should be used on the
// list returned by rhltable_lookup.
//

//
// rhl_for_each_entry_rcu - iterate over rcu hash table list of given type
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct rlist_head to use as a loop cursor.
// @list:	the head of the list
// @member:	name of the &struct rlist_head within the hashable struct.
//
// This hash chain list-traversal primitive should be used on the
// list returned by rhltable_lookup.
//

extern "C" {
    pub fn memcmp(ht->p.key_offset: ptr +, _arg: arg->key, _arg: ht->p.key_len) -> return;
}
// Internal function, do not use.
// An object might have been moved to a different hash chain,
// while we walk along it - better check and retry.
//
// Ensure we see any new tables.
//
// rhashtable_lookup - search hash table
// @ht:		hash table
// @key:	the pointer to the key
// @params:	hash table parameters
//
// Computes the hash value for the key and traverses the bucket chain looking
// for an entry with an identical key. The first matching entry is returned.
//
// This must only be called under the RCU read lock.
//
// Returns the first entry on which the compare function returned true.
//
// rhashtable_lookup_fast - search hash table, without RCU read lock
// @ht:		hash table
// @key:	the pointer to the key
// @params:	hash table parameters
//
// Computes the hash value for the key and traverses the bucket chain looking
// for an entry with an identical key. The first matching entry is returned.
//
// Only use this function when you have other mechanisms guaranteeing
// that the object won't go away after the RCU read lock is released.
//
// Returns the first entry on which the compare function returned true.
//
// rhltable_lookup - search hash list table
// @hlt:	hash table
// @key:	the pointer to the key
// @params:	hash table parameters
//
// Computes the hash value for the key and traverses the bucket chain looking
// for an entry with an identical key.  All matching entries are returned
// in a list.
//
// This must only be called under the RCU read lock.
//
// Returns the list of entries that match the given key.
//
// Internal function, please use rhashtable_insert_fast() instead. This
// function returns the existing element already in hashes if there is a clash,
// otherwise it returns an error via ERR_PTR().
//
extern "C" {
    pub fn rhashtable_insert_slow(_arg: ht, _arg: key, _arg: obj) -> return;
}
// Inserting at head of list makes unlocking free.
//
// rhashtable_insert_fast - insert object into hash table
// @ht:		hash table
// @obj:	pointer to hash head inside object
// @params:	hash table parameters
//
// Will take the per bucket bitlock to protect against mutual mutations
// on the same bucket. Multiple insertions may occur in parallel unless
// they map to the same bucket.
//
// It is safe to call this function from atomic context.
//
// Will trigger an automatic deferred table resizing if residency in the
// table grows beyond 70%.
//
extern "C" {
    pub fn PTR_ERR(_arg: ret) -> return;
}
//
// rhltable_insert_key - insert object into hash list table
// @hlt:	hash list table
// @key:	the pointer to the key
// @list:	pointer to hash list head inside object
// @params:	hash table parameters
//
// Will take the per bucket bitlock to protect against mutual mutations
// on the same bucket. Multiple insertions may occur in parallel unless
// they map to the same bucket.
//
// It is safe to call this function from atomic context.
//
// Will trigger an automatic deferred table resizing if residency in the
// table grows beyond 70%.
//
// rhltable_insert - insert object into hash list table
// @hlt:	hash list table
// @list:	pointer to hash list head inside object
// @params:	hash table parameters
//
// Will take the per bucket bitlock to protect against mutual mutations
// on the same bucket. Multiple insertions may occur in parallel unless
// they map to the same bucket.
//
// It is safe to call this function from atomic context.
//
// Will trigger an automatic deferred table resizing if residency in the
// table grows beyond 70%.
//
extern "C" {
    pub fn rhltable_insert_key(_arg: hlt, _arg: key, _arg: list, _arg: params) -> return;
}
//
// rhashtable_lookup_insert_fast - lookup and insert object into hash table
// @ht:		hash table
// @obj:	pointer to hash head inside object
// @params:	hash table parameters
//
// This lookup function may only be used for fixed key hash table (key_len
// parameter set). It will BUG() if used inappropriately.
//
// It is safe to call this function from atomic context.
//
// Will trigger an automatic deferred table resizing if residency in the
// table grows beyond 70%.
//
extern "C" {
    pub fn PTR_ERR(_arg: ret) -> return;
}
//
// rhashtable_lookup_get_insert_fast - lookup and insert object into hash table
// @ht:		hash table
// @obj:	pointer to hash head inside object
// @params:	hash table parameters
//
// Just like rhashtable_lookup_insert_fast(), but this function returns the
// object if it exists, NULL if it did not and the insertion was successful,
// and an ERR_PTR otherwise.
//
// rhashtable_lookup_insert_key - search and insert object to hash table
// with explicit key
// @ht:		hash table
// @key:	key
// @obj:	pointer to hash head inside object
// @params:	hash table parameters
//
// Lookups may occur in parallel with hashtable mutations and resizing.
//
// Will trigger an automatic deferred table resizing if residency in the
// table grows beyond 70%.
//
// Returns zero on success.
//
extern "C" {
    pub fn PTR_ERR(_arg: ret) -> return;
}
//
// rhashtable_lookup_get_insert_key - lookup and insert object into hash table
// @ht:		hash table
// @key:	key
// @obj:	pointer to hash head inside object
// @params:	hash table parameters
//
// Just like rhashtable_lookup_insert_key(), but this function returns the
// object if it exists, NULL if it does not and the insertion was successful,
// and an ERR_PTR otherwise.
//
extern "C" {
    pub fn __rhashtable_insert_fast(_arg: ht, _arg: key, _arg: obj, _arg: params, _arg: false) -> return;
}
// Internal function, please use rhashtable_remove_fast() instead
// Because we have already taken (and released) the bucket
// lock in old_tbl, if we find that future_tbl is not yet
// visible then that guarantees the entry to still be in
// the old tbl if it exists.
//
// rhashtable_remove_fast - remove object from hash table
// @ht:		hash table
// @obj:	pointer to hash head inside object
// @params:	hash table parameters
//
// Since the hash chain is single linked, the removal operation needs to
// walk the bucket chain upon removal. The removal operation is thus
// considerable slow if the hash table is not correctly sized.
//
// Will automatically shrink the table if permitted when residency drops
// below 30%.
//
// Returns zero on success, -ENOENT if the entry could not be found.
//
extern "C" {
    pub fn __rhashtable_remove_fast(_arg: ht, _arg: obj, _arg: params, _arg: false) -> return;
}
//
// rhltable_remove - remove object from hash list table
// @hlt:	hash list table
// @list:	pointer to hash list head inside object
// @params:	hash table parameters
//
// Since the hash chain is single linked, the removal operation needs to
// walk the bucket chain upon removal. The removal operation is thus
// considerably slower if the hash table is not correctly sized.
//
// Will automatically shrink the table if permitted when residency drops
// below 30%
//
// Returns zero on success, -ENOENT if the entry could not be found.
//
extern "C" {
    pub fn __rhashtable_remove_fast(_arg: &hlt->ht, _arg: &list->rhead, _arg: params, _arg: true) -> return;
}
// Internal function, please use rhashtable_replace_fast() instead
// Minimally, the old and new objects must have same hash
// (which should mean identifiers are the same).
//
// rhashtable_replace_fast - replace an object in hash table
// @ht:		hash table
// @obj_old:	pointer to hash head inside object being replaced
// @obj_new:	pointer to hash head inside object which is new
// @params:	hash table parameters
//
// Replacing an object doesn't affect the number of elements in the hash table
// or bucket, so we don't need to worry about shrinking or expanding the
// table here.
//
// Returns zero on success, -ENOENT if the entry could not be found,
// -EINVAL if hash is not the same for the old and new objects.
//
// Because we have already taken (and released) the bucket
// lock in old_tbl, if we find that future_tbl is not yet
// visible then that guarantees the entry to still be in
// the old tbl if it exists.
//
// rhltable_walk_enter - Initialise an iterator
// @hlt:	Table to walk over
// @iter:	Hash table Iterator
//
// This function prepares a hash table walk.
//
// Note that if you restart a walk after rhashtable_walk_stop you
// may see the same object twice.  Also, you may miss objects if
// there are removals in between rhashtable_walk_stop and the next
// call to rhashtable_walk_start.
//
// For a completely stable walk you should construct your own data
// structure outside the hash table.
//
// This function may be called from any process context, including
// non-preemptable context, but cannot be called from softirq or
// hardirq context.
//
// You must call rhashtable_walk_exit after this function returns.
//
// rhltable_free_and_destroy - free elements and destroy hash list table
// @hlt:	the hash list table to destroy
// @free_fn:	callback to release resources of element
// @arg:	pointer passed to free_fn
//
// See documentation for rhashtable_free_and_destroy.
//
