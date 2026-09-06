//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dpkg.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2013-2015 Freescale Semiconductor Inc.
//

// Data Path Key Generator API
// Contains initialization APIs and runtime APIs for the Key Generator
//
// Key Generator properties
//
// DPKG_NUM_OF_MASKS - Number of masks per key extraction
//
pub const DPKG_NUM_OF_MASKS: c_int = 4;
//
// DPKG_MAX_NUM_OF_EXTRACTS - Number of extractions per key profile
//
pub const DPKG_MAX_NUM_OF_EXTRACTS: c_int = 10;
//
// enum dpkg_extract_from_hdr_type - Selecting extraction by header types
// @DPKG_FROM_HDR: Extract selected bytes from header, by offset
// @DPKG_FROM_FIELD: Extract selected bytes from header, by offset from field
// @DPKG_FULL_FIELD: Extract a full field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpkg_extract_from_hdr_type {
    DPKG_FROM_HDR = 0,
    DPKG_FROM_FIELD = 1,
    DPKG_FULL_FIELD = 2
}

//
// enum dpkg_extract_type - Enumeration for selecting extraction type
// @DPKG_EXTRACT_FROM_HDR: Extract from the header
// @DPKG_EXTRACT_FROM_DATA: Extract from data not in specific header
// @DPKG_EXTRACT_FROM_PARSE: Extract from parser-result;
// e.g. can be used to extract header existence;
// please refer to 'Parse Result definition' section in the parser BG
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpkg_extract_type {
    DPKG_EXTRACT_FROM_HDR = 0,
    DPKG_EXTRACT_FROM_DATA = 1,
    DPKG_EXTRACT_FROM_PARSE = 3
}

//
// struct dpkg_mask - A structure for defining a single extraction mask
// @mask: Byte mask for the extracted content
// @offset: Offset within the extracted content
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpkg_mask {
    pub mask: u8,
    pub offset: u8,
}

// Protocol fields
// Ethernet fields

// VLAN fields

// IP (generic) fields

// IPV4 fields

// IPV6 fields

// ICMP fields

// IGMP fields

// TCP fields

// UDP fields

// UDP-lite fields

// UDP-encap-ESP fields

// SCTP fields

// DCCP fields

// IPHC fields

// SCTP fields

// L2TPV2 fields

// L2TPV3 fields

// PPP fields

// PPPoE fields

// PPP-Mux fields

// PPP-Mux sub-frame fields

// LLC fields

// NLPID fields

// SNAP fields

// LLC SNAP fields

// ARP fields

// RFC2684 fields

// User defined fields

// Payload fields

// GRE fields

// MINENCAP fields

// IPSEC AH fields

// IPSEC ESP fields

// MPLS fields

// MACSEC fields

// GTP fields

// Supported protocols
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_prot {
    NET_PROT_NONE = 0,
    NET_PROT_PAYLOAD,
    NET_PROT_ETH,
    NET_PROT_VLAN,
    NET_PROT_IPV4,
    NET_PROT_IPV6,
    NET_PROT_IP,
    NET_PROT_TCP,
    NET_PROT_UDP,
    NET_PROT_UDP_LITE,
    NET_PROT_IPHC,
    NET_PROT_SCTP,
    NET_PROT_SCTP_CHUNK_DATA,
    NET_PROT_PPPOE,
    NET_PROT_PPP,
    NET_PROT_PPPMUX,
    NET_PROT_PPPMUX_SUBFRM,
    NET_PROT_L2TPV2,
    NET_PROT_L2TPV3_CTRL,
    NET_PROT_L2TPV3_SESS,
    NET_PROT_LLC,
    NET_PROT_LLC_SNAP,
    NET_PROT_NLPID,
    NET_PROT_SNAP,
    NET_PROT_MPLS,
    NET_PROT_IPSEC_AH,
    NET_PROT_IPSEC_ESP,
    NET_PROT_UDP_ENC_ESP, /* RFC 3948 */
    NET_PROT_MACSEC,
    NET_PROT_GRE,
    NET_PROT_MINENCAP,
    NET_PROT_DCCP,
    NET_PROT_ICMP,
    NET_PROT_IGMP,
    NET_PROT_ARP,
    NET_PROT_CAPWAP_DATA,
    NET_PROT_CAPWAP_CTRL,
    NET_PROT_RFC2684,
    NET_PROT_ICMPV6,
    NET_PROT_FCOE,
    NET_PROT_FIP,
    NET_PROT_ISCSI,
    NET_PROT_GTP,
    NET_PROT_USER_DEFINED_L2,
    NET_PROT_USER_DEFINED_L3,
    NET_PROT_USER_DEFINED_L4,
    NET_PROT_USER_DEFINED_L5,
    NET_PROT_USER_DEFINED_SHIM1,
    NET_PROT_USER_DEFINED_SHIM2,

    NET_PROT_DUMMY_LAST
}

//
// struct dpkg_extract - A structure for defining a single extraction
// @type: Determines how the union below is interpreted:
// DPKG_EXTRACT_FROM_HDR: selects 'from_hdr';
// DPKG_EXTRACT_FROM_DATA: selects 'from_data';
// DPKG_EXTRACT_FROM_PARSE: selects 'from_parse'
// @extract: Selects extraction method
// @extract.from_hdr: Used when 'type = DPKG_EXTRACT_FROM_HDR'
// @extract.from_data: Used when 'type = DPKG_EXTRACT_FROM_DATA'
// @extract.from_parse:  Used when 'type = DPKG_EXTRACT_FROM_PARSE'
// @extract.from_hdr.prot: Any of the supported headers
// @extract.from_hdr.type: Defines the type of header extraction:
// DPKG_FROM_HDR: use size & offset below;
// DPKG_FROM_FIELD: use field, size and offset below;
// DPKG_FULL_FIELD: use field below
// @extract.from_hdr.field: One of the supported fields (NH_FLD_)
// @extract.from_hdr.size: Size in bytes
// @extract.from_hdr.offset: Byte offset
// @extract.from_hdr.hdr_index: Clear for cases not listed below;
// Used for protocols that may have more than a single
// header, 0 indicates an outer header;
// Supported protocols (possible values):
// NET_PROT_VLAN (0, HDR_INDEX_LAST);
// NET_PROT_MPLS (0, 1, HDR_INDEX_LAST);
// NET_PROT_IP(0, HDR_INDEX_LAST);
// NET_PROT_IPv4(0, HDR_INDEX_LAST);
// NET_PROT_IPv6(0, HDR_INDEX_LAST);
// @extract.from_data.size: Size in bytes
// @extract.from_data.offset: Byte offset
// @extract.from_parse.size: Size in bytes
// @extract.from_parse.offset: Byte offset
// @num_of_byte_masks: Defines the number of valid entries in the array below;
// This is	also the number of bytes to be used as masks
// @masks: Masks parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpkg_extract {
    pub type: dpkg_extract_type,
    pub prot: net_prot,
    pub type: dpkg_extract_from_hdr_type,
    pub field: u32,
    pub size: u8,
    pub offset: u8,
    pub hdr_index: u8,
    pub from_hdr: },
    pub size: u8,
    pub offset: u8,
    pub from_data: },
    pub size: u8,
    pub offset: u8,
    pub from_parse: },
    pub extract: },
    pub num_of_byte_masks: u8,
    pub masks: [dpkg_mask; DPKG_NUM_OF_MASKS],
}

//
// struct dpkg_profile_cfg - A structure for defining a full Key Generation
// profile (rule)
// @num_extracts: Defines the number of valid entries in the array below
// @extracts: Array of required extractions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpkg_profile_cfg {
    pub num_extracts: u8,
    pub extracts: [dpkg_extract; DPKG_MAX_NUM_OF_EXTRACTS],
}
