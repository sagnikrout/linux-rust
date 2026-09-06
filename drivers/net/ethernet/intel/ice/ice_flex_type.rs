//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_flex_type.h
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

// Packet Type (PTYPE) values
pub const ICE_PTYPE_MAC_PAY: c_int = 1;
pub const ICE_PTYPE_IPV4_PAY: c_int = 23;
pub const ICE_PTYPE_IPV4_UDP_PAY: c_int = 24;
pub const ICE_PTYPE_IPV4_TCP_PAY: c_int = 26;
pub const ICE_PTYPE_IPV4_SCTP_PAY: c_int = 27;
pub const ICE_PTYPE_IPV6_PAY: c_int = 89;
pub const ICE_PTYPE_IPV6_UDP_PAY: c_int = 90;
pub const ICE_PTYPE_IPV6_TCP_PAY: c_int = 92;
pub const ICE_PTYPE_IPV6_SCTP_PAY: c_int = 93;
pub const ICE_MAC_IPV4_ESP: c_int = 160;
pub const ICE_MAC_IPV6_ESP: c_int = 161;
pub const ICE_MAC_IPV4_AH: c_int = 162;
pub const ICE_MAC_IPV6_AH: c_int = 163;
pub const ICE_MAC_IPV4_NAT_T_ESP: c_int = 164;
pub const ICE_MAC_IPV6_NAT_T_ESP: c_int = 165;
pub const ICE_MAC_IPV4_GTPU: c_int = 329;
pub const ICE_MAC_IPV6_GTPU: c_int = 330;
pub const ICE_MAC_IPV4_GTPU_IPV4_FRAG: c_int = 331;
pub const ICE_MAC_IPV4_GTPU_IPV4_PAY: c_int = 332;
pub const ICE_MAC_IPV4_GTPU_IPV4_UDP_PAY: c_int = 333;
pub const ICE_MAC_IPV4_GTPU_IPV4_TCP: c_int = 334;
pub const ICE_MAC_IPV4_GTPU_IPV4_ICMP: c_int = 335;
pub const ICE_MAC_IPV6_GTPU_IPV4_FRAG: c_int = 336;
pub const ICE_MAC_IPV6_GTPU_IPV4_PAY: c_int = 337;
pub const ICE_MAC_IPV6_GTPU_IPV4_UDP_PAY: c_int = 338;
pub const ICE_MAC_IPV6_GTPU_IPV4_TCP: c_int = 339;
pub const ICE_MAC_IPV6_GTPU_IPV4_ICMP: c_int = 340;
pub const ICE_MAC_IPV4_GTPU_IPV6_FRAG: c_int = 341;
pub const ICE_MAC_IPV4_GTPU_IPV6_PAY: c_int = 342;
pub const ICE_MAC_IPV4_GTPU_IPV6_UDP_PAY: c_int = 343;
pub const ICE_MAC_IPV4_GTPU_IPV6_TCP: c_int = 344;
pub const ICE_MAC_IPV4_GTPU_IPV6_ICMPV6: c_int = 345;
pub const ICE_MAC_IPV6_GTPU_IPV6_FRAG: c_int = 346;
pub const ICE_MAC_IPV6_GTPU_IPV6_PAY: c_int = 347;
pub const ICE_MAC_IPV6_GTPU_IPV6_UDP_PAY: c_int = 348;
pub const ICE_MAC_IPV6_GTPU_IPV6_TCP: c_int = 349;
pub const ICE_MAC_IPV6_GTPU_IPV6_ICMPV6: c_int = 350;
pub const ICE_MAC_IPV4_PFCP_SESSION: c_int = 352;
pub const ICE_MAC_IPV6_PFCP_SESSION: c_int = 354;
pub const ICE_MAC_IPV4_L2TPV3: c_int = 360;
pub const ICE_MAC_IPV6_L2TPV3: c_int = 361;
// Attributes that can modify PTYPE definitions.
//
// These values will represent special attributes for PTYPEs, which will
// resolve into metadata packet flags definitions that can be used in the TCAM
// for identifying a PTYPE with specific characteristics.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ptype_attrib_type {
// GTP PTYPEs
    ICE_PTYPE_ATTR_GTP_PDU_EH,
    ICE_PTYPE_ATTR_GTP_SESSION,
    ICE_PTYPE_ATTR_GTP_DOWNLINK,
    ICE_PTYPE_ATTR_GTP_UPLINK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptype_attrib_info {
    pub flags: u16,
    pub mask: u16,
}

// TCAM flag definitions

// GTP attributes

pub const ICE_GTP_SESSION: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptype_attributes {
    pub ptype: u16,
    pub attrib: ice_ptype_attrib_type,
}

// Tunnel enabling
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tunnel_type {
    TNL_VXLAN = 0,
    TNL_GENEVE,
    TNL_GRETAP,
    TNL_GTPC,
    TNL_GTPU,
    TNL_PFCP,
    __TNL_TYPE_CNT,
    TNL_LAST = 0xFF,
    TNL_ALL = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tunnel_type_scan {
    pub type: ice_tunnel_type,
    pub label_prefix: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tunnel_entry {
    pub type: ice_tunnel_type,
    pub boost_addr: u16,
    pub port: u16,
    pub boost_entry: *mut ice_boost_tcam_entry,
    pub valid: u8,
}

pub const ICE_TUNNEL_MAX_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tunnel_table {
    pub tbl: [ice_tunnel_entry; ICE_TUNNEL_MAX_ENTRIES],
    pub count: u16,
    pub valid_count: [u16; __TNL_TYPE_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dvm_entry {
    pub boost_addr: u16,
    pub enable: u16,
    pub boost_entry: *mut ice_boost_tcam_entry,
}

pub const ICE_DVM_MAX_ENTRIES: c_int = 48;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dvm_table {
    pub tbl: [ice_dvm_entry; ICE_DVM_MAX_ENTRIES],
    pub count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pkg_es {
    pub count: __le16,
    pub offset: __le16,
    pub es: [ice_fv_word; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_es {
    pub sid: u32,
    pub count: u16,
    pub fvw: u16,
    pub ref_count: *mut u16,
    pub mask_ena: *mut u32,
    pub prof_map: list_head,
    pub t: *mut ice_fv_word,
    pub blk)*/: *mut *mut *mut u8 symm; / symmetric setting per profile (RSS,
    pub /: *mut *mut mutex prof_map_lock; / protect access to profiles list,
    pub written: *mut u8,
    pub /: *mut *mut u8 reverse; / set to true to reverse FV order,
}

// PTYPE Group management
// Note: XLT1 table takes 13-bit as input, and results in an 8-bit packet type
// group (PTG) ID as output.
//
// Note: PTG 0 is the default packet type group and it is assumed that all PTYPE
// are a part of this group until moved to a new PTG.
//
pub const ICE_DEFAULT_PTG: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptg_entry {
    pub first_ptype: *mut ice_ptg_ptype,
    pub in_use: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptg_ptype {
    pub next_ptype: *mut ice_ptg_ptype,
    pub ptg: u8,
}

pub const ICE_MAX_TCAM_PER_PROFILE: c_int = 32;
pub const ICE_MAX_PTG_PER_PROFILE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_prof_map {
    pub list: list_head,
    pub profile_cookie: u64,
    pub context: u64,
    pub prof_id: u8,
    pub ptg_cnt: u8,
    pub ptg: [u8; ICE_MAX_PTG_PER_PROFILE],
    pub attr: [ice_ptype_attrib_info; ICE_MAX_PTG_PER_PROFILE],
}

pub const ICE_INVALID_TCAM: c_uint = 0xFFFF;
pub const ICE_MAX_PTG_ATTRS: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tcam_inf {
    pub tcam_idx: u16,
    pub attr: ice_ptype_attrib_info,
    pub ptg: u8,
    pub prof_id: u8,
    pub in_use: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vsig_prof {
    pub list: list_head,
    pub profile_cookie: u64,
    pub prof_id: u8,
    pub tcam_count: u8,
    pub tcam: [ice_tcam_inf; ICE_MAX_TCAM_PER_PROFILE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vsig_entry {
    pub prop_lst: list_head,
    pub first_vsi: *mut ice_vsig_vsi,
    pub in_use: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vsig_vsi {
    pub next_vsi: *mut ice_vsig_vsi,
    pub prop_mask: u32,
    pub changed: u16,
    pub vsig: u16,
}

pub const ICE_XLT1_CNT: c_int = 1024;
pub const ICE_MAX_PTGS: c_int = 256;
// XLT1 Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_xlt1 {
    pub ptg_tbl: *mut ice_ptg_entry,
    pub ptypes: *mut ice_ptg_ptype,
    pub t: *mut u8,
    pub sid: u32,
    pub count: u16,
}

pub const ICE_XLT2_CNT: c_int = 768;
pub const ICE_MAX_VSIGS: c_int = 768;
// VSIG bit layout:
// [0:12]: incremental VSIG index 1 to ICE_MAX_VSIGS
// [13:15]: PF number of device
//

pub const ICE_PF_NUM_S: c_int = 13;

pub const ICE_DEFAULT_VSIG: c_int = 0;
// XLT2 Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_xlt2 {
    pub vsig_tbl: *mut ice_vsig_entry,
    pub vsis: *mut ice_vsig_vsi,
    pub t: *mut u16,
    pub sid: u32,
    pub count: u16,
}

// Profile ID Management
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_prof_id_key {
    pub flags: __le16,
    pub xlt1: u8,
    pub xlt2_cdid: __le16,
    pub __packed: },
// Keys are made up of two values, each one-half the size of the key.
// For TCAM, the entire key is 80 bits wide (or 2, 40-bit wide values)
//
pub const ICE_TCAM_KEY_VAL_SZ: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_prof_tcam_entry {
    pub addr: __le16,
    pub key: [u8; ICE_TCAM_KEY_SZ],
    pub prof_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_prof_id_section {
    pub count: __le16,
    pub entry: [ice_prof_tcam_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_prof_tcam {
    pub sid: u32,
    pub count: u16,
    pub max_prof_id: u16,
    pub t: *mut ice_prof_tcam_entry,
    pub /: *mut *mut u8 cdid_bits; / # CDID bits to use in key, 0, 2, 4, or 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_prof_redir {
    pub t: *mut u8,
    pub sid: u32,
    pub count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_mask {
    pub /: *mut *mut u16 mask; / 16-bit mask,
    pub /: *mut *mut u16 idx; / index,
    pub /: *mut *mut u16 ref; / reference count,
    pub /: *mut *mut u8 in_use; / non-zero if used,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_masks {
    pub /: *mut *mut mutex lock; / lock to protect this structure,
    pub /: *mut *mut u16 first; / first mask owned by the PF,
    pub /: *mut *mut u16 count; / number of masks owned by the PF,
pub const ICE_PROF_MASK_COUNT: c_int = 32;
    pub masks: [ice_mask; ICE_PROF_MASK_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_prof_id {
    pub id: *mut c_ulong,
    pub count: c_int,
}

// Tables per block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_blk_info {
    pub xlt1: ice_xlt1,
    pub xlt2: ice_xlt2,
    pub prof_id: ice_prof_id,
    pub prof: ice_prof_tcam,
    pub prof_redir: ice_prof_redir,
    pub es: ice_es,
    pub masks: ice_masks,
    pub /: *mut *mut u8 overwrite; / set to true to allow overwrite of table entries,
    pub is_list_init: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_chg_type {
    ICE_TCAM_NONE = 0,
    ICE_PTG_ES_ADD,
    ICE_TCAM_ADD,
    ICE_VSIG_ADD,
    ICE_VSIG_REM,
    ICE_VSI_MOVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_chs_chg {
    pub list_entry: list_head,
    pub type: ice_chg_type,
    pub add_ptg: u8,
    pub add_vsig: u8,
    pub add_tcam_idx: u8,
    pub add_prof: u8,
    pub ptype: u16,
    pub ptg: u8,
    pub prof_id: u8,
    pub vsi: u16,
    pub vsig: u16,
    pub orig_vsig: u16,
    pub tcam_idx: u16,
    pub attr: ice_ptype_attrib_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_prof_type {
    ICE_PROF_NON_TUN = 0x1,
    ICE_PROF_TUN_UDP = 0x2,
    ICE_PROF_TUN_GRE = 0x4,
    ICE_PROF_TUN_GTPU = 0x8,
    ICE_PROF_TUN_GTPC = 0x10,
    ICE_PROF_TUN_PFCP = 0x20,
    ICE_PROF_TUN_ALL = 0x3E,
    ICE_PROF_ALL = 0xFF,
}

// Number of bits/bytes contained in meta init entry. Note, this should be a
// multiple of 32 bits.
//
pub const ICE_META_INIT_BITS: c_int = 192;

// The meta init Flag field starts at this bit
pub const ICE_META_FLAGS_ST: c_int = 123;
// The entry and bit to check for Double VLAN Mode (DVM) support
pub const ICE_META_VLAN_MODE_ENTRY: c_int = 0;
pub const ICE_META_FLAG_VLAN_MODE: c_int = 60;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_meta_init_entry {
    pub bm: [__le32; ICE_META_INIT_DW_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_meta_init_section {
    pub count: __le16,
    pub offset: __le16,
    pub entry: ice_meta_init_entry,
}
