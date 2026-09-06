//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/location.h
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
// Copyright (C) 2015-2017 Intel Deutschland GmbH
// Copyright (C) 2018-2022 Intel Corporation
// Copyright (C) 2024-2026 Intel Corporation
//

// Macro flag: #define __iwl_fw_api_location_h__

//
// enum iwl_location_subcmd_ids - location group command IDs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_location_subcmd_ids {
//
// @TOF_RANGE_REQ_CMD: TOF ranging request,
// uses one of &struct iwl_tof_range_req_cmd_v5,
// &struct iwl_tof_range_req_cmd_v7,
// &struct iwl_tof_range_req_cmd_v8,
// &struct iwl_tof_range_req_cmd_v9,
// &struct iwl_tof_range_req_cmd_v11,
// &struct iwl_tof_range_req_cmd_v7
//
    TOF_RANGE_REQ_CMD = 0x0,
//
// @TOF_CONFIG_CMD: TOF configuration, uses &struct iwl_tof_config_cmd
//
    TOF_CONFIG_CMD = 0x1,
//
// @TOF_RANGE_ABORT_CMD: abort ongoing ranging, uses
// &struct iwl_tof_range_abort_cmd
//
    TOF_RANGE_ABORT_CMD = 0x2,
//
// @TOF_RANGE_REQ_EXT_CMD: TOF extended ranging config,
// uses &struct iwl_tof_range_req_ext_cmd
//
    TOF_RANGE_REQ_EXT_CMD = 0x3,
//
// @TOF_RESPONDER_CONFIG_CMD: FTM responder configuration, one of
// &struct iwl_tof_responder_config_cmd_v6,
// &struct iwl_tof_responder_config_cmd_v7,
// &struct iwl_tof_responder_config_cmd_v8 or
// &struct iwl_tof_responder_config_cmd_v9
//
    TOF_RESPONDER_CONFIG_CMD = 0x4,
//
// @TOF_RESPONDER_DYN_CONFIG_CMD: FTM dynamic configuration,
// uses &struct iwl_tof_responder_dyn_config_cmd
//
    TOF_RESPONDER_DYN_CONFIG_CMD = 0x5,
//
// @CSI_HEADER_NOTIFICATION: CSI header
//
    CSI_HEADER_NOTIFICATION = 0xFA,
//
// @CSI_CHUNKS_NOTIFICATION: CSI chunk,
// uses &struct iwl_csi_chunk_notification
//
    CSI_CHUNKS_NOTIFICATION = 0xFB,
//
// @TOF_LC_NOTIF: used for LCI/civic location, contains just
// the action frame
//
    TOF_LC_NOTIF = 0xFC,
//
// @TOF_RESPONDER_STATS: FTM responder statistics notification,
// uses &struct iwl_ftm_responder_stats
//
    TOF_RESPONDER_STATS = 0xFD,
//
// @TOF_MCSI_DEBUG_NOTIF: MCSI debug notification, uses
// &struct iwl_tof_mcsi_notif
//
    TOF_MCSI_DEBUG_NOTIF = 0xFE,
//
// @TOF_RANGE_RESPONSE_NOTIF: ranging response, using one of
// &struct iwl_tof_range_rsp_ntfy_v5,
// &struct iwl_tof_range_rsp_ntfy_v6,
// &struct iwl_tof_range_rsp_ntfy_v7,
// &struct iwl_tof_range_rsp_ntfy_v9 or
// &struct iwl_tof_range_rsp_ntfy
//
    TOF_RANGE_RESPONSE_NOTIF = 0xFF,
}

//
// enum iwl_location_frame_format - location frame formats
// @IWL_LOCATION_FRAME_FORMAT_LEGACY: legacy
// @IWL_LOCATION_FRAME_FORMAT_HT: HT
// @IWL_LOCATION_FRAME_FORMAT_VHT: VHT
// @IWL_LOCATION_FRAME_FORMAT_HE: HE
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_location_frame_format {
    IWL_LOCATION_FRAME_FORMAT_LEGACY,
    IWL_LOCATION_FRAME_FORMAT_HT,
    IWL_LOCATION_FRAME_FORMAT_VHT,
    IWL_LOCATION_FRAME_FORMAT_HE,
}

//
// enum iwl_location_bw - location bandwidth selection
// @IWL_LOCATION_BW_20MHZ: 20 MHz
// @IWL_LOCATION_BW_40MHZ: 40 MHz
// @IWL_LOCATION_BW_80MHZ: 80 MHz
// @IWL_LOCATION_BW_160MHZ: 160 MHz
// @IWL_LOCATION_BW_320MHZ: 320 MHz
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_location_bw {
    IWL_LOCATION_BW_20MHZ,
    IWL_LOCATION_BW_40MHZ,
    IWL_LOCATION_BW_80MHZ,
    IWL_LOCATION_BW_160MHZ,
    IWL_LOCATION_BW_320MHZ,
}

//
// enum iwl_location_format_bw - format/BW encoding
// @IWL_LOCATION_FMT_BW_FORMAT: &enum iwl_location_frame_format
// @IWL_LOCATION_FMT_BW_BANDWIDTH: &enum iwl_location_bw
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_location_format_bw {
    IWL_LOCATION_FMT_BW_FORMAT	= 0x0f,
    IWL_LOCATION_FMT_BW_BANDWIDTH	= 0xf0,
}

//
// struct iwl_tof_config_cmd - ToF configuration
// @tof_disabled: indicates if ToF is disabled (or not)
// @one_sided_disabled: indicates if one-sided is disabled (or not)
// @is_debug_mode: indiciates if debug mode is active
// @is_buf_required: indicates if channel estimation buffer is required
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_config_cmd {
    pub tof_disabled: u8,
    pub one_sided_disabled: u8,
    pub is_debug_mode: u8,
    pub is_buf_required: u8,
    pub __packed: },
//
// enum iwl_tof_bandwidth - values for iwl_tof_range_req_ap_entry.bandwidth
// @IWL_TOF_BW_20_LEGACY: 20 MHz non-HT
// @IWL_TOF_BW_20_HT: 20 MHz HT
// @IWL_TOF_BW_40: 40 MHz
// @IWL_TOF_BW_80: 80 MHz
// @IWL_TOF_BW_160: 160 MHz
// @IWL_TOF_BW_NUM: number of tof bandwidths
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_bandwidth {
    IWL_TOF_BW_20_LEGACY,
    IWL_TOF_BW_20_HT,
    IWL_TOF_BW_40,
    IWL_TOF_BW_80,
    IWL_TOF_BW_160,
    IWL_TOF_BW_NUM,
}

//
// enum iwl_tof_algo_type - Algorithym type for range measurement request
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_algo_type {
    IWL_TOF_ALGO_TYPE_MAX_LIKE	= 0,
    IWL_TOF_ALGO_TYPE_LINEAR_REG	= 1,
    IWL_TOF_ALGO_TYPE_FFT		= 2,

// Keep last
    IWL_TOF_ALGO_TYPE_INVALID,
}

//
// enum iwl_tof_mcsi_ntfy - Enable/Disable MCSI notifications
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_mcsi_enable {
    IWL_TOF_MCSI_DISABLED = 0,
    IWL_TOF_MCSI_ENABLED = 1,
}

//
// enum iwl_tof_responder_cmd_valid_field - valid fields in the responder cfg
// @IWL_TOF_RESPONDER_CMD_VALID_CHAN_INFO: channel info is valid
// @IWL_TOF_RESPONDER_CMD_VALID_TOA_OFFSET: ToA offset is valid
// @IWL_TOF_RESPONDER_CMD_VALID_COMMON_CALIB: common calibration mode is valid
// @IWL_TOF_RESPONDER_CMD_VALID_SPECIFIC_CALIB: spefici calibration mode is
// valid
// @IWL_TOF_RESPONDER_CMD_VALID_BSSID: BSSID is valid
// @IWL_TOF_RESPONDER_CMD_VALID_TX_ANT: TX antenna is valid
// @IWL_TOF_RESPONDER_CMD_VALID_ALGO_TYPE: algorithm type is valid
// @IWL_TOF_RESPONDER_CMD_VALID_NON_ASAP_SUPPORT: non-ASAP support is valid
// @IWL_TOF_RESPONDER_CMD_VALID_STATISTICS_REPORT_SUPPORT: statistics report
// support is valid
// @IWL_TOF_RESPONDER_CMD_VALID_MCSI_NOTIF_SUPPORT: MCSI notification support
// is valid
// @IWL_TOF_RESPONDER_CMD_VALID_FAST_ALGO_SUPPORT: fast algorithm support
// is valid
// @IWL_TOF_RESPONDER_CMD_VALID_RETRY_ON_ALGO_FAIL: retry on algorithm failure
// is valid
// @IWL_TOF_RESPONDER_CMD_VALID_STA_ID: station ID is valid
// @IWL_TOF_RESPONDER_CMD_VALID_NDP_SUPPORT: enable/disable NDP ranging support
// is valid
// @IWL_TOF_RESPONDER_CMD_VALID_NDP_PARAMS: NDP parameters are valid
// @IWL_TOF_RESPONDER_CMD_VALID_LMR_FEEDBACK: LMR feedback support is valid
// @IWL_TOF_RESPONDER_CMD_VALID_SESSION_ID: session id flag is valid
// @IWL_TOF_RESPONDER_CMD_VALID_BSS_COLOR: the bss_color field is valid
// @IWL_TOF_RESPONDER_CMD_VALID_MIN_MAX_TIME_BETWEEN_MSR: the
// min_time_between_msr and max_time_between_msr fields are valid
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_responder_cmd_valid_field {
    IWL_TOF_RESPONDER_CMD_VALID_CHAN_INFO = BIT(0),
    IWL_TOF_RESPONDER_CMD_VALID_TOA_OFFSET = BIT(1),
    IWL_TOF_RESPONDER_CMD_VALID_COMMON_CALIB = BIT(2),
    IWL_TOF_RESPONDER_CMD_VALID_SPECIFIC_CALIB = BIT(3),
    IWL_TOF_RESPONDER_CMD_VALID_BSSID = BIT(4),
    IWL_TOF_RESPONDER_CMD_VALID_TX_ANT = BIT(5),
    IWL_TOF_RESPONDER_CMD_VALID_ALGO_TYPE = BIT(6),
    IWL_TOF_RESPONDER_CMD_VALID_NON_ASAP_SUPPORT = BIT(7),
    IWL_TOF_RESPONDER_CMD_VALID_STATISTICS_REPORT_SUPPORT = BIT(8),
    IWL_TOF_RESPONDER_CMD_VALID_MCSI_NOTIF_SUPPORT = BIT(9),
    IWL_TOF_RESPONDER_CMD_VALID_FAST_ALGO_SUPPORT = BIT(10),
    IWL_TOF_RESPONDER_CMD_VALID_RETRY_ON_ALGO_FAIL = BIT(11),
    IWL_TOF_RESPONDER_CMD_VALID_STA_ID = BIT(12),
    IWL_TOF_RESPONDER_CMD_VALID_NDP_SUPPORT = BIT(22),
    IWL_TOF_RESPONDER_CMD_VALID_NDP_PARAMS = BIT(23),
    IWL_TOF_RESPONDER_CMD_VALID_LMR_FEEDBACK = BIT(24),
    IWL_TOF_RESPONDER_CMD_VALID_SESSION_ID = BIT(25),
    IWL_TOF_RESPONDER_CMD_VALID_BSS_COLOR = BIT(26),
    IWL_TOF_RESPONDER_CMD_VALID_MIN_MAX_TIME_BETWEEN_MSR = BIT(27),
}

//
// enum iwl_tof_responder_cfg_flags - responder configuration flags
// @IWL_TOF_RESPONDER_FLAGS_NON_ASAP_SUPPORT: non-ASAP support
// @IWL_TOF_RESPONDER_FLAGS_REPORT_STATISTICS: report statistics
// @IWL_TOF_RESPONDER_FLAGS_REPORT_MCSI: report MCSI
// @IWL_TOF_RESPONDER_FLAGS_ALGO_TYPE: algorithm type
// @IWL_TOF_RESPONDER_FLAGS_TOA_OFFSET_MODE: ToA offset mode
// @IWL_TOF_RESPONDER_FLAGS_COMMON_CALIB_MODE: common calibration mode
// @IWL_TOF_RESPONDER_FLAGS_SPECIFIC_CALIB_MODE: specific calibration mode
// @IWL_TOF_RESPONDER_FLAGS_FAST_ALGO_SUPPORT: fast algorithm support
// @IWL_TOF_RESPONDER_FLAGS_RETRY_ON_ALGO_FAIL: retry on algorithm fail
// @IWL_TOF_RESPONDER_FLAGS_FTM_TX_ANT: TX antenna mask
// @IWL_TOF_RESPONDER_FLAGS_NDP_SUPPORT: support NDP ranging
// @IWL_TOF_RESPONDER_FLAGS_LMR_FEEDBACK: request for LMR feedback if the
// initiator supports it
// @IWL_TOF_RESPONDER_FLAGS_SESSION_ID: send the session id in the initial FTM
// frame.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_responder_cfg_flags {
    IWL_TOF_RESPONDER_FLAGS_NON_ASAP_SUPPORT = BIT(0),
    IWL_TOF_RESPONDER_FLAGS_REPORT_STATISTICS = BIT(1),
    IWL_TOF_RESPONDER_FLAGS_REPORT_MCSI = BIT(2),
    IWL_TOF_RESPONDER_FLAGS_ALGO_TYPE = BIT(3) | BIT(4) | BIT(5),
    IWL_TOF_RESPONDER_FLAGS_TOA_OFFSET_MODE = BIT(6),
    IWL_TOF_RESPONDER_FLAGS_COMMON_CALIB_MODE = BIT(7),
    IWL_TOF_RESPONDER_FLAGS_SPECIFIC_CALIB_MODE = BIT(8),
    IWL_TOF_RESPONDER_FLAGS_FAST_ALGO_SUPPORT = BIT(9),
    IWL_TOF_RESPONDER_FLAGS_RETRY_ON_ALGO_FAIL = BIT(10),
    IWL_TOF_RESPONDER_FLAGS_FTM_TX_ANT = RATE_MCS_ANT_AB_MSK,
    IWL_TOF_RESPONDER_FLAGS_NDP_SUPPORT = BIT(24),
    IWL_TOF_RESPONDER_FLAGS_LMR_FEEDBACK = BIT(25),
    IWL_TOF_RESPONDER_FLAGS_SESSION_ID = BIT(27),
}

//
// struct iwl_tof_responder_config_cmd_v6 - ToF AP mode (for debug)
// @cmd_valid_fields: &iwl_tof_responder_cmd_valid_field
// @responder_cfg_flags: &iwl_tof_responder_cfg_flags
// @bandwidth: current AP Bandwidth: &enum iwl_tof_bandwidth
// @rate: current AP rate
// @channel_num: current AP Channel
// @ctrl_ch_position: coding of the control channel position relative to
// the center frequency, see iwl_mvm_get_ctrl_pos()
// @sta_id: index of the AP STA when in AP mode
// @reserved1: reserved
// @toa_offset: Artificial addition [pSec] for the ToA - to be used for debug
// purposes, simulating station movement by adding various values
// to this field
// @common_calib: XVT: common calibration value
// @specific_calib: XVT: specific calibration value
// @bssid: Current AP BSSID
// @reserved2: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_responder_config_cmd_v6 {
    pub cmd_valid_fields: __le32,
    pub responder_cfg_flags: __le32,
    pub bandwidth: u8,
    pub rate: u8,
    pub channel_num: u8,
    pub ctrl_ch_position: u8,
    pub sta_id: u8,
    pub reserved1: u8,
    pub toa_offset: __le16,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub reserved2: __le16,
    pub /: *mut *mut } __packed; / TOF_RESPONDER_CONFIG_CMD_API_S_VER_6,
//
// struct iwl_tof_responder_config_cmd_v7 - ToF AP mode (for debug)
// @cmd_valid_fields: &iwl_tof_responder_cmd_valid_field
// @responder_cfg_flags: &iwl_tof_responder_cfg_flags
// @format_bw: &enum iwl_location_format_bw
// @rate: current AP rate
// @channel_num: current AP Channel
// @ctrl_ch_position: coding of the control channel position relative to
// the center frequency, see iwl_mvm_get_ctrl_pos()
// @sta_id: index of the AP STA when in AP mode
// @reserved1: reserved
// @toa_offset: Artificial addition [pSec] for the ToA - to be used for debug
// purposes, simulating station movement by adding various values
// to this field
// @common_calib: XVT: common calibration value
// @specific_calib: XVT: specific calibration value
// @bssid: Current AP BSSID
// @reserved2: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_responder_config_cmd_v7 {
    pub cmd_valid_fields: __le32,
    pub responder_cfg_flags: __le32,
    pub format_bw: u8,
    pub rate: u8,
    pub channel_num: u8,
    pub ctrl_ch_position: u8,
    pub sta_id: u8,
    pub reserved1: u8,
    pub toa_offset: __le16,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub reserved2: __le16,
    pub /: *mut *mut } __packed; / TOF_RESPONDER_CONFIG_CMD_API_S_VER_7,
pub const IWL_RESPONDER_STS_POS: c_int = 3;
pub const IWL_RESPONDER_TOTAL_LTF_POS: c_int = 6;
//
// struct iwl_tof_responder_config_cmd_v8 - ToF AP mode (for debug)
// @cmd_valid_fields: &iwl_tof_responder_cmd_valid_field
// @responder_cfg_flags: &iwl_tof_responder_cfg_flags
// @format_bw: &enum iwl_location_format_bw
// @rate: current AP rate
// @channel_num: current AP Channel
// @ctrl_ch_position: coding of the control channel position relative to
// the center frequency, see iwl_mvm_get_ctrl_pos()
// @sta_id: index of the AP STA when in AP mode
// @reserved1: reserved
// @toa_offset: Artificial addition [pSec] for the ToA - to be used for debug
// purposes, simulating station movement by adding various values
// to this field
// @common_calib: XVT: common calibration value
// @specific_calib: XVT: specific calibration value
// @bssid: Current AP BSSID
// @r2i_ndp_params: parameters for R2I NDP.
// bits 0 - 2: max number of LTF repetitions
// bits 3 - 5: max number of spatial streams (supported values are < 2)
// bits 6 - 7: max number of total LTFs see
// &enum ieee80211_range_params_max_total_ltf
// @i2r_ndp_params: parameters for I2R NDP.
// bits 0 - 2: max number of LTF repetitions
// bits 3 - 5: max number of spatial streams
// bits 6 - 7: max number of total LTFs see
// &enum ieee80211_range_params_max_total_ltf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_responder_config_cmd_v8 {
    pub cmd_valid_fields: __le32,
    pub responder_cfg_flags: __le32,
    pub format_bw: u8,
    pub rate: u8,
    pub channel_num: u8,
    pub ctrl_ch_position: u8,
    pub sta_id: u8,
    pub reserved1: u8,
    pub toa_offset: __le16,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub r2i_ndp_params: u8,
    pub i2r_ndp_params: u8,
    pub /: *mut *mut } __packed; / TOF_RESPONDER_CONFIG_CMD_API_S_VER_8,
//
// struct iwl_tof_responder_config_cmd_v9 - ToF AP mode (for debug)
// @cmd_valid_fields: &iwl_tof_responder_cmd_valid_field
// @responder_cfg_flags: &iwl_tof_responder_cfg_flags
// @format_bw: &enum iwl_location_format_bw
// @bss_color: current AP bss_color
// @channel_num: current AP Channel
// @ctrl_ch_position: coding of the control channel position relative to
// the center frequency, see iwl_mvm_get_ctrl_pos()
// @sta_id: index of the AP STA when in AP mode
// @reserved1: reserved
// @toa_offset: Artificial addition [pSec] for the ToA - to be used for debug
// purposes, simulating station movement by adding various values
// to this field
// @common_calib: XVT: common calibration value
// @specific_calib: XVT: specific calibration value
// @bssid: Current AP BSSID
// @r2i_ndp_params: parameters for R2I NDP.
// bits 0 - 2: max number of LTF repetitions
// bits 3 - 5: max number of spatial streams (supported values are < 2)
// bits 6 - 7: max number of total LTFs see
// &enum ieee80211_range_params_max_total_ltf
// @i2r_ndp_params: parameters for I2R NDP.
// bits 0 - 2: max number of LTF repetitions
// bits 3 - 5: max number of spatial streams
// bits 6 - 7: max number of total LTFs see
// &enum ieee80211_range_params_max_total_ltf
// @min_time_between_msr: for non trigger based NDP ranging, minimum time
// between measurements in milliseconds.
// @max_time_between_msr: for non trigger based NDP ranging, maximum time
// between measurements in milliseconds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_responder_config_cmd_v9 {
    pub cmd_valid_fields: __le32,
    pub responder_cfg_flags: __le32,
    pub format_bw: u8,
    pub bss_color: u8,
    pub channel_num: u8,
    pub ctrl_ch_position: u8,
    pub sta_id: u8,
    pub reserved1: u8,
    pub toa_offset: __le16,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub r2i_ndp_params: u8,
    pub i2r_ndp_params: u8,
    pub min_time_between_msr: __le16,
    pub max_time_between_msr: __le16,
    pub /: *mut *mut } __packed; / TOF_RESPONDER_CONFIG_CMD_API_S_VER_8,
//
// struct iwl_tof_responder_config_cmd - ToF AP mode
// @cmd_valid_fields: &iwl_tof_responder_cmd_valid_field
// @responder_cfg_flags: &iwl_tof_responder_cfg_flags
// @format_bw: &enum iwl_location_format_bw
// @bss_color: current AP bss_color
// @channel_num: current AP Channel
// @ctrl_ch_position: coding of the control channel position relative to
// the center frequency, see iwl_mvm_get_ctrl_pos()
// @sta_id: index of the AP STA when in AP mode
// @band: current AP band
// @toa_offset: Artificial addition [pSec] for the ToA - to be used for debug
// purposes, simulating station movement by adding various values
// to this field
// @common_calib: XVT: common calibration value
// @specific_calib: XVT: specific calibration value
// @bssid: Current AP BSSID
// @r2i_ndp_params: parameters for R2I NDP.
// bits 0 - 2: max number of LTF repetitions
// bits 3 - 5: max number of spatial streams (supported values are < 2)
// bits 6 - 7: max number of total LTFs see
// &enum ieee80211_range_params_max_total_ltf
// @i2r_ndp_params: parameters for I2R NDP.
// bits 0 - 2: max number of LTF repetitions
// bits 3 - 5: max number of spatial streams
// bits 6 - 7: max number of total LTFs see
// &enum ieee80211_range_params_max_total_ltf
// @min_time_between_msr: for non trigger based NDP ranging, minimum time
// between measurements in milliseconds.
// @max_time_between_msr: for non trigger based NDP ranging, maximum time
// between measurements in milliseconds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_responder_config_cmd {
    pub cmd_valid_fields: __le32,
    pub responder_cfg_flags: __le32,
    pub format_bw: u8,
    pub bss_color: u8,
    pub channel_num: u8,
    pub ctrl_ch_position: u8,
    pub sta_id: u8,
    pub band: u8,
    pub toa_offset: __le16,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub r2i_ndp_params: u8,
    pub i2r_ndp_params: u8,
    pub min_time_between_msr: __le16,
    pub max_time_between_msr: __le16,
    pub /: *mut *mut } __packed; / TOF_RESPONDER_CONFIG_CMD_API_S_VER_10,
pub const IWL_LCI_CIVIC_IE_MAX_SIZE: c_int = 400;
//
// struct iwl_tof_responder_dyn_config_cmd_v2 - Dynamic responder settings
// @lci_len: The length of the 1st (LCI) part in the @lci_civic buffer
// @civic_len: The length of the 2nd (CIVIC) part in the @lci_civic buffer
// @lci_civic: The LCI/CIVIC buffer. LCI data (if exists) comes first, then, if
// needed, 0-padding such that the next part is dword-aligned, then CIVIC
// data (if exists) follows, and then 0-padding again to complete a
// 4-multiple long buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_responder_dyn_config_cmd_v2 {
    pub lci_len: __le32,
    pub civic_len: __le32,
    pub lci_civic: [u8; ],
    pub /: *mut *mut } __packed; / TOF_RESPONDER_DYN_CONFIG_CMD_API_S_VER_2,
pub const IWL_LCI_MAX_SIZE: c_int = 160;
pub const IWL_CIVIC_MAX_SIZE: c_int = 160;
pub const HLTK_11AZ_LEN: c_int = 32;
//
// enum iwl_responder_dyn_cfg_valid_flags - valid flags for dyn_config_cmd
// @IWL_RESPONDER_DYN_CFG_VALID_LCI: LCI data is valid
// @IWL_RESPONDER_DYN_CFG_VALID_CIVIC: Civic data is valid
// @IWL_RESPONDER_DYN_CFG_VALID_PASN_STA: the pasn_addr, HLTK and cipher fields
// are valid.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_responder_dyn_cfg_valid_flags {
    IWL_RESPONDER_DYN_CFG_VALID_LCI = BIT(0),
    IWL_RESPONDER_DYN_CFG_VALID_CIVIC = BIT(1),
    IWL_RESPONDER_DYN_CFG_VALID_PASN_STA = BIT(2),
}

//
// struct iwl_tof_responder_dyn_config_cmd - Dynamic responder settings
// @cipher: The negotiated cipher. see &enum iwl_location_cipher.
// @valid_flags: flags indicating which fields in the command are valid. see
// &enum iwl_responder_dyn_cfg_valid_flags.
// @lci_len: length of the LCI data in bytes
// @civic_len: length of the Civic data in bytes
// @lci_buf: the LCI buffer
// @civic_buf: the Civic buffer
// @hltk_buf: HLTK for secure LTF bits generation for the specified station
// @addr: mac address of the station for which to use the HLTK
// @reserved: for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_responder_dyn_config_cmd {
    pub cipher: u8,
    pub valid_flags: u8,
    pub lci_len: u8,
    pub civic_len: u8,
    pub lci_buf: [u8; IWL_LCI_MAX_SIZE],
    pub civic_buf: [u8; IWL_LCI_MAX_SIZE],
    pub hltk_buf: [u8; HLTK_11AZ_LEN],
    pub addr: [u8; ETH_ALEN],
    pub reserved: [u8; 2],
    pub /: *mut *mut } __packed; / TOF_RESPONDER_DYN_CONFIG_CMD_API_S_VER_3,
//
// struct iwl_tof_range_req_ext_cmd - extended range req for WLS
// @tsf_timer_offset_msec: the recommended time offset (mSec) from the AP's TSF
// @reserved: reserved
// @min_delta_ftm: Minimal time between two consecutive measurements,
// in units of 100us. 0 means no preference by station
// @ftm_format_and_bw20M: FTM Channel Spacing/Format for 20MHz: recommended
// value be sent to the AP
// @ftm_format_and_bw40M: FTM Channel Spacing/Format for 40MHz: recommended
// value to be sent to the AP
// @ftm_format_and_bw80M: FTM Channel Spacing/Format for 80MHz: recommended
// value to be sent to the AP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_ext_cmd {
    pub tsf_timer_offset_msec: __le16,
    pub reserved: __le16,
    pub min_delta_ftm: u8,
    pub ftm_format_and_bw20M: u8,
    pub ftm_format_and_bw40M: u8,
    pub ftm_format_and_bw80M: u8,
    pub __packed: },
//
// enum iwl_tof_location_query - values for query bitmap
// @IWL_TOF_LOC_LCI: query LCI
// @IWL_TOF_LOC_CIVIC: query civic
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_location_query {
    IWL_TOF_LOC_LCI = 0x01,
    IWL_TOF_LOC_CIVIC = 0x02,
}

//
// struct iwl_tof_range_req_ap_entry_v2 - AP configuration parameters
// @channel_num: Current AP Channel
// @bandwidth: Current AP Bandwidth. One of iwl_tof_bandwidth.
// @tsf_delta_direction: TSF relatively to the subject AP
// @ctrl_ch_position: Coding of the control channel position relative to the
// center frequency, see iwl_mvm_get_ctrl_pos().
// @bssid: AP's BSSID
// @measure_type: Measurement type: 0 - two sided, 1 - One sided
// @num_of_bursts: Recommended value to be sent to the AP.  2s Exponent of the
// number of measurement iterations (min 2^0 = 1, max 2^14)
// @burst_period: Recommended value to be sent to the AP. Measurement
// periodicity In units of 100ms. ignored if num_of_bursts = 0
// @samples_per_burst: 2-sided: the number of FTMs pairs in single Burst (1-31);
// 1-sided: how many rts/cts pairs should be used per burst.
// @retries_per_sample: Max number of retries that the LMAC should send
// in case of no replies by the AP.
// @tsf_delta: TSF Delta in units of microseconds.
// The difference between the AP TSF and the device local clock.
// @location_req: Location Request Bit[0] LCI should be sent in the FTMR;
// Bit[1] Civic should be sent in the FTMR
// @asap_mode: 0 - non asap mode, 1 - asap mode (not relevant for one sided)
// @enable_dyn_ack: Enable Dynamic ACK BW.
// 0: Initiator interact with regular AP;
// 1: Initiator interact with Responder machine: need to send the
// Initiator Acks with HT 40MHz / 80MHz, since the Responder should
// use it for its ch est measurement (this flag will be set when we
// configure the opposite machine to be Responder).
// @rssi: Last received value
// legal values: -128-0 (0x7f). above 0x0 indicating an invalid value.
// @algo_type: &enum iwl_tof_algo_type
// @notify_mcsi: &enum iwl_tof_mcsi_ntfy.
// @reserved: For alignment and future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_ap_entry_v2 {
    pub channel_num: u8,
    pub bandwidth: u8,
    pub tsf_delta_direction: u8,
    pub ctrl_ch_position: u8,
    pub bssid: [u8; ETH_ALEN],
    pub measure_type: u8,
    pub num_of_bursts: u8,
    pub burst_period: __le16,
    pub samples_per_burst: u8,
    pub retries_per_sample: u8,
    pub tsf_delta: __le32,
    pub location_req: u8,
    pub asap_mode: u8,
    pub enable_dyn_ack: u8,
    pub rssi: i8,
    pub algo_type: u8,
    pub notify_mcsi: u8,
    pub reserved: __le16,
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_AP_ENTRY_CMD_API_S_VER_2,
//
// enum iwl_initiator_ap_flags - per responder FTM configuration flags
// @IWL_INITIATOR_AP_FLAGS_ASAP: Request for ASAP measurement.
// @IWL_INITIATOR_AP_FLAGS_LCI_REQUEST: Request for LCI information
// @IWL_INITIATOR_AP_FLAGS_CIVIC_REQUEST: Request for CIVIC information
// @IWL_INITIATOR_AP_FLAGS_DYN_ACK: Send HT/VHT ack for FTM frames. If not set,
// 20Mhz dup acks will be sent.
// @IWL_INITIATOR_AP_FLAGS_ALGO_LR: Use LR algo type for rtt calculation.
// Default algo type is ML.
// @IWL_INITIATOR_AP_FLAGS_ALGO_FFT: Use FFT algo type for rtt calculation.
// Default algo type is ML.
// @IWL_INITIATOR_AP_FLAGS_MCSI_REPORT: Send the MCSI for each FTM frame to the
// driver.
// @IWL_INITIATOR_AP_FLAGS_NON_TB: Use non trigger based flow
// @IWL_INITIATOR_AP_FLAGS_TB: Use trigger based flow
// @IWL_INITIATOR_AP_FLAGS_SECURED: request secure LTF measurement
// @IWL_INITIATOR_AP_FLAGS_LMR_FEEDBACK: Send LMR feedback
// @IWL_INITIATOR_AP_FLAGS_USE_CALIB: Use calibration values from the request
// instead of fw internal values.
// @IWL_INITIATOR_AP_FLAGS_PMF: request to protect the negotiation and LMR
// frames with protected management frames.
// @IWL_INITIATOR_AP_FLAGS_TERMINATE_ON_LMR_FEEDBACK: terminate the session if
// the responder asked for LMR feedback although the initiator did not set
// the LMR feedback bit in the FTM request. If not set, the initiator will
// continue with the session and will provide the LMR feedback.
// @IWL_INITIATOR_AP_FLAGS_TEST_INCORRECT_SAC: send an incorrect SAC in the
// first NDP exchange. This is used for testing.
// @IWL_INITIATOR_AP_FLAGS_TEST_BAD_SLTF: use incorrect secure LTF tx key. This
// is used for testing. Only supported from version 15 of the range request
// command.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_initiator_ap_flags {
    IWL_INITIATOR_AP_FLAGS_ASAP = BIT(1),
    IWL_INITIATOR_AP_FLAGS_LCI_REQUEST = BIT(2),
    IWL_INITIATOR_AP_FLAGS_CIVIC_REQUEST = BIT(3),
    IWL_INITIATOR_AP_FLAGS_DYN_ACK = BIT(4),
    IWL_INITIATOR_AP_FLAGS_ALGO_LR = BIT(5),
    IWL_INITIATOR_AP_FLAGS_ALGO_FFT = BIT(6),
    IWL_INITIATOR_AP_FLAGS_MCSI_REPORT = BIT(8),
    IWL_INITIATOR_AP_FLAGS_NON_TB = BIT(9),
    IWL_INITIATOR_AP_FLAGS_TB = BIT(10),
    IWL_INITIATOR_AP_FLAGS_SECURED = BIT(11),
    IWL_INITIATOR_AP_FLAGS_LMR_FEEDBACK = BIT(12),
    IWL_INITIATOR_AP_FLAGS_USE_CALIB = BIT(13),
    IWL_INITIATOR_AP_FLAGS_PMF = BIT(14),
    IWL_INITIATOR_AP_FLAGS_TERMINATE_ON_LMR_FEEDBACK = BIT(15),
    IWL_INITIATOR_AP_FLAGS_TEST_INCORRECT_SAC = BIT(16),
    IWL_INITIATOR_AP_FLAGS_TEST_BAD_SLTF = BIT(17),
}

//
// struct iwl_tof_range_req_ap_entry_v3 - AP configuration parameters
// @initiator_ap_flags: see &enum iwl_initiator_ap_flags.
// @channel_num: AP Channel number
// @bandwidth: AP bandwidth. One of iwl_tof_bandwidth.
// @ctrl_ch_position: Coding of the control channel position relative to the
// center frequency, see iwl_mvm_get_ctrl_pos().
// @ftmr_max_retries: Max number of retries to send the FTMR in case of no
// reply from the AP.
// @bssid: AP's BSSID
// @burst_period: Recommended value to be sent to the AP. Measurement
// periodicity In units of 100ms. ignored if num_of_bursts_exp = 0
// @samples_per_burst: the number of FTMs pairs in single Burst (1-31);
// @num_of_bursts: Recommended value to be sent to the AP. 2s Exponent of
// the number of measurement iterations (min 2^0 = 1, max 2^14)
// @reserved: For alignment and future use
// @tsf_delta: not in use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_ap_entry_v3 {
    pub initiator_ap_flags: __le32,
    pub channel_num: u8,
    pub bandwidth: u8,
    pub ctrl_ch_position: u8,
    pub ftmr_max_retries: u8,
    pub bssid: [u8; ETH_ALEN],
    pub burst_period: __le16,
    pub samples_per_burst: u8,
    pub num_of_bursts: u8,
    pub reserved: __le16,
    pub tsf_delta: __le32,
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_AP_ENTRY_CMD_API_S_VER_3,
pub const TK_11AZ_LEN: c_int = 32;
//
// struct iwl_tof_range_req_ap_entry_v4 - AP configuration parameters
// @initiator_ap_flags: see &enum iwl_initiator_ap_flags.
// @channel_num: AP Channel number
// @format_bw: &enum iwl_location_format_bw
// @ctrl_ch_position: Coding of the control channel position relative to the
// center frequency, see iwl_mvm_get_ctrl_pos().
// @ftmr_max_retries: Max number of retries to send the FTMR in case of no
// reply from the AP.
// @bssid: AP's BSSID
// @burst_period: Recommended value to be sent to the AP. Measurement
// periodicity In units of 100ms. ignored if num_of_bursts_exp = 0
// @samples_per_burst: the number of FTMs pairs in single Burst (1-31);
// @num_of_bursts: Recommended value to be sent to the AP. 2s Exponent of
// the number of measurement iterations (min 2^0 = 1, max 2^14)
// @reserved: For alignment and future use
// @hltk: HLTK to be used for secured 11az measurement
// @tk: TK to be used for secured 11az measurement
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_ap_entry_v4 {
    pub initiator_ap_flags: __le32,
    pub channel_num: u8,
    pub format_bw: u8,
    pub ctrl_ch_position: u8,
    pub ftmr_max_retries: u8,
    pub bssid: [u8; ETH_ALEN],
    pub burst_period: __le16,
    pub samples_per_burst: u8,
    pub num_of_bursts: u8,
    pub reserved: __le16,
    pub hltk: [u8; HLTK_11AZ_LEN],
    pub tk: [u8; TK_11AZ_LEN],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_AP_ENTRY_CMD_API_S_VER_4,
//
// enum iwl_location_cipher - location cipher selection
// @IWL_LOCATION_CIPHER_CCMP_128: CCMP 128
// @IWL_LOCATION_CIPHER_GCMP_128: GCMP 128
// @IWL_LOCATION_CIPHER_GCMP_256: GCMP 256
// @IWL_LOCATION_CIPHER_INVALID: security is not used.
// @IWL_LOCATION_CIPHER_MAX: maximum value for this enum.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_location_cipher {
    IWL_LOCATION_CIPHER_CCMP_128,
    IWL_LOCATION_CIPHER_GCMP_128,
    IWL_LOCATION_CIPHER_GCMP_256,
    IWL_LOCATION_CIPHER_INVALID,
    IWL_LOCATION_CIPHER_MAX,
}

//
// struct iwl_tof_range_req_ap_entry_v6 - AP configuration parameters
// @initiator_ap_flags: see &enum iwl_initiator_ap_flags.
// @channel_num: AP Channel number
// @format_bw: &enum iwl_location_format_bw
// @ctrl_ch_position: Coding of the control channel position relative to the
// center frequency, see iwl_mvm_get_ctrl_pos().
// @ftmr_max_retries: Max number of retries to send the FTMR in case of no
// reply from the AP.
// @bssid: AP's BSSID
// @burst_period: Recommended value to be sent to the AP. Measurement
// periodicity In units of 100ms. ignored if num_of_bursts_exp = 0
// @samples_per_burst: the number of FTMs pairs in single Burst (1-31);
// @num_of_bursts: Recommended value to be sent to the AP. 2s Exponent of
// the number of measurement iterations (min 2^0 = 1, max 2^14)
// @sta_id: the station id of the AP. Only relevant when associated to the AP,
// otherwise should be set to &IWL_INVALID_STA.
// @cipher: pairwise cipher suite for secured measurement.
// &enum iwl_location_cipher.
// @hltk: HLTK to be used for secured 11az measurement
// @tk: TK to be used for secured 11az measurement
// @calib: An array of calibration values per FTM rx bandwidth.
// If &IWL_INITIATOR_AP_FLAGS_USE_CALIB is set, the fw will use the
// calibration value that corresponds to the rx bandwidth of the FTM
// frame.
// @beacon_interval: beacon interval of the AP in TUs. Only required if
// &IWL_INITIATOR_AP_FLAGS_TB is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_ap_entry_v6 {
    pub initiator_ap_flags: __le32,
    pub channel_num: u8,
    pub format_bw: u8,
    pub ctrl_ch_position: u8,
    pub ftmr_max_retries: u8,
    pub bssid: [u8; ETH_ALEN],
    pub burst_period: __le16,
    pub samples_per_burst: u8,
    pub num_of_bursts: u8,
    pub sta_id: u8,
    pub cipher: u8,
    pub hltk: [u8; HLTK_11AZ_LEN],
    pub tk: [u8; TK_11AZ_LEN],
    pub calib: [__le16; IWL_TOF_BW_NUM],
    pub beacon_interval: __le16,
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_AP_ENTRY_CMD_API_S_VER_6,
//
// struct iwl_tof_range_req_ap_entry_v7 - AP configuration parameters
// @initiator_ap_flags: see &enum iwl_initiator_ap_flags.
// @channel_num: AP Channel number
// @format_bw: &enum iwl_location_format_bw
// @ctrl_ch_position: Coding of the control channel position relative to the
// center frequency, see iwl_mvm_get_ctrl_pos().
// @ftmr_max_retries: Max number of retries to send the FTMR in case of no
// reply from the AP.
// @bssid: AP's BSSID
// @burst_period: Recommended value to be sent to the AP. Measurement
// periodicity In units of 100ms. ignored if num_of_bursts_exp = 0
// @samples_per_burst: the number of FTMs pairs in single Burst (1-31);
// @num_of_bursts: Recommended value to be sent to the AP. 2s Exponent of
// the number of measurement iterations (min 2^0 = 1, max 2^14)
// @sta_id: the station id of the AP. Only relevant when associated to the AP,
// otherwise should be set to &IWL_INVALID_STA.
// @cipher: pairwise cipher suite for secured measurement.
// &enum iwl_location_cipher.
// @hltk: HLTK to be used for secured 11az measurement
// @tk: TK to be used for secured 11az measurement
// @calib: An array of calibration values per FTM rx bandwidth.
// If &IWL_INITIATOR_AP_FLAGS_USE_CALIB is set, the fw will use the
// calibration value that corresponds to the rx bandwidth of the FTM
// frame.
// @beacon_interval: beacon interval of the AP in TUs. Only required if
// &IWL_INITIATOR_AP_FLAGS_TB is set.
// @rx_pn: the next expected PN for protected management frames Rx. LE byte
// order. Only valid if &IWL_INITIATOR_AP_FLAGS_SECURED is set and sta_id
// is set to &IWL_INVALID_STA.
// @tx_pn: the next PN to use for protected management frames Tx. LE byte
// order. Only valid if &IWL_INITIATOR_AP_FLAGS_SECURED is set and sta_id
// is set to &IWL_INVALID_STA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_ap_entry_v7 {
    pub initiator_ap_flags: __le32,
    pub channel_num: u8,
    pub format_bw: u8,
    pub ctrl_ch_position: u8,
    pub ftmr_max_retries: u8,
    pub bssid: [u8; ETH_ALEN],
    pub burst_period: __le16,
    pub samples_per_burst: u8,
    pub num_of_bursts: u8,
    pub sta_id: u8,
    pub cipher: u8,
    pub hltk: [u8; HLTK_11AZ_LEN],
    pub tk: [u8; TK_11AZ_LEN],
    pub calib: [__le16; IWL_TOF_BW_NUM],
    pub beacon_interval: __le16,
    pub rx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub tx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_AP_ENTRY_CMD_API_S_VER_7,
pub const IWL_LOCATION_MAX_STS_POS: c_int = 3;
pub const IWL_LOCATION_TOTAL_LTF_POS: c_int = 6;
//
// struct iwl_tof_range_req_ap_entry_v8 - AP configuration parameters
// @initiator_ap_flags: see &enum iwl_initiator_ap_flags.
// @channel_num: AP Channel number
// @format_bw: &enum iwl_location_format_bw
// @ctrl_ch_position: Coding of the control channel position relative to the
// center frequency, see iwl_mvm_get_ctrl_pos().
// @ftmr_max_retries: Max number of retries to send the FTMR in case of no
// reply from the AP.
// @bssid: AP's BSSID
// @burst_period: Recommended value to be sent to the AP. Measurement
// periodicity In units of 100ms. ignored if num_of_bursts_exp = 0
// @samples_per_burst: the number of FTMs pairs in single Burst (1-31);
// @num_of_bursts: Recommended value to be sent to the AP. 2s Exponent of
// the number of measurement iterations (min 2^0 = 1, max 2^14)
// @sta_id: the station id of the AP. Only relevant when associated to the AP,
// otherwise should be set to &IWL_INVALID_STA.
// @cipher: pairwise cipher suite for secured measurement.
// &enum iwl_location_cipher.
// @hltk: HLTK to be used for secured 11az measurement
// @tk: TK to be used for secured 11az measurement
// @calib: An array of calibration values per FTM rx bandwidth.
// If &IWL_INITIATOR_AP_FLAGS_USE_CALIB is set, the fw will use the
// calibration value that corresponds to the rx bandwidth of the FTM
// frame.
// @beacon_interval: beacon interval of the AP in TUs. Only required if
// &IWL_INITIATOR_AP_FLAGS_TB is set.
// @rx_pn: the next expected PN for protected management frames Rx. LE byte
// order. Only valid if &IWL_INITIATOR_AP_FLAGS_SECURED is set and sta_id
// is set to &IWL_INVALID_STA.
// @tx_pn: the next PN to use for protected management frames Tx. LE byte
// order. Only valid if &IWL_INITIATOR_AP_FLAGS_SECURED is set and sta_id
// is set to &IWL_INVALID_STA.
// @r2i_ndp_params: parameters for R2I NDP ranging negotiation.
// bits 0 - 2: max LTF repetitions
// bits 3 - 5: max number of spatial streams
// bits 6 - 7: reserved
// @i2r_ndp_params: parameters for I2R NDP ranging negotiation.
// bits 0 - 2: max LTF repetitions
// bits 3 - 5: max number of spatial streams (supported values are < 2)
// bits 6 - 7: reserved
// @r2i_max_total_ltf: R2I Max Total LTFs for NDP ranging negotiation.
// One of &enum ieee80211_range_params_max_total_ltf.
// @i2r_max_total_ltf: I2R Max Total LTFs for NDP ranging negotiation.
// One of &enum ieee80211_range_params_max_total_ltf.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_ap_entry_v8 {
    pub initiator_ap_flags: __le32,
    pub channel_num: u8,
    pub format_bw: u8,
    pub ctrl_ch_position: u8,
    pub ftmr_max_retries: u8,
    pub bssid: [u8; ETH_ALEN],
    pub burst_period: __le16,
    pub samples_per_burst: u8,
    pub num_of_bursts: u8,
    pub sta_id: u8,
    pub cipher: u8,
    pub hltk: [u8; HLTK_11AZ_LEN],
    pub tk: [u8; TK_11AZ_LEN],
    pub calib: [__le16; IWL_TOF_BW_NUM],
    pub beacon_interval: __le16,
    pub rx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub tx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub r2i_ndp_params: u8,
    pub i2r_ndp_params: u8,
    pub r2i_max_total_ltf: u8,
    pub i2r_max_total_ltf: u8,
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_AP_ENTRY_CMD_API_S_VER_8,
//
// struct iwl_tof_range_req_ap_entry_v9 - AP configuration parameters
// @initiator_ap_flags: see &enum iwl_initiator_ap_flags.
// @channel_num: AP Channel number
// @format_bw: &enum iwl_location_format_bw
// @ctrl_ch_position: Coding of the control channel position relative to the
// center frequency, see iwl_mvm_get_ctrl_pos().
// @ftmr_max_retries: Max number of retries to send the FTMR in case of no
// reply from the AP.
// @bssid: AP's BSSID
// @burst_period: For EDCA based ranging: Recommended value to be sent to the
// AP. Measurement periodicity In units of 100ms. ignored if
// num_of_bursts_exp = 0.
// For non trigger based NDP ranging, the maximum time between
// measurements in units of milliseconds.
// @samples_per_burst: the number of FTMs pairs in single Burst (1-31);
// @num_of_bursts: Recommended value to be sent to the AP. 2s Exponent of
// the number of measurement iterations (min 2^0 = 1, max 2^14)
// @sta_id: the station id of the AP. Only relevant when associated to the AP,
// otherwise should be set to &IWL_INVALID_STA.
// @cipher: pairwise cipher suite for secured measurement.
// &enum iwl_location_cipher.
// @hltk: HLTK to be used for secured 11az measurement
// @tk: TK to be used for secured 11az measurement
// @calib: An array of calibration values per FTM rx bandwidth.
// If &IWL_INITIATOR_AP_FLAGS_USE_CALIB is set, the fw will use the
// calibration value that corresponds to the rx bandwidth of the FTM
// frame.
// @beacon_interval: beacon interval of the AP in TUs. Only required if
// &IWL_INITIATOR_AP_FLAGS_TB is set.
// @bss_color: the BSS color of the responder. Only valid if
// &IWL_INITIATOR_AP_FLAGS_TB or &IWL_INITIATOR_AP_FLAGS_NON_TB is set.
// @rx_pn: the next expected PN for protected management frames Rx. LE byte
// order. Only valid if &IWL_INITIATOR_AP_FLAGS_SECURED is set and sta_id
// is set to &IWL_INVALID_STA.
// @tx_pn: the next PN to use for protected management frames Tx. LE byte
// order. Only valid if &IWL_INITIATOR_AP_FLAGS_SECURED is set and sta_id
// is set to &IWL_INVALID_STA.
// @r2i_ndp_params: parameters for R2I NDP ranging negotiation.
// bits 0 - 2: max LTF repetitions
// bits 3 - 5: max number of spatial streams
// bits 6 - 7: reserved
// @i2r_ndp_params: parameters for I2R NDP ranging negotiation.
// bits 0 - 2: max LTF repetitions
// bits 3 - 5: max number of spatial streams (supported values are < 2)
// bits 6 - 7: reserved
// @r2i_max_total_ltf: R2I Max Total LTFs for NDP ranging negotiation.
// One of &enum ieee80211_range_params_max_total_ltf.
// @i2r_max_total_ltf: I2R Max Total LTFs for NDP ranging negotiation.
// One of &enum ieee80211_range_params_max_total_ltf.
// @bss_color: the BSS color of the responder. Only valid if
// &IWL_INITIATOR_AP_FLAGS_NON_TB or &IWL_INITIATOR_AP_FLAGS_TB is set.
// @band: 0 for 5.2 GHz, 1 for 2.4 GHz, 2 for 6GHz
// @min_time_between_msr: For non trigger based NDP ranging, the minimum time
// between measurements in units of milliseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_ap_entry_v9 {
    pub initiator_ap_flags: __le32,
    pub channel_num: u8,
    pub format_bw: u8,
    pub ctrl_ch_position: u8,
    pub ftmr_max_retries: u8,
    pub bssid: [u8; ETH_ALEN],
    pub burst_period: __le16,
    pub samples_per_burst: u8,
    pub num_of_bursts: u8,
    pub sta_id: u8,
    pub cipher: u8,
    pub hltk: [u8; HLTK_11AZ_LEN],
    pub tk: [u8; TK_11AZ_LEN],
    pub calib: [__le16; IWL_TOF_BW_NUM],
    pub beacon_interval: u16,
    pub rx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub tx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub r2i_ndp_params: u8,
    pub i2r_ndp_params: u8,
    pub r2i_max_total_ltf: u8,
    pub i2r_max_total_ltf: u8,
    pub bss_color: u8,
    pub band: u8,
    pub min_time_between_msr: __le16,
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_AP_ENTRY_CMD_API_S_VER_9,
//
// struct iwl_tof_range_req_ap_entry - AP configuration parameters
// @initiator_ap_flags: see &enum iwl_initiator_ap_flags.
// @band: 0 for 5.2 GHz, 1 for 2.4 GHz, 2 for 6GHz
// @channel_num: AP Channel number
// @format_bw: &enum iwl_location_format_bw
// @ctrl_ch_position: Coding of the control channel position relative to the
// center frequency, see iwl_mvm_get_ctrl_pos().
// @bssid: AP's BSSID
// @burst_period: For EDCA based ranging: Recommended value to be sent to the
// AP. Measurement periodicity In units of 100ms. ignored if
// num_of_bursts_exp = 0.
// For non trigger based NDP ranging, the maximum time between
// measurements in units of milliseconds.
// @samples_per_burst: the number of FTMs pairs in single Burst (1-31);
// @num_of_bursts: Recommended value to be sent to the AP. 2s Exponent of
// the number of measurement iterations (min 2^0 = 1, max 2^14)
// @sta_id: the station id of the AP. Only relevant when associated to the AP,
// otherwise should be set to &IWL_INVALID_STA.
// @cipher: pairwise cipher suite for secured measurement.
// &enum iwl_location_cipher.
// @hltk: HLTK to be used for secured 11az measurement
// @tk: TK to be used for secured 11az measurement
// @calib: An array of calibration values per FTM rx bandwidth.
// If &IWL_INITIATOR_AP_FLAGS_USE_CALIB is set, the fw will use the
// calibration value that corresponds to the rx bandwidth of the FTM
// frame.
// @beacon_interval: beacon interval of the AP in TUs. Only required if
// &IWL_INITIATOR_AP_FLAGS_TB is set.
// @rx_pn: the next expected PN for protected management frames Rx. LE byte
// order. Only valid if &IWL_INITIATOR_AP_FLAGS_SECURED is set and sta_id
// is set to &IWL_INVALID_STA.
// @tx_pn: the next PN to use for protected management frames Tx. LE byte
// order. Only valid if &IWL_INITIATOR_AP_FLAGS_SECURED is set and sta_id
// is set to &IWL_INVALID_STA.
// @r2i_ndp_params: parameters for R2I NDP ranging negotiation.
// bits 0 - 2: max LTF repetitions
// bits 3 - 5: max number of spatial streams
// bits 6 - 7: max total LTFs. One of
// &enum ieee80211_range_params_max_total_ltf.
// @i2r_ndp_params: parameters for I2R NDP ranging negotiation.
// bits 0 - 2: max LTF repetitions
// bits 3 - 5: max number of spatial streams (supported values are < 2)
// bits 6 - 7: max total LTFs. One of
// &enum ieee80211_range_params_max_total_ltf.
// @min_time_between_msr: For non trigger based NDP ranging, the minimum time
// between measurements in units of milliseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_ap_entry {
    pub initiator_ap_flags: __le32,
    pub band: u8,
    pub channel_num: u8,
    pub format_bw: u8,
    pub ctrl_ch_position: u8,
    pub bssid: [u8; ETH_ALEN],
    pub burst_period: __le16,
    pub samples_per_burst: u8,
    pub num_of_bursts: u8,
    pub sta_id: u8,
    pub cipher: u8,
    pub hltk: [u8; HLTK_11AZ_LEN],
    pub tk: [u8; TK_11AZ_LEN],
    pub calib: [__le16; IWL_TOF_BW_NUM],
    pub beacon_interval: __le16,
    pub rx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub tx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub r2i_ndp_params: u8,
    pub i2r_ndp_params: u8,
    pub min_time_between_msr: __le16,
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_AP_ENTRY_CMD_API_S_VER_9,
//
// enum iwl_tof_response_mode - TOF response mode
// @IWL_MVM_TOF_RESPONSE_ASAP: report each AP measurement separately as soon as
// possible (not supported for this release)
// @IWL_MVM_TOF_RESPONSE_TIMEOUT: report all AP measurements as a batch upon
// timeout expiration
// @IWL_MVM_TOF_RESPONSE_COMPLETE: report all AP measurements as a batch at the
// earlier of: measurements completion / timeout
// expiration.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_response_mode {
    IWL_MVM_TOF_RESPONSE_ASAP,
    IWL_MVM_TOF_RESPONSE_TIMEOUT,
    IWL_MVM_TOF_RESPONSE_COMPLETE,
}

//
// enum iwl_tof_initiator_flags - TOF initiator flags
//
// @IWL_TOF_INITIATOR_FLAGS_FAST_ALGO_DISABLED: disable fast algo, meaning run
// the algo on ant A+B, instead of only one of them.
// @IWL_TOF_INITIATOR_FLAGS_RX_CHAIN_SEL_A: open RX antenna A for FTMs RX
// @IWL_TOF_INITIATOR_FLAGS_RX_CHAIN_SEL_B: open RX antenna B for FTMs RX
// @IWL_TOF_INITIATOR_FLAGS_RX_CHAIN_SEL_C: open RX antenna C for FTMs RX
// @IWL_TOF_INITIATOR_FLAGS_TX_CHAIN_SEL_A: use antenna A fo TX ACKs during FTM
// @IWL_TOF_INITIATOR_FLAGS_TX_CHAIN_SEL_B: use antenna B fo TX ACKs during FTM
// @IWL_TOF_INITIATOR_FLAGS_TX_CHAIN_SEL_C: use antenna C fo TX ACKs during FTM
// @IWL_TOF_INITIATOR_FLAGS_MACADDR_RANDOM: use random mac address for FTM
// @IWL_TOF_INITIATOR_FLAGS_SPECIFIC_CALIB: use the specific calib value from
// the range request command
// @IWL_TOF_INITIATOR_FLAGS_COMMON_CALIB: use the common calib value from the
// ragne request command
// @IWL_TOF_INITIATOR_FLAGS_NON_ASAP_SUPPORT: support non-asap measurements
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_initiator_flags {
    IWL_TOF_INITIATOR_FLAGS_FAST_ALGO_DISABLED = BIT(0),
    IWL_TOF_INITIATOR_FLAGS_RX_CHAIN_SEL_A = BIT(1),
    IWL_TOF_INITIATOR_FLAGS_RX_CHAIN_SEL_B = BIT(2),
    IWL_TOF_INITIATOR_FLAGS_RX_CHAIN_SEL_C = BIT(3),
    IWL_TOF_INITIATOR_FLAGS_TX_CHAIN_SEL_A = BIT(4),
    IWL_TOF_INITIATOR_FLAGS_TX_CHAIN_SEL_B = BIT(5),
    IWL_TOF_INITIATOR_FLAGS_TX_CHAIN_SEL_C = BIT(6),
    IWL_TOF_INITIATOR_FLAGS_MACADDR_RANDOM = BIT(7),
    IWL_TOF_INITIATOR_FLAGS_SPECIFIC_CALIB = BIT(15),
    IWL_TOF_INITIATOR_FLAGS_COMMON_CALIB   = BIT(16),
    IWL_TOF_INITIATOR_FLAGS_NON_ASAP_SUPPORT = BIT(20),
}

pub const IWL_TOF_MAX_APS: c_int = 5;
pub const IWL_MVM_TOF_MAX_TWO_SIDED_APS: c_int = 5;
//
// struct iwl_tof_range_req_cmd_v5 - start measurement cmd
// @initiator_flags: see flags @ iwl_tof_initiator_flags
// @request_id: A Token incremented per request. The same Token will be
// sent back in the range response
// @initiator: 0- NW initiated,  1 - Client Initiated
// @one_sided_los_disable: '0'- run ML-Algo for both ToF/OneSided,
// '1' - run ML-Algo for ToF only
// @req_timeout: Requested timeout of the response in units of 100ms.
// This is equivalent to the session time configured to the
// LMAC in Initiator Request
// @report_policy: Supported partially for this release: For current release -
// the range report will be uploaded as a batch when ready or
// when the session is done (successfully / partially).
// one of iwl_tof_response_mode.
// @reserved0: reserved
// @num_of_ap: Number of APs to measure (error if > IWL_TOF_MAX_APS)
// @macaddr_random: '0' Use default source MAC address (i.e. p2_p),
// '1' Use MAC Address randomization according to the below
// @range_req_bssid: ranging request BSSID
// @macaddr_template: MAC address template to use for non-randomized bits
// @macaddr_mask: Bits set to 0 shall be copied from the MAC address template.
// Bits set to 1 shall be randomized by the UMAC
// @ftm_rx_chains: Rx chain to open to receive Responder's FTMs (XVT)
// @ftm_tx_chains: Tx chain to send the ack to the Responder FTM (XVT)
// @common_calib: The common calib value to inject to this measurement calc
// @specific_calib: The specific calib value to inject to this measurement calc
// @ap: per-AP request data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_cmd_v5 {
    pub initiator_flags: __le32,
    pub request_id: u8,
    pub initiator: u8,
    pub one_sided_los_disable: u8,
    pub req_timeout: u8,
    pub report_policy: u8,
    pub reserved0: u8,
    pub num_of_ap: u8,
    pub macaddr_random: u8,
    pub range_req_bssid: [u8; ETH_ALEN],
    pub macaddr_template: [u8; ETH_ALEN],
    pub macaddr_mask: [u8; ETH_ALEN],
    pub ftm_rx_chains: u8,
    pub ftm_tx_chains: u8,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub ap: [iwl_tof_range_req_ap_entry_v2; IWL_TOF_MAX_APS],
    pub __packed: },
// LOCATION_RANGE_REQ_CMD_API_S_VER_5
//
// struct iwl_tof_range_req_cmd_v7 - start measurement cmd
// @initiator_flags: see flags @ iwl_tof_initiator_flags
// @request_id: A Token incremented per request. The same Token will be
// sent back in the range response
// @num_of_ap: Number of APs to measure (error if > IWL_TOF_MAX_APS)
// @range_req_bssid: ranging request BSSID
// @macaddr_mask: Bits set to 0 shall be copied from the MAC address template.
// Bits set to 1 shall be randomized by the UMAC
// @macaddr_template: MAC address template to use for non-randomized bits
// @req_timeout_ms: Requested timeout of the response in units of milliseconds.
// This is the session time for completing the measurement.
// @tsf_mac_id: report the measurement start time for each ap in terms of the
// TSF of this mac id. 0xff to disable TSF reporting.
// @common_calib: The common calib value to inject to this measurement calc
// @specific_calib: The specific calib value to inject to this measurement calc
// @ap: per-AP request data, see &struct iwl_tof_range_req_ap_entry_v2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_cmd_v7 {
    pub initiator_flags: __le32,
    pub request_id: u8,
    pub num_of_ap: u8,
    pub range_req_bssid: [u8; ETH_ALEN],
    pub macaddr_mask: [u8; ETH_ALEN],
    pub macaddr_template: [u8; ETH_ALEN],
    pub req_timeout_ms: __le32,
    pub tsf_mac_id: __le32,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub ap: [iwl_tof_range_req_ap_entry_v3; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_CMD_API_S_VER_7,
//
// struct iwl_tof_range_req_cmd_v8 - start measurement cmd
// @initiator_flags: see flags @ iwl_tof_initiator_flags
// @request_id: A Token incremented per request. The same Token will be
// sent back in the range response
// @num_of_ap: Number of APs to measure (error if > IWL_TOF_MAX_APS)
// @range_req_bssid: ranging request BSSID
// @macaddr_mask: Bits set to 0 shall be copied from the MAC address template.
// Bits set to 1 shall be randomized by the UMAC
// @macaddr_template: MAC address template to use for non-randomized bits
// @req_timeout_ms: Requested timeout of the response in units of milliseconds.
// This is the session time for completing the measurement.
// @tsf_mac_id: report the measurement start time for each ap in terms of the
// TSF of this mac id. 0xff to disable TSF reporting.
// @common_calib: The common calib value to inject to this measurement calc
// @specific_calib: The specific calib value to inject to this measurement calc
// @ap: per-AP request data, see &struct iwl_tof_range_req_ap_entry_v2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_cmd_v8 {
    pub initiator_flags: __le32,
    pub request_id: u8,
    pub num_of_ap: u8,
    pub range_req_bssid: [u8; ETH_ALEN],
    pub macaddr_mask: [u8; ETH_ALEN],
    pub macaddr_template: [u8; ETH_ALEN],
    pub req_timeout_ms: __le32,
    pub tsf_mac_id: __le32,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub ap: [iwl_tof_range_req_ap_entry_v4; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_CMD_API_S_VER_8,
//
// struct iwl_tof_range_req_cmd_v9 - start measurement cmd
// @initiator_flags: see flags @ iwl_tof_initiator_flags
// @request_id: A Token incremented per request. The same Token will be
// sent back in the range response
// @num_of_ap: Number of APs to measure (error if > IWL_TOF_MAX_APS)
// @range_req_bssid: ranging request BSSID
// @macaddr_mask: Bits set to 0 shall be copied from the MAC address template.
// Bits set to 1 shall be randomized by the UMAC
// @macaddr_template: MAC address template to use for non-randomized bits
// @req_timeout_ms: Requested timeout of the response in units of milliseconds.
// This is the session time for completing the measurement.
// @tsf_mac_id: report the measurement start time for each ap in terms of the
// TSF of this mac id. 0xff to disable TSF reporting.
// @ap: per-AP request data, see &struct iwl_tof_range_req_ap_entry_v2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_cmd_v9 {
    pub initiator_flags: __le32,
    pub request_id: u8,
    pub num_of_ap: u8,
    pub range_req_bssid: [u8; ETH_ALEN],
    pub macaddr_mask: [u8; ETH_ALEN],
    pub macaddr_template: [u8; ETH_ALEN],
    pub req_timeout_ms: __le32,
    pub tsf_mac_id: __le32,
    pub ap: [iwl_tof_range_req_ap_entry_v6; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_CMD_API_S_VER_9,
//
// struct iwl_tof_range_req_cmd_v11 - start measurement cmd
// @initiator_flags: see flags @ iwl_tof_initiator_flags
// @request_id: A Token incremented per request. The same Token will be
// sent back in the range response
// @num_of_ap: Number of APs to measure (error if > IWL_TOF_MAX_APS)
// @range_req_bssid: ranging request BSSID
// @macaddr_mask: Bits set to 0 shall be copied from the MAC address template.
// Bits set to 1 shall be randomized by the UMAC
// @macaddr_template: MAC address template to use for non-randomized bits
// @req_timeout_ms: Requested timeout of the response in units of milliseconds.
// This is the session time for completing the measurement.
// @tsf_mac_id: report the measurement start time for each ap in terms of the
// TSF of this mac id. 0xff to disable TSF reporting.
// @ap: per-AP request data, see &struct iwl_tof_range_req_ap_entry_v2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_cmd_v11 {
    pub initiator_flags: __le32,
    pub request_id: u8,
    pub num_of_ap: u8,
    pub range_req_bssid: [u8; ETH_ALEN],
    pub macaddr_mask: [u8; ETH_ALEN],
    pub macaddr_template: [u8; ETH_ALEN],
    pub req_timeout_ms: __le32,
    pub tsf_mac_id: __le32,
    pub ap: [iwl_tof_range_req_ap_entry_v7; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_CMD_API_S_VER_11,
//
// struct iwl_tof_range_req_cmd_v12 - start measurement cmd
// @initiator_flags: see flags @ iwl_tof_initiator_flags
// @request_id: A Token incremented per request. The same Token will be
// sent back in the range response
// @num_of_ap: Number of APs to measure (error if > IWL_TOF_MAX_APS)
// @range_req_bssid: ranging request BSSID
// @macaddr_mask: Bits set to 0 shall be copied from the MAC address template.
// Bits set to 1 shall be randomized by the UMAC
// @macaddr_template: MAC address template to use for non-randomized bits
// @req_timeout_ms: Requested timeout of the response in units of milliseconds.
// This is the session time for completing the measurement.
// @tsf_mac_id: report the measurement start time for each ap in terms of the
// TSF of this mac id. 0xff to disable TSF reporting.
// @ap: per-AP request data, see &struct iwl_tof_range_req_ap_entry_v2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_cmd_v12 {
    pub initiator_flags: __le32,
    pub request_id: u8,
    pub num_of_ap: u8,
    pub range_req_bssid: [u8; ETH_ALEN],
    pub macaddr_mask: [u8; ETH_ALEN],
    pub macaddr_template: [u8; ETH_ALEN],
    pub req_timeout_ms: __le32,
    pub tsf_mac_id: __le32,
    pub ap: [iwl_tof_range_req_ap_entry_v8; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_CMD_API_S_VER_12,
//
// struct iwl_tof_range_req_cmd_v13 - start measurement cmd
// @initiator_flags: see flags @ iwl_tof_initiator_flags
// @request_id: A Token incremented per request. The same Token will be
// sent back in the range response
// @num_of_ap: Number of APs to measure (error if > IWL_TOF_MAX_APS)
// @range_req_bssid: ranging request BSSID
// @macaddr_mask: Bits set to 0 shall be copied from the MAC address template.
// Bits set to 1 shall be randomized by the UMAC
// @macaddr_template: MAC address template to use for non-randomized bits
// @req_timeout_ms: Requested timeout of the response in units of milliseconds.
// This is the session time for completing the measurement.
// @tsf_mac_id: report the measurement start time for each ap in terms of the
// TSF of this mac id. 0xff to disable TSF reporting.
// @ap: per-AP request data, see &struct iwl_tof_range_req_ap_entry_v9.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_cmd_v13 {
    pub initiator_flags: __le32,
    pub request_id: u8,
    pub num_of_ap: u8,
    pub range_req_bssid: [u8; ETH_ALEN],
    pub macaddr_mask: [u8; ETH_ALEN],
    pub macaddr_template: [u8; ETH_ALEN],
    pub req_timeout_ms: __le32,
    pub tsf_mac_id: __le32,
    pub ap: [iwl_tof_range_req_ap_entry_v9; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_CMD_API_S_VER_13,
//
// struct iwl_tof_range_req_cmd - start measurement cmd
// @initiator_flags: see flags @ iwl_tof_initiator_flags
// @request_id: A Token incremented per request. The same Token will be
// sent back in the range response
// @num_of_ap: Number of APs to measure (error if > IWL_TOF_MAX_APS)
// @range_req_bssid: ranging request BSSID
// @macaddr_mask: Bits set to 0 shall be copied from the MAC address template.
// Bits set to 1 shall be randomized by the UMAC
// @macaddr_template: MAC address template to use for non-randomized bits
// @req_timeout_ms: Requested timeout of the response in units of milliseconds.
// This is the session time for completing the measurement.
// @tsf_mac_id: report the measurement start time for each ap in terms of the
// TSF of this mac id. 0xff to disable TSF reporting.
// @ap: per-AP request data, see &struct iwl_tof_range_req_ap_entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_req_cmd {
    pub initiator_flags: __le32,
    pub request_id: u8,
    pub num_of_ap: u8,
    pub range_req_bssid: [u8; ETH_ALEN],
    pub macaddr_mask: [u8; ETH_ALEN],
    pub macaddr_template: [u8; ETH_ALEN],
    pub req_timeout_ms: __le32,
    pub tsf_mac_id: __le32,
    pub ap: [iwl_tof_range_req_ap_entry; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_REQ_CMD_API_S_VER_15,
//
// enum iwl_tof_range_request_status - status of the sent request
// @IWL_TOF_RANGE_REQUEST_STATUS_SUCCESSFUL - FW successfully received the
// request
// @IWL_TOF_RANGE_REQUEST_STATUS_BUSY - FW is busy with a previous request, the
// sent request will not be handled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_range_request_status {
    IWL_TOF_RANGE_REQUEST_STATUS_SUCCESS,
    IWL_TOF_RANGE_REQUEST_STATUS_BUSY,
}

//
// enum iwl_tof_entry_status - TOF entry status
//
// @IWL_TOF_ENTRY_SUCCESS: successful measurement.
// @IWL_TOF_ENTRY_GENERAL_FAILURE: General failure.
// @IWL_TOF_ENTRY_NO_RESPONSE: Responder didn't reply to the request.
// @IWL_TOF_ENTRY_REQUEST_REJECTED: Responder rejected the request.
// @IWL_TOF_ENTRY_NOT_SCHEDULED: Time event was scheduled but not called yet.
// @IWL_TOF_ENTRY_TIMING_MEASURE_TIMEOUT: Time event triggered but no
// measurement was completed.
// @IWL_TOF_ENTRY_TARGET_DIFF_CH_CANNOT_CHANGE: No range due inability to switch
// from the primary channel.
// @IWL_TOF_ENTRY_RANGE_NOT_SUPPORTED: Device doesn't support FTM.
// @IWL_TOF_ENTRY_REQUEST_ABORT_UNKNOWN_REASON: Request aborted due to unknown
// reason.
// @IWL_TOF_ENTRY_LOCATION_INVALID_T1_T4_TIME_STAMP: Failure due to invalid
// T1/T4.
// @IWL_TOF_ENTRY_11MC_PROTOCOL_FAILURE: Failure due to invalid FTM frame
// structure.
// @IWL_TOF_ENTRY_REQUEST_CANNOT_SCHED: Request cannot be scheduled.
// @IWL_TOF_ENTRY_RESPONDER_CANNOT_COLABORATE: Responder cannot serve the
// initiator for some period, period supplied in @refusal_period.
// @IWL_TOF_ENTRY_BAD_REQUEST_ARGS: Bad request arguments.
// @IWL_TOF_ENTRY_WIFI_NOT_ENABLED: Wifi not enabled.
// @IWL_TOF_ENTRY_RESPONDER_OVERRIDE_PARAMS: Responder override the original
// parameters within the current session.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_entry_status {
    IWL_TOF_ENTRY_SUCCESS = 0,
    IWL_TOF_ENTRY_GENERAL_FAILURE = 1,
    IWL_TOF_ENTRY_NO_RESPONSE = 2,
    IWL_TOF_ENTRY_REQUEST_REJECTED = 3,
    IWL_TOF_ENTRY_NOT_SCHEDULED = 4,
    IWL_TOF_ENTRY_TIMING_MEASURE_TIMEOUT = 5,
    IWL_TOF_ENTRY_TARGET_DIFF_CH_CANNOT_CHANGE = 6,
    IWL_TOF_ENTRY_RANGE_NOT_SUPPORTED = 7,
    IWL_TOF_ENTRY_REQUEST_ABORT_UNKNOWN_REASON = 8,
    IWL_TOF_ENTRY_LOCATION_INVALID_T1_T4_TIME_STAMP = 9,
    IWL_TOF_ENTRY_11MC_PROTOCOL_FAILURE = 10,
    IWL_TOF_ENTRY_REQUEST_CANNOT_SCHED = 11,
    IWL_TOF_ENTRY_RESPONDER_CANNOT_COLABORATE = 12,
    IWL_TOF_ENTRY_BAD_REQUEST_ARGS = 13,
    IWL_TOF_ENTRY_WIFI_NOT_ENABLED = 14,
    IWL_TOF_ENTRY_RESPONDER_OVERRIDE_PARAMS = 15,
}

//
// struct iwl_tof_range_rsp_ap_entry_ntfy_v3 - AP parameters (response)
// @bssid: BSSID of the AP
// @measure_status: current APs measurement status, one of
// &enum iwl_tof_entry_status.
// @measure_bw: Current AP Bandwidth: 0  20MHz, 1  40MHz, 2  80MHz
// @rtt: The Round Trip Time that took for the last measurement for
// current AP [pSec]
// @rtt_variance: The Variance of the RTT values measured for current AP
// @rtt_spread: The Difference between the maximum and the minimum RTT
// values measured for current AP in the current session [pSec]
// @rssi: RSSI as uploaded in the Channel Estimation notification
// @rssi_spread: The Difference between the maximum and the minimum RSSI values
// measured for current AP in the current session
// @reserved: reserved
// @refusal_period: refusal period in case of
// @IWL_TOF_ENTRY_RESPONDER_CANNOT_COLABORATE [sec]
// @range: Measured range [cm]
// @range_variance: Measured range variance [cm]
// @timestamp: The GP2 Clock [usec] where Channel Estimation notification was
// uploaded by the LMAC
// @t2t3_initiator: as calculated from the algo in the initiator
// @t1t4_responder: as calculated from the algo in the responder
// @common_calib: Calib val that was used in for this AP measurement
// @specific_calib: val that was used in for this AP measurement
// @papd_calib_output: The result of the tof papd calibration that was injected
// into the algorithm.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ap_entry_ntfy_v3 {
    pub bssid: [u8; ETH_ALEN],
    pub measure_status: u8,
    pub measure_bw: u8,
    pub rtt: __le32,
    pub rtt_variance: __le32,
    pub rtt_spread: __le32,
    pub rssi: i8,
    pub rssi_spread: u8,
    pub reserved: u8,
    pub refusal_period: u8,
    pub range: __le32,
    pub range_variance: __le32,
    pub timestamp: __le32,
    pub t2t3_initiator: __le32,
    pub t1t4_responder: __le32,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub papd_calib_output: __le32,
    pub /: *mut *mut } __packed; / LOCATION_RANGE_RSP_AP_ETRY_NTFY_API_S_VER_3,
//
// struct iwl_tof_range_rsp_ap_entry_ntfy_v4 - AP parameters (response)
// @bssid: BSSID of the AP
// @measure_status: current APs measurement status, one of
// &enum iwl_tof_entry_status.
// @measure_bw: Current AP Bandwidth: 0  20MHz, 1  40MHz, 2  80MHz
// @rtt: The Round Trip Time that took for the last measurement for
// current AP [pSec]
// @rtt_variance: The Variance of the RTT values measured for current AP
// @rtt_spread: The Difference between the maximum and the minimum RTT
// values measured for current AP in the current session [pSec]
// @rssi: RSSI as uploaded in the Channel Estimation notification
// @rssi_spread: The Difference between the maximum and the minimum RSSI values
// measured for current AP in the current session
// @last_burst: 1 if no more FTM sessions are scheduled for this responder
// @refusal_period: refusal period in case of
// @IWL_TOF_ENTRY_RESPONDER_CANNOT_COLABORATE [sec]
// @timestamp: The GP2 Clock [usec] where Channel Estimation notification was
// uploaded by the LMAC
// @start_tsf: measurement start time in TSF of the mac specified in the range
// request
// @rx_rate_n_flags: rate and flags of the last FTM frame received from this
// responder
// @tx_rate_n_flags: rate and flags of the last ack sent to this responder
// @t2t3_initiator: as calculated from the algo in the initiator
// @t1t4_responder: as calculated from the algo in the responder
// @common_calib: Calib val that was used in for this AP measurement
// @specific_calib: val that was used in for this AP measurement
// @papd_calib_output: The result of the tof papd calibration that was injected
// into the algorithm.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ap_entry_ntfy_v4 {
    pub bssid: [u8; ETH_ALEN],
    pub measure_status: u8,
    pub measure_bw: u8,
    pub rtt: __le32,
    pub rtt_variance: __le32,
    pub rtt_spread: __le32,
    pub rssi: i8,
    pub rssi_spread: u8,
    pub last_burst: u8,
    pub refusal_period: u8,
    pub timestamp: __le32,
    pub start_tsf: __le32,
    pub rx_rate_n_flags: __le32,
    pub tx_rate_n_flags: __le32,
    pub t2t3_initiator: __le32,
    pub t1t4_responder: __le32,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub papd_calib_output: __le32,
    pub /: *mut *mut } __packed; / LOCATION_RANGE_RSP_AP_ETRY_NTFY_API_S_VER_4,
//
// struct iwl_tof_range_rsp_ap_entry_ntfy_v5 - AP parameters (response)
// @bssid: BSSID of the AP
// @measure_status: current APs measurement status, one of
// &enum iwl_tof_entry_status.
// @measure_bw: Current AP Bandwidth: 0  20MHz, 1  40MHz, 2  80MHz
// @rtt: The Round Trip Time that took for the last measurement for
// current AP [pSec]
// @rtt_variance: The Variance of the RTT values measured for current AP
// @rtt_spread: The Difference between the maximum and the minimum RTT
// values measured for current AP in the current session [pSec]
// @rssi: RSSI as uploaded in the Channel Estimation notification
// @rssi_spread: The Difference between the maximum and the minimum RSSI values
// measured for current AP in the current session
// @last_burst: 1 if no more FTM sessions are scheduled for this responder
// @refusal_period: refusal period in case of
// @IWL_TOF_ENTRY_RESPONDER_CANNOT_COLABORATE [sec]
// @timestamp: The GP2 Clock [usec] where Channel Estimation notification was
// uploaded by the LMAC
// @start_tsf: measurement start time in TSF of the mac specified in the range
// request
// @rx_rate_n_flags: rate and flags of the last FTM frame received from this
// responder
// @tx_rate_n_flags: rate and flags of the last ack sent to this responder
// @t2t3_initiator: as calculated from the algo in the initiator
// @t1t4_responder: as calculated from the algo in the responder
// @common_calib: Calib val that was used in for this AP measurement
// @specific_calib: val that was used in for this AP measurement
// @papd_calib_output: The result of the tof papd calibration that was injected
// into the algorithm.
// @rttConfidence: a value between 0 - 31 that represents the rtt accuracy.
// @reserved: for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ap_entry_ntfy_v5 {
    pub bssid: [u8; ETH_ALEN],
    pub measure_status: u8,
    pub measure_bw: u8,
    pub rtt: __le32,
    pub rtt_variance: __le32,
    pub rtt_spread: __le32,
    pub rssi: i8,
    pub rssi_spread: u8,
    pub last_burst: u8,
    pub refusal_period: u8,
    pub timestamp: __le32,
    pub start_tsf: __le32,
    pub rx_rate_n_flags: __le32,
    pub tx_rate_n_flags: __le32,
    pub t2t3_initiator: __le32,
    pub t1t4_responder: __le32,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub papd_calib_output: __le32,
    pub rttConfidence: u8,
    pub reserved: [u8; 3],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_RSP_AP_ETRY_NTFY_API_S_VER_5,
//
// struct iwl_tof_range_rsp_ap_entry_ntfy_v7 - AP parameters (response)
// @bssid: BSSID of the AP
// @measure_status: current APs measurement status, one of
// &enum iwl_tof_entry_status.
// @measure_bw: Current AP Bandwidth: 0  20MHz, 1  40MHz, 2  80MHz
// @rtt: The Round Trip Time that took for the last measurement for
// current AP [pSec]
// @rtt_variance: The Variance of the RTT values measured for current AP
// @rtt_spread: The Difference between the maximum and the minimum RTT
// values measured for current AP in the current session [pSec]
// @rssi: RSSI as uploaded in the Channel Estimation notification
// @rssi_spread: The Difference between the maximum and the minimum RSSI values
// measured for current AP in the current session
// @last_burst: 1 if no more FTM sessions are scheduled for this responder
// @refusal_period: refusal period in case of
// @IWL_TOF_ENTRY_RESPONDER_CANNOT_COLABORATE [sec]
// @timestamp: The GP2 Clock [usec] where Channel Estimation notification was
// uploaded by the LMAC
// @start_tsf: measurement start time in TSF of the mac specified in the range
// request
// @rx_rate_n_flags: rate and flags of the last FTM frame received from this
// responder
// @tx_rate_n_flags: rate and flags of the last ack sent to this responder
// @t2t3_initiator: as calculated from the algo in the initiator
// @t1t4_responder: as calculated from the algo in the responder
// @common_calib: Calib val that was used in for this AP measurement
// @specific_calib: val that was used in for this AP measurement
// @papd_calib_output: The result of the tof papd calibration that was injected
// into the algorithm.
// @rttConfidence: a value between 0 - 31 that represents the rtt accuracy.
// @reserved: for alignment
// @rx_pn: the last PN used for this responder Rx in case PMF is configured in
// LE byte order.
// @tx_pn: the last PN used for this responder Tx in case PMF is configured in
// LE byte order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ap_entry_ntfy_v7 {
    pub bssid: [u8; ETH_ALEN],
    pub measure_status: u8,
    pub measure_bw: u8,
    pub rtt: __le32,
    pub rtt_variance: __le32,
    pub rtt_spread: __le32,
    pub rssi: i8,
    pub rssi_spread: u8,
    pub last_burst: u8,
    pub refusal_period: u8,
    pub timestamp: __le32,
    pub start_tsf: __le32,
    pub rx_rate_n_flags: __le32,
    pub tx_rate_n_flags: __le32,
    pub t2t3_initiator: __le32,
    pub t1t4_responder: __le32,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub papd_calib_output: __le32,
    pub rttConfidence: u8,
    pub reserved: [u8; 3],
    pub rx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub tx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub LOCATION_RANGE_RSP_AP_ETRY_NTFY_API_S_VER_6,: *mut *mut } __packed; /,
//
// struct iwl_tof_range_rsp_ap_entry_ntfy - AP parameters (response)
// @bssid: BSSID of the AP
// @measure_status: current APs measurement status, one of
// &enum iwl_tof_entry_status.
// @measure_bw: Current AP Bandwidth: 0  20MHz, 1  40MHz, 2  80MHz
// @rtt: The Round Trip Time that took for the last measurement for
// current AP [pSec]
// @rtt_variance: The Variance of the RTT values measured for current AP
// @rtt_spread: The Difference between the maximum and the minimum RTT
// values measured for current AP in the current session [pSec]
// @rssi: RSSI as uploaded in the Channel Estimation notification
// @rssi_spread: The Difference between the maximum and the minimum RSSI values
// measured for current AP in the current session
// @last_burst: 1 if no more FTM sessions are scheduled for this responder
// @refusal_period: refusal period in case of
// @IWL_TOF_ENTRY_RESPONDER_CANNOT_COLABORATE [sec]
// @timestamp: The GP2 Clock [usec] where Channel Estimation notification was
// uploaded by the LMAC
// @start_tsf: measurement start time in TSF of the mac specified in the range
// request
// @reserved1: reserved, for backwards compatibility
// @t2t3_initiator: as calculated from the algo in the initiator
// @t1t4_responder: as calculated from the algo in the responder
// @common_calib: Calib val that was used in for this AP measurement
// @specific_calib: val that was used in for this AP measurement
// @papd_calib_output: The result of the tof papd calibration that was injected
// into the algorithm.
// @rttConfidence: a value between 0 - 31 that represents the rtt accuracy.
// @reserved: for alignment
// @rx_pn: the last PN used for this responder Rx in case PMF is configured in
// LE byte order.
// @tx_pn: the last PN used for this responder Tx in case PMF is configured in
// LE byte order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ap_entry_ntfy {
    pub bssid: [u8; ETH_ALEN],
    pub measure_status: u8,
    pub measure_bw: u8,
    pub rtt: __le32,
    pub rtt_variance: __le32,
    pub rtt_spread: __le32,
    pub rssi: i8,
    pub rssi_spread: u8,
    pub last_burst: u8,
    pub refusal_period: u8,
    pub timestamp: __le32,
    pub start_tsf: __le32,
    pub reserved1: [__le32; 2],
    pub t2t3_initiator: __le32,
    pub t1t4_responder: __le32,
    pub common_calib: __le16,
    pub specific_calib: __le16,
    pub papd_calib_output: __le32,
    pub rttConfidence: u8,
    pub reserved: [u8; 3],
    pub rx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub tx_pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_RSP_AP_ETRY_NTFY_API_S_VER_8,
//
// enum iwl_tof_response_status - tof response status
//
// @IWL_TOF_RESPONSE_SUCCESS: successful range.
// @IWL_TOF_RESPONSE_TIMEOUT: request aborted due to timeout expiration.
// partial result of ranges done so far is included in the response.
// @IWL_TOF_RESPONSE_ABORTED: Measurement aborted by command.
// @IWL_TOF_RESPONSE_FAILED: Measurement request command failed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tof_response_status {
    IWL_TOF_RESPONSE_SUCCESS = 0,
    IWL_TOF_RESPONSE_TIMEOUT = 1,
    IWL_TOF_RESPONSE_ABORTED = 4,
    IWL_TOF_RESPONSE_FAILED  = 5,
}

//
// struct iwl_tof_range_rsp_ntfy_v5 - ranging response notification
// @request_id: A Token ID of the corresponding Range request
// @request_status: status of current measurement session, one of
// &enum iwl_tof_response_status.
// @last_in_batch: reprot policy (when not all responses are uploaded at once)
// @num_of_aps: Number of APs to measure (error if > IWL_TOF_MAX_APS)
// @ap: per-AP data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ntfy_v5 {
    pub request_id: u8,
    pub request_status: u8,
    pub last_in_batch: u8,
    pub num_of_aps: u8,
    pub ap: [iwl_tof_range_rsp_ap_entry_ntfy_v3; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_RSP_NTFY_API_S_VER_5,
//
// struct iwl_tof_range_rsp_ntfy_v6 - ranging response notification
// @request_id: A Token ID of the corresponding Range request
// @num_of_aps: Number of APs results
// @last_report: 1 if no more FTM sessions are scheduled, 0 otherwise.
// @reserved: reserved
// @ap: per-AP data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ntfy_v6 {
    pub request_id: u8,
    pub num_of_aps: u8,
    pub last_report: u8,
    pub reserved: u8,
    pub ap: [iwl_tof_range_rsp_ap_entry_ntfy_v4; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_RSP_NTFY_API_S_VER_6,
//
// struct iwl_tof_range_rsp_ntfy_v7 - ranging response notification
// @request_id: A Token ID of the corresponding Range request
// @num_of_aps: Number of APs results
// @last_report: 1 if no more FTM sessions are scheduled, 0 otherwise.
// @reserved: reserved
// @ap: per-AP data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ntfy_v7 {
    pub request_id: u8,
    pub num_of_aps: u8,
    pub last_report: u8,
    pub reserved: u8,
    pub ap: [iwl_tof_range_rsp_ap_entry_ntfy_v5; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_RSP_NTFY_API_S_VER_7,
//
// struct iwl_tof_range_rsp_ntfy_v9 - ranging response notification
// @request_id: A Token ID of the corresponding Range request
// @num_of_aps: Number of APs results
// @last_report: 1 if no more FTM sessions are scheduled, 0 otherwise.
// @reserved: reserved
// @ap: per-AP data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ntfy_v9 {
    pub request_id: u8,
    pub num_of_aps: u8,
    pub last_report: u8,
    pub reserved: u8,
    pub ap: [iwl_tof_range_rsp_ap_entry_ntfy_v7; IWL_TOF_MAX_APS],
    pub LOCATION_RANGE_RSP_NTFY_API_S_VER_8,: *mut *mut } __packed; /,
// LOCATION_RANGE_RSP_NTFY_API_S_VER_9
//
// struct iwl_tof_range_rsp_ntfy - ranging response notification
// @request_id: A Token ID of the corresponding Range request
// @num_of_aps: Number of APs results
// @last_report: 1 if no more FTM sessions are scheduled, 0 otherwise.
// @reserved: reserved
// @ap: per-AP data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_rsp_ntfy {
    pub request_id: u8,
    pub num_of_aps: u8,
    pub last_report: u8,
    pub reserved: u8,
    pub ap: [iwl_tof_range_rsp_ap_entry_ntfy; IWL_TOF_MAX_APS],
    pub /: *mut *mut } __packed; / LOCATION_RANGE_RSP_NTFY_API_S_VER_10,

//
// struct iwl_tof_mcsi_notif - used for debug
// @token: token ID for the current session
// @role: '0' - initiator, '1' - responder
// @reserved: reserved
// @initiator_bssid: initiator machine
// @responder_bssid: responder machine
// @mcsi_buffer: debug data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_mcsi_notif {
    pub token: u8,
    pub role: u8,
    pub reserved: __le16,
    pub initiator_bssid: [u8; ETH_ALEN],
    pub responder_bssid: [u8; ETH_ALEN],
    pub 4]: *mut *mut u8 mcsi_buffer[IWL_MVM_TOF_MCSI_BUF_SIZE,
    pub __packed: },
//
// struct iwl_tof_range_abort_cmd - TOF range abort command
// @request_id: corresponds to a range request
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tof_range_abort_cmd {
    pub request_id: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ftm_responder_stats_flags {
    FTM_RESP_STAT_NON_ASAP_STARTED = BIT(0),
    FTM_RESP_STAT_NON_ASAP_IN_WIN = BIT(1),
    FTM_RESP_STAT_NON_ASAP_OUT_WIN = BIT(2),
    FTM_RESP_STAT_TRIGGER_DUP = BIT(3),
    FTM_RESP_STAT_DUP = BIT(4),
    FTM_RESP_STAT_DUP_IN_WIN = BIT(5),
    FTM_RESP_STAT_DUP_OUT_WIN = BIT(6),
    FTM_RESP_STAT_SCHED_SUCCESS = BIT(7),
    FTM_RESP_STAT_ASAP_REQ = BIT(8),
    FTM_RESP_STAT_NON_ASAP_REQ = BIT(9),
    FTM_RESP_STAT_ASAP_RESP = BIT(10),
    FTM_RESP_STAT_NON_ASAP_RESP = BIT(11),
    FTM_RESP_STAT_FAIL_INITIATOR_INACTIVE = BIT(12),
    FTM_RESP_STAT_FAIL_INITIATOR_OUT_WIN = BIT(13),
    FTM_RESP_STAT_FAIL_INITIATOR_RETRY_LIM = BIT(14),
    FTM_RESP_STAT_FAIL_NEXT_SERVED = BIT(15),
    FTM_RESP_STAT_FAIL_TRIGGER_ERR = BIT(16),
    FTM_RESP_STAT_FAIL_GC = BIT(17),
    FTM_RESP_STAT_SUCCESS = BIT(18),
    FTM_RESP_STAT_INTEL_IE = BIT(19),
    FTM_RESP_STAT_INITIATOR_ACTIVE = BIT(20),
    FTM_RESP_STAT_MEASUREMENTS_AVAILABLE = BIT(21),
    FTM_RESP_STAT_TRIGGER_UNKNOWN = BIT(22),
    FTM_RESP_STAT_PROCESS_FAIL = BIT(23),
    FTM_RESP_STAT_ACK = BIT(24),
    FTM_RESP_STAT_NACK = BIT(25),
    FTM_RESP_STAT_INVALID_INITIATOR_ID = BIT(26),
    FTM_RESP_STAT_TIMER_MIN_DELTA = BIT(27),
    FTM_RESP_STAT_INITIATOR_REMOVED = BIT(28),
    FTM_RESP_STAT_INITIATOR_ADDED = BIT(29),
    FTM_RESP_STAT_ERR_LIST_FULL = BIT(30),
    FTM_RESP_STAT_INITIATOR_SCHED_NOW = BIT(31),
}

//
// struct iwl_ftm_responder_stats - FTM responder statistics
// @addr: initiator address
// @success_ftm: number of successful ftm frames
// @ftm_per_burst: num of FTM frames that were received
// @flags: &enum ftm_responder_stats_flags
// @duration: actual duration of FTM
// @allocated_duration: time that was allocated for this FTM session
// @bw: FTM request bandwidth
// @rate: FTM request rate
// @reserved: for alingment and future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ftm_responder_stats {
    pub addr: [u8; ETH_ALEN],
    pub success_ftm: u8,
    pub ftm_per_burst: u8,
    pub flags: __le32,
    pub duration: __le32,
    pub allocated_duration: __le32,
    pub bw: u8,
    pub rate: u8,
    pub reserved: __le16,
    pub /: *mut *mut } __packed; / TOF_RESPONDER_STATISTICS_NTFY_S_VER_2,
pub const IWL_CSI_MAX_EXPECTED_CHUNKS: c_int = 16;
pub const IWL_CSI_CHUNK_CTL_NUM_MASK_VER_1: c_uint = 0x0003;
pub const IWL_CSI_CHUNK_CTL_IDX_MASK_VER_1: c_uint = 0x000c;
pub const IWL_CSI_CHUNK_CTL_NUM_MASK_VER_2: c_uint = 0x00ff;
pub const IWL_CSI_CHUNK_CTL_IDX_MASK_VER_2: c_uint = 0xff00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_csi_chunk_notification {
    pub token: __le32,
    pub seq: __le16,
    pub ctl: __le16,
    pub size: __le32,
    pub data: [u8; ],
    pub /: *mut *mut } __packed; / CSI_CHUNKS_HDR_NTFY_API_S_VER_1/VER_2,
