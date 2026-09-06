//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/vxlan.h
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
pub const __NET_VXLAN_H: c_int = 1;

pub const IANA_VXLAN_UDP_PORT: c_int = 4789;
pub const IANA_VXLAN_GPE_UDP_PORT: c_int = 4790;
// VXLAN protocol (RFC 7348) header:
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |R|R|R|R|I|R|R|R|               Reserved                        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                VXLAN Network Identifier (VNI) |   Reserved    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// I = VXLAN Network Identifier (VNI) present.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlanhdr {
    pub vx_flags: __be32,
    pub vx_vni: __be32,
}

// VXLAN header flags.

pub const VNI_HASH_BITS: c_int = 10;

pub const FDB_HASH_BITS: c_int = 8;

// Remote checksum offload for VXLAN (VXLAN_F_REMCSUM_[RT]X):
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |R|R|R|R|I|R|R|R|R|R|C|              Reserved                   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           VXLAN Network Identifier (VNI)      |O| Csum start  |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// C = Remote checksum offload bit. When set indicates that the
// remote checksum offload data is present.
//
// O = Offset bit. Indicates the checksum offset relative to
// checksum start.
//
// Csum start = Checksum start divided by two.
//
// http://tools.ietf.org/html/draft-herbert-vxlan-rco
//
// VXLAN-RCO header flags.

// Remote checksum offload header option

//
// VXLAN Group Based Policy Extension (VXLAN_F_GBP):
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |G|R|R|R|I|R|R|R|R|D|R|R|A|R|R|R|        Group Policy ID        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                VXLAN Network Identifier (VNI) |   Reserved    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// G = Group Policy ID present.
//
// D = Don't Learn bit. When set, this bit indicates that the egress
// VTEP MUST NOT learn the source address of the encapsulated frame.
//
// A = Indicates that the group policy has already been applied to
// this packet. Policies MUST NOT be applied by devices when the
// A bit is set.
//
// https://tools.ietf.org/html/draft-smith-vxlan-group-policy
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlanhdr_gbp {
    pub vx_flags: u8,

    pub policy_id: __be16,
    pub vx_vni: __be32,
}

// VXLAN-GBP header flags.

// skb->mark mapping
//
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |R|R|R|R|R|R|R|R|R|D|R|R|A|R|R|R|        Group Policy ID        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//

//
// VXLAN Generic Protocol Extension (VXLAN_F_GPE):
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |R|R|Ver|I|P|R|O|       Reserved                |Next Protocol  |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                VXLAN Network Identifier (VNI) |   Reserved    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Ver = Version. Indicates VXLAN GPE protocol version.
//
// P = Next Protocol Bit. The P bit is set to indicate that the
// Next Protocol field is present.
//
// O = OAM Flag Bit. The O bit is set to indicate that the packet
// is an OAM packet.
//
// Next Protocol = This 8 bit field indicates the protocol header
// immediately following the VXLAN GPE header.
//
// https://tools.ietf.org/html/draft-ietf-nvo3-vxlan-gpe-01
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlanhdr_gpe {

    pub reserved_flags3: u8,
    pub reserved_flags4: u8,
    pub next_protocol: u8,
    pub vx_vni: __be32,
}

// VXLAN-GPE header flags.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_metadata {
    pub gbp: u32,
}

// per UDP socket information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_sock {
    pub hlist: hlist_node,
    pub sk: *mut sock,
    pub rcu: rcu_head,
    pub vni_list: [hlist_head; VNI_HASH_SIZE],
    pub refcnt: refcount_t,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vxlan_addr {
    pub sin: sockaddr_in,
    pub sin6: sockaddr_in6,
    pub sa: sockaddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_rdst {
    pub remote_ip: vxlan_addr,
    pub remote_port: __be16,
    pub offloaded:1: u8,
    pub remote_vni: __be32,
    pub remote_ifindex: u32,
    pub remote_dev: *mut net_device,
    pub list: list_head,
    pub rcu: rcu_head,
    pub dst_cache: dst_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_config {
    pub remote_ip: vxlan_addr,
    pub saddr: vxlan_addr,
    pub vni: __be32,
    pub remote_ifindex: c_int,
    pub mtu: c_int,
    pub dst_port: __be16,
    pub port_min: u16,
    pub port_max: u16,
    pub tos: u8,
    pub ttl: u8,
    pub label: __be32,
    pub label_policy: ifla_vxlan_label_policy,
    pub flags: u32,
    pub age_interval: c_ulong,
    pub addrmax: c_uint,
    pub no_share: bool,
    pub df: ifla_vxlan_df,
    pub reserved_bits: vxlanhdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_vni_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_drops: u64,
    pub rx_errors: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_drops: u64,
    pub tx_errors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_vni_stats_pcpu {
    pub stats: vxlan_vni_stats,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_dev_node {
    pub hlist: hlist_node,
    pub vxlan: *mut vxlan_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_vni_node {
    pub vnode: rhash_head,
    pub /: *mut *mut vxlan_dev_node hlist4; / vni hash table for IPv4 socket,

    pub /: *mut *mut vxlan_dev_node hlist6; / vni hash table for IPv6 socket,

    pub vlist: list_head,
    pub vni: __be32,
    pub /: *mut *mut vxlan_addr remote_ip; / default remote ip for this vni,
    pub stats: *mut vxlan_vni_stats_pcpu __percpu,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_vni_group {
    pub vni_hash: rhashtable,
    pub vni_list: list_head,
    pub num_vnis: u32,
}

// Pseudo network device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_dev {
    pub /: *mut *mut vxlan_dev_node hlist4; / vni hash table for IPv4 socket,

    pub /: *mut *mut vxlan_dev_node hlist6; / vni hash table for IPv6 socket,

    pub /: *mut *mut list_head next; / vxlan's per namespace list,
    pub /: *mut *mut *mut vxlan_sock __rcu vn4_sock; / listening socket for IPv4,

    pub /: *mut *mut *mut vxlan_sock __rcu vn6_sock; / listening socket for IPv6,

    pub dev: *mut net_device,
    pub /: *mut *mut *mut net net; / netns for packet i/o,
    pub /: *mut *mut vxlan_rdst default_dst; / default destination,
    pub age_timer: timer_list,
    pub hash_lock: spinlock_t,
    pub addrcnt: c_uint,
    pub gro_cells: gro_cells,
    pub cfg: vxlan_config,
    pub vnigrp: *mut vxlan_vni_group __rcu,
    pub fdb_hash_tbl: rhashtable,
    pub mdb_tbl: rhashtable,
    pub fdb_list: hlist_head,
    pub mdb_list: hlist_head,
    pub mdb_seq: c_uint,
}

pub const VXLAN_F_LEARN: c_uint = 0x01;
pub const VXLAN_F_PROXY: c_uint = 0x02;
pub const VXLAN_F_RSC: c_uint = 0x04;
pub const VXLAN_F_L2MISS: c_uint = 0x08;
pub const VXLAN_F_L3MISS: c_uint = 0x10;
pub const VXLAN_F_IPV6: c_uint = 0x20;
pub const VXLAN_F_UDP_ZERO_CSUM_TX: c_uint = 0x40;
pub const VXLAN_F_UDP_ZERO_CSUM6_TX: c_uint = 0x80;
pub const VXLAN_F_UDP_ZERO_CSUM6_RX: c_uint = 0x100;
pub const VXLAN_F_REMCSUM_TX: c_uint = 0x200;
pub const VXLAN_F_REMCSUM_RX: c_uint = 0x400;
pub const VXLAN_F_GBP: c_uint = 0x800;
pub const VXLAN_F_REMCSUM_NOPARTIAL: c_uint = 0x1000;
pub const VXLAN_F_COLLECT_METADATA: c_uint = 0x2000;
pub const VXLAN_F_GPE: c_uint = 0x4000;
pub const VXLAN_F_IPV6_LINKLOCAL: c_uint = 0x8000;
pub const VXLAN_F_TTL_INHERIT: c_uint = 0x10000;
pub const VXLAN_F_VNIFILTER: c_uint = 0x20000;
pub const VXLAN_F_MDB: c_uint = 0x40000;
pub const VXLAN_F_LOCALBYPASS: c_uint = 0x80000;
pub const VXLAN_F_MC_ROUTE: c_uint = 0x100000;
// Flags that are used in the receive path. These flags must match in
// order for a socket to be shareable
//

// Flags that can be set together with VXLAN_F_GPE.

// VXLAN:     IP4/6 header + UDP + VXLAN + Ethernet header
// VXLAN-GPE: IP4/6 header + UDP + VXLAN

extern "C" {
    pub fn ipv6_addr_any(_arg: &ipa->sin6.sin6_addr) -> return;
}
extern "C" {
    pub fn ipv6_addr_is_multicast(_arg: &ipa->sin6.sin6_addr) -> return;
}
extern "C" {
    pub fn ipv4_is_multicast(_arg: ipa->sin.sin_addr.s_addr) -> return;
}

extern "C" {
    pub fn ipv4_is_multicast(_arg: ipa->sin.sin_addr.s_addr) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_notifier_vxlan_fdb_info {
    pub /: *mut *mut switchdev_notifier_info info; / must be first,
    pub remote_ip: vxlan_addr,
    pub remote_port: __be16,
    pub remote_vni: __be32,
    pub remote_ifindex: u32,
    pub eth_addr: [u8; ETH_ALEN],
    pub vni: __be32,
    pub offloaded: bool,
    pub added_by_user: bool,
}

extern "C" {
    pub fn vxlan_fdb_clear_offload(dev: *const net_device, vni: __be32);
}

