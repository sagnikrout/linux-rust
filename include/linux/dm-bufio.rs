//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dm-bufio.h
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
// Copyright (C) 2009-2011 Red Hat, Inc.
//
// Author: Mikulas Patocka <mpatocka@redhat.com>
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
//
// Flags for dm_bufio_client_create
//
pub const DM_BUFIO_CLIENT_NO_SLEEP: c_uint = 0x1;
//
// Create a buffered IO cache on a given device
//
// Release a buffered IO cache.
//
extern "C" {
    pub fn dm_bufio_client_destroy(c: *mut dm_bufio_client);
}
extern "C" {
    pub fn dm_bufio_client_reset(c: *mut dm_bufio_client);
}
//
// Set the sector range.
// When this function is called, there must be no I/O in progress on the bufio
// client.
//
extern "C" {
    pub fn dm_bufio_set_sector_offset(c: *mut dm_bufio_client, start: sector_t);
}
//
// WARNING: to avoid deadlocks, these conditions are observed:
//
// - At most one thread can hold at most "reserved_buffers" simultaneously.
// - Each other threads can hold at most one buffer.
// - Threads which call only dm_bufio_get can hold unlimited number of
// buffers.
//
// Read a given block from disk. Returns pointer to data.  Returns a
// pointer to dm_buffer that can be used to release the buffer or to make
// it dirty.
//
// Like dm_bufio_read, but return buffer from cache, don't read
// it. If the buffer is not in the cache, return NULL.
//
// Like dm_bufio_read, but don't read anything from the disk.  It is
// expected that the caller initializes the buffer and marks it dirty.
//
// Prefetch the specified blocks to the cache.
// The function starts to read the blocks and returns without waiting for
// I/O to finish.
//
// Release a reference obtained with dm_bufio_{read,get,new}. The data
// pointer and dm_buffer pointer is no longer valid after this call.
//
extern "C" {
    pub fn dm_bufio_release(b: *mut dm_buffer);
}
//
// Mark a buffer dirty. It should be called after the buffer is modified.
//
// In case of memory pressure, the buffer may be written after
// dm_bufio_mark_buffer_dirty, but before dm_bufio_write_dirty_buffers.  So
// dm_bufio_write_dirty_buffers guarantees that the buffer is on-disk but
// the actual writing may occur earlier.
//
extern "C" {
    pub fn dm_bufio_mark_buffer_dirty(b: *mut dm_buffer);
}
//
// Mark a part of the buffer dirty.
//
// The specified part of the buffer is scheduled to be written. dm-bufio may
// write the specified part of the buffer or it may write a larger superset.
//
// Initiate writing of dirty buffers, without waiting for completion.
//
extern "C" {
    pub fn dm_bufio_write_dirty_buffers_async(c: *mut dm_bufio_client);
}
//
// Write all dirty buffers. Guarantees that all dirty buffers created prior
// to this call are on disk when this call exits.
//
extern "C" {
    pub fn dm_bufio_write_dirty_buffers(c: *mut dm_bufio_client) -> c_int;
}
//
// Send an empty write barrier to the device to flush hardware disk cache.
//
extern "C" {
    pub fn dm_bufio_issue_flush(c: *mut dm_bufio_client) -> c_int;
}
//
// Send a discard request to the underlying device.
//
extern "C" {
    pub fn dm_bufio_issue_discard(c: *mut dm_bufio_client, block: sector_t, count: sector_t) -> c_int;
}
//
// Free the given buffer.
// This is just a hint, if the buffer is in use or dirty, this function
// does nothing.
//
extern "C" {
    pub fn dm_bufio_forget(c: *mut dm_bufio_client, block: sector_t);
}
//
// Free the given range of buffers.
// This is just a hint, if the buffer is in use or dirty, this function
// does nothing.
//
extern "C" {
    pub fn dm_bufio_forget_buffers(c: *mut dm_bufio_client, block: sector_t, n_blocks: sector_t);
}
//
// Set the minimum number of buffers before cleanup happens.
//
extern "C" {
    pub fn dm_bufio_set_minimum_buffers(c: *mut dm_bufio_client, n: c_uint);
}
extern "C" {
    pub fn dm_bufio_get_block_size(c: *mut dm_bufio_client) -> c_uint;
}
extern "C" {
    pub fn dm_bufio_get_device_size(c: *mut dm_bufio_client) -> sector_t;
}
extern "C" {
    pub fn dm_bufio_get_block_number(b: *mut dm_buffer) -> sector_t;
}
// ----------------------------------------------------------------
