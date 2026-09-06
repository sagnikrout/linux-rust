//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/smbdirect/socket.h
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
// Copyright (c) 2025 Stefan Metzmacher
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smbdirect_socket_status {
    SMBDIRECT_SOCKET_CREATED,
    SMBDIRECT_SOCKET_LISTENING,
    SMBDIRECT_SOCKET_RESOLVE_ADDR_NEEDED,
    SMBDIRECT_SOCKET_RESOLVE_ADDR_RUNNING,
    SMBDIRECT_SOCKET_RESOLVE_ADDR_FAILED,
    SMBDIRECT_SOCKET_RESOLVE_ROUTE_NEEDED,
    SMBDIRECT_SOCKET_RESOLVE_ROUTE_RUNNING,
    SMBDIRECT_SOCKET_RESOLVE_ROUTE_FAILED,
    SMBDIRECT_SOCKET_RDMA_CONNECT_NEEDED,
    SMBDIRECT_SOCKET_RDMA_CONNECT_RUNNING,
    SMBDIRECT_SOCKET_RDMA_CONNECT_FAILED,
    SMBDIRECT_SOCKET_NEGOTIATE_NEEDED,
    SMBDIRECT_SOCKET_NEGOTIATE_RUNNING,
    SMBDIRECT_SOCKET_NEGOTIATE_FAILED,
    SMBDIRECT_SOCKET_CONNECTED,
    SMBDIRECT_SOCKET_ERROR,
    SMBDIRECT_SOCKET_DISCONNECTING,
    SMBDIRECT_SOCKET_DISCONNECTED,
    SMBDIRECT_SOCKET_DESTROYED
}

//
// This can be used with %1pe to print errors as strings or '0'
// And it avoids warnings like: warn: passing zero to 'ERR_PTR'
// from smatch -p=kernel --pedantic
//
extern "C" {
    pub fn ERR_PTR(_arg: error) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smbdirect_keepalive_status {
    SMBDIRECT_KEEPALIVE_NONE,
    SMBDIRECT_KEEPALIVE_PENDING,
    SMBDIRECT_KEEPALIVE_SENT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_socket {
    pub status: smbdirect_socket_status,
    pub status_wait: wait_queue_head_t,
    pub first_error: c_int,
//
// This points to the workqueues to
// be used for this socket.
//
    pub accept: *mut workqueue_struct,
    pub connect: *mut workqueue_struct,
    pub idle: *mut workqueue_struct,
    pub refill: *mut workqueue_struct,
    pub immediate: *mut workqueue_struct,
    pub cleanup: *mut workqueue_struct,
    pub workqueues: },
    pub disconnect_work: work_struct,
//
// The reference counts.
//
// This holds the references by the
// frontend, typically the smb layer.
//
// It is typically 1 and a disconnect
// will happen if it reaches 0.
//
    pub disconnect: kref,
//
// This holds the reference by the
// backend, the code that manages
// the lifetime of the whole
// struct smbdirect_socket,
// if this reaches 0 it can will
// be freed.
//
// Can be REFCOUNT_MAX is part
// of another structure.
//
// This is equal or higher than
// the disconnect refcount.
//
    pub destroy: kref,
    pub refs: },
// RDMA related
    pub cm_id: *mut rdma_cm_id,
//
// The expected event in our current
// cm_id->event_handler, all other events
// are treated as an error.
//
    pub expected_event: rdma_cm_event_type,
//
// This is for iWarp MPA v1
//
    pub legacy_iwarp: bool,
    pub rdma: },
// IB verbs related
    pub pd: *mut ib_pd,
    pub poll_ctx: ib_poll_context,
    pub send_cq: *mut ib_cq,
    pub recv_cq: *mut ib_cq,
//
// shortcuts for rdma.cm_id->{qp,device};
//
    pub qp: *mut ib_qp,
    pub dev: *mut ib_device,
    pub ib: },
    pub parameters: smbdirect_socket_parameters,
//
// The state for connect/negotiation
//
    pub lock: spinlock_t,
    pub work: work_struct,
    pub connect: },
//
// The state for keepalive and timeout handling
//
    pub keepalive: smbdirect_keepalive_status,
    pub immediate_work: work_struct,
    pub timer_work: delayed_work,
    pub idle: },
//
// The state for listen sockets
//
    pub lock: spinlock_t,
    pub pending: list_head,
    pub ready: list_head,
    pub wait_queue: wait_queue_head_t,
//
// This starts as -1 and a value != -1
// means this socket was in LISTENING state
// before. Note the valid backlog can
// only be > 0.
//
    pub backlog: c_int,
    pub listen: },
//
// The state for sockets waiting
// for accept, either still waiting
// for the negotiation to finish
// or already ready with a usable
// connection.
//
    pub listener: *mut smbdirect_socket,
    pub list: list_head,
    pub accept: },
//
// The state for posted send buffers
//
// Memory pools for preallocating
// smbdirect_send_io buffers
//
    pub cache: *mut kmem_cache,
    pub pool: *mut mempool_t,
    pub gfp_mask: gfp_t,
    pub mem: },
//
// This is a coordination for smbdirect_send_batch.
//
// There's only one possible credit, which means
// only one instance is running at a time.
//
    pub count: core::sync::atomic::AtomicI32,
    pub wait_queue: wait_queue_head_t,
    pub bcredits: },
//
// The local credit state for ib_post_send()
//
    pub count: core::sync::atomic::AtomicI32,
    pub wait_queue: wait_queue_head_t,
    pub lcredits: },
//
// The remote credit state for the send side
//
    pub count: core::sync::atomic::AtomicI32,
    pub wait_queue: wait_queue_head_t,
    pub credits: },
//
// The state about posted/pending sends
//
    pub count: core::sync::atomic::AtomicI32,
//
// woken when count reached zero
//
    pub zero_wait_queue: wait_queue_head_t,
    pub pending: },
    pub send_io: },
//
// The state for posted receive buffers
//
// The type of PDU we are expecting
//
    pub expected: },
//
// Memory pools for preallocating
// smbdirect_recv_io buffers
//
    pub cache: *mut kmem_cache,
    pub pool: *mut mempool_t,
    pub gfp_mask: gfp_t,
    pub mem: },
//
// The list of free smbdirect_recv_io
// structures
//
    pub list: list_head,
    pub lock: spinlock_t,
    pub free: },
//
// The state for posted recv_io messages
// and the refill work struct.
//
    pub count: core::sync::atomic::AtomicI32,
    pub refill_work: work_struct,
    pub posted: },
//
// The credit state for the recv side
//
    pub target: u16,
    pub available: core::sync::atomic::AtomicI32,
    pub count: core::sync::atomic::AtomicI32,
    pub credits: },
//
// The list of arrived non-empty smbdirect_recv_io
// structures
//
// This represents the reassembly queue.
//
    pub list: list_head,
    pub lock: spinlock_t,
    pub wait_queue: wait_queue_head_t,
// total data length of reassembly queue
    pub data_length: c_int,
    pub queue_length: c_int,
// the offset to first buffer in reassembly queue
    pub first_entry_offset: c_int,
//
// Indicate if we have received a full packet on the
// connection This is used to identify the first SMBD
// packet of a assembled payload (SMB packet) in
// reassembly queue so we can return a RFC1002 length to
// upper layer to indicate the length of the SMB packet
// received
//
    pub full_packet_received: bool,
    pub reassembly: },
    pub recv_io: },
//
// The state for Memory registrations on the client
//
    pub type: ib_mr_type,
//
// The list of free smbdirect_mr_io
// structures
//
    pub list: list_head,
    pub lock: spinlock_t,
    pub all: },
//
// The number of available MRs ready for memory registration
//
    pub count: core::sync::atomic::AtomicI32,
    pub wait_queue: wait_queue_head_t,
    pub ready: },
//
// The number of used MRs
//
    pub count: core::sync::atomic::AtomicI32,
    pub used: },
    pub mr_io: },
//
// The state for RDMA read/write requests on the server
//
// Memory hints for
// smbdirect_rw_io structs
//
    pub gfp_mask: gfp_t,
    pub mem: },
//
// The credit state for the send side
//
// The maximum number of rw credits
//
    pub max: usize,
//
// The number of pages per credit
//
    pub num_pages: usize,
    pub count: core::sync::atomic::AtomicI32,
    pub wait_queue: wait_queue_head_t,
    pub credits: },
    pub rw_io: },
//
// For debug purposes
//
    pub get_receive_buffer: u64,
    pub put_receive_buffer: u64,
    pub enqueue_reassembly_queue: u64,
    pub dequeue_reassembly_queue: u64,
    pub send_empty: u64,
    pub statistics: },
    pub private_ptr: *mut c_void,
    pub cls): c_uint,
    pub vaf): *mut va_format,
    pub logging: },
}

//
// Should never be called as disable_[delayed_]work_sync() was used.
//
// Should never be called, the caller should
// set it's own functions.
//
// Should never be called, the caller should
// set it's own functions.
//

//
// This also sets status = SMBDIRECT_SOCKET_CREATED
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_send_io {
    pub socket: *mut smbdirect_socket,
    pub cqe: ib_cqe,
//
// The SGE entries for this work request
//
// The first points to the packet header
//
pub const SMBDIRECT_SEND_IO_MAX_SGE: c_int = 6;
    pub num_sge: usize,
    pub sge: [ib_sge; SMBDIRECT_SEND_IO_MAX_SGE],
//
// Link to the list of sibling smbdirect_send_io
// messages.
//
    pub sibling_list: list_head,
    pub wr: ib_send_wr,
// SMBD packet header follows this structure
    pub packet: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_send_batch {
//
// List of smbdirect_send_io messages
//
    pub msg_list: list_head,
//
// Number of list entries
//
    pub wr_cnt: usize,
//
// Possible remote key invalidation state
//
    pub need_invalidate_rkey: bool,
    pub remote_key: u32,
    pub credit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_recv_io {
    pub socket: *mut smbdirect_socket,
    pub cqe: ib_cqe,
//
// For now we only use a single SGE
// as we have just one large buffer
// per posted recv.
//
pub const SMBDIRECT_RECV_IO_MAX_SGE: c_int = 1;
    pub sge: ib_sge,
// Link to free or reassembly list
    pub list: list_head,
// Indicate if this is the 1st packet of a payload
    pub first_segment: bool,
// SMBD packet header and payload follows this structure
    pub packet: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smbdirect_mr_state {
    SMBDIRECT_MR_READY,
    SMBDIRECT_MR_REGISTERED,
    SMBDIRECT_MR_INVALIDATED,
    SMBDIRECT_MR_ERROR,
    SMBDIRECT_MR_DISABLED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_mr_io {
    pub socket: *mut smbdirect_socket,
    pub cqe: ib_cqe,
//
// We can have up to two references:
// 1. by the connection
// 2. by the registration
//
    pub kref: kref,
    pub mutex: mutex,
    pub list: list_head,
    pub state: smbdirect_mr_state,
    pub mr: *mut ib_mr,
    pub sgt: sg_table,
    pub dir: dma_data_direction,
    pub wr: ib_reg_wr,
    pub inv_wr: ib_send_wr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_rw_io {
    pub socket: *mut smbdirect_socket,
    pub cqe: ib_cqe,
    pub list: list_head,
    pub error: c_int,
    pub completion: *mut completion,
    pub rdma_ctx: rdma_rw_ctx,
    pub sgt: sg_table,
    pub sg_list: [scatterlist; ],
}

//
// Maximum number of retries on data transfer operations
//
pub const SMBDIRECT_RDMA_CM_RETRY: c_int = 6;
//
// No need to retry on Receiver Not Ready since SMB_DIRECT manages credits
//
pub const SMBDIRECT_RDMA_CM_RNR_RETRY: c_int = 0;
