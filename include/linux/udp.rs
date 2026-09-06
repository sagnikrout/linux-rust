//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/udp.h
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
// Definitions for the UDP protocol.
//
// Version:	@(#)udp.h	1.0.2	04/28/93
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//

extern "C" {
    pub fn ntohs(_arg: uh->len) -> return;
}
extern "C" {
    pub fn ntohs(_arg: uh->len) -> return;
}
pub const UDP_HTABLE_SIZE_MIN_PERNET: c_int = 128;

pub const UDP_HTABLE_SIZE_MAX: c_int = 65536;
// per NUMA structure for lockless producer usage.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_prod_queue {
    pub ____cacheline_aligned_in_smp: llist_head ll_root,
    pub rmem_alloc: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_sock {
// inet_sock has to be the first member
    pub inet: inet_sock,

    pub udp_flags: c_ulong,
    pub /: *mut *mut int pending; / Any pending frames ?,
    pub /: *mut *mut __u8 encap_type; / Is this an Encapsulation socket?,

// For UDP 4-tuple hash
    pub udp_lrpa_hash: __u16,
    pub udp_lrpa_node: hlist_nulls_node,

//
// Following member retains the information to create a UDP header
// when the socket is uncorked.
//
    pub /: *mut *mut __u16 len; / total length of pending frames,
    pub gso_size: __u16,
//
// For encapsulation sockets.
//
    pub skb): *mut *mut *mut int (encap_rcv)(struct sock sk, struct sk_buff,
    pub payload): *mut __be16 port, u32 info, u8,
    pub skb): *mut *mut *mut int (encap_err_lookup)(struct sock sk, struct sk_buff,
    pub sk): *mut *mut void (encap_destroy)(struct sock,
// GRO functions for UDP socket
    pub skb): *mut sk_buff,
    pub nhoff): c_int,
    pub udp_prod_queue: *mut udp_prod_queue,
// udp_recvmsg try to use this before splicing sk_receive_queue
    pub ____cacheline_aligned_in_smp: sk_buff_head reader_queue,
// This field is dirtied by udp_recvmsg()
    pub forward_deficit: c_int,
// This fields follows rcvbuf value, and is touched by udp_recvmsg
    pub forward_threshold: c_int,
// Cache friendly copy of sk->sk_peek_off >= 0
    pub peeking_with_offset: bool,
//
// Accounting for the tunnel GRO fastpath.
// Unprotected by compilers guard, as it uses space available in
// the last UDP socket cacheline.
//
    pub tunnel_list: hlist_node,
    pub drop_counters: numa_drop_counters,
}

extern "C" {
    pub fn udp_test_bit(_arg: NO_CHECK6_TX, _arg: sk) -> return;
}
extern "C" {
    pub fn udp_test_bit(_arg: NO_CHECK6_RX, _arg: sk) -> return;
}

// GSO packets lacking the SKB_GSO_UDP_TUNNEL/_CSUM bits might still
// land in a tunnel as the socket check in udp_gro_receive cannot be
// foolproof.
//

extern "C" {
    pub fn rcu_dereference(_arg: net->ipv4.udp_tunnel_gro[is_ipv6].sk) -> return;
}

