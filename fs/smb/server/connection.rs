//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/connection.h
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
//
// Copyright (C) 2018 Samsung Electronics Co., Ltd.
//

pub const KSMBD_SOCKET_BACKLOG: c_int = 16;
//
// Size of the per-connection SMB2 command sequence window. This mirrors
// SMB2_MAX_CREDITS, the maximum number of credits (and therefore the
// maximum number of outstanding sequence numbers) that can be granted on
// a connection. It must be a power of two so the window can be indexed as
// a ring.
//
pub const KSMBD_CMD_SEQ_WINDOW: c_int = 8192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_conn_stats {
    pub open_files_count: core::sync::atomic::AtomicI32,
    pub request_served: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_conn {
    pub vals: *mut smb_version_values,
    pub ops: *mut smb_version_ops,
    pub cmds: *mut smb_version_cmds,
    pub max_cmds: c_uint,
    pub srv_mutex: mutex,
    pub status: c_int,
    pub cli_cap: c_uint,
    pub stop_called: bool,
    pub inet_addr: __be32,
    pub inet6_addr: [u8; 16],
}

// smb session 1 per user
// How many request are running currently
// References which are made for this Server object
//
// Connection command sequence window. [seq_low, seq_high) is the
// range of granted sequence numbers (message IDs). seq_bitmap marks
// the ones in that range that have been granted but
// not yet consumed by a received request.  All three are protected by
// credits_lock.
//
// Lock to protect requests list
// dialect index that server chose
// Preauth Session Table
// Identifier for async message
// Negotiated SMB 3.1.1 compression capabilities.
// Bitmap indexed by SMB2_RDMA_TRANSFORM_* IDs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_conn_ops {
    pub conn): *mut *mut int (process_fn)(struct ksmbd_conn,
    pub conn): *mut *mut int (terminate_fn)(struct ksmbd_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_transport_write {
    pub iov: *mut kvec,
    pub iov_cnt: c_int,
    pub size: c_int,
    pub need_invalidate_rkey: bool,
    pub remote_key: c_uint,
    pub msg_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_transport_ops {
    pub t): *mut *mut void (disconnect)(struct ksmbd_transport,
    pub t): *mut *mut void (shutdown)(struct ksmbd_transport,
    pub max_retries): unsigned int size, int,
    pub tx): *const ksmbd_transport_write,
    pub desc_len): c_uint,
    pub desc_len): c_uint,
    pub kt): *mut *mut void (free_transport)(struct ksmbd_transport,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_transport {
    pub conn: *mut ksmbd_conn,
    pub ops: *const ksmbd_transport_ops,
}

pub const CONN_HASH_BITS: c_int = 12;
extern "C" {
    pub fn DECLARE_HASHTABLE(_arg: conn_list, _arg: CONN_HASH_BITS) -> extern;
}
extern "C" {
    pub fn ksmbd_conn_alive(conn: *mut ksmbd_conn) -> bool;
}
extern "C" {
    pub fn ksmbd_conn_wait_idle(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn ksmbd_conn_free(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn ksmbd_conn_put(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn ksmbd_conn_abort(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn ksmbd_conn_wq_init() -> c_int;
}
extern "C" {
    pub fn ksmbd_conn_wq_destroy();
}
extern "C" {
    pub fn ksmbd_conn_lookup_dialect(c: *mut ksmbd_conn) -> bool;
}
extern "C" {
    pub fn ksmbd_conn_write(work: *mut ksmbd_work) -> c_int;
}
extern "C" {
    pub fn ksmbd_conn_write_eor(work: *mut ksmbd_work) -> c_int;
}
extern "C" {
    pub fn ksmbd_conn_enqueue_request(work: *mut ksmbd_work);
}
extern "C" {
    pub fn ksmbd_conn_try_dequeue_request(work: *mut ksmbd_work);
}
extern "C" {
    pub fn ksmbd_conn_init_server_callbacks(ops: *mut ksmbd_conn_ops);
}
extern "C" {
    pub fn ksmbd_conn_handler_loop(p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ksmbd_conn_transport_init() -> c_int;
}
extern "C" {
    pub fn ksmbd_conn_transport_destroy();
}
extern "C" {
    pub fn ksmbd_conn_lock(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn ksmbd_conn_unlock(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn ksmbd_conn_r_count_inc(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn ksmbd_conn_r_count_dec(conn: *mut ksmbd_conn);
}
//
// WARNING
//
// This is a hack. We will move status to a proper place once we land
// a multi-sessions support.
//
extern "C" {
    pub fn ksmbd_all_conn_set_status(sess: *mut ksmbd_session, status: u32);
}
