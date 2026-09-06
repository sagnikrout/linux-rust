//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_dcb.h
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

pub const ICE_DCBX_STATUS_NOT_STARTED: c_int = 0;
pub const ICE_DCBX_STATUS_IN_PROGRESS: c_int = 1;
pub const ICE_DCBX_STATUS_DONE: c_int = 2;
pub const ICE_DCBX_STATUS_DIS: c_int = 7;
pub const ICE_TLV_TYPE_END: c_int = 0;
pub const ICE_TLV_TYPE_ORG: c_int = 127;
pub const ICE_IEEE_8021QAZ_OUI: c_uint = 0x0080C2;
pub const ICE_IEEE_SUBTYPE_ETS_CFG: c_int = 9;
pub const ICE_IEEE_SUBTYPE_ETS_REC: c_int = 10;
pub const ICE_IEEE_SUBTYPE_PFC_CFG: c_int = 11;
pub const ICE_IEEE_SUBTYPE_APP_PRI: c_int = 12;
pub const ICE_CEE_DCBX_OUI: c_uint = 0x001B21;
pub const ICE_CEE_DCBX_TYPE: c_int = 2;
pub const ICE_DSCP_OUI: c_uint = 0xFFFFFF;
pub const ICE_DSCP_SUBTYPE_DSCP2UP: c_uint = 0x41;
pub const ICE_DSCP_SUBTYPE_ENFORCE: c_uint = 0x42;
pub const ICE_DSCP_SUBTYPE_TCBW: c_uint = 0x43;
pub const ICE_DSCP_SUBTYPE_PFC: c_uint = 0x44;
pub const ICE_DSCP_IPV6_OFFSET: c_int = 80;
pub const ICE_CEE_SUBTYPE_PG_CFG: c_int = 2;
pub const ICE_CEE_SUBTYPE_PFC_CFG: c_int = 3;
pub const ICE_CEE_SUBTYPE_APP_PRI: c_int = 4;
pub const ICE_CEE_MAX_FEAT_TYPE: c_int = 3;
// Defines for LLDP TLV header
pub const ICE_LLDP_TLV_LEN_S: c_int = 0;

pub const ICE_LLDP_TLV_TYPE_S: c_int = 9;

pub const ICE_LLDP_TLV_SUBTYPE_S: c_int = 0;

pub const ICE_LLDP_TLV_OUI_S: c_int = 8;

// Defines for IEEE ETS TLV
pub const ICE_IEEE_ETS_MAXTC_S: c_int = 0;

pub const ICE_IEEE_ETS_CBS_S: c_int = 6;

pub const ICE_IEEE_ETS_WILLING_S: c_int = 7;

pub const ICE_IEEE_ETS_PRIO_0_S: c_int = 0;

pub const ICE_IEEE_ETS_PRIO_1_S: c_int = 4;

pub const ICE_CEE_PGID_PRIO_0_S: c_int = 0;

pub const ICE_CEE_PGID_PRIO_1_S: c_int = 4;

pub const ICE_CEE_PGID_STRICT: c_int = 15;
// Defines for IEEE TSA types
pub const ICE_IEEE_TSA_STRICT: c_int = 0;
pub const ICE_IEEE_TSA_ETS: c_int = 2;
// Defines for IEEE PFC TLV
pub const ICE_IEEE_PFC_CAP_S: c_int = 0;

pub const ICE_IEEE_PFC_MBC_S: c_int = 6;

pub const ICE_IEEE_PFC_WILLING_S: c_int = 7;

// Defines for IEEE APP TLV
pub const ICE_IEEE_APP_SEL_S: c_int = 0;

pub const ICE_IEEE_APP_PRIO_S: c_int = 5;

// TLV definitions for preparing MIB
pub const ICE_IEEE_TLV_ID_ETS_CFG: c_int = 3;
pub const ICE_IEEE_TLV_ID_ETS_REC: c_int = 4;
pub const ICE_IEEE_TLV_ID_PFC_CFG: c_int = 5;
pub const ICE_IEEE_TLV_ID_APP_PRI: c_int = 6;
pub const ICE_TLV_ID_END_OF_LLDPPDU: c_int = 7;

pub const ICE_TLV_ID_DSCP_UP: c_int = 3;
pub const ICE_TLV_ID_DSCP_ENF: c_int = 4;
pub const ICE_TLV_ID_DSCP_TC_BW: c_int = 5;
pub const ICE_TLV_ID_DSCP_TO_PFC: c_int = 6;
pub const ICE_IEEE_ETS_TLV_LEN: c_int = 25;
pub const ICE_IEEE_PFC_TLV_LEN: c_int = 6;
pub const ICE_IEEE_APP_TLV_LEN: c_int = 11;
pub const ICE_DSCP_UP_TLV_LEN: c_int = 148;
pub const ICE_DSCP_ENF_TLV_LEN: c_int = 132;
pub const ICE_DSCP_TC_BW_TLV_LEN: c_int = 25;
pub const ICE_DSCP_PFC_TLV_LEN: c_int = 6;
// IEEE 802.1AB LLDP Organization specific TLV
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_lldp_org_tlv {
    pub typelen: __be16,
    pub ouisubtype: __be32,
    pub tlvinfo: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_cee_tlv_hdr {
    pub typelen: __be16,
    pub operver: u8,
    pub maxver: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_cee_ctrl_tlv {
    pub hdr: ice_cee_tlv_hdr,
    pub seqno: __be32,
    pub ackno: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_cee_feat_tlv {
    pub hdr: ice_cee_tlv_hdr,
    pub /: *mut *mut u8 en_will_err; / Bits: |En|Will|Err|Reserved(5)|,
pub const ICE_CEE_FEAT_TLV_ENA_M: c_uint = 0x80;
pub const ICE_CEE_FEAT_TLV_WILLING_M: c_uint = 0x40;
pub const ICE_CEE_FEAT_TLV_ERR_M: c_uint = 0x20;
    pub subtype: u8,
    pub tlvinfo: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_cee_app_prio {
    pub protocol: __be16,
    pub /: *mut *mut u8 upper_oui_sel; / Bits: |Upper OUI(6)|Selector(2)|,
pub const ICE_CEE_APP_SELECTOR_M: c_uint = 0x03;
    pub lower_oui: __be16,
    pub prio_map: u8,
    pub __packed: },
    pub cd): *mut *mut int ice_aq_set_pfc_mode(struct ice_hw hw, u8 pfc_mode, struct ice_sq_cd,
    pub dcbcfg): *mut ice_dcbx_cfg,
    pub pi): *mut int ice_get_dcb_cfg(struct ice_port_info,
    pub pi): *mut int ice_set_dcb_cfg(struct ice_port_info,
    pub event): *mut ice_rq_event_info,
    pub enable_mib_change): *mut *mut int ice_init_dcb(struct ice_hw hw, bool,
    pub cmd_details): *mut ice_sq_cd,

    pub cd): *mut ice_sq_cd,
    pub cd): *mut *mut int ice_aq_start_lldp(struct ice_hw hw, bool persist, struct ice_sq_cd,
    pub cd): *mut *mut bool dcbx_agent_status, struct ice_sq_cd,
    pub ena_mib): *mut *mut int ice_cfg_lldp_mib_change(struct ice_hw hw, bool,

    pub 0: return,
    pub 0: return,
// dcbx_agent_status = false;
    pub 0: return,
    pub 0: return,

