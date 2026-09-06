//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/acx.h
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
// This file is part of wl1271
//
// Copyright (C) 1998-2009 Texas Instruments. All rights reserved.
// Copyright (C) 2008-2010 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

//
// HW Initiated interrupt Watchdog timer expiration

// Init sequence is done (masked interrupt, detection through polling only )

// Event was entered to Event MBOX #A

// Event was entered to Event MBOX #B

// Command processing completion

// Signaling the host on HW wakeup

// The MISC bit is used for aggregation of RX, TxComplete and TX rate update

// Trace message on MBOX #A

// Trace message on MBOX #B

// SW FW Initiated interrupt Watchdog timer expiration

pub const WL1271_ACX_INTR_ALL: c_uint = 0xFFFFFFFF;
// all possible interrupts - only appropriate ones will be masked in

// Target's information element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_header {
    pub cmd: wl1271_cmd_header,
// acx (or information element) header
    pub id: __le16,
// payload length (not including headers
    pub len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_error_counter {
    pub header: acx_header,
// The number of PLCP errors since the last time this
// information element was interrogated. This field is
// automatically cleared when it is interrogated.
    pub PLCP_error: __le32,
// The number of FCS errors since the last time this
// information element was interrogated. This field is
// automatically cleared when it is interrogated.
    pub FCS_error: __le32,
// The number of MPDUs without PLCP header errors received
// since the last time this information element was interrogated.
// This field is automatically cleared when it is interrogated.
    pub valid_frame: __le32,
// the number of missed sequence numbers in the squentially
// values of frames seq numbers
    pub seq_num_miss: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl12xx_role {
    WL1271_ROLE_STA = 0,
    WL1271_ROLE_IBSS,
    WL1271_ROLE_AP,
    WL1271_ROLE_DEVICE,
    WL1271_ROLE_P2P_CL,
    WL1271_ROLE_P2P_GO,
    WL1271_ROLE_MESH_POINT,

    WL12XX_INVALID_ROLE_TYPE = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_psm_mode {
// Active mode
    WL1271_PSM_CAM = 0,

// Power save mode
    WL1271_PSM_PS = 1,

// Extreme low power
    WL1271_PSM_ELP = 2,

    WL1271_PSM_MAX = WL1271_PSM_ELP,

// illegal out of band value of PSM mode
    WL1271_PSM_ILLEGAL = 0xff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_sleep_auth {
    pub header: acx_header,
// The sleep level authorization of the device.
// 0 - Always active
// 1 - Power down mode: light / fast sleep
// 2 - ELP mode: Deep / Max sleep
    pub sleep_auth: u8,
    pub padding: [u8; 3],
    pub __packed: },
}

pub const DEFAULT_UCAST_PRIORITY: c_int = 0;
pub const DEFAULT_RX_Q_PRIORITY: c_int = 0;

pub const DEFAULT_RXQ_TYPE: c_uint = 0x07    /* All frames, Data/Ctrl/Mgmt */;
pub const TRACE_BUFFER_MAX_SIZE: c_int = 256;
pub const DP_RX_PACKET_RING_CHUNK_SIZE: c_int = 1600;
pub const DP_TX_PACKET_RING_CHUNK_SIZE: c_int = 1600;
pub const DP_RX_PACKET_RING_CHUNK_NUM: c_int = 2;
pub const DP_TX_PACKET_RING_CHUNK_NUM: c_int = 2;
pub const DP_TX_COMPLETE_TIME_OUT: c_int = 20;
pub const TX_MSDU_LIFETIME_MIN: c_int = 0;
pub const TX_MSDU_LIFETIME_MAX: c_int = 3000;
pub const TX_MSDU_LIFETIME_DEF: c_int = 512;
pub const RX_MSDU_LIFETIME_MIN: c_int = 0;
pub const RX_MSDU_LIFETIME_MAX: c_uint = 0xFFFFFFFF;
pub const RX_MSDU_LIFETIME_DEF: c_int = 512000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rx_msdu_lifetime {
    pub header: acx_header,
//
// The maximum amount of time, in TU, before the
// firmware discards the MSDU.
//
    pub lifetime: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acx_slot_type {
    SLOT_TIME_LONG = 0,
    SLOT_TIME_SHORT = 1,
    DEFAULT_SLOT_TIME = SLOT_TIME_SHORT,
    MAX_SLOT_TIMES = 0xFF
}

pub const STATION_WONE_INDEX: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_slot {
    pub header: acx_header,
    pub role_id: u8,
    pub /: *mut *mut u8 wone_index; / Reserved,
    pub slot_time: u8,
    pub reserved: [u8; 5],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_dot11_grp_addr_tbl {
    pub header: acx_header,
    pub role_id: u8,
    pub enabled: u8,
    pub num_groups: u8,
    pub pad: [u8; 1],
    pub mac_table: [u8; ADDRESS_GROUP_MAX_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rx_timeout {
    pub header: acx_header,
    pub role_id: u8,
    pub reserved: u8,
    pub ps_poll_timeout: __le16,
    pub upsd_timeout: __le16,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rts_threshold {
    pub header: acx_header,
    pub role_id: u8,
    pub reserved: u8,
    pub threshold: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_beacon_filter_option {
    pub header: acx_header,
    pub role_id: u8,
    pub enable: u8,
//
// The number of beacons without the unicast TIM
// bit set that the firmware buffers before
// signaling the host about ready frames.
// When set to 0 and the filter is enabled, beacons
// without the unicast TIM bit set are dropped.
//
    pub max_num_beacons: u8,
    pub pad: [u8; 1],
    pub __packed: },
//
// ACXBeaconFilterEntry (not 221)
// Byte Offset     Size (Bytes)    Definition
// ===========     ============    ==========
// 0               1               IE identifier
// 1               1               Treatment bit mask
//
// ACXBeaconFilterEntry (221)
// Byte Offset     Size (Bytes)    Definition
// ===========     ============    ==========
// 0               1               IE identifier
// 1               1               Treatment bit mask
// 2               3               OUI
// 5               1               Type
// 6               2               Version
//
// Treatment bit mask - The information element handling:
// bit 0 - The information element is compared and transferred
// in case of change.
// bit 1 - The information element is transferred to the host
// with each appearance or disappearance.
// Note that both bits can be set at the same time.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_beacon_filter_ie_table {
    pub header: acx_header,
    pub role_id: u8,
    pub num_ie: u8,
    pub pad: [u8; 2],
    pub table: [u8; BEACON_FILTER_TABLE_MAX_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_conn_monit_params {
    pub header: acx_header,
    pub role_id: u8,
    pub padding: [u8; 3],
    pub /: *mut *mut __le32 synch_fail_thold; / number of beacons missed,
    pub /: *mut *mut __le32 bss_lose_timeout; / number of TU's from synch fail,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_bt_wlan_coex {
    pub header: acx_header,
    pub enable: u8,
    pub pad: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_bt_wlan_coex_param {
    pub header: acx_header,
    pub params: [__le32; WLCORE_CONF_SG_PARAMS_MAX],
    pub param_idx: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_dco_itrim_params {
    pub header: acx_header,
    pub enable: u8,
    pub padding: [u8; 3],
    pub timeout: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_energy_detection {
    pub header: acx_header,
// The RX Clear Channel Assessment threshold in the PHY
    pub rx_cca_threshold: __le16,
    pub tx_energy_detection: u8,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_beacon_broadcast {
    pub header: acx_header,
    pub role_id: u8,
// Enables receiving of broadcast packets in PS mode
    pub rx_broadcast_in_ps: u8,
    pub beacon_rx_timeout: __le16,
    pub broadcast_timeout: __le16,
// Consecutive PS Poll failures before updating the host
    pub ps_poll_threshold: u8,
    pub pad: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_event_mask {
    pub header: acx_header,
    pub event_mask: __le32,
    pub /: *mut *mut __le32 high_event_mask; / Unused,
    pub __packed: },

// When set, disable HW encryption
pub const DF_ENCRYPTION_DISABLE: c_uint = 0x01;
pub const DF_SNIFF_MODE_ENABLE: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_feature_config {
    pub header: acx_header,
    pub role_id: u8,
    pub padding: [u8; 3],
    pub options: __le32,
    pub data_flow_options: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_current_tx_power {
    pub header: acx_header,
    pub role_id: u8,
    pub current_tx_power: u8,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_wake_up_condition {
    pub header: acx_header,
    pub role_id: u8,
    pub /: *mut *mut u8 wake_up_event; / Only one bit can be set,
    pub listen_interval: u8,
    pub pad: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_aid {
    pub header: acx_header,
//
// To be set when associated with an AP.
//
    pub role_id: u8,
    pub reserved: u8,
    pub aid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acx_preamble_type {
    ACX_PREAMBLE_LONG = 0,
    ACX_PREAMBLE_SHORT = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_preamble {
    pub header: acx_header,
//
// When set, the WiLink transmits the frames with a short preamble and
// when cleared, the WiLink transmits the frames with a long preamble.
//
    pub role_id: u8,
    pub preamble: u8,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acx_ctsprotect_type {
    CTSPROTECT_DISABLE = 0,
    CTSPROTECT_ENABLE = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_ctsprotect {
    pub header: acx_header,
    pub role_id: u8,
    pub ctsprotect: u8,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rate_class {
    pub enabled_rates: __le32,
    pub short_retry_limit: u8,
    pub long_retry_limit: u8,
    pub aflags: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rate_policy {
    pub header: acx_header,
    pub rate_policy_idx: __le32,
    pub rate_policy: acx_rate_class,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_ac_cfg {
    pub header: acx_header,
    pub role_id: u8,
    pub ac: u8,
    pub aifsn: u8,
    pub cw_min: u8,
    pub cw_max: __le16,
    pub tx_op_limit: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_tid_config {
    pub header: acx_header,
    pub role_id: u8,
    pub queue_id: u8,
    pub channel_type: u8,
    pub tsid: u8,
    pub ps_scheme: u8,
    pub ack_policy: u8,
    pub padding: [u8; 2],
    pub apsd_conf: [__le32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_frag_threshold {
    pub header: acx_header,
    pub frag_threshold: __le16,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_tx_config_options {
    pub header: acx_header,
    pub /: *mut *mut __le16 tx_compl_timeout; / msec,
    pub /: *mut *mut __le16 tx_compl_threshold; / number of packets,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_config_memory {
    pub header: acx_header,
    pub rx_mem_block_num: u8,
    pub tx_min_mem_block_num: u8,
    pub num_stations: u8,
    pub num_ssid_profiles: u8,
    pub total_tx_descriptors: __le32,
    pub dyn_mem_enable: u8,
    pub tx_free_req: u8,
    pub rx_free_req: u8,
    pub tx_min: u8,
    pub fwlog_blocks: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_mem_map {
    pub header: acx_header,
    pub code_start: __le32,
    pub code_end: __le32,
    pub wep_defkey_start: __le32,
    pub wep_defkey_end: __le32,
    pub sta_table_start: __le32,
    pub sta_table_end: __le32,
    pub packet_template_start: __le32,
    pub packet_template_end: __le32,
// Address of the TX result interface (control block)
    pub tx_result: __le32,
    pub tx_result_queue_start: __le32,
    pub queue_memory_start: __le32,
    pub queue_memory_end: __le32,
    pub packet_memory_pool_start: __le32,
    pub packet_memory_pool_end: __le32,
    pub debug_buffer1_start: __le32,
    pub debug_buffer1_end: __le32,
    pub debug_buffer2_start: __le32,
    pub debug_buffer2_end: __le32,
// Number of blocks FW allocated for TX packets
    pub num_tx_mem_blocks: __le32,
// Number of blocks FW allocated for RX packets
    pub num_rx_mem_blocks: __le32,
// the following 4 fields are valid in SLAVE mode only
    pub tx_cbuf: *mut u8,
    pub rx_cbuf: *mut u8,
    pub rx_ctrl: __le32,
    pub tx_ctrl: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_rx_config_opt {
    pub header: acx_header,
    pub mblk_threshold: __le16,
    pub threshold: __le16,
    pub timeout: __le16,
    pub queue_type: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_bet_enable {
    pub header: acx_header,
    pub role_id: u8,
    pub enable: u8,
    pub max_consecutive: u8,
    pub padding: [u8; 1],
    pub __packed: },
pub const ACX_IPV4_VERSION: c_int = 4;
pub const ACX_IPV6_VERSION: c_int = 6;
pub const ACX_IPV4_ADDR_SIZE: c_int = 4;
// bitmap of enabled arp_filter features

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_arp_filter {
    pub header: acx_header,
    pub role_id: u8,
    pub /: *mut *mut u8 version; / ACX_IPV4_VERSION, ACX_IPV6_VERSION,
    pub /: *mut *mut u8 enable; / bitmap of enabled ARP filtering features,
    pub padding: [u8; 1],
    pub ARP: *mut *mut u8 address[16]; / The configured device IP address - all,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_pm_config {
    pub header: acx_header,
    pub host_clk_settling_time: __le32,
    pub host_fast_wakeup_support: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_keep_alive_mode {
    pub header: acx_header,
    pub role_id: u8,
    pub enabled: u8,
    pub padding: [u8; 2],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_keep_alive_config {
    pub header: acx_header,
    pub role_id: u8,
    pub index: u8,
    pub tpl_validation: u8,
    pub trigger: u8,
    pub period: __le32,
    pub __packed: },
// TODO: maybe this needs to be moved somewhere else?

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_rssi_snr_trigger {
    pub header: acx_header,
    pub role_id: u8,
    pub metric: u8,
    pub type: u8,
    pub dir: u8,
    pub threshold: __le16,
    pub /: *mut *mut __le16 pacing; / 0 - 60000 ms,
    pub hysteresis: u8,
    pub index: u8,
    pub enable: u8,
    pub padding: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_rssi_snr_avg_weights {
    pub header: acx_header,
    pub role_id: u8,
    pub padding: [u8; 3],
    pub rssi_beacon: u8,
    pub rssi_data: u8,
    pub snr_beacon: u8,
    pub snr_data: u8,
}

// special capability bit (not employed by the 802.11n spec)

//
// ACX_PEER_HT_CAP
// Configure HT capabilities - declare the capabilities of the peer
// we are connected to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_ht_capabilities {
    pub header: acx_header,
// bitmask of capability bits supported by the peer
    pub ht_capabilites: __le32,
// Indicates to which link these capabilities apply.
    pub hlid: u8,
//
// This the maximum A-MPDU length supported by the AP. The FW may not
// exceed this length when sending A-MPDUs
//
    pub ampdu_max_length: u8,
// This is the minimal spacing required when sending A-MPDUs to the AP
    pub ampdu_min_spacing: u8,
    pub padding: u8,
    pub __packed: },
//
// ACX_HT_BSS_OPERATION
// Configure HT capabilities - AP rules for behavior in the BSS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_ht_information {
    pub header: acx_header,
    pub role_id: u8,
// Values: 0 - RIFS not allowed, 1 - RIFS allowed
    pub rifs_mode: u8,
// Values: 0 - 3 like in spec
    pub ht_protection: u8,
// Values: 0 - GF protection not required, 1 - GF protection required
    pub gf_protection: u8,
// Values: 0 - TX Burst limit not required, 1 - TX Burst Limit required
    pub ht_tx_burst_limit: u8,
//
// Values: 0 - Dual CTS protection not required,
// 1 - Dual CTS Protection required
// Note: When this value is set to 1 FW will protect all TXOP with RTS
// frame and will not use CTS-to-self regardless of the value of the
// ACX_CTS_PROTECTION information element
//
    pub dual_cts_protection: u8,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_ba_initiator_policy {
    pub header: acx_header,
// Specifies role Id, Range 0-7, 0xFF means ANY role.
    pub role_id: u8,
//
// Per TID setting for allowing TX BA. Set a bit to 1 to allow
// TX BA sessions for the corresponding TID.
//
    pub tid_bitmap: u8,
// Windows size in number of packets
    pub win_size: u8,
    pub padding1: [u8; 1],
// As initiator inactivity timeout in time units(TU) of 1024us
    pub inactivity_timeout: u16,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_ba_receiver_setup {
    pub header: acx_header,
// Specifies link id, range 0-31
    pub hlid: u8,
    pub tid: u8,
    pub enable: u8,
// Windows size in number of packets
    pub win_size: u8,
// BA session starting sequence number.  RANGE 0-FFF
    pub ssn: u16,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_fw_tsf_information {
    pub header: acx_header,
    pub role_id: u8,
    pub padding1: [u8; 3],
    pub current_tsf_high: __le32,
    pub current_tsf_low: __le32,
    pub last_bttt_high: __le32,
    pub last_tbtt_low: __le32,
    pub last_dtim_count: u8,
    pub padding2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_ps_rx_streaming {
    pub header: acx_header,
    pub role_id: u8,
    pub tid: u8,
    pub enable: u8,
// interval between triggers (10-100 msec)
    pub period: u8,
// timeout before first trigger (0-200 msec)
    pub timeout: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_ap_max_tx_retry {
    pub header: acx_header,
    pub role_id: u8,
    pub padding_1: u8,
//
// the number of frames transmission failures before
// issuing the aging event.
//
    pub max_tx_retry: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_config_ps {
    pub header: acx_header,
    pub exit_retries: u8,
    pub enter_retries: u8,
    pub padding: [u8; 2],
    pub null_data_rate: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_inconnection_sta {
    pub header: acx_header,
    pub addr: [u8; ETH_ALEN],
    pub role_id: u8,
    pub padding: u8,
    pub __packed: },
//
// ACX_FM_COEX_CFG
// set the FM co-existence parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_fm_coex {
    pub header: acx_header,
// enable(1) / disable(0) the FM Coex feature
    pub enable: u8,
//
// Swallow period used in COEX PLL swallowing mechanism.
// 0xFF = use FW default
//
    pub swallow_period: u8,
//
// The N divider used in COEX PLL swallowing mechanism for Fref of
// 38.4/19.2 Mhz. 0xFF = use FW default
//
    pub n_divider_fref_set_1: u8,
//
// The N divider used in COEX PLL swallowing mechanism for Fref of
// 26/52 Mhz. 0xFF = use FW default
//
    pub n_divider_fref_set_2: u8,
//
// The M divider used in COEX PLL swallowing mechanism for Fref of
// 38.4/19.2 Mhz. 0xFFFF = use FW default
//
    pub m_divider_fref_set_1: __le16,
//
// The M divider used in COEX PLL swallowing mechanism for Fref of
// 26/52 Mhz. 0xFFFF = use FW default
//
    pub m_divider_fref_set_2: __le16,
//
// The time duration in uSec required for COEX PLL to stabilize.
// 0xFFFFFFFF = use FW default
//
    pub coex_pll_stabilization_time: __le32,
//
// The time duration in uSec required for LDO to stabilize.
// 0xFFFFFFFF = use FW default
//
    pub ldo_stabilization_time: __le16,
//
// The disturbed frequency band margin around the disturbed frequency
// center (single sided).
// For example, if 2 is configured, the following channels will be
// considered disturbed channel:
// 80 +- 0.1 MHz, 91 +- 0.1 MHz, 98 +- 0.1 MHz, 102 +- 0.1 MH
// 0xFF = use FW default
//
    pub fm_disturbed_band_margin: u8,
//
// The swallow clock difference of the swallowing mechanism.
// 0xFF = use FW default
//
    pub swallow_clk_diff: u8,
    pub __packed: },
pub const ACX_RATE_MGMT_ALL_PARAMS: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_set_rate_mgmt_params {
    pub header: acx_header,
    pub /: *mut *mut u8 index; / 0xff to configure all params,
    pub padding1: u8,
    pub rate_retry_score: __le16,
    pub per_add: __le16,
    pub per_th1: __le16,
    pub per_th2: __le16,
    pub max_per: __le16,
    pub inverse_curiosity_factor: u8,
    pub tx_fail_low_th: u8,
    pub tx_fail_high_th: u8,
    pub per_alpha_shift: u8,
    pub per_add_shift: u8,
    pub per_beta1_shift: u8,
    pub per_beta2_shift: u8,
    pub rate_check_up: u8,
    pub rate_check_down: u8,
    pub rate_retry_policy: [u8; ACX_RATE_MGMT_NUM_OF_RATES],
    pub padding2: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_config_hangover {
    pub header: acx_header,
    pub recover_time: __le32,
    pub hangover_period: u8,
    pub dynamic_mode: u8,
    pub early_termination_mode: u8,
    pub max_period: u8,
    pub min_period: u8,
    pub increase_delta: u8,
    pub decrease_delta: u8,
    pub quiet_time: u8,
    pub increase_time: u8,
    pub window_size: u8,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_default_rx_filter {
    pub header: acx_header,
    pub enable: u8,
// action of type FILTER_XXX
    pub default_action: u8,
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rx_filter_cfg {
    pub header: acx_header,
    pub enable: u8,
// 0 - WL1271_MAX_RX_FILTERS-1
    pub index: u8,
    pub action: u8,
    pub num_fields: u8,
    pub fields: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_roaming_stats {
    pub header: acx_header,
    pub role_id: u8,
    pub pad: [u8; 3],
    pub missed_beacons: u32,
    pub snr_data: u8,
    pub snr_bacon: u8,
    pub rssi_data: i8,
    pub rssi_beacon: i8,
    pub __packed: },
}

extern "C" {
    pub fn wl1271_acx_sleep_auth(wl: *mut wl1271, sleep_auth: u8) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_feature_cfg(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_rx_msdu_life_time(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_dco_itrim_params(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_sg_enable(wl: *mut wl1271, enable: bool) -> c_int;
}
extern "C" {
    pub fn wl12xx_acx_sg_cfg(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_cca_threshold(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_bcn_dtim_options(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_aid(wl: *mut wl1271, wlvif: *mut wl12xx_vif, aid: u16) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_event_mbox_mask(wl: *mut wl1271, event_mask: u32) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_statistics(wl: *mut wl1271, stats: *mut c_void) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_sta_rate_policies(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_frag_threshold(wl: *mut wl1271, frag_threshold: u32) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_tx_config_options(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl12xx_acx_mem_cfg(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_init_mem_config(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_init_rx_interrupt(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_smart_reflex(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_pm_config(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_ap_max_tx_retry(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl12xx_acx_config_ps(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_acx_fm_coex(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl12xx_acx_set_rate_mgmt_params(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl12xx_acx_config_hangover(wl: *mut wl1271) -> c_int;
}
