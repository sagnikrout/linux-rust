//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/rdmavt_qp.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2016 - 2020 Intel Corporation.
//

//
// Atomic bit definitions for r_aflags.
//
pub const RVT_R_WRID_VALID: c_int = 0;
pub const RVT_R_REWIND_SGE: c_int = 1;
//
// Bit definitions for r_flags.
//
pub const RVT_R_REUSE_SGE: c_uint = 0x01;
pub const RVT_R_RDMAR_SEQ: c_uint = 0x02;
pub const RVT_R_RSP_NAK: c_uint = 0x04;
pub const RVT_R_RSP_SEND: c_uint = 0x08;
pub const RVT_R_COMM_EST: c_uint = 0x10;
//
// If a packet's QP[23:16] bits match this value, then it is
// a PSM packet and the hardware will expect a KDETH header
// following the BTH.
//
pub const RVT_KDETH_QP_PREFIX: c_uint = 0x80;
pub const RVT_KDETH_QP_SUFFIX: c_uint = 0xffff;
pub const RVT_KDETH_QP_PREFIX_MASK: c_uint = 0x00ff0000;
pub const RVT_KDETH_QP_PREFIX_SHIFT: c_int = 16;

//
// If a packet's LNH == BTH and DEST QPN[23:16] in the BTH match this
// prefix value, then it is an AIP packet with a DETH containing the entropy
// value in byte 4 following the BTH.
//
pub const RVT_AIP_QP_PREFIX: c_uint = 0x81;
pub const RVT_AIP_QP_SUFFIX: c_uint = 0xffff;
pub const RVT_AIP_QP_PREFIX_MASK: c_uint = 0x00ff0000;
pub const RVT_AIP_QP_PREFIX_SHIFT: c_int = 16;

//
// Bit definitions for s_flags.
//
// RVT_S_SIGNAL_REQ_WR - set if QP send WRs contain completion signaled
// RVT_S_BUSY - send tasklet is processing the QP
// RVT_S_TIMER - the RC retry timer is active
// RVT_S_ACK_PENDING - an ACK is waiting to be sent after RDMA read/atomics
// RVT_S_WAIT_FENCE - waiting for all prior RDMA read or atomic SWQEs
// before processing the next SWQE
// RVT_S_WAIT_RDMAR - waiting for a RDMA read or atomic SWQE to complete
// before processing the next SWQE
// RVT_S_WAIT_RNR - waiting for RNR timeout
// RVT_S_WAIT_SSN_CREDIT - waiting for RC credits to process next SWQE
// RVT_S_WAIT_DMA - waiting for send DMA queue to drain before generating
// next send completion entry not via send DMA
// RVT_S_WAIT_PIO - waiting for a send buffer to be available
// RVT_S_WAIT_TX - waiting for a struct verbs_txreq to be available
// RVT_S_WAIT_DMA_DESC - waiting for DMA descriptors to be available
// RVT_S_WAIT_KMEM - waiting for kernel memory to be available
// RVT_S_WAIT_PSN - waiting for a packet to exit the send DMA queue
// RVT_S_WAIT_ACK - waiting for an ACK packet before sending more requests
// RVT_S_SEND_ONE - send one packet, request ACK, then wait for ACK
// RVT_S_ECN - a BECN was queued to the send engine
// RVT_S_MAX_BIT_MASK - The max bit that can be used by rdmavt
//
pub const RVT_S_SIGNAL_REQ_WR: c_uint = 0x0001;
pub const RVT_S_BUSY: c_uint = 0x0002;
pub const RVT_S_TIMER: c_uint = 0x0004;
pub const RVT_S_RESP_PENDING: c_uint = 0x0008;
pub const RVT_S_ACK_PENDING: c_uint = 0x0010;
pub const RVT_S_WAIT_FENCE: c_uint = 0x0020;
pub const RVT_S_WAIT_RDMAR: c_uint = 0x0040;
pub const RVT_S_WAIT_RNR: c_uint = 0x0080;
pub const RVT_S_WAIT_SSN_CREDIT: c_uint = 0x0100;
pub const RVT_S_WAIT_DMA: c_uint = 0x0200;
pub const RVT_S_WAIT_PIO: c_uint = 0x0400;
pub const RVT_S_WAIT_TX: c_uint = 0x0800;
pub const RVT_S_WAIT_DMA_DESC: c_uint = 0x1000;
pub const RVT_S_WAIT_KMEM: c_uint = 0x2000;
pub const RVT_S_WAIT_PSN: c_uint = 0x4000;
pub const RVT_S_WAIT_ACK: c_uint = 0x8000;
pub const RVT_S_SEND_ONE: c_uint = 0x10000;
pub const RVT_S_UNLIMITED_CREDIT: c_uint = 0x20000;
pub const RVT_S_ECN: c_uint = 0x40000;
pub const RVT_S_MAX_BIT_MASK: c_uint = 0x800000;
//
// Drivers should use s_flags starting with bit 31 down to the bit next to
// RVT_S_MAX_BIT_MASK
//
// Wait flags that would prevent any packet type from being sent.
//

//
// Wait flags that would prevent send work requests from making progress.
//

// Number of bits to pay attention to in the opcode for checking qp type
pub const RVT_OPCODE_QP_MASK: c_uint = 0xE0;
// Flags for checking QP state (see ib_rvt_state_ops[])
pub const RVT_POST_SEND_OK: c_uint = 0x01;
pub const RVT_POST_RECV_OK: c_uint = 0x02;
pub const RVT_PROCESS_RECV_OK: c_uint = 0x04;
pub const RVT_PROCESS_SEND_OK: c_uint = 0x08;
pub const RVT_PROCESS_NEXT_SEND_OK: c_uint = 0x10;
pub const RVT_FLUSH_SEND: c_uint = 0x20;
pub const RVT_FLUSH_RECV: c_uint = 0x40;

//
// Internal send flags
//

//
// struct rvt_ud_wr - IB UD work plus AH cache
// @wr: valid IB work request
// @attr: pointer to an allocated AH attribute
//
// Special case the UD WR so we can keep track of the AH attributes.
//
// NOTE: This data structure is stricly ordered wr then attr. I.e the attr
// MUST come after wr.  The ib_ud_wr is sized and copied in rvt_post_one_wr.
// The copy assumes that wr is first.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_ud_wr {
    pub wr: ib_ud_wr,
    pub attr: *mut rdma_ah_attr,
}

//
// Send work request queue entry.
// The size of the sg_list is determined when the QP is created and stored
// in qp->s_max_sge.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_swqe {
    pub /: *mut *mut ib_send_wr wr; / don't use wr.sg_list,
    pub ud_wr: rvt_ud_wr,
    pub reg_wr: ib_reg_wr,
    pub rdma_wr: ib_rdma_wr,
    pub atomic_wr: ib_atomic_wr,
}

//
// struct rvt_krwq - kernel struct receive work request
// @p_lock: lock to protect producer of the kernel buffer
// @head: index of next entry to fill
// @c_lock: lock to protect consumer of the kernel buffer
// @tail: index of next entry to pull
// @count: count is approximate of total receive entries posted
// @curr_wq: struct of receive work request queue entry
//
// This structure is used to contain the head pointer,
// tail pointer and receive work queue entries for kernel
// mode user.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_krwq {
    pub /: *mut *mut spinlock_t p_lock; / protect producer,
    pub /: *mut *mut u32 head; / new work requests posted to the head,
// protect consumer
    pub ____cacheline_aligned_in_smp: spinlock_t c_lock,
    pub /: *mut *mut u32 tail; / receives pull requests from here.,
    pub /: *mut *mut u32 count; / approx count of receive entries posted,
    pub curr_wq: *mut rvt_rwqe,
    pub wq: [rvt_rwqe; ],
}

//
// rvt_get_swqe_ah - Return the pointer to the struct rvt_ah
// @swqe: valid Send WQE
//
extern "C" {
    pub fn ibah_to_rvtah(_arg: swqe->ud_wr.wr.ah) -> return;
}
//
// rvt_get_swqe_ah_attr - Return the cached ah attribute information
// @swqe: valid Send WQE
//
// rvt_get_swqe_remote_qpn - Access the remote QPN value
// @swqe: valid Send WQE
//
// rvt_get_swqe_remote_qkey - Acces the remote qkey value
// @swqe: valid Send WQE
//
// rvt_get_swqe_pkey_index - Access the pkey index
// @swqe: valid Send WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_rq {
    pub wq: *mut rvt_rwq,
    pub kwq: *mut rvt_krwq,
    pub /: *mut *mut u32 size; / size of RWQE array,
    pub max_sge: u8,
// protect changes in this struct
    pub ____cacheline_aligned_in_smp: spinlock_t lock,
}

//
// rvt_get_rq_count - count numbers of request work queue entries
// in circular buffer
// @rq: data structure for request queue entry
// @head: head indices of the circular buffer
// @tail: tail indices of the circular buffer
//
// Return - total number of entries in the Receive Queue
//
// This structure holds the information that the send tasklet needs
// to send a RDMA read response or atomic operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_ack_entry {
    pub rdma_sge: rvt_sge,
    pub atomic_data: u64,
    pub psn: u32,
    pub lpsn: u32,
    pub opcode: u8,
    pub sent: u8,
    pub priv: *mut c_void,
}

pub const RC_QP_SCALING_INTERVAL: c_int = 5;
pub const RVT_OPERATION_PRIV: c_uint = 0x00000001;
pub const RVT_OPERATION_ATOMIC: c_uint = 0x00000002;
pub const RVT_OPERATION_ATOMIC_SGE: c_uint = 0x00000004;
pub const RVT_OPERATION_LOCAL: c_uint = 0x00000008;
pub const RVT_OPERATION_USE_RESERVE: c_uint = 0x00000010;
pub const RVT_OPERATION_IGN_RNR_CNT: c_uint = 0x00000020;

//
// struct rvt_operation_params - op table entry
// @length: the length to copy into the swqe entry
// @qpt_support: a bit mask indicating QP type support
// @flags: RVT_OPERATION flags (see above)
//
// This supports table driven post send so that
// the driver can have differing an potentially
// different sets of operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_operation_params {
    pub length: usize,
    pub qpt_support: u32,
    pub flags: u32,
}

//
// Common variables are protected by both r_rq.lock and s_lock in that order
// which only happens in modify_qp() or changing the QP 'state'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_qp {
    pub ibqp: ib_qp,
    pub /: *mut *mut *mut void priv; / Driver private data,
// read mostly fields above and below
    pub remote_ah_attr: rdma_ah_attr,
    pub alt_ah_attr: rdma_ah_attr,
    pub /: *mut *mut *mut rvt_qp __rcu next; / link list for QPN hash table,
    pub /: *mut *mut *mut rvt_swqe s_wq; / send work queue,
    pub ip: *mut rvt_mmap_info,
    pub /: *mut *mut unsigned long timeout_jiffies; / computed from timeout,
    pub /: *mut *mut int srate_mbps; / s_srate (below) converted to Mbit/s,
    pub /: *mut *mut pid_t pid; / pid for user mode QPs,
    pub remote_qpn: u32,
    pub /: *mut *mut u32 qkey; / QKEY for this QP (for UD or RD),
    pub /: *mut *mut u32 s_size; / send work queue size,
    pub /: *mut *mut u16 pmtu; / decoded from path_mtu,
    pub /: *mut *mut u8 log_pmtu; / shift for pmtu,
    pub /: *mut *mut u8 state; / QP state,
    pub /: *mut *mut u8 allowed_ops; / high order bits of allowed opcodes,
    pub qp_access_flags: u8,
    pub /: *mut *mut u8 alt_timeout; / Alternate path timeout for this QP,
    pub /: *mut *mut u8 timeout; / Timeout for this QP,
    pub s_srate: u8,
    pub s_mig_state: u8,
    pub port_num: u8,
    pub /: *mut *mut u8 s_pkey_index; / PKEY index to use,
    pub /: *mut *mut u8 s_alt_pkey_index; / Alternate path PKEY index to use,
    pub /: *mut *mut u8 r_max_rd_atomic; / max number of RDMA read/atomic to receive,
    pub /: *mut *mut u8 s_max_rd_atomic; / max number of RDMA read/atomic to send,
    pub /: *mut *mut u8 s_retry_cnt; / number of times to retry,
    pub s_rnr_retry_cnt: u8,
    pub /: *mut *mut u8 r_min_rnr_timer; / retry timeout value for RNR NAKs,
    pub /: *mut *mut u8 s_max_sge; / size of s_wq->sg_list,
    pub s_draining: u8,
// start of read/write fields
    pub ____cacheline_aligned_in_smp: atomic_t refcount,
    pub wait: wait_queue_head_t,
    pub s_ack_queue: *mut rvt_ack_entry,
    pub s_rdma_read_sge: rvt_sge_state,
    pub /: *mut *mut spinlock_t r_lock ____cacheline_aligned_in_smp; / used for APM,
    pub /: *mut *mut u32 r_psn; / expected rcv packet sequence number,
    pub r_aflags: c_ulong,
    pub /: *mut *mut u64 r_wr_id; / ID for current receive WQE,
    pub /: *mut *mut u32 r_ack_psn; / PSN for next ACK or atomic ACK,
    pub /: *mut *mut u32 r_len; / total length of r_sge,
    pub /: *mut *mut u32 r_rcv_len; / receive data len processed,
    pub /: *mut *mut u32 r_msn; / message sequence number,
    pub /: *mut *mut u8 r_state; / opcode of last packet received,
    pub r_flags: u8,
    pub /: *mut *mut u8 r_head_ack_queue; / index into s_ack_queue[],
    pub /: *mut *mut u8 r_adefered; / defered ack count,
    pub /: *mut *mut list_head rspwait; / link for waiting to respond,
    pub /: *mut *mut rvt_sge_state r_sge; / current receive data,
    pub /: *mut *mut rvt_rq r_rq; / receive work queue,
// post send line
    pub ____cacheline_aligned_in_smp: spinlock_t s_hlock,
    pub /: *mut *mut u32 s_head; / new entries added here,
    pub /: *mut *mut u32 s_next_psn; / PSN for next request,
    pub /: *mut *mut u32 s_avail; / number of entries avail,
    pub /: *mut *mut u32 s_ssn; / SSN of tail entry,
    pub /: *mut *mut atomic_t s_reserved_used; / reserved entries in use,
    pub ____cacheline_aligned_in_smp: spinlock_t s_lock,
    pub s_flags: u32,
    pub s_cur_sge: *mut rvt_sge_state,
    pub s_wqe: *mut rvt_swqe,
    pub /: *mut *mut rvt_sge_state s_sge; / current send request data,
    pub s_rdma_mr: *mut rvt_mregion,
    pub /: *mut *mut u32 s_len; / total length of s_sge,
    pub /: *mut *mut u32 s_rdma_read_len; / total length of s_rdma_read_sge,
    pub /: *mut *mut u32 s_last_psn; / last response PSN processed,
    pub /: *mut *mut u32 s_sending_psn; / lowest PSN that is being sent,
    pub /: *mut *mut u32 s_sending_hpsn; / highest PSN that is being sent,
    pub /: *mut *mut u32 s_psn; / current packet sequence number,
    pub /: *mut *mut u32 s_ack_rdma_psn; / PSN for sending RDMA read responses,
    pub /: *mut *mut u32 s_ack_psn; / PSN for acking sends and RDMA writes,
    pub /: *mut *mut u32 s_tail; / next entry to process,
    pub /: *mut *mut u32 s_cur; / current work queue entry,
    pub /: *mut *mut u32 s_acked; / last un-ACK'ed entry,
    pub /: *mut *mut u32 s_last; / last completed entry,
    pub /: *mut *mut u32 s_lsn; / limit sequence number (credit),
    pub /: *mut *mut u32 s_ahgpsn; / set to the psn in the copy of the header,
    pub /: *mut *mut u16 s_cur_size; / size of send packet in bytes,
    pub s_rdma_ack_cnt: u16,
    pub /: *mut *mut u8 s_hdrwords; / size of s_hdr in 32 bit words,
    pub s_ahgidx: i8,
    pub /: *mut *mut u8 s_state; / opcode of last packet sent,
    pub /: *mut *mut u8 s_ack_state; / opcode of packet to ACK,
    pub /: *mut *mut u8 s_nak_state; / non-zero if NAK is pending,
    pub /: *mut *mut u8 r_nak_state; / non-zero if NAK is pending,
    pub /: *mut *mut u8 s_retry; / requester retry counter,
    pub /: *mut *mut u8 s_rnr_retry; / requester RNR retry counter,
    pub /: *mut *mut u8 s_num_rd_atomic; / number of RDMA read/atomic pending,
    pub /: *mut *mut u8 s_tail_ack_queue; / index into s_ack_queue[],
    pub /: *mut *mut u8 s_acked_ack_queue; / index into s_ack_queue[],
    pub s_ack_rdma_sge: rvt_sge_state,
    pub s_timer: timer_list,
    pub s_rnr_timer: hrtimer,
    pub /: *mut *mut atomic_t local_ops_pending; / number of fast_reg/local_inv reqs,
//
// This sge list MUST be last. Do not add anything below here.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_srq {
    pub ibsrq: ib_srq,
    pub rq: rvt_rq,
    pub ip: *mut rvt_mmap_info,
// send signal when number of RWQEs < limit
    pub limit: u32,
}

extern "C" {
    pub fn container_of(_arg: ibsrq, rvt_srq: struct, _arg: ibsrq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, rvt_qp: struct, _arg: ibqp) -> return;
}

//
// QPN-map pages start out as NULL, they get allocated upon
// first use and are never deallocated. This way,
// large bitmaps are not allocated unless large numbers of QPs are used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_qpn_map {
    pub page: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_qpn_table {
    pub /: *mut *mut spinlock_t lock; / protect changes to the qp table,
    pub /: *mut *mut unsigned flags; / flags for QP0/1 allocated for each port,
    pub /: *mut *mut u32 last; / last QP number allocated,
    pub /: *mut *mut u32 nmaps; / size of the map table,
    pub limit: u16,
    pub incr: u8,
// bit map of free QP numbers other than 0/1
    pub map: [rvt_qpn_map; RVT_QPNMAP_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_qp_ibdev {
    pub qp_table_size: u32,
    pub qp_table_bits: u32,
    pub qp_table: *mut rvt_qp __rcu,
    pub /: *mut *mut spinlock_t qpt_lock; / qptable lock,
    pub qpn_table: rvt_qpn_table,
}

//
// There is one struct rvt_mcast for each multicast GID.
// All attached QPs are then stored as a list of
// struct rvt_mcast_qp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_mcast_qp {
    pub list: list_head,
    pub qp: *mut rvt_qp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_mcast_addr {
    pub mgid: ib_gid,
    pub lid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_mcast {
    pub rb_node: rb_node,
    pub mcast_addr: rvt_mcast_addr,
    pub qp_list: list_head,
    pub wait: wait_queue_head_t,
    pub refcount: core::sync::atomic::AtomicI32,
    pub n_attached: c_int,
}

//
// Since struct rvt_swqe is not a fixed size, we can't simply index into
// struct rvt_qp.s_wq.  This function does the array index computation.
//
// Since struct rvt_rwqe is not a fixed size, we can't simply index into
// struct rvt_rwq.wq.  This function does the array index computation.
//
// rvt_is_user_qp - return if this is user mode QP
// @qp: the target QP
//
// rvt_get_qp - get a QP reference
// @qp: the QP to hold
//
// rvt_put_qp - release a QP reference
// @qp: the QP to release
//
// rvt_put_swqe - drop mr refs held by swqe
// @wqe: the send wqe
//
// This drops any mr references held by the swqe
//
// rvt_qp_wqe_reserve - reserve operation
// @qp: the rvt qp
// @wqe: the send wqe
//
// This routine used in post send to record
// a wqe relative reserved operation use.
//
// rvt_qp_wqe_unreserve - clean reserved operation
// @qp: the rvt qp
// @flags: send wqe flags
//
// This decrements the reserve use count.
//
// This call MUST precede the change to
// s_last to insure that post send sees a stable
// s_avail.
//
// An smp_mp__after_atomic() is used to insure
// the compiler does not juggle the order of the s_last
// ring index and the decrementing of s_reserved_used.
//
// insure no compiler re-order up to s_last change
//
// Compare the lower 24 bits of the msn values.
// Returns an integer <, ==, or > than zero.
//
extern "C" {
    pub fn rvt_compute_aeth(qp: *mut rvt_qp) -> __be32;
}
extern "C" {
    pub fn rvt_get_credit(qp: *mut rvt_qp, aeth: u32);
}
extern "C" {
    pub fn rvt_restart_sge(ss: *mut rvt_sge_state, wqe: *mut rvt_swqe, len: u32) -> u32;
}
//
// rvt_div_round_up_mtu - round up divide
// @qp: the qp pair
// @len: the length
//
// Perform a shift based mtu round up divide
//
// rvt_div_mtu - shift-based divide
// @qp: the qp pair
// @len: the length
//
// Perform a shift based mtu divide
//
// rvt_timeout_to_jiffies - Convert a ULP timeout input into jiffies
// @timeout: timeout input(0 - 31).
//
// Return a timeout value in jiffies.
//
// rvt_lookup_qpn - return the QP with the given QPN
// @rdi: rvt device info structure
// @rvp: the ibport
// @qpn: the QP number to look up
//
// The caller must hold the rcu_read_lock(), and keep the lock until
// the returned qp is no longer in use.
//
// rvt_mod_retry_timer_ext - mod a retry timer
// @qp: the QP
// @shift: timeout shift to wait for multiple packets
// Modify a potentially already running retry timer
//
// 4.096 usec. * (1 << qp->timeout)
extern "C" {
    pub fn rvt_mod_retry_timer_ext(_arg: qp, _arg: 0) -> return;
}
//
// rvt_put_qp_swqe - drop refs held by swqe
// @qp: the send qp
// @wqe: the send wqe
//
// This drops any references held by the swqe
//
// rvt_qp_swqe_incr - increment ring index
// @qp: the qp
// @val: the starting value
//
// Return: the new value wrapping as appropriate
//
extern "C" {
    pub fn rvt_error_qp(qp: *mut rvt_qp, err: ib_wc_status) -> c_int;
}
//
// rvt_recv_cq - add a new entry to completion queue
// by receive queue
// @qp: receive queue
// @wc: work completion entry to add
// @solicited: true if @entry is solicited
//
// This is wrapper function for rvt_enter_cq function call by
// receive queue. If rvt_cq_enter return false, it means cq is
// full and the qp is put into error state.
//
// rvt_send_cq - add a new entry to completion queue
// by send queue
// @qp: send queue
// @wc: work completion entry to add
// @solicited: true if @entry is solicited
//
// This is wrapper function for rvt_enter_cq function call by
// send queue. If rvt_cq_enter return false, it means cq is
// full and the qp is put into error state.
//
// rvt_qp_complete_swqe - insert send completion
// @qp: the qp
// @wqe: the send wqe
// @opcode: wc operation (driver dependent)
// @status: completion status
//
// Update the s_last information, and then insert a send
// completion into the completion
// queue if the qp indicates it should be done.
//
// See IBTA 10.7.3.1 for info on completion
// control.
//
// Return: new last
//
// above fields required before writing s_last
// see rvt_qp_is_avail()
extern "C" {
    pub fn rvt_get_rwqe(qp: *mut rvt_qp, wr_id_only: bool) -> c_int;
}
extern "C" {
    pub fn rvt_comm_est(qp: *mut rvt_qp);
}
extern "C" {
    pub fn rvt_rc_error(qp: *mut rvt_qp, err: ib_wc_status);
}
extern "C" {
    pub fn rvt_rnr_tbl_to_usec(index: u32) -> c_ulong;
}
extern "C" {
    pub fn rvt_rc_rnr_retry(t: *mut hrtimer) -> hrtimer_restart;
}
extern "C" {
    pub fn rvt_add_rnr_timer(qp: *mut rvt_qp, aeth: u32);
}
extern "C" {
    pub fn rvt_del_timers_sync(qp: *mut rvt_qp);
}
extern "C" {
    pub fn rvt_stop_rc_timers(qp: *mut rvt_qp);
}
extern "C" {
    pub fn rvt_add_retry_timer_ext(qp: *mut rvt_qp, shift: u8);
}
extern "C" {
    pub fn rvt_ruc_loopback(qp: *mut rvt_qp);
}
//
// struct rvt_qp_iter - the iterator for QPs
// @qp: the current QP
//
// This structure defines the current iterator
// state for sequenced access to all QPs relative
// to an rvt_dev_info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_qp_iter {
    pub qp: *mut rvt_qp,
// private: backpointer
    pub rdi: *mut rvt_dev_info,
// private: callback routine
    pub v): *mut *mut *mut void (cb)(struct rvt_qp qp, u64,
// private: for arg to callback routine
    pub v: u64,
// private: number of SMI,GSI QPs for device
    pub specials: c_int,
// private: current iterator index
    pub n: c_int,
}

//
// ib_cq_tail - Return tail index of cq buffer
// @send_cq: The cq for send
//
// This is called in qp_iter_print to get tail
// of cq buffer.
//
// ib_cq_head - Return head index of cq buffer
// @send_cq: The cq for send
//
// This is called in qp_iter_print to get head
// of cq buffer.
//
// rvt_free_rq - free memory allocated for rvt_rq struct
// @rq: request queue data structure
//
// This function should only be called if the rvt_mmap_info()
// has not succeeded.
//
// rvt_to_iport - Get the ibport pointer
// @qp: the qp pointer
//
// This function returns the ibport pointer from the qp pointer.
//
// rvt_rc_credit_avail - Check if there are enough RC credits for the request
// @qp: the qp
// @wqe: the request
//
// This function returns false when there are not enough credits for the given
// request and true otherwise.
//
extern "C" {
    pub fn rvt_qp_iter_next(iter: *mut rvt_qp_iter) -> c_int;
}
extern "C" {
    pub fn rvt_qp_mr_clean(qp: *mut rvt_qp, lkey: u32);
}
