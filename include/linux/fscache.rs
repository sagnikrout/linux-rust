//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fscache.h
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
// General filesystem caching interface
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// NOTE!!! See:
//
// Documentation/filesystems/caching/netfs-api.rst
//
// for a description of the network filesystem interface declared here.
//

pub const FSCACHE_ADV_SINGLE_CHUNK: c_uint = 0x01 /* The object is a single chunk of data */;
pub const FSCACHE_ADV_WRITE_CACHE: c_uint = 0x00 /* Do cache if written to locally */;
pub const FSCACHE_ADV_WRITE_NOCACHE: c_uint = 0x02 /* Don't cache if written to locally */;
pub const FSCACHE_ADV_WANT_CACHE_SIZE: c_uint = 0x04 /* Retrieve cache size at runtime */;
pub const FSCACHE_INVAL_DIO_WRITE: c_uint = 0x01 /* Invalidate due to DIO write */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_want_state {
    FSCACHE_WANT_PARAMS,
    FSCACHE_WANT_WRITE,
    FSCACHE_WANT_READ,
}

//
// Data object state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_cookie_state {
    FSCACHE_COOKIE_STATE_QUIESCENT,		/* The cookie is uncached */
    FSCACHE_COOKIE_STATE_LOOKING_UP,	/* The cache object is being looked up */
    FSCACHE_COOKIE_STATE_CREATING,		/* The cache object is being created */
    FSCACHE_COOKIE_STATE_ACTIVE,		/* The cache is active, readable and writable */
    FSCACHE_COOKIE_STATE_INVALIDATING,	/* The cache is being invalidated */
    FSCACHE_COOKIE_STATE_FAILED,		/* The cache failed, withdraw to clear */
    FSCACHE_COOKIE_STATE_LRU_DISCARDING,	/* The cookie is being discarded by the LRU */
    FSCACHE_COOKIE_STATE_WITHDRAWING,	/* The cookie is being withdrawn */
    FSCACHE_COOKIE_STATE_RELINQUISHING,	/* The cookie is being relinquished */
    FSCACHE_COOKIE_STATE_DROPPED,		/* The cookie has been dropped */

    } __attribute__((mode(byte)));

//
// Volume representation cookie.
//
    struct fscache_volume {
    refcount_t			ref;
    atomic_t			n_cookies;	/* Number of data cookies in volume */
    atomic_t			n_accesses;	/* Number of cache accesses in progress */
    unsigned int			debug_id;
    unsigned int			key_hash;	/* Hash of key string */
    u8				*key;		/* Volume ID, eg. "afs@example.com@1234" */
    struct list_head		proc_link;	/* Link in /proc/fs/fscache/volumes */
    struct hlist_bl_node		hash_link;	/* Link in hash table */
    struct work_struct		work;
    struct fscache_cache		*cache;		/* The cache in which this resides */
    void				*cache_priv;	/* Cache private data */
    spinlock_t			lock;
    unsigned long			flags;

    u8				coherency_len;	/* Length of the coherency data */
    u8				coherency[];	/* Coherency data */
}

//
// Data file representation cookie.
// - a file will only appear in one cache
// - a request to cache a file may or may not be honoured, subject to
// constraints such as disk space
// - indices are created on disk just-in-time
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscache_cookie {
    pub ref: refcount_t,
    pub /: *mut *mut atomic_t n_active; / number of active users of cookie,
    pub /: *mut *mut atomic_t n_accesses; / Number of cache accesses in progress,
    pub debug_id: c_uint,
    pub /: *mut *mut unsigned int inval_counter; / Number of invalidations made,
    pub lock: spinlock_t,
    pub /: *mut *mut *mut fscache_volume volume; / Parent volume of this file.,
    pub /: *mut *mut *mut void cache_priv; / Cache-side representation,
    pub /: *mut *mut hlist_bl_node hash_link; / Link in hash table,
    pub /: *mut *mut list_head proc_link; / Link in proc list,
    pub /: *mut *mut list_head commit_link; / Link in commit queue,
    pub /: *mut *mut work_work; / Commit/relinq/withdraw work,
    pub /: *mut *mut loff_t object_size; / Size of the netfs object,
    pub /: *mut *mut unsigned long unused_at; / Time at which unused (jiffies),
    pub flags: c_ulong,

    pub state: fscache_cookie_state,
    pub /: *mut *mut *mut u8 advice; / FSCACHE_ADV_,
    pub /: *mut *mut u8 key_len; / Length of index key,
    pub /: *mut *mut u8 aux_len; / Length of auxiliary data,
    pub /: *mut *mut u32 key_hash; / Hash of volume, key, len,
    pub /: *mut *mut *mut void key; / Index key,
    pub /: *mut *mut u8 inline_key[16]; / - If the key is short enough,
}

//
// slow-path functions for when there is actually caching available, and the
// netfs does actually have a valid token
// - these are not to be called directly
// - these are undefined symbols when FS-Cache is not configured and the
// optimiser takes care of not using them
//
extern "C" {
    pub fn __fscache_relinquish_volume(: *mut fscache_volume, : *const c_void, _arg: bool);
}
extern "C" {
    pub fn __fscache_use_cookie(: *mut fscache_cookie, _arg: bool);
}
extern "C" {
    pub fn __fscache_unuse_cookie(: *mut fscache_cookie, : *const c_void, : *const loff_t);
}
extern "C" {
    pub fn __fscache_relinquish_cookie(: *mut fscache_cookie, _arg: bool);
}
extern "C" {
    pub fn __fscache_resize_cookie(: *mut fscache_cookie, _arg: loff_t);
}
extern "C" {
    pub fn __fscache_invalidate(: *mut fscache_cookie, : *const c_void, _arg: loff_t, int: unsigned);
}
extern "C" {
    pub fn __fscache_begin_read_operation(: *mut netfs_cache_resources, : *mut fscache_cookie) -> c_int;
}
extern "C" {
    pub fn __fscache_begin_write_operation(: *mut netfs_cache_resources, : *mut fscache_cookie) -> c_int;
}
extern "C" {
    pub fn __fscache_clear_page_bits(: *mut address_space, _arg: loff_t, _arg: usize);
}
//
// fscache_acquire_volume - Register a volume as desiring caching services
// @volume_key: An identification string for the volume
// @cache_name: The name of the cache to use (or NULL for the default)
// @coherency_data: Piece of arbitrary coherency data to check (or NULL)
// @coherency_len: The size of the coherency data
//
// Register a volume as desiring caching services if they're available.  The
// caller must provide an identifier for the volume and may also indicate which
// cache it should be in.  If a preexisting volume entry is found in the cache,
// the coherency data must match otherwise the entry will be invalidated.
//
// Returns a cookie pointer on success, -ENOMEM if out of memory or -EBUSY if a
// cache volume of that name is already acquired.  Note that "NULL" is a valid
// cookie pointer and can be returned if caching is refused.
//
// fscache_relinquish_volume - Cease caching a volume
// @volume: The volume cookie
// @coherency_data: Piece of arbitrary coherency data to set (or NULL)
// @invalidate: True if the volume should be invalidated
//
// Indicate that a filesystem no longer desires caching services for a volume.
// The caller must have relinquished all file cookies prior to calling this.
// The stored coherency data is updated.
//
// fscache_acquire_cookie - Acquire a cookie to represent a cache object
// @volume: The volume in which to locate/create this cookie
// @advice: Advice flags (FSCACHE_COOKIE_ADV_*)
// @index_key: The index key for this cookie
// @index_key_len: Size of the index key
// @aux_data: The auxiliary data for the cookie (may be NULL)
// @aux_data_len: Size of the auxiliary data buffer
// @object_size: The initial size of object
//
// Acquire a cookie to represent a data file within the given cache volume.
//
// See Documentation/filesystems/caching/netfs-api.rst for a complete
// description.
//
// fscache_use_cookie - Request usage of cookie attached to an object
// @cookie: The cookie representing the cache object
// @will_modify: If cache is expected to be modified locally
//
// Request usage of the cookie attached to an object.  The caller should tell
// the cache if the object's contents are about to be modified locally and then
// the cache can apply the policy that has been set to handle this case.
//
// fscache_unuse_cookie - Cease usage of cookie attached to an object
// @cookie: The cookie representing the cache object
// @aux_data: Updated auxiliary data (or NULL)
// @object_size: Revised size of the object (or NULL)
//
// Cease usage of the cookie attached to an object.  When the users count
// reaches zero then the cookie relinquishment will be permitted to proceed.
//
// fscache_relinquish_cookie - Return the cookie to the cache, maybe discarding
// it
// @cookie: The cookie being returned
// @retire: True if the cache object the cookie represents is to be discarded
//
// This function returns a cookie to the cache, forcibly discarding the
// associated cache object if retire is set to true.
//
// See Documentation/filesystems/caching/netfs-api.rst for a complete
// description.
//
// Find the auxiliary data on a cookie.
//
// Update the auxiliary data on a cookie.
//

//
// fscache_update_cookie - Request that a cache object be updated
// @cookie: The cookie representing the cache object
// @aux_data: The updated auxiliary data for the cookie (may be NULL)
// @object_size: The current size of the object (may be NULL)
//
// Request an update of the index data for the cache object associated with the
// cookie.  The auxiliary data on the cookie will be updated first if @aux_data
// is set and the object size will be updated and the object possibly trimmed
// if @object_size is set.
//
// See Documentation/filesystems/caching/netfs-api.rst for a complete
// description.
//
// fscache_resize_cookie - Request that a cache object be resized
// @cookie: The cookie representing the cache object
// @new_size: The new size of the object (may be NULL)
//
// Request that the size of an object be changed.
//
// See Documentation/filesystems/caching/netfs-api.rst for a complete
// description.
//
// fscache_invalidate - Notify cache that an object needs invalidation
// @cookie: The cookie representing the cache object
// @aux_data: The updated auxiliary data for the cookie (may be NULL)
// @size: The revised size of the object.
// @flags: Invalidation flags (FSCACHE_INVAL_*)
//
// Notify the cache that an object is needs to be invalidated and that it
// should abort any retrievals or stores it is doing on the cache.  This
// increments inval_counter on the cookie which can be used by the caller to
// reconsider I/O requests as they complete.
//
// If @flags has FSCACHE_INVAL_DIO_WRITE set, this indicates that this is due
// to a direct I/O write and will cause caching to be disabled on this cookie
// until it is completely unused.
//
// See Documentation/filesystems/caching/netfs-api.rst for a complete
// description.
//
// fscache_operation_valid - Return true if operations resources are usable
// @cres: The resources to check.
//
// Returns a pointer to the operations table if usable or NULL if not.
//
// fscache_begin_read_operation - Begin a read operation for the netfs lib
// @cres: The cache resources for the read being performed
// @cookie: The cookie representing the cache object
//
// Begin a read operation on behalf of the netfs helper library.  @cres
// indicates the cache resources to which the operation state should be
// attached; @cookie indicates the cache object that will be accessed.
//
// @cres->inval_counter is set from @cookie->inval_counter for comparison at
// the end of the operation.  This allows invalidation during the operation to
// be detected by the caller.
//
// Returns:
// * 0		- Success
// * -ENOBUFS	- No caching available
// * Other error code from the cache, such as -ENOMEM.
//
extern "C" {
    pub fn __fscache_begin_read_operation(_arg: cres, _arg: cookie) -> return;
}
//
// fscache_end_operation - End the read operation for the netfs lib
// @cres: The cache resources for the read operation
//
// Clean up the resources at the end of the read request.
//
// fscache_read - Start a read from the cache.
// @cres: The cache resources to use
// @start_pos: The beginning file offset in the cache file
// @iter: The buffer to fill - and also the length
// @read_hole: How to handle a hole in the data.
// @term_func: The function to call upon completion
// @term_func_priv: The private data for @term_func
//
// Start a read from the cache.  @cres indicates the cache object to read from
// and must be obtained by a call to fscache_begin_operation() beforehand.
//
// The data is read into the iterator, @iter, and that also indicates the size
// of the operation.  @start_pos is the start position in the file, though if
// @seek_data is set appropriately, the cache can use SEEK_DATA to find the
// next piece of data, writing zeros for the hole into the iterator.
//
// Upon termination of the operation, @term_func will be called and supplied
// with @term_func_priv plus the amount of data written, if successful, or the
// error code otherwise.
//
// @read_hole indicates how a partially populated region in the cache should be
// handled.  It can be one of a number of settings:
//
// NETFS_READ_HOLE_IGNORE - Just try to read (may return a short read).
//
// NETFS_READ_HOLE_FAIL - Give ENODATA if we encounter a hole.
//
// fscache_begin_write_operation - Begin a write operation for the netfs lib
// @cres: The cache resources for the write being performed
// @cookie: The cookie representing the cache object
//
// Begin a write operation on behalf of the netfs helper library.  @cres
// indicates the cache resources to which the operation state should be
// attached; @cookie indicates the cache object that will be accessed.
//
// @cres->inval_counter is set from @cookie->inval_counter for comparison at
// the end of the operation.  This allows invalidation during the operation to
// be detected by the caller.
//
// Returns:
// * 0		- Success
// * -ENOBUFS	- No caching available
// * Other error code from the cache, such as -ENOMEM.
//
extern "C" {
    pub fn __fscache_begin_write_operation(_arg: cres, _arg: cookie) -> return;
}
//
// fscache_write - Start a write to the cache.
// @cres: The cache resources to use
// @start_pos: The beginning file offset in the cache file
// @iter: The data to write - and also the length
// @term_func: The function to call upon completion
// @term_func_priv: The private data for @term_func
//
// Start a write to the cache.  @cres indicates the cache object to write to and
// must be obtained by a call to fscache_begin_operation() beforehand.
//
// The data to be written is obtained from the iterator, @iter, and that also
// indicates the size of the operation.  @start_pos is the start position in
// the file.
//
// Upon termination of the operation, @term_func will be called and supplied
// with @term_func_priv plus the amount of data written, if successful, or the
// error code otherwise.
//
// fscache_clear_page_bits - Clear the PG_fscache bits from a set of pages
// @mapping: The netfs inode to use as the source
// @start: The start position in @mapping
// @len: The amount of data to unlock
// @caching: If PG_fscache has been set
//
// Clear the PG_fscache flag from a sequence of pages and wake up anyone who's
// waiting.
//
// fscache_write_to_cache - Save a write to the cache and clear PG_fscache
// @cookie: The cookie representing the cache object
// @mapping: The netfs inode to use as the source
// @start: The start position in @mapping
// @len: The amount of data to write back
// @i_size: The new size of the inode
// @term_func: The function to call upon completion
// @term_func_priv: The private data for @term_func
// @using_pgpriv2: If we're using PG_private_2 to mark in-progress write
// @caching: If we actually want to do the caching
//
// Helper function for a netfs to write dirty data from an inode into the cache
// object that's backing it.
//
// @start and @len describe the range of the data.  This does not need to be
// page-aligned, but to satisfy DIO requirements, the cache may expand it up to
// the page boundaries on either end.  All the pages covering the range must be
// marked with PG_fscache.
//
// If given, @term_func will be called upon completion and supplied with
// @term_func_priv.  Note that if @using_pgpriv2 is set, the PG_private_2 flags
// will have been cleared by this point, so the netfs must retain its own pin
// on the mapping.
//
// fscache_note_page_release - Note that a netfs page got released
// @cookie: The cookie corresponding to the file
//
// Note that a page that has been copied to the cache has been released.  This
// means that future reads will need to look in the cache to see if it's there.
//
// If we've written data to the cache (HAVE_DATA) and there wasn't any
// data in the cache when we started (NO_DATA_TO_READ), it may no
// longer be true that we can skip reading from the cache - so clear
// the flag that causes reads to be skipped.
//
