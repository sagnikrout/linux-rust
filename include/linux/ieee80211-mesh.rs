//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee80211-mesh.h
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
// IEEE 802.11 mesh definitions
//
// Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
// <jkmaline@cc.hut.fi>
// Copyright (c) 2002-2003, Jouni Malinen <jkmaline@cc.hut.fi>
// Copyright (c) 2005, Devicescape Software, Inc.
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright (c) 2013 - 2014 Intel Mobile Communications GmbH
// Copyright (c) 2016 - 2017 Intel Deutschland GmbH
// Copyright (c) 2018 - 2025 Intel Corporation
//

pub const IEEE80211_MAX_MESH_ID_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211s_hdr {
    pub flags: u8,
    pub ttl: u8,
    pub seqnum: __le32,
    pub eaddr1: [u8; ETH_ALEN],
    pub eaddr2: [u8; ETH_ALEN],
    pub __aligned(2): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_hwmp_preq_target {
    pub flags: u8,
    pub addr: [u8; ETH_ALEN],
    pub sn: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_hwmp_preq_top {
    pub flags: u8,
    pub hopcount: u8,
    pub ttl: u8,
    pub preq_id: __le32,
    pub orig_addr: [u8; ETH_ALEN],
    pub orig_sn: __le32,
// optional AE, lifetime, metric, target
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_hwmp_preq_bottom {
    pub lifetime: __le32,
    pub metric: __le32,
    pub target_count: u8,
    pub targets: [ieee80211_mesh_hwmp_preq_target; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_hwmp_prep_top {
    pub flags: u8,
    pub hopcount: u8,
    pub ttl: u8,
    pub target_addr: [u8; ETH_ALEN],
    pub target_sn: __le32,
// optional Target External Address
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_hwmp_prep_bottom {
    pub lifetime: __le32,
    pub metric: __le32,
    pub orig_addr: [u8; ETH_ALEN],
    pub orig_sn: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_hwmp_perr_dst {
    pub flags: u8,
    pub addr: [u8; ETH_ALEN],
    pub sn: __le32,
// optional Destination External Address
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_hwmp_perr {
    pub ttl: u8,
    pub number_of_dst: u8,
// Destinations
    pub variable: [u8; ],
    pub __packed: },
// Mesh flags
pub const MESH_FLAGS_AE_A4: c_uint = 0x1;
pub const MESH_FLAGS_AE_A5_A6: c_uint = 0x2;
pub const MESH_FLAGS_AE: c_uint = 0x3;
pub const MESH_FLAGS_PS_DEEP: c_uint = 0x4;
// HWMP IE processing macros

//
// enum ieee80211_preq_flags - mesh PREQ element flags
//
// @IEEE80211_PREQ_PROACTIVE_PREP_FLAG: proactive PREP subfield
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_preq_flags {
    IEEE80211_PREQ_PROACTIVE_PREP_FLAG	= 1<<2,
}

//
// enum ieee80211_preq_target_flags - mesh PREQ element per target flags
//
// @IEEE80211_PREQ_TO_FLAG: target only subfield
// @IEEE80211_PREQ_USN_FLAG: unknown target HWMP sequence number subfield
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_preq_target_flags {
    IEEE80211_PREQ_TO_FLAG	= 1<<0,
    IEEE80211_PREQ_USN_FLAG	= 1<<2,
}

//
// struct ieee80211_mesh_chansw_params_ie - mesh channel switch parameters IE
// @mesh_ttl: Time To Live
// @mesh_flags: Flags
// @mesh_reason: Reason Code
// @mesh_pre_value: Precedence Value
//
// This structure represents the payload of the "Mesh Channel Switch
// Parameters element" as described in IEEE Std 802.11-2020 section
// 9.4.2.102.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_chansw_params_ie {
    pub mesh_ttl: u8,
    pub mesh_flags: u8,
    pub mesh_reason: __le16,
    pub mesh_pre_value: __le16,
    pub __packed: },
//
// struct ieee80211_meshconf_ie - Mesh Configuration element
// @meshconf_psel: Active Path Selection Protocol Identifier
// @meshconf_pmetric: Active Path Selection Metric Identifier
// @meshconf_congest: Congestion Control Mode Identifier
// @meshconf_synch: Synchronization Method Identifier
// @meshconf_auth: Authentication Protocol Identifier
// @meshconf_form: Mesh Formation Info
// @meshconf_cap: Mesh Capability (see &enum mesh_config_capab_flags)
//
// This structure represents the payload of the "Mesh Configuration
// element" as described in IEEE Std 802.11-2020 section 9.4.2.97.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_meshconf_ie {
    pub meshconf_psel: u8,
    pub meshconf_pmetric: u8,
    pub meshconf_congest: u8,
    pub meshconf_synch: u8,
    pub meshconf_auth: u8,
    pub meshconf_form: u8,
    pub meshconf_cap: u8,
    pub __packed: },
//
// enum mesh_config_capab_flags - Mesh Configuration IE capability field flags
//
// @IEEE80211_MESHCONF_CAPAB_ACCEPT_PLINKS: STA is willing to establish
// additional mesh peerings with other mesh STAs
// @IEEE80211_MESHCONF_CAPAB_FORWARDING: the STA forwards MSDUs
// @IEEE80211_MESHCONF_CAPAB_TBTT_ADJUSTING: TBTT adjustment procedure
// is ongoing
// @IEEE80211_MESHCONF_CAPAB_POWER_SAVE_LEVEL: STA is in deep sleep mode or has
// neighbors in deep sleep mode
//
// Enumerates the "Mesh Capability" as described in IEEE Std
// 802.11-2020 section 9.4.2.97.7.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mesh_config_capab_flags {
    IEEE80211_MESHCONF_CAPAB_ACCEPT_PLINKS		= 0x01,
    IEEE80211_MESHCONF_CAPAB_FORWARDING		= 0x08,
    IEEE80211_MESHCONF_CAPAB_TBTT_ADJUSTING		= 0x20,
    IEEE80211_MESHCONF_CAPAB_POWER_SAVE_LEVEL	= 0x40,
}

pub const IEEE80211_MESHCONF_FORM_CONNECTED_TO_GATE: c_uint = 0x1;
//
// mesh channel switch parameters element's flag indicator
//

//
// struct ieee80211_rann_ie - RANN (root announcement) element
// @rann_flags: Flags
// @rann_hopcount: Hop Count
// @rann_ttl: Element TTL
// @rann_addr: Root Mesh STA Address
// @rann_seq: HWMP Sequence Number
// @rann_interval: Interval
// @rann_metric: Metric
//
// This structure represents the payload of the "RANN element" as
// described in IEEE Std 802.11-2020 section 9.4.2.111.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_rann_ie {
    pub rann_flags: u8,
    pub rann_hopcount: u8,
    pub rann_ttl: u8,
    pub rann_addr: [u8; ETH_ALEN],
    pub rann_seq: __le32,
    pub rann_interval: __le32,
    pub rann_metric: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_rann_flags {
    RANN_FLAG_IS_GATE = 1 << 0,
}

// Mesh action codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_mesh_actioncode {
    WLAN_MESH_ACTION_LINK_METRIC_REPORT,
    WLAN_MESH_ACTION_HWMP_PATH_SELECTION,
    WLAN_MESH_ACTION_GATE_ANNOUNCEMENT,
    WLAN_MESH_ACTION_CONGESTION_CONTROL_NOTIFICATION,
    WLAN_MESH_ACTION_MCCA_SETUP_REQUEST,
    WLAN_MESH_ACTION_MCCA_SETUP_REPLY,
    WLAN_MESH_ACTION_MCCA_ADVERTISEMENT_REQUEST,
    WLAN_MESH_ACTION_MCCA_ADVERTISEMENT,
    WLAN_MESH_ACTION_MCCA_TEARDOWN,
    WLAN_MESH_ACTION_TBTT_ADJUSTMENT_REQUEST,
    WLAN_MESH_ACTION_TBTT_ADJUSTMENT_RESPONSE,
}

//
// enum ieee80211_mesh_sync_method - mesh synchronization method identifier
//
// @IEEE80211_SYNC_METHOD_NEIGHBOR_OFFSET: the default synchronization method
// @IEEE80211_SYNC_METHOD_VENDOR: a vendor specific synchronization method
// that will be specified in a vendor specific information element
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_mesh_sync_method {
    IEEE80211_SYNC_METHOD_NEIGHBOR_OFFSET = 1,
    IEEE80211_SYNC_METHOD_VENDOR = 255,
}

//
// enum ieee80211_mesh_path_protocol - mesh path selection protocol identifier
//
// @IEEE80211_PATH_PROTOCOL_HWMP: the default path selection protocol
// @IEEE80211_PATH_PROTOCOL_VENDOR: a vendor specific protocol that will
// be specified in a vendor specific information element
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_mesh_path_protocol {
    IEEE80211_PATH_PROTOCOL_HWMP = 1,
    IEEE80211_PATH_PROTOCOL_VENDOR = 255,
}

//
// enum ieee80211_mesh_path_metric - mesh path selection metric identifier
//
// @IEEE80211_PATH_METRIC_AIRTIME: the default path selection metric
// @IEEE80211_PATH_METRIC_VENDOR: a vendor specific metric that will be
// specified in a vendor specific information element
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_mesh_path_metric {
    IEEE80211_PATH_METRIC_AIRTIME = 1,
    IEEE80211_PATH_METRIC_VENDOR = 255,
}

//
// enum ieee80211_root_mode_identifier - root mesh STA mode identifier
//
// These attribute are used by dot11MeshHWMPRootMode to set root mesh STA mode
//
// @IEEE80211_ROOTMODE_NO_ROOT: the mesh STA is not a root mesh STA (default)
// @IEEE80211_ROOTMODE_ROOT: the mesh STA is a root mesh STA if greater than
// this value
// @IEEE80211_PROACTIVE_PREQ_NO_PREP: the mesh STA is a root mesh STA supports
// the proactive PREQ with proactive PREP subfield set to 0
// @IEEE80211_PROACTIVE_PREQ_WITH_PREP: the mesh STA is a root mesh STA
// supports the proactive PREQ with proactive PREP subfield set to 1
// @IEEE80211_PROACTIVE_RANN: the mesh STA is a root mesh STA supports
// the proactive RANN
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_root_mode_identifier {
    IEEE80211_ROOTMODE_NO_ROOT = 0,
    IEEE80211_ROOTMODE_ROOT = 1,
    IEEE80211_PROACTIVE_PREQ_NO_PREP = 2,
    IEEE80211_PROACTIVE_PREQ_WITH_PREP = 3,
    IEEE80211_PROACTIVE_RANN = 4,
}

    pub AE_F: return ie[0] &,
    pub )ie: *mut *mut ieee80211_mesh_hwmp_preq_top top = (void,
    pub 0]: ieee80211_mesh_preq_prep_ae_enabled(ie) ? ETH_ALEN :,
    pub )ie: *mut *mut ieee80211_mesh_hwmp_prep_top top = (void,
    pub 0]: ieee80211_mesh_preq_prep_ae_enabled(ie) ? ETH_ALEN :,
    pub )ie: *mut *mut ieee80211_mesh_hwmp_perr perr_ie = (void,
    pub dst: *mut ieee80211_mesh_hwmp_perr_dst,
    pub perr_ie->variable: *mut *mut u8 pos =,
    pub i: c_int,
    pub {: for (i = 0; i < dst_idx + 1; i++),
    pub )pos: *mut dst = (void,
// Destination External Address */ +
    pub /: *mut *mut 2 / Reason Code,
    pub dst: return,
    pub dst_idx): ieee80211_mesh_hwmp_perr_get_dst(ie,,
    pub dst->addr: return,
    pub dst_idx): ieee80211_mesh_hwmp_perr_get_dst(ie,,
    pub le32_to_cpu(dst->sn): return,
    pub dst_idx): ieee80211_mesh_hwmp_perr_get_dst(ie,,
    pub 0]): (dst->flags & AE_F) ? ETH_ALEN :,
// IEEE Std 802.11-2016 9.4.2.113 PREQ element
    pub target_count: u8,
    pub needed: c_int,
// Check if the element contains flags
    pub ieee80211_mesh_hwmp_preq_top): needed = sizeof(struct,
    pub false: return,
// Check if the element contains target_count
// Originator External Address */ +
    pub ieee80211_mesh_hwmp_preq_bottom): sizeof(struct,
    pub false: return,
    pub preq_elem_bottom->target_count: target_count =,
// IEEE Std 802.11-2016 Table 14-10 to 14-16
    pub false: return,
    pub ieee80211_mesh_hwmp_preq_target): *mut *mut needed += target_count  sizeof(struct,
    pub needed: return elen ==,
// IEEE Std 802.11-2016 9.4.2.114 PREP element
    pub needed: u8,
// Check if the element contains flags
    pub ieee80211_mesh_hwmp_prep_top): needed = sizeof(struct,
    pub false: return,
// Target External Address */ +
    pub ieee80211_mesh_hwmp_prep_bottom): sizeof(struct,
    pub needed: return elen ==,
// IEEE Std 802.11-2016 9.4.2.115 PERR element
    pub )pos: *mut *mut ieee80211_mesh_hwmp_perr perr_elem = (void,
    pub pos: *const *const u8 start =,
    pub number_of_dst: u8,
    pub needed: c_int,
    pub i: c_int,
    pub ieee80211_mesh_hwmp_perr): needed = sizeof(struct,
// Check if the element contains number of dst
    pub false: return,
    pub ieee80211_mesh_hwmp_perr): pos += sizeof(struct,
    pub perr_elem->number_of_dst: number_of_dst =,
    pub {: for (i = 0; i < number_of_dst; i++),
    pub )pos: *mut *mut ieee80211_mesh_hwmp_perr_dst dst = (void,
    pub ieee80211_mesh_hwmp_perr_dst): u8 dst_len = sizeof(struct,
// Check if the element contains flags
    pub false: return,
// Destination External Address */ +
    pub /: *mut *mut 2 / Reason Code,
    pub dst_len: needed +=,
    pub dst_len: pos +=,
    pub needed: return elen ==,
