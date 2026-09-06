//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/qmi_wlfw_v01.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
pub const WLFW_SERVICE_VERS_V01: c_uint = 0x01;
pub const QMI_WLFW_BDF_DOWNLOAD_REQ_V01: c_uint = 0x0025;
pub const QMI_WLFW_MEM_READY_IND_V01: c_uint = 0x0037;
pub const QMI_WLFW_DYNAMIC_FEATURE_MASK_RESP_V01: c_uint = 0x003B;
pub const QMI_WLFW_INITIATE_CAL_UPDATE_IND_V01: c_uint = 0x002A;
pub const QMI_WLFW_HOST_CAP_REQ_V01: c_uint = 0x0034;
pub const QMI_WLFW_M3_INFO_REQ_V01: c_uint = 0x003C;
pub const QMI_WLFW_CAP_REQ_V01: c_uint = 0x0024;
pub const QMI_WLFW_FW_INIT_DONE_IND_V01: c_uint = 0x0038;
pub const QMI_WLFW_CAL_REPORT_REQ_V01: c_uint = 0x0026;
pub const QMI_WLFW_M3_INFO_RESP_V01: c_uint = 0x003C;
pub const QMI_WLFW_CAL_UPDATE_RESP_V01: c_uint = 0x0029;
pub const QMI_WLFW_CAL_DOWNLOAD_RESP_V01: c_uint = 0x0027;
pub const QMI_WLFW_XO_CAL_IND_V01: c_uint = 0x003D;
pub const QMI_WLFW_INI_RESP_V01: c_uint = 0x002F;
pub const QMI_WLFW_CAL_REPORT_RESP_V01: c_uint = 0x0026;
pub const QMI_WLFW_MAC_ADDR_RESP_V01: c_uint = 0x0033;
pub const QMI_WLFW_INITIATE_CAL_DOWNLOAD_IND_V01: c_uint = 0x0028;
pub const QMI_WLFW_HOST_CAP_RESP_V01: c_uint = 0x0034;
pub const QMI_WLFW_MSA_READY_IND_V01: c_uint = 0x002B;
pub const QMI_WLFW_ATHDIAG_WRITE_RESP_V01: c_uint = 0x0031;
pub const QMI_WLFW_WLAN_MODE_REQ_V01: c_uint = 0x0022;
pub const QMI_WLFW_IND_REGISTER_REQ_V01: c_uint = 0x0020;
pub const QMI_WLFW_WLAN_CFG_RESP_V01: c_uint = 0x0023;
pub const QMI_WLFW_REQUEST_MEM_IND_V01: c_uint = 0x0035;
pub const QMI_WLFW_REJUVENATE_IND_V01: c_uint = 0x0039;
pub const QMI_WLFW_DYNAMIC_FEATURE_MASK_REQ_V01: c_uint = 0x003B;
pub const QMI_WLFW_ATHDIAG_WRITE_REQ_V01: c_uint = 0x0031;
pub const QMI_WLFW_WLAN_MODE_RESP_V01: c_uint = 0x0022;
pub const QMI_WLFW_RESPOND_MEM_REQ_V01: c_uint = 0x0036;
pub const QMI_WLFW_PIN_CONNECT_RESULT_IND_V01: c_uint = 0x002C;
pub const QMI_WLFW_FW_READY_IND_V01: c_uint = 0x0021;
pub const QMI_WLFW_MSA_READY_RESP_V01: c_uint = 0x002E;
pub const QMI_WLFW_CAL_UPDATE_REQ_V01: c_uint = 0x0029;
pub const QMI_WLFW_INI_REQ_V01: c_uint = 0x002F;
pub const QMI_WLFW_BDF_DOWNLOAD_RESP_V01: c_uint = 0x0025;
pub const QMI_WLFW_REJUVENATE_ACK_RESP_V01: c_uint = 0x003A;
pub const QMI_WLFW_MSA_INFO_RESP_V01: c_uint = 0x002D;
pub const QMI_WLFW_MSA_READY_REQ_V01: c_uint = 0x002E;
pub const QMI_WLFW_CAP_RESP_V01: c_uint = 0x0024;
pub const QMI_WLFW_REJUVENATE_ACK_REQ_V01: c_uint = 0x003A;
pub const QMI_WLFW_ATHDIAG_READ_RESP_V01: c_uint = 0x0030;
pub const QMI_WLFW_VBATT_REQ_V01: c_uint = 0x0032;
pub const QMI_WLFW_MAC_ADDR_REQ_V01: c_uint = 0x0033;
pub const QMI_WLFW_RESPOND_MEM_RESP_V01: c_uint = 0x0036;
pub const QMI_WLFW_VBATT_RESP_V01: c_uint = 0x0032;
pub const QMI_WLFW_MSA_INFO_REQ_V01: c_uint = 0x002D;
pub const QMI_WLFW_CAL_DOWNLOAD_REQ_V01: c_uint = 0x0027;
pub const QMI_WLFW_ATHDIAG_READ_REQ_V01: c_uint = 0x0030;
pub const QMI_WLFW_WLAN_CFG_REQ_V01: c_uint = 0x0023;
pub const QMI_WLFW_IND_REGISTER_RESP_V01: c_uint = 0x0020;
pub const QMI_WLFW_MAX_MEM_REG_V01: c_int = 2;
pub const QMI_WLFW_MAX_NUM_MEM_SEG_V01: c_int = 16;
pub const QMI_WLFW_MAX_NUM_CAL_V01: c_int = 5;
pub const QMI_WLFW_MAX_DATA_SIZE_V01: c_int = 6144;
pub const QMI_WLFW_FUNCTION_NAME_LEN_V01: c_int = 128;
pub const QMI_WLFW_MAX_NUM_CE_V01: c_int = 12;
pub const QMI_WLFW_MAX_TIMESTAMP_LEN_V01: c_int = 32;
pub const QMI_WLFW_MAX_ATHDIAG_DATA_SIZE_V01: c_int = 6144;
pub const QMI_WLFW_MAX_NUM_GPIO_V01: c_int = 32;
pub const QMI_WLFW_MAX_BUILD_ID_LEN_V01: c_int = 128;
pub const QMI_WLFW_MAX_NUM_MEM_CFG_V01: c_int = 2;
pub const QMI_WLFW_MAX_STR_LEN_V01: c_int = 16;
pub const QMI_WLFW_MAX_NUM_SHADOW_REG_V01: c_int = 24;
pub const QMI_WLFW_MAC_ADDR_SIZE_V01: c_int = 6;
pub const QMI_WLFW_MAX_SHADOW_REG_V2: c_int = 36;
pub const QMI_WLFW_MAX_NUM_SVC_V01: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlfw_driver_mode_enum_v01 {
    QMI_WLFW_MISSION_V01 = 0,
    QMI_WLFW_FTM_V01 = 1,
    QMI_WLFW_EPPING_V01 = 2,
    QMI_WLFW_WALTEST_V01 = 3,
    QMI_WLFW_OFF_V01 = 4,
    QMI_WLFW_CCPM_V01 = 5,
    QMI_WLFW_QVIT_V01 = 6,
    QMI_WLFW_CALIBRATION_V01 = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlfw_cal_temp_id_enum_v01 {
    QMI_WLFW_CAL_TEMP_IDX_0_V01 = 0,
    QMI_WLFW_CAL_TEMP_IDX_1_V01 = 1,
    QMI_WLFW_CAL_TEMP_IDX_2_V01 = 2,
    QMI_WLFW_CAL_TEMP_IDX_3_V01 = 3,
    QMI_WLFW_CAL_TEMP_IDX_4_V01 = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlfw_pipedir_enum_v01 {
    QMI_WLFW_PIPEDIR_NONE_V01 = 0,
    QMI_WLFW_PIPEDIR_IN_V01 = 1,
    QMI_WLFW_PIPEDIR_OUT_V01 = 2,
    QMI_WLFW_PIPEDIR_INOUT_V01 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlfw_mem_type_enum_v01 {
    QMI_WLFW_MEM_TYPE_MSA_V01 = 0,
    QMI_WLFW_MEM_TYPE_DDR_V01 = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_ce_tgt_pipe_cfg_s_v01 {
    pub pipe_num: __le32,
    pub pipe_dir: __le32,
    pub nentries: __le32,
    pub nbytes_max: __le32,
    pub flags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_ce_svc_pipe_cfg_s_v01 {
    pub service_id: __le32,
    pub pipe_dir: __le32,
    pub pipe_num: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_shadow_reg_cfg_s_v01 {
    pub id: u16,
    pub offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_shadow_reg_v2_cfg_s_v01 {
    pub addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_memory_region_info_s_v01 {
    pub region_addr: u64,
    pub size: u32,
    pub secure_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_mem_cfg_s_v01 {
    pub offset: u64,
    pub size: u32,
    pub secure_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_mem_seg_s_v01 {
    pub size: u32,
    pub type: wlfw_mem_type_enum_v01,
    pub mem_cfg_len: u32,
    pub mem_cfg: [wlfw_mem_cfg_s_v01; QMI_WLFW_MAX_NUM_MEM_CFG_V01],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_mem_seg_resp_s_v01 {
    pub addr: u64,
    pub size: u32,
    pub type: wlfw_mem_type_enum_v01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_rf_chip_info_s_v01 {
    pub chip_id: u32,
    pub chip_family: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_rf_board_info_s_v01 {
    pub board_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_soc_info_s_v01 {
    pub soc_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_fw_version_info_s_v01 {
    pub fw_version: u32,
    pub 1]: char fw_build_timestamp[QMI_WLFW_MAX_TIMESTAMP_LEN_V01 +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_ind_register_req_msg_v01 {
    pub fw_ready_enable_valid: u8,
    pub fw_ready_enable: u8,
    pub initiate_cal_download_enable_valid: u8,
    pub initiate_cal_download_enable: u8,
    pub initiate_cal_update_enable_valid: u8,
    pub initiate_cal_update_enable: u8,
    pub msa_ready_enable_valid: u8,
    pub msa_ready_enable: u8,
    pub pin_connect_result_enable_valid: u8,
    pub pin_connect_result_enable: u8,
    pub client_id_valid: u8,
    pub client_id: u32,
    pub request_mem_enable_valid: u8,
    pub request_mem_enable: u8,
    pub mem_ready_enable_valid: u8,
    pub mem_ready_enable: u8,
    pub fw_init_done_enable_valid: u8,
    pub fw_init_done_enable: u8,
    pub rejuvenate_enable_valid: u8,
    pub rejuvenate_enable: u32,
    pub xo_cal_enable_valid: u8,
    pub xo_cal_enable: u8,
}

pub const WLFW_IND_REGISTER_REQ_MSG_V01_MAX_MSG_LEN: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_ind_register_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub fw_status_valid: u8,
    pub fw_status: u64,
}

pub const WLFW_IND_REGISTER_RESP_MSG_V01_MAX_MSG_LEN: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_fw_ready_ind_msg_v01 {
    pub placeholder: c_char,
}

pub const WLFW_FW_READY_IND_MSG_V01_MAX_MSG_LEN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_msa_ready_ind_msg_v01 {
    pub placeholder: c_char,
}

pub const WLFW_MSA_READY_IND_MSG_V01_MAX_MSG_LEN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_pin_connect_result_ind_msg_v01 {
    pub pwr_pin_result_valid: u8,
    pub pwr_pin_result: u32,
    pub phy_io_pin_result_valid: u8,
    pub phy_io_pin_result: u32,
    pub rf_pin_result_valid: u8,
    pub rf_pin_result: u32,
}

pub const WLFW_PIN_CONNECT_RESULT_IND_MSG_V01_MAX_MSG_LEN: c_int = 21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_wlan_mode_req_msg_v01 {
    pub mode: wlfw_driver_mode_enum_v01,
    pub hw_debug_valid: u8,
    pub hw_debug: u8,
}

pub const WLFW_WLAN_MODE_REQ_MSG_V01_MAX_MSG_LEN: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_wlan_mode_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_WLAN_MODE_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_wlan_cfg_req_msg_v01 {
    pub host_version_valid: u8,
    pub 1]: char host_version[QMI_WLFW_MAX_STR_LEN_V01 +,
    pub tgt_cfg_valid: u8,
    pub tgt_cfg_len: u32,
    pub tgt_cfg: [wlfw_ce_tgt_pipe_cfg_s_v01; QMI_WLFW_MAX_NUM_CE_V01],
    pub svc_cfg_valid: u8,
    pub svc_cfg_len: u32,
    pub svc_cfg: [wlfw_ce_svc_pipe_cfg_s_v01; QMI_WLFW_MAX_NUM_SVC_V01],
    pub shadow_reg_valid: u8,
    pub shadow_reg_len: u32,
    pub shadow_reg: [wlfw_shadow_reg_cfg_s_v01; QMI_WLFW_MAX_NUM_SHADOW_REG_V01],
    pub shadow_reg_v2_valid: u8,
    pub shadow_reg_v2_len: u32,
    pub shadow_reg_v2: [wlfw_shadow_reg_v2_cfg_s_v01; QMI_WLFW_MAX_SHADOW_REG_V2],
}

pub const WLFW_WLAN_CFG_REQ_MSG_V01_MAX_MSG_LEN: c_int = 803;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_wlan_cfg_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_WLAN_CFG_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_cap_req_msg_v01 {
    pub placeholder: c_char,
}

pub const WLFW_CAP_REQ_MSG_V01_MAX_MSG_LEN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_cap_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub chip_info_valid: u8,
    pub chip_info: wlfw_rf_chip_info_s_v01,
    pub board_info_valid: u8,
    pub board_info: wlfw_rf_board_info_s_v01,
    pub soc_info_valid: u8,
    pub soc_info: wlfw_soc_info_s_v01,
    pub fw_version_info_valid: u8,
    pub fw_version_info: wlfw_fw_version_info_s_v01,
    pub fw_build_id_valid: u8,
    pub 1]: char fw_build_id[QMI_WLFW_MAX_BUILD_ID_LEN_V01 +,
    pub num_macs_valid: u8,
    pub num_macs: u8,
}

pub const WLFW_CAP_RESP_MSG_V01_MAX_MSG_LEN: c_int = 207;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_bdf_download_req_msg_v01 {
    pub valid: u8,
    pub file_id_valid: u8,
    pub file_id: wlfw_cal_temp_id_enum_v01,
    pub total_size_valid: u8,
    pub total_size: u32,
    pub seg_id_valid: u8,
    pub seg_id: u32,
    pub data_valid: u8,
    pub data_len: u32,
    pub data: [u8; QMI_WLFW_MAX_DATA_SIZE_V01],
    pub end_valid: u8,
    pub end: u8,
    pub bdf_type_valid: u8,
    pub bdf_type: u8,
}

pub const WLFW_BDF_DOWNLOAD_REQ_MSG_V01_MAX_MSG_LEN: c_int = 6182;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_bdf_download_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_BDF_DOWNLOAD_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_cal_report_req_msg_v01 {
    pub meta_data_len: u32,
    pub meta_data: [wlfw_cal_temp_id_enum_v01; QMI_WLFW_MAX_NUM_CAL_V01],
    pub xo_cal_data_valid: u8,
    pub xo_cal_data: u8,
}

pub const WLFW_CAL_REPORT_REQ_MSG_V01_MAX_MSG_LEN: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_cal_report_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_CAL_REPORT_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_initiate_cal_download_ind_msg_v01 {
    pub cal_id: wlfw_cal_temp_id_enum_v01,
}

pub const WLFW_INITIATE_CAL_DOWNLOAD_IND_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_cal_download_req_msg_v01 {
    pub valid: u8,
    pub file_id_valid: u8,
    pub file_id: wlfw_cal_temp_id_enum_v01,
    pub total_size_valid: u8,
    pub total_size: u32,
    pub seg_id_valid: u8,
    pub seg_id: u32,
    pub data_valid: u8,
    pub data_len: u32,
    pub data: [u8; QMI_WLFW_MAX_DATA_SIZE_V01],
    pub end_valid: u8,
    pub end: u8,
}

pub const WLFW_CAL_DOWNLOAD_REQ_MSG_V01_MAX_MSG_LEN: c_int = 6178;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_cal_download_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_CAL_DOWNLOAD_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_initiate_cal_update_ind_msg_v01 {
    pub cal_id: wlfw_cal_temp_id_enum_v01,
    pub total_size: u32,
}

pub const WLFW_INITIATE_CAL_UPDATE_IND_MSG_V01_MAX_MSG_LEN: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_cal_update_req_msg_v01 {
    pub cal_id: wlfw_cal_temp_id_enum_v01,
    pub seg_id: u32,
}

pub const WLFW_CAL_UPDATE_REQ_MSG_V01_MAX_MSG_LEN: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_cal_update_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub file_id_valid: u8,
    pub file_id: wlfw_cal_temp_id_enum_v01,
    pub total_size_valid: u8,
    pub total_size: u32,
    pub seg_id_valid: u8,
    pub seg_id: u32,
    pub data_valid: u8,
    pub data_len: u32,
    pub data: [u8; QMI_WLFW_MAX_DATA_SIZE_V01],
    pub end_valid: u8,
    pub end: u8,
}

pub const WLFW_CAL_UPDATE_RESP_MSG_V01_MAX_MSG_LEN: c_int = 6181;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_msa_info_req_msg_v01 {
    pub msa_addr: u64,
    pub size: u32,
}

pub const WLFW_MSA_INFO_REQ_MSG_V01_MAX_MSG_LEN: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_msa_info_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub mem_region_info_len: u32,
    pub mem_region_info: [wlfw_memory_region_info_s_v01; QMI_WLFW_MAX_MEM_REG_V01],
}

pub const WLFW_MSA_INFO_RESP_MSG_V01_MAX_MSG_LEN: c_int = 37;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_msa_ready_req_msg_v01 {
    pub placeholder: c_char,
}

pub const WLFW_MSA_READY_REQ_MSG_V01_MAX_MSG_LEN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_msa_ready_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_MSA_READY_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_ini_req_msg_v01 {
    pub enablefwlog_valid: u8,
    pub enablefwlog: u8,
}

pub const WLFW_INI_REQ_MSG_V01_MAX_MSG_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_ini_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_INI_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_athdiag_read_req_msg_v01 {
    pub offset: u32,
    pub mem_type: u32,
    pub data_len: u32,
}

pub const WLFW_ATHDIAG_READ_REQ_MSG_V01_MAX_MSG_LEN: c_int = 21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_athdiag_read_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub data_valid: u8,
    pub data_len: u32,
    pub data: [u8; QMI_WLFW_MAX_ATHDIAG_DATA_SIZE_V01],
}

pub const WLFW_ATHDIAG_READ_RESP_MSG_V01_MAX_MSG_LEN: c_int = 6156;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_athdiag_write_req_msg_v01 {
    pub offset: u32,
    pub mem_type: u32,
    pub data_len: u32,
    pub data: [u8; QMI_WLFW_MAX_ATHDIAG_DATA_SIZE_V01],
}

pub const WLFW_ATHDIAG_WRITE_REQ_MSG_V01_MAX_MSG_LEN: c_int = 6163;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_athdiag_write_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_ATHDIAG_WRITE_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_vbatt_req_msg_v01 {
    pub voltage_uv: u64,
}

pub const WLFW_VBATT_REQ_MSG_V01_MAX_MSG_LEN: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_vbatt_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_VBATT_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_mac_addr_req_msg_v01 {
    pub mac_addr_valid: u8,
    pub mac_addr: [u8; QMI_WLFW_MAC_ADDR_SIZE_V01],
}

pub const WLFW_MAC_ADDR_REQ_MSG_V01_MAX_MSG_LEN: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_mac_addr_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_MAC_ADDR_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
pub const QMI_WLFW_MAX_NUM_GPIO_V01: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_host_cap_req_msg_v01 {
    pub daemon_support_valid: u8,
    pub daemon_support: u32,
    pub wake_msi_valid: u8,
    pub wake_msi: u32,
    pub gpios_valid: u8,
    pub gpios_len: u32,
    pub gpios: [u32; QMI_WLFW_MAX_NUM_GPIO_V01],
    pub nm_modem_valid: u8,
    pub nm_modem: u8,
    pub bdf_support_valid: u8,
    pub bdf_support: u8,
    pub bdf_cache_support_valid: u8,
    pub bdf_cache_support: u8,
    pub m3_support_valid: u8,
    pub m3_support: u8,
    pub m3_cache_support_valid: u8,
    pub m3_cache_support: u8,
    pub cal_filesys_support_valid: u8,
    pub cal_filesys_support: u8,
    pub cal_cache_support_valid: u8,
    pub cal_cache_support: u8,
    pub cal_done_valid: u8,
    pub cal_done: u8,
    pub mem_bucket_valid: u8,
    pub mem_bucket: u32,
    pub mem_cfg_mode_valid: u8,
    pub mem_cfg_mode: u8,
}

pub const WLFW_HOST_CAP_REQ_MSG_V01_MAX_MSG_LEN: c_int = 189;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_host_cap_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_HOST_CAP_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_request_mem_ind_msg_v01 {
    pub mem_seg_len: u32,
    pub mem_seg: [wlfw_mem_seg_s_v01; QMI_WLFW_MAX_NUM_MEM_SEG_V01],
}

pub const WLFW_REQUEST_MEM_IND_MSG_V01_MAX_MSG_LEN: c_int = 564;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_respond_mem_req_msg_v01 {
    pub mem_seg_len: u32,
    pub mem_seg: [wlfw_mem_seg_resp_s_v01; QMI_WLFW_MAX_NUM_MEM_SEG_V01],
}

pub const WLFW_RESPOND_MEM_REQ_MSG_V01_MAX_MSG_LEN: c_int = 260;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_respond_mem_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_RESPOND_MEM_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_mem_ready_ind_msg_v01 {
    pub placeholder: c_char,
}

pub const WLFW_MEM_READY_IND_MSG_V01_MAX_MSG_LEN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_fw_init_done_ind_msg_v01 {
    pub placeholder: c_char,
}

pub const WLFW_FW_INIT_DONE_IND_MSG_V01_MAX_MSG_LEN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_rejuvenate_ind_msg_v01 {
    pub cause_for_rejuvenation_valid: u8,
    pub cause_for_rejuvenation: u8,
    pub requesting_sub_system_valid: u8,
    pub requesting_sub_system: u8,
    pub line_number_valid: u8,
    pub line_number: u16,
    pub function_name_valid: u8,
    pub 1]: char function_name[QMI_WLFW_FUNCTION_NAME_LEN_V01 +,
}

pub const WLFW_REJUVENATE_IND_MSG_V01_MAX_MSG_LEN: c_int = 144;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_rejuvenate_ack_req_msg_v01 {
    pub placeholder: c_char,
}

pub const WLFW_REJUVENATE_ACK_REQ_MSG_V01_MAX_MSG_LEN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_rejuvenate_ack_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_REJUVENATE_ACK_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_dynamic_feature_mask_req_msg_v01 {
    pub mask_valid: u8,
    pub mask: u64,
}

pub const WLFW_DYNAMIC_FEATURE_MASK_REQ_MSG_V01_MAX_MSG_LEN: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_dynamic_feature_mask_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub prev_mask_valid: u8,
    pub prev_mask: u64,
    pub curr_mask_valid: u8,
    pub curr_mask: u64,
}

pub const WLFW_DYNAMIC_FEATURE_MASK_RESP_MSG_V01_MAX_MSG_LEN: c_int = 29;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_m3_info_req_msg_v01 {
    pub addr: u64,
    pub size: u32,
}

pub const WLFW_M3_INFO_REQ_MSG_V01_MAX_MSG_LEN: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_m3_info_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const WLFW_M3_INFO_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlfw_xo_cal_ind_msg_v01 {
    pub xo_cal_data: u8,
}

pub const WLFW_XO_CAL_IND_MSG_V01_MAX_MSG_LEN: c_int = 4;
