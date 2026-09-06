//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/mac.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2012-2014, 2018-2022, 2024-2025 Intel Corporation
// Copyright (C) 2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_mac_h__
//
// The first MAC indices (starting from 0) are available to the driver,
// AUX indices follows - 1 for non-CDB, 2 for CDB.
//
pub const MAC_INDEX_AUX: c_int = 4;
pub const MAC_INDEX_MIN_DRIVER: c_int = 0;

pub const IWL_STATION_COUNT_MAX: c_int = 16;
pub const IWL_INVALID_STA: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ac {
    AC_BK,
    AC_BE,
    AC_VI,
    AC_VO,
    AC_NUM,
}

//
// enum iwl_mac_protection_flags - MAC context flags
// @MAC_PROT_FLG_TGG_PROTECT: 11g protection when transmitting OFDM frames,
// this will require CCK RTS/CTS2self.
// RTS/CTS will protect full burst time.
// @MAC_PROT_FLG_HT_PROT: enable HT protection
// @MAC_PROT_FLG_FAT_PROT: protect 40 MHz transmissions
// @MAC_PROT_FLG_SELF_CTS_EN: allow CTS2self
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mac_protection_flags {
    MAC_PROT_FLG_TGG_PROTECT	= BIT(3),
    MAC_PROT_FLG_HT_PROT		= BIT(23),
    MAC_PROT_FLG_FAT_PROT		= BIT(24),
    MAC_PROT_FLG_SELF_CTS_EN	= BIT(30),
}

//
// enum iwl_mac_types - Supported MAC types
// @FW_MAC_TYPE_FIRST: lowest supported MAC type
// @FW_MAC_TYPE_AUX: Auxiliary MAC (internal)
// @FW_MAC_TYPE_LISTENER: monitor MAC type (?)
// @FW_MAC_TYPE_PIBSS: Pseudo-IBSS
// @FW_MAC_TYPE_IBSS: IBSS
// @FW_MAC_TYPE_BSS_STA: BSS (managed) station
// @FW_MAC_TYPE_P2P_DEVICE: P2P Device
// @FW_MAC_TYPE_P2P_STA: P2P client
// @FW_MAC_TYPE_GO: P2P GO
// @FW_MAC_TYPE_NAN: NAN (since version 4)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mac_types {
    FW_MAC_TYPE_FIRST = 1,
    FW_MAC_TYPE_AUX = FW_MAC_TYPE_FIRST,
    FW_MAC_TYPE_LISTENER,
    FW_MAC_TYPE_PIBSS,
    FW_MAC_TYPE_IBSS,
    FW_MAC_TYPE_BSS_STA,
    FW_MAC_TYPE_P2P_DEVICE,
    FW_MAC_TYPE_P2P_STA,
    FW_MAC_TYPE_GO,
    FW_MAC_TYPE_NAN,
}

//
// enum iwl_tsf_id - TSF hw timer ID
// @TSF_ID_A: use TSF A
// @TSF_ID_B: use TSF B
// @TSF_ID_C: use TSF C
// @TSF_ID_D: use TSF D
// @NUM_TSF_IDS: number of TSF timers available
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tsf_id {
    TSF_ID_A = 0,
    TSF_ID_B = 1,
    TSF_ID_C = 2,
    TSF_ID_D = 3,
    NUM_TSF_IDS = 4,
}

//
// struct iwl_mac_data_ap - configuration data for AP MAC context
// @beacon_time: beacon transmit time in system time
// @beacon_tsf: beacon transmit time in TSF
// @bi: beacon interval in TU
// @reserved1: reserved
// @dtim_interval: dtim transmit time in TU
// @reserved2: reserved
// @mcast_qid: queue ID for multicast traffic.
// NOTE: obsolete from VER2 and on
// @beacon_template: beacon template ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_data_ap {
    pub beacon_time: __le32,
    pub beacon_tsf: __le64,
    pub bi: __le32,
    pub reserved1: __le32,
    pub dtim_interval: __le32,
    pub reserved2: __le32,
    pub mcast_qid: __le32,
    pub beacon_template: __le32,
    pub /: *mut *mut } __packed; / AP_MAC_DATA_API_S_VER_2,
//
// struct iwl_mac_data_ibss - configuration data for IBSS MAC context
// @beacon_time: beacon transmit time in system time
// @beacon_tsf: beacon transmit time in TSF
// @bi: beacon interval in TU
// @reserved: reserved
// @beacon_template: beacon template ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_data_ibss {
    pub beacon_time: __le32,
    pub beacon_tsf: __le64,
    pub bi: __le32,
    pub reserved: __le32,
    pub beacon_template: __le32,
    pub /: *mut *mut } __packed; / IBSS_MAC_DATA_API_S_VER_1,
//
// enum iwl_mac_data_policy - policy of the data path for this MAC
// @TWT_SUPPORTED: twt is supported
// @MORE_DATA_ACK_SUPPORTED: AP supports More Data Ack according to
// paragraph 9.4.1.17 in P802.11ax_D4 specification. Used for TWT
// early termination detection.
// @FLEXIBLE_TWT_SUPPORTED: AP supports flexible TWT schedule
// @PROTECTED_TWT_SUPPORTED: AP supports protected TWT frames (with 11w)
// @BROADCAST_TWT_SUPPORTED: AP and STA support broadcast TWT
// @COEX_HIGH_PRIORITY_ENABLE: high priority mode for BT coex, to be used
// during 802.1X negotiation (and allowed during 4-way-HS)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mac_data_policy {
    TWT_SUPPORTED = BIT(0),
    MORE_DATA_ACK_SUPPORTED = BIT(1),
    FLEXIBLE_TWT_SUPPORTED = BIT(2),
    PROTECTED_TWT_SUPPORTED = BIT(3),
    BROADCAST_TWT_SUPPORTED = BIT(4),
    COEX_HIGH_PRIORITY_ENABLE = BIT(5),
}

//
// struct iwl_mac_data_sta - configuration data for station MAC context
// @is_assoc: 1 for associated state, 0 otherwise
// @dtim_time: DTIM arrival time in system time
// @dtim_tsf: DTIM arrival time in TSF
// @bi: beacon interval in TU, applicable only when associated
// @reserved1: reserved
// @dtim_interval: DTIM interval in TU, applicable only when associated
// @data_policy: see &enum iwl_mac_data_policy
// @listen_interval: in beacon intervals, applicable only when associated
// @assoc_id: unique ID assigned by the AP during association
// @assoc_beacon_arrive_time: TSF of first beacon after association
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_data_sta {
    pub is_assoc: __le32,
    pub dtim_time: __le32,
    pub dtim_tsf: __le64,
    pub bi: __le32,
    pub reserved1: __le32,
    pub dtim_interval: __le32,
    pub data_policy: __le32,
    pub listen_interval: __le32,
    pub assoc_id: __le32,
    pub assoc_beacon_arrive_time: __le32,
    pub /: *mut *mut } __packed; / STA_MAC_DATA_API_S_VER_2,
//
// struct iwl_mac_data_go - configuration data for P2P GO MAC context
// @ap: iwl_mac_data_ap struct with most config data
// @ctwin: client traffic window in TU (period after TBTT when GO is present).
// 0 indicates that there is no CT window.
// @opp_ps_enabled: indicate that opportunistic PS allowed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_data_go {
    pub ap: iwl_mac_data_ap,
    pub ctwin: __le32,
    pub opp_ps_enabled: __le32,
    pub /: *mut *mut } __packed; / GO_MAC_DATA_API_S_VER_1,
//
// struct iwl_mac_data_p2p_sta - configuration data for P2P client MAC context
// @sta: iwl_mac_data_sta struct with most config data
// @ctwin: client traffic window in TU (period after TBTT when GO is present).
// 0 indicates that there is no CT window.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_data_p2p_sta {
    pub sta: iwl_mac_data_sta,
    pub ctwin: __le32,
    pub /: *mut *mut } __packed; / P2P_STA_MAC_DATA_API_S_VER_2,
//
// struct iwl_mac_data_pibss - Pseudo IBSS config data
// @stats_interval: interval in TU between statistics notifications to host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_data_pibss {
    pub stats_interval: __le32,
    pub /: *mut *mut } __packed; / PIBSS_MAC_DATA_API_S_VER_1,
//
// struct iwl_mac_data_p2p_dev - configuration data for the P2P Device MAC
// context.
// @is_disc_extended: if set to true, P2P Device discoverability is enabled on
// other channels as well. This should be to true only in case that the
// device is discoverable and there is an active GO. Note that setting this
// field when not needed, will increase the number of interrupts and have
// effect on the platform power, as this setting opens the Rx filters on
// all macs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_data_p2p_dev {
    pub is_disc_extended: __le32,
    pub /: *mut *mut } __packed; / _P2P_DEV_MAC_DATA_API_S_VER_1,
//
// enum iwl_mac_filter_flags - MAC context filter flags
// @MAC_FILTER_IN_PROMISC: accept all data frames
// @MAC_FILTER_IN_CONTROL_AND_MGMT: pass all management and
// control frames to the host
// @MAC_FILTER_ACCEPT_GRP: accept multicast frames
// @MAC_FILTER_DIS_DECRYPT: don't decrypt unicast frames
// @MAC_FILTER_DIS_GRP_DECRYPT: don't decrypt multicast frames
// @MAC_FILTER_IN_BEACON: transfer foreign BSS's beacons to host
// (in station mode when associated)
// @MAC_FILTER_OUT_BCAST: filter out all broadcast frames
// @MAC_FILTER_IN_CRC32: extract FCS and append it to frames
// @MAC_FILTER_IN_PROBE_REQUEST: pass probe requests to host
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mac_filter_flags {
    MAC_FILTER_IN_PROMISC		= BIT(0),
    MAC_FILTER_IN_CONTROL_AND_MGMT	= BIT(1),
    MAC_FILTER_ACCEPT_GRP		= BIT(2),
    MAC_FILTER_DIS_DECRYPT		= BIT(3),
    MAC_FILTER_DIS_GRP_DECRYPT	= BIT(4),
    MAC_FILTER_IN_BEACON		= BIT(6),
    MAC_FILTER_OUT_BCAST		= BIT(8),
    MAC_FILTER_IN_CRC32		= BIT(11),
    MAC_FILTER_IN_PROBE_REQUEST	= BIT(12),
//
// @MAC_FILTER_IN_11AX: mark BSS as supporting 802.11ax
//
    MAC_FILTER_IN_11AX		= BIT(14),
}

//
// enum iwl_mac_qos_flags - QoS flags
// @MAC_QOS_FLG_UPDATE_EDCA: ?
// @MAC_QOS_FLG_TGN: HT is enabled
// @MAC_QOS_FLG_TXOP_TYPE: ?
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mac_qos_flags {
    MAC_QOS_FLG_UPDATE_EDCA	= BIT(0),
    MAC_QOS_FLG_TGN		= BIT(1),
    MAC_QOS_FLG_TXOP_TYPE	= BIT(4),
}

//
// struct iwl_ac_qos - QOS timing params for MAC_CONTEXT_CMD
// @cw_min: Contention window, start value in numbers of slots.
// Should be a power-of-2, minus 1.  Device's default is 0x0f.
// @cw_max: Contention window, max value in numbers of slots.
// Should be a power-of-2, minus 1.  Device's default is 0x3f.
// @aifsn:  Number of slots in Arbitration Interframe Space (before
// performing random backoff timing prior to Tx).  Device default 1.
// @fifos_mask: FIFOs used by this MAC for this AC
// @edca_txop:  Length of Tx opportunity, in uSecs.  Device default is 0.
//
// One instance of this config struct for each of 4 EDCA access categories
// in struct iwl_qosparam_cmd.
//
// Device will automatically increase contention window by (2*CW) + 1 for each
// transmission retry.  Device uses cw_max as a bit mask, ANDed with new CW
// value, to cap the CW value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ac_qos {
    pub cw_min: __le16,
    pub cw_max: __le16,
    pub aifsn: u8,
    pub /: *mut *mut u8 fifos_mask; / not in use since _VER_3,
    pub edca_txop: __le16,
    pub /: *mut *mut } __packed; / AC_QOS_API_S_VER_2, _VER_3,
//
// struct iwl_mac_ctx_cmd - command structure to configure MAC contexts
// ( MAC_CONTEXT_CMD = 0x28 )
// @id_and_color: ID and color of the MAC
// @action: action to perform, see &enum iwl_ctxt_action
// @mac_type: one of &enum iwl_mac_types
// @tsf_id: TSF HW timer, one of &enum iwl_tsf_id
// @node_addr: MAC address
// @reserved_for_node_addr: reserved
// @bssid_addr: BSSID
// @reserved_for_bssid_addr: reserved
// @cck_rates: basic rates available for CCK
// @ofdm_rates: basic rates available for OFDM
// @protection_flags: combination of &enum iwl_mac_protection_flags
// @cck_short_preamble: 0x20 for enabling short preamble, 0 otherwise
// @short_slot: 0x10 for enabling short slots, 0 otherwise
// @filter_flags: combination of &enum iwl_mac_filter_flags
// @qos_flags: from &enum iwl_mac_qos_flags
// @ac: one iwl_mac_qos configuration for each AC
// @ap: AP specific config data, see &struct iwl_mac_data_ap
// @go: GO specific config data, see &struct iwl_mac_data_go
// @sta: BSS client specific config data, see &struct iwl_mac_data_sta
// @p2p_sta: P2P client specific config data, see &struct iwl_mac_data_p2p_sta
// @p2p_dev: P2P-device specific config data, see &struct iwl_mac_data_p2p_dev
// @pibss: Pseudo-IBSS specific data, unused; see struct iwl_mac_data_pibss
// @ibss: IBSS specific config data, see &struct iwl_mac_data_ibss
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_ctx_cmd {
// COMMON_INDEX_HDR_API_S_VER_1
    pub id_and_color: __le32,
    pub action: __le32,
// MAC_CONTEXT_COMMON_DATA_API_S_VER_1
    pub mac_type: __le32,
    pub tsf_id: __le32,
    pub node_addr: [u8; 6],
    pub reserved_for_node_addr: __le16,
    pub bssid_addr: [u8; 6],
    pub reserved_for_bssid_addr: __le16,
    pub cck_rates: __le32,
    pub ofdm_rates: __le32,
    pub protection_flags: __le32,
    pub cck_short_preamble: __le32,
    pub short_slot: __le32,
    pub filter_flags: __le32,
// MAC_QOS_PARAM_API_S_VER_1
    pub qos_flags: __le32,
    pub ac: [iwl_ac_qos; AC_NUM+1],
// MAC_CONTEXT_COMMON_DATA_API_S
    pub ap: iwl_mac_data_ap,
    pub go: iwl_mac_data_go,
    pub sta: iwl_mac_data_sta,
    pub p2p_sta: iwl_mac_data_p2p_sta,
    pub p2p_dev: iwl_mac_data_p2p_dev,
    pub pibss: iwl_mac_data_pibss,
    pub ibss: iwl_mac_data_ibss,
}

pub const IWL_NONQOS_SEQ_GET: c_uint = 0x1;
pub const IWL_NONQOS_SEQ_SET: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nonqos_seq_query_cmd {
    pub get_set_flag: __le32,
    pub mac_id_n_color: __le32,
    pub value: __le16,
    pub reserved: __le16,
    pub /: *mut *mut } __packed; / NON_QOS_TX_COUNTER_GET_SET_API_S_VER_1,
//
// struct iwl_missed_beacons_notif_ver_3 - information on missed beacons
// ( MISSED_BEACONS_NOTIFICATION = 0xa2 )
// @mac_id: interface ID
// @consec_missed_beacons_since_last_rx: number of consecutive missed
// beacons since last RX.
// @consec_missed_beacons: number of consecutive missed beacons
// @num_expected_beacons: number of expected beacons
// @num_recvd_beacons: number of received beacons
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_missed_beacons_notif_ver_3 {
    pub mac_id: __le32,
    pub consec_missed_beacons_since_last_rx: __le32,
    pub consec_missed_beacons: __le32,
    pub num_expected_beacons: __le32,
    pub num_recvd_beacons: __le32,
    pub /: *mut *mut } __packed; / MISSED_BEACON_NTFY_API_S_VER_3,
//
// struct iwl_missed_beacons_notif_v4 - information on missed beacons
// ( MISSED_BEACONS_NOTIFICATION = 0xa2 )
// @link_id: fw link ID
// @consec_missed_beacons_since_last_rx: number of consecutive missed
// beacons since last RX.
// @consec_missed_beacons: number of consecutive missed beacons
// @num_expected_beacons: number of expected beacons
// @num_recvd_beacons: number of received beacons
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_missed_beacons_notif_v4 {
    pub link_id: __le32,
    pub consec_missed_beacons_since_last_rx: __le32,
    pub consec_missed_beacons: __le32,
    pub num_expected_beacons: __le32,
    pub num_recvd_beacons: __le32,
    pub /: *mut *mut } __packed; / MISSED_BEACON_NTFY_API_S_VER_4,
//
// struct iwl_he_backoff_conf - used for backoff configuration
// Per each trigger-based AC, (set by MU EDCA Parameter set info-element)
// used for backoff configuration of TXF5..TXF8 trigger based.
// The MU-TIMER is reloaded w/ MU_TIME each time a frame from the AC is sent via
// trigger-based TX.
// @cwmin: CW min
// @cwmax: CW max
// @aifsn: AIFSN
// AIFSN=0, means that no backoff from the specified TRIG-BASED AC is
// allowed till the MU-TIMER is 0
// @mu_time: MU time in 8TU units
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_he_backoff_conf {
    pub cwmin: __le16,
    pub cwmax: __le16,
    pub aifsn: __le16,
    pub mu_time: __le16,
    pub /: *mut *mut } __packed; / AC_QOS_DOT11AX_API_S,
//
// enum iwl_he_pkt_ext_constellations - PPE constellation indices
// @IWL_HE_PKT_EXT_BPSK: BPSK
// @IWL_HE_PKT_EXT_QPSK:  QPSK
// @IWL_HE_PKT_EXT_16QAM: 16-QAM
// @IWL_HE_PKT_EXT_64QAM: 64-QAM
// @IWL_HE_PKT_EXT_256QAM: 256-QAM
// @IWL_HE_PKT_EXT_1024QAM: 1024-QAM
// @IWL_HE_PKT_EXT_4096QAM: 4096-QAM, for EHT only
// @IWL_HE_PKT_EXT_NONE: not defined
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_he_pkt_ext_constellations {
    IWL_HE_PKT_EXT_BPSK = 0,
    IWL_HE_PKT_EXT_QPSK,
    IWL_HE_PKT_EXT_16QAM,
    IWL_HE_PKT_EXT_64QAM,
    IWL_HE_PKT_EXT_256QAM,
    IWL_HE_PKT_EXT_1024QAM,
    IWL_HE_PKT_EXT_4096QAM,
    IWL_HE_PKT_EXT_NONE,
}

pub const MAX_HE_SUPP_NSS: c_int = 2;
pub const MAX_CHANNEL_BW_INDX_API_D_VER_1: c_int = 4;
pub const MAX_CHANNEL_BW_INDX_API_D_VER_2: c_int = 5;
//
// struct iwl_he_pkt_ext_v1 - QAM thresholds
// The required PPE is set via HE Capabilities IE, per Nss x BW x MCS
// The IE is organized in the following way:
// Support for Nss x BW (or RU) matrix:
// (0=SISO, 1=MIMO2) x (0-20MHz, 1-40MHz, 2-80MHz, 3-160MHz)
// Each entry contains 2 QAM thresholds for 8us and 16us:
// 0=BPSK, 1=QPSK, 2=16QAM, 3=64QAM, 4=256QAM, 5=1024QAM, 6=RES, 7=NONE
// i.e. QAM_th1 < QAM_th2 such if TX uses QAM_tx:
// QAM_tx < QAM_th1            --> PPE=0us
// QAM_th1 <= QAM_tx < QAM_th2 --> PPE=8us
// QAM_th2 <= QAM_tx           --> PPE=16us
// @pkt_ext_qam_th: QAM thresholds
// For each Nss/Bw define 2 QAM thrsholds (0..5)
// For rates below the low_th, no need for PPE
// For rates between low_th and high_th, need 8us PPE
// For rates equal or higher then the high_th, need 16us PPE
// Nss (0-siso, 1-mimo2) x BW (0-20MHz, 1-40MHz, 2-80MHz, 3-160MHz) x
// (0-low_th, 1-high_th)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_he_pkt_ext_v1 {
    pub pkt_ext_qam_th: [u8; MAX_HE_SUPP_NSS][MAX_CHANNEL_BW_INDX_API_D_VER_1][2],
    pub /: *mut *mut } __packed; / PKT_EXT_DOT11AX_API_S_VER_1,
//
// struct iwl_he_pkt_ext_v2 - QAM thresholds
// The required PPE is set via HE Capabilities IE, per Nss x BW x MCS
// The IE is organized in the following way:
// Support for Nss x BW (or RU) matrix:
// (0=SISO, 1=MIMO2) x (0-20MHz, 1-40MHz, 2-80MHz, 3-160MHz)
// Each entry contains 2 QAM thresholds for 8us and 16us:
// 0=BPSK, 1=QPSK, 2=16QAM, 3=64QAM, 4=256QAM, 5=1024QAM, 6=RES, 7=NONE
// i.e. QAM_th1 < QAM_th2 such if TX uses QAM_tx:
// QAM_tx < QAM_th1            --> PPE=0us
// QAM_th1 <= QAM_tx < QAM_th2 --> PPE=8us
// QAM_th2 <= QAM_tx           --> PPE=16us
// @pkt_ext_qam_th: QAM thresholds
// For each Nss/Bw define 2 QAM thrsholds (0..5)
// For rates below the low_th, no need for PPE
// For rates between low_th and high_th, need 8us PPE
// For rates equal or higher then the high_th, need 16us PPE
// Nss (0-siso, 1-mimo2) x
// BW (0-20MHz, 1-40MHz, 2-80MHz, 3-160MHz, 4-320MHz) x
// (0-low_th, 1-high_th)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_he_pkt_ext_v2 {
    pub pkt_ext_qam_th: [u8; MAX_HE_SUPP_NSS][MAX_CHANNEL_BW_INDX_API_D_VER_2][2],
    pub /: *mut *mut } __packed; / PKT_EXT_DOT11AX_API_S_VER_2,
//
// enum iwl_he_sta_ctxt_flags - HE STA context flags
// @STA_CTXT_HE_REF_BSSID_VALID: ref bssid addr valid (for receiving specific
// control frames such as TRIG, NDPA, BACK)
// @STA_CTXT_HE_BSS_COLOR_DIS: BSS color disable, don't use the BSS
// color for RX filter but use MAC header
// @STA_CTXT_HE_PARTIAL_BSS_COLOR: partial BSS color allocation
// @STA_CTXT_HE_32BIT_BA_BITMAP: indicates the receiver supports BA bitmap
// of 32-bits
// @STA_CTXT_HE_PACKET_EXT: indicates that the packet-extension info is valid
// and should be used
// @STA_CTXT_HE_TRIG_RND_ALLOC: indicates that trigger based random allocation
// is enabled according to UORA element existence
// @STA_CTXT_HE_CONST_TRIG_RND_ALLOC: used for AV testing
// @STA_CTXT_HE_ACK_ENABLED: indicates that the AP supports receiving ACK-
// enabled AGG, i.e. both BACK and non-BACK frames in a single AGG
// @STA_CTXT_HE_MU_EDCA_CW: indicates that there is an element of MU EDCA
// parameter set, i.e. the backoff counters for trig-based ACs
// @STA_CTXT_HE_NIC_NOT_ACK_ENABLED: mark that the NIC doesn't support receiving
// ACK-enabled AGG, (i.e. both BACK and non-BACK frames in single AGG).
// If the NIC is not ACK_ENABLED it may use the EOF-bit in first non-0
// len delim to determine if AGG or single.
// @STA_CTXT_HE_RU_2MHZ_BLOCK: indicates that 26-tone RU OFDMA transmission are
// not allowed (as there are OBSS that might classify such transmissions as
// radar pulses).
// @STA_CTXT_HE_NDP_FEEDBACK_ENABLED: mark support for NDP feedback and change
// of threshold
// @STA_CTXT_EHT_PUNCTURE_MASK_VALID: indicates the puncture_mask field is valid
// @STA_CTXT_EHT_LONG_PPE_ENABLED: indicates the PPE requirement should be
// extended to 20us for BW > 160Mhz or for MCS w/ 4096-QAM.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_he_sta_ctxt_flags {
    STA_CTXT_HE_REF_BSSID_VALID		= BIT(4),
    STA_CTXT_HE_BSS_COLOR_DIS		= BIT(5),
    STA_CTXT_HE_PARTIAL_BSS_COLOR		= BIT(6),
    STA_CTXT_HE_32BIT_BA_BITMAP		= BIT(7),
    STA_CTXT_HE_PACKET_EXT			= BIT(8),
    STA_CTXT_HE_TRIG_RND_ALLOC		= BIT(9),
    STA_CTXT_HE_CONST_TRIG_RND_ALLOC	= BIT(10),
    STA_CTXT_HE_ACK_ENABLED			= BIT(11),
    STA_CTXT_HE_MU_EDCA_CW			= BIT(12),
    STA_CTXT_HE_NIC_NOT_ACK_ENABLED		= BIT(13),
    STA_CTXT_HE_RU_2MHZ_BLOCK		= BIT(14),
    STA_CTXT_HE_NDP_FEEDBACK_ENABLED	= BIT(15),
    STA_CTXT_EHT_PUNCTURE_MASK_VALID	= BIT(16),
    STA_CTXT_EHT_LONG_PPE_ENABLED		= BIT(17),
}

//
// enum iwl_he_htc_flags - HE HTC support flags
// @IWL_HE_HTC_SUPPORT: HE-HTC support
// @IWL_HE_HTC_UL_MU_RESP_SCHED: HE UL MU response schedule
// support via A-control field
// @IWL_HE_HTC_BSR_SUPP: BSR support in A-control field
// @IWL_HE_HTC_OMI_SUPP: A-OMI support in A-control field
// @IWL_HE_HTC_BQR_SUPP: A-BQR support in A-control field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_he_htc_flags {
    IWL_HE_HTC_SUPPORT			= BIT(0),
    IWL_HE_HTC_UL_MU_RESP_SCHED		= BIT(3),
    IWL_HE_HTC_BSR_SUPP			= BIT(4),
    IWL_HE_HTC_OMI_SUPP			= BIT(5),
    IWL_HE_HTC_BQR_SUPP			= BIT(6),
}

//
// @IWL_HE_HTC_LINK_ADAP_NO_FEEDBACK: the STA does not provide HE MFB
// @IWL_HE_HTC_LINK_ADAP_UNSOLICITED: the STA provides only unsolicited HE MFB
// @IWL_HE_HTC_LINK_ADAP_BOTH: the STA is capable of providing HE MFB in
// response to HE MRQ and if the STA provides unsolicited HE MFB
//

//
// struct iwl_he_sta_context_cmd_v1 - configure FW to work with HE AP
// @sta_id: STA id
// @tid_limit: max num of TIDs in TX HE-SU multi-TID agg
// 0 - bad value, 1 - multi-tid not supported, 2..8 - tid limit
// @reserved1: reserved byte for future use
// @reserved2: reserved byte for future use
// @flags: see %iwl_11ax_sta_ctxt_flags
// @ref_bssid_addr: reference BSSID used by the AP
// @reserved0: reserved 2 bytes for aligning the ref_bssid_addr field to 8 bytes
// @htc_flags: which features are supported in HTC
// @frag_flags: frag support in A-MSDU
// @frag_level: frag support level
// @frag_max_num: max num of "open" MSDUs in the receiver (in power of 2)
// @frag_min_size: min frag size (except last frag)
// @pkt_ext: optional, exists according to PPE-present bit in the HE-PHY capa
// @bss_color: 11ax AP ID that is used in the HE SIG-A to mark inter BSS frame
// @htc_trig_based_pkt_ext: default PE in 4us units
// @frame_time_rts_th: HE duration RTS threshold, in units of 32us
// @rand_alloc_ecwmin: random CWmin = 2**ECWmin-1
// @rand_alloc_ecwmax: random CWmax = 2**ECWmax-1
// @reserved3: reserved byte for future use
// @trig_based_txf: MU EDCA Parameter set for the trigger based traffic queues
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_he_sta_context_cmd_v1 {
    pub sta_id: u8,
    pub tid_limit: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub flags: __le32,
// The below fields are set via Multiple BSSID IE
    pub ref_bssid_addr: [u8; 6],
    pub reserved0: __le16,
// The below fields are set via HE-capabilities IE
    pub htc_flags: __le32,
    pub frag_flags: u8,
    pub frag_level: u8,
    pub frag_max_num: u8,
    pub frag_min_size: u8,
// The below fields are set via PPE thresholds element
    pub pkt_ext: iwl_he_pkt_ext_v1,
// The below fields are set via HE-Operation IE
    pub bss_color: u8,
    pub htc_trig_based_pkt_ext: u8,
    pub frame_time_rts_th: __le16,
// Random access parameter set (i.e. RAPS)
    pub rand_alloc_ecwmin: u8,
    pub rand_alloc_ecwmax: u8,
    pub reserved3: __le16,
// The below fields are set via MU EDCA parameter set element
    pub trig_based_txf: [iwl_he_backoff_conf; AC_NUM],
    pub /: *mut *mut } __packed; / STA_CONTEXT_DOT11AX_API_S_VER_1,
//
// struct iwl_he_sta_context_cmd_v2 - configure FW to work with HE AP
// @sta_id: STA id
// @tid_limit: max num of TIDs in TX HE-SU multi-TID agg
// 0 - bad value, 1 - multi-tid not supported, 2..8 - tid limit
// @reserved1: reserved byte for future use
// @reserved2: reserved byte for future use
// @flags: see %iwl_11ax_sta_ctxt_flags
// @ref_bssid_addr: reference BSSID used by the AP
// @reserved0: reserved 2 bytes for aligning the ref_bssid_addr field to 8 bytes
// @htc_flags: which features are supported in HTC
// @frag_flags: frag support in A-MSDU
// @frag_level: frag support level
// @frag_max_num: max num of "open" MSDUs in the receiver (in power of 2)
// @frag_min_size: min frag size (except last frag)
// @pkt_ext: optional, exists according to PPE-present bit in the HE-PHY capa
// @bss_color: 11ax AP ID that is used in the HE SIG-A to mark inter BSS frame
// @htc_trig_based_pkt_ext: default PE in 4us units
// @frame_time_rts_th: HE duration RTS threshold, in units of 32us
// @rand_alloc_ecwmin: random CWmin = 2**ECWmin-1
// @rand_alloc_ecwmax: random CWmax = 2**ECWmax-1
// @reserved3: reserved byte for future use
// @trig_based_txf: MU EDCA Parameter set for the trigger based traffic queues
// @max_bssid_indicator: indicator of the max bssid supported on the associated
// bss
// @bssid_index: index of the associated VAP
// @ema_ap: AP supports enhanced Multi BSSID advertisement
// @profile_periodicity: number of Beacon periods that are needed to receive the
// complete VAPs info
// @bssid_count: actual number of VAPs in the MultiBSS Set
// @reserved4: alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_he_sta_context_cmd_v2 {
    pub sta_id: u8,
    pub tid_limit: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub flags: __le32,
// The below fields are set via Multiple BSSID IE
    pub ref_bssid_addr: [u8; 6],
    pub reserved0: __le16,
// The below fields are set via HE-capabilities IE
    pub htc_flags: __le32,
    pub frag_flags: u8,
    pub frag_level: u8,
    pub frag_max_num: u8,
    pub frag_min_size: u8,
// The below fields are set via PPE thresholds element
    pub pkt_ext: iwl_he_pkt_ext_v1,
// The below fields are set via HE-Operation IE
    pub bss_color: u8,
    pub htc_trig_based_pkt_ext: u8,
    pub frame_time_rts_th: __le16,
// Random access parameter set (i.e. RAPS)
    pub rand_alloc_ecwmin: u8,
    pub rand_alloc_ecwmax: u8,
    pub reserved3: __le16,
// The below fields are set via MU EDCA parameter set element
    pub trig_based_txf: [iwl_he_backoff_conf; AC_NUM],
    pub max_bssid_indicator: u8,
    pub bssid_index: u8,
    pub ema_ap: u8,
    pub profile_periodicity: u8,
    pub bssid_count: u8,
    pub reserved4: [u8; 3],
    pub /: *mut *mut } __packed; / STA_CONTEXT_DOT11AX_API_S_VER_2,
//
// struct iwl_he_sta_context_cmd_v3 - configure FW to work with HE AP
// @sta_id: STA id
// @tid_limit: max num of TIDs in TX HE-SU multi-TID agg
// 0 - bad value, 1 - multi-tid not supported, 2..8 - tid limit
// @reserved1: reserved byte for future use
// @reserved2: reserved byte for future use
// @flags: see %iwl_11ax_sta_ctxt_flags
// @ref_bssid_addr: reference BSSID used by the AP
// @reserved0: reserved 2 bytes for aligning the ref_bssid_addr field to 8 bytes
// @htc_flags: which features are supported in HTC
// @frag_flags: frag support in A-MSDU
// @frag_level: frag support level
// @frag_max_num: max num of "open" MSDUs in the receiver (in power of 2)
// @frag_min_size: min frag size (except last frag)
// @pkt_ext: optional, exists according to PPE-present bit in the HE-PHY capa
// @bss_color: 11ax AP ID that is used in the HE SIG-A to mark inter BSS frame
// @htc_trig_based_pkt_ext: default PE in 4us units
// @frame_time_rts_th: HE duration RTS threshold, in units of 32us
// @rand_alloc_ecwmin: random CWmin = 2**ECWmin-1
// @rand_alloc_ecwmax: random CWmax = 2**ECWmax-1
// @puncture_mask: puncture mask for EHT
// @trig_based_txf: MU EDCA Parameter set for the trigger based traffic queues
// @max_bssid_indicator: indicator of the max bssid supported on the associated
// bss
// @bssid_index: index of the associated VAP
// @ema_ap: AP supports enhanced Multi BSSID advertisement
// @profile_periodicity: number of Beacon periods that are needed to receive the
// complete VAPs info
// @bssid_count: actual number of VAPs in the MultiBSS Set
// @reserved4: alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_he_sta_context_cmd_v3 {
    pub sta_id: u8,
    pub tid_limit: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub flags: __le32,
// The below fields are set via Multiple BSSID IE
    pub ref_bssid_addr: [u8; 6],
    pub reserved0: __le16,
// The below fields are set via HE-capabilities IE
    pub htc_flags: __le32,
    pub frag_flags: u8,
    pub frag_level: u8,
    pub frag_max_num: u8,
    pub frag_min_size: u8,
// The below fields are set via PPE thresholds element
    pub pkt_ext: iwl_he_pkt_ext_v2,
// The below fields are set via HE-Operation IE
    pub bss_color: u8,
    pub htc_trig_based_pkt_ext: u8,
    pub frame_time_rts_th: __le16,
// Random access parameter set (i.e. RAPS)
    pub rand_alloc_ecwmin: u8,
    pub rand_alloc_ecwmax: u8,
    pub puncture_mask: __le16,
// The below fields are set via MU EDCA parameter set element
    pub trig_based_txf: [iwl_he_backoff_conf; AC_NUM],
    pub max_bssid_indicator: u8,
    pub bssid_index: u8,
    pub ema_ap: u8,
    pub profile_periodicity: u8,
    pub bssid_count: u8,
    pub reserved4: [u8; 3],
    pub /: *mut *mut } __packed; / STA_CONTEXT_DOT11AX_API_S_VER_2,
//
// struct iwl_he_monitor_cmd - configure air sniffer for HE
// @bssid: the BSSID to sniff for
// @reserved1: reserved for dword alignment
// @aid: the AID to track on for HE MU
// @reserved2: reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_he_monitor_cmd {
    pub bssid: [u8; 6],
    pub reserved1: __le16,
    pub aid: __le16,
    pub reserved2: [u8; 6],
    pub /: *mut *mut } __packed; / HE_AIR_SNIFFER_CONFIG_CMD_API_S_VER_1,
