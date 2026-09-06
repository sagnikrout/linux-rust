//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inet_sock.h
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for inet_sock
//
// Authors:	Many, reorganised here by
// Arnaldo Carvalho de Melo <acme@mandriva.com>
//

pub const IP_OPTIONS_DATA_FIXED_SIZE: c_int = 40;
// struct ip_options - IP Options
//
// @faddr - Saved first hop address
// @nexthop - Saved nexthop address in LSRR and SSRR
// @is_strictroute - Strict source route
// @srr_is_hit - Packet destination addr was our one
// @is_changed - IP checksum more not valid
// @rr_needaddr - Need to record addr of outgoing dev
// @ts_needtime - Need to record timestamp
// @ts_needaddr - Need to record addr of outgoing dev
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_options {
    pub faddr: __be32,
    pub nexthop: __be32,
    pub optlen: c_uchar,
    pub srr: c_uchar,
    pub rr: c_uchar,
    pub ts: c_uchar,
    pub router_alert: c_uchar,
    pub cipso: c_uchar,
    pub __pad2: c_uchar,
    pub __data: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_options_rcu {
    pub rcu: rcu_head,
// Must be last as it ends in a flexible-array member.
    pub opt: ip_options,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_request_sock {
    pub req: request_sock,

    pub 1: smc_ok :,
    pub ir_mark: u32,
    pub ireq_opt: *mut ip_options_rcu __rcu,

    pub ipv6_opt: *mut ipv6_txoptions,
    pub pktopts: *mut sk_buff,
}

extern "C" {
    pub fn l3mdev_master_ifindex_by_index(_arg: net, _arg: skb->skb_iif) -> return;
}

extern "C" {
    pub fn inet_bound_dev_eq(_arg: true, _arg: bound_dev_if, _arg: dif, _arg: sdif) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_cork {
    pub opt: *mut ipv6_txoptions,
    pub hop_limit: u8,
    pub tclass: u8,
    pub dontfrag:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_cork {
    pub flags: c_uint,
    pub addr: __be32,
    pub opt: *mut ip_options,
    pub fragsize: c_uint,
    pub /: *mut *mut int length; / Total length of all frames,
    pub dst: *mut dst_entry,
    pub tx_flags: u8,
    pub ttl: __u8,
    pub tos: __s16,
    pub priority: u32,
    pub gso_size: __u16,
    pub ts_opt_id: u32,
    pub transmit_time: u64,
    pub mark: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_cork_full {
    pub base: inet_cork,
    pub fl: flowi,

    pub base6: inet6_cork,

}

// struct inet_sock - representation of INET sockets
//
// @sk - ancestor class
// @pinet6 - pointer to IPv6 control block
// @inet_daddr - Foreign IPv4 addr
// @inet_rcv_saddr - Bound local IPv4 addr
// @inet_dport - Destination port
// @inet_num - Local port
// @inet_flags - various atomic flags
// @inet_saddr - Sending source
// @uc_ttl - Unicast TTL
// @inet_sport - Source port
// @inet_id - ID counter for DF pkts
// @tos - TOS
// @mc_ttl - Multicasting TTL
// @uc_index - Unicast outgoing device index
// @mc_index - Multicast device index
// @mc_list - Group array
// @cork - info to build ip hdr on each ip frag while socket is corked
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_sock {
// sk and pinet6 has to be the first two members of inet_sock
    pub sk: sock,

    pub pinet6: *mut ipv6_pinfo,
    pub ipv6_fl_list: *mut ipv6_fl_socklist __rcu,

// Socket demultiplex comparisons on incoming packets.

    pub inet_flags: c_ulong,
    pub inet_saddr: __be32,
    pub uc_ttl: __s16,
    pub inet_sport: __be16,
    pub inet_opt: *mut ip_options_rcu __rcu,
    pub inet_id: core::sync::atomic::AtomicI32,
    pub tos: __u8,
    pub min_ttl: __u8,
    pub mc_ttl: __u8,
    pub pmtudisc: __u8,
    pub rcv_tos: __u8,
    pub convert_csum: __u8,
    pub uc_index: c_int,
    pub mc_index: c_int,
    pub mc_addr: __be32,
    pub /: *mut *mut u32 local_port_range; / high << 16 | low,
    pub mc_list: *mut ip_mc_socklist __rcu,
    pub cork: inet_cork_full,
}

// cmsg flags for inet

extern "C" {
    pub fn inet_dsfield_to_dscp(_arg: READ_ONCE(inet->tos)) -> return;
}

//
// sk_to_full_sk - Access to a full socket
// @sk: pointer to a socket
//
// SYNACK messages might be attached to request sockets.
// Some places want to reach the listener in this case.
//

// sk_to_full_sk() variant with a const argument

extern "C" {
    pub fn sk_to_full_sk(_arg: skb->sk) -> return;
}

extern "C" {
    pub fn inet_sk_rebuild_header(sk: *mut sock) -> c_int;
}
//
// inet_sk_state_load - read sk->sk_state for lockless contexts
// @sk: socket pointer
//
// Paired with inet_sk_state_store(). Used in places we don't hold socket lock:
// tcp_diag_get_info(), tcp_get_info(), tcp_poll(), get_tcp4_sock() ...
//
// state change might impact lockless readers.
extern "C" {
    pub fn smp_load_acquire(_arg: &sk->sk_state) -> return;
}
//
// inet_sk_state_store - update sk->sk_state
// @sk: socket pointer
// @newstate: new state
//
// Paired with inet_sk_state_load(). Should be used in contexts where
// state change might impact lockless readers.
//
extern "C" {
    pub fn inet_sk_state_store(sk: *mut sock, newstate: c_int);
}
extern "C" {
    pub fn inet_sk_set_state(sk: *mut sock, state: c_int);
}
