//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_flow.h
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
// Copyright (c) 2019, Intel Corporation.

pub const ICE_FLOW_ENTRY_HANDLE_INVAL: c_int = 0;
pub const ICE_FLOW_FLD_OFF_INVAL: c_uint = 0xffff;
// Generate flow hash field from flow field type(s)

pub const ICE_HASH_INVALID: c_int = 0;

pub const ICE_FLOW_FIELD_IPV4_SRC_OFFSET: c_int = 12;
pub const ICE_FLOW_FIELD_IPV4_DST_OFFSET: c_int = 16;
pub const ICE_FLOW_FIELD_IPV6_SRC_OFFSET: c_int = 8;
pub const ICE_FLOW_FIELD_IPV6_DST_OFFSET: c_int = 24;
pub const ICE_FLOW_FIELD_SRC_PORT_OFFSET: c_int = 0;
pub const ICE_FLOW_FIELD_DST_PORT_OFFSET: c_int = 2;
// Protocol header fields within a packet segment. A segment consists of one or
// more protocol headers that make up a logical group of protocol headers. Each
// logical group of protocol headers encapsulates or is encapsulated using/by
// tunneling or encapsulation protocols for network virtualization such as GRE,
// VxLAN, etc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_flow_seg_hdr {
    ICE_FLOW_SEG_HDR_NONE		= 0x00000000,
    ICE_FLOW_SEG_HDR_ETH		= 0x00000001,
    ICE_FLOW_SEG_HDR_VLAN		= 0x00000002,
    ICE_FLOW_SEG_HDR_IPV4		= 0x00000004,
    ICE_FLOW_SEG_HDR_IPV6		= 0x00000008,
    ICE_FLOW_SEG_HDR_ARP		= 0x00000010,
    ICE_FLOW_SEG_HDR_ICMP		= 0x00000020,
    ICE_FLOW_SEG_HDR_TCP		= 0x00000040,
    ICE_FLOW_SEG_HDR_UDP		= 0x00000080,
    ICE_FLOW_SEG_HDR_SCTP		= 0x00000100,
    ICE_FLOW_SEG_HDR_GRE		= 0x00000200,
    ICE_FLOW_SEG_HDR_GTPC		= 0x00000400,
    ICE_FLOW_SEG_HDR_GTPC_TEID	= 0x00000800,
    ICE_FLOW_SEG_HDR_GTPU_IP	= 0x00001000,
    ICE_FLOW_SEG_HDR_GTPU_EH	= 0x00002000,
    ICE_FLOW_SEG_HDR_GTPU_DWN	= 0x00004000,
    ICE_FLOW_SEG_HDR_GTPU_UP	= 0x00008000,
    ICE_FLOW_SEG_HDR_PPPOE		= 0x00010000,
    ICE_FLOW_SEG_HDR_PFCP_NODE	= 0x00020000,
    ICE_FLOW_SEG_HDR_PFCP_SESSION	= 0x00040000,
    ICE_FLOW_SEG_HDR_L2TPV3		= 0x00080000,
    ICE_FLOW_SEG_HDR_ESP		= 0x00100000,
    ICE_FLOW_SEG_HDR_AH		= 0x00200000,
    ICE_FLOW_SEG_HDR_NAT_T_ESP	= 0x00400000,
    ICE_FLOW_SEG_HDR_ETH_NON_IP	= 0x00800000,
    ICE_FLOW_SEG_HDR_GTPU_NON_IP	= 0x01000000,
    ICE_FLOW_SEG_HDR_L2TPV2		= 0x10000000,
// The following is an additive bit for ICE_FLOW_SEG_HDR_IPV4 and
// ICE_FLOW_SEG_HDR_IPV6.
//
    ICE_FLOW_SEG_HDR_IPV_FRAG	= 0x40000000,
    ICE_FLOW_SEG_HDR_IPV_OTHER	= 0x80000000,
}

// These segments all have the same PTYPES, but are otherwise distinguished by
// the value of the gtp_eh_pdu and gtp_eh_pdu_link flags:
//
// gtp_eh_pdu     gtp_eh_pdu_link
// ICE_FLOW_SEG_HDR_GTPU_IP           0              0
// ICE_FLOW_SEG_HDR_GTPU_EH           1              don't care
// ICE_FLOW_SEG_HDR_GTPU_DWN          1              0
// ICE_FLOW_SEG_HDR_GTPU_UP           1              1
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_flow_field {
// L2
    ICE_FLOW_FIELD_IDX_ETH_DA,
    ICE_FLOW_FIELD_IDX_ETH_SA,
    ICE_FLOW_FIELD_IDX_S_VLAN,
    ICE_FLOW_FIELD_IDX_C_VLAN,
    ICE_FLOW_FIELD_IDX_ETH_TYPE,
// L3
    ICE_FLOW_FIELD_IDX_IPV4_DSCP,
    ICE_FLOW_FIELD_IDX_IPV6_DSCP,
    ICE_FLOW_FIELD_IDX_IPV4_TTL,
    ICE_FLOW_FIELD_IDX_IPV4_PROT,
    ICE_FLOW_FIELD_IDX_IPV6_TTL,
    ICE_FLOW_FIELD_IDX_IPV6_PROT,
    ICE_FLOW_FIELD_IDX_IPV4_SA,
    ICE_FLOW_FIELD_IDX_IPV4_DA,
    ICE_FLOW_FIELD_IDX_IPV6_SA,
    ICE_FLOW_FIELD_IDX_IPV6_DA,
    ICE_FLOW_FIELD_IDX_IPV4_CHKSUM,
    ICE_FLOW_FIELD_IDX_IPV4_ID,
    ICE_FLOW_FIELD_IDX_IPV6_ID,
    ICE_FLOW_FIELD_IDX_IPV6_PRE32_SA,
    ICE_FLOW_FIELD_IDX_IPV6_PRE32_DA,
    ICE_FLOW_FIELD_IDX_IPV6_PRE48_SA,
    ICE_FLOW_FIELD_IDX_IPV6_PRE48_DA,
    ICE_FLOW_FIELD_IDX_IPV6_PRE64_SA,
    ICE_FLOW_FIELD_IDX_IPV6_PRE64_DA,
// L4
    ICE_FLOW_FIELD_IDX_TCP_SRC_PORT,
    ICE_FLOW_FIELD_IDX_TCP_DST_PORT,
    ICE_FLOW_FIELD_IDX_UDP_SRC_PORT,
    ICE_FLOW_FIELD_IDX_UDP_DST_PORT,
    ICE_FLOW_FIELD_IDX_SCTP_SRC_PORT,
    ICE_FLOW_FIELD_IDX_SCTP_DST_PORT,
    ICE_FLOW_FIELD_IDX_TCP_FLAGS,
    ICE_FLOW_FIELD_IDX_TCP_CHKSUM,
    ICE_FLOW_FIELD_IDX_UDP_CHKSUM,
    ICE_FLOW_FIELD_IDX_SCTP_CHKSUM,
// ARP
    ICE_FLOW_FIELD_IDX_ARP_SIP,
    ICE_FLOW_FIELD_IDX_ARP_DIP,
    ICE_FLOW_FIELD_IDX_ARP_SHA,
    ICE_FLOW_FIELD_IDX_ARP_DHA,
    ICE_FLOW_FIELD_IDX_ARP_OP,
// ICMP
    ICE_FLOW_FIELD_IDX_ICMP_TYPE,
    ICE_FLOW_FIELD_IDX_ICMP_CODE,
// GRE
    ICE_FLOW_FIELD_IDX_GRE_KEYID,
// GTPC_TEID
    ICE_FLOW_FIELD_IDX_GTPC_TEID,
// GTPU_IP
    ICE_FLOW_FIELD_IDX_GTPU_IP_TEID,
// GTPU_EH
    ICE_FLOW_FIELD_IDX_GTPU_EH_TEID,
    ICE_FLOW_FIELD_IDX_GTPU_EH_QFI,
// GTPU_UP
    ICE_FLOW_FIELD_IDX_GTPU_UP_TEID,
    ICE_FLOW_FIELD_IDX_GTPU_UP_QFI,
// GTPU_DWN
    ICE_FLOW_FIELD_IDX_GTPU_DWN_TEID,
    ICE_FLOW_FIELD_IDX_GTPU_DWN_QFI,
    ICE_FLOW_FIELD_IDX_PPPOE_SESS_ID,
// PFCP
    ICE_FLOW_FIELD_IDX_PFCP_SEID,
    ICE_FLOW_FIELD_IDX_L2TPV3_SESS_ID,
// ESP
    ICE_FLOW_FIELD_IDX_ESP_SPI,
// AH
    ICE_FLOW_FIELD_IDX_AH_SPI,
// NAT_T ESP
    ICE_FLOW_FIELD_IDX_NAT_T_ESP_SPI,
// L2TPV2 SESSION ID
    ICE_FLOW_FIELD_IDX_L2TPV2_SESS_ID,
// L2TPV2_LEN SESSION ID
    ICE_FLOW_FIELD_IDX_L2TPV2_LEN_SESS_ID,
// The total number of enums must not exceed 64
    ICE_FLOW_FIELD_IDX_MAX
}

// Supported RSS offloads  This macro is defined to support
// VIRTCHNL_OP_GET_RSS_HASHCFG_CAPS ops. PF driver sends the RSS hardware
// capabilities to the caller of this ops.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_rss_cfg_hdr_type {
    ICE_RSS_OUTER_HEADERS, /* take outer headers as inputset. */
    ICE_RSS_INNER_HEADERS, /* take inner headers as inputset. */
// take inner headers as inputset for packet with outer ipv4.
    ICE_RSS_INNER_HEADERS_W_OUTER_IPV4,
// take inner headers as inputset for packet with outer ipv6.
    ICE_RSS_INNER_HEADERS_W_OUTER_IPV6,
// take outer headers first then inner headers as inputset
// take inner as inputset for GTPoGRE with outer IPv4 + GRE.
    ICE_RSS_INNER_HEADERS_W_OUTER_IPV4_GRE,
// take inner as inputset for GTPoGRE with outer IPv6 + GRE.
    ICE_RSS_INNER_HEADERS_W_OUTER_IPV6_GRE,
    ICE_RSS_ANY_HEADERS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_rss_hash_cfg {
    pub /: *mut *mut u32 addl_hdrs; / protocol header fields,
    pub /: *mut *mut *mut u64 hash_flds; / hash bit field (ICE_FLOW_HASH_) to configure,
    pub /: *mut *mut ice_rss_cfg_hdr_type hdr_type; / to specify inner or outer,
    pub /: *mut *mut bool symm; / symmetric or asymmetric hash,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_flow_dir {
    ICE_FLOW_RX		= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_flow_priority {
    ICE_FLOW_PRIO_LOW,
    ICE_FLOW_PRIO_NORMAL,
    ICE_FLOW_PRIO_HIGH
}

pub const ICE_FLOW_SEG_SINGLE: c_int = 1;
pub const ICE_FLOW_SEG_MAX: c_int = 2;
pub const ICE_FLOW_SEG_RAW_FLD_MAX: c_int = 2;
pub const ICE_FLOW_SW_FIELD_VECTOR_MAX: c_int = 48;
pub const ICE_FLOW_FV_EXTRACT_SZ: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_flow_seg_xtrct {
    pub /: *mut *mut u8 prot_id; / Protocol ID of extracted header field,
    pub /: *mut *mut u16 off; / Starting offset of the field in header in bytes,
    pub /: *mut *mut u8 idx; / Index of FV entry used,
    pub /: *mut *mut u8 disp; / Displacement of field in bits fr. FV entry's start,
    pub /: *mut *mut u16 mask; / Mask for field,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_flow_fld_match_type {
    ICE_FLOW_FLD_TYPE_REG,		/* Value, mask */
    ICE_FLOW_FLD_TYPE_RANGE,	/* Value, mask, last (upper bound) */
    ICE_FLOW_FLD_TYPE_PREFIX,	/* IP address, prefix, size of prefix */
    ICE_FLOW_FLD_TYPE_SIZE,		/* Value, mask, size of match */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_flow_fld_loc {
// Describe offsets of field information relative to the beginning of
// input buffer provided when adding flow entries.
//
    pub /: *mut *mut u16 val; / Offset where the value is located,
    pub /: *mut *mut u16 mask; / Offset where the mask/prefix value is located,
    pub /: *mut *mut u16 last; / Length or offset where the upper value is located,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_flow_fld_info {
    pub type: ice_flow_fld_match_type,
// Location where to retrieve data from an input buffer
    pub src: ice_flow_fld_loc,
// Location where to put the data into the final entry buffer
    pub entry: ice_flow_fld_loc,
    pub xtrct: ice_flow_seg_xtrct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_flow_seg_fld_raw {
    pub info: ice_flow_fld_info,
    pub /: *mut *mut u16 off; / Offset from the start of the segment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_flow_seg_info {
    pub /: *mut *mut u32 hdrs; / Bitmask indicating protocol headers present,
    pub /: *mut *mut u64 match; / Bitmask indicating header fields to be matched,
    pub /: *mut *mut u64 range; / Bitmask indicating header fields matched as ranges,
    pub fields: [ice_flow_fld_info; ICE_FLOW_FIELD_IDX_MAX],
    pub /: *mut *mut u8 raws_cnt; / Number of raw fields to be matched,
    pub raws: [ice_flow_seg_fld_raw; ICE_FLOW_SEG_RAW_FLD_MAX],
}

// This structure describes a flow entry, and is tracked only in this file
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_flow_entry {
    pub l_entry: list_head,
    pub id: u64,
    pub prof: *mut ice_flow_prof,
    pub priority: ice_flow_priority,
    pub vsi_handle: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_flow_prof {
    pub l_entry: list_head,
    pub id: u64,
    pub dir: ice_flow_dir,
    pub segs_cnt: u8,
// Keep track of flow entries associated with this flow profile
    pub entries_lock: mutex,
    pub entries: list_head,
    pub segs: [ice_flow_seg_info; ICE_FLOW_SEG_MAX],
// software VSI handles referenced by this flow profile
    pub ICE_MAX_VSI): DECLARE_BITMAP(vsis,,
    pub /: *mut *mut bool symm; / Symmetric Hash for RSS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_rss_raw_cfg {
    pub prof: ice_parser_profile,
    pub raw_ena: bool,
    pub symm: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_rss_cfg {
    pub l_entry: list_head,
// bitmap of VSIs added to the RSS entry
    pub ICE_MAX_VSI): DECLARE_BITMAP(vsis,,
    pub hash: ice_rss_hash_cfg,
}

extern "C" {
    pub fn ice_flow_rem_prof(hw: *mut ice_hw, blk: ice_block, prof_id: u64) -> c_int;
}
extern "C" {
    pub fn ice_flow_rem_entry(hw: *mut ice_hw, blk: ice_block, entry_h: u64) -> c_int;
}
extern "C" {
    pub fn ice_flow_rem_vsi_prof(hw: *mut ice_hw, vsi_handle: u16, prof_id: u64) -> c_int;
}
extern "C" {
    pub fn ice_rem_vsi_rss_list(hw: *mut ice_hw, vsi_handle: u16);
}
extern "C" {
    pub fn ice_replay_rss_cfg(hw: *mut ice_hw, vsi_handle: u16) -> c_int;
}
extern "C" {
    pub fn ice_set_rss_cfg_symm(hw: *mut ice_hw, vsi: *mut ice_vsi, symm: bool) -> c_int;
}
extern "C" {
    pub fn ice_rem_vsi_rss_cfg(hw: *mut ice_hw, vsi_handle: u16) -> c_int;
}
extern "C" {
    pub fn ice_get_rss_cfg(hw: *mut ice_hw, vsi_handle: u16, hdrs: u32, symm: *mut bool) -> u64;
}
