//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/scsi/fc/fc_ns.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright(c) 2007 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

//
// Fibre Channel Services - Name Service (dNS)
// From T11.org FC-GS-2 Rev 5.3 November 1998.
//
// Common-transport sub-type for Name Server.
//

//
// Name server Requests.
// Note:  this is an incomplete list, some unused requests are omitted.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ns_req {
    FC_NS_GA_NXT =	0x0100,		/* get all next */
    FC_NS_GI_A =	0x0101,		/* get identifiers - scope */
    FC_NS_GPN_ID =	0x0112,		/* get port name by ID */
    FC_NS_GNN_ID =	0x0113,		/* get node name by ID */
    FC_NS_GSPN_ID = 0x0118,		/* get symbolic port name */
    FC_NS_GID_PN =	0x0121,		/* get ID for port name */
    FC_NS_GID_NN =	0x0131,		/* get IDs for node name */
    FC_NS_GID_FT =	0x0171,		/* get IDs by FC4 type */
    FC_NS_GPN_FT =	0x0172,		/* get port names by FC4 type */
    FC_NS_GID_PT =	0x01a1,		/* get IDs by port type */
    FC_NS_RPN_ID =	0x0212,		/* reg port name for ID */
    FC_NS_RNN_ID =	0x0213,		/* reg node name for ID */
    FC_NS_RFT_ID =	0x0217,		/* reg FC4 type for ID */
    FC_NS_RSPN_ID =	0x0218,		/* reg symbolic port name */
    FC_NS_RFF_ID =	0x021f,		/* reg FC4 Features for ID */
    FC_NS_RSNN_NN =	0x0239,		/* reg symbolic node name */
}

//
// Port type values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ns_pt {
    FC_NS_UNID_PORT = 0x00,	/* unidentified */
    FC_NS_N_PORT =	0x01,	/* N port */
    FC_NS_NL_PORT =	0x02,	/* NL port */
    FC_NS_FNL_PORT = 0x03,	/* F/NL port */
    FC_NS_NX_PORT =	0x7f,	/* Nx port */
    FC_NS_F_PORT =	0x81,	/* F port */
    FC_NS_FL_PORT =	0x82,	/* FL port */
    FC_NS_E_PORT =	0x84,	/* E port */
    FC_NS_B_PORT =	0x85,	/* B port */
}

//
// Port type object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_pt_obj {
    pub pt_type: __u8,
}

//
// Port ID object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_fid {
    pub /: *mut *mut __u8 fp_flags; / flags for responses only,
    pub fp_fid: [__u8; 3],
}

//
// fp_flags in port ID object, for responses only.
//
pub const FC_NS_FID_LAST: c_uint = 0x80		/* last object */;
//
// FC4-types object.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_fts {
    pub /: *mut *mut __be32 ff_type_map[FC_NS_TYPES / FC_NS_BPW]; / bitmap of FC-4 types,
}

//
// FC4-features object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_ff {
    pub /: *mut *mut *mut __be32 fd_feat[FC_NS_TYPES  4 / FC_NS_BPW]; / 4-bits per FC-type,
}

//
// GID_PT request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_gid_pt {
    pub fn_pt_type: __u8,
    pub fn_domain_id_scope: __u8,
    pub fn_area_id_scope: __u8,
    pub fn_resvd: __u8,
}

//
// GID_FT or GPN_FT request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_gid_ft {
    pub fn_resvd: __u8,
    pub fn_domain_id_scope: __u8,
    pub fn_area_id_scope: __u8,
    pub fn_fc4_type: __u8,
}

//
// GPN_FT response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_gpn_ft_resp {
    pub /: *mut *mut __u8 fp_flags; / see fp_flags definitions above,
    pub /: *mut *mut __u8 fp_fid[3]; / port ID,
    pub fp_resvd: __be32,
    pub /: *mut *mut __be64 fp_wwpn; / port name,
}

//
// GID_PN request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_gid_pn {
    pub /: *mut *mut __be64 fn_wwpn; / port name,
}

//
// GID_PN response or GSPN_ID request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_gid_pn_resp {
    pub fp_resvd: __u8,
    pub /: *mut *mut __u8 fp_fid[3]; / port ID,
}

//
// GSPN_ID response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_gspn_resp {
    pub fp_name_len: __u8,
    pub fp_name: [c_char; ],
}

//
// RFT_ID request - register FC-4 types for ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_rft_id {
    pub /: *mut *mut fc_ns_fid fr_fid; / port ID object,
    pub /: *mut *mut fc_ns_fts fr_fts; / FC-4 types object,
}

//
// RPN_ID request - register port name for ID.
// RNN_ID request - register node name for ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_rn_id {
    pub /: *mut *mut fc_ns_fid fr_fid; / port ID object,
    pub /: *mut *mut __be64 fr_wwn; / node name or port name,
    pub __attribute__((__packed__)): },
//
// RSNN_NN request - register symbolic node name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_rsnn {
    pub /: *mut *mut __be64 fr_wwn; / node name,
    pub fr_name_len: __u8,
    pub fr_name: [c_char; ],
    pub __attribute__((__packed__)): },
//
// RSPN_ID request - register symbolic port name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_rspn {
    pub /: *mut *mut fc_ns_fid fr_fid; / port ID object,
    pub fr_name_len: __u8,
    pub fr_name: [c_char; ],
    pub __attribute__((__packed__)): },
//
// RFF_ID request - register FC-4 Features for ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_rff_id {
    pub /: *mut *mut fc_ns_fid fr_fid; / port ID object,
    pub fr_resvd: [__u8; 2],
    pub /: *mut *mut __u8 fr_feat; / FC-4 Feature bits,
    pub /: *mut *mut __u8 fr_type; / FC-4 type,
    pub __attribute__((__packed__)): },
