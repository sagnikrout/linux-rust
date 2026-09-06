//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/libie/rx.c
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
// Copyright (C) 2024-2025 Intel Corporation

// O(1) converting i40e/ice/iavf's 8/10-bit hardware packet type to a parsed
// bitfield struct.
//
// A few supplementary definitions for when XDP hash types do not coincide
// with what can be generated from ptype definitions by means of preprocessor
// concatenation.
//

    .outer_ip		= LIBETH_RX_PT_OUTER_##oip,	   \
    .outer_frag		= LIBETH_RX_PT_##ofrag,		   \
    .tunnel_type		= LIBETH_RX_PT_TUNNEL_IP_##tun,	   \
    .tunnel_end_prot	= LIBETH_RX_PT_TUNNEL_END_##tp,	   \
    .tunnel_end_frag	= LIBETH_RX_PT_##tefr,		   \
    .inner_prot		= LIBETH_RX_PT_INNER_##iprot,	   \
    .payload_layer		= LIBETH_RX_PT_PAYLOAD_##pl,	   \
    .hash_type		= XDP_RSS_L3_##oip |		   \
    XDP_RSS_L4_##iprot |		   \
    XDP_RSS_TYPE_##pl,		   \
    }

    LIBIE_RX_PT(L2, NOT_FRAG, NONE, NONE, NOT_FRAG, iprot, pl)

    LIBIE_RX_PT(IPV##oip, FRAG, NONE, NONE, NOT_FRAG, NONE, L3)

    LIBIE_RX_PT(IPV##oip, NOT_FRAG, tun, teprot, tefr, NONE, L3)

    LIBIE_RX_PT(IPV##oip, NOT_FRAG, tun, teprot, NOT_FRAG, iprot, L4)

    LIBIE_RX_PT_IP_L3(oip, tun, ver, NOT_FRAG),			   \
    LIBIE_RX_PT_IP_L4(oip, tun, ver, UDP),				   \
    LIBIE_RX_PT_UNUSED,						   \
    LIBIE_RX_PT_IP_L4(oip, tun, ver, TCP),				   \
    LIBIE_RX_PT_IP_L4(oip, tun, ver, SCTP),				   \
    LIBIE_RX_PT_IP_L4(oip, tun, ver, ICMP)
// IPv oip --> tun --> IPv ver

    LIBIE_RX_PT_IP_L3(oip, tun, ver, FRAG),				   \
    LIBIE_RX_PT_IP_NOF(oip, tun, ver)
// Non Tunneled IPv oip

    LIBIE_RX_PT_IP_FRAG(oip),					   \
    LIBIE_RX_PT_IP_NOF(oip, NONE, NONE)
// IPv oip --> tun --> { IPv4, IPv6 }

    LIBIE_RX_PT_IP_TUN_VER(oip, tun, IPV4),				   \
    LIBIE_RX_PT_IP_TUN_VER(oip, tun, IPV6)
// IPv oip --> GRE/NAT tun --> { x, IPv4, IPv6 }

    LIBIE_RX_PT_IP_L3(oip, tun, NONE, NOT_FRAG),			   \
    LIBIE_RX_PT_IP_TUN(oip, tun)
// Non Tunneled IPv oip
// IPv oip --> { IPv4, IPv6 }
// IPv oip --> GRE/NAT --> { x, IPv4, IPv6 }
// IPv oip --> GRE/NAT --> MAC --> { x, IPv4, IPv6 }
// IPv oip --> GRE/NAT --> MAC/VLAN --> { x, IPv4, IPv6 }
//

    LIBIE_RX_PT_IP_RAW(oip),					   \
    LIBIE_RX_PT_IP_TUN(oip, IP),					   \
    LIBIE_RX_PT_IP_GRE(oip, GRENAT),				   \
    LIBIE_RX_PT_IP_GRE(oip, GRENAT_MAC),				   \
    LIBIE_RX_PT_IP_GRE(oip, GRENAT_MAC_VLAN)
// Lookup table mapping for O(1) parsing
    const struct libeth_rx_pt libie_rx_pt_lut[LIBIE_RX_PT_NUM] = {
// L2 packet types
    LIBIE_RX_PT_UNUSED,
    LIBIE_RX_PT_L2,
    LIBIE_RX_PT_TS,
    LIBIE_RX_PT_L2,
    LIBIE_RX_PT_UNUSED,
    LIBIE_RX_PT_UNUSED,
    LIBIE_RX_PT_L2,
    LIBIE_RX_PT_L2,
    LIBIE_RX_PT_UNUSED,
    LIBIE_RX_PT_UNUSED,
    LIBIE_RX_PT_L2,
    LIBIE_RX_PT_UNUSED,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_L3,
    LIBIE_RX_PT_IP(4),
    LIBIE_RX_PT_IP(6),
    };
    EXPORT_SYMBOL_GPL(libie_rx_pt_lut);
    MODULE_DESCRIPTION("Intel(R) Ethernet common library");
    MODULE_IMPORT_NS("LIBETH");
    MODULE_LICENSE("GPL");
