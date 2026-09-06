//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_ddp.h
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
// Copyright (c) 2022, Intel Corporation.

// Package minimal version supported
pub const ICE_PKG_SUPP_VER_MAJ: c_int = 1;
pub const ICE_PKG_SUPP_VER_MNR: c_int = 3;
// Package format version
pub const ICE_PKG_FMT_VER_MAJ: c_int = 1;
pub const ICE_PKG_FMT_VER_MNR: c_int = 0;
pub const ICE_PKG_FMT_VER_UPD: c_int = 0;
pub const ICE_PKG_FMT_VER_DFT: c_int = 0;
pub const ICE_PKG_CNT: c_int = 4;
pub const ICE_FV_OFFSET_INVAL: c_uint = 0x1FF;
// Extraction Sequence (Field Vector) Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fv_word {
    pub prot_id: u8,
    pub /: *mut *mut u16 off; / Offset within the protocol header,
    pub resvrd: u8,
    pub __packed: },
pub const ICE_MAX_NUM_PROFILES: c_int = 256;
pub const ICE_MAX_FV_WORDS: c_int = 48;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fv {
    pub ew: [ice_fv_word; ICE_MAX_FV_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ddp_state {
// Indicates that this call to ice_init_pkg
// successfully loaded the requested DDP package
//
    ICE_DDP_PKG_SUCCESS = 0,

// Generic error for already loaded errors, it is mapped later to
// the more specific one (one of the next 3)
//
    ICE_DDP_PKG_ALREADY_LOADED = -1,

// Indicates that a DDP package of the same version has already been
// loaded onto the device by a previous call or by another PF
//
    ICE_DDP_PKG_SAME_VERSION_ALREADY_LOADED = -2,

// The device has a DDP package that is not supported by the driver
    ICE_DDP_PKG_ALREADY_LOADED_NOT_SUPPORTED = -3,

// The device has a compatible package
// (but different from the request) already loaded
//
    ICE_DDP_PKG_COMPATIBLE_ALREADY_LOADED = -4,

// The firmware loaded on the device is not compatible with
// the DDP package loaded
//
    ICE_DDP_PKG_FW_MISMATCH = -5,

// The DDP package file is invalid
    ICE_DDP_PKG_INVALID_FILE = -6,

// The version of the DDP package provided is higher than
// the driver supports
//
    ICE_DDP_PKG_FILE_VERSION_TOO_HIGH = -7,

// The version of the DDP package provided is lower than the
// driver supports
//
    ICE_DDP_PKG_FILE_VERSION_TOO_LOW = -8,

// The signature of the DDP package file provided is invalid
    ICE_DDP_PKG_FILE_SIGNATURE_INVALID = -9,

// The DDP package file security revision is too low and not
// supported by firmware
//
    ICE_DDP_PKG_FILE_REVISION_TOO_LOW = -10,

// An error occurred in firmware while loading the DDP package
    ICE_DDP_PKG_LOAD_ERROR = -11,

// Other errors
    ICE_DDP_PKG_ERR = -12
}

// Package and segment headers and tables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pkg_hdr {
    pub pkg_format_ver: ice_pkg_ver,
    pub seg_count: __le32,
    pub seg_offset: [__le32; ],
}

// Package signing algorithm types
pub const SEGMENT_SIGN_TYPE_INVALID: c_uint = 0x00000000;
pub const SEGMENT_SIGN_TYPE_RSA2K: c_uint = 0x00000001;
pub const SEGMENT_SIGN_TYPE_RSA3K: c_uint = 0x00000002;
pub const SEGMENT_SIGN_TYPE_RSA3K_SBB: c_uint = 0x00000003 /* Secure Boot Block */;
pub const SEGMENT_SIGN_TYPE_RSA3K_E825: c_uint = 0x00000005;
// generic segment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_generic_seg_hdr {
pub const SEGMENT_TYPE_INVALID: c_uint = 0x00000000;
pub const SEGMENT_TYPE_METADATA: c_uint = 0x00000001;
pub const SEGMENT_TYPE_ICE_E810: c_uint = 0x00000010;
pub const SEGMENT_TYPE_SIGNING: c_uint = 0x00001001;
pub const SEGMENT_TYPE_ICE_RUN_TIME_CFG: c_uint = 0x00000020;
pub const SEGMENT_TYPE_ICE_E830: c_uint = 0x00000017;
    pub seg_type: __le32,
    pub seg_format_ver: ice_pkg_ver,
    pub seg_size: __le32,
    pub seg_id: [c_char; ICE_PKG_NAME_SIZE],
}

// ice specific segment
#[repr(C)]
#[derive(Copy, Clone)]
pub union ice_device_id {
    pub device_id: __le16,
    pub vendor_id: __le16,
    pub dev_vend_id: },
    pub id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_device_id_entry {
    pub device: ice_device_id,
    pub sub_device: ice_device_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_seg {
    pub hdr: ice_generic_seg_hdr,
    pub device_table_count: __le32,
    pub device_table: [ice_device_id_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_nvm_table {
    pub table_count: __le32,
    pub vers: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_buf {
pub const ICE_PKG_BUF_SIZE: c_int = 4096;
    pub buf: [u8; ICE_PKG_BUF_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_buf_table {
    pub buf_count: __le32,
    pub buf_array: [ice_buf; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_run_time_cfg_seg {
    pub hdr: ice_generic_seg_hdr,
    pub rsvd: [u8; 8],
    pub buf_table: ice_buf_table,
}

// global metadata specific segment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_global_metadata_seg {
    pub hdr: ice_generic_seg_hdr,
    pub pkg_ver: ice_pkg_ver,
    pub rsvd: __le32,
    pub pkg_name: [c_char; ICE_PKG_NAME_SIZE],
}

pub const ICE_MIN_S_OFF: c_int = 12;
pub const ICE_MAX_S_OFF: c_int = 4095;
pub const ICE_MIN_S_SZ: c_int = 1;
pub const ICE_MAX_S_SZ: c_int = 4084;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sign_seg {
    pub hdr: ice_generic_seg_hdr,
    pub seg_id: __le32,
    pub sign_type: __le32,
    pub signed_seg_idx: __le32,
    pub signed_buf_start: __le32,
    pub signed_buf_count: __le32,
pub const ICE_SIGN_SEG_FLAGS_VALID: c_uint = 0x80000000;
pub const ICE_SIGN_SEG_FLAGS_LAST: c_uint = 0x00000001;
    pub flags: __le32,
pub const ICE_SIGN_SEG_RESERVED_COUNT: c_int = 40;
    pub reserved: [u8; ICE_SIGN_SEG_RESERVED_COUNT],
    pub buf_tbl: ice_buf_table,
}

// section information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_section_entry {
    pub type: __le32,
    pub offset: __le16,
    pub size: __le16,
}

pub const ICE_MIN_S_COUNT: c_int = 1;
pub const ICE_MAX_S_COUNT: c_int = 511;
pub const ICE_MIN_S_DATA_END: c_int = 12;
pub const ICE_MAX_S_DATA_END: c_int = 4096;
pub const ICE_METADATA_BUF: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_buf_hdr {
    pub section_count: __le16,
    pub data_end: __le16,
    pub section_entry: [ice_section_entry; ],
}

// ice package section IDs
pub const ICE_SID_METADATA: c_int = 1;
pub const ICE_SID_XLT0_SW: c_int = 10;
pub const ICE_SID_XLT_KEY_BUILDER_SW: c_int = 11;
pub const ICE_SID_XLT1_SW: c_int = 12;
pub const ICE_SID_XLT2_SW: c_int = 13;
pub const ICE_SID_PROFID_TCAM_SW: c_int = 14;
pub const ICE_SID_PROFID_REDIR_SW: c_int = 15;
pub const ICE_SID_FLD_VEC_SW: c_int = 16;
pub const ICE_SID_CDID_KEY_BUILDER_SW: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_meta_sect {
    pub ver: ice_pkg_ver,
pub const ICE_META_SECT_NAME_SIZE: c_int = 28;
    pub name: [c_char; ICE_META_SECT_NAME_SIZE],
    pub track_id: __le32,
}

pub const ICE_SID_CDID_REDIR_SW: c_int = 18;
pub const ICE_SID_XLT0_ACL: c_int = 20;
pub const ICE_SID_XLT_KEY_BUILDER_ACL: c_int = 21;
pub const ICE_SID_XLT1_ACL: c_int = 22;
pub const ICE_SID_XLT2_ACL: c_int = 23;
pub const ICE_SID_PROFID_TCAM_ACL: c_int = 24;
pub const ICE_SID_PROFID_REDIR_ACL: c_int = 25;
pub const ICE_SID_FLD_VEC_ACL: c_int = 26;
pub const ICE_SID_CDID_KEY_BUILDER_ACL: c_int = 27;
pub const ICE_SID_CDID_REDIR_ACL: c_int = 28;
pub const ICE_SID_XLT0_FD: c_int = 30;
pub const ICE_SID_XLT_KEY_BUILDER_FD: c_int = 31;
pub const ICE_SID_XLT1_FD: c_int = 32;
pub const ICE_SID_XLT2_FD: c_int = 33;
pub const ICE_SID_PROFID_TCAM_FD: c_int = 34;
pub const ICE_SID_PROFID_REDIR_FD: c_int = 35;
pub const ICE_SID_FLD_VEC_FD: c_int = 36;
pub const ICE_SID_CDID_KEY_BUILDER_FD: c_int = 37;
pub const ICE_SID_CDID_REDIR_FD: c_int = 38;
pub const ICE_SID_XLT0_RSS: c_int = 40;
pub const ICE_SID_XLT_KEY_BUILDER_RSS: c_int = 41;
pub const ICE_SID_XLT1_RSS: c_int = 42;
pub const ICE_SID_XLT2_RSS: c_int = 43;
pub const ICE_SID_PROFID_TCAM_RSS: c_int = 44;
pub const ICE_SID_PROFID_REDIR_RSS: c_int = 45;
pub const ICE_SID_FLD_VEC_RSS: c_int = 46;
pub const ICE_SID_CDID_KEY_BUILDER_RSS: c_int = 47;
pub const ICE_SID_CDID_REDIR_RSS: c_int = 48;
pub const ICE_SID_RXPARSER_CAM: c_int = 50;
pub const ICE_SID_RXPARSER_NOMATCH_CAM: c_int = 51;
pub const ICE_SID_RXPARSER_IMEM: c_int = 52;
pub const ICE_SID_RXPARSER_MARKER_PTYPE: c_int = 55;
pub const ICE_SID_RXPARSER_BOOST_TCAM: c_int = 56;
pub const ICE_SID_RXPARSER_PROTO_GRP: c_int = 57;
pub const ICE_SID_RXPARSER_METADATA_INIT: c_int = 58;
pub const ICE_SID_TXPARSER_BOOST_TCAM: c_int = 66;
pub const ICE_SID_RXPARSER_MARKER_GRP: c_int = 72;
pub const ICE_SID_RXPARSER_PG_SPILL: c_int = 76;
pub const ICE_SID_RXPARSER_NOMATCH_SPILL: c_int = 78;
pub const ICE_SID_XLT0_PE: c_int = 80;
pub const ICE_SID_XLT_KEY_BUILDER_PE: c_int = 81;
pub const ICE_SID_XLT1_PE: c_int = 82;
pub const ICE_SID_XLT2_PE: c_int = 83;
pub const ICE_SID_PROFID_TCAM_PE: c_int = 84;
pub const ICE_SID_PROFID_REDIR_PE: c_int = 85;
pub const ICE_SID_FLD_VEC_PE: c_int = 86;
pub const ICE_SID_CDID_KEY_BUILDER_PE: c_int = 87;
pub const ICE_SID_CDID_REDIR_PE: c_int = 88;
pub const ICE_SID_RXPARSER_FLAG_REDIR: c_int = 97;
// Label Metadata section IDs
pub const ICE_SID_LBL_FIRST: c_uint = 0x80000010;
pub const ICE_SID_LBL_RXPARSER_TMEM: c_uint = 0x80000018;
// The following define MUST be updated to reflect the last label section ID
pub const ICE_SID_LBL_LAST: c_uint = 0x80000038;
// Label ICE runtime configuration section IDs
pub const ICE_SID_TX_5_LAYER_TOPO: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_block {
    ICE_BLK_SW = 0,
    ICE_BLK_ACL,
    ICE_BLK_FD,
    ICE_BLK_RSS,
    ICE_BLK_PE,
    ICE_BLK_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_sect {
    ICE_XLT0 = 0,
    ICE_XLT_KB,
    ICE_XLT1,
    ICE_XLT2,
    ICE_PROF_TCAM,
    ICE_PROF_REDIR,
    ICE_VEC_TBL,
    ICE_CDID_KB,
    ICE_CDID_REDIR,
    ICE_SECT_COUNT
}

// package labels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_label {
    pub value: __le16,
pub const ICE_PKG_LABEL_SIZE: c_int = 64;
    pub name: [c_char; ICE_PKG_LABEL_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_label_section {
    pub count: __le16,
    pub label: [ice_label; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sw_fv_section {
    pub count: __le16,
    pub base_offset: __le16,
    pub fv: [ice_fv; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sw_fv_list_entry {
    pub list_entry: list_head,
    pub profile_id: u32,
    pub fv_ptr: *mut ice_fv,
}

// The BOOST TCAM stores the match packet header in reverse order, meaning
// the fields are reversed; in addition, this means that the normally big endian
// fields of the packet are now little endian.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_boost_key_value {
pub const ICE_BOOST_REMAINING_HV_KEY: c_int = 15;
    pub remaining_hv_key: [u8; ICE_BOOST_REMAINING_HV_KEY],
    pub hv_dst_port_key: __le16,
    pub hv_src_port_key: __le16,
    pub tcam_search_key: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_boost_key {
    pub key: ice_boost_key_value,
    pub key2: ice_boost_key_value,
}

// package Boost TCAM entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_boost_tcam_entry {
    pub addr: __le16,
    pub reserved: __le16,
// break up the 40 bytes of key into different fields
    pub key: ice_boost_key,
    pub boost_hit_index_group: u8,
// The following contains bitfields which are not on byte boundaries.
// These fields are currently unused by driver software.
//
pub const ICE_BOOST_BIT_FIELDS: c_int = 43;
    pub bit_fields: [u8; ICE_BOOST_BIT_FIELDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_boost_tcam_section {
    pub count: __le16,
    pub reserved: __le16,
    pub tcam: [ice_boost_tcam_entry; ],
}

// package Marker Ptype TCAM entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_marker_ptype_tcam_entry {
pub const ICE_MARKER_PTYPE_TCAM_ADDR_MAX: c_int = 1024;
    pub addr: __le16,
    pub ptype: __le16,
    pub keys: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_marker_ptype_tcam_section {
    pub count: __le16,
    pub reserved: __le16,
    pub tcam: [ice_marker_ptype_tcam_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_xlt1_section {
    pub count: __le16,
    pub offset: __le16,
    pub value: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_xlt2_section {
    pub count: __le16,
    pub offset: __le16,
    pub value: [__le16; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_prof_redir_section {
    pub count: __le16,
    pub offset: __le16,
    pub redir_value: [u8; ],
}

// package buffer building
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_buf_build {
    pub buf: ice_buf,
    pub reserved_section_table_entries: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pkg_enum {
    pub buf_table: *mut ice_buf_table,
    pub buf_idx: u32,
    pub type: u32,
    pub buf: *const ice_buf_hdr,
    pub sect_idx: u32,
    pub sect: *mut c_void,
    pub sect_type: u32,
    pub entry_idx: u32,
    pub offset): *mut *mut *mut *mut void (handler)(u32 sect_type, void section, u32 index, u32,
}

extern "C" {
    pub fn ice_update_pkg_no_lock(hw: *mut ice_hw, bufs: *mut ice_buf, count: u32) -> c_int;
}
extern "C" {
    pub fn ice_update_pkg(hw: *mut ice_hw, bufs: *mut ice_buf, count: u32) -> c_int;
}
extern "C" {
    pub fn ice_pkg_buf_reserve_section(bld: *mut ice_buf_build, count: u16) -> c_int;
}
extern "C" {
    pub fn ice_pkg_buf_get_active_sections(bld: *mut ice_buf_build) -> u16;
}
extern "C" {
    pub fn ice_cfg_tx_topo(hw: *mut ice_hw, buf: *const c_void, len: u32) -> c_int;
}
