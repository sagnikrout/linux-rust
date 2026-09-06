//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/sta.h
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
// Copyright (C) 2012-2014, 2018-2021, 2023, 2025-2026 Intel Corporation
// Copyright (C) 2013-2014 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_sta_h__
//
// enum iwl_sta_flags - flags for the ADD_STA host command
// @STA_FLG_REDUCED_TX_PWR_CTRL: reduced TX power (control frames)
// @STA_FLG_REDUCED_TX_PWR_DATA: reduced TX power (data frames)
// @STA_FLG_DISABLE_TX: set if TX should be disabled
// @STA_FLG_PS: set if STA is in Power Save
// @STA_FLG_DRAIN_FLOW: drain flow
// @STA_FLG_PAN: STA is for PAN interface
// @STA_FLG_CLASS_AUTH: station is authenticated
// @STA_FLG_CLASS_ASSOC: station is associated
// @STA_FLG_RTS_MIMO_PROT: station requires RTS MIMO protection (dynamic SMPS)
// @STA_FLG_MAX_AGG_SIZE_MSK: maximal size for A-MPDU (mask)
// @STA_FLG_MAX_AGG_SIZE_SHIFT: maximal size for A-MPDU (bit shift)
// @STA_FLG_MAX_AGG_SIZE_8K: maximal size for A-MPDU (8k supported)
// @STA_FLG_MAX_AGG_SIZE_16K: maximal size for A-MPDU (16k supported)
// @STA_FLG_MAX_AGG_SIZE_32K: maximal size for A-MPDU (32k supported)
// @STA_FLG_MAX_AGG_SIZE_64K: maximal size for A-MPDU (64k supported)
// @STA_FLG_MAX_AGG_SIZE_128K: maximal size for A-MPDU (128k supported)
// @STA_FLG_MAX_AGG_SIZE_256K: maximal size for A-MPDU (256k supported)
// @STA_FLG_MAX_AGG_SIZE_512K: maximal size for A-MPDU (512k supported)
// @STA_FLG_MAX_AGG_SIZE_1024K: maximal size for A-MPDU (1024k supported)
// @STA_FLG_MAX_AGG_SIZE_2M: maximal size for A-MPDU (2M supported)
// @STA_FLG_MAX_AGG_SIZE_4M: maximal size for A-MPDU (4M supported)
// @STA_FLG_AGG_MPDU_DENS_MSK: maximal MPDU density for Tx aggregation
// @STA_FLG_FAT_EN_MSK: support for channel width (for Tx). This flag is
// initialised by driver and can be updated by fw upon reception of
// action frames that can change the channel width. When cleared the fw
// will send all the frames in 20MHz even when FAT channel is requested.
// @STA_FLG_FAT_EN_20MHZ: no wide channels are supported, only 20 MHz
// @STA_FLG_FAT_EN_40MHZ: wide channels up to 40 MHz supported
// @STA_FLG_FAT_EN_80MHZ: wide channels up to 80 MHz supported
// @STA_FLG_FAT_EN_160MHZ: wide channels up to 160 MHz supported
// @STA_FLG_MIMO_EN_MSK: support for MIMO. This flag is initialised by the
// driver and can be updated by fw upon reception of action frames.
// @STA_FLG_MIMO_EN_SISO: no support for MIMO
// @STA_FLG_MIMO_EN_MIMO2: 2 streams supported
// @STA_FLG_MIMO_EN_MIMO3: 3 streams supported
// @STA_FLG_AGG_MPDU_DENS_MSK: A-MPDU density (mask)
// @STA_FLG_AGG_MPDU_DENS_SHIFT: A-MPDU density (bit shift)
// @STA_FLG_AGG_MPDU_DENS_2US: A-MPDU density (2 usec gap)
// @STA_FLG_AGG_MPDU_DENS_4US: A-MPDU density (4 usec gap)
// @STA_FLG_AGG_MPDU_DENS_8US: A-MPDU density (8 usec gap)
// @STA_FLG_AGG_MPDU_DENS_16US: A-MPDU density (16 usec gap)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sta_flags {
    STA_FLG_REDUCED_TX_PWR_CTRL	= BIT(3),
    STA_FLG_REDUCED_TX_PWR_DATA	= BIT(6),

    STA_FLG_DISABLE_TX		= BIT(4),

    STA_FLG_PS			= BIT(8),
    STA_FLG_DRAIN_FLOW		= BIT(12),
    STA_FLG_PAN			= BIT(13),
    STA_FLG_CLASS_AUTH		= BIT(14),
    STA_FLG_CLASS_ASSOC		= BIT(15),
    STA_FLG_RTS_MIMO_PROT		= BIT(17),

    STA_FLG_MAX_AGG_SIZE_SHIFT	= 19,
    STA_FLG_MAX_AGG_SIZE_8K		= (0 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_16K	= (1 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_32K	= (2 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_64K	= (3 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_128K	= (4 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_256K	= (5 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_512K	= (6 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_1024K	= (7 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_2M		= (8 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_4M		= (9 << STA_FLG_MAX_AGG_SIZE_SHIFT),
    STA_FLG_MAX_AGG_SIZE_MSK	= (0xf << STA_FLG_MAX_AGG_SIZE_SHIFT),

    STA_FLG_AGG_MPDU_DENS_SHIFT	= 23,
    STA_FLG_AGG_MPDU_DENS_2US	= (4 << STA_FLG_AGG_MPDU_DENS_SHIFT),
    STA_FLG_AGG_MPDU_DENS_4US	= (5 << STA_FLG_AGG_MPDU_DENS_SHIFT),
    STA_FLG_AGG_MPDU_DENS_8US	= (6 << STA_FLG_AGG_MPDU_DENS_SHIFT),
    STA_FLG_AGG_MPDU_DENS_16US	= (7 << STA_FLG_AGG_MPDU_DENS_SHIFT),
    STA_FLG_AGG_MPDU_DENS_MSK	= (7 << STA_FLG_AGG_MPDU_DENS_SHIFT),

    STA_FLG_FAT_EN_20MHZ		= (0 << 26),
    STA_FLG_FAT_EN_40MHZ		= (1 << 26),
    STA_FLG_FAT_EN_80MHZ		= (2 << 26),
    STA_FLG_FAT_EN_160MHZ		= (3 << 26),
    STA_FLG_FAT_EN_MSK		= (3 << 26),

    STA_FLG_MIMO_EN_SISO		= (0 << 28),
    STA_FLG_MIMO_EN_MIMO2		= (1 << 28),
    STA_FLG_MIMO_EN_MIMO3		= (2 << 28),
    STA_FLG_MIMO_EN_MSK		= (3 << 28),
}

//
// enum iwl_sta_key_flag - key flags for the ADD_STA host command
// @STA_KEY_FLG_NO_ENC: no encryption
// @STA_KEY_FLG_WEP: WEP encryption algorithm
// @STA_KEY_FLG_CCM: CCMP encryption algorithm
// @STA_KEY_FLG_TKIP: TKIP encryption algorithm
// @STA_KEY_FLG_EXT: extended cipher algorithm (depends on the FW support)
// @STA_KEY_FLG_GCMP: GCMP encryption algorithm
// @STA_KEY_FLG_CMAC: CMAC encryption algorithm
// @STA_KEY_FLG_ENC_UNKNOWN: unknown encryption algorithm
// @STA_KEY_FLG_EN_MSK: mask for encryption algorithmi value
// @STA_KEY_FLG_WEP_KEY_MAP: wep is either a group key (0 - legacy WEP) or from
// station info array (1 - n 1X mode)
// @STA_KEY_FLG_AMSDU_SPP: SPP (signaling and payload protected) A-MSDU
// @STA_KEY_FLG_KEYID_MSK: the index of the key
// @STA_KEY_FLG_KEYID_POS: key index bit position
// @STA_KEY_NOT_VALID: key is invalid
// @STA_KEY_FLG_WEP_13BYTES: set for 13 bytes WEP key
// @STA_KEY_FLG_KEY_32BYTES: for non-wep key set for 32 bytes key
// @STA_KEY_MULTICAST: set for multical key
// @STA_KEY_MFP: key is used for Management Frame Protection
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sta_key_flag {
    STA_KEY_FLG_NO_ENC		= (0 << 0),
    STA_KEY_FLG_WEP			= (1 << 0),
    STA_KEY_FLG_CCM			= (2 << 0),
    STA_KEY_FLG_TKIP		= (3 << 0),
    STA_KEY_FLG_EXT			= (4 << 0),
    STA_KEY_FLG_GCMP		= (5 << 0),
    STA_KEY_FLG_CMAC		= (6 << 0),
    STA_KEY_FLG_ENC_UNKNOWN		= (7 << 0),
    STA_KEY_FLG_EN_MSK		= (7 << 0),

    STA_KEY_FLG_WEP_KEY_MAP		= BIT(3),
    STA_KEY_FLG_AMSDU_SPP		= BIT(7),
    STA_KEY_FLG_KEYID_POS		 = 8,
    STA_KEY_FLG_KEYID_MSK		= (3 << STA_KEY_FLG_KEYID_POS),
    STA_KEY_NOT_VALID		= BIT(11),
    STA_KEY_FLG_WEP_13BYTES		= BIT(12),
    STA_KEY_FLG_KEY_32BYTES		= BIT(12),
    STA_KEY_MULTICAST		= BIT(14),
    STA_KEY_MFP			= BIT(15),
}

//
// enum iwl_sta_modify_flag - indicate to the fw what flag are being changed
// @STA_MODIFY_QUEUE_REMOVAL: this command removes a queue
// @STA_MODIFY_TID_DISABLE_TX: this command modifies %tid_disable_tx
// @STA_MODIFY_UAPSD_ACS: this command modifies %uapsd_acs
// @STA_MODIFY_ADD_BA_TID: this command modifies %add_immediate_ba_tid
// @STA_MODIFY_REMOVE_BA_TID: this command modifies %remove_immediate_ba_tid
// @STA_MODIFY_SLEEPING_STA_TX_COUNT: this command modifies %sleep_tx_count
// @STA_MODIFY_PROT_TH: modify RTS threshold
// @STA_MODIFY_QUEUES: modify the queues used by this station
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sta_modify_flag {
    STA_MODIFY_QUEUE_REMOVAL		= BIT(0),
    STA_MODIFY_TID_DISABLE_TX		= BIT(1),
    STA_MODIFY_UAPSD_ACS			= BIT(2),
    STA_MODIFY_ADD_BA_TID			= BIT(3),
    STA_MODIFY_REMOVE_BA_TID		= BIT(4),
    STA_MODIFY_SLEEPING_STA_TX_COUNT	= BIT(5),
    STA_MODIFY_PROT_TH			= BIT(6),
    STA_MODIFY_QUEUES			= BIT(7),
}

//
// enum iwl_sta_mode - station command mode
// @STA_MODE_ADD: add new station
// @STA_MODE_MODIFY: modify the station
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sta_mode {
    STA_MODE_ADD	= 0,
    STA_MODE_MODIFY	= 1,
}

//
// enum iwl_sta_sleep_flag - type of sleep of the station
// @STA_SLEEP_STATE_AWAKE: station is awake
// @STA_SLEEP_STATE_PS_POLL: station is PS-polling
// @STA_SLEEP_STATE_UAPSD: station uses U-APSD
// @STA_SLEEP_STATE_MOREDATA: set more-data bit on
// (last) released frame
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sta_sleep_flag {
    STA_SLEEP_STATE_AWAKE		= 0,
    STA_SLEEP_STATE_PS_POLL		= BIT(0),
    STA_SLEEP_STATE_UAPSD		= BIT(1),
    STA_SLEEP_STATE_MOREDATA	= BIT(2),
}

pub const IWL_ADD_STA_STATUS_MASK: c_uint = 0xFF;
pub const IWL_ADD_STA_BAID_VALID_MASK: c_uint = 0x8000;
pub const IWL_ADD_STA_BAID_MASK: c_uint = 0x7F00;
pub const IWL_ADD_STA_BAID_SHIFT: c_int = 8;
//
// struct iwl_mvm_add_sta_cmd_v7 - Add/modify a station in the fw's sta table.
// ( REPLY_ADD_STA = 0x18 )
// @add_modify: see &enum iwl_sta_mode
// @awake_acs: ACs to transmit data on while station is sleeping (for U-APSD)
// @tid_disable_tx: is tid BIT(tid) enabled for Tx. Clear BIT(x) to enable
// AMPDU for tid x. Set %STA_MODIFY_TID_DISABLE_TX to change this field.
// @mac_id_n_color: the Mac context this station belongs to,
// see &enum iwl_ctxt_id_and_color
// @addr: station's MAC address
// @reserved2: reserved
// @sta_id: index of station in uCode's station table
// @modify_mask: from &enum iwl_sta_modify_flag, selects what to change
// @reserved3: reserved
// @station_flags: look at &enum iwl_sta_flags
// @station_flags_msk: what of %station_flags have changed,
// also &enum iwl_sta_flags
// @add_immediate_ba_tid: tid for which to add block-ack support (Rx)
// Set %STA_MODIFY_ADD_BA_TID to use this field, and also set
// add_immediate_ba_ssn.
// @remove_immediate_ba_tid: tid for which to remove block-ack support (Rx)
// Set %STA_MODIFY_REMOVE_BA_TID to use this field
// @add_immediate_ba_ssn: ssn for the Rx block-ack session. Used together with
// add_immediate_ba_tid.
// @sleep_tx_count: number of packets to transmit to station even though it is
// asleep. Used to synchronise PS-poll and u-APSD responses while ucode
// keeps track of STA sleep state.
// @sleep_state_flags: Look at &enum iwl_sta_sleep_flag.
// @assoc_id: assoc_id to be sent in VHT PLCP (9-bit), for grp use 0, for AP
// mac-addr.
// @beamform_flags: beam forming controls
// @tfd_queue_msk: tfd queues used by this station
//
// The device contains an internal table of per-station information, with info
// on security keys, aggregation parameters, and Tx rates for initial Tx
// attempt and any retries (set by REPLY_TX_LINK_QUALITY_CMD).
//
// ADD_STA sets up the table entry for one station, either creating a new
// entry, or modifying a pre-existing one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_add_sta_cmd_v7 {
    pub add_modify: u8,
    pub awake_acs: u8,
    pub tid_disable_tx: __le16,
    pub mac_id_n_color: __le32,
    pub /: *mut *mut u8 addr[ETH_ALEN]; / _STA_ID_MODIFY_INFO_API_S_VER_1,
    pub reserved2: __le16,
    pub sta_id: u8,
    pub modify_mask: u8,
    pub reserved3: __le16,
    pub station_flags: __le32,
    pub station_flags_msk: __le32,
    pub add_immediate_ba_tid: u8,
    pub remove_immediate_ba_tid: u8,
    pub add_immediate_ba_ssn: __le16,
    pub sleep_tx_count: __le16,
    pub sleep_state_flags: __le16,
    pub assoc_id: __le16,
    pub beamform_flags: __le16,
    pub tfd_queue_msk: __le32,
    pub /: *mut *mut } __packed; / ADD_STA_CMD_API_S_VER_7,
//
// enum iwl_sta_type - FW station types
// ( REPLY_ADD_STA = 0x18 )
// @IWL_STA_LINK: Link station - normal RX and TX traffic.
// @IWL_STA_GENERAL_PURPOSE: General purpose. In AP mode used for beacons
// and probe responses.
// @IWL_STA_MULTICAST: multicast traffic,
// @IWL_STA_TDLS_LINK: TDLS link station
// @IWL_STA_AUX_ACTIVITY: auxilary station (scan, ROC and so on).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sta_type {
    IWL_STA_LINK,
    IWL_STA_GENERAL_PURPOSE,
    IWL_STA_MULTICAST,
    IWL_STA_TDLS_LINK,
    IWL_STA_AUX_ACTIVITY,
}

//
// struct iwl_mvm_add_sta_cmd - Add/modify a station in the fw's sta table.
// ( REPLY_ADD_STA = 0x18 )
// @add_modify: see &enum iwl_sta_mode
// @awake_acs: ACs to transmit data on while station is sleeping (for U-APSD)
// @tid_disable_tx: is tid BIT(tid) enabled for Tx. Clear BIT(x) to enable
// AMPDU for tid x. Set %STA_MODIFY_TID_DISABLE_TX to change this field.
// @mac_id_n_color: the Mac context this station belongs to,
// see &enum iwl_ctxt_id_and_color
// @addr: station's MAC address
// @reserved2: reserved
// @sta_id: index of station in uCode's station table
// @modify_mask: from &enum iwl_sta_modify_flag, selects what to change
// @reserved3: reserved
// @station_flags: look at &enum iwl_sta_flags
// @station_flags_msk: what of %station_flags have changed,
// also &enum iwl_sta_flags
// @add_immediate_ba_tid: tid for which to add block-ack support (Rx)
// Set %STA_MODIFY_ADD_BA_TID to use this field, and also set
// add_immediate_ba_ssn.
// @remove_immediate_ba_tid: tid for which to remove block-ack support (Rx)
// Set %STA_MODIFY_REMOVE_BA_TID to use this field
// @add_immediate_ba_ssn: ssn for the Rx block-ack session. Used together with
// add_immediate_ba_tid.
// @sleep_tx_count: number of packets to transmit to station even though it is
// asleep. Used to synchronise PS-poll and u-APSD responses while ucode
// keeps track of STA sleep state.
// @station_type: type of this station. See &enum iwl_sta_type.
// @sleep_state_flags: Look at &enum iwl_sta_sleep_flag.
// @assoc_id: assoc_id to be sent in VHT PLCP (9-bit), for grp use 0, for AP
// mac-addr.
// @beamform_flags: beam forming controls
// @tfd_queue_msk: tfd queues used by this station.
// Obselete for new TX API (9 and above).
// @rx_ba_window: aggregation window size
// @sp_length: the size of the SP in actual number of frames
// @uapsd_acs:  4 LS bits are trigger enabled ACs, 4 MS bits are the deliver
// enabled ACs.
//
// The device contains an internal table of per-station information, with info
// on security keys, aggregation parameters, and Tx rates for initial Tx
// attempt and any retries (set by REPLY_TX_LINK_QUALITY_CMD).
//
// ADD_STA sets up the table entry for one station, either creating a new
// entry, or modifying a pre-existing one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_add_sta_cmd {
    pub add_modify: u8,
    pub awake_acs: u8,
    pub tid_disable_tx: __le16,
    pub /: *mut *mut __le32 mac_id_n_color; / can be used for lmac id when using cmd v12,
    pub /: *mut *mut u8 addr[ETH_ALEN]; / _STA_ID_MODIFY_INFO_API_S_VER_1,
    pub reserved2: __le16,
    pub sta_id: u8,
    pub modify_mask: u8,
    pub reserved3: __le16,
    pub station_flags: __le32,
    pub station_flags_msk: __le32,
    pub add_immediate_ba_tid: u8,
    pub remove_immediate_ba_tid: u8,
    pub add_immediate_ba_ssn: __le16,
    pub sleep_tx_count: __le16,
    pub sleep_state_flags: u8,
    pub station_type: u8,
    pub assoc_id: __le16,
    pub beamform_flags: __le16,
    pub tfd_queue_msk: __le32,
    pub rx_ba_window: __le16,
    pub sp_length: u8,
    pub uapsd_acs: u8,
    pub /: *mut *mut } __packed; / ADD_STA_CMD_API_S_VER_10,
//
// struct iwl_mvm_add_sta_key_common - add/modify sta key common part
// ( REPLY_ADD_STA_KEY = 0x17 )
// @sta_id: index of station in uCode's station table
// @key_offset: key offset in key storage
// @key_flags: type &enum iwl_sta_key_flag
// @key: key material data
// @rx_secur_seq_cnt: RX security sequence counter for the key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_add_sta_key_common {
    pub sta_id: u8,
    pub key_offset: u8,
    pub key_flags: __le16,
    pub key: [u8; 32],
    pub rx_secur_seq_cnt: [u8; 16],
    pub __packed: },
//
// struct iwl_mvm_add_sta_key_cmd_v1 - add/modify sta key
// @common: see &struct iwl_mvm_add_sta_key_common
// @tkip_rx_tsc_byte2: TSC[2] for key mix ph1 detection
// @reserved: reserved
// @tkip_rx_ttak: 10-byte unicast TKIP TTAK for Rx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_add_sta_key_cmd_v1 {
    pub common: iwl_mvm_add_sta_key_common,
    pub tkip_rx_tsc_byte2: u8,
    pub reserved: u8,
    pub tkip_rx_ttak: [__le16; 5],
    pub /: *mut *mut } __packed; / ADD_MODIFY_STA_KEY_API_S_VER_1,
//
// struct iwl_mvm_add_sta_key_cmd - add/modify sta key
// @common: see &struct iwl_mvm_add_sta_key_common
// @rx_mic_key: TKIP RX unicast or multicast key
// @tx_mic_key: TKIP TX key
// @transmit_seq_cnt: TSC, transmit packet number
//
// Note: This is used for both v2 and v3, the difference being
// in the way the common.rx_secur_seq_cnt is used, in v2 that's
// the strange hole format, in v3 it's just a u64.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_add_sta_key_cmd {
    pub common: iwl_mvm_add_sta_key_common,
    pub rx_mic_key: __le64,
    pub tx_mic_key: __le64,
    pub transmit_seq_cnt: __le64,
    pub /: *mut *mut } __packed; / ADD_MODIFY_STA_KEY_API_S_VER_2, ADD_MODIFY_STA_KEY_API_S_VER_3,
//
// enum iwl_mvm_add_sta_rsp_status - status in the response to ADD_STA command
// @ADD_STA_SUCCESS: operation was executed successfully
// @ADD_STA_STATIONS_OVERLOAD: no room left in the fw's station table
// @ADD_STA_IMMEDIATE_BA_FAILURE: can't add Rx block ack session
// @ADD_STA_MODIFY_NON_EXISTING_STA: driver requested to modify a station that
// doesn't exist.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_add_sta_rsp_status {
    ADD_STA_SUCCESS			= 0x1,
    ADD_STA_STATIONS_OVERLOAD	= 0x2,
    ADD_STA_IMMEDIATE_BA_FAILURE	= 0x4,
    ADD_STA_MODIFY_NON_EXISTING_STA	= 0x8,
}

//
// struct iwl_mvm_rm_sta_cmd - Add / modify a station in the fw's station table
// ( REMOVE_STA = 0x19 )
// @sta_id: the station id of the station to be removed
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_rm_sta_cmd {
    pub sta_id: u8,
    pub reserved: [u8; 3],
    pub /: *mut *mut } __packed; / REMOVE_STA_CMD_API_S_VER_2,
//
// struct iwl_mvm_mgmt_mcast_key_cmd_v1 - IGTK command
// ( MGMT_MCAST_KEY = 0x1f )
// @ctrl_flags: &enum iwl_sta_key_flag
// @igtk: IGTK key material
// @k1: unused
// @k2: unused
// @sta_id: station ID that support IGTK
// @key_id: key ID
// @receive_seq_cnt: initial RSC/PN needed for replay check
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_mgmt_mcast_key_cmd_v1 {
    pub ctrl_flags: __le32,
    pub igtk: [u8; 16],
    pub k1: [u8; 16],
    pub k2: [u8; 16],
    pub key_id: __le32,
    pub sta_id: __le32,
    pub receive_seq_cnt: __le64,
    pub /: *mut *mut } __packed; / SEC_MGMT_MULTICAST_KEY_CMD_API_S_VER_1,
//
// struct iwl_mvm_mgmt_mcast_key_cmd - IGTK command
// ( MGMT_MCAST_KEY = 0x1f )
// @ctrl_flags: &enum iwl_sta_key_flag
// @igtk: IGTK master key
// @sta_id: station ID that support IGTK
// @key_id: key ID
// @receive_seq_cnt: initial RSC/PN needed for replay check
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_mgmt_mcast_key_cmd {
    pub ctrl_flags: __le32,
    pub igtk: [u8; 32],
    pub key_id: __le32,
    pub sta_id: __le32,
    pub receive_seq_cnt: __le64,
    pub /: *mut *mut } __packed; / SEC_MGMT_MULTICAST_KEY_CMD_API_S_VER_2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_wep_key {
    pub key_index: u8,
    pub key_offset: u8,
    pub reserved1: __le16,
    pub key_size: u8,
    pub reserved2: [u8; 3],
    pub key: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_wep_key_cmd {
    pub mac_id_n_color: __le32,
    pub num_keys: u8,
    pub decryption_type: u8,
    pub flags: u8,
    pub reserved: u8,
    pub wep_key: [iwl_mvm_wep_key; ],
    pub /: *mut *mut } __packed; / SEC_CURR_WEP_KEY_CMD_API_S_VER_2,
//
// struct iwl_mvm_eosp_notification - EOSP notification from firmware
// @remain_frame_count: # of frames remaining, non-zero if SP was cut
// short by GO absence
// @sta_id: station ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_eosp_notification {
    pub remain_frame_count: __le32,
    pub sta_id: __le32,
    pub /: *mut *mut } __packed; / UAPSD_EOSP_NTFY_API_S_VER_1,
