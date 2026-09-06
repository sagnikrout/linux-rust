//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfa_defs_cna.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

// FC physical port statistics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_fc_stats {
    pub /: *mut *mut u64 secs_reset; /!< Seconds since stats is reset,
    pub /: *mut *mut u64 tx_frames; /!< Tx frames,
    pub /: *mut *mut u64 tx_words; /!< Tx words,
    pub /: *mut *mut u64 tx_lip; /!< Tx LIP,
    pub /: *mut *mut u64 tx_nos; /!< Tx NOS,
    pub /: *mut *mut u64 tx_ols; /!< Tx OLS,
    pub /: *mut *mut u64 tx_lr; /!< Tx LR,
    pub /: *mut *mut u64 tx_lrr; /!< Tx LRR,
    pub /: *mut *mut u64 rx_frames; /!< Rx frames,
    pub /: *mut *mut u64 rx_words; /!< Rx words,
    pub /: *mut *mut u64 lip_count; /!< Rx LIP,
    pub /: *mut *mut u64 nos_count; /!< Rx NOS,
    pub /: *mut *mut u64 ols_count; /!< Rx OLS,
    pub /: *mut *mut u64 lr_count; /!< Rx LR,
    pub /: *mut *mut u64 lrr_count; /!< Rx LRR,
    pub /: *mut *mut u64 invalid_crcs; /!< Rx CRC err frames,
    pub /: *mut *mut u64 invalid_crc_gd_eof; /!< Rx CRC err good EOF frames,
    pub /: *mut *mut u64 undersized_frm; /!< Rx undersized frames,
    pub /: *mut *mut u64 oversized_frm; /!< Rx oversized frames,
    pub /: *mut *mut u64 bad_eof_frm; /!< Rx frames with bad EOF,
    pub /: *mut *mut u64 error_frames; /!< Errored frames,
    pub /: *mut *mut u64 dropped_frames; /!< Dropped frames,
    pub /: *mut *mut u64 link_failures; /!< Link Failure (LF) count,
    pub /: *mut *mut u64 loss_of_syncs; /!< Loss of sync count,
    pub /: *mut *mut u64 loss_of_signals; /!< Loss of signal count,
    pub /: *mut *mut u64 primseq_errs; /!< Primitive sequence protocol err.,
    pub /: *mut *mut u64 bad_os_count; /!< Invalid ordered sets,
    pub /: *mut *mut u64 err_enc_out; /!< Encoding err nonframe_8b10b,
    pub /: *mut *mut u64 err_enc; /!< Encoding err frame_8b10b,
    pub /: *mut *mut u64 bbsc_frames_lost; /!< Credit Recovery-Frames Lost,
    pub /: *mut *mut u64 bbsc_credits_lost; /!< Credit Recovery-Credits Lost,
    pub /: *mut *mut u64 bbsc_link_resets; /!< Credit Recovery-Link Resets,
}

// Eth Physical Port statistics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_eth_stats {
    pub /: *mut *mut u64 secs_reset; /!< Seconds since stats is reset,
    pub /: *mut *mut u64 frame_64; /!< Frames 64 bytes,
    pub /: *mut *mut u64 frame_65_127; /!< Frames 65-127 bytes,
    pub /: *mut *mut u64 frame_128_255; /!< Frames 128-255 bytes,
    pub /: *mut *mut u64 frame_256_511; /!< Frames 256-511 bytes,
    pub /: *mut *mut u64 frame_512_1023; /!< Frames 512-1023 bytes,
    pub /: *mut *mut u64 frame_1024_1518; /!< Frames 1024-1518 bytes,
    pub /: *mut *mut u64 frame_1519_1522; /!< Frames 1519-1522 bytes,
    pub /: *mut *mut u64 tx_bytes; /!< Tx bytes,
    pub /: *mut *mut u64 tx_packets; /!< Tx packets,
    pub /: *mut *mut u64 tx_mcast_packets; /!< Tx multicast packets,
    pub /: *mut *mut u64 tx_bcast_packets; /!< Tx broadcast packets,
    pub /: *mut *mut u64 tx_control_frame; /!< Tx control frame,
    pub /: *mut *mut u64 tx_drop; /!< Tx drops,
    pub /: *mut *mut u64 tx_jabber; /!< Tx jabber,
    pub /: *mut *mut u64 tx_fcs_error; /!< Tx FCS errors,
    pub /: *mut *mut u64 tx_fragments; /!< Tx fragments,
    pub /: *mut *mut u64 rx_bytes; /!< Rx bytes,
    pub /: *mut *mut u64 rx_packets; /!< Rx packets,
    pub /: *mut *mut u64 rx_mcast_packets; /!< Rx multicast packets,
    pub /: *mut *mut u64 rx_bcast_packets; /!< Rx broadcast packets,
    pub /: *mut *mut u64 rx_control_frames; /!< Rx control frames,
    pub /: *mut *mut u64 rx_unknown_opcode; /!< Rx unknown opcode,
    pub /: *mut *mut u64 rx_drop; /!< Rx drops,
    pub /: *mut *mut u64 rx_jabber; /!< Rx jabber,
    pub /: *mut *mut u64 rx_fcs_error; /!< Rx FCS errors,
    pub /: *mut *mut u64 rx_alignment_error; /!< Rx alignment errors,
    pub /: *mut *mut u64 rx_frame_length_error; /!< Rx frame len errors,
    pub /: *mut *mut u64 rx_code_error; /!< Rx code errors,
    pub /: *mut *mut u64 rx_fragments; /!< Rx fragments,
    pub /: *mut *mut u64 rx_pause; /!< Rx pause,
    pub /: *mut *mut u64 rx_zero_pause; /!< Rx zero pause,
    pub /: *mut *mut u64 tx_pause; /!< Tx pause,
    pub /: *mut *mut u64 tx_zero_pause; /!< Tx zero pause,
    pub /: *mut *mut u64 rx_fcoe_pause; /!< Rx FCoE pause,
    pub /: *mut *mut u64 rx_fcoe_zero_pause; /!< Rx FCoE zero pause,
    pub /: *mut *mut u64 tx_fcoe_pause; /!< Tx FCoE pause,
    pub /: *mut *mut u64 tx_fcoe_zero_pause; /!< Tx FCoE zero pause,
    pub /: *mut *mut u64 rx_iscsi_pause; /!< Rx iSCSI pause,
    pub /: *mut *mut u64 rx_iscsi_zero_pause; /!< Rx iSCSI zero pause,
    pub /: *mut *mut u64 tx_iscsi_pause; /!< Tx iSCSI pause,
    pub /: *mut *mut u64 tx_iscsi_zero_pause; /!< Tx iSCSI zero pause,
}

// Port statistics.
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfa_port_stats_u {
    pub fc: bfa_port_fc_stats,
    pub eth: bfa_port_eth_stats,
}

pub const BFA_CEE_LLDP_SYS_CAP_OTHER: c_uint = 0x0001;
pub const BFA_CEE_LLDP_SYS_CAP_REPEATER: c_uint = 0x0002;
pub const BFA_CEE_LLDP_SYS_CAP_MAC_BRIDGE: c_uint = 0x0004;
pub const BFA_CEE_LLDP_SYS_CAP_WLAN_AP: c_uint = 0x0008;
pub const BFA_CEE_LLDP_SYS_CAP_ROUTER: c_uint = 0x0010;
pub const BFA_CEE_LLDP_SYS_CAP_TELEPHONE: c_uint = 0x0020;
pub const BFA_CEE_LLDP_SYS_CAP_DOCSIS_CD: c_uint = 0x0040;
pub const BFA_CEE_LLDP_SYS_CAP_STATION: c_uint = 0x0080;
pub const BFA_CEE_LLDP_SYS_CAP_CVLAN: c_uint = 0x0100;
pub const BFA_CEE_LLDP_SYS_CAP_SVLAN: c_uint = 0x0200;
pub const BFA_CEE_LLDP_SYS_CAP_TPMR: c_uint = 0x0400;
// LLDP string type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_lldp_str {
    pub sub_type: u8,
    pub len: u8,
    pub rsvd: [u8; 2],
    pub value: [u8; BFA_CEE_LLDP_MAX_STRING_LEN],
    pub __packed: },
// LLDP parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_lldp_cfg {
    pub chassis_id: bfa_cee_lldp_str,
    pub port_id: bfa_cee_lldp_str,
    pub port_desc: bfa_cee_lldp_str,
    pub sys_name: bfa_cee_lldp_str,
    pub sys_desc: bfa_cee_lldp_str,
    pub mgmt_addr: bfa_cee_lldp_str,
    pub time_to_live: u16,
    pub enabled_system_cap: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_cee_dcbx_version {
    DCBX_PROTOCOL_PRECEE	= 1,
    DCBX_PROTOCOL_CEE	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_cee_lls {
// LLS is down because the TLV not sent by the peer
    CEE_LLS_DOWN_NO_TLV = 0,
// LLS is down as advertised by the peer
    CEE_LLS_DOWN	= 1,
    CEE_LLS_UP	= 2,
}

// CEE/DCBX parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_dcbx_cfg {
    pub pgid: [u8; BFA_CEE_DCBX_MAX_PRIORITY],
    pub pg_percentage: [u8; BFA_CEE_DCBX_MAX_PGID],
    pub /: *mut *mut u8 pfc_primap; / bitmap of priorties with PFC enabled,
    pub /: *mut *mut u8 fcoe_primap; / bitmap of priorities used for FcoE traffic,
    pub /: *mut *mut u8 iscsi_primap; / bitmap of priorities used for iSCSI traffic,
    pub /: *mut *mut u8 dcbx_version; / operating version:CEE or preCEE,
    pub /: *mut *mut u8 lls_fcoe; / FCoE Logical Link Status,
    pub /: *mut *mut u8 lls_lan; / LAN Logical Link Status,
    pub rsvd: [u8; 2],
    pub __packed: },
// CEE status
// Making this to tri-state for the benefit of port list command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_cee_status {
    CEE_UP = 0,
    CEE_PHY_UP = 1,
    CEE_LOOPBACK = 2,
    CEE_PHY_DOWN = 3,
}

// CEE Query
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_attr {
    pub cee_status: u8,
    pub error_reason: u8,
    pub lldp_remote: bfa_cee_lldp_cfg,
    pub dcbx_remote: bfa_cee_dcbx_cfg,
    pub src_mac: [u8; ETH_ALEN],
    pub link_speed: u8,
    pub nw_priority: u8,
    pub filler: [u8; 2],
    pub __packed: },
// LLDP/DCBX/CEE Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_stats {
    pub /: *mut *mut u32 lldp_tx_frames; /!< LLDP Tx Frames,
    pub /: *mut *mut u32 lldp_rx_frames; /!< LLDP Rx Frames,
    pub /: *mut *mut u32 lldp_rx_frames_invalid; /!< LLDP Rx Frames invalid,
    pub /: *mut *mut u32 lldp_rx_frames_new; /!< LLDP Rx Frames new,
    pub /: *mut *mut u32 lldp_tlvs_unrecognized; /!< LLDP Rx unrecognized TLVs,
    pub /: *mut *mut u32 lldp_rx_shutdown_tlvs; /!< LLDP Rx shutdown TLVs,
    pub /: *mut *mut u32 lldp_info_aged_out; /!< LLDP remote info aged out,
    pub /: *mut *mut u32 dcbx_phylink_ups; /!< DCBX phy link ups,
    pub /: *mut *mut u32 dcbx_phylink_downs; /!< DCBX phy link downs,
    pub /: *mut *mut u32 dcbx_rx_tlvs; /!< DCBX Rx TLVs,
    pub /: *mut *mut u32 dcbx_rx_tlvs_invalid; /!< DCBX Rx TLVs invalid,
    pub /: *mut *mut u32 dcbx_control_tlv_error; /!< DCBX control TLV errors,
    pub /: *mut *mut u32 dcbx_feature_tlv_error; /!< DCBX feature TLV errors,
    pub /: *mut *mut u32 dcbx_cee_cfg_new; /!< DCBX new CEE cfg rcvd,
    pub /: *mut *mut u32 cee_status_down; /!< CEE status down,
    pub /: *mut *mut u32 cee_status_up; /!< CEE status up,
    pub /: *mut *mut u32 cee_hw_cfg_changed; /!< CEE hw cfg changed,
    pub /: *mut *mut u32 cee_rx_invalid_cfg; /!< CEE invalid cfg,
    pub __packed: },
