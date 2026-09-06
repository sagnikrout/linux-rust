//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/hif_api_mib.h
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


// SPDX-License-Identifier: GPL-2.0-only or Apache-2.0
//
// WF200 hardware interface definitions
//
// Copyright (c) 2018-2020, Silicon Laboratories Inc.
//

pub const HIF_API_IPV4_ADDRESS_SIZE: c_int = 4;
pub const HIF_API_IPV6_ADDRESS_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_mib_ids {
    HIF_MIB_ID_GL_OPERATIONAL_POWER_MODE        = 0x2000,
    HIF_MIB_ID_GL_BLOCK_ACK_INFO                = 0x2001,
    HIF_MIB_ID_GL_SET_MULTI_MSG                 = 0x2002,
    HIF_MIB_ID_CCA_CONFIG                       = 0x2003,
    HIF_MIB_ID_ETHERTYPE_DATAFRAME_CONDITION    = 0x2010,
    HIF_MIB_ID_PORT_DATAFRAME_CONDITION         = 0x2011,
    HIF_MIB_ID_MAGIC_DATAFRAME_CONDITION        = 0x2012,
    HIF_MIB_ID_MAC_ADDR_DATAFRAME_CONDITION     = 0x2013,
    HIF_MIB_ID_IPV4_ADDR_DATAFRAME_CONDITION    = 0x2014,
    HIF_MIB_ID_IPV6_ADDR_DATAFRAME_CONDITION    = 0x2015,
    HIF_MIB_ID_UC_MC_BC_DATAFRAME_CONDITION     = 0x2016,
    HIF_MIB_ID_CONFIG_DATA_FILTER               = 0x2017,
    HIF_MIB_ID_SET_DATA_FILTERING               = 0x2018,
    HIF_MIB_ID_ARP_IP_ADDRESSES_TABLE           = 0x2019,
    HIF_MIB_ID_NS_IP_ADDRESSES_TABLE            = 0x201A,
    HIF_MIB_ID_RX_FILTER                        = 0x201B,
    HIF_MIB_ID_BEACON_FILTER_TABLE              = 0x201C,
    HIF_MIB_ID_BEACON_FILTER_ENABLE             = 0x201D,
    HIF_MIB_ID_GRP_SEQ_COUNTER                  = 0x2030,
    HIF_MIB_ID_TSF_COUNTER                      = 0x2031,
    HIF_MIB_ID_STATISTICS_TABLE                 = 0x2032,
    HIF_MIB_ID_COUNTERS_TABLE                   = 0x2033,
    HIF_MIB_ID_MAX_TX_POWER_LEVEL               = 0x2034,
    HIF_MIB_ID_EXTENDED_COUNTERS_TABLE          = 0x2035,
    HIF_MIB_ID_DOT11_MAC_ADDRESS                = 0x2040,
    HIF_MIB_ID_DOT11_MAX_TRANSMIT_MSDU_LIFETIME = 0x2041,
    HIF_MIB_ID_DOT11_MAX_RECEIVE_LIFETIME       = 0x2042,
    HIF_MIB_ID_DOT11_WEP_DEFAULT_KEY_ID         = 0x2043,
    HIF_MIB_ID_DOT11_RTS_THRESHOLD              = 0x2044,
    HIF_MIB_ID_SLOT_TIME                        = 0x2045,
    HIF_MIB_ID_CURRENT_TX_POWER_LEVEL           = 0x2046,
    HIF_MIB_ID_NON_ERP_PROTECTION               = 0x2047,
    HIF_MIB_ID_TEMPLATE_FRAME                   = 0x2048,
    HIF_MIB_ID_BEACON_WAKEUP_PERIOD             = 0x2049,
    HIF_MIB_ID_RCPI_RSSI_THRESHOLD              = 0x204A,
    HIF_MIB_ID_BLOCK_ACK_POLICY                 = 0x204B,
    HIF_MIB_ID_OVERRIDE_INTERNAL_TX_RATE        = 0x204C,
    HIF_MIB_ID_SET_ASSOCIATION_MODE             = 0x204D,
    HIF_MIB_ID_SET_UAPSD_INFORMATION            = 0x204E,
    HIF_MIB_ID_SET_TX_RATE_RETRY_POLICY         = 0x204F,
    HIF_MIB_ID_PROTECTED_MGMT_POLICY            = 0x2050,
    HIF_MIB_ID_SET_HT_PROTECTION                = 0x2051,
    HIF_MIB_ID_KEEP_ALIVE_PERIOD                = 0x2052,
    HIF_MIB_ID_ARP_KEEP_ALIVE_PERIOD            = 0x2053,
    HIF_MIB_ID_INACTIVITY_TIMER                 = 0x2054,
    HIF_MIB_ID_INTERFACE_PROTECTION             = 0x2055,
    HIF_MIB_ID_BEACON_STATS                     = 0x2056,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_op_power_mode {
    HIF_OP_POWER_MODE_ACTIVE    = 0x0,
    HIF_OP_POWER_MODE_DOZE      = 0x1,
    HIF_OP_POWER_MODE_QUIESCENT = 0x2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_gl_operational_power_mode {
    pub power_mode:4: u8,
    pub reserved1:3: u8,
    pub wup_ind_activation:1: u8,
    pub reserved2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_gl_set_multi_msg {
    pub enable_multi_tx_conf:1: u8,
    pub reserved1:7: u8,
    pub reserved2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_arp_ns_frame_treatment {
    HIF_ARP_NS_FILTERING_DISABLE = 0x0,
    HIF_ARP_NS_FILTERING_ENABLE  = 0x1,
    HIF_ARP_NS_REPLY_ENABLE      = 0x2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_arp_ip_addr_table {
    pub condition_idx: u8,
    pub arp_enable: u8,
    pub reserved: [u8; 2],
    pub ipv4_address: [u8; HIF_API_IPV4_ADDRESS_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_rx_filter {
    pub reserved1:1: u8,
    pub bssid_filter:1: u8,
    pub reserved2:1: u8,
    pub fwd_probe_req:1: u8,
    pub keep_alive_filter:1: u8,
    pub reserved3:3: u8,
    pub reserved4: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ie_table_entry {
    pub ie_id: u8,
    pub has_changed:1: u8,
    pub no_longer:1: u8,
    pub has_appeared:1: u8,
    pub reserved:1: u8,
    pub num_match_data:4: u8,
    pub oui: [u8; 3],
    pub match_data: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_bcn_filter_table {
    pub num_of_info_elmts: __le32,
    pub ie_table: [wfx_hif_ie_table_entry; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_beacon_filter {
    HIF_BEACON_FILTER_DISABLE  = 0x0,
    HIF_BEACON_FILTER_ENABLE   = 0x1,
    HIF_BEACON_FILTER_AUTO_ERP = 0x2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_bcn_filter_enable {
    pub enable: __le32,
    pub bcn_count: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_extended_count_table {
    pub count_drop_plcp: __le32,
    pub count_drop_fcs: __le32,
    pub count_tx_frames: __le32,
    pub count_rx_frames: __le32,
    pub count_rx_frames_failed: __le32,
    pub count_drop_decryption: __le32,
    pub count_drop_tkip_mic: __le32,
    pub count_drop_no_key: __le32,
    pub count_tx_frames_multicast: __le32,
    pub count_tx_frames_success: __le32,
    pub count_tx_frames_failed: __le32,
    pub count_tx_frames_retried: __le32,
    pub count_tx_frames_multi_retried: __le32,
    pub count_drop_duplicate: __le32,
    pub count_rts_success: __le32,
    pub count_rts_failed: __le32,
    pub count_ack_failed: __le32,
    pub count_rx_frames_multicast: __le32,
    pub count_rx_frames_success: __le32,
    pub count_drop_cmac_icv: __le32,
    pub count_drop_cmac_replay: __le32,
    pub count_drop_ccmp_replay: __le32,
    pub count_drop_bip_mic: __le32,
    pub count_rx_bcn_success: __le32,
    pub count_rx_bcn_miss: __le32,
    pub count_rx_bcn_dtim: __le32,
    pub count_rx_bcn_dtim_aid0_clr: __le32,
    pub count_rx_bcn_dtim_aid0_set: __le32,
    pub reserved: [__le32; 12],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_count_table {
    pub count_drop_plcp: __le32,
    pub count_drop_fcs: __le32,
    pub count_tx_frames: __le32,
    pub count_rx_frames: __le32,
    pub count_rx_frames_failed: __le32,
    pub count_drop_decryption: __le32,
    pub count_drop_tkip_mic: __le32,
    pub count_drop_no_key: __le32,
    pub count_tx_frames_multicast: __le32,
    pub count_tx_frames_success: __le32,
    pub count_tx_frames_failed: __le32,
    pub count_tx_frames_retried: __le32,
    pub count_tx_frames_multi_retried: __le32,
    pub count_drop_duplicate: __le32,
    pub count_rts_success: __le32,
    pub count_rts_failed: __le32,
    pub count_ack_failed: __le32,
    pub count_rx_frames_multicast: __le32,
    pub count_rx_frames_success: __le32,
    pub count_drop_cmac_icv: __le32,
    pub count_drop_cmac_replay: __le32,
    pub count_drop_ccmp_replay: __le32,
    pub count_drop_bip_mic: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_mac_address {
    pub mac_addr: [u8; ETH_ALEN],
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_wep_default_key_id {
    pub wep_default_key_id: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_dot11_rts_threshold {
    pub threshold: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_slot_time {
    pub slot_time: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_current_tx_power_level {
    pub /: *mut *mut __le32 power_level; / signed value,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_non_erp_protection {
    pub use_cts_to_self:1: u8,
    pub reserved1:7: u8,
    pub reserved2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_tmplt {
    HIF_TMPLT_PRBREQ = 0x0,
    HIF_TMPLT_BCN    = 0x1,
    HIF_TMPLT_NULL   = 0x2,
    HIF_TMPLT_QOSNUL = 0x3,
    HIF_TMPLT_PSPOLL = 0x4,
    HIF_TMPLT_PRBRES = 0x5,
    HIF_TMPLT_ARP    = 0x6,
    HIF_TMPLT_NA     = 0x7
}

pub const HIF_API_MAX_TEMPLATE_FRAME_SIZE: c_int = 700;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_template_frame {
    pub frame_type: u8,
    pub init_rate:7: u8,
    pub mode:1: u8,
    pub frame_length: __le16,
    pub frame: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_beacon_wake_up_period {
    pub wakeup_period_min: u8,
    pub receive_dtim:1: u8,
    pub reserved1:7: u8,
    pub wakeup_period_max: u8,
    pub reserved2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_rcpi_rssi_threshold {
    pub detection:1: u8,
    pub rcpi_rssi:1: u8,
    pub upperthresh:1: u8,
    pub lowerthresh:1: u8,
    pub reserved:4: u8,
    pub lower_threshold: u8,
    pub upper_threshold: u8,
    pub rolling_average_count: u8,
    pub __packed: },
pub const DEFAULT_BA_MAX_RX_BUFFER_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_block_ack_policy {
    pub block_ack_tx_tid_policy: u8,
    pub reserved1: u8,
    pub block_ack_rx_tid_policy: u8,
    pub block_ack_rx_max_buffer_size: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_mpdu_start_spacing {
    HIF_MPDU_START_SPACING_NO_RESTRIC = 0x0,
    HIF_MPDU_START_SPACING_QUARTER    = 0x1,
    HIF_MPDU_START_SPACING_HALF       = 0x2,
    HIF_MPDU_START_SPACING_ONE        = 0x3,
    HIF_MPDU_START_SPACING_TWO        = 0x4,
    HIF_MPDU_START_SPACING_FOUR       = 0x5,
    HIF_MPDU_START_SPACING_EIGHT      = 0x6,
    HIF_MPDU_START_SPACING_SIXTEEN    = 0x7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_set_association_mode {
    pub preambtype_use:1: u8,
    pub mode:1: u8,
    pub rateset:1: u8,
    pub spacing:1: u8,
    pub reserved1:4: u8,
    pub short_preamble:1: u8,
    pub reserved2:7: u8,
    pub greenfield:1: u8,
    pub reserved3:7: u8,
    pub mpdu_start_spacing: u8,
    pub basic_rate_set: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_set_uapsd_information {
    pub trig_bckgrnd:1: u8,
    pub trig_be:1: u8,
    pub trig_video:1: u8,
    pub trig_voice:1: u8,
    pub reserved1:4: u8,
    pub deliv_bckgrnd:1: u8,
    pub deliv_be:1: u8,
    pub deliv_video:1: u8,
    pub deliv_voice:1: u8,
    pub reserved2:4: u8,
    pub min_auto_trigger_interval: __le16,
    pub max_auto_trigger_interval: __le16,
    pub auto_trigger_step: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_tx_rate_retry_policy {
    pub policy_index: u8,
    pub short_retry_count: u8,
    pub long_retry_count: u8,
    pub first_rate_sel:2: u8,
    pub terminate:1: u8,
    pub count_init:1: u8,
    pub reserved1:4: u8,
    pub rate_recovery_count: u8,
    pub reserved2: [u8; 3],
    pub rates: [u8; 12],
    pub __packed: },
pub const HIF_TX_RETRY_POLICY_MAX: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_set_tx_rate_retry_policy {
    pub num_tx_rate_policies: u8,
    pub reserved: [u8; 3],
    pub tx_rate_retry_policy: [wfx_hif_tx_rate_retry_policy; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_protected_mgmt_policy {
    pub pmf_enable:1: u8,
    pub unpmf_allowed:1: u8,
    pub host_enc_auth_frames:1: u8,
    pub reserved1:5: u8,
    pub reserved2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_mib_keep_alive_period {
    pub keep_alive_period: __le16,
    pub reserved: [u8; 2],
    pub __packed: },
