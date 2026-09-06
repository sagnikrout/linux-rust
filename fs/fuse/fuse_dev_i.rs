//! Automatically rewritten from C Header to Rust Module
//! Source: fs/fuse/fuse_dev_i.h
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
// Copyright (C) 2001-2008  Miklos Szeredi <miklos@szeredi.hu>
//

// Ordinary requests have even IDs, while interrupts IDs are odd

//
// enum fuse_req_flag - Request flags
//
// @FR_ISREPLY:		set if the request has reply
// @FR_FORCE:		force sending of the request even if interrupted
// @FR_BACKGROUND:	request is sent in the background
// @FR_WAITING:		request is counted as "waiting"
// @FR_ABORTED:		the request was aborted
// @FR_INTERRUPTED:	the request has been interrupted
// @FR_LOCKED:		data is being copied to/from the request
// @FR_PENDING:		request is not yet in userspace
// @FR_SENT:		request is in userspace, waiting for an answer
// @FR_FINISHED:	request is finished
// @FR_PRIVATE:		request is on private list
// @FR_ASYNC:		request is asynchronous
// @FR_URING:		request is handled through fuse-io-uring
// @FR_SYNC_WAKEUP:	use synchronous wakeup when queueing this request to
// give the scheduler a hint about the waker task
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_req_flag {
    FR_ISREPLY,
    FR_FORCE,
    FR_BACKGROUND,
    FR_WAITING,
    FR_ABORTED,
    FR_INTERRUPTED,
    FR_LOCKED,
    FR_PENDING,
    FR_SENT,
    FR_FINISHED,
    FR_PRIVATE,
    FR_ASYNC,
    FR_URING,
    FR_SYNC_WAKEUP,
}

//
// struct fuse_req - A request to the client
//
// .waitq.lock protects the following fields:
// - FR_ABORTED
// - FR_LOCKED (may also be modified under fpq->lock, tested under both)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_req {
//
// @list: This can be on either pending processing or io lists in
// fuse_conn
//
    pub list: list_head,
// @intr_entry: Entry on the interrupts list
    pub intr_entry: list_head,
// @args: Input/output arguments
    pub args: *mut fuse_args,
// @count: refcount
    pub count: refcount_t,
// @flags: Request flags, updated with test/set/clear_bit()
    pub flags: c_ulong,
// @in: The request input header
// @in.h: The request input header
    pub h: fuse_in_header,
    pub in: },
// @out: The request output header
// @out.h: The request output header
    pub h: fuse_out_header,
    pub out: },
// @waitq: Used to wake up the task waiting for completion of request
    pub waitq: wait_queue_head_t,

//
// @argbuf: virtio-fs's physically contiguous buffer for in and out
// args
//
    pub argbuf: *mut c_void,

// @chan: fuse_chan this request belongs to
    pub chan: *mut fuse_chan,

    pub ring_entry: *mut c_void,
    pub ring_queue: *mut c_void,

// @create_time: When (in jiffies) the request was created
    pub create_time: c_ulong,
}

// One forget request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_forget_link {
    pub forget_one: fuse_forget_one,
    pub next: *mut fuse_forget_link,
}

//
// struct fuse_iqueue_ops - Input queue callbacks
//
// Input queue signalling is device-specific.  For example, the /dev/fuse file
// uses fiq->waitq and fasync to wake processes that are waiting on queue
// readiness.  These callbacks allow other device types to respond to input
// queue activity.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_iqueue_ops {
//
// @send_forget: Send one forget
//
    pub link): *mut *mut *mut void (send_forget)(struct fuse_iqueue fiq, struct fuse_forget_link,
//
// @send_interrupt: Send interrupt for request
//
    pub req): *mut *mut *mut void (send_interrupt)(struct fuse_iqueue fiq, struct fuse_req,
//
// @send_req: Send one request
//
    pub req): *mut *mut *mut void (send_req)(struct fuse_iqueue fiq, struct fuse_req,
//
// @release: Clean up when fuse_iqueue is destroyed
//
    pub fiq): *mut *mut void (release)(struct fuse_iqueue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_iqueue {
// Connection established
    pub connected: unsigned,
// Lock protecting accesses to members of this structure
    pub lock: spinlock_t,
// Readers of the connection are waiting on this
    pub waitq: wait_queue_head_t,
// The next unique request id
    pub reqctr: u64,
// The list of pending requests
    pub pending: list_head,
// Pending interrupts
    pub interrupts: list_head,
// Queue of pending forgets
    pub forget_list_head: fuse_forget_link,
    pub forget_list_tail: *mut fuse_forget_link,
// Batching of FORGET requests (positive indicates FORGET batch)
    pub forget_batch: c_int,
// O_ASYNC requests
    pub fasync: *mut fasync_struct,
// Device-specific callbacks
    pub ops: *const fuse_iqueue_ops,
// Device-specific state
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_chan {
// Lock protecting:
//
    pub lock: spinlock_t,
// back pointer: fc->chan->conn == fc
    pub conn: *mut fuse_conn,
// Input queue
    pub iq: fuse_iqueue,
// List of device instances belonging to this connection
    pub devices: list_head,
// Maximum number of outstanding background requests
    pub max_background: unsigned,
// Number of requests currently in the background
    pub num_background: unsigned,
// Number of background requests currently queued for userspace
    pub active_background: unsigned,
// The list of background requests set aside for later queuing
    pub bg_queue: list_head,
// Protects: max_background, num_background, active_background, bg_queue, blocked
    pub bg_lock: spinlock_t,
// Flag indicating that INIT reply has been received. Allocating
// any fuse request will be suspended until the flag is set
    pub initialized: c_int,
// Flag indicating if connection is blocked.  This will be
    pub blocked: c_int,
// waitq for blocked connection
    pub blocked_waitq: wait_queue_head_t,
// Connection established, cleared on umount, connection
    pub connected: unsigned,
// The number of requests waiting for completion
    pub num_waiting: core::sync::atomic::AtomicI32,
// Is interrupt not implemented by fs?
    pub no_interrupt: bool,
// Use io_uring for communication
    pub io_uring: c_uint,
// Negotiated minor version
    pub minor: c_uint,
// Maximum write size
    pub max_write: c_uint,
// Maximum number of pages that can be used in a single request
    pub max_pages: c_uint,
// Before being installed into fud, contains the preallocated pq array
    pub pq_prealloc: *mut list_head,
// Connection aborted via sysfs, respond with ECONNABORTED on device I/O
    pub abort_with_err: bool,

// uring connection information
    pub ring: *mut fuse_ring,

// Only used if the connection opts into request timeouts
// Worker for checking if any requests have timed out
    pub work: delayed_work,
// Request timeout (in jiffies). 0 = no timeout
    pub req_timeout: c_uint,
    pub timeout: },
}

pub const FUSE_PQ_HASH_BITS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_pqueue {
// Connection established
    pub connected: unsigned,
// Lock protecting accessess to  members of this structure
    pub lock: spinlock_t,
// Hash table of requests being processed
    pub processing: *mut list_head,
// The list of requests under I/O
    pub io: list_head,
}

//
// struct fuse_dev - Fuse device instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_dev {
// @ref: Reference count of this object
    pub ref: refcount_t,
// @sync_init: Issue FUSE_INIT synchronously
    pub sync_init: bool,
// @chan: Fuse channel for this device
    pub chan: *mut fuse_chan,
// @pq: Processing queue
    pub pq: fuse_pqueue,
// @entry: list entry on fch->devices
    pub entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_copy_state {
    pub req: *mut fuse_req,
    pub iter: *mut iov_iter,
    pub pipebufs: *mut pipe_buffer,
    pub currbuf: *mut pipe_buffer,
    pub pipe: *mut pipe_inode_info,
    pub nr_segs: c_ulong,
    pub pg: *mut page,
    pub len: c_uint,
    pub offset: c_uint,
    pub write:1: bool,
    pub move_folios:1: bool,
    pub is_uring:1: bool,
// set when the payload is zero-copied. folios are filled in place
    pub skip_folio_copy:1: bool,
    pub /: *mut *mut unsigned int copied_sz; / copied size into the user buffer,
    pub ring: },
}

// fud->chan gets assigned to this value when /dev/fuse is closed

//
// Lockless access is OK, because fud->chan is set once during mount and is valid
// until the file is released.
//
// fud->chan is set to FUSE_DEV_CHAN_DISCONNECTED only after the containing file is
// released, so result is safe to dereference in most cases.  Exceptions are:
// fuse_dev_put() and fuse_fill_super_common().
//
// Pairs with xchg() in fuse_dev_install()
extern "C" {
    pub fn smp_load_acquire(_arg: &fud->chan) -> return;
}
extern "C" {
    pub fn fuse_iqueue_init(fiq: *mut fuse_iqueue, ops: *const fuse_iqueue_ops, priv: *mut c_void);
}
extern "C" {
    pub fn fuse_req_hash(unique: u64) -> c_uint;
}
extern "C" {
    pub fn fuse_dev_end_requests(head: *mut list_head);
}
extern "C" {
    pub fn fuse_request_bg_finish(fch: *mut fuse_chan, req: *mut fuse_req);
}
//
// Return the number of bytes in an arguments list
//
extern "C" {
    pub fn fuse_len_args(numargs: c_uint, args: *mut fuse_arg) -> c_uint;
}
extern "C" {
    pub fn fuse_dev_queue_interrupt(fiq: *mut fuse_iqueue, req: *mut fuse_req);
}
extern "C" {
    pub fn fuse_remove_pending_req(req: *mut fuse_req, lock: *mut spinlock_t) -> bool;
}
extern "C" {
    pub fn fuse_request_expired(fch: *mut fuse_chan, list: *mut list_head) -> bool;
}
//
// Assign a unique id to a fuse request
//
extern "C" {
    pub fn fuse_request_assign_unique(fiq: *mut fuse_iqueue, req: *mut fuse_req);
}
//
// Get the next unique ID for a request
//
extern "C" {
    pub fn fuse_get_unique(fiq: *mut fuse_iqueue) -> u64;
}
extern "C" {
    pub fn fuse_dev_release(inode: *mut inode, file: *mut file) -> c_int;
}
//
// Initialize the fuse processing queue
//
extern "C" {
    pub fn fuse_pqueue_init(fpq: *mut fuse_pqueue);
}
//
// End a finished request
//
extern "C" {
    pub fn fuse_request_end(req: *mut fuse_req);
}
