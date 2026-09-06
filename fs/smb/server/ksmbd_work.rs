//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/ksmbd_work.h
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
// Copyright (C) 2019 Samsung Electronics Co., Ltd.
//

pub const KSMBD_WORK_INLINE_IOVS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_read {
    pub buf: *mut c_void,
    pub entry: list_head,
}

// one of these for every pending CIFS request at the connection
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_work {
// Server corresponding to this mid
    pub conn: *mut ksmbd_conn,
    pub sess: *mut ksmbd_session,
    pub tcon: *mut ksmbd_tree_connect,
// Pointer to received SMB header
    pub request_buf: *mut c_void,
// Response buffer
    pub response_buf: *mut c_void,
    pub aux_read_list: list_head,
    pub iov: *mut kvec,
    pub iov_alloc_cnt: c_int,
    pub iov_cnt: c_int,
    pub iov_idx: c_int,
    pub iov_inline: [kvec; KSMBD_WORK_INLINE_IOVS],
// Next cmd hdr in compound req buf
    pub next_smb2_rcv_hdr_off: c_int,
// Next cmd hdr in compound rsp buf
    pub next_smb2_rsp_hdr_off: c_int,
// Current cmd hdr in compound rsp buf
    pub curr_smb2_rsp_hdr_off: c_int,
//
// Current Local FID assigned compound response if SMB2 CREATE
// command is present in compound request
//
    pub compound_fid: u64,
    pub compound_pfid: u64,
    pub compound_sid: u64,
    pub compound_status: __le32,
    pub saved_cred: *const cred,
// Number of granted credits
    pub credits_granted: c_uint,
//
// Credit charge added to conn->outstanding_credits at receive time
// for the SMB2 PDU currently being processed, pending release.  Zero
// once the charge has been returned (on the response or error path).
//
    pub credit_charge: c_ushort,
// response smb header size
    pub response_sz: c_uint,
    pub tr_buf: *mut c_void,
// Contiguous SMB2 compression transform owned by this work item.
    pub compress_buf: *mut c_void,
    pub state: c_uint,
// No response for cancelled request
    pub send_no_response:1: bool,
// Request is encrypted
    pub encrypted:1: bool,
// READ response should be wrapped in a compression transform.
    pub compress_response:1: bool,
// Is this SYNC or ASYNC ksmbd_work
    pub asynchronous:1: bool,
// Work owns a reference to @conn.
    pub owns_conn_ref:1: bool,
    pub need_invalidate_rkey:1: bool,
    pub request_open_chseq_tracked:1: bool,
    pub session_setup_reauth:1: bool,
    pub remote_key: c_uint,
// cancel works
    pub async_id: c_int,
    pub cancel_argv: *mut c_void,
    pub argv): *mut *mut void (cancel_fn)(void,
//
// Refcounted open associated with the SMB2 command currently being
// processed.
//
    pub request_open: *mut ksmbd_file,
    pub request_open_chseq: __le16,
    pub work: work_struct,
// List head at conn->requests
    pub request_entry: list_head,
// List head at conn->async_requests
    pub async_request_entry: list_head,
    pub fp_entry: list_head,
// List head at ksmbd_file->notify_pendings
    pub notify_entry: list_head,
}

//
// ksmbd_resp_buf_next - Get next buffer on compound response.
// @work: smb work containing response buffer
//
// ksmbd_resp_buf_curr - Get current buffer on compound response.
// @work: smb work containing response buffer
//
// ksmbd_req_buf_next - Get next buffer on compound request.
// @work: smb work containing response buffer
//
extern "C" {
    pub fn ksmbd_free_work_struct(work: *mut ksmbd_work);
}
extern "C" {
    pub fn ksmbd_work_pool_destroy();
}
extern "C" {
    pub fn ksmbd_work_pool_init() -> c_int;
}
extern "C" {
    pub fn ksmbd_workqueue_init() -> c_int;
}
extern "C" {
    pub fn ksmbd_workqueue_destroy();
}
extern "C" {
    pub fn ksmbd_queue_work(work: *mut ksmbd_work) -> bool;
}
extern "C" {
    pub fn ksmbd_iov_pin_rsp(work: *mut ksmbd_work, ib: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn allocate_interim_rsp_buf(work: *mut ksmbd_work) -> c_int;
}
