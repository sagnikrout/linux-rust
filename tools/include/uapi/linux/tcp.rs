//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/tcp.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the TCP protocol.
//
// Version:	@(#)tcp.h	1.0.2	04/28/93
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub source: __be16,
    pub dest: __be16,
    pub seq: __be32,
    pub ack_seq: __be32,

    pub window: __be16,
    pub check: __sum16,
    pub urg_ptr: __be16,
}

//
// The union cast uses a gcc extension to avoid aliasing problems
// (union is compatible to any of its members)
// This means this part of the code is -fstrict-aliasing safe now.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_word_hdr {
    pub hdr: tcphdr,
    pub words: [__be32; 5],
}

//
// TCP general constants
//

// TCP socket options

pub const TCP_REPAIR_QUEUE: c_int = 20;
pub const TCP_QUEUE_SEQ: c_int = 21;
pub const TCP_REPAIR_OPTIONS: c_int = 22;

pub const TCP_TIMESTAMP: c_int = 24;

pub const TCP_ZEROCOPY_RECEIVE: c_int = 35;

pub const TCP_REPAIR_ON: c_int = 1;
pub const TCP_REPAIR_OFF: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_repair_opt {
    pub opt_code: __u32,
    pub opt_val: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_repair_window {
    pub snd_wl1: __u32,
    pub snd_wnd: __u32,
    pub max_window: __u32,
    pub rcv_wnd: __u32,
    pub rcv_wup: __u32,
}

// why fastopen failed from client perspective
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_fastopen_client_fail {
    TFO_STATUS_UNSPEC, /* catch-all */
    TFO_COOKIE_UNAVAILABLE, /* if not in TFO_CLIENT_NO_COOKIE mode */
    TFO_DATA_NOT_ACKED, /* SYN-ACK did not ack SYN data */
    TFO_SYN_RETRANSMITTED, /* SYN-ACK did not ack SYN data after timeout */
}

// for TCP_INFO socket option
pub const TCPI_OPT_TIMESTAMPS: c_int = 1;
pub const TCPI_OPT_SACK: c_int = 2;
pub const TCPI_OPT_WSCALE: c_int = 4;

//
// Sender's congestion state indicating normal or abnormal situations
// in the last round of packets sent. The state is driven by the ACK
// information and timer events.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_ca_state {
//
// Nothing bad has been observed recently.
// No apparent reordering, packet loss, or ECN marks.
//
    TCP_CA_Open = 0,

//
// The sender enters disordered state when it has received DUPACKs or
// SACKs in the last round of packets sent. This could be due to packet
// loss or reordering but needs further information to confirm packets
// have been lost.
//
    TCP_CA_Disorder = 1,

//
// The sender enters Congestion Window Reduction (CWR) state when it
// has received ACKs with ECN-ECE marks, or has experienced congestion
// or packet discard on the sender host (e.g. qdisc).
//
    TCP_CA_CWR = 2,

//
// The sender is in fast recovery and retransmitting lost packets,
// typically triggered by ACK events.
//
    TCP_CA_Recovery = 3,

//
// The sender is in loss recovery triggered by retransmission timeout.
//
    TCP_CA_Loss = 4

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_info {
    pub tcpi_state: __u8,
    pub tcpi_ca_state: __u8,
    pub tcpi_retransmits: __u8,
    pub tcpi_probes: __u8,
    pub tcpi_backoff: __u8,
    pub tcpi_options: __u8,
    pub 4: __u8 tcpi_snd_wscale : 4, tcpi_rcv_wscale :,
    pub tcpi_fastopen_client_fail:2: __u8 tcpi_delivery_rate_app_limited:1,,
    pub tcpi_rto: __u32,
    pub tcpi_ato: __u32,
    pub tcpi_snd_mss: __u32,
    pub tcpi_rcv_mss: __u32,
    pub tcpi_unacked: __u32,
    pub tcpi_sacked: __u32,
    pub tcpi_lost: __u32,
    pub tcpi_retrans: __u32,
    pub tcpi_fackets: __u32,
// Times.
    pub tcpi_last_data_sent: __u32,
    pub /: *mut *mut __u32 tcpi_last_ack_sent; / Not remembered, sorry.,
    pub tcpi_last_data_recv: __u32,
    pub tcpi_last_ack_recv: __u32,
// Metrics.
    pub tcpi_pmtu: __u32,
    pub tcpi_rcv_ssthresh: __u32,
    pub tcpi_rtt: __u32,
    pub tcpi_rttvar: __u32,
    pub tcpi_snd_ssthresh: __u32,
    pub tcpi_snd_cwnd: __u32,
    pub tcpi_advmss: __u32,
    pub tcpi_reordering: __u32,
    pub tcpi_rcv_rtt: __u32,
    pub tcpi_rcv_space: __u32,
    pub tcpi_total_retrans: __u32,
    pub tcpi_pacing_rate: __u64,
    pub tcpi_max_pacing_rate: __u64,
    pub /: *mut *mut __u64 tcpi_bytes_acked; / RFC4898 tcpEStatsAppHCThruOctetsAcked,
    pub /: *mut *mut __u64 tcpi_bytes_received; / RFC4898 tcpEStatsAppHCThruOctetsReceived,
    pub /: *mut *mut __u32 tcpi_segs_out; / RFC4898 tcpEStatsPerfSegsOut,
    pub /: *mut *mut __u32 tcpi_segs_in; / RFC4898 tcpEStatsPerfSegsIn,
    pub tcpi_notsent_bytes: __u32,
    pub tcpi_min_rtt: __u32,
    pub /: *mut *mut __u32 tcpi_data_segs_in; / RFC4898 tcpEStatsDataSegsIn,
    pub /: *mut *mut __u32 tcpi_data_segs_out; / RFC4898 tcpEStatsDataSegsOut,
    pub tcpi_delivery_rate: __u64,
    pub /: *mut *mut __u64 tcpi_busy_time; / Time (usec) busy sending data,
    pub /: *mut *mut __u64 tcpi_rwnd_limited; / Time (usec) limited by receive window,
    pub /: *mut *mut __u64 tcpi_sndbuf_limited; / Time (usec) limited by send buffer,
    pub tcpi_delivered: __u32,
    pub tcpi_delivered_ce: __u32,
    pub /: *mut *mut __u64 tcpi_bytes_sent; / RFC4898 tcpEStatsPerfHCDataOctetsOut,
    pub /: *mut *mut __u64 tcpi_bytes_retrans; / RFC4898 tcpEStatsPerfOctetsRetrans,
    pub /: *mut *mut __u32 tcpi_dsack_dups; / RFC4898 tcpEStatsStackDSACKDups,
    pub /: *mut *mut __u32 tcpi_reord_seen; / reordering events seen,
    pub /: *mut *mut __u32 tcpi_rcv_ooopack; / Out-of-order packets received,
    pub after: *mut *mut __u32 tcpi_snd_wnd; / peer's advertised receive window,
// scaling (bytes)
//
}

// netlink attributes types for SCM_TIMESTAMPING_OPT_STATS
// for TCP_MD5SIG socket option
pub const TCP_MD5SIG_MAXKEYLEN: c_int = 80;
// tcp_md5sig extension flags for TCP_MD5SIG_EXT
pub const TCP_MD5SIG_FLAG_PREFIX: c_uint = 0x1	/* address prefix length */;
pub const TCP_MD5SIG_FLAG_IFINDEX: c_uint = 0x2	/* ifindex set */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_md5sig {
    pub /: *mut *mut __kernel_sockaddr_storage tcpm_addr; / address associated,
    pub /: *mut *mut __u8 tcpm_flags; / extension flags,
    pub /: *mut *mut __u8 tcpm_prefixlen; / address prefix,
    pub /: *mut *mut __u16 tcpm_keylen; / key length,
    pub /: *mut *mut int tcpm_ifindex; / device index for scope,
    pub /: *mut *mut __u8 tcpm_key[TCP_MD5SIG_MAXKEYLEN]; / key (binary),
}

// INET_DIAG_MD5SIG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_diag_md5sig {
    pub tcpm_family: __u8,
    pub tcpm_prefixlen: __u8,
    pub tcpm_keylen: __u16,
    pub tcpm_addr: [__be32; 4],
    pub tcpm_key: [__u8; TCP_MD5SIG_MAXKEYLEN],
}

// setsockopt(fd, IPPROTO_TCP, TCP_ZEROCOPY_RECEIVE, ...)
pub const TCP_RECEIVE_ZEROCOPY_FLAG_TLB_CLEAN_HINT: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_zerocopy_receive {
    pub /: *mut *mut __u64 address; / in: address of mapping,
    pub /: *mut *mut __u32 length; / in/out: number of bytes to map/mapped,
    pub /: *mut *mut __u32 recv_skip_hint; / out: amount of bytes to skip,
    pub /: *mut *mut __u32 inq; / out: amount of bytes in read queue,
    pub /: *mut *mut __s32 err; / out: socket error,
    pub /: *mut *mut __u64 copybuf_address; / in: copybuf address (small reads),
    pub /: *mut *mut __s32 copybuf_len; / in/out: copybuf bytes avail/used or error,
    pub /: *mut *mut __u32 flags; / in: flags,
}
