//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/indexer.h
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
// Copyright 2023 Red Hat
//

//
// UDS public API
//
// The Universal Deduplication System (UDS) is an efficient name-value store. When used for
// deduplicating storage, the names are generally hashes of data blocks and the associated data is
// where that block is located on the underlying storage medium. The stored names are expected to
// be randomly distributed among the space of possible names. If this assumption is violated, the
// UDS index will store fewer names than normal but will otherwise continue to work. The data
// associated with each name can be any 16-byte value.
//
// A client must first create an index session to interact with an index. Once created, the session
// can be shared among multiple threads or users. When a session is destroyed, it will also close
// and save any associated index.
//
// To make a request, a client must allocate a uds_request structure and set the required fields
// before launching it. UDS will invoke the provided callback to complete the request. After the
// callback has been called, the uds_request structure can be freed or reused for a new request.
// There are five types of requests:
//
// A UDS_UPDATE request will associate the provided name with the provided data. Any previous data
// associated with that name will be discarded.
//
// A UDS_QUERY request will return the data associated with the provided name, if any. The entry
// for the name will also be marked as most recent, as if the data had been updated.
//
// A UDS_POST request is a combination of UDS_QUERY and UDS_UPDATE. If there is already data
// associated with the provided name, that data is returned. If there is no existing association,
// the name is associated with the newly provided data. This request is equivalent to a UDS_QUERY
// request followed by a UDS_UPDATE request if no data is found, but it is much more efficient.
//
// A UDS_QUERY_NO_UPDATE request will return the data associated with the provided name, but will
// not change the recency of the entry for the name. This request is primarily useful for testing,
// to determine whether an entry exists without changing the internal state of the index.
//
// A UDS_DELETE request removes any data associated with the provided name. This operation is
// generally not necessary, because the index will automatically discard its oldest entries once it
// becomes full.
//
// General UDS constants and structures
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uds_request_type {
// Create or update the mapping for a name, and make the name most recent.
    UDS_UPDATE,

// Return any mapped data for a name, and make the name most recent.
    UDS_QUERY,

//
// Return any mapped data for a name, or map the provided data to the name if there is no
// current data, and make the name most recent.
//
    UDS_POST,

// Return any mapped data for a name without updating its recency.
    UDS_QUERY_NO_UPDATE,

// Remove any mapping for a name.
    UDS_DELETE,

    } __packed;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uds_open_index_type {
// Create a new index.
    UDS_CREATE,

// Load an existing index and try to recover if necessary.
    UDS_LOAD,

// Load an existing index, but only if it was saved cleanly.
    UDS_NO_REBUILD,
}

// The record name size in bytes
// The maximum record data size in bytes
//
// A type representing a UDS memory configuration which is either a positive integer number of
// gigabytes or one of the six special constants for configurations smaller than one gigabyte.
//
pub type uds_memory_config_size_t = c_int;
// The maximum configurable amount of memory
// Flag indicating that the index has one less chapter than usual
// Special values indicating sizes less than 1 GB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_record_name {
    pub name: [c_uchar; UDS_RECORD_NAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_record_data {
    pub data: [c_uchar; UDS_RECORD_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_volume_record {
    pub name: uds_record_name,
    pub data: uds_record_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_parameters {
// The block_device used for storage
    pub bdev: *mut block_device,
// The maximum allowable size of the index on storage
    pub size: usize,
// The offset where the index should start
    pub offset: off_t,
// The maximum memory allocation, in GB
    pub memory_size: uds_memory_config_size_t,
// Whether the index should include sparse chapters
    pub sparse: bool,
// A 64-bit nonce to validate the index
    pub nonce: u64,
// The number of threads used to process index requests
    pub zone_count: c_uint,
// The number of threads used to read volume pages
    pub read_threads: c_uint,
}

//
// These statistics capture characteristics of the current index, including resource usage and
// requests processed since the index was opened.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_index_stats {
// The total number of records stored in the index
    pub entries_indexed: u64,
// An estimate of the index's memory usage, in bytes
    pub memory_used: u64,
// The number of collisions recorded in the volume index
    pub collisions: u64,
// The number of entries discarded from the index since startup
    pub entries_discarded: u64,
// The time at which these statistics were fetched
    pub current_time: i64,
// The number of post calls that found an existing entry
    pub posts_found: u64,
// The number of post calls that added an entry
    pub posts_not_found: u64,
//
// The number of post calls that found an existing entry that is current enough to only
// exist in memory and not have been committed to disk yet
//
    pub in_memory_posts_found: u64,
//
// The number of post calls that found an existing entry in the dense portion of the index
//
    pub dense_posts_found: u64,
//
// The number of post calls that found an existing entry in the sparse portion of the index
//
    pub sparse_posts_found: u64,
// The number of update calls that updated an existing entry
    pub updates_found: u64,
// The number of update calls that added a new entry
    pub updates_not_found: u64,
// The number of delete requests that deleted an existing entry
    pub deletions_found: u64,
// The number of delete requests that did nothing
    pub deletions_not_found: u64,
// The number of query calls that found existing entry
    pub queries_found: u64,
// The number of query calls that did not find an entry
    pub queries_not_found: u64,
// The total number of requests processed
    pub requests: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uds_index_region {
// No location information has been determined
    UDS_LOCATION_UNKNOWN = 0,
// The index page entry has been found
    UDS_LOCATION_INDEX_PAGE_LOOKUP,
// The record page entry has been found
    UDS_LOCATION_RECORD_PAGE_LOOKUP,
// The record is not in the index
    UDS_LOCATION_UNAVAILABLE,
// The record was found in the open chapter
    UDS_LOCATION_IN_OPEN_CHAPTER,
// The record was found in the dense part of the index
    UDS_LOCATION_IN_DENSE,
// The record was found in the sparse part of the index
    UDS_LOCATION_IN_SPARSE,
    } __packed;

// Zone message requests are used to communicate between index zones.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uds_zone_message_type {
// A standard request with no message
    UDS_MESSAGE_NONE = 0,
// Add a chapter to the sparse chapter index cache
    UDS_MESSAGE_SPARSE_CACHE_BARRIER,
// Close a chapter to keep the zone from falling behind
    UDS_MESSAGE_ANNOUNCE_CHAPTER_CLOSED,
    } __packed;

    struct uds_zone_message {
// The type of message, determining how it will be processed
    enum uds_zone_message_type type;
// The virtual chapter number to which the message applies
    u64 virtual_chapter;
    } __packed;

    struct uds_index_session;
    struct uds_index;
    struct uds_request;

// Once this callback has been invoked, the uds_request structure can be reused or freed.
    typedef void (*uds_request_callback_fn)(struct uds_request *request);

    struct uds_request {
// These input fields must be set before launching a request.

// The name of the record to look up or create
    struct uds_record_name record_name;
// New data to associate with the record name, if applicable
    struct uds_record_data new_metadata;
// A callback to invoke when the request is complete
    uds_request_callback_fn callback;
// The index session that will manage this request
    struct uds_index_session *session;
// The type of operation to perform, as describe above
    enum uds_request_type type;

// These output fields are set when a request is complete.

// The existing data associated with the request name, if any
    struct uds_record_data old_metadata;
// True if the record name had an existing entry in the index
    bool found;
// Either UDS_SUCCESS or an error code for the request
    int status;

// The remaining fields are used internally and should not be altered by clients.
    struct_group(internal,
// The virtual chapter containing the record name, if known
    u64 virtual_chapter;
// The region of the index containing the record name
    enum uds_index_region location;
// If true, process request immediately by waking the worker thread
    bool unbatched;
// If true, continue this request before processing newer requests
    bool requeued;
// Control message for coordinating between zones
    struct uds_zone_message zone_message;
// The number of the zone which will process this request
    unsigned int zone_number;
// A link for adding a request to a lock-free queue
    struct funnel_queue_entry queue_link;
// A link for adding a request to a standard linked list
    struct uds_request *next_request;
// A pointer to the index processing this request
    struct uds_index *index;
    );
}

// Compute the number of bytes needed to store an index.
// A session is required for most index operations.
extern "C" {
    pub fn uds_create_index_session(session: *mut uds_index_session) -> int __must_check;
}
// Destroying an index session also closes and saves the associated index.
extern "C" {
    pub fn uds_destroy_index_session(session: *mut uds_index_session) -> c_int;
}
//
// Create or open an index with an existing session. This operation fails if the index session is
// suspended, or if there is already an open index.
//
// Wait until all callbacks for index operations are complete, and prevent new index operations
// from starting. New index operations will fail with EBUSY until the session is resumed. Also
// optionally saves the index.
//
extern "C" {
    pub fn uds_suspend_index_session(session: *mut uds_index_session, save: bool) -> int __must_check;
}
//
// Allow new index operations for an index, whether it was suspended or not. If the index is
// suspended and the supplied block device differs from the current backing store, the index will
// start using the new backing store instead.
//
// Wait until all outstanding index operations are complete.
extern "C" {
    pub fn uds_flush_index_session(session: *mut uds_index_session) -> int __must_check;
}
// Close an index. This operation fails if the index session is suspended.
extern "C" {
    pub fn uds_close_index(session: *mut uds_index_session) -> int __must_check;
}
// Get index statistics since the last time the index was opened.
// This function will fail if any required field of the request is not set.
extern "C" {
    pub fn uds_launch_request(request: *mut uds_request) -> int __must_check;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond_var {
    pub wait_queue: wait_queue_head_t,
}

extern "C" {
    pub fn uds_wait_cond(cv: *mut cond_var, mutex: *mut mutex);
}
