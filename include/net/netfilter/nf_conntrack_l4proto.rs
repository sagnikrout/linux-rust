//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_l4proto.h
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
//
// Header for use in defining a given L4 protocol for connection tracking.
//
// 16 Dec 2003: Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
// - generalized L3 protocol dependent part.
//
// Derived from include/linux/netfiter_ipv4/ip_conntrack_protcol.h
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_l4proto {
// L4 Protocol number.
    pub l4proto: u_int8_t,
// Resolve clashes on insertion races.
    pub allow_clash: bool,
// protoinfo nlattr size, closes a hole
    pub nlattr_size: u16,
// called by gc worker if table is full
    pub ct): *const *const bool (can_early_drop)(struct nf_conn,
// convert protoinfo to nfnetlink attributes
    pub destroy): *mut *mut nf_conn ct, bool,
// convert nfnetlink attributes to protoinfo
    pub ct): *mut *mut *mut int (from_nlattr)(struct nlattr tb[], struct nf_conn,
    pub t): *const nf_conntrack_tuple,
// Calculate tuple nlattr size
    pub (*nlattr_tuple_size)(void): *mut c_uint,
    pub flags): u_int32_t,
    pub nla_policy: *const nla_policy,
    pub data): *mut *mut net net, void,
    pub data): *const *const *const int (obj_to_nlattr)(struct sk_buff skb, void,
    pub obj_size: u16,
    pub nlattr_max: u16,
    pub nla_policy: *const nla_policy,
    pub ctnl_timeout: },

// Print out the private part of the conntrack.
    pub ): *mut *mut *mut void (print_conntrack)(struct seq_file s, struct nf_conn,

}

extern "C" {
    pub fn nf_conntrack_generic_init_net(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_tcp_init_net(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_udp_init_net(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_gre_init_net(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_sctp_init_net(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_icmp_init_net(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_icmpv6_init_net(net: *mut net);
}
// Existing built-in generic protocol
// Generic netlink helpers
extern "C" {
    pub fn nf_ct_port_nlattr_tuple_size() -> c_uint;
}

// Caller must check nf_ct_protonum(ct) is IPPROTO_TCP before calling.

