//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/filter.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2005-2013 Solarflare Communications Inc.
//

//
// enum efx_filter_match_flags - Flags for hardware filter match type
// @EFX_FILTER_MATCH_REM_HOST: Match by remote IP host address
// @EFX_FILTER_MATCH_LOC_HOST: Match by local IP host address
// @EFX_FILTER_MATCH_REM_MAC: Match by remote MAC address
// @EFX_FILTER_MATCH_REM_PORT: Match by remote TCP/UDP port
// @EFX_FILTER_MATCH_LOC_MAC: Match by local MAC address
// @EFX_FILTER_MATCH_LOC_PORT: Match by local TCP/UDP port
// @EFX_FILTER_MATCH_ETHER_TYPE: Match by Ether-type
// @EFX_FILTER_MATCH_INNER_VID: Match by inner VLAN ID
// @EFX_FILTER_MATCH_OUTER_VID: Match by outer VLAN ID
// @EFX_FILTER_MATCH_IP_PROTO: Match by IP transport protocol
// @EFX_FILTER_MATCH_LOC_MAC_IG: Match by local MAC address I/G bit.
// @EFX_FILTER_MATCH_ENCAP_TYPE: Match by encapsulation type.
// Used for RX default unicast and multicast/broadcast filters.
//
// Only some combinations are supported, depending on NIC type:
//
// - Huntington supports filter matching controlled by firmware, potentially
// using {TCP,UDP}/IPv{4,6} 4-tuple or local 2-tuple, local MAC or I/G bit,
// with or without outer and inner VID
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_filter_match_flags {
    EFX_FILTER_MATCH_REM_HOST =	0x0001,
    EFX_FILTER_MATCH_LOC_HOST =	0x0002,
    EFX_FILTER_MATCH_REM_MAC =	0x0004,
    EFX_FILTER_MATCH_REM_PORT =	0x0008,
    EFX_FILTER_MATCH_LOC_MAC =	0x0010,
    EFX_FILTER_MATCH_LOC_PORT =	0x0020,
    EFX_FILTER_MATCH_ETHER_TYPE =	0x0040,
    EFX_FILTER_MATCH_INNER_VID =	0x0080,
    EFX_FILTER_MATCH_OUTER_VID =	0x0100,
    EFX_FILTER_MATCH_IP_PROTO =	0x0200,
    EFX_FILTER_MATCH_LOC_MAC_IG =	0x0400,
    EFX_FILTER_MATCH_ENCAP_TYPE =	0x0800,
}

//
// enum efx_filter_priority - priority of a hardware filter specification
// @EFX_FILTER_PRI_HINT: Performance hint
// @EFX_FILTER_PRI_AUTO: Automatic filter based on device address list
// or hardware requirements.  This may only be used by the filter
// implementation for each NIC type.
// @EFX_FILTER_PRI_MANUAL: Manually configured filter
// @EFX_FILTER_PRI_REQUIRED: Required for correct behaviour (user-level
// networking and SR-IOV)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_filter_priority {
    EFX_FILTER_PRI_HINT = 0,
    EFX_FILTER_PRI_AUTO,
    EFX_FILTER_PRI_MANUAL,
    EFX_FILTER_PRI_REQUIRED,
}

//
// enum efx_filter_flags - flags for hardware filter specifications
// @EFX_FILTER_FLAG_RX_RSS: Use RSS to spread across multiple queues.
// By default, matching packets will be delivered only to the
// specified queue. If this flag is set, they will be delivered
// to a range of queues offset from the specified queue number
// according to the indirection table.
// @EFX_FILTER_FLAG_RX_SCATTER: Enable DMA scatter on the receiving
// queue.
// @EFX_FILTER_FLAG_RX_OVER_AUTO: Indicates a filter that is
// overriding an automatic filter (priority
// %EFX_FILTER_PRI_AUTO).  This may only be set by the filter
// implementation for each type.  A removal request will restore
// the automatic filter in its place.
// @EFX_FILTER_FLAG_RX: Filter is for RX
// @EFX_FILTER_FLAG_TX: Filter is for TX
// @EFX_FILTER_FLAG_VPORT_ID: Virtual port ID for adapter switching.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_filter_flags {
    EFX_FILTER_FLAG_RX_RSS = 0x01,
    EFX_FILTER_FLAG_RX_SCATTER = 0x02,
    EFX_FILTER_FLAG_RX_OVER_AUTO = 0x04,
    EFX_FILTER_FLAG_RX = 0x08,
    EFX_FILTER_FLAG_TX = 0x10,
    EFX_FILTER_FLAG_VPORT_ID = 0x20,
}

// enum efx_encap_type - types of encapsulation
// @EFX_ENCAP_TYPE_NONE: no encapsulation
// @EFX_ENCAP_TYPE_VXLAN: VXLAN encapsulation
// @EFX_ENCAP_TYPE_NVGRE: NVGRE encapsulation
// @EFX_ENCAP_TYPE_GENEVE: GENEVE encapsulation
// @EFX_ENCAP_FLAG_IPV6: indicates IPv6 outer frame
//
// Contains both enumerated types and flags.
// To get just the type, OR with @EFX_ENCAP_TYPES_MASK.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_encap_type {
    EFX_ENCAP_TYPE_NONE = 0,
    EFX_ENCAP_TYPE_VXLAN = 1,
    EFX_ENCAP_TYPE_NVGRE = 2,
    EFX_ENCAP_TYPE_GENEVE = 3,

    EFX_ENCAP_TYPES_MASK = 7,
    EFX_ENCAP_FLAG_IPV6 = 8,
}

//
// struct efx_filter_spec - specification for a hardware filter
// @match_flags: Match type flags, from &enum efx_filter_match_flags
// @priority: Priority of the filter, from &enum efx_filter_priority
// @flags: Miscellaneous flags, from &enum efx_filter_flags
// @rss_context: RSS context to use, if %EFX_FILTER_FLAG_RX_RSS is set.  This
// is a user_id (with 0 meaning the driver/default RSS context), not an
// MCFW context_id.
// @dmaq_id: Source/target queue index, or %EFX_FILTER_RX_DMAQ_ID_DROP for
// an RX drop filter
// @vport_id: Virtual port ID associated with RX queue, for adapter switching,
// if %EFX_FILTER_FLAG_VPORT_ID is set.  This is an MCFW vport_id, or on
// EF100 an mport selector.
// @outer_vid: Outer VLAN ID to match, if %EFX_FILTER_MATCH_OUTER_VID is set
// @inner_vid: Inner VLAN ID to match, if %EFX_FILTER_MATCH_INNER_VID is set
// @loc_mac: Local MAC address to match, if %EFX_FILTER_MATCH_LOC_MAC or
// %EFX_FILTER_MATCH_LOC_MAC_IG is set
// @rem_mac: Remote MAC address to match, if %EFX_FILTER_MATCH_REM_MAC is set
// @ether_type: Ether-type to match, if %EFX_FILTER_MATCH_ETHER_TYPE is set
// @ip_proto: IP transport protocol to match, if %EFX_FILTER_MATCH_IP_PROTO
// is set
// @loc_host: Local IP host to match, if %EFX_FILTER_MATCH_LOC_HOST is set
// @rem_host: Remote IP host to match, if %EFX_FILTER_MATCH_REM_HOST is set
// @loc_port: Local TCP/UDP port to match, if %EFX_FILTER_MATCH_LOC_PORT is set
// @rem_port: Remote TCP/UDP port to match, if %EFX_FILTER_MATCH_REM_PORT is set
// @encap_type: Encapsulation type to match (from &enum efx_encap_type), if
// %EFX_FILTER_MATCH_ENCAP_TYPE is set
//
// The efx_filter_init_rx() or efx_filter_init_tx() function *must* be
// used to initialise the structure.  The efx_filter_set_*() functions
// may then be used to set @rss_context, @match_flags and related
// fields.
//
// The @priority field is used by software to determine whether a new
// filter may replace an old one.  The hardware priority of a filter
// depends on which fields are matched.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_filter_spec {
    pub match_flags:12: u32,
    pub priority:2: u32,
    pub flags:6: u32,
    pub dmaq_id:12: u32,
    pub rss_context: u32,
    pub vport_id: u32,
    pub outer_vid: __be16,
    pub inner_vid: __be16,
    pub loc_mac: [u8; ETH_ALEN],
    pub rem_mac: [u8; ETH_ALEN],
    pub ether_type: __be16,
    pub ip_proto: u8,
    pub loc_host: [__be32; 4],
    pub rem_host: [__be32; 4],
    pub loc_port: __be16,
    pub rem_port: __be16,
    pub encap_type:4: u32,
// total 65 bytes
}

//
// efx_filter_set_ipv4_local - specify IPv4 host, transport protocol and port
// @spec: Specification to initialise
// @proto: Transport layer protocol number
// @host: Local host address (network byte order)
// @port: Local port (network byte order)
//
// efx_filter_set_ipv6_local - specify IPv6 host, transport protocol and port
// @spec: Specification to initialise
// @proto: Transport layer protocol number
// @host: Local host address (network byte order)
// @port: Local port (network byte order)
//
// efx_filter_set_ipv4_full - specify IPv4 hosts, transport protocol and ports
// @spec: Specification to initialise
// @proto: Transport layer protocol number
// @lhost: Local host address (network byte order)
// @lport: Local port (network byte order)
// @rhost: Remote host address (network byte order)
// @rport: Remote port (network byte order)
//
// efx_filter_set_eth_local - specify local Ethernet address and/or VID
// @spec: Specification to initialise
// @vid: Outer VLAN ID to match, or %EFX_FILTER_VID_UNSPEC
// @addr: Local Ethernet MAC address, or %NULL
//
// efx_filter_set_uc_def - specify matching otherwise-unmatched unicast
// @spec: Specification to initialise
//
// efx_filter_set_mc_def - specify matching otherwise-unmatched multicast
// @spec: Specification to initialise
//
// efx_filter_set_vport_id - override virtual port id relating to filter
// @spec: Specification to initialise
// @vport_id: firmware ID of the virtual port
//
