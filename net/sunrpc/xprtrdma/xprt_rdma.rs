//! Automatically rewritten from C Header to Rust Module
//! Source: net/sunrpc/xprtrdma/xprt_rdma.h
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
// Copyright (c) 2014-2017 Oracle.  All rights reserved.
// Copyright (c) 2003-2007 Network Appliance, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the BSD-type
// license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//
// Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials provided
// with the distribution.
//
// Neither the name of the Network Appliance, Inc. nor the names of
// its contributors may be used to endorse or promote products
// derived from this software without specific prior written
// permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

//
// RDMA Endpoint -- connection endpoint details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_ep {
    pub re_kref: kref,
    pub re_id: *mut rdma_cm_id,
    pub re_pd: *mut ib_pd,
    pub re_max_rdma_segs: c_uint,
    pub re_max_fr_depth: c_uint,
    pub re_write_pad_mr: *mut rpcrdma_mr,
    pub re_mrtype: ib_mr_type,
    pub re_done: completion,
    pub re_send_count: c_uint,
    pub re_send_batch: c_uint,
    pub re_max_inline_send: c_uint,
    pub re_max_inline_recv: c_uint,
    pub re_async_rc: c_int,
    pub re_connect_status: c_int,
    pub re_receiving: core::sync::atomic::AtomicI32,
    pub re_force_disconnect: core::sync::atomic::AtomicI32,
    pub re_attr: ib_qp_init_attr,
    pub re_connect_wait: wait_queue_head_t,
    pub re_xprt: *mut rpc_xprt,
    pub re_remote_cma: rdma_conn_param,
    pub re_rn: rpcrdma_notification,
    pub re_receive_count: c_int,
    pub /: *mut *mut unsigned int re_max_requests; / depends on device,
    pub re_recv_batch: c_uint,
    pub /: *mut *mut unsigned int re_inline_send; / negotiated,
    pub /: *mut *mut unsigned int re_inline_recv; / negotiated,
    pub re_completion_ids: core::sync::atomic::AtomicI32,
    pub re_write_pad: [c_char; XDR_UNIT],
}

// Pre-allocate extra Work Requests for handling reverse-direction
// Receives and Sends. This is a fixed value because the Work Queues
// are allocated when the forward channel is set up, long before the
// backchannel is provisioned. This value is two times
// NFS4_DEF_CB_SLOT_TABLE_SIZE.
//

// Registered buffer -- registered kmalloc'd memory for RDMA SEND/RECV
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_regbuf {
    pub rg_iov: ib_sge,
    pub rg_device: *mut ib_device,
    pub rg_direction: dma_data_direction,
    pub rg_data: *mut c_void,
}

// Do not use emergency memory reserves, and fail quickly if memory
// cannot be allocated easily. These flags may be used wherever there
// is robust logic to handle a failure to allocate.
//

// To ensure a transport can always make forward progress,
// the number of RDMA segments allowed in header chunk lists
// is capped at 16. This prevents less-capable devices from
// overrunning the Send buffer while building chunk lists.
//
// Elements of the Read list take up more room than the
// Write list or Reply chunk. 16 read segments means the
// chunk lists cannot consume more than
//
// ((16 + 2) * read segment size) + 1 XDR words,
//
// or about 400 bytes. The fixed part of the header is
// another 24 bytes. Thus when the inline threshold is
// 1024 bytes, at least 600 bytes are available for RPC
// message bodies.
//
// struct rpcrdma_rep -- this structure encapsulates state required
// to receive and complete an RPC Reply, asychronously. It needs
// several pieces of state:
//
// o receive buffer and ib_sge (donated to provider)
// o status of receive (success or not, length, inv rkey)
// o bookkeeping state to get run by reply handler (XDR stream)
//
// These structures are allocated during transport initialization.
// N of these are associated with a transport instance, managed by
// struct rpcrdma_buffer. N is the max number of outstanding RPCs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_rep {
    pub rr_cqe: ib_cqe,
    pub rr_cid: rpc_rdma_cid,
    pub rr_xid: __be32,
    pub rr_vers: __be32,
    pub rr_proc: __be32,
    pub rr_wc_flags: c_int,
    pub rr_inv_rkey: u32,
    pub rr_rdmabuf: *mut rpcrdma_regbuf,
    pub rr_rxprt: *mut rpcrdma_xprt,
    pub rr_rqst: *mut rpc_rqst,
    pub rr_hdrbuf: xdr_buf,
    pub rr_stream: xdr_stream,
    pub rr_node: llist_node,
    pub rr_recv_wr: ib_recv_wr,
    pub rr_all: list_head,
}

// To reduce the rate at which a transport invokes ib_post_recv
// (and thus the hardware doorbell rate), xprtrdma posts Receive
// WRs in batches.
//
// Setting this to zero disables Receive post batching.
//
// struct rpcrdma_sendctx - DMA mapped SGEs to unmap after Send completes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_sendctx {
    pub sc_cqe: ib_cqe,
    pub sc_cid: rpc_rdma_cid,
    pub sc_req: *mut rpcrdma_req,
    pub sc_unmap_count: c_uint,
    pub sc_sges: [ib_sge; ],
}

//
// struct rpcrdma_mr - external memory region metadata
//
// An external memory region is any buffer or page that is registered
// on the fly (ie, not pre-registered).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_mr {
    pub mr_list: list_head,
    pub mr_req: *mut rpcrdma_req,
    pub mr_ibmr: *mut ib_mr,
    pub mr_device: *mut ib_device,
    pub mr_sg: *mut scatterlist,
    pub mr_nents: c_int,
    pub mr_dir: dma_data_direction,
    pub mr_cqe: ib_cqe,
    pub mr_linv_done: completion,
    pub mr_regwr: ib_reg_wr,
    pub mr_invwr: ib_send_wr,
}

//
// struct rpcrdma_req -- structure central to the request/reply sequence.
//
// N of these are associated with a transport instance, and stored in
// struct rpcrdma_buffer. N is the max number of outstanding requests.
//
// It includes pre-registered buffer memory for send AND recv.
// The recv buffer, however, is not owned by this structure, and
// is "donated" to the hardware when a recv is posted. When a
// reply is handled, the recv buffer used is given back to the
// struct rpcrdma_req associated with the request.
//
// In addition to the basic memory, this structure includes an array
// of iovs for send operations. The reason is that the iovs passed to
// ib_post_{send,recv} must not be modified until the work request
// completes.
//
// Maximum number of page-sized "segments" per chunk list to be
// registered or invalidated. Must handle a Reply chunk:
//
// struct rpcrdma_xdr_cursor - tracks position within an xdr_buf
// for iterative MR registration
// @xc_buf: the xdr_buf being iterated
// @xc_page_offset: byte offset into the page region consumed so far
// @xc_flags: combination of XC_* bits
//
// Each XC_*_DONE flag indicates that this region has no
// remaining MR registration work.  That condition holds both when the region
// has already been registered by a prior frwr_map() call and
// when the region is excluded from this chunk type (pre-set
// at init time by rpcrdma_xdr_cursor_init()).  frwr_map()
// treats the two cases identically: skip the region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_xdr_cursor {
    pub xc_buf: *const xdr_buf,
    pub xc_page_offset: c_uint,
    pub xc_flags: c_uint,
}

// The Send SGE array is provisioned to send a maximum size
// inline request:
// - RPC-over-RDMA header
// - xdr_buf head iovec
// - RPCRDMA_MAX_INLINE bytes, in pages
// - xdr_buf tail iovec
//
// The actual number of array elements consumed by each RPC
// depends on the device's max_sge limit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_req {
    pub rl_node: llist_node,
    pub rl_slot: rpc_rqst,
    pub rl_reply: *mut rpcrdma_rep,
    pub rl_stream: xdr_stream,
    pub rl_hdrbuf: xdr_buf,
    pub rl_wr: ib_send_wr,
    pub rl_sendctx: *mut rpcrdma_sendctx,
    pub /: *mut *mut *mut rpcrdma_regbuf rl_rdmabuf; / xprt header,
    pub /: *mut *mut *mut rpcrdma_regbuf rl_sendbuf; / rq_snd_buf,
    pub /: *mut *mut *mut rpcrdma_regbuf rl_recvbuf; / rq_rcv_buf,
    pub rl_all: list_head,
    pub rl_kref: kref,
    pub rl_free_mrs: list_head,
    pub rl_registered: list_head,
}

extern "C" {
    pub fn container_of(_arg: rqst, rpcrdma_req: struct, _arg: rl_slot) -> return;
}
//
// struct rpcrdma_buffer -- holds pre-registered memory for inline
// requests/replies, and client/server credits.
//
// One of these is associated with a transport instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_buffer {
    pub rb_lock: spinlock_t,
    pub rb_send_bufs: llist_head,
    pub rb_mrs: list_head,
    pub rb_sc_head: c_ulong,
    pub rb_sc_tail: c_ulong,
    pub rb_sc_last: c_ulong,
    pub rb_sc_ctxs: *mut rpcrdma_sendctx,
    pub rb_allreqs: list_head,
    pub rb_all_mrs: list_head,
    pub rb_all_reps: list_head,
    pub rb_free_reps: llist_head,
    pub rb_max_requests: __be32,
    pub /: *mut *mut u32 rb_credits; / most recent credit grant,
    pub rb_bc_srv_max_requests: u32,
    pub rb_bc_max_requests: u32,
    pub rb_refresh_worker: work_struct,
}

//
// Statistics for RPCRDMA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_stats {
// accessed when sending a call
    pub read_chunk_count: c_ulong,
    pub write_chunk_count: c_ulong,
    pub reply_chunk_count: c_ulong,
    pub total_rdma_request: c_ulonglong,
// rarely accessed error counters
    pub pullup_copy_count: c_ulonglong,
    pub hardway_register_count: c_ulong,
    pub failed_marshal_count: c_ulong,
    pub bad_reply_count: c_ulong,
    pub mrs_recycled: c_ulong,
    pub mrs_orphaned: c_ulong,
    pub mrs_allocated: c_ulong,
    pub empty_sendctx_q: c_ulong,
// accessed when receiving a reply
    pub total_rdma_reply: c_ulonglong,
    pub fixup_copy_count: c_ulonglong,
    pub local_inv_needed: c_ulong,
    pub nomsg_call_count: c_ulong,
    pub bcall_count: c_ulong,
}

//
// RPCRDMA transport -- encapsulates the structures above for
// integration with RPC.
//
// The contained structures are embedded, not pointers,
// for convenience. This structure need not be visible externally.
//
// It is allocated and initialized during mount, and released
// during unmount.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_xprt {
    pub rx_xprt: rpc_xprt,
    pub rx_ep: *mut rpcrdma_ep,
    pub rx_buf: rpcrdma_buffer,
    pub rx_connect_worker: delayed_work,
    pub rx_timeout: rpc_timeout,
    pub rx_stats: rpcrdma_stats,
}

// Setting this to 0 ensures interoperability with early servers.
// Setting this to 1 enhances unaligned read/write performance.
// Default is 0, see sysctl entry and rpc_rdma.c
// This setting controls the hunt for a supported memory
// registration strategy.
//
// Endpoint calls - xprtrdma/verbs.c
//
extern "C" {
    pub fn rpcrdma_force_disconnect(ep: *mut rpcrdma_ep);
}
extern "C" {
    pub fn rpcrdma_flush_disconnect(r_xprt: *mut rpcrdma_xprt, wc: *mut ib_wc);
}
extern "C" {
    pub fn rpcrdma_xprt_connect(r_xprt: *mut rpcrdma_xprt) -> c_int;
}
extern "C" {
    pub fn rpcrdma_xprt_disconnect(r_xprt: *mut rpcrdma_xprt);
}
extern "C" {
    pub fn rpcrdma_post_recvs(r_xprt: *mut rpcrdma_xprt, needed: c_int);
}
//
// Buffer calls - xprtrdma/verbs.c
//
extern "C" {
    pub fn rpcrdma_req_setup(r_xprt: *mut rpcrdma_xprt, req: *mut rpcrdma_req) -> c_int;
}
extern "C" {
    pub fn rpcrdma_req_destroy(req: *mut rpcrdma_req);
}
extern "C" {
    pub fn rpcrdma_buffer_create(: *mut rpcrdma_xprt) -> c_int;
}
extern "C" {
    pub fn rpcrdma_buffer_destroy(: *mut rpcrdma_buffer);
}
extern "C" {
    pub fn rpcrdma_mrs_refresh(r_xprt: *mut rpcrdma_xprt);
}
extern "C" {
    pub fn rpcrdma_rep_put(buf: *mut rpcrdma_buffer, rep: *mut rpcrdma_rep);
}
extern "C" {
    pub fn rpcrdma_reply_put(buffers: *mut rpcrdma_buffer, req: *mut rpcrdma_req);
}
extern "C" {
    pub fn rpcrdma_req_put(req: *mut rpcrdma_req);
}
//
// rpcrdma_regbuf_is_mapped - check if buffer is DMA mapped
//
// Returns true if the buffer is now mapped to rb->rg_device.
//
// rpcrdma_regbuf_dma_map - DMA-map a regbuf
// @r_xprt: controlling transport instance
// @rb: regbuf to be mapped
//
// Returns true if the buffer is currently DMA mapped.
//
extern "C" {
    pub fn __rpcrdma_regbuf_dma_map(_arg: r_xprt, _arg: rb) -> return;
}
//
// Wrappers for chunk registration, shared by read/write chunk code.
//
// Memory registration calls xprtrdma/frwr_ops.c
//
extern "C" {
    pub fn frwr_reset(req: *mut rpcrdma_req);
}
extern "C" {
    pub fn frwr_query_device(ep: *mut rpcrdma_ep, device: *const ib_device) -> c_int;
}
extern "C" {
    pub fn frwr_mr_init(r_xprt: *mut rpcrdma_xprt, mr: *mut rpcrdma_mr) -> c_int;
}
extern "C" {
    pub fn frwr_mr_release(mr: *mut rpcrdma_mr);
}
extern "C" {
    pub fn frwr_send(r_xprt: *mut rpcrdma_xprt, req: *mut rpcrdma_req) -> c_int;
}
extern "C" {
    pub fn frwr_reminv(rep: *mut rpcrdma_rep, mrs: *mut list_head);
}
extern "C" {
    pub fn frwr_unmap_sync(r_xprt: *mut rpcrdma_xprt, req: *mut rpcrdma_req);
}
extern "C" {
    pub fn frwr_unmap_async(r_xprt: *mut rpcrdma_xprt, req: *mut rpcrdma_req);
}
extern "C" {
    pub fn frwr_wp_create(r_xprt: *mut rpcrdma_xprt) -> c_int;
}
//
// RPC/RDMA protocol calls - xprtrdma/rpc_rdma.c
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpcrdma_chunktype {
    rpcrdma_noch = 0,
    rpcrdma_noch_pullup,
    rpcrdma_noch_mapped,
    rpcrdma_readch,
    rpcrdma_areadch,
    rpcrdma_writech,
    rpcrdma_replych
}

extern "C" {
    pub fn rpcrdma_sendctx_unmap(sc: *mut rpcrdma_sendctx);
}
extern "C" {
    pub fn rpcrdma_marshal_req(r_xprt: *mut rpcrdma_xprt, rqst: *mut rpc_rqst) -> c_int;
}
extern "C" {
    pub fn rpcrdma_set_max_header_sizes(ep: *mut rpcrdma_ep);
}
extern "C" {
    pub fn rpcrdma_reset_cwnd(r_xprt: *mut rpcrdma_xprt);
}
extern "C" {
    pub fn rpcrdma_complete_rqst(rep: *mut rpcrdma_rep);
}
extern "C" {
    pub fn rpcrdma_unpin_rqst(rep: *mut rpcrdma_rep);
}
extern "C" {
    pub fn rpcrdma_reply_handler(rep: *mut rpcrdma_rep);
}
// RPC/RDMA module init - xprtrdma/transport.c
//
extern "C" {
    pub fn xprt_rdma_format_addresses(xprt: *mut rpc_xprt, sap: *mut sockaddr);
}
extern "C" {
    pub fn xprt_rdma_free_addresses(xprt: *mut rpc_xprt);
}
extern "C" {
    pub fn xprt_rdma_close(xprt: *mut rpc_xprt);
}
extern "C" {
    pub fn xprt_rdma_print_stats(xprt: *mut rpc_xprt, seq: *mut seq_file);
}
extern "C" {
    pub fn xprt_rdma_init() -> c_int;
}
extern "C" {
    pub fn xprt_rdma_cleanup();
}
// Backchannel calls - xprtrdma/backchannel.c
//

extern "C" {
    pub fn xprt_rdma_bc_setup(: *mut rpc_xprt, int: unsigned) -> c_int;
}
extern "C" {
    pub fn xprt_rdma_bc_maxpayload(: *mut rpc_xprt) -> usize;
}
extern "C" {
    pub fn xprt_rdma_bc_max_slots(: *mut rpc_xprt) -> c_uint;
}
extern "C" {
    pub fn rpcrdma_bc_receive_call(: *mut rpcrdma_xprt, : *mut rpcrdma_rep);
}
extern "C" {
    pub fn xprt_rdma_bc_send_reply(rqst: *mut rpc_rqst) -> c_int;
}
extern "C" {
    pub fn xprt_rdma_bc_free_rqst(: *mut rpc_rqst);
}
extern "C" {
    pub fn xprt_rdma_bc_destroy(: *mut rpc_xprt, int: unsigned);
}

