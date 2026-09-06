//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/svc_rdma.h
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
// Copyright (c) 2005-2006 Network Appliance, Inc. All rights reserved.
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
// Author: Tom Tucker <tom@opengridcomputing.com>
//

// Default and maximum inline threshold sizes
// RPC/RDMA parameters and stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svcxprt_rdma {
    pub /: *mut *mut svc_xprt sc_xprt; / SVC transport structure,
    pub /: *mut *mut *mut rdma_cm_id sc_cm_id; / RDMA connection id,
    pub /: *mut *mut list_head sc_accept_q; / Conn. waiting accept,
    pub /: *mut *mut rpcrdma_notification sc_rn; / removal notification,
    pub /: *mut *mut u32 sc_ord; / RDMA read limit,
    pub sc_max_send_sges: c_uint,
    pub /: *mut *mut bool sc_snd_w_inv; / OK to use Send With Invalidate,
    pub /: *mut *mut atomic_t sc_sq_avail; / SQEs ready to be consumed,
    pub /: *mut *mut unsigned int sc_sq_depth; / Depth of SQ,
    pub /: *mut *mut atomic_t sc_sq_ticket_head; / Next ticket to issue,
    pub /: *mut *mut atomic_t sc_sq_ticket_tail; / Ticket currently serving,
    pub /: *mut *mut wait_queue_head_t sc_sq_ticket_wait; / Ticket ordering waitlist,
    pub /: *mut *mut __be32 sc_fc_credits; / Forward credits,
    pub /: *mut *mut u32 sc_max_requests; / Max requests,
    pub /: *mut *mut u32 sc_max_bc_requests;/ Backward credits,
    pub /: *mut *mut int sc_max_req_size; / Size of each RQ WR buf,
    pub sc_port_num: u8,
    pub sc_pd: *mut ib_pd,
    pub sc_send_lock: spinlock_t,
    pub sc_send_ctxts: llist_head,
    pub sc_rw_ctxt_lock: spinlock_t,
    pub sc_rw_ctxts: llist_head,
    pub sc_pending_recvs: u32,
    pub sc_recv_batch: u32,
    pub sc_rq_dto_q: list_head,
    pub sc_read_complete_q: list_head,
    pub sc_rq_dto_lock: spinlock_t,
    pub sc_qp: *mut ib_qp,
    pub sc_rq_cq: *mut ib_cq,
    pub sc_sq_cq: *mut ib_cq,
    pub /: *mut *mut spinlock_t sc_lock; / transport lock,
    pub /: *mut *mut wait_queue_head_t sc_send_wait; / SQ exhaustion waitlist,
    pub sc_flags: c_ulong,
    pub sc_work: work_struct,
    pub sc_recv_ctxts: llist_head,
    pub sc_send_release_list: llist_head,
    pub sc_completion_ids: core::sync::atomic::AtomicI32,
}

// sc_flags
pub const RDMAXPRT_CONN_PENDING: c_int = 3;
extern "C" {
    pub fn container_of(_arg: xprt, svcxprt_rdma: struct, _arg: sc_xprt) -> return;
}
//
// Default connection parameters
//

//
// svc_rdma_send_cid_init - Initialize a Receive Queue completion ID
// @rdma: controlling transport
// @cid: completion ID to initialize
//
// svc_rdma_send_cid_init - Initialize a Send Queue completion ID
// @rdma: controlling transport
// @cid: completion ID to initialize
//
// A chunk context tracks all I/O for moving one Read or Write
// chunk. This is a set of rdma_rw's that handle data movement
// for all segments of one chunk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_rdma_chunk_ctxt {
    pub cc_cid: rpc_rdma_cid,
    pub cc_cqe: ib_cqe,
    pub cc_rwctxts: list_head,
    pub cc_posttime: ktime_t,
    pub cc_sqecount: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_rdma_recv_ctxt {
    pub rc_node: llist_node,
    pub rc_list: list_head,
    pub rc_recv_wr: ib_recv_wr,
    pub rc_cqe: ib_cqe,
    pub rc_cid: rpc_rdma_cid,
    pub rc_recv_sge: ib_sge,
    pub rc_recv_buf: *mut c_void,
    pub rc_stream: xdr_stream,
    pub rc_byte_len: u32,
    pub rc_inv_rkey: u32,
    pub rc_msgtype: __be32,
// State for pulling a Read chunk
    pub rc_pageoff: c_uint,
    pub rc_curpage: c_uint,
    pub rc_readbytes: c_uint,
    pub rc_saved_arg: xdr_buf,
    pub rc_cc: svc_rdma_chunk_ctxt,
    pub rc_call_pcl: svc_rdma_pcl,
    pub rc_read_pcl: svc_rdma_pcl,
    pub rc_cur_result_payload: *mut svc_rdma_chunk,
    pub rc_write_pcl: svc_rdma_pcl,
    pub rc_reply_pcl: svc_rdma_pcl,
    pub rc_page_count: c_uint,
    pub rc_maxpages: c_ulong,
    pub __counted_by(rc_maxpages): *mut *mut page rc_pages[],
}

//
// State for sending a Write chunk.
// - Tracks progress of writing one chunk over all its segments
// - Stores arguments for the SGL constructor functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_rdma_write_info {
    pub wi_rdma: *mut svcxprt_rdma,
    pub wi_list: list_head,
    pub wi_chunk: *const svc_rdma_chunk,
// write state of this chunk
    pub wi_seg_off: c_uint,
    pub wi_seg_no: c_uint,
// SGL constructor arguments
    pub wi_xdr: *const xdr_buf,
    pub wi_base: *mut c_uchar,
    pub wi_next_off: c_uint,
    pub wi_cc: svc_rdma_chunk_ctxt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_rdma_send_ctxt {
    pub sc_node: llist_node,
    pub sc_cid: rpc_rdma_cid,
    pub sc_rdma: *mut svcxprt_rdma,
    pub sc_send_wr: ib_send_wr,
    pub sc_wr_chain: *mut ib_send_wr,
    pub sc_sqecount: c_int,
    pub sc_cqe: ib_cqe,
    pub sc_hdrbuf: xdr_buf,
    pub sc_stream: xdr_stream,
    pub sc_write_info_list: list_head,
    pub sc_reply_info: svc_rdma_write_info,
    pub sc_xprt_buf: *mut c_void,
    pub sc_page_count: c_int,
    pub sc_cur_sge_no: c_int,
    pub sc_maxpages: c_ulong,
    pub sc_pages: *mut page,
    pub sc_sges: [ib_sge; ],
}

// svc_rdma_backchannel.c
// svc_rdma_recvfrom.c
extern "C" {
    pub fn svc_rdma_recv_ctxts_destroy(rdma: *mut svcxprt_rdma);
}
extern "C" {
    pub fn svc_rdma_post_recvs(rdma: *mut svcxprt_rdma) -> bool;
}
extern "C" {
    pub fn svc_rdma_flush_recv_queues(rdma: *mut svcxprt_rdma);
}
extern "C" {
    pub fn svc_rdma_release_ctxt(xprt: *mut svc_xprt, ctxt: *mut c_void);
}
extern "C" {
    pub fn svc_rdma_recvfrom(: *mut svc_rqst) -> c_int;
}
// svc_rdma_rw.c
extern "C" {
    pub fn svc_rdma_destroy_rw_ctxts(rdma: *mut svcxprt_rdma);
}
// svc_rdma_sendto.c
extern "C" {
    pub fn svc_rdma_send_ctxts_destroy(rdma: *mut svcxprt_rdma);
}
extern "C" {
    pub fn svc_rdma_send_ctxts_drain(rdma: *mut svcxprt_rdma);
}
extern "C" {
    pub fn svc_rdma_wake_send_waiters(rdma: *mut svcxprt_rdma, avail: c_int);
}
extern "C" {
    pub fn svc_rdma_sendto(: *mut svc_rqst) -> c_int;
}
// svc_rdma_transport.c
extern "C" {
    pub fn svc_rdma_xprt_deferred_close(rdma: *mut svcxprt_rdma);
}

// svc_rdma.c
extern "C" {
    pub fn svc_rdma_init() -> c_int;
}
extern "C" {
    pub fn svc_rdma_cleanup();
}
