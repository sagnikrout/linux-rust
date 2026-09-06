//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/scsi/zfcp_reqlist.h
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
// zfcp device driver
//
// Data structure and helper functions for tracking pending FSF
// requests.
//
// Copyright IBM Corp. 2009, 2023
//

// number of hash buckets

//
// struct zfcp_reqlist - Container for request list (reqlist)
// @lock: Spinlock for protecting the hash list
// @buckets: Array of hashbuckets, each is a list of requests in this bucket
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_reqlist {
    pub lock: spinlock_t,
    pub buckets: [list_head; ZFCP_REQ_LIST_BUCKETS],
}

//
// zfcp_reqlist_alloc - Allocate and initialize reqlist
//
// Returns pointer to allocated reqlist on success, or NULL on
// allocation failure.
//
// zfcp_reqlist_isempty - Check whether the request list empty
// @rl: pointer to reqlist
//
// Returns: 1 if list is empty, 0 if not
//
// zfcp_reqlist_free - Free allocated memory for reqlist
// @rl: The reqlist where to free memory
//
// sanity check
//
// zfcp_reqlist_find - Lookup FSF request by its request id
// @rl: The reqlist where to lookup the FSF request
// @req_id: The request id to look for
//
// Returns a pointer to the FSF request with the specified request id
// or NULL if there is no known FSF request with this id.
//
// zfcp_reqlist_find_rm - Lookup request by id and remove it from reqlist
// @rl: reqlist where to search and remove entry
// @req_id: The request id of the request to look for
//
// This functions tries to find the FSF request with the specified
// id and then removes it from the reqlist. The reqlist lock is held
// during both steps of the operation.
//
// Returns: Pointer to the FSF request if the request has been found,
// NULL if it has not been found.
//
// zfcp_reqlist_add - Add entry to reqlist
// @rl: reqlist where to add the entry
// @req: The entry to add
//
// The request id always increases. As an optimization new requests
// are added here with list_add_tail at the end of the bucket lists
// while old requests are looked up starting at the beginning of the
// lists.
//
// zfcp_reqlist_move - Move all entries from reqlist to simple list
// @rl: The zfcp_reqlist where to remove all entries
// @list: The list where to move all entries
//
// zfcp_reqlist_apply_for_all() - apply a function to every request.
// @rl: the requestlist that contains the target requests.
// @f: the function to apply to each request; the first parameter of the
// function will be the target-request; the second parameter is the same
// pointer as given with the argument @data.
// @data: freely chosen argument; passed through to @f as second parameter.
//
// Uses :c:macro:`list_for_each_entry` to iterate over the lists in the hash-
// table (not a 'safe' variant, so don't modify the list).
//
// Holds @rl->lock over the entire request-iteration.
//
