//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_vcap_impl.h
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver VCAP implementation
//
// Copyright (c) 2022 Microchip Technology Inc. and its subsidiaries.
//
// The Sparx5 Chip Register Model can be browsed at this location:
// https://github.com/microchip-ung/sparx-5_reginfo
//

pub const SPARX5_IS2_LOOKUPS: c_int = 4;
pub const SPARX5_IS0_LOOKUPS: c_int = 6;
pub const SPARX5_ES0_LOOKUPS: c_int = 1;
pub const SPARX5_ES2_LOOKUPS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_vcap_inst {
    pub /: *mut *mut vcap_type vtype; / type of vcap,
    pub /: *mut *mut int vinst; / instance number within the same type,
    pub /: *mut *mut int lookups; / number of lookups in this vcap type,
    pub /: *mut *mut int lookups_per_instance; / number of lookups in this instance,
    pub /: *mut *mut int first_cid; / first chain id in this vcap,
    pub /: *mut *mut int last_cid; / last chain id in this vcap,
    pub /: *mut *mut int count; / number of available addresses, not in super vcap,
    pub /: *mut *mut int map_id; / id in the super vcap block mapping (if applicable),
    pub /: *mut *mut int blockno; / starting block in super vcap (if applicable),
    pub /: *mut *mut int blocks; / number of blocks in super vcap (if applicable),
    pub /: *mut *mut bool ingress; / is vcap in the ingress path,
}

// IS0 port keyset selection control
// IS0 ethernet, IPv4, IPv6 traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is0_port_sel_etype {
    VCAP_IS0_PS_ETYPE_DEFAULT, /* None or follow depending on class */
    VCAP_IS0_PS_ETYPE_MLL,
    VCAP_IS0_PS_ETYPE_SGL_MLBS,
    VCAP_IS0_PS_ETYPE_DBL_MLBS,
    VCAP_IS0_PS_ETYPE_TRI_MLBS,
    VCAP_IS0_PS_ETYPE_TRI_VID,
    VCAP_IS0_PS_ETYPE_LL_FULL,
    VCAP_IS0_PS_ETYPE_NORMAL_SRC,
    VCAP_IS0_PS_ETYPE_NORMAL_DST,
    VCAP_IS0_PS_ETYPE_NORMAL_7TUPLE,
    VCAP_IS0_PS_ETYPE_NORMAL_5TUPLE_IP4,
    VCAP_IS0_PS_ETYPE_PURE_5TUPLE_IP4,
    VCAP_IS0_PS_ETYPE_DBL_VID_IDX,
    VCAP_IS0_PS_ETYPE_ETAG,
    VCAP_IS0_PS_ETYPE_NO_LOOKUP,
}

// IS0 MPLS traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is0_port_sel_mpls_uc_mc {
    VCAP_IS0_PS_MPLS_FOLLOW_ETYPE,
    VCAP_IS0_PS_MPLS_MLL,
    VCAP_IS0_PS_MPLS_SGL_MLBS,
    VCAP_IS0_PS_MPLS_DBL_MLBS,
    VCAP_IS0_PS_MPLS_TRI_MLBS,
    VCAP_IS0_PS_MPLS_TRI_VID,
    VCAP_IS0_PS_MPLS_LL_FULL,
    VCAP_IS0_PS_MPLS_NORMAL_SRC,
    VCAP_IS0_PS_MPLS_NORMAL_DST,
    VCAP_IS0_PS_MPLS_NORMAL_7TUPLE,
    VCAP_IS0_PS_MPLS_NORMAL_5TUPLE_IP4,
    VCAP_IS0_PS_MPLS_PURE_5TUPLE_IP4,
    VCAP_IS0_PS_MPLS_DBL_VID_IDX,
    VCAP_IS0_PS_MPLS_ETAG,
    VCAP_IS0_PS_MPLS_NO_LOOKUP,
}

// IS0 MBLS traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is0_port_sel_mlbs {
    VCAP_IS0_PS_MLBS_FOLLOW_ETYPE,
    VCAP_IS0_PS_MLBS_SGL_MLBS,
    VCAP_IS0_PS_MLBS_DBL_MLBS,
    VCAP_IS0_PS_MLBS_TRI_MLBS,
    VCAP_IS0_PS_MLBS_NO_LOOKUP = 17,
}

// IS2 port keyset selection control
// IS2 non-ethernet traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is2_port_sel_noneth {
    VCAP_IS2_PS_NONETH_MAC_ETYPE,
    VCAP_IS2_PS_NONETH_CUSTOM_1,
    VCAP_IS2_PS_NONETH_CUSTOM_2,
    VCAP_IS2_PS_NONETH_NO_LOOKUP
}

// IS2 IPv4 unicast traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is2_port_sel_ipv4_uc {
    VCAP_IS2_PS_IPV4_UC_MAC_ETYPE,
    VCAP_IS2_PS_IPV4_UC_IP4_TCP_UDP_OTHER,
    VCAP_IS2_PS_IPV4_UC_IP_7TUPLE,
}

// IS2 IPv4 multicast traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is2_port_sel_ipv4_mc {
    VCAP_IS2_PS_IPV4_MC_MAC_ETYPE,
    VCAP_IS2_PS_IPV4_MC_IP4_TCP_UDP_OTHER,
    VCAP_IS2_PS_IPV4_MC_IP_7TUPLE,
    VCAP_IS2_PS_IPV4_MC_IP4_VID,
}

// IS2 IPv6 unicast traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is2_port_sel_ipv6_uc {
    VCAP_IS2_PS_IPV6_UC_MAC_ETYPE,
    VCAP_IS2_PS_IPV6_UC_IP_7TUPLE,
    VCAP_IS2_PS_IPV6_UC_IP6_STD,
    VCAP_IS2_PS_IPV6_UC_IP4_TCP_UDP_OTHER,
}

// IS2 IPv6 multicast traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is2_port_sel_ipv6_mc {
    VCAP_IS2_PS_IPV6_MC_MAC_ETYPE,
    VCAP_IS2_PS_IPV6_MC_IP_7TUPLE,
    VCAP_IS2_PS_IPV6_MC_IP6_VID,
    VCAP_IS2_PS_IPV6_MC_IP6_STD,
    VCAP_IS2_PS_IPV6_MC_IP4_TCP_UDP_OTHER,
}

// IS2 ARP traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is2_port_sel_arp {
    VCAP_IS2_PS_ARP_MAC_ETYPE,
    VCAP_IS2_PS_ARP_ARP,
}

// ES0 port keyset selection control
// ES0 Egress port traffic type classification
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_es0_port_sel {
    VCAP_ES0_PS_NORMAL_SELECTION,
    VCAP_ES0_PS_FORCE_ISDX_LOOKUPS,
    VCAP_ES0_PS_FORCE_VID_LOOKUPS,
    VCAP_ES0_PS_RESERVED,
}

// ES2 port keyset selection control
// ES2 IPv4 traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_es2_port_sel_ipv4 {
    VCAP_ES2_PS_IPV4_MAC_ETYPE,
    VCAP_ES2_PS_IPV4_IP_7TUPLE,
    VCAP_ES2_PS_IPV4_IP4_TCP_UDP_VID,
    VCAP_ES2_PS_IPV4_IP4_TCP_UDP_OTHER,
    VCAP_ES2_PS_IPV4_IP4_VID,
    VCAP_ES2_PS_IPV4_IP4_OTHER,
}

// ES2 IPv6 traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_es2_port_sel_ipv6 {
    VCAP_ES2_PS_IPV6_MAC_ETYPE,
    VCAP_ES2_PS_IPV6_IP_7TUPLE,
    VCAP_ES2_PS_IPV6_IP_7TUPLE_VID,
    VCAP_ES2_PS_IPV6_IP_7TUPLE_STD,
    VCAP_ES2_PS_IPV6_IP6_VID,
    VCAP_ES2_PS_IPV6_IP6_STD,
    VCAP_ES2_PS_IPV6_IP4_DOWNGRADE,
}

// ES2 ARP traffic type keyset generation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_es2_port_sel_arp {
    VCAP_ES2_PS_ARP_MAC_ETYPE,
    VCAP_ES2_PS_ARP_ARP,
}

// Selects TPID for ES0 matching
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_TPID_SEL {
    SPX5_TPID_SEL_UNTAGGED,
    SPX5_TPID_SEL_8100,
    SPX5_TPID_SEL_UNUSED_0,
    SPX5_TPID_SEL_UNUSED_1,
    SPX5_TPID_SEL_88A8,
    SPX5_TPID_SEL_TPIDCFG_1,
    SPX5_TPID_SEL_TPIDCFG_2,
    SPX5_TPID_SEL_TPIDCFG_3,
}

// Get the port keyset for the vcap lookup
// Change the port keyset for the lookup and protocol
// Check if the ethertype is supported by the vcap port classification
extern "C" {
    pub fn sparx5_vcap_is_known_etype(admin: *mut vcap_admin, etype: u16) -> bool;
}
