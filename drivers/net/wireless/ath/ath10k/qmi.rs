//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/qmi.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2018 The Linux Foundation. All rights reserved.
//

pub const MAX_NUM_MEMORY_REGIONS: c_int = 2;
pub const MAX_TIMESTAMP_LEN: c_int = 32;
pub const MAX_BUILD_ID_LEN: c_int = 128;
pub const MAX_NUM_CAL_V01: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_qmi_driver_event_type {
    ATH10K_QMI_EVENT_SERVER_ARRIVE,
    ATH10K_QMI_EVENT_SERVER_EXIT,
    ATH10K_QMI_EVENT_FW_READY_IND,
    ATH10K_QMI_EVENT_FW_DOWN_IND,
    ATH10K_QMI_EVENT_MSA_READY_IND,
    ATH10K_QMI_EVENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_msa_mem_info {
    pub addr: phys_addr_t,
    pub size: u32,
    pub secure: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_qmi_chip_info {
    pub chip_id: u32,
    pub chip_family: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_qmi_board_info {
    pub board_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_qmi_soc_info {
    pub soc_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_qmi_cal_data {
    pub cal_id: u32,
    pub total_size: u32,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_tgt_pipe_cfg {
    pub pipe_num: __le32,
    pub pipe_dir: __le32,
    pub nentries: __le32,
    pub nbytes_max: __le32,
    pub flags: __le32,
    pub reserved: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_svc_pipe_cfg {
    pub service_id: __le32,
    pub pipe_dir: __le32,
    pub pipe_num: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_shadow_reg_cfg {
    pub ce_id: __le16,
    pub reg_offset: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_qmi_wlan_enable_cfg {
    pub num_ce_tgt_cfg: u32,
    pub ce_tgt_cfg: *mut ath10k_tgt_pipe_cfg,
    pub num_ce_svc_pipe_cfg: u32,
    pub ce_svc_cfg: *mut ath10k_svc_pipe_cfg,
    pub num_shadow_reg_cfg: u32,
    pub shadow_reg_cfg: *mut ath10k_shadow_reg_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_qmi_driver_event {
    pub list: list_head,
    pub type: ath10k_qmi_driver_event_type,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_qmi_state {
    ATH10K_QMI_STATE_INIT_DONE,
    ATH10K_QMI_STATE_DEINIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_qmi {
    pub ar: *mut ath10k,
    pub qmi_hdl: qmi_handle,
    pub sq: sockaddr_qrtr,
    pub event_work: work_struct,
    pub event_wq: *mut workqueue_struct,
    pub event_list: list_head,
    pub /: *mut *mut spinlock_t event_lock; / spinlock for qmi event list,
    pub nr_mem_region: u32,
    pub mem_region: [ath10k_msa_mem_info; MAX_NUM_MEMORY_REGIONS],
    pub chip_info: ath10k_qmi_chip_info,
    pub board_info: ath10k_qmi_board_info,
    pub soc_info: ath10k_qmi_soc_info,
    pub 1]: char fw_build_id[MAX_BUILD_ID_LEN +,
    pub fw_version: u32,
    pub fw_ready: bool,
    pub 1]: char fw_build_timestamp[MAX_TIMESTAMP_LEN +,
    pub cal_data: [ath10k_qmi_cal_data; MAX_NUM_CAL_V01],
    pub msa_fixed_perm: bool,
    pub no_msa_ready_indicator: bool,
    pub state: ath10k_qmi_state,
}

extern "C" {
    pub fn ath10k_qmi_wlan_disable(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_qmi_init(ar: *mut ath10k, msa_size: u32) -> c_int;
}
extern "C" {
    pub fn ath10k_qmi_deinit(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_qmi_set_fw_log_mode(ar: *mut ath10k, fw_log_mode: u8) -> c_int;
}
