//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/acx.h
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
// This file is part of wl1251
//
// Copyright (c) 1998-2007 Texas Instruments Incorporated
// Copyright (C) 2008 Nokia Corporation
//

// Target's information element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_header {
    pub cmd: wl1251_cmd_header,
// acx (or information element) header
    pub id: u16,
// payload length (not including headers
    pub len: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_error_counter {
    pub header: acx_header,
// The number of PLCP errors since the last time this
// information element was interrogated. This field is
// automatically cleared when it is interrogated.
    pub PLCP_error: u32,
// The number of FCS errors since the last time this
// information element was interrogated. This field is
// automatically cleared when it is interrogated.
    pub FCS_error: u32,
// The number of MPDUs without PLCP header errors received
// since the last time this information element was interrogated.
// This field is automatically cleared when it is interrogated.
    pub valid_frame: u32,
// the number of missed sequence numbers in the squentially
// values of frames seq numbers
    pub seq_num_miss: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_revision {
    pub header: acx_header,
//
// The WiLink firmware version, an ASCII string x.x.x.x,
// that uniquely identifies the current firmware.
// The left most digit is incremented each time a
// significant change is made to the firmware, such as
// code redesign or new platform support.
// The second digit is incremented when major enhancements
// are added or major fixes are made.
// The third digit is incremented for each GA release.
// The fourth digit is incremented for each build.
// The first two digits identify a firmware release version,
// in other words, a unique set of features.
// The first three digits identify a GA release.
//
    pub fw_version: [c_char; 20],
//
// This 4 byte field specifies the WiLink hardware version.
// bits 0  - 15: Reserved.
// bits 16 - 23: Version ID - The WiLink version ID
// (1 = first spin, 2 = second spin, and so on).
// bits 24 - 31: Chip ID - The WiLink chip ID.
//
    pub hw_version: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_psm_mode {
// Active mode
    WL1251_PSM_CAM = 0,

// Power save mode
    WL1251_PSM_PS = 1,

// Extreme low power
    WL1251_PSM_ELP = 2,
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
pub const DEFAULT_NUM_STATIONS: c_int = 1;

pub const DEFAULT_RXQ_TYPE: c_uint = 0x07    /* All frames, Data/Ctrl/Mgmt */;
pub const TRACE_BUFFER_MAX_SIZE: c_int = 256;
pub const DP_RX_PACKET_RING_CHUNK_SIZE: c_int = 1600;
pub const DP_TX_PACKET_RING_CHUNK_SIZE: c_int = 1600;
pub const DP_RX_PACKET_RING_CHUNK_NUM: c_int = 2;
pub const DP_TX_PACKET_RING_CHUNK_NUM: c_int = 2;
pub const DP_TX_COMPLETE_TIME_OUT: c_int = 20;
pub const FW_TX_CMPLT_BLOCK_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_data_path_params {
    pub header: acx_header,
    pub rx_packet_ring_chunk_size: u16,
    pub tx_packet_ring_chunk_size: u16,
    pub rx_packet_ring_chunk_num: u8,
    pub tx_packet_ring_chunk_num: u8,
//
// Maximum number of packets that can be gathered
// in the TX complete ring before an interrupt
// is generated.
//
    pub tx_complete_threshold: u8,
// Number of pending TX complete entries in cyclic ring.
    pub tx_complete_ring_depth: u8,
//
// Max num microseconds since a packet enters the TX
// complete ring until an interrupt is generated.
//
    pub tx_complete_timeout: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_data_path_params_resp {
    pub header: acx_header,
    pub rx_packet_ring_chunk_size: u16,
    pub tx_packet_ring_chunk_size: u16,
    pub rx_packet_ring_chunk_num: u8,
    pub tx_packet_ring_chunk_num: u8,
    pub pad: [u8; 2],
    pub rx_packet_ring_addr: u32,
    pub tx_packet_ring_addr: u32,
    pub rx_control_addr: u32,
    pub tx_control_addr: u32,
    pub tx_complete_addr: u32,
    pub __packed: },
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
    pub lifetime: u32,
    pub __packed: },
//
// RX Config Options Table
// Bit		Definition
// ===		==========
// 31:14		Reserved
// 13		Copy RX Status - when set, write three receive status words
// to top of rx'd MPDUs.
// When cleared, do not write three status words (added rev 1.5)
// 12		Reserved
// 11		RX Complete upon FCS error - when set, give rx complete
// interrupt for FCS errors, after the rx filtering, e.g. unicast
// frames not to us with FCS error will not generate an interrupt.
// 10		SSID Filter Enable - When set, the WiLink discards all beacon,
// probe request, and probe response frames with an SSID that does
// not match the SSID specified by the host in the START/JOIN
// command.
// When clear, the WiLink receives frames with any SSID.
// 9		Broadcast Filter Enable - When set, the WiLink discards all
// broadcast frames. When clear, the WiLink receives all received
// broadcast frames.
// 8:6		Reserved
// 5		BSSID Filter Enable - When set, the WiLink discards any frames
// with a BSSID that does not match the BSSID specified by the
// host.
// When clear, the WiLink receives frames from any BSSID.
// 4		MAC Addr Filter - When set, the WiLink discards any frames
// with a destination address that does not match the MAC address
// of the adaptor.
// When clear, the WiLink receives frames destined to any MAC
// address.
// 3		Promiscuous - When set, the WiLink receives all valid frames
// (i.e., all frames that pass the FCS check).
// When clear, only frames that pass the other filters specified
// are received.
// 2		FCS - When set, the WiLink includes the FCS with the received
// frame.
// When cleared, the FCS is discarded.
// 1		PLCP header - When set, write all data from baseband to frame
// buffer including PHY header.
// 0		Reserved - Always equal to 0.
//
// RX Filter Options Table
// Bit		Definition
// ===		==========
// 31:12		Reserved - Always equal to 0.
// 11		Association - When set, the WiLink receives all association
// related frames (association request/response, reassociation
// request/response, and disassociation). When clear, these frames
// are discarded.
// 10		Auth/De auth - When set, the WiLink receives all authentication
// and de-authentication frames. When clear, these frames are
// discarded.
// 9		Beacon - When set, the WiLink receives all beacon frames.
// When clear, these frames are discarded.
// 8		Contention Free - When set, the WiLink receives all contention
// free frames.
// When clear, these frames are discarded.
// 7		Control - When set, the WiLink receives all control frames.
// When clear, these frames are discarded.
// 6		Data - When set, the WiLink receives all data frames.
// When clear, these frames are discarded.
// 5		FCS Error - When set, the WiLink receives frames that have FCS
// errors.
// When clear, these frames are discarded.
// 4		Management - When set, the WiLink receives all management
// frames.
// When clear, these frames are discarded.
// 3		Probe Request - When set, the WiLink receives all probe request
// frames.
// When clear, these frames are discarded.
// 2		Probe Response - When set, the WiLink receives all probe
// response frames.
// When clear, these frames are discarded.
// 1		RTS/CTS/ACK - When set, the WiLink receives all RTS, CTS and ACK
// frames.
// When clear, these frames are discarded.
// 0		Rsvd Type/Sub Type - When set, the WiLink receives all frames
// that have reserved frame types and sub types as defined by the
// 802.11 specification.
// When clear, these frames are discarded.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rx_config {
    pub header: acx_header,
    pub config_options: u32,
    pub filter_options: u32,
    pub __packed: },
}

pub const MAX_NUM_OF_802_1d_TAGS: c_int = 8;
pub const AC_PARAMS_MAX_TSID: c_int = 15;
pub const MAX_APSD_CONF: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_tx_queue_qos_config {
    pub header: acx_header,
    pub qid: u8,
    pub pad: [u8; 3],
// Max number of blocks allowd in the queue
    pub high_threshold: u16,
// Lowest memory blocks guaranteed for this queue
    pub low_threshold: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_packet_detection {
    pub header: acx_header,
    pub threshold: u32,
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
    pub /: *mut *mut u8 wone_index; / Reserved,
    pub slot_time: u8,
    pub reserved: [u8; 6],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_dot11_grp_addr_tbl {
    pub header: acx_header,
    pub enabled: u8,
    pub num_groups: u8,
    pub pad: [u8; 2],
    pub mac_table: [u8; ACX_MC_ADDRESS_GROUP_MAX_LEN],
    pub __packed: },
pub const RX_TIMEOUT_PS_POLL_MIN: c_int = 0;

pub const RX_TIMEOUT_UPSD_MIN: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rx_timeout {
    pub header: acx_header,
//
// The longest time the STA will wait to receive
// traffic from the AP after a PS-poll has been
// transmitted.
//
    pub ps_poll_timeout: u16,
//
// The longest time the STA will wait to receive
// traffic from the AP after a frame has been sent
// from an UPSD enabled queue.
//
    pub upsd_timeout: u16,
    pub __packed: },
pub const RTS_THRESHOLD_MIN: c_int = 0;
pub const RTS_THRESHOLD_MAX: c_int = 4096;
pub const RTS_THRESHOLD_DEF: c_int = 2347;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rts_threshold {
    pub header: acx_header,
    pub threshold: u16,
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_acx_low_rssi_type {
//
// The event is a "Level" indication which keeps triggering
// as long as the average RSSI is below the threshold.
//
    WL1251_ACX_LOW_RSSI_TYPE_LEVEL = 0,

//
// The event is an "Edge" indication which triggers
// only when the RSSI threshold is crossed from above.
//
    WL1251_ACX_LOW_RSSI_TYPE_EDGE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_low_rssi {
    pub header: acx_header,
//
// The threshold (in dBm) below (or above after low rssi
// indication) which the firmware generates an interrupt to the
// host. This parameter is signed.
//
    pub threshold: i8,
//
// The weight of the current RSSI sample, before adding the new
// sample, that is used to calculate the average RSSI.
//
    pub weight: u8,
//
// The number of Beacons/Probe response frames that will be
// received before issuing the Low or Regained RSSI event.
//
    pub depth: u8,
//
// Configures how the Low RSSI Event is triggered. Refer to
// enum wl1251_acx_low_rssi_type for more.
//
    pub type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_beacon_filter_option {
    pub header: acx_header,
    pub enable: u8,
//
// The number of beacons without the unicast TIM
// bit set that the firmware buffers before
// signaling the host about ready frames.
// When set to 0 and the filter is enabled, beacons
// without the unicast TIM bit set are dropped.
//
    pub max_num_beacons: u8,
    pub pad: [u8; 2],
    pub __packed: },
//
// ACXBeaconFilterEntry (not 221)
// Byte Offset     Size (Bytes)    Definition
// ===========     ============    ==========
// 0				1               IE identifier
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
    pub num_ie: u8,
    pub pad: [u8; 3],
    pub table: [u8; BEACON_FILTER_TABLE_MAX_SIZE],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_conn_monit_params {
    pub header: acx_header,
    pub /: *mut *mut u32 synch_fail_thold; / number of beacons missed,
    pub /: *mut *mut u32 bss_lose_timeout; / number of TU's from synch fail,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_bt_wlan_coex {
    pub header: acx_header,
//
// 0 -> PTA enabled
// 1 -> PTA disabled
// 2 -> sense no active mode, i.e.
// an interrupt is sent upon
// BT activity.
// 3 -> PTA is switched on in response
// to the interrupt sending.
//
    pub enable: u8,
    pub pad: [u8; 3],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_bt_wlan_coex_param {
    pub header: acx_header,
//
// The minimum rate of a received WLAN packet in the STA,
// during protective mode, of which a new BT-HP request
// during this Rx will always be respected and gain the antenna.
//
    pub min_rate: u32,
// Max time the BT HP will be respected.
    pub bt_hp_max_time: u16,
// Max time the WLAN HP will be respected.
    pub wlan_hp_max_time: u16,
//
// The time between the last BT activity
// and the moment when the sense mode returns
// to SENSE_INACTIVE.
//
    pub sense_disable_timer: u16,
// Time before the next BT HP instance
    pub rx_time_bt_hp: u16,
    pub tx_time_bt_hp: u16,
// range: 10-20000    default: 1500
    pub rx_time_bt_hp_fast: u16,
    pub tx_time_bt_hp_fast: u16,
// range: 2000-65535  default: 8700
    pub wlan_cycle_fast: u16,
// range: 0 - 15000 (Msec) default: 1000
    pub bt_anti_starvation_period: u16,
// range 400-10000(Usec) default: 3000
    pub next_bt_lp_packet: u16,
// Deafult: worst case for BT DH5 traffic
    pub wake_up_beacon: u16,
// range: 0-50000(Usec) default: 1050
    pub hp_dm_max_guard_time: u16,
//
// This is to prevent both BT & WLAN antenna
// starvation.
// Range: 100-50000(Usec) default:2550
//
    pub next_wlan_packet: u16,
// 0 -> shared antenna
    pub antenna_type: u8,
//
// 0 -> TI legacy
// 1 -> Palau
//
    pub signal_type: u8,
//
// BT AFH status
// 0 -> no AFH
// 1 -> from dedicated GPIO
// 2 -> AFH on (from host)
//
    pub afh_leverage_on: u8,
//
// The number of cycles during which no
// TX will be sent after 1 cycle of RX
// transaction in protective mode
//
    pub quiet_cycle_num: u8,
//
// The maximum number of CTSs that will
// be sent for receiving RX packet in
// protective mode
//
    pub max_cts: u8,
//
// The number of WLAN packets
// transferred in common mode before
// switching to BT.
//
    pub wlan_packets_num: u8,
//
// The number of BT packets
// transferred in common mode before
// switching to WLAN.
//
    pub bt_packets_num: u8,
// range: 1-255  default: 5
    pub missed_rx_avalanche: u8,
// range: 0-1    default: 1
    pub wlan_elp_hp: u8,
// range: 0 - 15  default: 4
    pub bt_anti_starvation_cycles: u8,
    pub ack_mode_dual_ant: u8,
//
// Allow PA_SD assertion/de-assertion
// during enabled BT activity.
//
    pub pa_sd_enable: u8,
//
// Enable/Disable PTA in auto mode:
// Support Both Active & P.S modes
//
    pub pta_auto_mode_enable: u8,
// range: 0 - 20  default: 1
    pub bt_hp_respected_num: u8,
    pub __packed: },
pub const CCA_THRSH_ENABLE_ENERGY_D: c_uint = 0x140A;
pub const CCA_THRSH_DISABLE_ENERGY_D: c_uint = 0xFFEF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_energy_detection {
    pub header: acx_header,
// The RX Clear Channel Assessment threshold in the PHY
    pub rx_cca_threshold: u16,
    pub tx_energy_detection: u8,
    pub pad: u8,
    pub __packed: },
pub const BCN_RX_TIMEOUT_DEF_VALUE: c_int = 10000;
pub const BROADCAST_RX_TIMEOUT_DEF_VALUE: c_int = 20000;
pub const RX_BROADCAST_IN_PS_DEF_VALUE: c_int = 1;
pub const CONSECUTIVE_PS_POLL_FAILURE_DEF: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_beacon_broadcast {
    pub header: acx_header,
    pub beacon_rx_timeout: u16,
    pub broadcast_timeout: u16,
// Enables receiving of broadcast packets in PS mode
    pub rx_broadcast_in_ps: u8,
// Consecutive PS Poll failures before updating the host
    pub ps_poll_threshold: u8,
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_event_mask {
    pub header: acx_header,
    pub event_mask: u32,
    pub /: *mut *mut u32 high_event_mask; / Unused,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_fw_gen_frame_rates {
    pub header: acx_header,
    pub /: *mut *mut *mut u8 tx_ctrl_frame_rate; / RATE_,
    pub /: *mut *mut *mut *mut u8 tx_ctrl_frame_mod; / CCK_ or PBCC_,
    pub tx_mgt_frame_rate: u8,
    pub tx_mgt_frame_mod: u8,
    pub __packed: },
// STA MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_dot11_station_id {
    pub header: acx_header,
    pub mac: [u8; ETH_ALEN],
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_feature_config {
    pub header: acx_header,
    pub options: u32,
    pub data_flow_options: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_current_tx_power {
    pub header: acx_header,
    pub current_tx_power: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_dot11_default_key {
    pub header: acx_header,
    pub id: u8,
    pub pad: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_tsf_info {
    pub header: acx_header,
    pub current_tsf_msb: u32,
    pub current_tsf_lsb: u32,
    pub last_TBTT_msb: u32,
    pub last_TBTT_lsb: u32,
    pub last_dtim_count: u8,
    pub pad: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acx_wake_up_event {
    WAKE_UP_EVENT_BEACON_BITMAP	= 0x01, /* Wake on every Beacon*/
    WAKE_UP_EVENT_DTIM_BITMAP	= 0x02,	/* Wake on every DTIM*/
    WAKE_UP_EVENT_N_DTIM_BITMAP	= 0x04, /* Wake on every Nth DTIM */
    WAKE_UP_EVENT_N_BEACONS_BITMAP	= 0x08, /* Wake on every Nth Beacon */
    WAKE_UP_EVENT_BITS_MASK		= 0x0F
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_wake_up_condition {
    pub header: acx_header,
    pub /: *mut *mut u8 wake_up_event; / Only one bit can be set,
    pub listen_interval: u8,
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_aid {
    pub header: acx_header,
//
// To be set when associated with an AP.
//
    pub aid: u16,
    pub pad: [u8; 2],
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
    pub preamble: u8,
    pub padding: [u8; 3],
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
    pub ctsprotect: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_tx_statistics {
    pub internal_desc_overflow: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rx_statistics {
    pub out_of_mem: u32,
    pub hdr_overflow: u32,
    pub hw_stuck: u32,
    pub dropped: u32,
    pub fcs_err: u32,
    pub xfr_hint_trig: u32,
    pub path_reset: u32,
    pub reset_counter: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_dma_statistics {
    pub rx_requested: u32,
    pub rx_errors: u32,
    pub tx_requested: u32,
    pub tx_errors: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_isr_statistics {
// host command complete
    pub cmd_cmplt: u32,
// fiqisr()
    pub fiqs: u32,
// (INT_STS_ND & INT_TRIG_RX_HEADER)
    pub rx_headers: u32,
// (INT_STS_ND & INT_TRIG_RX_CMPLT)
    pub rx_completes: u32,
// (INT_STS_ND & INT_TRIG_NO_RX_BUF)
    pub rx_mem_overflow: u32,
// (INT_STS_ND & INT_TRIG_S_RX_RDY)
    pub rx_rdys: u32,
// irqisr()
    pub irqs: u32,
// (INT_STS_ND & INT_TRIG_TX_PROC)
    pub tx_procs: u32,
// (INT_STS_ND & INT_TRIG_DECRYPT_DONE)
    pub decrypt_done: u32,
// (INT_STS_ND & INT_TRIG_DMA0)
    pub dma0_done: u32,
// (INT_STS_ND & INT_TRIG_DMA1)
    pub dma1_done: u32,
// (INT_STS_ND & INT_TRIG_TX_EXC_CMPLT)
    pub tx_exch_complete: u32,
// (INT_STS_ND & INT_TRIG_COMMAND)
    pub commands: u32,
// (INT_STS_ND & INT_TRIG_RX_PROC)
    pub rx_procs: u32,
// (INT_STS_ND & INT_TRIG_PM_802)
    pub hw_pm_mode_changes: u32,
// (INT_STS_ND & INT_TRIG_ACKNOWLEDGE)
    pub host_acknowledges: u32,
// (INT_STS_ND & INT_TRIG_PM_PCI)
    pub pci_pm: u32,
// (INT_STS_ND & INT_TRIG_ACM_WAKEUP)
    pub wakeups: u32,
// (INT_STS_ND & INT_TRIG_LOW_RSSI)
    pub low_rssi: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_wep_statistics {
// WEP address keys configured
    pub addr_key_count: u32,
// default keys configured
    pub default_key_count: u32,
    pub reserved: u32,
// number of times that WEP key not found on lookup
    pub key_not_found: u32,
// number of times that WEP key decryption failed
    pub decrypt_fail: u32,
// WEP packets decrypted
    pub packets: u32,
// WEP decrypt interrupts
    pub interrupt: u32,
    pub __packed: },
pub const ACX_MISSED_BEACONS_SPREAD: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_pwr_statistics {
// the amount of enters into power save mode (both PD & ELP)
    pub ps_enter: u32,
// the amount of enters into ELP mode
    pub elp_enter: u32,
// the amount of missing beacon interrupts to the host
    pub missing_bcns: u32,
// the amount of wake on host-access times
    pub wake_on_host: u32,
// the amount of wake on timer-expire
    pub wake_on_timer_exp: u32,
// the number of packets that were transmitted with PS bit set
    pub tx_with_ps: u32,
// the number of packets that were transmitted with PS bit clear
    pub tx_without_ps: u32,
// the number of received beacons
    pub rcvd_beacons: u32,
// the number of entering into PowerOn (power save off)
    pub power_save_off: u32,
// the number of entries into power save mode
    pub enable_ps: u16,
//
// the number of exits from power save, not including failed PS
// transitions
//
    pub disable_ps: u16,
//
// the number of times the TSF counter was adjusted because
// of drift
//
    pub fix_tsf_ps: u32,
// Gives statistics about the spread continuous missed beacons.
// The 16 LSB are dedicated for the PS mode.
// The 16 MSB are dedicated for the PS mode.
// cont_miss_bcns_spread[0] - single missed beacon.
// cont_miss_bcns_spread[1] - two continuous missed beacons.
// cont_miss_bcns_spread[2] - three continuous missed beacons.
// ...
// cont_miss_bcns_spread[9] - ten and more continuous missed beacons.
//
    pub cont_miss_bcns_spread: [u32; ACX_MISSED_BEACONS_SPREAD],
// the number of beacons in awake mode
    pub rcvd_awake_beacons: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_mic_statistics {
    pub rx_pkts: u32,
    pub calc_failure: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_aes_statistics {
    pub encrypt_fail: u32,
    pub decrypt_fail: u32,
    pub encrypt_packets: u32,
    pub decrypt_packets: u32,
    pub encrypt_interrupt: u32,
    pub decrypt_interrupt: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_event_statistics {
    pub heart_beat: u32,
    pub calibration: u32,
    pub rx_mismatch: u32,
    pub rx_mem_empty: u32,
    pub rx_pool: u32,
    pub oom_late: u32,
    pub phy_transmit_error: u32,
    pub tx_stuck: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_ps_statistics {
    pub pspoll_timeouts: u32,
    pub upsd_timeouts: u32,
    pub upsd_max_sptime: u32,
    pub upsd_max_apturn: u32,
    pub pspoll_max_apturn: u32,
    pub pspoll_utilization: u32,
    pub upsd_utilization: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rxpipe_statistics {
    pub rx_prep_beacon_drop: u32,
    pub descr_host_int_trig_rx_data: u32,
    pub beacon_buffer_thres_host_int_trig_rx_data: u32,
    pub missed_beacon_host_int_trig_rx_data: u32,
    pub tx_xfr_host_int_trig_rx_data: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_statistics {
    pub header: acx_header,
    pub tx: acx_tx_statistics,
    pub rx: acx_rx_statistics,
    pub dma: acx_dma_statistics,
    pub isr: acx_isr_statistics,
    pub wep: acx_wep_statistics,
    pub pwr: acx_pwr_statistics,
    pub aes: acx_aes_statistics,
    pub mic: acx_mic_statistics,
    pub event: acx_event_statistics,
    pub ps: acx_ps_statistics,
    pub rxpipe: acx_rxpipe_statistics,
    pub __packed: },
pub const ACX_MAX_RATE_CLASSES: c_int = 8;
pub const ACX_RATE_MASK_UNSPECIFIED: c_int = 0;
pub const ACX_RATE_RETRY_LIMIT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rate_class {
    pub enabled_rates: u32,
    pub short_retry_limit: u8,
    pub long_retry_limit: u8,
    pub aflags: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_rate_policy {
    pub header: acx_header,
    pub rate_class_cnt: u32,
    pub rate_class: [acx_rate_class; ACX_MAX_RATE_CLASSES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_memory {
    pub /: *mut *mut __le16 num_stations; / number of STAs to be supported.,
    pub reserved_1: u16,
//
// Nmber of memory buffers for the RX mem pool.
// The actual number may be less if there are
// not enough blocks left for the minimum num
// of TX ones.
//
    pub rx_mem_block_num: u8,
    pub reserved_2: u8,
    pub /: *mut *mut u8 num_tx_queues; / From 1 to 16,
    pub /: *mut *mut *mut u8 host_if_options; / HOST_IF,
    pub tx_min_mem_block_num: u8,
    pub num_ssid_profiles: u8,
    pub debug_buffer_size: __le16,
    pub __packed: },
pub const ACX_RX_DESC_MIN: c_int = 1;
pub const ACX_RX_DESC_MAX: c_int = 127;
pub const ACX_RX_DESC_DEF: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_rx_queue_config {
    pub num_descs: u8,
    pub pad: u8,
    pub type: u8,
    pub priority: u8,
    pub dma_address: __le32,
    pub __packed: },
pub const ACX_TX_DESC_MIN: c_int = 1;
pub const ACX_TX_DESC_MAX: c_int = 127;
pub const ACX_TX_DESC_DEF: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_tx_queue_config {
    pub num_descs: u8,
    pub pad: [u8; 2],
    pub attributes: u8,
    pub __packed: },
pub const MAX_TX_QUEUE_CONFIGS: c_int = 5;
pub const MAX_TX_QUEUES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_config_memory {
    pub header: acx_header,
    pub mem_config: wl1251_acx_memory,
    pub rx_queue_config: wl1251_acx_rx_queue_config,
    pub tx_queue_config: [wl1251_acx_tx_queue_config; MAX_TX_QUEUE_CONFIGS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_mem_map {
    pub header: acx_header,
    pub code_start: *mut c_void,
    pub code_end: *mut c_void,
    pub wep_defkey_start: *mut c_void,
    pub wep_defkey_end: *mut c_void,
    pub sta_table_start: *mut c_void,
    pub sta_table_end: *mut c_void,
    pub packet_template_start: *mut c_void,
    pub packet_template_end: *mut c_void,
    pub queue_memory_start: *mut c_void,
    pub queue_memory_end: *mut c_void,
    pub packet_memory_pool_start: *mut c_void,
    pub packet_memory_pool_end: *mut c_void,
    pub debug_buffer1_start: *mut c_void,
    pub debug_buffer1_end: *mut c_void,
    pub debug_buffer2_start: *mut c_void,
    pub debug_buffer2_end: *mut c_void,
// Number of blocks FW allocated for TX packets
    pub num_tx_mem_blocks: u32,
// Number of blocks FW allocated for RX packets
    pub num_rx_mem_blocks: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_wr_tbtt_and_dtim {
    pub header: acx_header,
// Time in TUs between two consecutive beacons
    pub tbtt: u16,
//
// DTIM period
// For BSS: Number of TBTTs in a DTIM period (range: 1-10)
// For IBSS: value shall be set to 1
//
    pub dtim: u8,
    pub padding: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_acx_bet_mode {
    WL1251_ACX_BET_DISABLE = 0,
    WL1251_ACX_BET_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_bet_enable {
    pub header: acx_header,
//
// Specifies if beacon early termination procedure is enabled or
// disabled, see enum wl1251_acx_bet_mode.
//
    pub enable: u8,
//
// Specifies the maximum number of consecutive beacons that may be
// early terminated. After this number is reached at least one full
// beacon must be correctly received in FW before beacon ET
// resumes. Range 0 - 255.
//
    pub max_consecutive: u8,
    pub padding: [u8; 2],
    pub __packed: },
pub const ACX_IPV4_VERSION: c_int = 4;
pub const ACX_IPV6_VERSION: c_int = 6;
pub const ACX_IPV4_ADDR_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_arp_filter {
    pub header: acx_header,
    pub IPv6.*/: *mut *mut u8 version; / The IP version: 4 - IPv4, 6 -,
    pub /: *mut *mut u8 enable; / 1 - ARP filtering is enabled, 0 - disabled,
    pub padding: [u8; 2],
    pub packets.: *mut *mut u8 address[16]; / The IP address used to filter ARP,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_ac_cfg {
    pub header: acx_header,
//
// Access Category - The TX queue's access category
// (refer to AccessCategory_enum)
//
    pub ac: u8,
//
// The contention window minimum size (in slots) for
// the access class.
//
    pub cw_min: u8,
//
// The contention window maximum size (in slots) for
// the access class.
//
    pub cw_max: u16,
// The AIF value (in slots) for the access class.
    pub aifsn: u8,
    pub reserved: u8,
// The TX Op Limit (in microseconds) for the access class.
    pub txop_limit: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_acx_channel_type {
    CHANNEL_TYPE_DCF	= 0,
    CHANNEL_TYPE_EDCF	= 1,
    CHANNEL_TYPE_HCCA	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_acx_ps_scheme {
// regular ps: simple sending of packets
    WL1251_ACX_PS_SCHEME_LEGACY	= 0,

// sending a packet triggers a unscheduled apsd downstream
    WL1251_ACX_PS_SCHEME_UPSD_TRIGGER	= 1,

// a pspoll packet will be sent before every data packet
    WL1251_ACX_PS_SCHEME_LEGACY_PSPOLL	= 2,

// scheduled apsd mode
    WL1251_ACX_PS_SCHEME_SAPSD		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_acx_ack_policy {
    WL1251_ACX_ACK_POLICY_LEGACY	= 0,
    WL1251_ACX_ACK_POLICY_NO_ACK	= 1,
    WL1251_ACX_ACK_POLICY_BLOCK	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_acx_tid_cfg {
    pub header: acx_header,
// tx queue id number (0-7)
    pub queue: u8,
// channel access type for the queue, enum wl1251_acx_channel_type
    pub type: u8,
// EDCA: ac index (0-3), HCCA: traffic stream id (8-15)
    pub tsid: u8,
// ps scheme of the specified queue, enum wl1251_acx_ps_scheme
    pub ps_scheme: u8,
// the tx queue ack policy, enum wl1251_acx_ack_policy
    pub ack_policy: u8,
    pub padding: [u8; 3],
// not supported
    pub apsdconf: [u32; 2],
    pub __packed: },
//
// RX packet is ready in Xfer buffer #0

// TX result(s) are in the TX complete buffer

// OBSOLETE

// RX packet is ready in Xfer buffer #1

// Event was entered to Event MBOX #A

// Event was entered to Event MBOX #B

// OBSOLETE

// Trace message on MBOX #A

// Trace message on MBOX #B

// Command processing completion

// Init sequence is done

pub const WL1251_ACX_INTR_ALL: c_uint = 0xFFFFFFFF;

}

extern "C" {
    pub fn wl1251_acx_station_id(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_default_key(wl: *mut wl1251, key_id: u8) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_sleep_auth(wl: *mut wl1251, sleep_auth: u8) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_fw_version(wl: *mut wl1251, buf: *mut c_char, len: usize) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_tx_power(wl: *mut wl1251, power: c_int) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_feature_cfg(wl: *mut wl1251, data_flow_options: u32) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_rx_msdu_life_time(wl: *mut wl1251, life_time: u32) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_rx_config(wl: *mut wl1251, config: u32, filter: u32) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_pd_threshold(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_slot(wl: *mut wl1251, slot_time: acx_slot_type) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_service_period_timeout(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_rts_threshold(wl: *mut wl1251, rts_threshold: u16) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_beacon_filter_opt(wl: *mut wl1251, enable_filter: bool) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_beacon_filter_table(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_conn_monit_params(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_sg_enable(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_sg_cfg(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_cca_threshold(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_bcn_dtim_options(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_aid(wl: *mut wl1251, aid: u16) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_event_mbox_mask(wl: *mut wl1251, event_mask: u32) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_set_preamble(wl: *mut wl1251, preamble: acx_preamble_type) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_statistics(wl: *mut wl1251, stats: *mut acx_statistics) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_tsf_info(wl: *mut wl1251, mactime: *mut u64) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_mem_cfg(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_wr_tbtt_and_dtim(wl: *mut wl1251, tbtt: u16, dtim: u8) -> c_int;
}
extern "C" {
    pub fn wl1251_acx_arp_ip_filter(wl: *mut wl1251, enable: bool, address: __be32) -> c_int;
}
