//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/glock.h
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
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
//

// Options for hostdata parser
//
// lm_lockname types
//
pub const LM_TYPE_RESERVED: c_uint = 0x00;
pub const LM_TYPE_NONDISK: c_uint = 0x01;
pub const LM_TYPE_INODE: c_uint = 0x02;
pub const LM_TYPE_RGRP: c_uint = 0x03;
pub const LM_TYPE_META: c_uint = 0x04;
pub const LM_TYPE_IOPEN: c_uint = 0x05;
pub const LM_TYPE_FLOCK: c_uint = 0x06;
pub const LM_TYPE_PLOCK: c_uint = 0x07;
pub const LM_TYPE_QUOTA: c_uint = 0x08;
pub const LM_TYPE_JOURNAL: c_uint = 0x09;
//
// lm_lock() states
//
// SHARED is compatible with SHARED, not with DEFERRED or EX.
// DEFERRED is compatible with DEFERRED, not with SHARED or EX.
//
pub const LM_ST_UNLOCKED: c_int = 0;
pub const LM_ST_EXCLUSIVE: c_int = 1;
pub const LM_ST_DEFERRED: c_int = 2;
pub const LM_ST_SHARED: c_int = 3;
//
// lm_lock() flags
//
// LM_FLAG_TRY
// Don't wait to acquire the lock if it can't be granted immediately.
//
// LM_FLAG_TRY_1CB
// Send one blocking callback if TRY is set and the lock is not granted.
//
// LM_FLAG_RECOVER
// GFS sets this flag on lock requests it makes while doing journal recovery.
// While ordinary requests are blocked until the end of recovery, requests
// with this flag set do proceed.
//
// LM_FLAG_ANY
// A SHARED request may also be granted in DEFERRED, or a DEFERRED request may
// also be granted in SHARED.  The preferred state is whichever is compatible
// with other granted locks, or the specified state if no other locks exist.
//
// In addition, when a lock is already held in EX mode locally, a SHARED or
// DEFERRED mode request with the LM_FLAG_ANY flag set will be granted.
// (The LM_FLAG_ANY flag is only use for SHARED mode requests currently.)
//
// LM_FLAG_NODE_SCOPE
// This holder agrees to share the lock within this node. In other words,
// the glock is held in EX mode according to DLM, but local holders on the
// same node can share it.
//
pub const LM_FLAG_TRY: c_uint = 0x0001;
pub const LM_FLAG_TRY_1CB: c_uint = 0x0002;
pub const LM_FLAG_RECOVER: c_uint = 0x0004;
pub const LM_FLAG_ANY: c_uint = 0x0008;
pub const LM_FLAG_NODE_SCOPE: c_uint = 0x0020;
pub const GL_ASYNC: c_uint = 0x0040;
pub const GL_EXACT: c_uint = 0x0080;
pub const GL_SKIP: c_uint = 0x0100;
pub const GL_NOPID: c_uint = 0x0200;
pub const GL_NOCACHE: c_uint = 0x0400;
pub const GL_NOBLOCK: c_uint = 0x0800;
//
// lm_async_cb return flags
//
// LM_OUT_ST_MASK
// Masks the lower two bits of lock state in the returned value.
//
// LM_OUT_TRY_AGAIN
// The trylock request failed.
//
// LM_OUT_DEADLOCK
// The lock request failed because it would deadlock.
//
// LM_OUT_CANCELED
// The lock request was canceled.
//
// LM_OUT_ERROR
// The lock request timed out or failed.
//
pub const LM_OUT_ST_MASK: c_uint = 0x00000003;
pub const LM_OUT_TRY_AGAIN: c_uint = 0x00000020;
pub const LM_OUT_DEADLOCK: c_uint = 0x00000010;
pub const LM_OUT_CANCELED: c_uint = 0x00000008;
pub const LM_OUT_ERROR: c_uint = 0x00000004;
//
// lm_recovery_done() messages
//
pub const LM_RD_GAVEUP: c_int = 308;
pub const LM_RD_SUCCESS: c_int = 309;
pub const GLR_TRYFAILED: c_int = 13;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm_lockops {
    pub lm_proto_name: *const c_char,
    pub table): *const *const *const int (lm_mount) (struct gfs2_sbd sdp, char,
    pub sdp): *mut *mut void (lm_first_done) (struct gfs2_sbd,
    pub result): c_uint,
    pub clean): *mut *mut *mut void (lm_unmount) (struct gfs2_sbd sdp, bool,
    pub sdp): *mut *mut void (lm_withdraw) (struct gfs2_sbd,
    pub gl): *mut *mut void (lm_put_lock) (struct gfs2_glock,
    pub flags): c_uint,
    pub gl): *mut *mut void (lm_cancel) (struct gfs2_glock,
    pub lm_tokens: *const match_table_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_glock_aspace {
    pub glock: gfs2_glock,
    pub mapping: address_space,
}

// Look in glock's list of holders for one with current task as owner
extern "C" {
    pub fn gfs2_glock_put(gl: *mut gfs2_glock);
}
extern "C" {
    pub fn gfs2_glock_put_async(gl: *mut gfs2_glock);
}
extern "C" {
    pub fn gfs2_holder_uninit(gh: *mut gfs2_holder);
}
extern "C" {
    pub fn gfs2_glock_nq(gh: *mut gfs2_holder) -> c_int;
}
extern "C" {
    pub fn gfs2_glock_poll(gh: *mut gfs2_holder) -> c_int;
}
extern "C" {
    pub fn gfs2_instantiate(gh: *mut gfs2_holder) -> c_int;
}
extern "C" {
    pub fn gfs2_glock_holder_ready(gh: *mut gfs2_holder) -> c_int;
}
extern "C" {
    pub fn gfs2_glock_wait(gh: *mut gfs2_holder) -> c_int;
}
extern "C" {
    pub fn gfs2_glock_dq(gh: *mut gfs2_holder);
}
extern "C" {
    pub fn gfs2_glock_dq_wait(gh: *mut gfs2_holder);
}
extern "C" {
    pub fn gfs2_glock_dq_uninit(gh: *mut gfs2_holder);
}
extern "C" {
    pub fn gfs2_glock_nq_m(num_gh: c_uint, ghs: *mut gfs2_holder) -> c_int;
}
extern "C" {
    pub fn gfs2_glock_dq_m(num_gh: c_uint, ghs: *mut gfs2_holder);
}

extern "C" {
    pub fn gfs2_print_dbg(seq: *mut seq_file, fmt: *const c_char, ...);
}
//
// gfs2_glock_nq_init - initialize a holder and enqueue it on a glock
// @gl: the glock
// @state: the state we're requesting
// @flags: the modifier flags
// @gh: the holder structure
//
// Returns: 0, GLR_*, or errno
//
extern "C" {
    pub fn gfs2_glock_cb(gl: *mut gfs2_glock, state: c_uint);
}
extern "C" {
    pub fn gfs2_glock_complete(gl: *mut gfs2_glock, ret: c_int);
}
extern "C" {
    pub fn gfs2_queue_try_to_evict(gl: *mut gfs2_glock) -> bool;
}
extern "C" {
    pub fn gfs2_queue_verify_delete(gl: *mut gfs2_glock, later: bool) -> bool;
}
extern "C" {
    pub fn gfs2_cancel_delete_work(gl: *mut gfs2_glock);
}
extern "C" {
    pub fn gfs2_flush_delete_work(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_wait_glocks(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_withdraw_glocks(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_glock_thaw(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_glock_free(gl: *mut gfs2_glock);
}
extern "C" {
    pub fn gfs2_glock_free_later(gl: *mut gfs2_glock);
}
extern "C" {
    pub fn gfs2_glock_init() -> int __init;
}
extern "C" {
    pub fn gfs2_glock_exit();
}
extern "C" {
    pub fn gfs2_create_debugfs_file(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_delete_debugfs_file(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_register_debugfs();
}
extern "C" {
    pub fn gfs2_unregister_debugfs();
}
extern "C" {
    pub fn glock_set_object(gl: *mut gfs2_glock, object: *mut c_void);
}
extern "C" {
    pub fn glock_clear_object(gl: *mut gfs2_glock, object: *mut c_void);
}
extern "C" {
    pub fn gfs2_inode_remember_delete(gl: *mut gfs2_glock, generation: u64);
}
extern "C" {
    pub fn gfs2_inode_already_deleted(gl: *mut gfs2_glock, generation: u64) -> bool;
}
