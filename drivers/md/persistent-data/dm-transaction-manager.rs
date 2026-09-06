//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/persistent-data/dm-transaction-manager.h
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
// Copyright (C) 2011 Red Hat, Inc.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
//
// This manages the scope of a transaction.  It also enforces immutability
// of the on-disk data structures by limiting access to writeable blocks.
//
// Clients should not fiddle with the block manager directly.
//
extern "C" {
    pub fn dm_tm_destroy(tm: *mut dm_transaction_manager);
}
//
// The non-blocking version of a transaction manager is intended for use in
// fast path code that needs to do lookups e.g. a dm mapping function.
// You create the non-blocking variant from a normal tm.  The interface is
// the same, except that most functions will just return -EWOULDBLOCK.
// Methods that return void yet may block should not be called on a clone
// viz. dm_tm_inc, dm_tm_dec.  Call dm_tm_destroy() as you would with a normal
// tm when you've finished with it.  You may not destroy the original prior
// to clones.
//
// We use a 2-phase commit here.
//
// i) Make all changes for the transaction *except* for the superblock.
// Then call dm_tm_pre_commit() to flush them to disk.
//
// ii) Lock your superblock.  Update.  Then call dm_tm_commit() which will
// unlock the superblock and flush it.  No other blocks should be updated
// during this period.  Care should be taken to never unlock a partially
// updated superblock; perform any operations that could fail *before* you
// take the superblock lock.
//
extern "C" {
    pub fn dm_tm_pre_commit(tm: *mut dm_transaction_manager) -> c_int;
}
extern "C" {
    pub fn dm_tm_commit(tm: *mut dm_transaction_manager, superblock: *mut dm_block) -> c_int;
}
//
// These methods are the only way to get hold of a writeable block.
//
// dm_tm_new_block() is pretty self-explanatory.  Make sure you do actually
// write to the whole of @data before you unlock, otherwise you could get
// a data leak.  (The other option is for tm_new_block() to zero new blocks
// before handing them out, which will be redundant in most, if not all,
// cases).
// Zeroes the new block and returns with write lock held.
//
// dm_tm_shadow_block() allocates a new block and copies the data from @orig
// to it.  It then decrements the reference count on original block.  Use
// this to update the contents of a block in a data structure, don't
// confuse this with a clone - you shouldn't access the orig block after
// this operation.  Because the tm knows the scope of the transaction it
// can optimise requests for a shadow of a shadow to a no-op.  Don't forget
// to unlock when you've finished with the shadow.
//
// The @inc_children flag is used to tell the caller whether it needs to
// adjust reference counts for children.  (Data in the block may refer to
// other blocks.)
//
// Shadowing implicitly drops a reference on @orig so you must not have
// it locked when you call this.
//
// Read access.  You can lock any block you want.  If there's a write lock
// on it outstanding then it'll block.
//
extern "C" {
    pub fn dm_tm_unlock(tm: *mut dm_transaction_manager, b: *mut dm_block);
}
//
// Functions for altering the reference count of a block directly.
//
extern "C" {
    pub fn dm_tm_inc(tm: *mut dm_transaction_manager, b: dm_block_t);
}
extern "C" {
    pub fn dm_tm_inc_range(tm: *mut dm_transaction_manager, b: dm_block_t, e: dm_block_t);
}
extern "C" {
    pub fn dm_tm_dec(tm: *mut dm_transaction_manager, b: dm_block_t);
}
extern "C" {
    pub fn dm_tm_dec_range(tm: *mut dm_transaction_manager, b: dm_block_t, e: dm_block_t);
}
//
// Builds up runs of adjacent blocks, and then calls the given fn
// (typically dm_tm_inc/dec).  Very useful when you have to perform
// the same tm operation on all values in a btree leaf.
//
extern "C" {
    pub fn void(: *mut *mut dm_tm_run_fn)(struct dm_transaction_manager, _arg: dm_block_t, _arg: dm_block_t) -> typedef;
}
extern "C" {
    pub fn dm_tm_ref(tm: *mut dm_transaction_manager, b: dm_block_t, result: *mut u32) -> c_int;
}
//
// Finds out if a given block is shared (ie. has a reference count higher
// than one).
//
// If you're using a non-blocking clone the tm will build up a list of
// requested blocks that weren't in core.  This call will request those
// blocks to be prefetched.
//
extern "C" {
    pub fn dm_tm_issue_prefetches(tm: *mut dm_transaction_manager);
}
//
// A little utility that ties the knot by producing a transaction manager
// that has a space map managed by the transaction manager...
//
// Returns a tm that has an open transaction to write the new disk sm.
// Caller should store the new sm root and commit.
//
// The superblock location is passed so the metadata space map knows it
// shouldn't be used.
//
