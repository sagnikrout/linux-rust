//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/relay.h
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
// linux/include/linux/relay.h
//
// Copyright (C) 2002, 2003 - Tom Zanussi (zanussi@us.ibm.com), IBM Corp
// Copyright (C) 1999, 2000, 2001, 2002 - Karim Yaghmour (karim@opersys.com)
//
// CONFIG_RELAY definitions and declarations
//

//
// Tracks changes to rchan/rchan_buf structs
//
pub const RELAYFS_CHANNEL_VERSION: c_int = 7;
//
// Relay buffer statistics
//
// Per-cpu relay channel buffer
//
// Relay channel data structure
//
// Relay channel client callbacks
//
// subbuf_start - called on buffer-switch to a new sub-buffer
// @buf: the channel buffer containing the new sub-buffer
// @subbuf: the start of the new sub-buffer
// @prev_subbuf: the start of the previous sub-buffer
//
// The client should return 1 to continue logging, 0 to stop
// logging.
//
// This callback is optional.
//
// NOTE: subbuf_start will also be invoked when the buffer is
// created, so that the first sub-buffer can be initialized
// if necessary.  In this case, prev_subbuf will be NULL.
//
// NOTE: the client can reserve bytes at the beginning of the new
// sub-buffer by calling subbuf_start_reserve() in this callback.
//
// create_buf_file - create file to represent a relay channel buffer
// @filename: the name of the file to create
// @parent: the parent of the file to create
// @mode: the mode of the file to create
// @buf: the channel buffer
// @is_global: outparam - set non-zero if the buffer should be global
//
// Called during relay_open(), once for each per-cpu buffer,
// to allow the client to create a file to be used to
// represent the corresponding channel buffer.  If the file is
// created outside of relay, the parent must also exist in
// that filesystem.
//
// The callback should return the dentry of the file created
// to represent the relay buffer.
//
// Setting the is_global outparam to a non-zero value will
// cause relay_open() to create a single global buffer rather
// than the default set of per-cpu buffers.
//
// This callback is mandatory.
//
// See Documentation/filesystems/relay.rst for more info.
//
// remove_buf_file - remove file representing a relay channel buffer
// @dentry: the dentry of the file to remove
//
// Called during relay_close(), once for each per-cpu buffer,
// to allow the client to remove a file used to represent a
// channel buffer.
//
// The callback should return 0 if successful, negative if not.
//
// This callback is mandatory.
//
// CONFIG_RELAY kernel API, kernel/relay.c
//
extern "C" {
    pub fn relay_close(chan: *mut rchan);
}
extern "C" {
    pub fn relay_flush(chan: *mut rchan);
}
extern "C" {
    pub fn relay_stats(chan: *mut rchan, flags: c_int) -> usize;
}
extern "C" {
    pub fn relay_reset(chan: *mut rchan);
}
extern "C" {
    pub fn relay_buf_full(buf: *mut rchan_buf) -> c_int;
}
//
// relay_write - write data into the channel
// @chan: relay channel
// @data: data to be written
// @length: number of bytes to write
//
// Writes data into the current cpu's channel buffer.
//
// Protects the buffer by disabling interrupts.  Use this
// if you might be logging from interrupt context.  Try
// __relay_write() if you know you	won't be logging from
// interrupt context.
//
// __relay_write - write data into the channel
// @chan: relay channel
// @data: data to be written
// @length: number of bytes to write
//
// Writes data into the current cpu's channel buffer.
//
// Protects the buffer by disabling preemption.  Use
// relay_write() if you might be logging from interrupt
// context.
//
// relay_reserve - reserve slot in channel buffer
// @chan: relay channel
// @length: number of bytes to reserve
//
// Returns pointer to reserved slot, NULL if full.
//
// Reserves a slot in the current cpu's channel buffer.
// Does not protect the buffer at all - caller must provide
// appropriate synchronization.
//
// subbuf_start_reserve - reserve bytes at the start of a sub-buffer
// @buf: relay channel buffer
// @length: number of bytes to reserve
//
// Helper function used to reserve bytes at the beginning of
// a sub-buffer in the subbuf_start() callback.
//
// exported relay file operations, kernel/relay.c
//

extern "C" {
    pub fn relay_prepare_cpu(cpu: c_uint) -> c_int;
}

