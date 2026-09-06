//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/fc/fc_fip.h
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
//
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
//

//
// This version is based on:
// http://www.t11.org/ftp/t11/pub/fc/bb-5/08-543v1.pdf
// and T11 FC-BB-6 13-091v5.pdf (December 2013 VN2VN proposal)
//

pub const FIP_DEF_FC_MAP: c_uint = 0x0efc00 /* default FCoE MAP (MAC OUI) value */;

//
// VN2VN proposed-standard values.
//
pub const FIP_VN_FC_MAP: c_uint = 0x0efd00 /* MAC OUI for VN2VN use */;

//
// Multicast MAC addresses.  T11-adopted.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_header {
    pub /: *mut *mut __u8 fip_ver; / upper 4 bits are the version,
    pub /: *mut *mut __u8 fip_resv1; / reserved,
    pub /: *mut *mut __be16 fip_op; / operation code,
    pub /: *mut *mut __u8 fip_resv2; / reserved,
    pub /: *mut *mut __u8 fip_subcode; / lower 4 bits are sub-code,
    pub /: *mut *mut __be16 fip_dl_len; / length of descriptors in words,
    pub /: *mut *mut __be16 fip_flags; / header flags,
    pub __attribute__((packed)): },
pub const FIP_VER_SHIFT: c_int = 4;

//
// fip_op.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_opcode {
    FIP_OP_DISC =	1,		/* discovery, advertisement, etc. */
    FIP_OP_LS =	2,		/* Link Service request or reply */
    FIP_OP_CTRL =	3,		/* Keep Alive / Link Reset */
    FIP_OP_VLAN =	4,		/* VLAN discovery */
    FIP_OP_VN2VN =	5,		/* VN2VN operation */
    FIP_OP_VENDOR_MIN = 0xfff8,	/* min vendor-specific opcode */
    FIP_OP_VENDOR_MAX = 0xfffe,	/* max vendor-specific opcode */
}

//
// Subcodes for FIP_OP_DISC.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_disc_subcode {
    FIP_SC_SOL =	1,		/* solicitation */
    FIP_SC_ADV =	2,		/* advertisement */
}

//
// Subcodes for FIP_OP_LS.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_trans_subcode {
    FIP_SC_REQ =	1,		/* request */
    FIP_SC_REP =	2,		/* reply */
}

//
// Subcodes for FIP_OP_RESET.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_reset_subcode {
    FIP_SC_KEEP_ALIVE = 1,		/* keep-alive from VN_Port */
    FIP_SC_CLR_VLINK = 2,		/* clear virtual link from VF_Port */
}

//
// Subcodes for FIP_OP_VLAN.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_vlan_subcode {
    FIP_SC_VL_REQ =	1,		/* vlan request */
    FIP_SC_VL_NOTE = 2,		/* vlan notification */
    FIP_SC_VL_VN2VN_NOTE = 3,	/* VN2VN vlan notification */
}

//
// Subcodes for FIP_OP_VN2VN.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_vn2vn_subcode {
    FIP_SC_VN_PROBE_REQ = 1,	/* probe request */
    FIP_SC_VN_PROBE_REP = 2,	/* probe reply */
    FIP_SC_VN_CLAIM_NOTIFY = 3,	/* claim notification */
    FIP_SC_VN_CLAIM_REP = 4,	/* claim response */
    FIP_SC_VN_BEACON = 5,		/* beacon */
}

//
// flags in header fip_flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_flag {
    FIP_FL_FPMA =	0x8000,		/* supports FPMA fabric-provided MACs */
    FIP_FL_SPMA =	0x4000,		/* supports SPMA server-provided MACs */
    FIP_FL_FCF =	0x0020,		/* originated from a controlling FCF */
    FIP_FL_FDF =	0x0010,		/* originated from an FDF */
    FIP_FL_REC_OR_P2P = 0x0008,	/* configured addr or point-to-point */
    FIP_FL_AVAIL =	0x0004,		/* available for FLOGI/ELP */
    FIP_FL_SOL =	0x0002,		/* this is a solicited message */
    FIP_FL_FPORT =	0x0001,		/* sent from an F port */
}

//
// Common descriptor header format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_desc {
    pub /: *mut *mut __u8 fip_dtype; / type - see below,
    pub /: *mut *mut __u8 fip_dlen; / length - in 32-bit words,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_desc_type {
    FIP_DT_PRI =	1,		/* priority for forwarder selection */
    FIP_DT_MAC =	2,		/* MAC address */
    FIP_DT_MAP_OUI = 3,		/* FC-MAP OUI */
    FIP_DT_NAME =	4,		/* switch name or node name */
    FIP_DT_FAB =	5,		/* fabric descriptor */
    FIP_DT_FCOE_SIZE = 6,		/* max FCoE frame size */
    FIP_DT_FLOGI =	7,		/* FLOGI request or response */
    FIP_DT_FDISC =	8,		/* FDISC request or response */
    FIP_DT_LOGO =	9,		/* LOGO request or response */
    FIP_DT_ELP =	10,		/* ELP request or response */
    FIP_DT_VN_ID =	11,		/* VN_Node Identifier */
    FIP_DT_FKA =	12,		/* advertisement keep-alive period */
    FIP_DT_VENDOR =	13,		/* vendor ID */
    FIP_DT_VLAN =	14,		/* vlan number */
    FIP_DT_FC4F =	15,		/* FC-4 features */
    FIP_DT_LIMIT,			/* max defined desc_type + 1 */
    FIP_DT_NON_CRITICAL = 128,	/* First non-critical descriptor */
    FIP_DT_CLR_VLINKS = 128,	/* Clear virtual links reason code */
    FIP_DT_VENDOR_BASE = 241,	/* first vendor-specific desc_type */
}

//
// FIP_DT_PRI - priority descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_pri_desc {
    pub fd_desc: fip_desc,
    pub fd_resvd: __u8,
    pub /: *mut *mut __u8 fd_pri; / FCF priority: higher is better,
    pub __attribute__((packed)): },
//
// FIP_DT_MAC - MAC address descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_mac_desc {
    pub fd_desc: fip_desc,
    pub fd_mac: [__u8; ETH_ALEN],
    pub __attribute__((packed)): },
//
// FIP_DT_MAP - descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_map_desc {
    pub fd_desc: fip_desc,
    pub fd_resvd: [__u8; 3],
    pub fd_map: [__u8; 3],
    pub __attribute__((packed)): },
//
// FIP_DT_NAME descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_wwn_desc {
    pub fd_desc: fip_desc,
    pub fd_resvd: [__u8; 2],
    pub /: *mut *mut __be64 fd_wwn; / 64-bit WWN, unaligned,
    pub __attribute__((packed)): },
//
// FIP_DT_FAB descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_fab_desc {
    pub fd_desc: fip_desc,
    pub /: *mut *mut __be16 fd_vfid; / virtual fabric ID,
    pub fd_resvd: __u8,
    pub /: *mut *mut __u8 fd_map[3]; / FC-MAP value,
    pub /: *mut *mut __be64 fd_wwn; / fabric name, unaligned,
    pub __attribute__((packed)): },
//
// FIP_DT_FCOE_SIZE descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_size_desc {
    pub fd_desc: fip_desc,
    pub fd_size: __be16,
    pub __attribute__((packed)): },
//
// Descriptor that encapsulates an ELS or ILS frame.
// The encapsulated frame immediately follows this header, without
// SOF, EOF, or CRC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_encaps {
    pub fd_desc: fip_desc,
    pub fd_resvd: [__u8; 2],
    pub __attribute__((packed)): },
//
// FIP_DT_VN_ID - VN_Node Identifier descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_vn_desc {
    pub fd_desc: fip_desc,
    pub fd_mac: [__u8; ETH_ALEN],
    pub fd_resvd: __u8,
    pub fd_fc_id: [__u8; 3],
    pub /: *mut *mut __be64 fd_wwpn; / port name, unaligned,
    pub __attribute__((packed)): },
//
// FIP_DT_FKA - Advertisement keep-alive period.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_fka_desc {
    pub fd_desc: fip_desc,
    pub fd_resvd: __u8,
    pub /: *mut *mut __u8 fd_flags; / bit0 is fka disable flag,
    pub /: *mut *mut __be32 fd_fka_period; / adv./keep-alive period in mS,
    pub __attribute__((packed)): },
//
// flags for fip_fka_desc.fd_flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_fka_flags {
    FIP_FKA_ADV_D =	0x01,		/* no need for FKA from ENode */
}

// FIP_DT_FKA flags
//
// FIP_DT_VLAN descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_vlan_desc {
    pub fd_desc: fip_desc,
    pub /: *mut *mut __be16 fd_vlan; / Note: highest 4 bytes are unused,
    pub __attribute__((packed)): },
//
// FIP_DT_FC4F - FC-4 features.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_fc4_feat {
    pub fd_desc: fip_desc,
    pub fd_resvd: [__u8; 2],
    pub fd_fts: fc_ns_fts,
    pub fd_ff: fc_ns_ff,
    pub __attribute__((packed)): },
//
// FIP_DT_VENDOR descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_vendor_desc {
    pub fd_desc: fip_desc,
    pub fd_resvd: [__u8; 2],
    pub fd_vendor_id: [__u8; 8],
    pub __attribute__((packed)): },
