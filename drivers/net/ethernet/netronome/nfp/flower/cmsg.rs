//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/flower/cmsg.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017-2018 Netronome Systems, Inc.

// GRE Tunnel flags

// Compressed HW representation of TCP Flags

pub const NFP_FL_SC_ACT_DROP: c_uint = 0x80000000;
pub const NFP_FL_SC_ACT_USER: c_uint = 0x7D000000;
pub const NFP_FL_SC_ACT_POPV: c_uint = 0x6A000000;
pub const NFP_FL_SC_ACT_NULL: c_uint = 0x00000000;
// The maximum action list size (in bytes) supported by the NFP.
//
pub const NFP_FL_MAX_A_SIZ: c_int = 1216;
pub const NFP_FL_LW_SIZ: c_int = 2;
// Maximum allowed geneve options
pub const NFP_FL_MAX_GENEVE_OPT_ACT: c_int = 32;
pub const NFP_FL_MAX_GENEVE_OPT_CNT: c_int = 64;
pub const NFP_FL_MAX_GENEVE_OPT_KEY: c_int = 32;
pub const NFP_FL_MAX_GENEVE_OPT_KEY_V6: c_int = 8;
// Action opcodes
pub const NFP_FL_ACTION_OPCODE_OUTPUT: c_int = 0;
pub const NFP_FL_ACTION_OPCODE_PUSH_VLAN: c_int = 1;
pub const NFP_FL_ACTION_OPCODE_POP_VLAN: c_int = 2;
pub const NFP_FL_ACTION_OPCODE_PUSH_MPLS: c_int = 3;
pub const NFP_FL_ACTION_OPCODE_POP_MPLS: c_int = 4;
pub const NFP_FL_ACTION_OPCODE_SET_TUNNEL: c_int = 6;
pub const NFP_FL_ACTION_OPCODE_SET_ETHERNET: c_int = 7;
pub const NFP_FL_ACTION_OPCODE_SET_MPLS: c_int = 8;
pub const NFP_FL_ACTION_OPCODE_SET_IPV4_ADDRS: c_int = 9;
pub const NFP_FL_ACTION_OPCODE_SET_IPV4_TTL_TOS: c_int = 10;
pub const NFP_FL_ACTION_OPCODE_SET_IPV6_SRC: c_int = 11;
pub const NFP_FL_ACTION_OPCODE_SET_IPV6_DST: c_int = 12;
pub const NFP_FL_ACTION_OPCODE_SET_IPV6_TC_HL_FL: c_int = 13;
pub const NFP_FL_ACTION_OPCODE_SET_UDP: c_int = 14;
pub const NFP_FL_ACTION_OPCODE_SET_TCP: c_int = 15;
pub const NFP_FL_ACTION_OPCODE_PRE_LAG: c_int = 16;
pub const NFP_FL_ACTION_OPCODE_PRE_TUNNEL: c_int = 17;
pub const NFP_FL_ACTION_OPCODE_METER: c_int = 24;
pub const NFP_FL_ACTION_OPCODE_PUSH_GENEVE: c_int = 26;
pub const NFP_FL_ACTION_OPCODE_NUM: c_int = 32;

// LAG ports
pub const NFP_FL_LAG_OUT: c_uint = 0xC0DE0000;
// Tunnel ports
pub const NFP_FL_PORT_TYPE_TUN: c_uint = 0x50000000;

pub const NFP_FLOWER_WORKQ_MAX_SKBS: c_int = 30000;
// Cmesg reply (empirical) timeout

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_flower_tun_type {
    NFP_FL_TUNNEL_NONE =	0,
    NFP_FL_TUNNEL_GRE =	1,
    NFP_FL_TUNNEL_VXLAN =	2,
    NFP_FL_TUNNEL_GENEVE =	4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_act_head {
    pub jump_id: u8,
    pub len_lw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_set_eth {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
    pub 2]: *mut *mut u8 eth_addr_mask[ETH_ALEN,
    pub 2]: *mut *mut u8 eth_addr_val[ETH_ALEN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_set_ip4_addrs {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
    pub ipv4_src_mask: __be32,
    pub ipv4_src: __be32,
    pub ipv4_dst_mask: __be32,
    pub ipv4_dst: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_set_ip4_ttl_tos {
    pub head: nfp_fl_act_head,
    pub ipv4_ttl_mask: u8,
    pub ipv4_tos_mask: u8,
    pub ipv4_ttl: u8,
    pub ipv4_tos: u8,
    pub reserved: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_set_ipv6_tc_hl_fl {
    pub head: nfp_fl_act_head,
    pub ipv6_tc_mask: u8,
    pub ipv6_hop_limit_mask: u8,
    pub reserved: __be16,
    pub ipv6_tc: u8,
    pub ipv6_hop_limit: u8,
    pub ipv6_label_mask: __be32,
    pub ipv6_label: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_set_ipv6_addr {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
    pub mask: __be32,
    pub exact: __be32,
    pub ipv6: [}; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_set_tport {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
    pub tp_port_mask: [u8; 4],
    pub tp_port_val: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_output {
    pub head: nfp_fl_act_head,
    pub flags: __be16,
    pub port: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_push_vlan {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
    pub vlan_tpid: __be16,
    pub vlan_tci: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_pop_vlan {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_pre_lag {
    pub head: nfp_fl_act_head,
    pub group_id: __be16,
    pub lag_version: [u8; 3],
    pub instance: u8,
}

pub const NFP_FL_PRE_LAG_VER_OFF: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_pre_tunnel {
    pub head: nfp_fl_act_head,
    pub flags: __be16,
    pub ipv4_dst: __be32,
    pub ipv6_dst: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_set_tun {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
    pub __packed: __be64 tun_id,
    pub tun_type_index: __be32,
    pub tun_flags: __be16,
    pub ttl: u8,
    pub tos: u8,
    pub outer_vlan_tpid: __be16,
    pub outer_vlan_tci: __be16,
    pub tun_len: u8,
    pub res2: u8,
    pub tun_proto: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_push_geneve {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
    pub class: __be16,
    pub type: u8,
    pub length: u8,
    pub opt_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_push_mpls {
    pub head: nfp_fl_act_head,
    pub ethtype: __be16,
    pub lse: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_pop_mpls {
    pub head: nfp_fl_act_head,
    pub ethtype: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_set_mpls {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
    pub lse_mask: __be32,
    pub lse: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_meter {
    pub head: nfp_fl_act_head,
    pub reserved: __be16,
    pub meter_id: __be32,
}

// Metadata with L2 (1W/4B)
// ----------------------------------------------------------------
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |    key_type   |    mask_id    | PCP |p|   vlan outermost VID  |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// ^                               ^
// NOTE: |             TCI               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_meta_tci {
    pub nfp_flow_key_layer: u8,
    pub mask_id: u8,
    pub tci: __be16,
}

// Extended metadata for additional key_layers (1W/4B)
// ----------------------------------------------------------------
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                      nfp_flow_key_layer2                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_ext_meta {
    pub nfp_flow_key_layer2: __be32,
}

// Port details (1W/4B)
// ----------------------------------------------------------------
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                         port_ingress                          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_in_port {
    pub in_port: __be32,
}

// L2 details (4W/16B)
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                     mac_addr_dst, 31 - 0                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |      mac_addr_dst, 47 - 32    |     mac_addr_src, 15 - 0      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                     mac_addr_src, 47 - 16                     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |       mpls outermost label            |  TC |B|   reserved  |q|
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_mac_mpls {
    pub mac_dst: [u8; 6],
    pub mac_src: [u8; 6],
    pub mpls_lse: __be32,
}

// VLAN details (2W/8B)
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           outer_tpid          |           outer_tci           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           inner_tpid          |           inner_tci           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_vlan {
    pub outer_tpid: __be16,
    pub outer_tci: __be16,
    pub inner_tpid: __be16,
    pub inner_tci: __be16,
}

// L4 ports (for UDP, TCP, SCTP) (1W/4B)
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |            port_src           |           port_dst            |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_tp_ports {
    pub port_src: __be16,
    pub port_dst: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_ip_ext {
    pub tos: u8,
    pub proto: u8,
    pub ttl: u8,
    pub flags: u8,
}

// L3 IPv4 details (3W/12B)
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |    DSCP   |ECN|   protocol    |      ttl      |     flags     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                        ipv4_addr_src                          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                        ipv4_addr_dst                          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_ipv4 {
    pub ip_ext: nfp_flower_ip_ext,
    pub ipv4_src: __be32,
    pub ipv4_dst: __be32,
}

// L3 IPv6 details (10W/40B)
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |    DSCP   |ECN|   protocol    |      ttl      |     flags     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |   ipv6_exthdr   | res |            ipv6_flow_label            |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src,   31 - 0                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src,  63 - 32                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src,  95 - 64                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src, 127 - 96                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst,   31 - 0                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst,  63 - 32                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst,  95 - 64                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst, 127 - 96                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_ipv6 {
    pub ip_ext: nfp_flower_ip_ext,
    pub ipv6_flow_label_exthdr: __be32,
    pub ipv6_src: in6_addr,
    pub ipv6_dst: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_tun_ipv4 {
    pub src: __be32,
    pub dst: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_tun_ipv6 {
    pub src: in6_addr,
    pub dst: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_tun_ip_ext {
    pub tos: u8,
    pub ttl: u8,
}

// Flow Frame IPv4 UDP TUNNEL --> Tunnel details (4W/16B)
// -----------------------------------------------------------------
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                         ipv4_addr_src                         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                         ipv4_addr_dst                         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           Reserved            |      tos      |      ttl      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                            Reserved                           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                     VNI                       |   Reserved    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_ipv4_udp_tun {
    pub ipv4: nfp_flower_tun_ipv4,
    pub reserved1: __be16,
    pub ip_ext: nfp_flower_tun_ip_ext,
    pub reserved2: __be32,
    pub tun_id: __be32,
}

// Flow Frame IPv6 UDP TUNNEL --> Tunnel details (11W/44B)
// -----------------------------------------------------------------
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src,   31 - 0                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src,  63 - 32                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src,  95 - 64                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src, 127 - 96                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst,   31 - 0                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst,  63 - 32                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst,  95 - 64                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst, 127 - 96                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           Reserved            |      tos      |      ttl      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                            Reserved                           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                     VNI                       |   Reserved    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_ipv6_udp_tun {
    pub ipv6: nfp_flower_tun_ipv6,
    pub reserved1: __be16,
    pub ip_ext: nfp_flower_tun_ip_ext,
    pub reserved2: __be32,
    pub tun_id: __be32,
}

// Flow Frame GRE TUNNEL --> Tunnel details (6W/24B)
// -----------------------------------------------------------------
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                         ipv4_addr_src                         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                         ipv4_addr_dst                         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           tun_flags           |       tos     |       ttl     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |            Reserved           |           Ethertype           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                              Key                              |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                           Reserved                            |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_ipv4_gre_tun {
    pub ipv4: nfp_flower_tun_ipv4,
    pub tun_flags: __be16,
    pub ip_ext: nfp_flower_tun_ip_ext,
    pub reserved1: __be16,
    pub ethertype: __be16,
    pub tun_key: __be32,
    pub reserved2: __be32,
}

// Flow Frame GRE TUNNEL V6 --> Tunnel details (12W/48B)
// -----------------------------------------------------------------
// 3                   2                   1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src,   31 - 0                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src,  63 - 32                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src,  95 - 64                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_src, 127 - 96                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst,   31 - 0                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst,  63 - 32                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst,  95 - 64                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  ipv6_addr_dst, 127 - 96                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           tun_flags           |       tos     |       ttl     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |            Reserved           |           Ethertype           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                              Key                              |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                           Reserved                            |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_ipv6_gre_tun {
    pub ipv6: nfp_flower_tun_ipv6,
    pub tun_flags: __be16,
    pub ip_ext: nfp_flower_tun_ip_ext,
    pub reserved1: __be16,
    pub ethertype: __be16,
    pub tun_key: __be32,
    pub reserved2: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_geneve_options {
    pub data: [u8; NFP_FL_MAX_GENEVE_OPT_KEY],
}

pub const NFP_FL_TUN_VNI_OFFSET: c_int = 8;
// The base header for a control message packet.
// Defines an 8-bit version, and an 8-bit type, padded
// to a 32-bit word. Rest of the packet is type-specific.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_cmsg_hdr {
    pub pad: __be16,
    pub type: u8,
    pub version: u8,
}

pub const NFP_FLOWER_CMSG_VER1: c_int = 1;
// Types defined for port related control messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_flower_cmsg_type_port {
    NFP_FLOWER_CMSG_TYPE_FLOW_ADD =		0,
    NFP_FLOWER_CMSG_TYPE_FLOW_MOD =		1,
    NFP_FLOWER_CMSG_TYPE_FLOW_DEL =		2,
    NFP_FLOWER_CMSG_TYPE_LAG_CONFIG =	4,
    NFP_FLOWER_CMSG_TYPE_PORT_REIFY =	6,
    NFP_FLOWER_CMSG_TYPE_MAC_REPR =		7,
    NFP_FLOWER_CMSG_TYPE_PORT_MOD =		8,
    NFP_FLOWER_CMSG_TYPE_MERGE_HINT =	9,
    NFP_FLOWER_CMSG_TYPE_NO_NEIGH =		10,
    NFP_FLOWER_CMSG_TYPE_TUN_MAC =		11,
    NFP_FLOWER_CMSG_TYPE_ACTIVE_TUNS =	12,
    NFP_FLOWER_CMSG_TYPE_TUN_NEIGH =	13,
    NFP_FLOWER_CMSG_TYPE_TUN_IPS =		14,
    NFP_FLOWER_CMSG_TYPE_FLOW_STATS =	15,
    NFP_FLOWER_CMSG_TYPE_PORT_ECHO =	16,
    NFP_FLOWER_CMSG_TYPE_QOS_MOD =		18,
    NFP_FLOWER_CMSG_TYPE_QOS_DEL =		19,
    NFP_FLOWER_CMSG_TYPE_QOS_STATS =	20,
    NFP_FLOWER_CMSG_TYPE_PRE_TUN_RULE =	21,
    NFP_FLOWER_CMSG_TYPE_TUN_IPS_V6 =	22,
    NFP_FLOWER_CMSG_TYPE_NO_NEIGH_V6 =	23,
    NFP_FLOWER_CMSG_TYPE_TUN_NEIGH_V6 =	24,
    NFP_FLOWER_CMSG_TYPE_ACTIVE_TUNS_V6 =	25,
    NFP_FLOWER_CMSG_TYPE_MAX =		32,
}

// NFP_FLOWER_CMSG_TYPE_MAC_REPR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_cmsg_mac_repr {
    pub reserved: [u8; 3],
    pub num_ports: u8,
    pub idx: u8,
    pub info: u8,
    pub nbi_port: u8,
    pub phys_port: u8,
    pub ports: [}; ],
}

// NFP_FLOWER_CMSG_TYPE_PORT_MOD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_cmsg_portmod {
    pub portnum: __be32,
    pub reserved: u8,
    pub info: u8,
    pub mtu: __be16,
}

// NFP_FLOWER_CMSG_TYPE_PORT_REIFY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_cmsg_portreify {
    pub portnum: __be32,
    pub reserved: u16,
    pub info: __be16,
}

// NFP_FLOWER_CMSG_TYPE_FLOW_MERGE_HINT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_cmsg_merge_hint {
    pub reserved: [u8; 3],
    pub count: u8,
    pub host_ctx: __be32,
    pub host_cookie: __be64,
    pub flow: [} __packed; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_flower_cmsg_port_type {
    NFP_FLOWER_CMSG_PORT_TYPE_UNSPEC =	0x0,
    NFP_FLOWER_CMSG_PORT_TYPE_PHYS_PORT =	0x1,
    NFP_FLOWER_CMSG_PORT_TYPE_PCIE_PORT =	0x2,
    NFP_FLOWER_CMSG_PORT_TYPE_OTHER_PORT =  0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_flower_cmsg_port_vnic_type {
    NFP_FLOWER_CMSG_PORT_VNIC_TYPE_VF =	0x0,
    NFP_FLOWER_CMSG_PORT_VNIC_TYPE_PF =	0x1,
    NFP_FLOWER_CMSG_PORT_VNIC_TYPE_CTRL =	0x2,
}

extern "C" {
    pub fn nfp_flower_cmsg_portreify(repr: *mut nfp_repr, exists: bool) -> c_int;
}
extern "C" {
    pub fn nfp_flower_cmsg_process_rx(work: *mut work_struct);
}
extern "C" {
    pub fn nfp_flower_cmsg_rx(app: *mut nfp_app, skb: *mut sk_buff);
}
