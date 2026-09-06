//! Automatically rewritten from C Header to Rust Module
//! Source: fs/fuse/dev_uring_i.h
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
// FUSE: Filesystem in Userspace
// Copyright (c) 2023-2024 DataDirect Networks.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_ring_req_state {
    FRRS_INVALID = 0,

// The ring entry received from userspace and it is being processed
    FRRS_COMMIT,

// The ring entry is waiting for new fuse requests
    FRRS_AVAILABLE,

// The ring entry got assigned a fuse req
    FRRS_FUSE_REQ,

// The ring entry is in or on the way to user space
    FRRS_USERSPACE,

// The ring entry is in teardown
    FRRS_TEARDOWN,

// The ring entry is released, but not freed yet
    FRRS_RELEASED,
}

// how a queue's payload buffers are provided
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_queue_payload_mode {
// not yet committed (a bufpool may still be added)
    FUSE_PAYLOAD_UNSET = 0,
// each entry registers its own payload buffer
    FUSE_PAYLOAD_PER_ENT,
// each entry's payload buffer is assigned from a bufpool
    FUSE_PAYLOAD_BUFPOOL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_bufpool {
    pub registered: bool,
//
// io_uring registered buffer table index for this pool, bound at
// ADD_BUFPOOL time. Only valid if the bufpool is registered
//
    pub registered_index: u16,
// starting uaddr of the bufpool
    pub base_uaddr: uintptr_t,
// size of each buffer in the pool
    pub buf_size: usize,
// total number of buffers in the pool
    pub nr_bufs: c_uint,
// bitmap tracking which buffers are free
    pub free_map: [c_ulong; ],
}

// A fuse ring entry, part of the ring queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_ring_ent {
// userspace buffer
    pub headers: *mut fuse_uring_req_header __user,
    pub payload: iovec,
// buffer id in the pool, if bufpools are used. ignored otherwise
    pub buf_id: c_uint,
// true if the request's pages are being zero-copied
    pub zero_copied: bool,
    pub zero_copy_index: c_uint,
// the ring queue that owns the request
    pub queue: *mut fuse_ring_queue,
// fields below are protected by queue->lock
    pub cmd: *mut io_uring_cmd,
    pub list: list_head,
    pub state: fuse_ring_req_state,
    pub fuse_req: *mut fuse_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_ring_queue {
//
// back pointer to the main fuse uring structure that holds this
// queue
//
    pub ring: *mut fuse_ring,
// queue id, corresponds to the cpu core
    pub qid: c_uint,
//
// queue lock, taken when any value in the queue changes _and_ also
// a ring entry state changes.
//
    pub lock: spinlock_t,
// available ring entries (struct fuse_ring_ent)
    pub ent_avail_queue: list_head,
//
// entries in the process of being committed or in the process
// to be sent to userspace
//
    pub ent_w_req_queue: list_head,
    pub ent_commit_queue: list_head,
// entries in userspace
    pub ent_in_userspace: list_head,
// entries that are released
    pub ent_released: list_head,
// fuse requests waiting for an entry slot
    pub fuse_req_queue: list_head,
// background fuse requests
    pub fuse_req_bg_queue: list_head,
    pub fpq: fuse_pqueue,
    pub active_background: c_uint,
    pub stopped: bool,
// how this queue's payload buffers are provided
    pub payload_mode: fuse_queue_payload_mode,
// only allocated when payload_mode == FUSE_PAYLOAD_BUFPOOL
    pub bufpool: *mut fuse_bufpool,
    pub zero_copy: bool,
}

//
// Describes if uring is for communication and holds alls the data needed
// for uring communication
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_ring {
// back pointer
    pub chan: *mut fuse_chan,
// number of ring queues
    pub nr_queues: usize,
// maximum payload/arg size
    pub max_payload_sz: usize,
    pub queues: *mut fuse_ring_queue,
//
// Log ring entry states on stop when entries cannot be released
//
    pub 1: unsigned int stop_debug_log :,
    pub stop_waitq: wait_queue_head_t,
// async tear down
    pub async_teardown_work: delayed_work,
// log
    pub teardown_time: c_ulong,
    pub queue_refs: core::sync::atomic::AtomicI32,
    pub ready: bool,
}

extern "C" {
    pub fn fuse_uring_conn_init(fch: *mut fuse_chan);
}
extern "C" {
    pub fn fuse_uring_stop_queues(ring: *mut fuse_ring);
}
extern "C" {
    pub fn fuse_uring_abort_end_requests(ring: *mut fuse_ring);
}
extern "C" {
    pub fn fuse_uring_cmd(cmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn fuse_uring_queue_fuse_req(fiq: *mut fuse_iqueue, req: *mut fuse_req);
}
extern "C" {
    pub fn fuse_uring_queue_bq_req(req: *mut fuse_req) -> bool;
}
extern "C" {
    pub fn fuse_uring_remove_pending_req(req: *mut fuse_req) -> bool;
}
extern "C" {
    pub fn fuse_uring_request_expired(fch: *mut fuse_chan) -> bool;
}

