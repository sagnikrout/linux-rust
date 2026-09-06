//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/txq.h
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
// Copyright (C) 2005-2014, 2019-2021, 2023-2024 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_txq_h__
//
// DQA queue numbers
//
// @IWL_MVM_DQA_CMD_QUEUE: a queue reserved for sending HCMDs to the FW
// @IWL_MVM_DQA_AUX_QUEUE: a queue reserved for aux frames
// @IWL_MVM_DQA_P2P_DEVICE_QUEUE: a queue reserved for P2P device frames
// @IWL_MVM_DQA_INJECT_MONITOR_QUEUE: a queue reserved for injection using
// monitor mode. Note this queue is the same as the queue for P2P device
// but we can't have active monitor mode along with P2P device anyway.
// @IWL_MVM_DQA_GCAST_QUEUE: a queue reserved for P2P GO/SoftAP GCAST frames
// @IWL_MVM_DQA_BSS_CLIENT_QUEUE: a queue reserved for BSS activity, to ensure
// that we are never left without the possibility to connect to an AP.
// @IWL_MVM_DQA_MIN_MGMT_QUEUE: first TXQ in pool for MGMT and non-QOS frames.
// Each MGMT queue is mapped to a single STA
// MGMT frames are frames that return true on ieee80211_is_mgmt()
// @IWL_MVM_DQA_MAX_MGMT_QUEUE: last TXQ in pool for MGMT frames
// @IWL_MVM_DQA_AP_PROBE_RESP_QUEUE: a queue reserved for P2P GO/SoftAP probe
// responses
// @IWL_MVM_DQA_MIN_DATA_QUEUE: first TXQ in pool for DATA frames.
// DATA frames are intended for !ieee80211_is_mgmt() frames, but if
// the MGMT TXQ pool is exhausted, mgmt frames can be sent on DATA queues
// as well
// @IWL_MVM_DQA_MAX_DATA_QUEUE: last TXQ in pool for DATA frames
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_dqa_txq {
    IWL_MVM_DQA_CMD_QUEUE = 0,
    IWL_MVM_DQA_AUX_QUEUE = 1,
    IWL_MVM_DQA_P2P_DEVICE_QUEUE = 2,
    IWL_MVM_DQA_INJECT_MONITOR_QUEUE = 2,
    IWL_MVM_DQA_GCAST_QUEUE = 3,
    IWL_MVM_DQA_BSS_CLIENT_QUEUE = 4,
    IWL_MVM_DQA_MIN_MGMT_QUEUE = 5,
    IWL_MVM_DQA_MAX_MGMT_QUEUE = 8,
    IWL_MVM_DQA_AP_PROBE_RESP_QUEUE = 9,
    IWL_MVM_DQA_MIN_DATA_QUEUE = 10,
    IWL_MVM_DQA_MAX_DATA_QUEUE = 30,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_tx_fifo {
    IWL_MVM_TX_FIFO_BK = 0,
    IWL_MVM_TX_FIFO_BE,
    IWL_MVM_TX_FIFO_VI,
    IWL_MVM_TX_FIFO_VO,
    IWL_MVM_TX_FIFO_MCAST = 5,
    IWL_MVM_TX_FIFO_CMD = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_gen2_tx_fifo {
    IWL_GEN2_TX_FIFO_CMD = 0,
    IWL_GEN2_EDCA_TX_FIFO_BK,
    IWL_GEN2_EDCA_TX_FIFO_BE,
    IWL_GEN2_EDCA_TX_FIFO_VI,
    IWL_GEN2_EDCA_TX_FIFO_VO,
    IWL_GEN2_TRIG_TX_FIFO_BK,
    IWL_GEN2_TRIG_TX_FIFO_BE,
    IWL_GEN2_TRIG_TX_FIFO_VI,
    IWL_GEN2_TRIG_TX_FIFO_VO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bz_tx_fifo {
    IWL_BZ_EDCA_TX_FIFO_BK,
    IWL_BZ_EDCA_TX_FIFO_BE,
    IWL_BZ_EDCA_TX_FIFO_VI,
    IWL_BZ_EDCA_TX_FIFO_VO,
    IWL_BZ_TRIG_TX_FIFO_BK,
    IWL_BZ_TRIG_TX_FIFO_BE,
    IWL_BZ_TRIG_TX_FIFO_VI,
    IWL_BZ_TRIG_TX_FIFO_VO,
}

//
// enum iwl_tx_queue_cfg_actions - TXQ config options
// @TX_QUEUE_CFG_ENABLE_QUEUE: enable a queue
// @TX_QUEUE_CFG_TFD_SHORT_FORMAT: use short TFD format
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tx_queue_cfg_actions {
    TX_QUEUE_CFG_ENABLE_QUEUE		= BIT(0),
    TX_QUEUE_CFG_TFD_SHORT_FORMAT		= BIT(1),
}

pub const IWL_DEFAULT_QUEUE_SIZE_HE: c_int = 1024;
pub const IWL_DEFAULT_QUEUE_SIZE: c_int = 256;
pub const IWL_MGMT_QUEUE_SIZE: c_int = 16;
pub const IWL_CMD_QUEUE_SIZE: c_int = 32;
//
// struct iwl_tx_queue_cfg_cmd - txq hw scheduler config command
// @sta_id: station id
// @tid: tid of the queue
// @flags: see &enum iwl_tx_queue_cfg_actions
// @cb_size: size of TFD cyclic buffer. Value is exponent - 3.
// Minimum value 0 (8 TFDs), maximum value 5 (256 TFDs)
// @byte_cnt_addr: address of byte count table
// @tfdq_addr: address of TFD circular buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tx_queue_cfg_cmd {
    pub sta_id: u8,
    pub tid: u8,
    pub flags: __le16,
    pub cb_size: __le32,
    pub byte_cnt_addr: __le64,
    pub tfdq_addr: __le64,
    pub /: *mut *mut } __packed; / TX_QUEUE_CFG_CMD_API_S_VER_2,
//
// struct iwl_tx_queue_cfg_rsp - response to txq hw scheduler config
// @queue_number: queue number assigned to this RA -TID
// @flags: set on failure
// @write_pointer: initial value for write pointer
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tx_queue_cfg_rsp {
    pub queue_number: __le16,
    pub flags: __le16,
    pub write_pointer: __le16,
    pub reserved: __le16,
    pub /: *mut *mut } __packed; / TX_QUEUE_CFG_RSP_API_S_VER_2,
