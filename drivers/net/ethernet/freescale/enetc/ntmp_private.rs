//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/enetc/ntmp_private.h
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
//
// NTMP table request and response data buffer formats
// Copyright 2025-2026 NXP
//

pub const NTMP_EID_REQ_LEN: c_int = 8;
pub const NTMP_STATUS_RESP_LEN: c_int = 4;
pub const NETC_CBDR_BD_NUM: c_int = 256;

pub const NETC_CBDR_CLEAN_WORK: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub union netc_cbd {
    pub addr: __le64,
    pub len: __le32,

    pub cmd: u8,

    pub access_method: u8,

pub const NTMP_AM_ENTRY_ID: c_int = 0;
pub const NTMP_AM_EXACT_KEY: c_int = 1;
pub const NTMP_AM_SEARCH: c_int = 2;
pub const NTMP_AM_TERNARY_KEY: c_int = 3;
    pub table_id: u8,
    pub ver_cci_rr: u8,

pub const NTMP_HDR_VER2: c_int = 2;
    pub resv: [__le32; 3],
    pub npf: __le32,

    pub /: *mut *mut } req_hdr; / NTMP Request Message Header Format,
    pub resv0: [__le32; 3],
    pub num_matched: __le16,
    pub error_rr: __le16,
    pub resv1: [__le32; 4],
    pub /: *mut *mut } resp_hdr; / NTMP Response Message Header Format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntmp_cmn_req_data {
    pub update_act: __le16,
    pub dbg_opt: u8,
    pub tblv_qact: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntmp_cmn_resp_query {
    pub entry_id: __le32,
}

// Generic structure for request data by entry ID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntmp_req_by_eid {
    pub crd: ntmp_cmn_req_data,
    pub entry_id: __le32,
}

// MAC Address Filter Table Request Data Buffer Format of Add action
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maft_req_add {
    pub rbe: ntmp_req_by_eid,
    pub keye: maft_keye_data,
    pub cfge: maft_cfge_data,
}

// MAC Address Filter Table Response Data Buffer Format of Query action
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maft_resp_query {
    pub entry_id: __le32,
    pub keye: maft_keye_data,
    pub cfge: maft_cfge_data,
}

// RSS Table Request Data Buffer Format of Update action
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsst_req_update {
    pub rbe: ntmp_req_by_eid,
    pub groups: [u8; ],
}

// Ingress Port Filter Table Response Data Buffer Format of Query action
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipft_resp_query {
    pub status: __le32,
    pub entry_id: __le32,
    pub keye: ipft_keye_data,
    pub /: *mut *mut __le64 match_count; / STSE_DATA,
    pub cfge: ipft_cfge_data,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipft_ak_eid {
    pub entry_id: __le32,
    pub resv: [__le32; 52],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ipft_access_key {
    pub eid: ipft_ak_eid,
    pub keye: ipft_keye_data,
}

// Ingress Port Filter Table Request Data Buffer Format of Update and
// Add actions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipft_req_ua {
    pub crd: ntmp_cmn_req_data,
    pub ak: ipft_access_key,
    pub cfge: ipft_cfge_data,
}

// Ingress Port Filter Table Request Data Buffer Format of Query and
// Delete actions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipft_req_qd {
    pub rbe: ntmp_req_by_eid,
    pub resv: [__le32; 52],
}

// Access Key Format of FDB Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdbt_ak_eid {
    pub entry_id: __le32,
    pub resv: [__le32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdbt_ak_exact {
    pub keye: fdbt_keye_data,
    pub resv: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdbt_ak_search {
    pub resume_eid: __le32,
    pub keye: fdbt_keye_data,
    pub cfge: fdbt_cfge_data,
    pub acte: u8,
    pub keye_mc: u8,

    pub cfge_mc: u8,

pub const FDBT_CFGE_MC_ANY: c_int = 0;
pub const FDBT_CFGE_MC_DYNAMIC: c_int = 1;
pub const FDBT_CFGE_MC_PORT_BITMAP: c_int = 2;
pub const FDBT_CFGE_MC_DYNAMIC_AND_PORT_BITMAP: c_int = 3;
    pub acte_mc: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fdbt_access_key {
    pub eid: fdbt_ak_eid,
    pub exact: fdbt_ak_exact,
    pub search: fdbt_ak_search,
}

// FDB Table Request Data Buffer Format of Update and Add actions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdbt_req_ua {
    pub crd: ntmp_cmn_req_data,
    pub ak: fdbt_access_key,
    pub cfge: fdbt_cfge_data,
}

// FDB Table Request Data Buffer Format of Query and Delete actions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdbt_req_qd {
    pub crd: ntmp_cmn_req_data,
    pub ak: fdbt_access_key,
}

// FDB Table Response Data Buffer Format of Query action
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdbt_resp_query {
    pub status: __le32,
    pub entry_id: __le32,
    pub keye: fdbt_keye_data,
    pub cfge: fdbt_cfge_data,
    pub acte: u8,
    pub resv: [u8; 3],
}

// Access Key Format of VLAN Filter Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vft_ak_exact {
    pub /: *mut *mut __le16 vid; / bit0~11: VLAN ID, other bits are reserved,
    pub resv: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vft_access_key {
    pub /: *mut *mut __le32 entry_id; / entry_id match,
    pub exact: vft_ak_exact,
    pub /: *mut *mut __le32 resume_entry_id; / search,
}

// VLAN Filter Table Request Data Buffer Format of Update and Add actions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vft_req_ua {
    pub crd: ntmp_cmn_req_data,
    pub ak: vft_access_key,
    pub cfge: vft_cfge_data,
}

// VLAN Filter Table Request Data Buffer Format of Query and Delete actions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vft_req_qd {
    pub crd: ntmp_cmn_req_data,
    pub ak: vft_access_key,
}

// Egress Treatment Table Request Data Buffer Format of Update and Add
// actions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ett_req_ua {
    pub rbe: ntmp_req_by_eid,
    pub cfge: ett_cfge_data,
}

// Buffer Pool Table Request Data Buffer Format of Update action
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpt_req_update {
    pub rbe: ntmp_req_by_eid,
    pub cfge: bpt_cfge_data,
}
