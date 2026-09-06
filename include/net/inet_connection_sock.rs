//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inet_connection_sock.h
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
// NET		Generic infrastructure for INET connection oriented protocols.
//
// Definitions for inet_connection_sock
//
// Authors:	Many people, see the TCP sources
//
// From code originally in TCP
//

// Cancel timers, when they are not required.

//
// Pointers to address related TCP functions
// (i.e. things that depend on the address family)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_connection_sock_af_ops {
    pub fl): *mut *mut *mut *mut int (queue_xmit)(struct sock sk, struct sk_buff skb, struct flowi,
    pub net_header_len: u16,
    pub sk): *mut *mut int (rebuild_header)(struct sock,
    pub skb): *const *const *const void (sk_rx_dst_set)(struct sock sk, struct sk_buff,
    pub skb): *mut *mut *mut int (conn_request)(struct sock sk, struct sk_buff,
    pub sk)): *const sock,
    pub optlen): sockptr_t optval, unsigned int,
    pub optlen): *mut *mut char __user optval, int __user,
    pub sk): *mut *mut void (mtu_reduced)(struct sock,
}

// inet_connection_sock - INET connection oriented sock
//
// @icsk_accept_queue:	   FIFO of established children
// @icsk_bind_hash:	   Bind node
// @icsk_bind2_hash:	   Bind node in the bhash2 table
// @icsk_delack_timer:     Delayed ACK timer
// @icsk_keepalive_timer:  Keepalive timer
// @mptcp_tout_timer: mptcp timer
// @icsk_rto:		   Retransmit timeout
// @icsk_pmtu_cookie	   Last pmtu seen by socket
// @icsk_ca_ops		   Pluggable congestion control hook
// @icsk_af_ops		   Operations which are AF_INET{4,6} specific
// @icsk_ulp_ops	   Pluggable ULP control hook
// @icsk_ulp_data	   ULP private data
// @icsk_ca_state:	   Congestion control state
// @icsk_retransmits:	   Number of unrecovered [RTO] timeouts
// @icsk_pending:	   Scheduled timer event
// @icsk_backoff:	   Backoff
// @icsk_syn_retries:      Number of allowed SYN (or equivalent) retries
// @icsk_probes_out:	   unanswered 0 window probes
// @icsk_ext_hdr_len:	   Network protocol overhead (IP/IPv6 options)
// @icsk_ack:		   Delayed ACK control data
// @icsk_mtup;		   MTU probing control data
// @icsk_probes_tstamp:    Probe timestamp (cleared by non-zero window ack)
// @icsk_user_timeout:	   TCP_USER_TIMEOUT value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_connection_sock {
// inet_sock has to be the first member!
    pub icsk_inet: inet_sock,
    pub icsk_accept_queue: request_sock_queue,
    pub icsk_bind_hash: *mut inet_bind_bucket,
    pub icsk_bind2_hash: *mut inet_bind2_bucket,
    pub icsk_delack_timer: timer_list,
    pub icsk_keepalive_timer: timer_list,
    pub mptcp_tout_timer: timer_list,
}

extern "C" {
    pub fn int(sk: *mut *mut icsk_sync_mss)(struct sock, pmtu: u32) -> unsigned;
}
pub const ATO_BITS: c_int = 8;
// Range of MTUs to search
// Information on the current probe.
// Is the MTUP feature enabled for this connection?

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inet_csk_ack_state_t {
    ICSK_ACK_SCHED	= 1,
    ICSK_ACK_TIMER  = 2,
    ICSK_ACK_PUSHED = 4,
    ICSK_ACK_PUSHED2 = 8,
    ICSK_ACK_NOW = 16,	/* Send the next ACK immediately (once) */
    ICSK_ACK_NOMEM = 32,
}

extern "C" {
    pub fn inet_csk_clear_xmit_timers(sk: *mut sock);
}
extern "C" {
    pub fn inet_csk_clear_xmit_timers_sync(sk: *mut sock);
}
extern "C" {
    pub fn READ_ONCE(_arg: sk->tcp_retransmit_timer.expires) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: icsk->icsk_delack_timer.expires) -> return;
}

//
// Reset the retransmission timer
//
extern "C" {
    pub fn inet_csk_get_port(sk: *mut sock, snum: c_ushort) -> c_int;
}
extern "C" {
    pub fn inet_csk_reqsk_queue_hash_add(sk: *mut sock, req: *mut request_sock) -> bool;
}
extern "C" {
    pub fn reqsk_queue_len(_arg: &inet_csk(sk)->icsk_accept_queue) -> return;
}
extern "C" {
    pub fn inet_csk_reqsk_queue_len(READ_ONCE(sk->sk_max_ack_backlog: sk) >) -> return;
}
extern "C" {
    pub fn inet_csk_reqsk_queue_drop(sk: *mut sock, req: *mut request_sock) -> bool;
}
extern "C" {
    pub fn inet_csk_reqsk_queue_drop_and_put(sk: *mut sock, req: *mut request_sock);
}
extern "C" {
    pub fn inet_csk_destroy_sock(sk: *mut sock);
}
extern "C" {
    pub fn inet_csk_prepare_for_destroy_sock(sk: *mut sock);
}
extern "C" {
    pub fn inet_csk_prepare_forced_close(sk: *mut sock);
}
//
// LISTEN is a special case for poll..
//
extern "C" {
    pub fn inet_csk_listen_start(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn inet_csk_listen_stop(sk: *mut sock);
}
// update the fast reuse flag when adding a socket
