//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/journal.h
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
// THE JOURNAL:
//
// The journal is treated as a circular buffer of buckets - a journal entry
// never spans two buckets. This means (not implemented yet) we can resize the
// journal at runtime, and will be needed for bcache on raw flash support.
//
// Journal entries contain a list of keys, ordered by the time they were
// inserted; thus journal replay just has to reinsert the keys.
//
// We also keep some things in the journal header that are logically part of the
// superblock - all the things that are frequently updated. This is for future
// bcache on raw flash support; the superblock (which will become another
// journal) can't be moved or wear leveled, so it contains just enough
// information to find the main journal, and the superblock only has to be
// rewritten when we want to move/wear level the main journal.
//
// Currently, we don't journal BTREE_REPLACE operations - this will hopefully be
// fixed eventually. This isn't a bug - BTREE_REPLACE is used for insertions
// from cache misses, which don't have to be journaled, and for writeback and
// moving gc we work around it by flushing the btree to disk before updating the
// gc information. But it is a potential issue with incremental garbage
// collection, and it's fragile.
//
// OPEN JOURNAL ENTRIES:
//
// Each journal entry contains, in the header, the sequence number of the last
// journal entry still open - i.e. that has keys that haven't been flushed to
// disk in the btree.
//
// We track this by maintaining a refcount for every open journal entry, in a
// fifo; each entry in the fifo corresponds to a particular journal
// entry/sequence number. When the refcount at the tail of the fifo goes to
// zero, we pop it off - thus, the size of the fifo tells us the number of open
// journal entries
//
// We take a refcount on a journal entry when we add some keys to a journal
// entry that we're going to insert (held by struct btree_op), and then when we
// insert those keys into the btree the btree write we're setting up takes a
// copy of that refcount (held by struct btree_write). That refcount is dropped
// when the btree write completes.
//
// A struct btree_write can only hold a refcount on a single journal entry, but
// might contain keys for many journal entries - we handle this by making sure
// it always has a refcount on the _oldest_ journal entry of all the journal
// entries it has keys for.
//
// JOURNAL RECLAIM:
//
// As mentioned previously, our fifo of refcounts tells us the number of open
// journal entries; from that and the current journal sequence number we compute
// last_seq - the oldest journal entry we still need. We write last_seq in each
// journal entry, and we also have to keep track of where it exists on disk so
// we don't overwrite it when we loop around the journal.
//
// To do that we track, for each journal bucket, the sequence number of the
// newest journal entry it contains - if we don't need that journal entry we
// don't need anything in that bucket anymore. From that we track the last
// journal bucket we still need; all this is tracked in struct journal_device
// and updated by journal_reclaim().
//
// JOURNAL FILLING UP:
//
// There are two ways the journal could fill up; either we could run out of
// space to write to, or we could have too many open journal entries and run out
// of room in the fifo of refcounts. Since those refcounts are decremented
// without any locking we can't safely resize that fifo, so we handle it the
// same way.
//
// If the journal fills up, we start flushing dirty btree nodes until we can
// allocate space for a journal write again - preferentially flushing btree
// nodes that are pinning the oldest journal entries first.
//
// Only used for holding the journal entries we read in btree_journal_read()
// during cache_registration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct journal_replay {
    pub list: list_head,
    pub pin: *mut core::sync::atomic::AtomicI32,
    pub j: jset,
}

//
// We put two of these in struct journal; we used them for writes to the
// journal that are being staged or in flight.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct journal_write {
    pub data: *mut jset,
pub const JSET_BITS: c_int = 3;
    pub c: *mut cache_set,
    pub wait: closure_waitlist,
    pub dirty: bool,
    pub need_write: bool,
}

// Embedded in struct cache_set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct journal {
    pub lock: spinlock_t,
    pub flush_write_lock: spinlock_t,
    pub btree_flushing: bool,
    pub do_reserve: bool,
// used when waiting because the journal was full
    pub wait: closure_waitlist,
    pub io: closure,
    pub io_in_flight: c_int,
    pub work: delayed_work,
// Number of blocks free in the bucket(s) we're currently writing to
    pub blocks_free: c_uint,
    pub seq: u64,
    pub pin): DECLARE_FIFO(atomic_t,,
    pub cur: *mut journal_write w[2],,
}

//
// Embedded in struct cache. First three fields refer to the array of journal
// buckets, in cache_sb.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct journal_device {
//
// For each journal bucket, contains the max sequence number of the
// journal writes it contains - so we know when a bucket can be reused.
//
    pub seq: [u64; SB_JOURNAL_BUCKETS],
// Journal bucket we're currently writing to
    pub cur_idx: c_uint,
// Last journal bucket that still contains an open journal entry
    pub last_idx: c_uint,
// Bio for journal reads/writes to this device
    pub bio: bio,
    pub bv: [bio_vec; 8],
}

pub const BTREE_FLUSH_NR: c_int = 8;

pub const JOURNAL_PIN: c_int = 20000;

extern "C" {
    pub fn bch_journal_next(j: *mut journal);
}
extern "C" {
    pub fn bch_journal_mark(c: *mut cache_set, list: *mut list_head);
}
extern "C" {
    pub fn bch_journal_meta(c: *mut cache_set, cl: *mut closure);
}
extern "C" {
    pub fn bch_journal_read(c: *mut cache_set, list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn bch_journal_replay(c: *mut cache_set, list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn bch_journal_free(c: *mut cache_set);
}
extern "C" {
    pub fn bch_journal_alloc(c: *mut cache_set) -> c_int;
}
extern "C" {
    pub fn bch_journal_space_reserve(j: *mut journal);
}
