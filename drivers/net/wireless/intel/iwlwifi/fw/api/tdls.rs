//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/tdls.h
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
// Copyright (C) 2012-2014, 2018, 2024-2025 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_tdls_h__

pub const IWL_TDLS_STA_COUNT: c_int = 4;
// Type of TDLS request
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tdls_channel_switch_type {
    TDLS_SEND_CHAN_SW_REQ = 0,
    TDLS_SEND_CHAN_SW_RESP_AND_MOVE_CH,
    TDLS_MOVE_CH,
}

//
// struct iwl_tdls_channel_switch_timing - Switch timing in TDLS channel-switch
// @frame_timestamp: GP2 timestamp of channel-switch request/response packet
// received from peer
// @max_offchan_duration: What amount of microseconds out of a DTIM is given
// to the TDLS off-channel communication. For instance if the DTIM is
// 200TU and the TDLS peer is to be given 25% of the time, the value
// given will be 50TU, or 50 * 1024 if translated into microseconds.
// @switch_time: switch time the peer sent in its channel switch timing IE
// @switch_timeout: switch timeout the peer sent in its channel switch timing IE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tdls_channel_switch_timing {
    pub /: *mut *mut __le32 frame_timestamp; / GP2 time of peer packet Rx,
    pub /: *mut *mut __le32 max_offchan_duration; / given in micro-seconds,
    pub /: *mut *mut __le32 switch_time; / given in micro-seconds,
    pub /: *mut *mut __le32 switch_timeout; / given in micro-seconds,
    pub /: *mut *mut } __packed; / TDLS_STA_CHANNEL_SWITCH_TIMING_DATA_API_S_VER_1,
pub const IWL_TDLS_CH_SW_FRAME_MAX_SIZE: c_int = 200;
//
// struct iwl_tdls_channel_switch_frame - TDLS channel switch frame template
//
// A template representing a TDLS channel-switch request or response frame
//
// @switch_time_offset: offset to the channel switch timing IE in the template
// @tx_cmd: Tx parameters for the frame
// @data: frame data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tdls_channel_switch_frame {
    pub switch_time_offset: __le32,
    pub tx_cmd: iwl_tx_cmd_v6_params,
    pub data: [u8; IWL_TDLS_CH_SW_FRAME_MAX_SIZE],
    pub /: *mut *mut } __packed; / TDLS_STA_CHANNEL_SWITCH_FRAME_API_S_VER_1,
//
// struct iwl_tdls_channel_switch_cmd_tail - tail of iwl_tdls_channel_switch_cmd
//
// @timing: timing related data for command
// @frame: channel-switch request/response template, depending to switch_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tdls_channel_switch_cmd_tail {
    pub timing: iwl_tdls_channel_switch_timing,
    pub frame: iwl_tdls_channel_switch_frame,
    pub __packed: },
//
// struct iwl_tdls_channel_switch_cmd - TDLS channel switch command
//
// The command is sent to initiate a channel switch and also in response to
// incoming TDLS channel-switch request/response packets from remote peers.
//
// @switch_type: see &enum iwl_tdls_channel_switch_type
// @peer_sta_id: station id of TDLS peer
// @ci: channel we switch to
// @tail: command tail
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tdls_channel_switch_cmd {
    pub switch_type: u8,
    pub peer_sta_id: __le32,
    pub ci: iwl_fw_channel_info,
    pub tail: iwl_tdls_channel_switch_cmd_tail,
    pub /: *mut *mut } __packed; / TDLS_STA_CHANNEL_SWITCH_CMD_API_S_VER_1,
//
// struct iwl_tdls_channel_switch_notif - TDLS channel switch start notification
//
// @status: non-zero on success
// @offchannel_duration: duration given in microseconds
// @sta_id: peer currently performing the channel-switch with
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tdls_channel_switch_notif {
    pub status: __le32,
    pub offchannel_duration: __le32,
    pub sta_id: __le32,
    pub /: *mut *mut } __packed; / TDLS_STA_CHANNEL_SWITCH_NTFY_API_S_VER_1,
//
// struct iwl_tdls_sta_info - TDLS station info
//
// @sta_id: station id of the TDLS peer
// @tx_to_peer_tid: TID reserved vs. the peer for FW based Tx
// @tx_to_peer_ssn: initial SSN the FW should use for Tx on its TID vs the peer
// @is_initiator: 1 if the peer is the TDLS link initiator, 0 otherwise
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tdls_sta_info {
    pub sta_id: u8,
    pub tx_to_peer_tid: u8,
    pub tx_to_peer_ssn: __le16,
    pub is_initiator: __le32,
    pub /: *mut *mut } __packed; / TDLS_STA_INFO_VER_1,
//
// struct iwl_tdls_config_cmd - TDLS basic config command
//
// @id_and_color: MAC id and color being configured
// @tdls_peer_count: amount of currently connected TDLS peers
// @tx_to_ap_tid: TID reverved vs. the AP for FW based Tx
// @tx_to_ap_ssn: initial SSN the FW should use for Tx on its TID vs. the AP
// @sta_info: per-station info. Only the first tdls_peer_count entries are set
// @pti_req_data_offset: offset of network-level data for the PTI template
// @pti_req_tx_cmd: Tx parameters for PTI request template
// @pti_req_template: PTI request template data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tdls_config_cmd {
    pub /: *mut *mut __le32 id_and_color; / mac id and color,
    pub tdls_peer_count: u8,
    pub tx_to_ap_tid: u8,
    pub tx_to_ap_ssn: __le16,
    pub sta_info: [iwl_tdls_sta_info; IWL_TDLS_STA_COUNT],
    pub pti_req_data_offset: __le32,
    pub pti_req_tx_cmd: iwl_tx_cmd_v6_params,
    pub pti_req_template: [u8; ],
    pub /: *mut *mut } __packed; / TDLS_CONFIG_CMD_API_S_VER_1,
//
// struct iwl_tdls_config_sta_info_res - TDLS per-station config information
//
// @sta_id: station id of the TDLS peer
// @tx_to_peer_last_seq: last sequence number used by FW during FW-based Tx to
// the peer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tdls_config_sta_info_res {
    pub sta_id: __le16,
    pub tx_to_peer_last_seq: __le16,
    pub /: *mut *mut } __packed; / TDLS_STA_INFO_RSP_VER_1,
//
// struct iwl_tdls_config_res - TDLS config information from FW
//
// @tx_to_ap_last_seq: last sequence number used by FW during FW-based Tx to AP
// @sta_info: per-station TDLS config information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tdls_config_res {
    pub tx_to_ap_last_seq: __le32,
    pub sta_info: [iwl_tdls_config_sta_info_res; IWL_TDLS_STA_COUNT],
    pub /: *mut *mut } __packed; / TDLS_CONFIG_RSP_API_S_VER_1,
