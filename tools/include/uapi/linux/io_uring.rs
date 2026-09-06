//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/io_uring.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT
//
// Header file for the io_uring interface.
//
// Copyright (C) 2019 Jens Axboe
// Copyright (C) 2019 Christoph Hellwig
//

//
// this file is shared with liburing and that has to autodetect
// if linux/time_types.h is available or not, it can
// define UAPI_LINUX_IO_URING_H_SKIP_LINUX_TIME_TYPES_H
// if linux/time_types.h is not available
//

//
// IO submission data structure (Submission Queue Entry)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sqe {
    pub /: *mut *mut __u8 opcode; / type of operation for this sqe,
    pub /: *mut *mut __u8 flags; / IOSQE_ flags,
    pub /: *mut *mut __u16 ioprio; / ioprio for the request,
    pub /: *mut *mut __s32 fd; / file descriptor to do IO on,
    pub /: *mut *mut __u64 off; / offset into file,
    pub addr2: __u64,
    pub cmd_op: __u32,
    pub __pad1: __u32,
}

// pack this to avoid bogus arm OABI complaints
// index into fixed buffers, if used
// for grouped buffer selection
// personality to use, if used
//
// If the ring is initialized with IORING_SETUP_SQE128, then
// this field is used for 80 bytes of arbitrary command data
//
// If sqe->file_index is set to this for opcodes that instantiate a new
// direct descriptor (like openat/openat2/accept), then io_uring will allocate
// an available direct descriptor instead of having the application pass one
// in. The picked direct descriptor will be returned in cqe->res, or -ENFILE
// if the space is full.
//

//
// sqe->flags
//
// use fixed fileset

// issue after inflight IO

// links next sqe

// like LINK, but stronger

// always go async

// select buffer from sqe->buf_group

// don't post CQE if request succeeded

//
// io_uring_setup() flags
//

//
// Cooperative task running. When requests complete, they often require
// forcing the submitter to transition to the kernel to complete. If this
// flag is set, work will be done when the task transitions anyway, rather
// than force an inter-processor interrupt reschedule. This avoids interrupting
// a task running in userspace, and saves an IPI.
//

//
// If COOP_TASKRUN is set, get notified if task work is available for
// running and a kernel transition would be needed to run it. This sets
// IORING_SQ_TASKRUN in the sq ring flags. Not valid with COOP_TASKRUN.
//

//
// Only one task is allowed to submit requests
//

//
// Defer running task work to get events.
// Rather than running bits of task work whenever the task transitions
// try to do it just before it is needed.
//

//
// Application provides the memory for the rings
//

//
// Register the ring fd in itself for use with
// IORING_REGISTER_USE_REGISTERED_RING; return a registered fd index rather
// than an fd.
//

//
// Removes indirection through the SQ index array.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum io_uring_op {
    IORING_OP_NOP,
    IORING_OP_READV,
    IORING_OP_WRITEV,
    IORING_OP_FSYNC,
    IORING_OP_READ_FIXED,
    IORING_OP_WRITE_FIXED,
    IORING_OP_POLL_ADD,
    IORING_OP_POLL_REMOVE,
    IORING_OP_SYNC_FILE_RANGE,
    IORING_OP_SENDMSG,
    IORING_OP_RECVMSG,
    IORING_OP_TIMEOUT,
    IORING_OP_TIMEOUT_REMOVE,
    IORING_OP_ACCEPT,
    IORING_OP_ASYNC_CANCEL,
    IORING_OP_LINK_TIMEOUT,
    IORING_OP_CONNECT,
    IORING_OP_FALLOCATE,
    IORING_OP_OPENAT,
    IORING_OP_CLOSE,
    IORING_OP_FILES_UPDATE,
    IORING_OP_STATX,
    IORING_OP_READ,
    IORING_OP_WRITE,
    IORING_OP_FADVISE,
    IORING_OP_MADVISE,
    IORING_OP_SEND,
    IORING_OP_RECV,
    IORING_OP_OPENAT2,
    IORING_OP_EPOLL_CTL,
    IORING_OP_SPLICE,
    IORING_OP_PROVIDE_BUFFERS,
    IORING_OP_REMOVE_BUFFERS,
    IORING_OP_TEE,
    IORING_OP_SHUTDOWN,
    IORING_OP_RENAMEAT,
    IORING_OP_UNLINKAT,
    IORING_OP_MKDIRAT,
    IORING_OP_SYMLINKAT,
    IORING_OP_LINKAT,
    IORING_OP_MSG_RING,
    IORING_OP_FSETXATTR,
    IORING_OP_SETXATTR,
    IORING_OP_FGETXATTR,
    IORING_OP_GETXATTR,
    IORING_OP_SOCKET,
    IORING_OP_URING_CMD,
    IORING_OP_SEND_ZC,
    IORING_OP_SENDMSG_ZC,
    IORING_OP_READ_MULTISHOT,
    IORING_OP_WAITID,
    IORING_OP_FUTEX_WAIT,
    IORING_OP_FUTEX_WAKE,
    IORING_OP_FUTEX_WAITV,

// this goes last, obviously
    IORING_OP_LAST,
}

//
// sqe->uring_cmd_flags		top 8bits aren't available for userspace
// IORING_URING_CMD_FIXED	use registered buffer; pass this flag
// along with setting sqe->buf_index.
//

//
// sqe->fsync_flags
//

//
// sqe->timeout_flags
//

//
// sqe->splice_flags
// extends splice(2) flags
//

//
// POLL_ADD flags. Note that since sqe->poll_events is the flag space, the
// command flags for POLL_ADD are stored in sqe->len.
//
// IORING_POLL_ADD_MULTI	Multishot poll. Sets IORING_CQE_F_MORE if
// the poll handler will continue to report
// CQEs on behalf of the same SQE.
//
// IORING_POLL_UPDATE		Update existing poll request, matching
// sqe->addr as the old user_data field.
//
// IORING_POLL_LEVEL		Level triggered poll.
//

//
// ASYNC_CANCEL flags.
//
// IORING_ASYNC_CANCEL_ALL	Cancel all requests that match the given key
// IORING_ASYNC_CANCEL_FD	Key off 'fd' for cancelation rather than the
// request 'user_data'
// IORING_ASYNC_CANCEL_ANY	Match any request
// IORING_ASYNC_CANCEL_FD_FIXED	'fd' passed in is a fixed descriptor
// IORING_ASYNC_CANCEL_USERDATA	Match on user_data, default for no other key
// IORING_ASYNC_CANCEL_OP	Match request based on opcode
//

//
// send/sendmsg and recv/recvmsg flags (sqe->ioprio)
//
// IORING_RECVSEND_POLL_FIRST	If set, instead of first attempting to send
// or receive and arm poll if that yields an
// -EAGAIN result, arm poll upfront and skip
// the initial transfer attempt.
//
// IORING_RECV_MULTISHOT	Multishot recv. Sets IORING_CQE_F_MORE if
// the handler will continue to report
// CQEs on behalf of the same SQE.
//
// IORING_RECVSEND_FIXED_BUF	Use registered buffers, the index is stored in
// the buf_index field.
//
// IORING_SEND_ZC_REPORT_USAGE
// If set, SEND[MSG]_ZC should report
// the zerocopy usage in cqe.res
// for the IORING_CQE_F_NOTIF cqe.
// 0 is reported if zerocopy was actually possible.
// IORING_NOTIF_USAGE_ZC_COPIED if data was copied
// (at least partially).
//

//
// cqe.res for IORING_CQE_F_NOTIF if
// IORING_SEND_ZC_REPORT_USAGE was requested
//
// It should be treated as a flag, all other
// bits of cqe.res should be treated as reserved!
//

//
// accept flags stored in sqe->ioprio
//

//
// IORING_OP_MSG_RING command types, stored in sqe->addr
//
// IORING_OP_MSG_RING flags (sqe->msg_ring_flags)
//
// IORING_MSG_RING_CQE_SKIP	Don't post a CQE to the target ring. Not
// applicable for IORING_MSG_DATA, obviously.
//

// Pass through the flags from sqe->file_index to cqe->flags

//
// IO completion data structure (Completion Queue Entry)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_cqe {
    pub /: *mut *mut __u64 user_data; / sqe->data submission passed back,
    pub /: *mut *mut __s32 res; / result code for this event,
    pub flags: __u32,
//
// If the ring is initialized with IORING_SETUP_CQE32, then this field
// contains 16-bytes of padding, doubling the size of the CQE.
//
    pub big_cqe: [__u64; ],
}

//
// cqe->flags
//
// IORING_CQE_F_BUFFER	If set, the upper 16 bits are the buffer ID
// IORING_CQE_F_MORE	If set, parent SQE will generate more CQE entries
// IORING_CQE_F_SOCK_NONEMPTY	If set, more data to read after socket recv
// IORING_CQE_F_NOTIF	Set for notification CQEs. Can be used to distinct
// them from sends.
//

//
// Magic offsets for the application to mmap the data it needs
//

pub const IORING_OFF_CQ_RING: c_uint = 0x8000000ULL;
pub const IORING_OFF_SQES: c_uint = 0x10000000ULL;
pub const IORING_OFF_PBUF_RING: c_uint = 0x80000000ULL;
pub const IORING_OFF_PBUF_SHIFT: c_int = 16;
pub const IORING_OFF_MMAP_MASK: c_uint = 0xf8000000ULL;
//
// Filled with the offset for mmap(2)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_sqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub flags: __u32,
    pub dropped: __u32,
    pub array: __u32,
    pub resv1: __u32,
    pub user_addr: __u64,
}

//
// sq_ring->flags
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_cqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub overflow: __u32,
    pub cqes: __u32,
    pub flags: __u32,
    pub resv1: __u32,
    pub user_addr: __u64,
}

//
// cq_ring->flags
//
// disable eventfd notifications

//
// io_uring_enter(2) flags
//

//
// Passed in for io_uring_setup(2). Copied back with updated info on success
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_params {
    pub sq_entries: __u32,
    pub cq_entries: __u32,
    pub flags: __u32,
    pub sq_thread_cpu: __u32,
    pub sq_thread_idle: __u32,
    pub features: __u32,
    pub wq_fd: __u32,
    pub resv: [__u32; 3],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

//
// io_uring_params->features flags
//

//
// io_uring_register(2) opcodes and arguments
//
// extended with tagging
// set/clear io-wq thread affinities
// set/get max number of io-wq workers
// register/unregister io_uring fd with the ring
// register ring based provide buffer group
// sync cancelation API
// register a range of fixed file slots for automatic slot allocation
// this goes last
// flag added to the opcode to use a registered ring fd
// io-wq worker categories
// deprecated, see struct io_uring_rsrc_update
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_files_update {
    pub offset: __u32,
    pub resv: __u32,
    pub fds: *mut *mut *mut *mut __aligned_u64 / __s32  /,
}

//
// Register a fully sparse file space, rather than pass in an array of all
// -1 file descriptors.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_rsrc_register {
    pub nr: __u32,
    pub flags: __u32,
    pub resv2: __u64,
    pub data: __aligned_u64,
    pub tags: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_rsrc_update {
    pub offset: __u32,
    pub resv: __u32,
    pub data: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_rsrc_update2 {
    pub offset: __u32,
    pub resv: __u32,
    pub data: __aligned_u64,
    pub tags: __aligned_u64,
    pub nr: __u32,
    pub resv2: __u32,
}

// Skip updating fd indexes set to this value in the fd table

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_probe_op {
    pub op: __u8,
    pub resv: __u8,
    pub /: *mut *mut *mut __u16 flags; / IO_URING_OP_ flags,
    pub resv2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_probe {
    pub /: *mut *mut __u8 last_op; / last opcode supported,
    pub /: *mut *mut __u8 ops_len; / length of ops[] array below,
    pub resv: __u16,
    pub resv2: [__u32; 3],
    pub ops: [io_uring_probe_op; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_restriction {
    pub opcode: __u16,
    pub /: *mut *mut __u8 register_op; / IORING_RESTRICTION_REGISTER_OP,
    pub /: *mut *mut __u8 sqe_op; / IORING_RESTRICTION_SQE_OP,
    pub /: *mut *mut *mut __u8 sqe_flags; / IORING_RESTRICTION_SQE_FLAGS_,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_buf {
    pub addr: __u64,
    pub len: __u32,
    pub bid: __u16,
    pub resv: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_buf_ring {
//
// To avoid spilling into more pages than we need to, the
// ring tail is overlaid with the io_uring_buf->resv field.
//
    pub resv1: __u64,
    pub resv2: __u32,
    pub resv3: __u16,
    pub tail: __u16,
}

//
// Flags for IORING_REGISTER_PBUF_RING.
//
// IOU_PBUF_RING_MMAP:	If set, kernel will allocate the memory for the ring.
// The application must not set a ring_addr in struct
// io_uring_buf_reg, instead it must subsequently call
// mmap(2) with the offset set as:
// IORING_OFF_PBUF_RING | (bgid << IORING_OFF_PBUF_SHIFT)
// to get a virtual mapping for the ring.
//
// argument for IORING_(UN)REGISTER_PBUF_RING
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_buf_reg {
    pub ring_addr: __u64,
    pub ring_entries: __u32,
    pub bgid: __u16,
    pub flags: __u16,
    pub resv: [__u64; 3],
}

//
// io_uring_restriction->opcode values
//
// Allow an io_uring_register(2) opcode
// Allow an sqe opcode
// Allow sqe flags
// Require sqe flags (these flags must be set on each submission)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_getevents_arg {
    pub sigmask: __u64,
    pub sigmask_sz: __u32,
    pub pad: __u32,
    pub ts: __u64,
}

//
// Argument for IORING_REGISTER_SYNC_CANCEL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sync_cancel_reg {
    pub addr: __u64,
    pub fd: __s32,
    pub flags: __u32,
    pub timeout: __kernel_timespec,
    pub opcode: __u8,
    pub pad: [__u8; 7],
    pub pad2: [__u64; 3],
}

//
// Argument for IORING_REGISTER_FILE_ALLOC_RANGE
// The range is specified as [off, off + len)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_file_index_range {
    pub off: __u32,
    pub len: __u32,
    pub resv: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_recvmsg_out {
    pub namelen: __u32,
    pub controllen: __u32,
    pub payloadlen: __u32,
    pub flags: __u32,
}

//
// Argument for IORING_OP_URING_CMD when file is a socket
//

