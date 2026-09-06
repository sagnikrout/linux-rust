//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/rds.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB)
//
// Copyright (c) 2008, 2018 Oracle and/or its affiliates. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const RDS_IB_ABI_VERSION: c_uint = 0x301;
pub const SOL_RDS: c_int = 276;
//
// setsockopt/getsockopt for SOL_RDS
//
pub const RDS_CANCEL_SENT_TO: c_int = 1;
pub const RDS_GET_MR: c_int = 2;
pub const RDS_FREE_MR: c_int = 3;
// deprecated: RDS_BARRIER 4
pub const RDS_RECVERR: c_int = 5;
pub const RDS_CONG_MONITOR: c_int = 6;
pub const RDS_GET_MR_FOR_DEST: c_int = 7;
pub const SO_RDS_TRANSPORT: c_int = 8;
// Socket option to tap receive path latency
// SO_RDS: SO_RDS_MSG_RXPATH_LATENCY
// Format used struct rds_rx_trace_so
//
pub const SO_RDS_MSG_RXPATH_LATENCY: c_int = 10;
// supported values for SO_RDS_TRANSPORT
pub const RDS_TRANS_IB: c_int = 0;
pub const RDS_TRANS_GAP: c_int = 1;
pub const RDS_TRANS_TCP: c_int = 2;
pub const RDS_TRANS_COUNT: c_int = 3;

// don't use RDS_TRANS_IWARP - it is deprecated

// IOCTLS commands for SOL_RDS

pub type rds_tos_t = __u8;
//
// Control message types for SOL_RDS.
//
// CMSG_RDMA_ARGS (sendmsg)
// Request a RDMA transfer to/from the specified
// memory ranges.
// The cmsg_data is a struct rds_rdma_args.
// RDS_CMSG_RDMA_DEST (recvmsg, sendmsg)
// Kernel informs application about intended
// source/destination of a RDMA transfer
// RDS_CMSG_RDMA_MAP (sendmsg)
// Application asks kernel to map the given
// memory range into a IB MR, and send the
// R_Key along in an RDS extension header.
// The cmsg_data is a struct rds_get_mr_args,
// the same as for the GET_MR setsockopt.
// RDS_CMSG_RDMA_STATUS (recvmsg)
// Returns the status of a completed RDMA operation.
// RDS_CMSG_RXPATH_LATENCY(recvmsg)
// Returns rds message latencies in various stages of receive
// path in nS. Its set per socket using SO_RDS_MSG_RXPATH_LATENCY
// socket option. Legitimate points are defined in
// enum rds_message_rxpath_latency. More points can be added in
// future. CSMG format is struct rds_cmsg_rx_trace.
//
pub const RDS_CMSG_RDMA_ARGS: c_int = 1;
pub const RDS_CMSG_RDMA_DEST: c_int = 2;
pub const RDS_CMSG_RDMA_MAP: c_int = 3;
pub const RDS_CMSG_RDMA_STATUS: c_int = 4;
pub const RDS_CMSG_CONG_UPDATE: c_int = 5;
pub const RDS_CMSG_ATOMIC_FADD: c_int = 6;
pub const RDS_CMSG_ATOMIC_CSWP: c_int = 7;
pub const RDS_CMSG_MASKED_ATOMIC_FADD: c_int = 8;
pub const RDS_CMSG_MASKED_ATOMIC_CSWP: c_int = 9;
pub const RDS_CMSG_RXPATH_LATENCY: c_int = 11;
pub const RDS_CMSG_ZCOPY_COOKIE: c_int = 12;
pub const RDS_CMSG_ZCOPY_COMPLETION: c_int = 13;
pub const RDS_INFO_FIRST: c_int = 10000;
pub const RDS_INFO_COUNTERS: c_int = 10000;
pub const RDS_INFO_CONNECTIONS: c_int = 10001;
// 10002 aka RDS_INFO_FLOWS is deprecated
pub const RDS_INFO_SEND_MESSAGES: c_int = 10003;
pub const RDS_INFO_RETRANS_MESSAGES: c_int = 10004;
pub const RDS_INFO_RECV_MESSAGES: c_int = 10005;
pub const RDS_INFO_SOCKETS: c_int = 10006;
pub const RDS_INFO_TCP_SOCKETS: c_int = 10007;
pub const RDS_INFO_IB_CONNECTIONS: c_int = 10008;
pub const RDS_INFO_CONNECTION_STATS: c_int = 10009;
pub const RDS_INFO_IWARP_CONNECTIONS: c_int = 10010;
// PF_RDS6 options
pub const RDS6_INFO_CONNECTIONS: c_int = 10011;
pub const RDS6_INFO_SEND_MESSAGES: c_int = 10012;
pub const RDS6_INFO_RETRANS_MESSAGES: c_int = 10013;
pub const RDS6_INFO_RECV_MESSAGES: c_int = 10014;
pub const RDS6_INFO_SOCKETS: c_int = 10015;
pub const RDS6_INFO_TCP_SOCKETS: c_int = 10016;
pub const RDS6_INFO_IB_CONNECTIONS: c_int = 10017;
pub const RDS_INFO_LAST: c_int = 10017;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_info_counter {
    pub name: [__u8; 32],
    pub value: __u64,
    pub __attribute__((packed)): },
pub const RDS_INFO_CONNECTION_FLAG_SENDING: c_uint = 0x01;
pub const RDS_INFO_CONNECTION_FLAG_CONNECTING: c_uint = 0x02;
pub const RDS_INFO_CONNECTION_FLAG_CONNECTED: c_uint = 0x04;
pub const TRANSNAMSIZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_info_connection {
    pub next_tx_seq: __u64,
    pub next_rx_seq: __u64,
    pub laddr: __be32,
    pub faddr: __be32,
    pub /: *mut *mut __u8 transport[TRANSNAMSIZ]; / null term ascii,
    pub flags: __u8,
    pub tos: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds6_info_connection {
    pub next_tx_seq: __u64,
    pub next_rx_seq: __u64,
    pub laddr: in6_addr,
    pub faddr: in6_addr,
    pub /: *mut *mut __u8 transport[TRANSNAMSIZ]; / null term ascii,
    pub flags: __u8,
    pub __attribute__((packed)): },
pub const RDS_INFO_MESSAGE_FLAG_ACK: c_uint = 0x01;
pub const RDS_INFO_MESSAGE_FLAG_FAST_ACK: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_info_message {
    pub seq: __u64,
    pub len: __u32,
    pub laddr: __be32,
    pub faddr: __be32,
    pub lport: __be16,
    pub fport: __be16,
    pub flags: __u8,
    pub tos: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds6_info_message {
    pub seq: __u64,
    pub len: __u32,
    pub laddr: in6_addr,
    pub faddr: in6_addr,
    pub lport: __be16,
    pub fport: __be16,
    pub flags: __u8,
    pub tos: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_info_socket {
    pub sndbuf: __u32,
    pub bound_addr: __be32,
    pub connected_addr: __be32,
    pub bound_port: __be16,
    pub connected_port: __be16,
    pub rcvbuf: __u32,
    pub inum: __u64,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds6_info_socket {
    pub sndbuf: __u32,
    pub bound_addr: in6_addr,
    pub connected_addr: in6_addr,
    pub bound_port: __be16,
    pub connected_port: __be16,
    pub rcvbuf: __u32,
    pub inum: __u64,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_info_tcp_socket {
    pub local_addr: __be32,
    pub local_port: __be16,
    pub peer_addr: __be32,
    pub peer_port: __be16,
    pub hdr_rem: __u64,
    pub data_rem: __u64,
    pub last_sent_nxt: __u32,
    pub last_expected_una: __u32,
    pub last_seen_una: __u32,
    pub tos: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds6_info_tcp_socket {
    pub local_addr: in6_addr,
    pub local_port: __be16,
    pub peer_addr: in6_addr,
    pub peer_port: __be16,
    pub hdr_rem: __u64,
    pub data_rem: __u64,
    pub last_sent_nxt: __u32,
    pub last_expected_una: __u32,
    pub last_seen_una: __u32,
    pub __attribute__((packed)): },
pub const RDS_IB_GID_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_info_rdma_connection {
    pub src_addr: __be32,
    pub dst_addr: __be32,
    pub src_gid: [__u8; RDS_IB_GID_LEN],
    pub dst_gid: [__u8; RDS_IB_GID_LEN],
    pub max_send_wr: __u32,
    pub max_recv_wr: __u32,
    pub max_send_sge: __u32,
    pub rdma_mr_max: __u32,
    pub rdma_mr_size: __u32,
    pub tos: __u8,
    pub sl: __u8,
    pub cache_allocs: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds6_info_rdma_connection {
    pub src_addr: in6_addr,
    pub dst_addr: in6_addr,
    pub src_gid: [__u8; RDS_IB_GID_LEN],
    pub dst_gid: [__u8; RDS_IB_GID_LEN],
    pub max_send_wr: __u32,
    pub max_recv_wr: __u32,
    pub max_send_sge: __u32,
    pub rdma_mr_max: __u32,
    pub rdma_mr_size: __u32,
    pub tos: __u8,
    pub sl: __u8,
    pub cache_allocs: __u32,
}

// RDS message Receive Path Latency points
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rds_message_rxpath_latency {
    RDS_MSG_RX_HDR_TO_DGRAM_START = 0,
    RDS_MSG_RX_DGRAM_REASSEMBLE,
    RDS_MSG_RX_DGRAM_DELIVERED,
    RDS_MSG_RX_DGRAM_TRACE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_rx_trace_so {
    pub rx_traces: __u8,
    pub rx_trace_pos: [__u8; RDS_MSG_RX_DGRAM_TRACE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_cmsg_rx_trace {
    pub rx_traces: __u8,
    pub rx_trace_pos: [__u8; RDS_MSG_RX_DGRAM_TRACE_MAX],
    pub rx_trace: [__u64; RDS_MSG_RX_DGRAM_TRACE_MAX],
}

//
// Congestion monitoring.
// Congestion control in RDS happens at the host connection
// level by exchanging a bitmap marking congested ports.
// By default, a process sleeping in poll() is always woken
// up when the congestion map is updated.
// With explicit monitoring, an application can have more
// fine-grained control.
// The application installs a 64bit mask value in the socket,
// where each bit corresponds to a group of ports.
// When a congestion update arrives, RDS checks the set of
// ports that are now uncongested against the list bit mask
// installed in the socket, and if they overlap, we queue a
// cong_notification on the socket.
//
// To install the congestion monitor bitmask, use RDS_CONG_MONITOR
// with the 64bit mask.
// Congestion updates are received via RDS_CMSG_CONG_UPDATE
// control messages.
//
// The correspondence between bits and ports is
// 1 << (portnum % 64)
//
pub const RDS_CONG_MONITOR_SIZE: c_int = 64;

//
// RDMA related types
//
// This encapsulates a remote memory location.
// In the current implementation, it contains the R_Key
// of the remote memory region, and the offset into it
// (so that the application does not have to worry about
// alignment).
//
pub type rds_rdma_cookie_t = __u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_iovec {
    pub addr: __u64,
    pub bytes: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_get_mr_args {
    pub vec: rds_iovec,
    pub cookie_addr: __u64,
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_get_mr_for_dest_args {
    pub dest_addr: __kernel_sockaddr_storage,
    pub vec: rds_iovec,
    pub cookie_addr: __u64,
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_free_mr_args {
    pub cookie: rds_rdma_cookie_t,
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_rdma_args {
    pub cookie: rds_rdma_cookie_t,
    pub remote_vec: rds_iovec,
    pub local_vec_addr: __u64,
    pub nr_local: __u64,
    pub flags: __u64,
    pub user_token: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_atomic_args {
    pub cookie: rds_rdma_cookie_t,
    pub local_addr: __u64,
    pub remote_addr: __u64,
    pub compare: __u64,
    pub swap: __u64,
    pub cswp: },
    pub add: __u64,
    pub fadd: },
    pub compare: __u64,
    pub swap: __u64,
    pub compare_mask: __u64,
    pub swap_mask: __u64,
    pub m_cswp: },
    pub add: __u64,
    pub nocarry_mask: __u64,
    pub m_fadd: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_rdma_notify {
    pub user_token: __u64,
    pub status: __s32,
}

pub const RDS_RDMA_SUCCESS: c_int = 0;
pub const RDS_RDMA_REMOTE_ERROR: c_int = 1;
pub const RDS_RDMA_CANCELED: c_int = 2;
pub const RDS_RDMA_DROPPED: c_int = 3;
pub const RDS_RDMA_OTHER_ERROR: c_int = 4;
pub const RDS_MAX_ZCOOKIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_zcopy_cookies {
    pub num: __u32,
    pub cookies: [__u32; RDS_MAX_ZCOOKIES],
}

//
// Common set of flags for all RDMA related structs
//
pub const RDS_RDMA_READWRITE: c_uint = 0x0001;
pub const RDS_RDMA_FENCE: c_uint = 0x0002	/* use FENCE for immediate send */;
pub const RDS_RDMA_INVALIDATE: c_uint = 0x0004	/* invalidate R_Key after freeing MR */;
pub const RDS_RDMA_USE_ONCE: c_uint = 0x0008	/* free MR after use */;
pub const RDS_RDMA_DONTWAIT: c_uint = 0x0010	/* Don't wait in SET_BARRIER */;
pub const RDS_RDMA_NOTIFY_ME: c_uint = 0x0020	/* Notify when operation completes */;
pub const RDS_RDMA_SILENT: c_uint = 0x0040	/* Do not interrupt remote */;
