//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ip6_tunnel.h
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

// capable of sending packets
pub const IP6_TNL_F_CAP_XMIT: c_uint = 0x10000;
// capable of receiving packets
pub const IP6_TNL_F_CAP_RCV: c_uint = 0x20000;
// determine capability on a per-packet basis
pub const IP6_TNL_F_CAP_PER_PACKET: c_uint = 0x40000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ip6_tnl_parm {
    pub /: *mut *mut char name[IFNAMSIZ]; / name of tunnel device,
    pub /: *mut *mut int link; / ifindex of underlying L2 interface,
    pub /: *mut *mut __u8 proto; / tunnel protocol,
    pub /: *mut *mut __u8 encap_limit; / encapsulation limit for tunnel,
    pub /: *mut *mut __u8 hop_limit; / hop limit for tunnel,
    pub collect_md: bool,
    pub /: *mut *mut __be32 flowinfo; / traffic class and flowlabel for tunnel,
    pub /: *mut *mut __u32 flags; / tunnel flags,
    pub /: *mut *mut in6_addr laddr; / local tunnel end-point address,
    pub /: *mut *mut in6_addr raddr; / remote tunnel end-point address,
    pub i_key: __be32,
    pub o_key: __be32,
    pub fwmark: __u32,
    pub /: *mut *mut __u32 index; / ERSPAN type II index,
    pub /: *mut *mut __u8 erspan_ver; / ERSPAN version,
    pub /: *mut *mut __u8 dir; / direction,
    pub /: *mut *mut __u16 hwid; / hwid,
}

// IPv6 tunnel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl {
    pub /: *mut *mut *mut ip6_tnl __rcu next; / next tunnel in list,
    pub /: *mut *mut *mut net_device dev; / virtual device associated with tunnel,
    pub dev_tracker: netdevice_tracker,
    pub /: *mut *mut *mut net net; / netns for packet i/o,
    pub /: *mut *mut __ip6_tnl_parm parms; / tunnel configuration parameters,
    pub /: *mut *mut flowi fl; / flowi template for xmit,
    pub /: *mut *mut dst_cache dst_cache; / cached dst,
    pub gro_cells: gro_cells,
    pub err_count: c_int,
    pub err_time: c_ulong,
// These fields used only by GRE
    pub /: *mut *mut __u32 i_seqno; / The last seen seqno,
    pub /: *mut *mut atomic_t o_seqno; / The last output seqno,
    pub /: *mut *mut int hlen; / tun_hlen + encap_hlen,
    pub /: *mut *mut int tun_hlen; / Precalculated header length,
    pub /: *mut *mut int encap_hlen; / Encap header length (FOU,GUE),
    pub encap: ip_tunnel_encap,
    pub mlink: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl_encap_ops {
    pub e): *mut *mut size_t (encap_hlen)(struct ip_tunnel_encap,
    pub fl6): *mut *mut u8 protocol, struct flowi6,
    pub info): u8 type, u8 code, int offset, __be32,
}

// Tunnel encapsulation limit destination sub-option
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_tlv_tnl_enc_lim {
    pub /: *mut *mut __u8 type; / type-code for option,
    pub /: *mut *mut __u8 length; / option length,
    pub /: *mut *mut __u8 encap_limit; / tunnel encapsulation limit,
    pub __packed: },
    pub raddr): *const in6_addr,
    pub log_ecn_error): bool,
    pub raddr): *const in6_addr,
    pub proto): *mut *mut *mut flowi6 fl6, int encap_limit, __u32 pmtu, __u8,
    pub raw): *mut *mut __u16 ip6_tnl_parse_tlv_enc_lim(struct sk_buff skb, __u8,
    pub raddr): *const in6_addr,
    pub dev): *const *const net ip6_tnl_get_link_net(net_device,
    pub dev): *const int ip6_tnl_get_iflink(struct net_device,
    pub new_mtu): *mut *mut int ip6_tnl_change_mtu(struct net_device dev, int,
    pub err: int pkt_len,,
    pub tx_errors): DEV_STATS_INC(dev,,
    pub SKB_DROP_REASON_RECURSION_LIMIT): kfree_skb_reason(skb,,
    pub inet6_skb_parm)): memset(skb->cb, 0, sizeof(struct,
    pub ip6cb_flags: IP6CB(skb)->flags =,
    pub skb_inner_network_offset(skb): pkt_len = skb->len -,
    pub skb): err = ip6_local_out(skb_dst_dev_net(skb), sk,,
    pub -1: pkt_len =,
    pub pkt_len): iptunnel_xmit_stats(dev,,

