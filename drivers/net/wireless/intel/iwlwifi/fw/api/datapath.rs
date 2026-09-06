//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/datapath.h
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
// Copyright (C) 2024-2026 Intel Corporation
// Copyright (C) 2012-2014, 2018-2022 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_datapath_h__
//
// enum iwl_data_path_subcmd_ids - data path group commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_data_path_subcmd_ids {
//
// @DQA_ENABLE_CMD: &struct iwl_dqa_enable_cmd
//
    DQA_ENABLE_CMD = 0x0,

//
// @UPDATE_MU_GROUPS_CMD: &struct iwl_mu_group_mgmt_cmd
//
    UPDATE_MU_GROUPS_CMD = 0x1,

//
// @TRIGGER_RX_QUEUES_NOTIF_CMD: &struct iwl_rxq_sync_cmd
//
    TRIGGER_RX_QUEUES_NOTIF_CMD = 0x2,

//
// @WNM_PLATFORM_PTM_REQUEST_CMD: &struct iwl_time_sync_cfg_cmd
//
    WNM_PLATFORM_PTM_REQUEST_CMD = 0x3,

//
// @WNM_80211V_TIMING_MEASUREMENT_CONFIG_CMD:
// &struct iwl_time_sync_cfg_cmd
//
    WNM_80211V_TIMING_MEASUREMENT_CONFIG_CMD = 0x4,

//
// @STA_HE_CTXT_CMD: &struct iwl_he_sta_context_cmd_v1,
// &struct iwl_he_sta_context_cmd_v2 or
// &struct iwl_he_sta_context_cmd_v3
//
    STA_HE_CTXT_CMD = 0x7,

//
// @RLC_CONFIG_CMD: &struct iwl_rlc_config_cmd
//
    RLC_CONFIG_CMD = 0x8,

//
// @RFH_QUEUE_CONFIG_CMD: &struct iwl_rfh_queue_config
//
    RFH_QUEUE_CONFIG_CMD = 0xD,

//
// @TLC_MNG_CONFIG_CMD: &struct iwl_tlc_config_cmd_v4 or
// &struct iwl_tlc_config_cmd.
//
    TLC_MNG_CONFIG_CMD = 0xF,

//
// @HE_AIR_SNIFFER_CONFIG_CMD: &struct iwl_he_monitor_cmd
//
    HE_AIR_SNIFFER_CONFIG_CMD = 0x13,

//
// @CHEST_COLLECTOR_FILTER_CONFIG_CMD: Configure the CSI
// matrix collection, uses &struct iwl_channel_estimation_cfg
//
    CHEST_COLLECTOR_FILTER_CONFIG_CMD = 0x14,

//
// @RX_BAID_ALLOCATION_CONFIG_CMD: Allocate/deallocate a BAID for an RX
// blockack session, uses &struct iwl_rx_baid_cfg_cmd for the
// command, and &struct iwl_rx_baid_cfg_resp as a response.
//
    RX_BAID_ALLOCATION_CONFIG_CMD = 0x16,

//
// @SCD_QUEUE_CONFIG_CMD: new scheduler queue allocation/config/removal
// command, uses &struct iwl_scd_queue_cfg_cmd and the response
// is (same as before) &struct iwl_tx_queue_cfg_rsp.
//
    SCD_QUEUE_CONFIG_CMD = 0x17,

//
// @SEC_KEY_CMD: security key command, uses &struct iwl_sec_key_cmd
//
    SEC_KEY_CMD = 0x18,

//
// @RSC_NOTIF: notification to update each Rx queue with the RSC. This
// notification is sent after resume and uses
// &struct iwl_wowlan_all_rsc_tsc_v5.
//
    RSC_NOTIF = 0xF1,

//
// @ESR_MODE_NOTIF: notification to recommend/force a wanted esr mode,
// uses &struct iwl_esr_mode_notif or &struct iwl_esr_mode_notif_v1
//
    ESR_MODE_NOTIF = 0xF3,

//
// @MONITOR_NOTIF: Datapath monitoring notification, using
// &struct iwl_datapath_monitor_notif
//
    MONITOR_NOTIF = 0xF4,

//
// @RX_NO_DATA_NOTIF: &struct iwl_rx_no_data or &struct iwl_rx_no_data_ver_3
//
    RX_NO_DATA_NOTIF = 0xF5,

//
// @THERMAL_DUAL_CHAIN_REQUEST: firmware request for SMPS mode,
// &struct iwl_thermal_dual_chain_request
//
    THERMAL_DUAL_CHAIN_REQUEST = 0xF6,

//
// @TLC_MNG_UPDATE_NOTIF: &struct iwl_tlc_update_notif
//
    TLC_MNG_UPDATE_NOTIF = 0xF7,

//
// @BEACON_FILTER_IN_NOTIF: &struct iwl_beacon_filter_notif
//
    BEACON_FILTER_IN_NOTIF = 0xF8,

//
// @PHY_AIR_SNIFFER_NOTIF: &struct iwl_rx_phy_air_sniffer_ntfy
//
    PHY_AIR_SNIFFER_NOTIF = 0xF9,

//
// @STA_PM_NOTIF: &struct iwl_mvm_pm_state_notification
//
    STA_PM_NOTIF = 0xFD,

//
// @MU_GROUP_MGMT_NOTIF: &struct iwl_mu_group_mgmt_notif
//
    MU_GROUP_MGMT_NOTIF = 0xFE,

//
// @RX_QUEUES_NOTIFICATION: &struct iwl_rxq_sync_notification
//
    RX_QUEUES_NOTIFICATION = 0xFF,
}

//
// struct iwl_mu_group_mgmt_cmd - VHT MU-MIMO group configuration
//
// @reserved: reserved
// @membership_status: a bitmap of MU groups
// @user_position:the position of station in a group. If the station is in the
// group then bits (group * 2) is the position -1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mu_group_mgmt_cmd {
    pub reserved: __le32,
    pub membership_status: [__le32; 2],
    pub user_position: [__le32; 4],
    pub /: *mut *mut } __packed; / MU_GROUP_ID_MNG_TABLE_API_S_VER_1,
//
// struct iwl_mu_group_mgmt_notif - VHT MU-MIMO group id notification
//
// @membership_status: a bitmap of MU groups
// @user_position: the position of station in a group. If the station is in the
// group then bits (group * 2) is the position -1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mu_group_mgmt_notif {
    pub membership_status: [__le32; 2],
    pub user_position: [__le32; 4],
    pub /: *mut *mut } __packed; / MU_GROUP_MNG_NTFY_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_channel_estimation_flags {
    IWL_CHANNEL_ESTIMATION_ENABLE	= BIT(0),
    IWL_CHANNEL_ESTIMATION_TIMER	= BIT(1),
    IWL_CHANNEL_ESTIMATION_COUNTER	= BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_time_sync_protocol_type {
    IWL_TIME_SYNC_PROTOCOL_TM	= BIT(0),
    IWL_TIME_SYNC_PROTOCOL_FTM	= BIT(1),
}

//
// struct iwl_time_sync_cfg_cmd - TM/FTM time sync measurement configuration
//
// @protocols: The type of frames to raise notifications for. A bitmap
// of @iwl_time_sync_protocol_type
// @peer_addr: peer address with which TM/FTM measurements are required
// @reserved: for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_sync_cfg_cmd {
    pub protocols: __le32,
    pub peer_addr: [u8; ETH_ALEN],
    pub reserved: [u8; 2],
    pub /: *mut *mut } __packed; / WNM_80211V_TIMING_MEASUREMENT_CONFIG_CMD_API_S_VER_1,
//
// enum iwl_synced_time_operation - PTM request options
//
// @IWL_SYNCED_TIME_OPERATION_READ_ARTB: read only the ARTB time
// @IWL_SYNCED_TIME_OPERATION_READ_GP2: read only the GP2 time
// @IWL_SYNCED_TIME_OPERATION_READ_BOTH: latch the ARTB and GP2 clocks and
// provide timestamps from both clocks for the same time point
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_synced_time_operation {
    IWL_SYNCED_TIME_OPERATION_READ_ARTB = 1,
    IWL_SYNCED_TIME_OPERATION_READ_GP2,
    IWL_SYNCED_TIME_OPERATION_READ_BOTH,
}

//
// struct iwl_synced_time_cmd - request synced GP2/ARTB timestamps
//
// @operation: one of &enum iwl_synced_time_operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_synced_time_cmd {
    pub operation: __le32,
    pub /: *mut *mut } __packed; / WNM_80211V_TIMING_CMD_API_S_VER_1,
//
// struct iwl_synced_time_rsp - response to iwl_synced_time_cmd
//
// @operation: one of &enum iwl_synced_time_operation
// @platform_timestamp_hi: high DWORD of the ARTB clock timestamp in nanoseconds
// @platform_timestamp_lo: low DWORD of the ARTB clock timestamp in nanoseconds
// @gp2_timestamp_hi: high DWORD of the GP2 clock timestamp in 10's of
// nanoseconds
// @gp2_timestamp_lo: low DWORD of the GP2 clock timestamp in 10's of
// nanoseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_synced_time_rsp {
    pub operation: __le32,
    pub platform_timestamp_hi: __le32,
    pub platform_timestamp_lo: __le32,
    pub gp2_timestamp_hi: __le32,
    pub gp2_timestamp_lo: __le32,
    pub /: *mut *mut } __packed; / WNM_80211V_TIMING_RSP_API_S_VER_1,
// PTP_CTX_MAX_DATA_SIZE_IN_API_D_VER_1
pub const PTP_CTX_MAX_DATA_SIZE: c_int = 128;
//
// struct iwl_time_msmt_ptp_ctx - Vendor specific element
// to allow a space for flexibility for the userspace App
//
// @ftm: FTM specific vendor element
// @ftm.element_id: element id of vendor specific ie
// @ftm.length: length of vendor specific ie
// @ftm.reserved: for alignment
// @ftm.data: vendor specific data blob
// @tm: TM specific vendor element
// @tm.element_id: element id of vendor specific ie
// @tm.length: length of vendor specific ie
// @tm.data: vendor specific data blob
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_msmt_ptp_ctx {
// Differentiate between FTM and TM specific Vendor elements
    pub element_id: u8,
    pub length: u8,
    pub reserved: __le16,
    pub data: [u8; PTP_CTX_MAX_DATA_SIZE],
    pub ftm: },
    pub element_id: u8,
    pub length: u8,
    pub data: [u8; PTP_CTX_MAX_DATA_SIZE],
    pub tm: },
}

//
// struct iwl_time_msmt_notify - Time Sync measurement notification
// for TM/FTM, along with additional meta data.
//
// @peer_addr: peer address
// @reserved: for alignment
// @dialog_token: measurement flow dialog token number
// @followup_dialog_token: Measurement flow previous dialog token number
// @t1_hi: high dword of t1-time of the Tx'ed action frame departure on
// sender side in units of 10 nano seconds
// @t1_lo: low dword of t1-time of the Tx'ed action frame departure on
// sender side in units of 10 nano seconds
// @t1_max_err: maximum t1-time error in units of 10 nano seconds
// @t4_hi: high dword of t4-time of the Rx'ed action frame's Ack arrival on
// sender side in units of 10 nano seconds
// @t4_lo: low dword of t4-time of the Rx'ed action frame's Ack arrival on
// sender side in units of 10 nano seconds
// @t4_max_err: maximum t4-time error in units of 10 nano seconds
// @t2_hi: high dword of t2-time of the Rx'ed action frame arrival on
// receiver side in units of 10 nano seconds
// @t2_lo: low dword of t2-time of the Rx'ed action frame arrival on
// receiver side in units of 10 nano seconds
// @t2_max_err: maximum t2-time error in units of 10 nano seconds
// @t3_hi: high dword of t3-time of the Tx'ed action frame's Ack departure on
// receiver side in units of 10 nano seconds
// @t3_lo: low dword of t3-time of the Tx'ed action frame's Ack departure on
// receiver side in units of 10 nano seconds
// @t3_max_err: maximum t3-time error in units of 10 nano seconds
// @ptp: vendor specific information element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_msmt_notify {
    pub peer_addr: [u8; ETH_ALEN],
    pub reserved: [u8; 2],
    pub dialog_token: __le32,
    pub followup_dialog_token: __le32,
    pub t1_hi: __le32,
    pub t1_lo: __le32,
    pub t1_max_err: __le32,
    pub t4_hi: __le32,
    pub t4_lo: __le32,
    pub t4_max_err: __le32,
    pub t2_hi: __le32,
    pub t2_lo: __le32,
    pub t2_max_err: __le32,
    pub t3_hi: __le32,
    pub t3_lo: __le32,
    pub t3_max_err: __le32,
    pub ptp: iwl_time_msmt_ptp_ctx,
    pub /: *mut *mut } __packed; / WNM_80211V_TIMING_MEASUREMENT_NTFY_API_S_VER_1,
//
// struct iwl_time_msmt_cfm_notify - Time Sync measurement confirmation
// notification for TM/FTM. Sent on receipt of 802.11 Ack from peer for the
// Tx'ed TM/FTM measurement action frame.
//
// @peer_addr: peer address
// @reserved: for alignment
// @dialog_token: measurement flow dialog token number
// @t1_hi: high dword of t1-time of the Tx'ed action frame departure on
// sender side in units of 10 nano seconds
// @t1_lo: low dword of t1-time of the Tx'ed action frame departure on
// sender side in units of 10 nano seconds
// @t1_max_err: maximum t1-time error in units of 10 nano seconds
// @t4_hi: high dword of t4-time of the Rx'ed action frame's Ack arrival on
// sender side in units of 10 nano seconds
// @t4_lo: low dword of t4-time of the Rx'ed action frame's Ack arrival on
// sender side in units of 10 nano seconds
// @t4_max_err: maximum t4-time error in units of 10 nano seconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_msmt_cfm_notify {
    pub peer_addr: [u8; ETH_ALEN],
    pub reserved: [u8; 2],
    pub dialog_token: __le32,
    pub t1_hi: __le32,
    pub t1_lo: __le32,
    pub t1_max_err: __le32,
    pub t4_hi: __le32,
    pub t4_lo: __le32,
    pub t4_max_err: __le32,
    pub /: *mut *mut } __packed; / WNM_80211V_TIMING_MEASUREMENT_CONFIRM_NTFY_API_S_VER_1,
//
// struct iwl_channel_estimation_cfg - channel estimation reporting config
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_channel_estimation_cfg {
//
// @flags: flags, see &enum iwl_channel_estimation_flags
//
    pub flags: __le32,
//
// @timer: if enabled via flags, automatically disable after this many
// microseconds
//
    pub timer: __le32,
//
// @count: if enabled via flags, automatically disable after this many
// frames with channel estimation matrix were captured
//
    pub count: __le32,
//
// @rate_n_flags_mask: only try to record the channel estimation matrix
// if the rate_n_flags value for the received frame (let's call
// that rx_rnf) matches the mask/value given here like this:
// (rx_rnf & rate_n_flags_mask) == rate_n_flags_val.
//
    pub rate_n_flags_mask: __le32,
//
// @rate_n_flags_val: see @rate_n_flags_mask
//
    pub rate_n_flags_val: __le32,
//
// @reserved: reserved (for alignment)
//
    pub reserved: __le32,
//
// @frame_types: bitmap of frame types to capture, the received frame's
// subtype|type takes 6 bits in the frame and the corresponding bit
// in this field must be set to 1 to capture channel estimation for
// that frame type. Set to all-ones to enable capturing for all
// frame types.
//
    pub frame_types: __le64,
    pub /: *mut *mut } __packed; / CHEST_COLLECTOR_FILTER_CMD_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_datapath_monitor_notif_type {
    IWL_DP_MON_NOTIF_TYPE_EXT_CCA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_datapath_monitor_notif {
    pub type: __le32,
    pub link_id: u8,
    pub reserved: [u8; 3],
    pub /: *mut *mut } __packed; / MONITOR_NTF_API_S_VER_1,
//
// enum iwl_thermal_dual_chain_req_events - firmware SMPS request event
// @THERMAL_DUAL_CHAIN_REQ_ENABLE: (re-)enable dual-chain operation
// (subject to other constraints)
// @THERMAL_DUAL_CHAIN_REQ_DISABLE: disable dual-chain operation
// (static SMPS)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_thermal_dual_chain_req_events {
    THERMAL_DUAL_CHAIN_REQ_ENABLE,
    THERMAL_DUAL_CHAIN_REQ_DISABLE,
}

//
// struct iwl_thermal_dual_chain_request - SMPS request
// @event: the type of request, see &enum iwl_thermal_dual_chain_req_events
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_thermal_dual_chain_request {
    pub event: __le32,
    pub /: *mut *mut } __packed; / THERMAL_DUAL_CHAIN_DISABLE_REQ_NTFY_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_rlc_chain_info {
    IWL_RLC_CHAIN_INFO_DRIVER_FORCE		= BIT(0),
    IWL_RLC_CHAIN_INFO_VALID		= 0x000e,
    IWL_RLC_CHAIN_INFO_FORCE		= 0x0070,
    IWL_RLC_CHAIN_INFO_FORCE_MIMO		= 0x0380,
    IWL_RLC_CHAIN_INFO_COUNT		= 0x0c00,
    IWL_RLC_CHAIN_INFO_MIMO_COUNT		= 0x3000,
}

//
// struct iwl_rlc_properties - RLC properties
// @rx_chain_info: RX chain info, &enum iwl_rlc_chain_info
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rlc_properties {
    pub rx_chain_info: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / RLC_PROPERTIES_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sad_mode {
    IWL_SAD_MODE_ENABLED		= BIT(0),
    IWL_SAD_MODE_DEFAULT_ANT_MSK	= 0x6,
    IWL_SAD_MODE_DEFAULT_ANT_FW	= 0x0,
    IWL_SAD_MODE_DEFAULT_ANT_A	= 0x2,
    IWL_SAD_MODE_DEFAULT_ANT_B	= 0x4,
}

//
// struct iwl_sad_properties - SAD properties
// @chain_a_sad_mode: chain A SAD mode, &enum iwl_sad_mode
// @chain_b_sad_mode: chain B SAD mode, &enum iwl_sad_mode
// @mac_id: MAC index
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sad_properties {
    pub chain_a_sad_mode: __le32,
    pub chain_b_sad_mode: __le32,
    pub mac_id: __le32,
    pub reserved: __le32,
    pub __packed: },
//
// struct iwl_rlc_config_cmd - RLC configuration
// @phy_id: PHY index
// @rlc: RLC properties, &struct iwl_rlc_properties
// @sad: SAD (single antenna diversity) options, &struct iwl_sad_properties
// @flags: flags (unused)
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rlc_config_cmd {
    pub phy_id: __le32,
    pub rlc: iwl_rlc_properties,
    pub sad: iwl_sad_properties,
    pub flags: u8,
    pub reserved: [u8; 3],
    pub /: *mut *mut } __packed; / RLC_CONFIG_CMD_API_S_VER_2,

//
// enum iwl_rx_baid_action - BAID allocation/config action
// @IWL_RX_BAID_ACTION_ADD: add a new BAID session
// @IWL_RX_BAID_ACTION_MODIFY: modify the BAID session
// @IWL_RX_BAID_ACTION_REMOVE: remove the BAID session
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_rx_baid_action {
    IWL_RX_BAID_ACTION_ADD,
    IWL_RX_BAID_ACTION_MODIFY,
    IWL_RX_BAID_ACTION_REMOVE,
}

//
// struct iwl_rx_baid_cfg_cmd_alloc - BAID allocation data
// @sta_id_mask: station ID mask
// @tid: the TID for this session
// @reserved: reserved
// @ssn: the starting sequence number
// @win_size: RX BA session window size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_baid_cfg_cmd_alloc {
    pub sta_id_mask: __le32,
    pub tid: u8,
    pub reserved: [u8; 3],
    pub ssn: __le16,
    pub win_size: __le16,
    pub /: *mut *mut } __packed; / RX_BAID_ALLOCATION_ADD_CMD_API_S_VER_1,
//
// struct iwl_rx_baid_cfg_cmd_modify - BAID modification data
// @old_sta_id_mask: old station ID mask
// @new_sta_id_mask: new station ID mask
// @tid: TID of the BAID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_baid_cfg_cmd_modify {
    pub old_sta_id_mask: __le32,
    pub new_sta_id_mask: __le32,
    pub tid: __le32,
    pub /: *mut *mut } __packed; / RX_BAID_ALLOCATION_MODIFY_CMD_API_S_VER_2,
//
// struct iwl_rx_baid_cfg_cmd_remove_v1 - BAID removal data
// @baid: the BAID to remove
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_baid_cfg_cmd_remove_v1 {
    pub baid: __le32,
    pub /: *mut *mut } __packed; / RX_BAID_ALLOCATION_REMOVE_CMD_API_S_VER_1,
//
// struct iwl_rx_baid_cfg_cmd_remove - BAID removal data
// @sta_id_mask: the station mask of the BAID to remove
// @tid: the TID of the BAID to remove
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_baid_cfg_cmd_remove {
    pub sta_id_mask: __le32,
    pub tid: __le32,
    pub /: *mut *mut } __packed; / RX_BAID_ALLOCATION_REMOVE_CMD_API_S_VER_2,
//
// struct iwl_rx_baid_cfg_cmd - BAID allocation/config command
// @action: the action, from &enum iwl_rx_baid_action
// @alloc: allocation data
// @modify: modify data
// @remove_v1: remove data (version 1)
// @remove: remove data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_baid_cfg_cmd {
    pub action: __le32,
    pub alloc: iwl_rx_baid_cfg_cmd_alloc,
    pub modify: iwl_rx_baid_cfg_cmd_modify,
    pub remove_v1: iwl_rx_baid_cfg_cmd_remove_v1,
    pub remove: iwl_rx_baid_cfg_cmd_remove,
}

//
// struct iwl_rx_baid_cfg_resp - BAID allocation response
// @baid: the allocated BAID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_baid_cfg_resp {
    pub baid: __le32,
}

//
// enum iwl_scd_queue_cfg_operation - scheduler queue operation
// @IWL_SCD_QUEUE_ADD: allocate a new queue
// @IWL_SCD_QUEUE_REMOVE: remove a queue
// @IWL_SCD_QUEUE_MODIFY: modify a queue
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scd_queue_cfg_operation {
    IWL_SCD_QUEUE_ADD = 0,
    IWL_SCD_QUEUE_REMOVE = 1,
    IWL_SCD_QUEUE_MODIFY = 2,
}

//
// struct iwl_scd_queue_cfg_cmd - scheduler queue allocation command
// @operation: the operation, see &enum iwl_scd_queue_cfg_operation
// @u: union depending on command usage
// @u.add.sta_mask: station mask
// @u.add.tid: TID
// @u.add.reserved: reserved
// @u.add.flags: flags from &enum iwl_tx_queue_cfg_actions, except
// %TX_QUEUE_CFG_ENABLE_QUEUE is not valid
// @u.add.cb_size: size code
// @u.add.bc_dram_addr: byte-count table IOVA
// @u.add.tfdq_dram_addr: TFD queue IOVA
// @u.remove.sta_mask: station mask of queue to remove
// @u.remove.tid: TID of queue to remove
// @u.modify.old_sta_mask: old station mask for modify
// @u.modify.tid: TID of queue to modify
// @u.modify.new_sta_mask: new station mask for modify
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scd_queue_cfg_cmd {
    pub operation: __le32,
    pub sta_mask: __le32,
    pub tid: u8,
    pub reserved: [u8; 3],
    pub flags: __le32,
    pub cb_size: __le32,
    pub bc_dram_addr: __le64,
    pub tfdq_dram_addr: __le64,
    pub /: *mut *mut } __packed add; / TX_QUEUE_CFG_CMD_ADD_API_S_VER_1,
    pub sta_mask: __le32,
    pub tid: __le32,
    pub /: *mut *mut } __packed remove; / TX_QUEUE_CFG_CMD_REMOVE_API_S_VER_1,
    pub old_sta_mask: __le32,
    pub tid: __le32,
    pub new_sta_mask: __le32,
    pub /: *mut *mut } __packed modify; / TX_QUEUE_CFG_CMD_MODIFY_API_S_VER_1,
    pub /: *mut *mut } __packed u; / TX_QUEUE_CFG_CMD_OPERATION_API_U_VER_1,
    pub /: *mut *mut } __packed; / TX_QUEUE_CFG_CMD_API_S_VER_3,
//
// enum iwl_sec_key_flags - security key command key flags
// @IWL_SEC_KEY_FLAG_CIPHER_MASK: cipher mask
// @IWL_SEC_KEY_FLAG_CIPHER_WEP: WEP cipher
// @IWL_SEC_KEY_FLAG_CIPHER_CCMP: CCMP/CMAC cipher
// @IWL_SEC_KEY_FLAG_CIPHER_TKIP: TKIP cipher
// @IWL_SEC_KEY_FLAG_CIPHER_GCMP: GCMP/GMAC cipher
// @IWL_SEC_KEY_FLAG_NO_TX: don't install for TX
// @IWL_SEC_KEY_FLAG_KEY_SIZE: large key size (WEP-104, GCMP-256, GMAC-256)
// @IWL_SEC_KEY_FLAG_MFP: MFP is in used for this key
// @IWL_SEC_KEY_FLAG_MCAST_KEY: this is a multicast key
// @IWL_SEC_KEY_FLAG_SPP_AMSDU: SPP A-MSDU should be used
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sec_key_flags {
    IWL_SEC_KEY_FLAG_CIPHER_MASK	= 0x07,
    IWL_SEC_KEY_FLAG_CIPHER_WEP	= 0x01,
    IWL_SEC_KEY_FLAG_CIPHER_CCMP	= 0x02,
    IWL_SEC_KEY_FLAG_CIPHER_TKIP	= 0x03,
    IWL_SEC_KEY_FLAG_CIPHER_GCMP	= 0x05,
    IWL_SEC_KEY_FLAG_NO_TX		= 0x08,
    IWL_SEC_KEY_FLAG_KEY_SIZE	= 0x10,
    IWL_SEC_KEY_FLAG_MFP		= 0x20,
    IWL_SEC_KEY_FLAG_MCAST_KEY	= 0x40,
    IWL_SEC_KEY_FLAG_SPP_AMSDU	= 0x80,
}

pub const IWL_SEC_WEP_KEY_OFFSET: c_int = 3;
//
// struct iwl_sec_key_cmd - security key command
// @action: action from &enum iwl_ctxt_action
// @u: union depending on command type
// @u.add.sta_mask: station mask for the new key
// @u.add.key_id: key ID (0-7) for the new key
// @u.add.key_flags: key flags per &enum iwl_sec_key_flags
// @u.add.key: key material. WEP keys should start from &IWL_SEC_WEP_KEY_OFFSET.
// @u.add.tkip_mic_rx_key: TKIP MIC RX key
// @u.add.tkip_mic_tx_key: TKIP MIC TX key
// @u.add.rx_seq: RX sequence counter value
// @u.add.tx_seq: TX sequence counter value
// @u.modify.old_sta_mask: old station mask
// @u.modify.new_sta_mask: new station mask
// @u.modify.key_id: key ID
// @u.modify.key_flags: new key flags
// @u.remove.sta_mask: station mask
// @u.remove.key_id: key ID
// @u.remove.key_flags: key flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sec_key_cmd {
    pub action: __le32,
    pub sta_mask: __le32,
    pub key_id: __le32,
    pub key_flags: __le32,
    pub key: [u8; 32],
    pub tkip_mic_rx_key: [u8; 8],
    pub tkip_mic_tx_key: [u8; 8],
    pub rx_seq: __le64,
    pub tx_seq: __le64,
    pub /: *mut *mut } __packed add; / SEC_KEY_ADD_CMD_API_S_VER_1,
    pub old_sta_mask: __le32,
    pub new_sta_mask: __le32,
    pub key_id: __le32,
    pub key_flags: __le32,
    pub /: *mut *mut } __packed modify; / SEC_KEY_MODIFY_CMD_API_S_VER_1,
    pub sta_mask: __le32,
    pub key_id: __le32,
    pub key_flags: __le32,
    pub /: *mut *mut } __packed remove; / SEC_KEY_REMOVE_CMD_API_S_VER_1,
    pub /: *mut *mut } __packed u; / SEC_KEY_OPERATION_API_U_VER_1,
    pub /: *mut *mut } __packed; / SEC_KEY_CMD_API_S_VER_1,
