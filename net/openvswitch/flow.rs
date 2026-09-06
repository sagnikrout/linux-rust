//! Automatically rewritten from C Header to Rust Module
//! Source: net/openvswitch/flow.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2007-2017 Nicira, Inc.
//
pub const FLOW_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sw_flow_mac_proto {
    MAC_PROTO_NONE = 0,
    MAC_PROTO_ETHERNET,
}

pub const SW_FLOW_KEY_INVALID: c_uint = 0x80;
pub const MPLS_LABEL_DEPTH: c_int = 3;
// Bit definitions for IPv6 Extension Header pseudo-field.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ofp12_ipv6exthdr_flags {
    OFPIEH12_NONEXT = 1 << 0,   /* "No next header" encountered. */
    OFPIEH12_ESP    = 1 << 1,   /* Encrypted Sec Payload header present. */
    OFPIEH12_AUTH   = 1 << 2,   /* Authentication header present. */
    OFPIEH12_DEST   = 1 << 3,   /* 1 or 2 dest headers present. */
    OFPIEH12_FRAG   = 1 << 4,   /* Fragment header present. */
    OFPIEH12_ROUTER = 1 << 5,   /* Router header present. */
    OFPIEH12_HOP    = 1 << 6,   /* Hop-by-hop header present. */
    OFPIEH12_UNREP  = 1 << 7,   /* Unexpected repeats encountered. */
    OFPIEH12_UNSEQ  = 1 << 8    /* Unexpected sequencing encountered. */
}

// Store options at the end of the array if they are less than the
// maximum size. This allows us to get the benefits of variable length
// matching for small options.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovs_tunnel_info {
    pub tun_dst: *mut metadata_dst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_head {
    pub 802.1ad.*/: *mut *mut __be16 tpid; / Vlan type. Generally 802.1q or,
    pub /: *mut *mut __be16 tci; / 0 if no VLAN, VLAN_CFI_MASK set otherwise.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovs_key_nsh {
    pub base: ovs_nsh_key_base,
    pub context: [__be32; NSH_MD1_CONTEXT_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_flow_key {
    pub tun_opts: [u8; IP_TUNNEL_OPTS_MAX],
    pub tun_opts_len: u8,
    pub /: *mut *mut ip_tunnel_key tun_key; / Encapsulating tunnel key.,
    pub /: *mut *mut u32 priority; / Packet QoS priority.,
    pub /: *mut *mut u32 skb_mark; / SKB mark.,
    pub /: *mut *mut u16 in_port; / Input switch port (or DP_MAX_PORTS).,
    pub /: *mut *mut } __packed phy; / Safe when right after 'tun_key'.,
    pub /: *mut *mut u8 mac_proto; / MAC layer protocol (e.g. Ethernet).,
    pub /: *mut *mut u8 tun_proto; / Protocol of encapsulating tunnel.,
    pub /: *mut *mut u32 ovs_flow_hash; / Datapath computed hash value.,
    pub /: *mut *mut u32 recirc_id; / Recirculation ID.,
    pub /: *mut *mut u8 src[ETH_ALEN]; / Ethernet source address.,
    pub /: *mut *mut u8 dst[ETH_ALEN]; / Ethernet destination address.,
    pub vlan: vlan_head,
    pub cvlan: vlan_head,
    pub /: *mut *mut __be16 type; / Ethernet frame type.,
    pub eth: },
// Filling a hole of two bytes.
    pub ct_state: u8,
    pub IP: *mut *mut u8 ct_orig_proto; / CT original direction tuple,
// protocol.
//
    pub /: *mut *mut u8 proto; / IP protocol or lower 8 bits of ARP opcode.,
    pub /: *mut *mut u8 tos; / IP ToS.,
    pub /: *mut *mut u8 ttl; / IP TTL/hop limit.,
    pub /: *mut *mut *mut u8 frag; / One of OVS_FRAG_TYPE_.,
    pub ip: },
}

// Connection tracking fields not packed above.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_flow_key_range {
    pub start: unsigned short int,
    pub end: unsigned short int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_flow_mask {
    pub ref_count: c_int,
    pub rcu: rcu_head,
    pub range: sw_flow_key_range,
    pub key: sw_flow_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_flow_match {
    pub key: *mut sw_flow_key,
    pub range: sw_flow_key_range,
    pub mask: *mut sw_flow_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_flow_id {
    pub ufid_len: u32,
    pub 4]: u32 ufid[MAX_UFID_LENGTH /,
    pub unmasked_key: *mut sw_flow_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_flow_actions {
    pub rcu: rcu_head,
    pub /: *mut *mut size_t orig_len; / From flow_cmd_new netlink actions size,
    pub actions_len: u32,
    pub actions: [nlattr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_flow_stats {
    pub /: *mut *mut u64 packet_count; / Number of packets matched.,
    pub /: *mut *mut u64 byte_count; / Number of bytes matched.,
    pub /: *mut *mut unsigned long used; / Last used time (in jiffies).,
    pub /: *mut *mut spinlock_t lock; / Lock for atomic stats update.,
    pub /: *mut *mut __be16 tcp_flags; / Union of seen TCP flags.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_flow {
    pub rcu: rcu_head,
    pub node: [hlist_node; 2],
    pub hash: u32,
    pub ufid_table: } flow_table,,
    pub on: *mut *mut int stats_last_writer; / CPU id of the last writer,
// 'stats[0]'.
//
    pub key: sw_flow_key,
    pub id: sw_flow_id,
    pub cpu_used_mask: *mut cpumask,
    pub mask: *mut sw_flow_mask,
    pub sf_acts: *mut sw_flow_actions __rcu,
    pub one: *mut *mut *mut sw_flow_stats __rcu stats[]; / One for each CPU. First,
// is allocated at flow creation time,
// the rest are allocated on demand
// while holding the 'stats[0].lock'.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arp_eth_header {
    pub /: *mut *mut __be16 ar_hrd; / format of hardware address,
    pub /: *mut *mut __be16 ar_pro; / format of protocol address,
    pub /: *mut *mut unsigned char ar_hln; / length of hardware address,
    pub /: *mut *mut unsigned char ar_pln; / length of protocol address,
    pub /: *mut *mut __be16 ar_op; / ARP opcode (command),
// Ethernet+IPv4 specific members.
    pub /: *mut *mut unsigned char ar_sha[ETH_ALEN]; / sender hardware address,
    pub /: *mut *mut unsigned char ar_sip[4]; / sender IP address,
    pub /: *mut *mut unsigned char ar_tha[ETH_ALEN]; / target hardware address,
    pub /: *mut *mut unsigned char ar_tip[4]; / target IP address,
    pub __packed: },
    pub ~SW_FLOW_KEY_INVALID: return key->mac_proto &,
    pub 0: return mac_proto == MAC_PROTO_ETHERNET ? ETH_HLEN :,
    pub __ovs_mac_header_len(ovs_key_mac_proto(key)): return,
    pub sfid->ufid_len: return,
    pub !ovs_identifier_is_ufid(sfid): return,
    pub ): *const sk_buff,
    pub tcp_flags): *mut *mut unsigned long used, __be16,
    pub ): *mut void ovs_flow_stats_clear(struct sw_flow,
    pub flow_jiffies): u64 ovs_flow_used_time(unsigned long,
    pub key): *mut *mut int ovs_flow_key_update(struct sk_buff skb, struct sw_flow_key,
    pub key): *mut *mut int ovs_flow_key_update_l3l4(struct sk_buff skb, struct sw_flow_key,
    pub key): *mut sw_flow_key,
// Extract key from packet coming from userspace.
    pub log): *mut *mut sw_flow_key key, bool,
