//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/index-session.h
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
// The index session mediates all interactions with a UDS index. Once the index session is created,
// it can be used to open, close, suspend, or recreate an index. It implements the majority of the
// functions in the top-level UDS API.
//
// If any deduplication request fails due to an internal error, the index is marked disabled. It
// will not accept any further requests and can only be closed. Closing the index will clear the
// disabled flag, and the index can then be reopened and recovered using the same index session.
//
// Post requests that found an entry
// Post requests found in the open chapter
// Post requests found in the dense index
// Post requests found in the sparse index
// Post requests that did not find an entry
// Update requests that found an entry
// Update requests that did not find an entry
// Delete requests that found an entry
// Delete requests that did not find an entry
// Query requests that found an entry
// Query requests that did not find an entry
// Total number of requests
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum index_suspend_status {
// An index load has started but the index is not ready for use.
    INDEX_OPENING = 0,
// The index is able to handle requests.
    INDEX_READY,
// The index is attempting to suspend a rebuild.
    INDEX_SUSPENDING,
// An index rebuild has been suspended.
    INDEX_SUSPENDED,
// An index rebuild is being stopped in order to shut down.
    INDEX_FREEING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_load_context {
    pub mutex: mutex,
    pub cond: cond_var,
    pub status: index_suspend_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_index_session {
    pub state: c_uint,
    pub index: *mut uds_index,
    pub callback_queue: *mut uds_request_queue,
    pub parameters: uds_parameters,
    pub load_context: index_load_context,
    pub request_mutex: mutex,
    pub request_cond: cond_var,
    pub request_count: c_int,
    pub stats: session_stats,
}
