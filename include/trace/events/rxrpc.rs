//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/rxrpc.h
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
// AF_RXRPC tracepoints
//
// Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Declare tracing information enums and their string mappings for display.
//

// AFS errors */						\
// rxperf errors */						\
// RxKAD security errors */					\
// RxGK security errors */					\
// rxrpc errors */						\

// ---- Must update size of stat_why_req_ack[] if more are added!

//
// Generate enums for tracing information.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_abort_reason {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_bundle_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_call_poke_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_call_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_client_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_congest_change {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_conn_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_local_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_peer_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_pmtud_reduce_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_propose_ack_outcome {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_propose_ack_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_receive_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_recvmsg_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_req_ack_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_rotate_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_rtt_rx_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_rtt_tx_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_sack_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_skb_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_timer_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_tlp_ack_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_tlp_probe_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_tq_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_tx_point {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_txbuf_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_txdata_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_txqueue_trace {

//
// Export enum symbols via userspace.
//

    rxrpc_abort_reasons;
    rxrpc_bundle_traces;
    rxrpc_ca_states;
    rxrpc_call_poke_traces;
    rxrpc_call_traces;
    rxrpc_client_traces;
    rxrpc_congest_changes;
    rxrpc_conn_traces;
    rxrpc_local_traces;
    rxrpc_pmtud_reduce_traces;
    rxrpc_propose_ack_traces;
    rxrpc_rack_timer_modes;
    rxrpc_receive_traces;
    rxrpc_recvmsg_traces;
    rxrpc_req_ack_traces;
    rxrpc_rotate_traces;
    rxrpc_rtt_rx_traces;
    rxrpc_rtt_tx_traces;
    rxrpc_sack_traces;
    rxrpc_skb_traces;
    rxrpc_timer_traces;
    rxrpc_tlp_ack_traces;
    rxrpc_tlp_probe_traces;
    rxrpc_tq_traces;
    rxrpc_tx_points;
    rxrpc_txbuf_traces;
    rxrpc_txdata_traces;
    rxrpc_txqueue_traces;

//
// Now redefine the EM() and E_() macros to map the enums to the strings that
// will be printed in the output.
//

    TRACE_EVENT(rxrpc_local,
    TP_PROTO(unsigned int local_debug_id, enum rxrpc_local_trace op,
    int ref, int usage),

    TP_ARGS(local_debug_id, op, ref, usage),

    TP_STRUCT__entry(
    __field(unsigned int,	local)
    __field(int,		op)
    __field(int,		ref)
    __field(int,		usage)
    ),

    TP_fast_assign(
    __entry->local = local_debug_id;
    __entry->op = op;
    __entry->ref = ref;
    __entry->usage = usage;
    ),

    TP_printk("L=%08x %s r=%d u=%d",
    __entry->local,
    __print_symbolic(__entry->op, rxrpc_local_traces),
    __entry->ref,
    __entry->usage)
    );

    TRACE_EVENT(rxrpc_iothread_rx,
    TP_PROTO(struct rxrpc_local *local, unsigned int nr_rx),
    TP_ARGS(local, nr_rx),
    TP_STRUCT__entry(
    __field(unsigned int,	local)
    __field(unsigned int,	nr_rx)
    ),
    TP_fast_assign(
    __entry->local = local->debug_id;
    __entry->nr_rx = nr_rx;
    ),
    TP_printk("L=%08x nrx=%u", __entry->local, __entry->nr_rx)
    );

    TRACE_EVENT(rxrpc_peer,
    TP_PROTO(unsigned int peer_debug_id, int ref, enum rxrpc_peer_trace why),

    TP_ARGS(peer_debug_id, ref, why),

    TP_STRUCT__entry(
    __field(unsigned int,	peer)
    __field(int,		ref)
    __field(enum rxrpc_peer_trace, why)
    ),

    TP_fast_assign(
    __entry->peer = peer_debug_id;
    __entry->ref = ref;
    __entry->why = why;
    ),

    TP_printk("P=%08x %s r=%d",
    __entry->peer,
    __print_symbolic(__entry->why, rxrpc_peer_traces),
    __entry->ref)
    );

    TRACE_EVENT(rxrpc_bundle,
    TP_PROTO(unsigned int bundle_debug_id, int ref, enum rxrpc_bundle_trace why),

    TP_ARGS(bundle_debug_id, ref, why),

    TP_STRUCT__entry(
    __field(unsigned int,	bundle)
    __field(int,		ref)
    __field(int,		why)
    ),

    TP_fast_assign(
    __entry->bundle = bundle_debug_id;
    __entry->ref = ref;
    __entry->why = why;
    ),

    TP_printk("CB=%08x %s r=%d",
    __entry->bundle,
    __print_symbolic(__entry->why, rxrpc_bundle_traces),
    __entry->ref)
    );

    TRACE_EVENT(rxrpc_conn,
    TP_PROTO(unsigned int conn_debug_id, int ref, enum rxrpc_conn_trace why),

    TP_ARGS(conn_debug_id, ref, why),

    TP_STRUCT__entry(
    __field(unsigned int,	conn)
    __field(int,		ref)
    __field(int,		why)
    ),

    TP_fast_assign(
    __entry->conn = conn_debug_id;
    __entry->ref = ref;
    __entry->why = why;
    ),

    TP_printk("C=%08x %s r=%d",
    __entry->conn,
    __print_symbolic(__entry->why, rxrpc_conn_traces),
    __entry->ref)
    );

    TRACE_EVENT(rxrpc_client,
    TP_PROTO(struct rxrpc_connection *conn, int channel,
    enum rxrpc_client_trace op),

    TP_ARGS(conn, channel, op),

    TP_STRUCT__entry(
    __field(unsigned int,		conn)
    __field(u32,			cid)
    __field(int,			channel)
    __field(int,			usage)
    __field(enum rxrpc_client_trace,	op)
    ),

    TP_fast_assign(
    __entry->conn = conn ? conn->debug_id : 0;
    __entry->channel = channel;
    __entry->usage = conn ? refcount_read(&conn->ref) : -2;
    __entry->op = op;
    __entry->cid = conn ? conn->proto.cid : 0;
    ),

    TP_printk("C=%08x h=%2d %s i=%08x u=%d",
    __entry->conn,
    __entry->channel,
    __print_symbolic(__entry->op, rxrpc_client_traces),
    __entry->cid,
    __entry->usage)
    );

    TRACE_EVENT(rxrpc_call,
    TP_PROTO(unsigned int call_debug_id, int ref, unsigned long aux,
    enum rxrpc_call_trace why),

    TP_ARGS(call_debug_id, ref, aux, why),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(int,		ref)
    __field(int,		why)
    __field(unsigned long,	aux)
    ),

    TP_fast_assign(
    __entry->call = call_debug_id;
    __entry->ref = ref;
    __entry->why = why;
    __entry->aux = aux;
    ),

    TP_printk("c=%08x %s r=%d a=%lx",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_call_traces),
    __entry->ref,
    __entry->aux)
    );

    TRACE_EVENT(rxrpc_skb,
    TP_PROTO(struct sk_buff *skb, int usage, int mod_count,
    enum rxrpc_skb_trace why),

    TP_ARGS(skb, usage, mod_count, why),

    TP_STRUCT__entry(
    __field(struct sk_buff *,		skb)
    __field(int,			usage)
    __field(int,			mod_count)
    __field(enum rxrpc_skb_trace,	why)
    ),

    TP_fast_assign(
    __entry->skb = skb;
    __entry->usage = usage;
    __entry->mod_count = mod_count;
    __entry->why = why;
    ),

    TP_printk("s=%p Rx %s u=%d m=%d",
    __entry->skb,
    __print_symbolic(__entry->why, rxrpc_skb_traces),
    __entry->usage,
    __entry->mod_count)
    );

    TRACE_EVENT(rxrpc_rx_packet,
    TP_PROTO(struct rxrpc_skb_priv *sp),

    TP_ARGS(sp),

    TP_STRUCT__entry(
    __field_struct(struct rxrpc_host_header,	hdr)
    ),

    TP_fast_assign(
    memcpy(&__entry->hdr, &sp->hdr, sizeof(__entry->hdr));
    ),

    TP_printk("%08x:%08x:%08x:%04x %08x %08x %02x %02x %s",
    __entry->hdr.epoch, __entry->hdr.cid,
    __entry->hdr.callNumber, __entry->hdr.serviceId,
    __entry->hdr.serial, __entry->hdr.seq,
    __entry->hdr.securityIndex, __entry->hdr.flags,
    __print_symbolic(__entry->hdr.type, rxrpc_pkts))
    );

    TRACE_EVENT(rxrpc_rx_done,
    TP_PROTO(int result, int abort_code),

    TP_ARGS(result, abort_code),

    TP_STRUCT__entry(
    __field(int,	result)
    __field(int,	abort_code)
    ),

    TP_fast_assign(
    __entry->result = result;
    __entry->abort_code = abort_code;
    ),

    TP_printk("r=%d a=%d", __entry->result, __entry->abort_code)
    );

    TRACE_EVENT(rxrpc_abort_call,
    TP_PROTO(const struct rxrpc_call *call, int abort_code),

    TP_ARGS(call, abort_code),

    TP_STRUCT__entry(
    __field(unsigned int,		call_nr)
    __field(enum rxrpc_abort_reason,	why)
    __field(int,			abort_code)
    __field(int,			error)
    ),

    TP_fast_assign(
    __entry->call_nr	= call->debug_id;
    __entry->why	= call->send_abort_why;
    __entry->abort_code	= abort_code;
    __entry->error	= call->send_abort_err;
    ),

    TP_printk("c=%08x a=%d e=%d %s",
    __entry->call_nr,
    __entry->abort_code, __entry->error,
    __print_symbolic(__entry->why, rxrpc_abort_reasons))
    );

    TRACE_EVENT(rxrpc_abort,
    TP_PROTO(unsigned int call_nr, enum rxrpc_abort_reason why,
    u32 cid, u32 call_id, rxrpc_seq_t seq, int abort_code, int error),

    TP_ARGS(call_nr, why, cid, call_id, seq, abort_code, error),

    TP_STRUCT__entry(
    __field(unsigned int,		call_nr)
    __field(enum rxrpc_abort_reason,	why)
    __field(u32,			cid)
    __field(u32,			call_id)
    __field(rxrpc_seq_t,		seq)
    __field(int,			abort_code)
    __field(int,			error)
    ),

    TP_fast_assign(
    __entry->call_nr = call_nr;
    __entry->why = why;
    __entry->cid = cid;
    __entry->call_id = call_id;
    __entry->abort_code = abort_code;
    __entry->error = error;
    __entry->seq = seq;
    ),

    TP_printk("c=%08x %08x:%08x s=%u a=%d e=%d %s",
    __entry->call_nr,
    __entry->cid, __entry->call_id, __entry->seq,
    __entry->abort_code, __entry->error,
    __print_symbolic(__entry->why, rxrpc_abort_reasons))
    );

    TRACE_EVENT(rxrpc_call_complete,
    TP_PROTO(struct rxrpc_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_call_completion,	compl)
    __field(int,			error)
    __field(u32,			abort_code)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->compl = call->completion;
    __entry->error = call->error;
    __entry->abort_code = call->abort_code;
    ),

    TP_printk("c=%08x %s r=%d ac=%d",
    __entry->call,
    __print_symbolic(__entry->compl, rxrpc_completions),
    __entry->error,
    __entry->abort_code)
    );

    TRACE_EVENT(rxrpc_txqueue,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_txqueue_trace why),

    TP_ARGS(call, why),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_txqueue_trace,	why)
    __field(rxrpc_seq_t,		tx_bottom)
    __field(rxrpc_seq_t,		acks_hard_ack)
    __field(rxrpc_seq_t,		tx_top)
    __field(rxrpc_seq_t,		send_top)
    __field(int,			tx_winsize)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->why = why;
    __entry->tx_bottom = call->tx_bottom;
    __entry->acks_hard_ack = call->acks_hard_ack;
    __entry->tx_top = call->tx_top;
    __entry->send_top = call->send_top;
    __entry->tx_winsize = call->tx_winsize;
    ),

    TP_printk("c=%08x %s b=%08x h=%08x n=%u/%u/%u/%u",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_txqueue_traces),
    __entry->tx_bottom,
    __entry->acks_hard_ack,
    __entry->acks_hard_ack - __entry->tx_bottom,
    __entry->tx_top - __entry->acks_hard_ack,
    __entry->send_top - __entry->tx_top,
    __entry->tx_winsize)
    );

    TRACE_EVENT(rxrpc_transmit,
    TP_PROTO(struct rxrpc_call *call, rxrpc_seq_t send_top, int space),

    TP_ARGS(call, send_top, space),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_seq_t,	seq)
    __field(u16,		space)
    __field(u16,		tx_winsize)
    __field(u16,		cong_cwnd)
    __field(u16,		cong_extra)
    __field(u16,		in_flight)
    __field(u16,		prepared)
    __field(u16,		pmtud_jumbo)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->seq	= call->tx_top + 1;
    __entry->space	= space;
    __entry->tx_winsize	= call->tx_winsize;
    __entry->cong_cwnd	= call->cong_cwnd;
    __entry->cong_extra	= call->cong_extra;
    __entry->prepared	= send_top - call->tx_bottom;
    __entry->in_flight	= call->tx_top - call->tx_bottom;
    __entry->pmtud_jumbo = call->peer->pmtud_jumbo;
    ),

    TP_printk("c=%08x q=%08x sp=%u tw=%u cw=%u+%u pr=%u if=%u pj=%u",
    __entry->call,
    __entry->seq,
    __entry->space,
    __entry->tx_winsize,
    __entry->cong_cwnd,
    __entry->cong_extra,
    __entry->prepared,
    __entry->in_flight,
    __entry->pmtud_jumbo)
    );

    TRACE_EVENT(rxrpc_tx_rotate,
    TP_PROTO(struct rxrpc_call *call, rxrpc_seq_t seq, rxrpc_seq_t to),

    TP_ARGS(call, seq, to),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_seq_t,	seq)
    __field(rxrpc_seq_t,	to)
    __field(rxrpc_seq_t,	top)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->seq	= seq;
    __entry->to		= to;
    __entry->top	= call->tx_top;
    ),

    TP_printk("c=%08x q=%08x-%08x-%08x",
    __entry->call,
    __entry->seq,
    __entry->to,
    __entry->top)
    );

    TRACE_EVENT(rxrpc_rx_data,
    TP_PROTO(unsigned int call, rxrpc_seq_t seq,
    rxrpc_serial_t serial, u8 flags),

    TP_ARGS(call, seq, serial, flags),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_seq_t,	seq)
    __field(rxrpc_serial_t,	serial)
    __field(u8,			flags)
    ),

    TP_fast_assign(
    __entry->call = call;
    __entry->seq = seq;
    __entry->serial = serial;
    __entry->flags = flags;
    ),

    TP_printk("c=%08x DATA %08x q=%08x fl=%02x",
    __entry->call,
    __entry->serial,
    __entry->seq,
    __entry->flags)
    );

    TRACE_EVENT(rxrpc_rx_ack,
    TP_PROTO(struct rxrpc_call *call, struct rxrpc_skb_priv *sp),

    TP_ARGS(call, sp),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_serial_t,	serial)
    __field(rxrpc_serial_t,	ack_serial)
    __field(rxrpc_seq_t,	first)
    __field(rxrpc_seq_t,	prev)
    __field(u8,			reason)
    __field(u8,			n_acks)
    __field(u8,			user_status)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->serial	= sp->hdr.serial;
    __entry->user_status = sp->hdr.userStatus;
    __entry->ack_serial = sp->ack.acked_serial;
    __entry->first	= sp->ack.first_ack;
    __entry->prev	= sp->ack.prev_ack;
    __entry->reason	= sp->ack.reason;
    __entry->n_acks	= sp->ack.nr_acks;
    ),

    TP_printk("c=%08x %08x %s r=%08x us=%02x f=%08x p=%08x n=%u",
    __entry->call,
    __entry->serial,
    __print_symbolic(__entry->reason, rxrpc_ack_names),
    __entry->ack_serial,
    __entry->user_status,
    __entry->first,
    __entry->prev,
    __entry->n_acks)
    );

    TRACE_EVENT(rxrpc_rx_abort,
    TP_PROTO(struct rxrpc_call *call, rxrpc_serial_t serial,
    u32 abort_code),

    TP_ARGS(call, serial, abort_code),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_serial_t,	serial)
    __field(u32,		abort_code)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->serial = serial;
    __entry->abort_code = abort_code;
    ),

    TP_printk("c=%08x ABORT %08x ac=%d",
    __entry->call,
    __entry->serial,
    __entry->abort_code)
    );

    TRACE_EVENT(rxrpc_rx_conn_abort,
    TP_PROTO(const struct rxrpc_connection *conn, const struct sk_buff *skb),

    TP_ARGS(conn, skb),

    TP_STRUCT__entry(
    __field(unsigned int,	conn)
    __field(rxrpc_serial_t,	serial)
    __field(u32,		abort_code)
    ),

    TP_fast_assign(
    __entry->conn = conn->debug_id;
    __entry->serial = rxrpc_skb(skb)->hdr.serial;
    __entry->abort_code = skb->priority;
    ),

    TP_printk("C=%08x ABORT %08x ac=%d",
    __entry->conn,
    __entry->serial,
    __entry->abort_code)
    );

    TRACE_EVENT(rxrpc_tx_challenge,
    TP_PROTO(struct rxrpc_connection *conn, rxrpc_serial_t serial,
    u32 version, u32 nonce),

    TP_ARGS(conn, serial, version, nonce),

    TP_STRUCT__entry(
    __field(unsigned int,	conn)
    __field(rxrpc_serial_t,	serial)
    __field(u32,		version)
    __field(u32,		nonce)
    __field(u16,		service_id)
    __field(u8,			security_ix)
    ),

    TP_fast_assign(
    __entry->conn = conn->debug_id;
    __entry->serial = serial;
    __entry->version = version;
    __entry->nonce = nonce;
    __entry->service_id = conn->service_id;
    __entry->security_ix = conn->security_ix;
    ),

    TP_printk("C=%08x CHALLENGE r=%08x sv=%u+%u v=%x n=%x",
    __entry->conn,
    __entry->serial,
    __entry->service_id,
    __entry->security_ix,
    __entry->version,
    __entry->nonce)
    );

    TRACE_EVENT(rxrpc_rx_challenge,
    TP_PROTO(struct rxrpc_connection *conn, rxrpc_serial_t serial,
    u32 version, u32 nonce, u32 min_level),

    TP_ARGS(conn, serial, version, nonce, min_level),

    TP_STRUCT__entry(
    __field(unsigned int,	conn)
    __field(rxrpc_serial_t,	serial)
    __field(u32,		version)
    __field(u32,		nonce)
    __field(u32,		min_level)
    __field(u16,		service_id)
    __field(u8,			security_ix)
    ),

    TP_fast_assign(
    __entry->conn = conn->debug_id;
    __entry->serial = serial;
    __entry->version = version;
    __entry->nonce = nonce;
    __entry->min_level = min_level;
    __entry->service_id = conn->service_id;
    __entry->security_ix = conn->security_ix;
    ),

    TP_printk("C=%08x CHALLENGE r=%08x sv=%u+%u v=%x n=%x ml=%x",
    __entry->conn,
    __entry->serial,
    __entry->service_id,
    __entry->security_ix,
    __entry->version,
    __entry->nonce,
    __entry->min_level)
    );

    TRACE_EVENT(rxrpc_tx_response,
    TP_PROTO(struct rxrpc_connection *conn, rxrpc_serial_t serial,
    struct rxrpc_skb_priv *rsp),

    TP_ARGS(conn, serial, rsp),

    TP_STRUCT__entry(
    __field(unsigned int,	conn)
    __field(rxrpc_serial_t,	serial)
    __field(rxrpc_serial_t,	challenge)
    __field(u32,		version)
    __field(u32,		kvno)
    __field(u16,		ticket_len)
    __field(u16,		appdata_len)
    __field(u16,		service_id)
    __field(u8,			security_ix)
    ),

    TP_fast_assign(
    __entry->conn	= conn->debug_id;
    __entry->serial	= serial;
    __entry->challenge	= rsp->resp.challenge_serial;
    __entry->version	= rsp->resp.version;
    __entry->kvno	= rsp->resp.kvno;
    __entry->ticket_len = rsp->resp.ticket_len;
    __entry->service_id = conn->service_id;
    __entry->security_ix = conn->security_ix;
    ),

    TP_printk("C=%08x RESPONSE r=%08x cr=%08x sv=%u+%u v=%x kv=%x tl=%u",
    __entry->conn,
    __entry->serial,
    __entry->challenge,
    __entry->service_id,
    __entry->security_ix,
    __entry->version,
    __entry->kvno,
    __entry->ticket_len)
    );

    TRACE_EVENT(rxrpc_rx_response,
    TP_PROTO(struct rxrpc_connection *conn, rxrpc_serial_t serial,
    u32 version, u32 kvno, u32 ticket_len),

    TP_ARGS(conn, serial, version, kvno, ticket_len),

    TP_STRUCT__entry(
    __field(unsigned int,	conn)
    __field(rxrpc_serial_t,	serial)
    __field(u32,		version)
    __field(u32,		kvno)
    __field(u32,		ticket_len)
    __field(u8,			security_ix)
    ),

    TP_fast_assign(
    __entry->conn = conn->debug_id;
    __entry->serial = serial;
    __entry->version = version;
    __entry->kvno = kvno;
    __entry->ticket_len = ticket_len;
    __entry->security_ix = conn->security_ix;
    ),

    TP_printk("C=%08x RESPONSE r=%08x sx=%u v=%x kvno=%x tl=%x",
    __entry->conn,
    __entry->serial,
    __entry->security_ix,
    __entry->version,
    __entry->kvno,
    __entry->ticket_len)
    );

    TRACE_EVENT(rxrpc_rx_rwind_change,
    TP_PROTO(struct rxrpc_call *call, rxrpc_serial_t serial,
    u32 rwind, bool wake),

    TP_ARGS(call, serial, rwind, wake),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_serial_t,	serial)
    __field(u32,		rwind)
    __field(bool,		wake)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->serial = serial;
    __entry->rwind = rwind;
    __entry->wake = wake;
    ),

    TP_printk("c=%08x %08x rw=%u%s",
    __entry->call,
    __entry->serial,
    __entry->rwind,
    __entry->wake ? " wake" : "")
    );

    TRACE_EVENT(rxrpc_tx_packet,
    TP_PROTO(unsigned int call_id, struct rxrpc_wire_header *whdr,
    enum rxrpc_tx_point where),

    TP_ARGS(call_id, whdr, where),

    TP_STRUCT__entry(
    __field(unsigned int,			call)
    __field(enum rxrpc_tx_point,		where)
    __field_struct(struct rxrpc_wire_header,	whdr)
    ),

    TP_fast_assign(
    __entry->call = call_id;
    memcpy(&__entry->whdr, whdr, sizeof(__entry->whdr));
    __entry->where = where;
    ),

    TP_printk("c=%08x %08x:%08x:%08x:%04x %08x %08x %02x %02x %s %s",
    __entry->call,
    ntohl(__entry->whdr.epoch),
    ntohl(__entry->whdr.cid),
    ntohl(__entry->whdr.callNumber),
    ntohs(__entry->whdr.serviceId),
    ntohl(__entry->whdr.serial),
    ntohl(__entry->whdr.seq),
    __entry->whdr.type, __entry->whdr.flags,
    __entry->whdr.type <= 15 ?
    __print_symbolic(__entry->whdr.type, rxrpc_pkts) : "?UNK",
    __print_symbolic(__entry->where, rxrpc_tx_points))
    );

    TRACE_EVENT(rxrpc_tx_data,
    TP_PROTO(struct rxrpc_call *call, rxrpc_seq_t seq,
    rxrpc_serial_t serial, unsigned int flags,
    enum rxrpc_txdata_trace trace),

    TP_ARGS(call, seq, serial, flags, trace),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_seq_t,	seq)
    __field(rxrpc_serial_t,	serial)
    __field(u32,		cid)
    __field(u32,		call_id)
    __field(u16,		flags)
    __field(enum rxrpc_txdata_trace, trace)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->cid = call->cid;
    __entry->call_id = call->call_id;
    __entry->seq = seq;
    __entry->serial = serial;
    __entry->flags = flags;
    __entry->trace = trace;
    ),

    TP_printk("c=%08x DATA %08x:%08x %08x q=%08x fl=%02x%s",
    __entry->call,
    __entry->cid,
    __entry->call_id,
    __entry->serial,
    __entry->seq,
    __entry->flags & RXRPC_TXBUF_WIRE_FLAGS,
    __print_symbolic(__entry->trace, rxrpc_txdata_traces))
    );

    TRACE_EVENT(rxrpc_tx_ack,
    TP_PROTO(unsigned int call, rxrpc_serial_t serial,
    rxrpc_seq_t ack_first, rxrpc_serial_t ack_serial,
    u8 reason, u8 n_acks, u16 rwind,
    enum rxrpc_propose_ack_trace trace),

    TP_ARGS(call, serial, ack_first, ack_serial, reason, n_acks, rwind, trace),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_serial_t,	serial)
    __field(rxrpc_seq_t,	ack_first)
    __field(rxrpc_serial_t,	ack_serial)
    __field(u8,			reason)
    __field(u8,			n_acks)
    __field(u16,		rwind)
    __field(enum rxrpc_propose_ack_trace, trace)
    ),

    TP_fast_assign(
    __entry->call = call;
    __entry->serial = serial;
    __entry->ack_first = ack_first;
    __entry->ack_serial = ack_serial;
    __entry->reason = reason;
    __entry->n_acks = n_acks;
    __entry->rwind = rwind;
    __entry->trace = trace;
    ),

    TP_printk(" c=%08x ACK  %08x %s f=%08x r=%08x n=%u rw=%u %s",
    __entry->call,
    __entry->serial,
    __print_symbolic(__entry->reason, rxrpc_ack_names),
    __entry->ack_first,
    __entry->ack_serial,
    __entry->n_acks,
    __entry->rwind,
    __print_symbolic(__entry->trace, rxrpc_propose_ack_traces))
    );

    TRACE_EVENT(rxrpc_receive,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_receive_trace why,
    rxrpc_serial_t serial, rxrpc_seq_t seq),

    TP_ARGS(call, why, serial, seq),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_receive_trace,	why)
    __field(rxrpc_serial_t,		serial)
    __field(rxrpc_seq_t,		seq)
    __field(rxrpc_seq_t,		window)
    __field(rxrpc_seq_t,		wtop)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->why = why;
    __entry->serial = serial;
    __entry->seq = seq;
    __entry->window = call->ackr_window;
    __entry->wtop = call->ackr_wtop;
    ),

    TP_printk("c=%08x %s r=%08x q=%08x w=%08x-%08x",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_receive_traces),
    __entry->serial,
    __entry->seq,
    __entry->window,
    __entry->wtop)
    );

    TRACE_EVENT(rxrpc_recvmsg,
    TP_PROTO(unsigned int call_debug_id, enum rxrpc_recvmsg_trace why,
    int ret),

    TP_ARGS(call_debug_id, why, ret),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_recvmsg_trace,	why)
    __field(int,			ret)
    ),

    TP_fast_assign(
    __entry->call = call_debug_id;
    __entry->why = why;
    __entry->ret = ret;
    ),

    TP_printk("c=%08x %s ret=%d",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_recvmsg_traces),
    __entry->ret)
    );

    TRACE_EVENT(rxrpc_recvdata,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_recvmsg_trace why,
    rxrpc_seq_t seq, unsigned int offset, unsigned int len,
    int ret),

    TP_ARGS(call, why, seq, offset, len, ret),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_recvmsg_trace,	why)
    __field(rxrpc_seq_t,		seq)
    __field(unsigned int,		offset)
    __field(unsigned int,		len)
    __field(int,			ret)
    ),

    TP_fast_assign(
    __entry->call = call ? call->debug_id : 0;
    __entry->why = why;
    __entry->seq = seq;
    __entry->offset = offset;
    __entry->len = len;
    __entry->ret = ret;
    ),

    TP_printk("c=%08x %s q=%08x o=%u l=%u ret=%d",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_recvmsg_traces),
    __entry->seq,
    __entry->offset,
    __entry->len,
    __entry->ret)
    );

    TRACE_EVENT(rxrpc_rtt_tx,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_rtt_tx_trace why,
    int slot, rxrpc_serial_t send_serial),

    TP_ARGS(call, why, slot, send_serial),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_rtt_tx_trace,	why)
    __field(int,			slot)
    __field(rxrpc_serial_t,		send_serial)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->why = why;
    __entry->slot = slot;
    __entry->send_serial = send_serial;
    ),

    TP_printk("c=%08x [%d] %s sr=%08x",
    __entry->call,
    __entry->slot,
    __print_symbolic(__entry->why, rxrpc_rtt_tx_traces),
    __entry->send_serial)
    );

    TRACE_EVENT(rxrpc_rtt_rx,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_rtt_rx_trace why,
    int slot,
    rxrpc_serial_t send_serial, rxrpc_serial_t resp_serial,
    u32 rtt, u32 srtt, u32 rto),

    TP_ARGS(call, why, slot, send_serial, resp_serial, rtt, srtt, rto),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_rtt_rx_trace,	why)
    __field(int,			slot)
    __field(rxrpc_serial_t,		send_serial)
    __field(rxrpc_serial_t,		resp_serial)
    __field(u32,			rtt)
    __field(u32,			srtt)
    __field(u32,			rto)
    __field(u32,			min_rtt)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->why = why;
    __entry->slot = slot;
    __entry->send_serial = send_serial;
    __entry->resp_serial = resp_serial;
    __entry->rtt = rtt;
    __entry->srtt = srtt;
    __entry->rto = rto;
    __entry->min_rtt = minmax_get(&call->min_rtt)
    ),

    TP_printk("c=%08x [%d] %s sr=%08x rr=%08x rtt=%u srtt=%u rto=%u min=%u",
    __entry->call,
    __entry->slot,
    __print_symbolic(__entry->why, rxrpc_rtt_rx_traces),
    __entry->send_serial,
    __entry->resp_serial,
    __entry->rtt,
    __entry->srtt / 8,
    __entry->rto,
    __entry->min_rtt)
    );

    TRACE_EVENT(rxrpc_timer_set,
    TP_PROTO(struct rxrpc_call *call, ktime_t delay,
    enum rxrpc_timer_trace why),

    TP_ARGS(call, delay, why),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_timer_trace,	why)
    __field(ktime_t,			delay)
    ),

    TP_fast_assign(
    __entry->call		= call->debug_id;
    __entry->why		= why;
    __entry->delay		= delay;
    ),

    TP_printk("c=%08x %s to=%lld",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_timer_traces),
    ktime_to_us(__entry->delay))
    );

    TRACE_EVENT(rxrpc_timer_exp,
    TP_PROTO(struct rxrpc_call *call, ktime_t delay,
    enum rxrpc_timer_trace why),

    TP_ARGS(call, delay, why),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_timer_trace,	why)
    __field(ktime_t,			delay)
    ),

    TP_fast_assign(
    __entry->call		= call->debug_id;
    __entry->why		= why;
    __entry->delay		= delay;
    ),

    TP_printk("c=%08x %s to=%lld",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_timer_traces),
    ktime_to_us(__entry->delay))
    );

    TRACE_EVENT(rxrpc_timer_can,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_timer_trace why),

    TP_ARGS(call, why),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_timer_trace,	why)
    ),

    TP_fast_assign(
    __entry->call		= call->debug_id;
    __entry->why		= why;
    ),

    TP_printk("c=%08x %s",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_timer_traces))
    );

    TRACE_EVENT(rxrpc_timer_restart,
    TP_PROTO(struct rxrpc_call *call, ktime_t delay, unsigned long delayj),

    TP_ARGS(call, delay, delayj),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(unsigned long,		delayj)
    __field(ktime_t,			delay)
    ),

    TP_fast_assign(
    __entry->call		= call->debug_id;
    __entry->delayj		= delayj;
    __entry->delay		= delay;
    ),

    TP_printk("c=%08x to=%lld j=%ld",
    __entry->call,
    ktime_to_us(__entry->delay),
    __entry->delayj)
    );

    TRACE_EVENT(rxrpc_timer_expired,
    TP_PROTO(struct rxrpc_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    ),

    TP_fast_assign(
    __entry->call		= call->debug_id;
    ),

    TP_printk("c=%08x EXPIRED",
    __entry->call)
    );

    TRACE_EVENT(rxrpc_rx_lose,
    TP_PROTO(struct rxrpc_skb_priv *sp),

    TP_ARGS(sp),

    TP_STRUCT__entry(
    __field_struct(struct rxrpc_host_header,	hdr)
    ),

    TP_fast_assign(
    memcpy(&__entry->hdr, &sp->hdr, sizeof(__entry->hdr));
    ),

    TP_printk("%08x:%08x:%08x:%04x %08x %08x %02x %02x %s *LOSE*",
    __entry->hdr.epoch, __entry->hdr.cid,
    __entry->hdr.callNumber, __entry->hdr.serviceId,
    __entry->hdr.serial, __entry->hdr.seq,
    __entry->hdr.type, __entry->hdr.flags,
    __entry->hdr.type <= 15 ?
    __print_symbolic(__entry->hdr.type, rxrpc_pkts) : "?UNK")
    );

    TRACE_EVENT(rxrpc_propose_ack,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_propose_ack_trace why,
    u8 ack_reason, rxrpc_serial_t serial),

    TP_ARGS(call, why, ack_reason, serial),

    TP_STRUCT__entry(
    __field(unsigned int,			call)
    __field(enum rxrpc_propose_ack_trace,	why)
    __field(rxrpc_serial_t,			serial)
    __field(u8,					ack_reason)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->why	= why;
    __entry->serial	= serial;
    __entry->ack_reason	= ack_reason;
    ),

    TP_printk("c=%08x %s %s r=%08x",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_propose_ack_traces),
    __print_symbolic(__entry->ack_reason, rxrpc_ack_names),
    __entry->serial)
    );

    TRACE_EVENT(rxrpc_send_ack,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_propose_ack_trace why,
    u8 ack_reason, rxrpc_serial_t serial),

    TP_ARGS(call, why, ack_reason, serial),

    TP_STRUCT__entry(
    __field(unsigned int,			call)
    __field(enum rxrpc_propose_ack_trace,	why)
    __field(rxrpc_serial_t,			serial)
    __field(u8,					ack_reason)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->why	= why;
    __entry->serial	= serial;
    __entry->ack_reason	= ack_reason;
    ),

    TP_printk("c=%08x %s %s r=%08x",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_propose_ack_traces),
    __print_symbolic(__entry->ack_reason, rxrpc_ack_names),
    __entry->serial)
    );

    TRACE_EVENT(rxrpc_drop_ack,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_propose_ack_trace why,
    u8 ack_reason, rxrpc_serial_t serial, bool nobuf),

    TP_ARGS(call, why, ack_reason, serial, nobuf),

    TP_STRUCT__entry(
    __field(unsigned int,			call)
    __field(enum rxrpc_propose_ack_trace,	why)
    __field(rxrpc_serial_t,			serial)
    __field(u8,					ack_reason)
    __field(bool,				nobuf)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->why	= why;
    __entry->serial	= serial;
    __entry->ack_reason	= ack_reason;
    __entry->nobuf	= nobuf;
    ),

    TP_printk("c=%08x %s %s r=%08x nbf=%u",
    __entry->call,
    __print_symbolic(__entry->why, rxrpc_propose_ack_traces),
    __print_symbolic(__entry->ack_reason, rxrpc_ack_names),
    __entry->serial, __entry->nobuf)
    );

    TRACE_EVENT(rxrpc_retransmit,
    TP_PROTO(struct rxrpc_call *call,
    struct rxrpc_send_data_req *req,
    struct rxrpc_txbuf *txb),

    TP_ARGS(call, req, txb),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(unsigned int,	qbase)
    __field(rxrpc_seq_t,	seq)
    __field(rxrpc_serial_t,	serial)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->qbase = req->tq->qbase;
    __entry->seq = req->seq;
    __entry->serial = txb->serial;
    ),

    TP_printk("c=%08x tq=%x q=%x r=%x",
    __entry->call,
    __entry->qbase,
    __entry->seq,
    __entry->serial)
    );

    TRACE_EVENT(rxrpc_congest,
    TP_PROTO(struct rxrpc_call *call, struct rxrpc_ack_summary *summary),

    TP_ARGS(call, summary),

    TP_STRUCT__entry(
    __field(unsigned int,			call)
    __field(enum rxrpc_ca_state,		ca_state)
    __field(rxrpc_seq_t,			hard_ack)
    __field(rxrpc_seq_t,			top)
    __field(rxrpc_seq_t,			lowest_nak)
    __field(u16,				nr_sacks)
    __field(u16,				nr_snacks)
    __field(u16,				cwnd)
    __field(u16,				ssthresh)
    __field(u16,				cumul_acks)
    __field(u16,				dup_acks)
    __field_struct(struct rxrpc_ack_summary,	sum)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->ca_state	= call->cong_ca_state;
    __entry->hard_ack	= call->acks_hard_ack;
    __entry->top	= call->tx_top;
    __entry->lowest_nak	= call->acks_lowest_nak;
    __entry->nr_sacks	= call->acks_nr_sacks;
    __entry->nr_snacks	= call->acks_nr_snacks;
    __entry->cwnd	= call->cong_cwnd;
    __entry->ssthresh	= call->cong_ssthresh;
    __entry->cumul_acks	= call->cong_cumul_acks;
    __entry->dup_acks	= call->cong_dup_acks;
    memcpy(&__entry->sum, summary, sizeof(__entry->sum));
    ),

    TP_printk("c=%08x r=%08x %s q=%08x %s cw=%u ss=%u A=%u+%u/%u+%u r=%u b=%u u=%u d=%u l=%x%s%s%s",
    __entry->call,
    __entry->sum.acked_serial,
    __print_symbolic(__entry->sum.ack_reason, rxrpc_ack_names),
    __entry->hard_ack,
    __print_symbolic(__entry->ca_state, rxrpc_ca_states),
    __entry->cwnd,
    __entry->ssthresh,
    __entry->nr_sacks, __entry->sum.nr_new_sacks,
    __entry->nr_snacks, __entry->sum.nr_new_snacks,
    __entry->sum.nr_new_hacks,
    __entry->top - __entry->hard_ack,
    __entry->cumul_acks,
    __entry->dup_acks,
    __entry->lowest_nak, __entry->sum.new_low_snack ? "!" : "",
    __print_symbolic(__entry->sum.change, rxrpc_congest_changes),
    __entry->sum.retrans_timeo ? " rTxTo" : "")
    );

    TRACE_EVENT(rxrpc_reset_cwnd,
    TP_PROTO(struct rxrpc_call *call, ktime_t since_last_tx, ktime_t rtt),

    TP_ARGS(call, since_last_tx, rtt),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum rxrpc_ca_state,	ca_state)
    __field(unsigned short,		cwnd)
    __field(unsigned short,		extra)
    __field(rxrpc_seq_t,		hard_ack)
    __field(rxrpc_seq_t,		prepared)
    __field(ktime_t,			since_last_tx)
    __field(ktime_t,			rtt)
    __field(bool,			has_data)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->ca_state	= call->cong_ca_state;
    __entry->cwnd	= call->cong_cwnd;
    __entry->extra	= call->cong_extra;
    __entry->hard_ack	= call->acks_hard_ack;
    __entry->prepared	= call->send_top - call->tx_bottom;
    __entry->since_last_tx = since_last_tx;
    __entry->rtt	= rtt;
    __entry->has_data	= call->tx_bottom != call->tx_top;
    ),

    TP_printk("c=%08x q=%08x %s cw=%u+%u pr=%u tm=%llu/%llu d=%u",
    __entry->call,
    __entry->hard_ack,
    __print_symbolic(__entry->ca_state, rxrpc_ca_states),
    __entry->cwnd,
    __entry->extra,
    __entry->prepared,
    ktime_to_us(__entry->since_last_tx),
    ktime_to_us(__entry->rtt),
    __entry->has_data)
    );

    TRACE_EVENT(rxrpc_disconnect_call,
    TP_PROTO(struct rxrpc_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(u32,		abort_code)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->abort_code = call->abort_code;
    ),

    TP_printk("c=%08x ab=%08x",
    __entry->call,
    __entry->abort_code)
    );

    TRACE_EVENT(rxrpc_improper_term,
    TP_PROTO(struct rxrpc_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(u32,		abort_code)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->abort_code = call->abort_code;
    ),

    TP_printk("c=%08x ab=%08x",
    __entry->call,
    __entry->abort_code)
    );

    TRACE_EVENT(rxrpc_connect_call,
    TP_PROTO(struct rxrpc_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(unsigned long,		user_call_ID)
    __field(u32,			cid)
    __field(u32,			call_id)
    __field_struct(struct sockaddr_rxrpc, srx)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->user_call_ID = call->user_call_ID;
    __entry->cid = call->cid;
    __entry->call_id = call->call_id;
    __entry->srx = call->dest_srx;
    ),

    TP_printk("c=%08x u=%p %08x:%08x dst=%pISp",
    __entry->call,
    (void *)__entry->user_call_ID,
    __entry->cid,
    __entry->call_id,
    &__entry->srx.transport)
    );

    TRACE_EVENT(rxrpc_apply_acks,
    TP_PROTO(struct rxrpc_call *call, struct rxrpc_txqueue *tq),

    TP_ARGS(call, tq),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(unsigned int,	nr_rep)
    __field(rxrpc_seq_t,	qbase)
    __field(unsigned long,	acks)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->qbase = tq->qbase;
    __entry->acks = tq->segment_acked;
    __entry->nr_rep = tq->nr_reported_acks;
    ),

    TP_printk("c=%08x tq=%x acks=%016lx rep=%u",
    __entry->call,
    __entry->qbase,
    __entry->acks,
    __entry->nr_rep)
    );

    TRACE_EVENT(rxrpc_resend,
    TP_PROTO(struct rxrpc_call *call, rxrpc_serial_t ack_serial),

    TP_ARGS(call, ack_serial),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_seq_t,	seq)
    __field(rxrpc_seq_t,	transmitted)
    __field(rxrpc_serial_t,	ack_serial)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->seq = call->acks_hard_ack;
    __entry->transmitted = call->tx_transmitted;
    __entry->ack_serial = ack_serial;
    ),

    TP_printk("c=%08x r=%x q=%x tq=%x",
    __entry->call,
    __entry->ack_serial,
    __entry->seq,
    __entry->transmitted)
    );

    TRACE_EVENT(rxrpc_resend_lost,
    TP_PROTO(struct rxrpc_call *call, struct rxrpc_txqueue *tq, unsigned long lost),

    TP_ARGS(call, tq, lost),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_seq_t,	qbase)
    __field(u8,			nr_rep)
    __field(unsigned long,	lost)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->qbase = tq->qbase;
    __entry->nr_rep = tq->nr_reported_acks;
    __entry->lost = lost;
    ),

    TP_printk("c=%08x tq=%x lost=%016lx nr=%u",
    __entry->call,
    __entry->qbase,
    __entry->lost,
    __entry->nr_rep)
    );

    TRACE_EVENT(rxrpc_rotate,
    TP_PROTO(struct rxrpc_call *call, struct rxrpc_txqueue *tq,
    struct rxrpc_ack_summary *summary, rxrpc_seq_t seq,
    enum rxrpc_rotate_trace trace),

    TP_ARGS(call, tq, summary, seq, trace),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_seq_t,	qbase)
    __field(rxrpc_seq_t,	seq)
    __field(unsigned int,	nr_rep)
    __field(enum rxrpc_rotate_trace, trace)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->qbase = tq->qbase;
    __entry->seq = seq;
    __entry->nr_rep = tq->nr_reported_acks;
    __entry->trace = trace;
    ),

    TP_printk("c=%08x tq=%x q=%x nr=%x %s",
    __entry->call,
    __entry->qbase,
    __entry->seq,
    __entry->nr_rep,
    __print_symbolic(__entry->trace, rxrpc_rotate_traces))
    );

    TRACE_EVENT(rxrpc_rx_icmp,
    TP_PROTO(struct rxrpc_peer *peer, struct sock_extended_err *ee,
    struct sockaddr_rxrpc *srx),

    TP_ARGS(peer, ee, srx),

    TP_STRUCT__entry(
    __field(unsigned int,			peer)
    __field_struct(struct sock_extended_err,	ee)
    __field_struct(struct sockaddr_rxrpc,	srx)
    ),

    TP_fast_assign(
    __entry->peer = peer->debug_id;
    memcpy(&__entry->ee, ee, sizeof(__entry->ee));
    memcpy(&__entry->srx, srx, sizeof(__entry->srx));
    ),

    TP_printk("P=%08x o=%u t=%u c=%u i=%u d=%u e=%d %pISp",
    __entry->peer,
    __entry->ee.ee_origin,
    __entry->ee.ee_type,
    __entry->ee.ee_code,
    __entry->ee.ee_info,
    __entry->ee.ee_data,
    __entry->ee.ee_errno,
    &__entry->srx.transport)
    );

    TRACE_EVENT(rxrpc_tx_fail,
    TP_PROTO(unsigned int debug_id, rxrpc_serial_t serial, int ret,
    enum rxrpc_tx_point where),

    TP_ARGS(debug_id, serial, ret, where),

    TP_STRUCT__entry(
    __field(unsigned int,		debug_id)
    __field(rxrpc_serial_t,		serial)
    __field(int,			ret)
    __field(enum rxrpc_tx_point,	where)
    ),

    TP_fast_assign(
    __entry->debug_id = debug_id;
    __entry->serial = serial;
    __entry->ret = ret;
    __entry->where = where;
    ),

    TP_printk("c=%08x r=%x ret=%d %s",
    __entry->debug_id,
    __entry->serial,
    __entry->ret,
    __print_symbolic(__entry->where, rxrpc_tx_points))
    );

    TRACE_EVENT(rxrpc_call_reset,
    TP_PROTO(struct rxrpc_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,	debug_id)
    __field(u32,		cid)
    __field(u32,		call_id)
    __field(rxrpc_serial_t,	call_serial)
    __field(rxrpc_serial_t,	conn_serial)
    __field(rxrpc_seq_t,	tx_seq)
    __field(rxrpc_seq_t,	rx_seq)
    ),

    TP_fast_assign(
    __entry->debug_id = call->debug_id;
    __entry->cid = call->cid;
    __entry->call_id = call->call_id;
    __entry->call_serial = call->rx_serial;
    __entry->conn_serial = call->conn->hi_serial;
    __entry->tx_seq = call->acks_hard_ack;
    __entry->rx_seq = call->rx_highest_seq;
    ),

    TP_printk("c=%08x %08x:%08x r=%08x/%08x tx=%08x rx=%08x",
    __entry->debug_id,
    __entry->cid, __entry->call_id,
    __entry->call_serial, __entry->conn_serial,
    __entry->tx_seq, __entry->rx_seq)
    );

    TRACE_EVENT(rxrpc_notify_socket,
    TP_PROTO(unsigned int debug_id, rxrpc_serial_t serial),

    TP_ARGS(debug_id, serial),

    TP_STRUCT__entry(
    __field(unsigned int,	debug_id)
    __field(rxrpc_serial_t,	serial)
    ),

    TP_fast_assign(
    __entry->debug_id = debug_id;
    __entry->serial = serial;
    ),

    TP_printk("c=%08x r=%08x",
    __entry->debug_id,
    __entry->serial)
    );

    TRACE_EVENT(rxrpc_rx_discard_ack,
    TP_PROTO(struct rxrpc_call *call, rxrpc_serial_t serial,
    rxrpc_seq_t hard_ack, rxrpc_seq_t prev_pkt),

    TP_ARGS(call, serial, hard_ack, prev_pkt),

    TP_STRUCT__entry(
    __field(unsigned int,	debug_id)
    __field(rxrpc_serial_t,	serial)
    __field(rxrpc_seq_t,	hard_ack)
    __field(rxrpc_seq_t,	prev_pkt)
    __field(rxrpc_seq_t,	acks_hard_ack)
    __field(rxrpc_seq_t,	acks_prev_seq)
    ),

    TP_fast_assign(
    __entry->debug_id		= call->debug_id;
    __entry->serial		= serial;
    __entry->hard_ack		= hard_ack;
    __entry->prev_pkt		= prev_pkt;
    __entry->acks_hard_ack	= call->acks_hard_ack;
    __entry->acks_prev_seq	= call->acks_prev_seq;
    ),

    TP_printk("c=%08x r=%08x %08x<%08x %08x<%08x",
    __entry->debug_id,
    __entry->serial,
    __entry->hard_ack,
    __entry->acks_hard_ack,
    __entry->prev_pkt,
    __entry->acks_prev_seq)
    );

    TRACE_EVENT(rxrpc_req_ack,
    TP_PROTO(unsigned int call_debug_id, rxrpc_seq_t seq,
    enum rxrpc_req_ack_trace why),

    TP_ARGS(call_debug_id, seq, why),

    TP_STRUCT__entry(
    __field(unsigned int,		call_debug_id)
    __field(rxrpc_seq_t,		seq)
    __field(enum rxrpc_req_ack_trace,	why)
    ),

    TP_fast_assign(
    __entry->call_debug_id = call_debug_id;
    __entry->seq = seq;
    __entry->why = why;
    ),

    TP_printk("c=%08x q=%08x REQ-%s",
    __entry->call_debug_id,
    __entry->seq,
    __print_symbolic(__entry->why, rxrpc_req_ack_traces))
    );

    TRACE_EVENT(rxrpc_txbuf,
    TP_PROTO(unsigned int debug_id,
    unsigned int call_debug_id, rxrpc_seq_t seq,
    int ref, enum rxrpc_txbuf_trace what),

    TP_ARGS(debug_id, call_debug_id, seq, ref, what),

    TP_STRUCT__entry(
    __field(unsigned int,		debug_id)
    __field(unsigned int,		call_debug_id)
    __field(rxrpc_seq_t,		seq)
    __field(int,			ref)
    __field(enum rxrpc_txbuf_trace,	what)
    ),

    TP_fast_assign(
    __entry->debug_id = debug_id;
    __entry->call_debug_id = call_debug_id;
    __entry->seq = seq;
    __entry->ref = ref;
    __entry->what = what;
    ),

    TP_printk("B=%08x c=%08x q=%08x %s r=%d",
    __entry->debug_id,
    __entry->call_debug_id,
    __entry->seq,
    __print_symbolic(__entry->what, rxrpc_txbuf_traces),
    __entry->ref)
    );

    TRACE_EVENT(rxrpc_tq,
    TP_PROTO(struct rxrpc_call *call, struct rxrpc_txqueue *tq,
    rxrpc_seq_t seq, enum rxrpc_tq_trace trace),

    TP_ARGS(call, tq, seq, trace),

    TP_STRUCT__entry(
    __field(unsigned int,		call_debug_id)
    __field(rxrpc_seq_t,		qbase)
    __field(rxrpc_seq_t,		seq)
    __field(enum rxrpc_tq_trace,	trace)
    ),

    TP_fast_assign(
    __entry->call_debug_id = call->debug_id;
    __entry->qbase = tq ? tq->qbase : call->tx_qbase;
    __entry->seq = seq;
    __entry->trace = trace;
    ),

    TP_printk("c=%08x bq=%08x q=%08x %s",
    __entry->call_debug_id,
    __entry->qbase,
    __entry->seq,
    __print_symbolic(__entry->trace, rxrpc_tq_traces))
    );

    TRACE_EVENT(rxrpc_poke_call,
    TP_PROTO(struct rxrpc_call *call, bool busy,
    enum rxrpc_call_poke_trace what),

    TP_ARGS(call, busy, what),

    TP_STRUCT__entry(
    __field(unsigned int,		call_debug_id)
    __field(bool,			busy)
    __field(enum rxrpc_call_poke_trace,	what)
    ),

    TP_fast_assign(
    __entry->call_debug_id = call->debug_id;
    __entry->busy = busy;
    __entry->what = what;
    ),

    TP_printk("c=%08x %s%s",
    __entry->call_debug_id,
    __print_symbolic(__entry->what, rxrpc_call_poke_traces),
    __entry->busy ? "!" : "")
    );

    TRACE_EVENT(rxrpc_call_poked,
    TP_PROTO(struct rxrpc_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,	call_debug_id)
    ),

    TP_fast_assign(
    __entry->call_debug_id = call->debug_id;
    ),

    TP_printk("c=%08x",
    __entry->call_debug_id)
    );

    TRACE_EVENT(rxrpc_sack,
    TP_PROTO(struct rxrpc_call *call, rxrpc_seq_t seq,
    unsigned int sack, enum rxrpc_sack_trace what),

    TP_ARGS(call, seq, sack, what),

    TP_STRUCT__entry(
    __field(unsigned int,		call_debug_id)
    __field(rxrpc_seq_t,		seq)
    __field(unsigned int,		sack)
    __field(enum rxrpc_sack_trace,	what)
    ),

    TP_fast_assign(
    __entry->call_debug_id = call->debug_id;
    __entry->seq = seq;
    __entry->sack = sack;
    __entry->what = what;
    ),

    TP_printk("c=%08x q=%08x %s k=%x",
    __entry->call_debug_id,
    __entry->seq,
    __print_symbolic(__entry->what, rxrpc_sack_traces),
    __entry->sack)
    );

    TRACE_EVENT(rxrpc_pmtud_tx,
    TP_PROTO(struct rxrpc_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,	peer_debug_id)
    __field(unsigned int,	call_debug_id)
    __field(rxrpc_serial_t,	ping_serial)
    __field(unsigned short,	pmtud_trial)
    __field(unsigned short,	pmtud_good)
    __field(unsigned short,	pmtud_bad)
    ),

    TP_fast_assign(
    __entry->peer_debug_id = call->peer->debug_id;
    __entry->call_debug_id = call->debug_id;
    __entry->ping_serial = call->conn->pmtud_probe;
    __entry->pmtud_trial = call->peer->pmtud_trial;
    __entry->pmtud_good = call->peer->pmtud_good;
    __entry->pmtud_bad = call->peer->pmtud_bad;
    ),

    TP_printk("P=%08x c=%08x pr=%08x %u-%u-%u",
    __entry->peer_debug_id,
    __entry->call_debug_id,
    __entry->ping_serial,
    __entry->pmtud_good,
    __entry->pmtud_trial,
    __entry->pmtud_bad)
    );

    TRACE_EVENT(rxrpc_pmtud_rx,
    TP_PROTO(struct rxrpc_connection *conn, rxrpc_serial_t resp_serial),

    TP_ARGS(conn, resp_serial),

    TP_STRUCT__entry(
    __field(unsigned int,	peer_debug_id)
    __field(unsigned int,	call_debug_id)
    __field(rxrpc_serial_t,	ping_serial)
    __field(rxrpc_serial_t,	resp_serial)
    __field(unsigned short,	max_data)
    __field(u8,			jumbo_max)
    ),

    TP_fast_assign(
    __entry->peer_debug_id = conn->peer->debug_id;
    __entry->call_debug_id = conn->pmtud_call;
    __entry->ping_serial = conn->pmtud_probe;
    __entry->resp_serial = resp_serial;
    __entry->max_data = conn->peer->max_data;
    __entry->jumbo_max = conn->peer->pmtud_jumbo;
    ),

    TP_printk("P=%08x c=%08x pr=%08x rr=%08x max=%u jm=%u",
    __entry->peer_debug_id,
    __entry->call_debug_id,
    __entry->ping_serial,
    __entry->resp_serial,
    __entry->max_data,
    __entry->jumbo_max)
    );

    TRACE_EVENT(rxrpc_pmtud_lost,
    TP_PROTO(struct rxrpc_connection *conn, rxrpc_serial_t resp_serial),

    TP_ARGS(conn, resp_serial),

    TP_STRUCT__entry(
    __field(unsigned int,	peer_debug_id)
    __field(unsigned int,	call_debug_id)
    __field(rxrpc_serial_t,	ping_serial)
    __field(rxrpc_serial_t,	resp_serial)
    ),

    TP_fast_assign(
    __entry->peer_debug_id = conn->peer->debug_id;
    __entry->call_debug_id = conn->pmtud_call;
    __entry->ping_serial = conn->pmtud_probe;
    __entry->resp_serial = resp_serial;
    ),

    TP_printk("P=%08x c=%08x pr=%08x rr=%08x",
    __entry->peer_debug_id,
    __entry->call_debug_id,
    __entry->ping_serial,
    __entry->resp_serial)
    );

    TRACE_EVENT(rxrpc_pmtud_reduce,
    TP_PROTO(struct rxrpc_peer *peer, rxrpc_serial_t serial,
    unsigned int max_data, enum rxrpc_pmtud_reduce_trace reason),

    TP_ARGS(peer, serial, max_data, reason),

    TP_STRUCT__entry(
    __field(unsigned int,	peer_debug_id)
    __field(rxrpc_serial_t,	serial)
    __field(unsigned int,	max_data)
    __field(enum rxrpc_pmtud_reduce_trace, reason)
    ),

    TP_fast_assign(
    __entry->peer_debug_id = peer->debug_id;
    __entry->serial = serial;
    __entry->max_data = max_data;
    __entry->reason = reason;
    ),

    TP_printk("P=%08x %s r=%08x m=%u",
    __entry->peer_debug_id,
    __print_symbolic(__entry->reason, rxrpc_pmtud_reduce_traces),
    __entry->serial, __entry->max_data)
    );

    TRACE_EVENT(rxrpc_rack,
    TP_PROTO(struct rxrpc_call *call, ktime_t timo),

    TP_ARGS(call, timo),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_serial_t,	ack_serial)
    __field(rxrpc_seq_t,	seq)
    __field(enum rxrpc_rack_timer_mode, mode)
    __field(unsigned short,	nr_sent)
    __field(unsigned short,	nr_lost)
    __field(unsigned short,	nr_resent)
    __field(unsigned short,	nr_sacked)
    __field(ktime_t,		timo)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->ack_serial	= call->rx_serial;
    __entry->seq	= call->rack_end_seq;
    __entry->mode	= call->rack_timer_mode;
    __entry->nr_sent	= call->tx_nr_sent;
    __entry->nr_lost	= call->tx_nr_lost;
    __entry->nr_resent	= call->tx_nr_resent;
    __entry->nr_sacked	= call->acks_nr_sacks;
    __entry->timo	= timo;
    ),

    TP_printk("c=%08x r=%08x q=%08x %s slrs=%u,%u,%u,%u t=%lld",
    __entry->call, __entry->ack_serial, __entry->seq,
    __print_symbolic(__entry->mode, rxrpc_rack_timer_modes),
    __entry->nr_sent, __entry->nr_lost,
    __entry->nr_resent, __entry->nr_sacked,
    ktime_to_us(__entry->timo))
    );

    TRACE_EVENT(rxrpc_rack_update,
    TP_PROTO(struct rxrpc_call *call, struct rxrpc_ack_summary *summary),

    TP_ARGS(call, summary),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_serial_t,	ack_serial)
    __field(rxrpc_seq_t,	seq)
    __field(int,		xmit_ts)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->ack_serial	= call->rx_serial;
    __entry->seq	= call->rack_end_seq;
    __entry->xmit_ts	= ktime_sub(call->acks_latest_ts, call->rack_xmit_ts);
    ),

    TP_printk("c=%08x r=%08x q=%08x xt=%lld",
    __entry->call, __entry->ack_serial, __entry->seq,
    ktime_to_us(__entry->xmit_ts))
    );

    TRACE_EVENT(rxrpc_rack_scan_loss,
    TP_PROTO(struct rxrpc_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(ktime_t,		rack_rtt)
    __field(ktime_t,		rack_reo_wnd)
    ),

    TP_fast_assign(
    __entry->call		= call->debug_id;
    __entry->rack_rtt		= call->rack_rtt;
    __entry->rack_reo_wnd	= call->rack_reo_wnd;
    ),

    TP_printk("c=%08x rtt=%lld reow=%lld",
    __entry->call, ktime_to_us(__entry->rack_rtt),
    ktime_to_us(__entry->rack_reo_wnd))
    );

    TRACE_EVENT(rxrpc_rack_scan_loss_tq,
    TP_PROTO(struct rxrpc_call *call, const struct rxrpc_txqueue *tq,
    unsigned long nacks),

    TP_ARGS(call, tq, nacks),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_seq_t,	qbase)
    __field(unsigned long,	nacks)
    __field(unsigned long,	lost)
    __field(unsigned long,	retrans)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->qbase	= tq->qbase;
    __entry->nacks	= nacks;
    __entry->lost	= tq->segment_lost;
    __entry->retrans	= tq->segment_retransmitted;
    ),

    TP_printk("c=%08x q=%08x n=%lx l=%lx r=%lx",
    __entry->call, __entry->qbase,
    __entry->nacks, __entry->lost, __entry->retrans)
    );

    TRACE_EVENT(rxrpc_rack_detect_loss,
    TP_PROTO(struct rxrpc_call *call, struct rxrpc_ack_summary *summary,
    rxrpc_seq_t seq),

    TP_ARGS(call, summary, seq),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_serial_t,	ack_serial)
    __field(rxrpc_seq_t,	seq)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->ack_serial	= call->rx_serial;
    __entry->seq	= seq;
    ),

    TP_printk("c=%08x r=%08x q=%08x",
    __entry->call, __entry->ack_serial, __entry->seq)
    );

    TRACE_EVENT(rxrpc_rack_mark_loss_tq,
    TP_PROTO(struct rxrpc_call *call, const struct rxrpc_txqueue *tq),

    TP_ARGS(call, tq),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(rxrpc_seq_t,	qbase)
    __field(rxrpc_seq_t,	trans)
    __field(unsigned long,	acked)
    __field(unsigned long,	lost)
    __field(unsigned long,	retrans)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->qbase	= tq->qbase;
    __entry->trans	= call->tx_transmitted;
    __entry->acked	= tq->segment_acked;
    __entry->lost	= tq->segment_lost;
    __entry->retrans	= tq->segment_retransmitted;
    ),

    TP_printk("c=%08x tq=%08x txq=%08x a=%lx l=%lx r=%lx",
    __entry->call, __entry->qbase, __entry->trans,
    __entry->acked, __entry->lost, __entry->retrans)
    );

    TRACE_EVENT(rxrpc_tlp_probe,
    TP_PROTO(struct rxrpc_call *call, enum rxrpc_tlp_probe_trace trace),

    TP_ARGS(call, trace),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(rxrpc_serial_t,		serial)
    __field(rxrpc_seq_t,		seq)
    __field(enum rxrpc_tlp_probe_trace,	trace)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->serial	= call->tlp_serial;
    __entry->seq	= call->tlp_seq;
    __entry->trace	= trace;
    ),

    TP_printk("c=%08x r=%08x pq=%08x %s",
    __entry->call, __entry->serial, __entry->seq,
    __print_symbolic(__entry->trace, rxrpc_tlp_probe_traces))
    );

    TRACE_EVENT(rxrpc_tlp_ack,
    TP_PROTO(struct rxrpc_call *call, struct rxrpc_ack_summary *summary,
    enum rxrpc_tlp_ack_trace trace),

    TP_ARGS(call, summary, trace),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(rxrpc_serial_t,		serial)
    __field(rxrpc_seq_t,		tlp_seq)
    __field(rxrpc_seq_t,		hard_ack)
    __field(enum rxrpc_tlp_ack_trace,	trace)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->serial	= call->tlp_serial;
    __entry->tlp_seq	= call->tlp_seq;
    __entry->hard_ack	= call->acks_hard_ack;
    __entry->trace	= trace;
    ),

    TP_printk("c=%08x r=%08x pq=%08x hq=%08x %s",
    __entry->call, __entry->serial,
    __entry->tlp_seq, __entry->hard_ack,
    __print_symbolic(__entry->trace, rxrpc_tlp_ack_traces))
    );

    TRACE_EVENT(rxrpc_rack_timer,
    TP_PROTO(struct rxrpc_call *call, ktime_t delay, bool exp),

    TP_ARGS(call, delay, exp),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(bool,			exp)
    __field(enum rxrpc_rack_timer_mode,	mode)
    __field(ktime_t,			delay)
    ),

    TP_fast_assign(
    __entry->call		= call->debug_id;
    __entry->exp		= exp;
    __entry->mode		= call->rack_timer_mode;
    __entry->delay		= delay;
    ),

    TP_printk("c=%08x %s %s to=%lld",
    __entry->call,
    __entry->exp ? "Exp" : "Set",
    __print_symbolic(__entry->mode, rxrpc_rack_timer_modes),
    ktime_to_us(__entry->delay))
    );

    TRACE_EVENT(rxrpc_rxgk_rekey,
    TP_PROTO(struct rxrpc_connection *conn,
    unsigned int current_key, unsigned int requested_key),

    TP_ARGS(conn, current_key, requested_key),

    TP_STRUCT__entry(
    __field(unsigned int,	conn)
    __field(unsigned int,	current_key)
    __field(unsigned int,	requested_key)
    ),

    TP_fast_assign(
    __entry->conn		= conn->debug_id;
    __entry->current_key	= current_key;
    __entry->requested_key	= requested_key;
    ),

    TP_printk("C=%08x cur=%x req=%x",
    __entry->conn,
    __entry->current_key,
    __entry->requested_key)
    );

// This part must be outside protection
