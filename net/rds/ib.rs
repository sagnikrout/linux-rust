//! Automatically rewritten from C Header to Rust Module
//! Source: net/rds/ib.h
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

pub const RDS_IB_MAX_SGE: c_int = 8;
pub const RDS_IB_RECV_SGE: c_int = 2;
pub const RDS_IB_DEFAULT_RECV_WR: c_int = 1024;
pub const RDS_IB_DEFAULT_SEND_WR: c_int = 256;
pub const RDS_IB_DEFAULT_FR_WR: c_int = 512;
pub const RDS_IB_DEFAULT_RETRY_COUNT: c_int = 1;
pub const RDS_IB_SUPPORTED_PROTOCOLS: c_uint = 0x00000003	/* minor versions supported */;
pub const RDS_IB_RECYCLE_BATCH_COUNT: c_int = 32;
pub const RDS_IB_WC_MAX: c_int = 32;
//
// IB posts RDS_FRAG_SIZE fragments of pages to the receive queues to
// try and minimize the amount of memory tied up both the device and
// socket receive queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_page_frag {
    pub f_item: list_head,
    pub f_cache_entry: list_head,
    pub f_sg: scatterlist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_incoming {
    pub ii_frags: list_head,
    pub ii_cache_entry: list_head,
    pub ii_inc: rds_incoming,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_cache_head {
    pub first: *mut list_head,
    pub count: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_refill_cache {
    pub percpu: *mut rds_ib_cache_head __percpu,
    pub xfer: *mut list_head,
    pub ready: *mut list_head,
}

// This is the common structure for the IB private data exchange in setting up
// an RDS connection.  The exchange is different for IPv4 and IPv6 connections.
// The reason is that the address size is different and the addresses
// exchanged are in the beginning of the structure.  Hence it is not possible
// for interoperability if same structure is used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_conn_priv_cmn {
    pub ricpc_protocol_major: u8,
    pub ricpc_protocol_minor: u8,
    pub /: *mut *mut __be16 ricpc_protocol_minor_mask; / bitmask,
    pub ricpc_dp_toss: u8,
    pub ripc_reserved1: u8,
    pub ripc_reserved2: __be16,
    pub ricpc_ack_seq: __be64,
    pub /: *mut *mut __be32 ricpc_credit; / non-zero enables flow ctl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_connect_private {
// Add new fields at the end, and don't permute existing fields.
    pub dp_saddr: __be32,
    pub dp_daddr: __be32,
    pub dp_cmn: rds_ib_conn_priv_cmn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds6_ib_connect_private {
// Add new fields at the end, and don't permute existing fields.
    pub dp_saddr: in6_addr,
    pub dp_daddr: in6_addr,
    pub dp_cmn: rds_ib_conn_priv_cmn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rds_ib_conn_priv {
    pub ricp_v4: rds_ib_connect_private,
    pub ricp_v6: rds6_ib_connect_private,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_send_work {
    pub s_op: *mut c_void,
    pub s_wr: ib_send_wr,
    pub s_rdma_wr: ib_rdma_wr,
    pub s_atomic_wr: ib_atomic_wr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_recv_work {
    pub r_ibinc: *mut rds_ib_incoming,
    pub r_frag: *mut rds_page_frag,
    pub r_wr: ib_recv_wr,
    pub r_sge: [ib_sge; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_work_ring {
    pub w_nr: u32,
    pub w_alloc_ptr: u32,
    pub w_alloc_ctr: u32,
    pub w_free_ptr: u32,
    pub w_free_ctr: core::sync::atomic::AtomicI32,
}

// Rings are posted with all the allocations they'll need to queue the
// incoming message to the receiving socket so this can't fail.
// All fragments start with a header, so we can make sure we're not receiving
// garbage, and we can tell a small 8 byte fragment from an ACK frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_ack_state {
    pub ack_next: u64,
    pub ack_recv: u64,
    pub ack_required:1: c_uint,
    pub ack_next_valid:1: c_uint,
    pub ack_recv_valid:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_connection {
    pub ib_node: list_head,
    pub rds_ibdev: *mut rds_ib_device,
    pub conn: *mut rds_connection,
// alphabet soup, IBTA style
    pub i_cm_id: *mut rdma_cm_id,
    pub i_pd: *mut ib_pd,
    pub i_send_cq: *mut ib_cq,
    pub i_recv_cq: *mut ib_cq,
    pub i_send_wc: [ib_wc; RDS_IB_WC_MAX],
    pub i_recv_wc: [ib_wc; RDS_IB_WC_MAX],
// To control the number of wrs from fastreg
    pub i_fastreg_wrs: core::sync::atomic::AtomicI32,
    pub i_fastreg_inuse_count: core::sync::atomic::AtomicI32,
// interrupt handling
    pub i_send_tasklet: tasklet_struct,
    pub i_recv_tasklet: tasklet_struct,
// tx
    pub i_send_ring: rds_ib_work_ring,
    pub i_data_op: *mut rm_data_op,
    pub i_send_hdrs: *mut rds_header,
    pub i_send_hdrs_dma: *mut dma_addr_t,
    pub i_sends: *mut rds_ib_send_work,
    pub i_signaled_sends: core::sync::atomic::AtomicI32,
// rx
    pub i_recv_mutex: mutex,
    pub i_recv_ring: rds_ib_work_ring,
    pub i_ibinc: *mut rds_ib_incoming,
    pub i_recv_data_rem: u32,
    pub i_recv_hdrs: *mut rds_header,
    pub i_recv_hdrs_dma: *mut dma_addr_t,
    pub i_recvs: *mut rds_ib_recv_work,
    pub /: *mut *mut u64 i_ack_recv; / last ACK received,
    pub i_cache_incs: rds_ib_refill_cache,
    pub i_cache_frags: rds_ib_refill_cache,
    pub i_cache_allocs: core::sync::atomic::AtomicI32,
// sending acks
    pub i_ack_flags: c_ulong,

    pub /: *mut *mut atomic64_t i_ack_next; / next ACK to send,

    pub /: *mut *mut spinlock_t i_ack_lock; / protect i_ack_next,
    pub /: *mut *mut u64 i_ack_next; / next ACK to send,

    pub i_ack: *mut rds_header,
    pub i_ack_wr: ib_send_wr,
    pub i_ack_sge: ib_sge,
    pub i_ack_dma: dma_addr_t,
    pub i_ack_queued: c_ulong,
// Flow control related information
//
// Our algorithm uses a pair variables that we need to access
// atomically - one for the send credits, and one posted
// recv credits we need to transfer to remote.
// Rather than protect them using a slow spinlock, we put both into
// a single atomic_t and update it using cmpxchg
//
    pub i_credits: core::sync::atomic::AtomicI32,
// Protocol version specific information
    pub /: *mut *mut unsigned int i_flowctl:1; / enable/disable flow ctl,
// Batched completions
    pub i_unsignaled_wrs: c_uint,
// Endpoint role in connection
    pub i_active_side: bool,
    pub i_cq_quiesce: core::sync::atomic::AtomicI32,
// Send/Recv vectors
    pub i_scq_vector: c_int,
    pub i_rcq_vector: c_int,
    pub i_sl: u8,
}

// This assumes that atomic_t is at least 32 bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_ipaddr {
    pub list: list_head,
    pub ipaddr: __be32,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_device {
    pub list: list_head,
    pub ipaddr_list: list_head,
    pub conn_list: list_head,
    pub dev: *mut ib_device,
    pub pd: *mut ib_pd,
    pub odp_capable:1: u8,
    pub max_mrs: c_uint,
    pub mr_1m_pool: *mut rds_ib_mr_pool,
    pub mr_8k_pool: *mut rds_ib_mr_pool,
    pub max_8k_mrs: c_uint,
    pub max_1m_mrs: c_uint,
    pub max_sge: c_int,
    pub max_wrs: c_uint,
    pub max_initiator_depth: c_uint,
    pub max_responder_resources: c_uint,
    pub /: *mut *mut spinlock_t spinlock; / protect the above,
    pub refcount: refcount_t,
    pub free_work: work_struct,
    pub vector_load: *mut c_int,
}

// bits for i_ack_flags
pub const IB_ACK_IN_FLIGHT: c_int = 0;
pub const IB_ACK_REQUESTED: c_int = 1;
// Magic WR_ID for ACKs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_statistics {
    pub s_ib_connect_raced: u64,
    pub s_ib_listen_closed_stale: u64,
    pub s_ib_evt_handler_call: u64,
    pub s_ib_tasklet_call: u64,
    pub s_ib_tx_cq_event: u64,
    pub s_ib_tx_ring_full: u64,
    pub s_ib_tx_throttle: u64,
    pub s_ib_tx_sg_mapping_failure: u64,
    pub s_ib_tx_stalled: u64,
    pub s_ib_tx_credit_updates: u64,
    pub s_ib_rx_cq_event: u64,
    pub s_ib_rx_ring_empty: u64,
    pub s_ib_rx_refill_from_cq: u64,
    pub s_ib_rx_refill_from_thread: u64,
    pub s_ib_rx_alloc_limit: u64,
    pub s_ib_rx_total_frags: u64,
    pub s_ib_rx_total_incs: u64,
    pub s_ib_rx_credit_updates: u64,
    pub s_ib_ack_sent: u64,
    pub s_ib_ack_send_failure: u64,
    pub s_ib_ack_send_delayed: u64,
    pub s_ib_ack_send_piggybacked: u64,
    pub s_ib_ack_received: u64,
    pub s_ib_rdma_mr_8k_alloc: u64,
    pub s_ib_rdma_mr_8k_free: u64,
    pub s_ib_rdma_mr_8k_used: u64,
    pub s_ib_rdma_mr_8k_pool_flush: u64,
    pub s_ib_rdma_mr_8k_pool_wait: u64,
    pub s_ib_rdma_mr_8k_pool_depleted: u64,
    pub s_ib_rdma_mr_1m_alloc: u64,
    pub s_ib_rdma_mr_1m_free: u64,
    pub s_ib_rdma_mr_1m_used: u64,
    pub s_ib_rdma_mr_1m_pool_flush: u64,
    pub s_ib_rdma_mr_1m_pool_wait: u64,
    pub s_ib_rdma_mr_1m_pool_depleted: u64,
    pub s_ib_rdma_mr_8k_reused: u64,
    pub s_ib_rdma_mr_1m_reused: u64,
    pub s_ib_atomic_cswp: u64,
    pub s_ib_atomic_fadd: u64,
    pub s_ib_recv_added_to_cache: u64,
    pub s_ib_recv_removed_from_cache: u64,
}

//
// Fake ib_dma_sync_sg_for_{cpu,device} as long as ib_verbs.h
// doesn't define it.
//

// ib.c
extern "C" {
    pub fn rds_ib_dev_put(rds_ibdev: *mut rds_ib_device);
}
// ib_cm.c
extern "C" {
    pub fn rds_ib_conn_alloc(conn: *mut rds_connection, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn rds_ib_conn_free(arg: *mut c_void);
}
extern "C" {
    pub fn rds_ib_conn_path_connect(cp: *mut rds_conn_path) -> c_int;
}
extern "C" {
    pub fn rds_ib_conn_path_shutdown(cp: *mut rds_conn_path);
}
extern "C" {
    pub fn __rds_ib_conn_error(conn: *mut rds_connection, : *const c_char, ...);
}
extern "C" {
    pub fn rds_ib_cm_initiate_connect(cm_id: *mut rdma_cm_id, isv6: bool) -> c_int;
}

// ib_rdma.c
extern "C" {
    pub fn rds_ib_add_conn(rds_ibdev: *mut rds_ib_device, conn: *mut rds_connection);
}
extern "C" {
    pub fn rds_ib_remove_conn(rds_ibdev: *mut rds_ib_device, conn: *mut rds_connection);
}
extern "C" {
    pub fn rds_ib_destroy_nodev_conns();
}
extern "C" {
    pub fn rds_ib_mr_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc);
}
// ib_recv.c
extern "C" {
    pub fn rds_ib_recv_init() -> c_int;
}
extern "C" {
    pub fn rds_ib_recv_exit();
}
extern "C" {
    pub fn rds_ib_recv_path(conn: *mut rds_conn_path) -> c_int;
}
extern "C" {
    pub fn rds_ib_recv_alloc_caches(ic: *mut rds_ib_connection, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn rds_ib_recv_free_caches(ic: *mut rds_ib_connection);
}
extern "C" {
    pub fn rds_ib_recv_refill(conn: *mut rds_connection, prefill: c_int, gfp: gfp_t);
}
extern "C" {
    pub fn rds_ib_inc_free(inc: *mut rds_incoming);
}
extern "C" {
    pub fn rds_ib_inc_copy_to_user(inc: *mut rds_incoming, to: *mut iov_iter) -> c_int;
}
extern "C" {
    pub fn rds_ib_recv_init_ring(ic: *mut rds_ib_connection);
}
extern "C" {
    pub fn rds_ib_recv_clear_ring(ic: *mut rds_ib_connection);
}
extern "C" {
    pub fn rds_ib_recv_init_ack(ic: *mut rds_ib_connection);
}
extern "C" {
    pub fn rds_ib_attempt_ack(ic: *mut rds_ib_connection);
}
extern "C" {
    pub fn rds_ib_ack_send_complete(ic: *mut rds_ib_connection);
}
extern "C" {
    pub fn rds_ib_piggyb_ack(ic: *mut rds_ib_connection) -> u64;
}
extern "C" {
    pub fn rds_ib_set_ack(ic: *mut rds_ib_connection, seq: u64, ack_required: c_int);
}
// ib_ring.c
extern "C" {
    pub fn rds_ib_ring_init(ring: *mut rds_ib_work_ring, nr: u32);
}
extern "C" {
    pub fn rds_ib_ring_resize(ring: *mut rds_ib_work_ring, nr: u32);
}
extern "C" {
    pub fn rds_ib_ring_alloc(ring: *mut rds_ib_work_ring, val: u32, pos: *mut u32) -> u32;
}
extern "C" {
    pub fn rds_ib_ring_free(ring: *mut rds_ib_work_ring, val: u32);
}
extern "C" {
    pub fn rds_ib_ring_unalloc(ring: *mut rds_ib_work_ring, val: u32);
}
extern "C" {
    pub fn rds_ib_ring_empty(ring: *mut rds_ib_work_ring) -> c_int;
}
extern "C" {
    pub fn rds_ib_ring_low(ring: *mut rds_ib_work_ring) -> c_int;
}
extern "C" {
    pub fn rds_ib_ring_oldest(ring: *mut rds_ib_work_ring) -> u32;
}
extern "C" {
    pub fn rds_ib_ring_completed(ring: *mut rds_ib_work_ring, wr_id: u32, oldest: u32) -> u32;
}
// ib_send.c
extern "C" {
    pub fn rds_ib_xmit_path_complete(cp: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_ib_send_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc);
}
extern "C" {
    pub fn rds_ib_send_init_ring(ic: *mut rds_ib_connection);
}
extern "C" {
    pub fn rds_ib_send_clear_ring(ic: *mut rds_ib_connection);
}
extern "C" {
    pub fn rds_ib_xmit_rdma(conn: *mut rds_connection, op: *mut rm_rdma_op) -> c_int;
}
extern "C" {
    pub fn rds_ib_send_add_credits(conn: *mut rds_connection, credits: c_uint);
}
extern "C" {
    pub fn rds_ib_advertise_credits(conn: *mut rds_connection, posted: c_uint);
}
extern "C" {
    pub fn rds_ib_xmit_atomic(conn: *mut rds_connection, op: *mut rm_atomic_op) -> c_int;
}
// ib_stats.c

// ib_sysctl.c
extern "C" {
    pub fn rds_ib_sysctl_init() -> c_int;
}
extern "C" {
    pub fn rds_ib_sysctl_exit();
}
