//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ip_tunnels.h
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
pub const __NET_IP_TUNNELS_H: c_int = 1;

// Recursion limit for tunnel xmit to detect routing loops.
// Unlike XMIT_RECURSION_LIMIT (8) used in the no-qdisc path, tunnel
// recursion involves route lookups and full IP output, consuming much
// more stack per level, so a lower limit is needed.
//
pub const IP_TUNNEL_RECURSION_LIMIT: c_int = 5;
// Keep error state on tunnel for 30 sec

// Used to memset ip_tunnel padding.

// Used to memset ipv4 address padding.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_key {
    pub tun_id: __be64,
    pub src: __be32,
    pub dst: __be32,
    pub ipv4: },
    pub src: in6_addr,
    pub dst: in6_addr,
    pub ipv6: },
    pub u: },
    pub /: *mut *mut __be32 label; / Flow Label for IPv6,
    pub nhid: u32,
    pub /: *mut *mut u8 tos; / TOS for IPv4, TC for IPv6,
    pub /: *mut *mut u8 ttl; / TTL for IPv4, HL for IPv6,
    pub tp_src: __be16,
    pub tp_dst: __be16,
    pub flow_flags: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_encap {
    pub type: u16,
    pub flags: u16,
    pub sport: __be16,
    pub dport: __be16,
}

// Flags for ip_tunnel_info mode.
pub const IP_TUNNEL_INFO_TX: c_uint = 0x01	/* represents tx tunnel parameters */;
pub const IP_TUNNEL_INFO_IPV6: c_uint = 0x02	/* key contains IPv6 addresses */;
pub const IP_TUNNEL_INFO_BRIDGE: c_uint = 0x04	/* represents a bridged tunnel id */;
// Maximum tunnel options length.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_info {
    pub key: ip_tunnel_key,
    pub encap: ip_tunnel_encap,

    pub dst_cache: dst_cache,

    pub options_len: u8,
    pub mode: u8,
    pub __counted_by(options_len): u8 options[] __aligned_largest,
}

// 6rd prefix/relay information

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_6rd_parm {
    pub prefix: in6_addr,
    pub relay_prefix: __be32,
    pub prefixlen: u16,
    pub relay_prefixlen: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_prl_entry {
    pub next: *mut ip_tunnel_prl_entry __rcu,
    pub addr: __be32,
    pub flags: u16,
    pub rcu_head: rcu_head,
}

// Kernel-side variant of ip_tunnel_parm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_parm_kern {
    pub name: [c_char; IFNAMSIZ],
    pub i_key: __be32,
    pub o_key: __be32,
    pub link: c_int,
    pub iph: iphdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel {
    pub next: *mut ip_tunnel __rcu,
    pub hash_node: hlist_node,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub /: *mut *mut *mut net net; / netns for packet i/o,
    pub error: *mut *mut unsigned long err_time; / Time when the last ICMP,
// arrived
    pub /: *mut *mut int err_count; / Number of arrived ICMP errors,
// These four fields used only by GRE
    pub /: *mut *mut u32 i_seqno; / The last seen seqno,
    pub /: *mut *mut atomic_t o_seqno; / The last output seqno,
    pub /: *mut *mut int tun_hlen; / Precalculated header length,
// These four fields used only by ERSPAN
    pub /: *mut *mut u32 index; / ERSPAN type II index,
    pub /: *mut *mut u8 erspan_ver; / ERSPAN version,
    pub /: *mut *mut u8 dir; / ERSPAN direction,
    pub /: *mut *mut u16 hwid; / ERSPAN hardware ID,
    pub dst_cache: dst_cache,
    pub parms: ip_tunnel_parm_kern,
    pub mlink: c_int,
    pub /: *mut *mut int encap_hlen; / Encap header length (FOU,GUE),
    pub /: *mut *mut int hlen; / tun_hlen + encap_hlen,
    pub encap: ip_tunnel_encap,
// for SIT

    pub ip6rd: ip_tunnel_6rd_parm,

    pub /: *mut *mut *mut ip_tunnel_prl_entry __rcu prl; / potential router list,
    pub /: *mut *mut unsigned int prl_count; / # of entries in PRL,
    pub ip_tnl_net_id: c_uint,
    pub gro_cells: gro_cells,
    pub fwmark: __u32,
    pub collect_md: bool,
    pub ignore_df: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tnl_ptk_info {
    pub proto: __be16,
    pub key: __be32,
    pub seq: __be32,
    pub hdr_len: c_int,
}

pub const PACKET_RCVD: c_int = 0;
pub const PACKET_REJECT: c_int = 1;
pub const PACKET_NEXT: c_int = 2;
pub const IP_TNL_HASH_BITS: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_net {
    pub fb_tunnel_dev: *mut net_device,
    pub rtnl_link_ops: *mut rtnl_link_ops,
    pub tunnels: [hlist_head; IP_TNL_HASH_SIZE],
    pub collect_md_tun: *mut ip_tunnel __rcu,
    pub type: c_int,
}

extern "C" {
    pub fn ip_tunnel_flags_intersect(_arg: flags, _arg: present) -> return;
}
extern "C" {
    pub fn ip_tunnel_flags_subset(_arg: flags, _arg: supp) -> return;
}
// For the tunnel types on the top of IPsec, the tp_src and tp_dst of
// the upper tunnel are used.
// E.g: GRE over IPSEC, the tp_src and tp_port are zero.
//
// Clear struct padding.
// tun_info)

// Returns the least-significant 32 bits of a __be64.

// Legacy VRF/l3mdev use case
extern "C" {
    pub fn __ip_tunnel_init(dev: *mut net_device) -> c_int;
}

extern "C" {
    pub fn ip_tunnel_uninit(dev: *mut net_device);
}
extern "C" {
    pub fn ip_tunnel_dellink(dev: *mut net_device, head: *mut list_head);
}
extern "C" {
    pub fn ip_tunnel_get_iflink(dev: *const net_device) -> c_int;
}
extern "C" {
    pub fn ip_tunnel_parm_to_user(data: *mut void __user, kp: *mut ip_tunnel_parm_kern) -> bool;
}
extern "C" {
    pub fn ip_tunnel_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn ip_tunnel_md_udp_encap(skb: *mut sk_buff, info: *mut ip_tunnel_info);
}
extern "C" {
    pub fn ip_tunnel_setup(dev: *mut net_device, net_id: c_uint);
}
extern "C" {
    pub fn ip_tunnel_parse_protocol(skb: *const sk_buff) -> __be16;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_encap_ops {
    pub e): *mut *mut size_t (encap_hlen)(struct ip_tunnel_encap,
    pub fl4): *mut *mut u8 protocol, struct flowi4,
    pub info): *mut *mut *mut int (err_handler)(struct sk_buff skb, u32,
}

pub const MAX_IPTUN_ENCAP_OPS: c_int = 8;

extern "C" {
    pub fn pskb_network_may_pull_reason(_arg: skb, _arg: nhlen) -> return;
}
// Variant of pskb_inet_may_pull().
//
// Essentially this is skb_protocol(skb, true)
// And we get MAC len.
//

// For ETH_P_IPV6/ETH_P_IP we make sure to pull
// a base network header in skb->head.
//
// Extract dsfield from inner protocol
extern "C" {
    pub fn ipv6_get_dsfield()iph: *const (struct ipv6hdr) -> return;
}
extern "C" {
    pub fn ip6_flowlabel()iph: *const (struct ipv6hdr) -> return;
}
// Propagate ECN bits out
extern "C" {
    pub fn INET_ECN_encapsulate(_arg: tos, _arg: inner) -> return;
}
extern "C" {
    pub fn __iptunnel_pull_header(_arg: skb, _arg: hdr_len, _arg: inner_proto, _arg: false, _arg: xnet) -> return;
}
// we must cap headroom to some upperlimit, else pskb_expand_head
// will overflow header offsets in skb_headers_offset_update().
//
extern "C" {
    pub fn iptunnel_handle_offloads(skb: *mut sk_buff, gso_type_mask: c_int) -> c_int;
}
// Returns > 0 if metadata should be collected
extern "C" {
    pub fn static_branch_unlikely(_arg: &ip_tunnel_metadata_cnt) -> return;
}
extern "C" {
    pub fn ip_tunnel_core_init() -> void __init;
}
extern "C" {
    pub fn ip_tunnel_need_metadata();
}
extern "C" {
    pub fn ip_tunnel_unneed_metadata();
}

