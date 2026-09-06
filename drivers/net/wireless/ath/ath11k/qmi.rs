//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/qmi.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH11K_QMI_WLANFW_TIMEOUT_MS: c_int = 10000;
pub const ATH11K_QMI_MAX_BDF_FILE_NAME_SIZE: c_int = 64;
pub const ATH11K_QMI_CALDB_ADDRESS: c_uint = 0x4BA00000;
pub const ATH11K_QMI_WLANFW_MAX_BUILD_ID_LEN_V01: c_int = 128;
pub const ATH11K_QMI_WLFW_SERVICE_VERS_V01: c_uint = 0x01;
pub const ATH11K_QMI_WLFW_SERVICE_INS_ID_V01: c_uint = 0x02;
pub const ATH11K_QMI_WLFW_SERVICE_INS_ID_V01_QCA6390: c_uint = 0x01;
pub const ATH11K_QMI_WLFW_SERVICE_INS_ID_V01_IPQ8074: c_uint = 0x02;
pub const ATH11K_QMI_WLFW_SERVICE_INS_ID_V01_QCN9074: c_uint = 0x07;
pub const ATH11K_QMI_WLFW_SERVICE_INS_ID_V01_WCN6750: c_uint = 0x03;
pub const ATH11K_QMI_WLANFW_MAX_TIMESTAMP_LEN_V01: c_int = 32;
pub const ATH11K_QMI_RESP_LEN_MAX: c_int = 8192;
pub const ATH11K_QMI_WLANFW_MAX_NUM_MEM_SEG_V01: c_int = 52;
pub const ATH11K_QMI_CALDB_SIZE: c_uint = 0x480000;
pub const ATH11K_QMI_BDF_EXT_STR_LENGTH: c_uint = 0x20;
pub const ATH11K_QMI_FW_MEM_REQ_SEGMENT_CNT: c_int = 5;
pub const QMI_WLFW_REQUEST_MEM_IND_V01: c_uint = 0x0035;
pub const QMI_WLFW_FW_MEM_READY_IND_V01: c_uint = 0x0037;
pub const QMI_WLFW_COLD_BOOT_CAL_DONE_IND_V01: c_uint = 0x003E;
pub const QMI_WLFW_FW_READY_IND_V01: c_uint = 0x0021;
pub const QMI_WLFW_FW_INIT_DONE_IND_V01: c_uint = 0x0038;
pub const QMI_WLANFW_MAX_DATA_SIZE_V01: c_int = 6144;
pub const ATH11K_FIRMWARE_MODE_OFF: c_int = 4;

pub const ATH11K_QMI_DEVICE_BAR_SIZE: c_uint = 0x200000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_qmi_file_type {
    ATH11K_QMI_FILE_TYPE_BDF_GOLDEN,
    ATH11K_QMI_FILE_TYPE_CALDATA = 2,
    ATH11K_QMI_FILE_TYPE_EEPROM,
    ATH11K_QMI_MAX_FILE_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_qmi_bdf_type {
    ATH11K_QMI_BDF_TYPE_BIN			= 0,
    ATH11K_QMI_BDF_TYPE_ELF			= 1,
    ATH11K_QMI_BDF_TYPE_REGDB		= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_qmi_event_type {
    ATH11K_QMI_EVENT_SERVER_ARRIVE,
    ATH11K_QMI_EVENT_SERVER_EXIT,
    ATH11K_QMI_EVENT_REQUEST_MEM,
    ATH11K_QMI_EVENT_FW_MEM_READY,
    ATH11K_QMI_EVENT_FW_READY,
    ATH11K_QMI_EVENT_COLD_BOOT_CAL_START,
    ATH11K_QMI_EVENT_COLD_BOOT_CAL_DONE,
    ATH11K_QMI_EVENT_REGISTER_DRIVER,
    ATH11K_QMI_EVENT_UNREGISTER_DRIVER,
    ATH11K_QMI_EVENT_RECOVERY,
    ATH11K_QMI_EVENT_FORCE_FW_ASSERT,
    ATH11K_QMI_EVENT_POWER_UP,
    ATH11K_QMI_EVENT_POWER_DOWN,
    ATH11K_QMI_EVENT_FW_INIT_DONE,
    ATH11K_QMI_EVENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_qmi_driver_event {
    pub list: list_head,
    pub type: ath11k_qmi_event_type,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_qmi_ce_cfg {
    pub tgt_ce: *const ce_pipe_config,
    pub tgt_ce_len: c_int,
    pub svc_to_ce_map: *const service_to_pipe,
    pub svc_to_ce_map_len: c_int,
    pub shadow_reg: *const u8,
    pub shadow_reg_len: c_int,
    pub shadow_reg_v2: *mut u32,
    pub shadow_reg_v2_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_qmi_event_msg {
    pub list: list_head,
    pub type: ath11k_qmi_event_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_mem_chunk {
    pub size: u32,
    pub type: u32,
    pub prev_size: u32,
    pub prev_type: u32,
    pub paddr: dma_addr_t,
    pub vaddr: *mut u32,
    pub iaddr: *mut void __iomem,
    pub anyaddr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_info {
    pub chip_id: u32,
    pub chip_family: u32,
    pub board_id: u32,
    pub soc_id: u32,
    pub fw_version: u32,
    pub eeprom_caldata: u32,
    pub 1]: char fw_build_timestamp[ATH11K_QMI_WLANFW_MAX_TIMESTAMP_LEN_V01 +,
    pub 1]: char fw_build_id[ATH11K_QMI_WLANFW_MAX_BUILD_ID_LEN_V01 +,
    pub bdf_ext: [c_char; ATH11K_QMI_BDF_EXT_STR_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m3_mem_region {
    pub size: u32,
    pub paddr: dma_addr_t,
    pub vaddr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_qmi {
    pub ab: *mut ath11k_base,
    pub handle: qmi_handle,
    pub sq: sockaddr_qrtr,
    pub event_work: work_struct,
    pub event_wq: *mut workqueue_struct,
    pub event_list: list_head,
    pub /: *mut *mut spinlock_t event_lock; / spinlock for qmi event list,
    pub ce_cfg: ath11k_qmi_ce_cfg,
    pub target_mem: [target_mem_chunk; ATH11K_QMI_WLANFW_MAX_NUM_MEM_SEG_V01],
    pub mem_seg_count: u32,
    pub target_mem_mode: u32,
    pub target_mem_delayed: bool,
    pub cal_done: u8,
    pub target: target_info,
    pub m3_mem: m3_mem_region,
    pub service_ins_id: c_uint,
    pub cold_boot_waitq: wait_queue_head_t,
}

pub const QMI_WLANFW_HOST_CAP_REQ_MSG_V01_MAX_LEN: c_int = 261;
pub const QMI_WLANFW_HOST_CAP_REQ_V01: c_uint = 0x0034;
pub const QMI_WLANFW_HOST_CAP_RESP_MSG_V01_MAX_LEN: c_int = 7;
pub const QMI_WLFW_HOST_CAP_RESP_V01: c_uint = 0x0034;
pub const QMI_WLFW_MAX_NUM_GPIO_V01: c_int = 32;
pub const QMI_IPQ8074_FW_MEM_MODE: c_uint = 0xFF;
pub const HOST_DDR_REGION_TYPE: c_uint = 0x1;
pub const BDF_MEM_REGION_TYPE: c_uint = 0x2;
pub const M3_DUMP_REGION_TYPE: c_uint = 0x3;
pub const CALDB_MEM_REGION_TYPE: c_uint = 0x4;
pub const PAGEABLE_MEM_REGION_TYPE: c_uint = 0x9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_host_cap_req_msg_v01 {
    pub num_clients_valid: u8,
    pub num_clients: u32,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_host_cap_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const QMI_WLANFW_IND_REGISTER_REQ_MSG_V01_MAX_LEN: c_int = 54;
pub const QMI_WLANFW_IND_REGISTER_REQ_V01: c_uint = 0x0020;
pub const QMI_WLANFW_IND_REGISTER_RESP_MSG_V01_MAX_LEN: c_int = 18;
pub const QMI_WLANFW_IND_REGISTER_RESP_V01: c_uint = 0x0020;
pub const QMI_WLANFW_CLIENT_ID: c_uint = 0x4b4e454c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_ind_register_req_msg_v01 {
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
    pub fw_mem_ready_enable_valid: u8,
    pub fw_mem_ready_enable: u8,
    pub fw_init_done_enable_valid: u8,
    pub fw_init_done_enable: u8,
    pub rejuvenate_enable_valid: u8,
    pub rejuvenate_enable: u32,
    pub xo_cal_enable_valid: u8,
    pub xo_cal_enable: u8,
    pub cal_done_enable_valid: u8,
    pub cal_done_enable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_ind_register_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub fw_status_valid: u8,
    pub fw_status: u64,
}

pub const QMI_WLANFW_REQUEST_MEM_IND_MSG_V01_MAX_LEN: c_int = 1824;
pub const QMI_WLANFW_RESPOND_MEM_REQ_MSG_V01_MAX_LEN: c_int = 888;
pub const QMI_WLANFW_RESPOND_MEM_RESP_MSG_V01_MAX_LEN: c_int = 7;
pub const QMI_WLANFW_REQUEST_MEM_IND_V01: c_uint = 0x0035;
pub const QMI_WLANFW_RESPOND_MEM_REQ_V01: c_uint = 0x0036;
pub const QMI_WLANFW_RESPOND_MEM_RESP_V01: c_uint = 0x0036;
pub const QMI_WLANFW_MAX_NUM_MEM_CFG_V01: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_mem_cfg_s_v01 {
    pub offset: u64,
    pub size: u32,
    pub secure_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qmi_wlanfw_mem_type_enum_v01 {
    WLANFW_MEM_TYPE_ENUM_MIN_VAL_V01 = INT_MIN,
    QMI_WLANFW_MEM_TYPE_MSA_V01 = 0,
    QMI_WLANFW_MEM_TYPE_DDR_V01 = 1,
    QMI_WLANFW_MEM_BDF_V01 = 2,
    QMI_WLANFW_MEM_M3_V01 = 3,
    QMI_WLANFW_MEM_CAL_V01 = 4,
    QMI_WLANFW_MEM_DPD_V01 = 5,
    WLANFW_MEM_TYPE_ENUM_MAX_VAL_V01 = INT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_mem_seg_s_v01 {
    pub size: u32,
    pub type: qmi_wlanfw_mem_type_enum_v01,
    pub mem_cfg_len: u32,
    pub mem_cfg: [qmi_wlanfw_mem_cfg_s_v01; QMI_WLANFW_MAX_NUM_MEM_CFG_V01],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_request_mem_ind_msg_v01 {
    pub mem_seg_len: u32,
    pub mem_seg: [qmi_wlanfw_mem_seg_s_v01; ATH11K_QMI_WLANFW_MAX_NUM_MEM_SEG_V01],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_mem_seg_resp_s_v01 {
    pub addr: u64,
    pub size: u32,
    pub type: qmi_wlanfw_mem_type_enum_v01,
    pub restore: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_respond_mem_req_msg_v01 {
    pub mem_seg_len: u32,
    pub mem_seg: [qmi_wlanfw_mem_seg_resp_s_v01; ATH11K_QMI_WLANFW_MAX_NUM_MEM_SEG_V01],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_respond_mem_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_fw_mem_ready_ind_msg_v01 {
    pub placeholder: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_fw_ready_ind_msg_v01 {
    pub placeholder: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_fw_cold_cal_done_ind_msg_v01 {
    pub placeholder: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlfw_fw_init_done_ind_msg_v01 {
    pub placeholder: c_char,
}

pub const QMI_WLANFW_CAP_REQ_MSG_V01_MAX_LEN: c_int = 0;
pub const QMI_WLANFW_CAP_RESP_MSG_V01_MAX_LEN: c_int = 235;
pub const QMI_WLANFW_CAP_REQ_V01: c_uint = 0x0024;
pub const QMI_WLANFW_CAP_RESP_V01: c_uint = 0x0024;
pub const QMI_WLANFW_DEVICE_INFO_REQ_V01: c_uint = 0x004C;
pub const QMI_WLANFW_DEVICE_INFO_REQ_MSG_V01_MAX_LEN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qmi_wlanfw_pipedir_enum_v01 {
    QMI_WLFW_PIPEDIR_NONE_V01 = 0,
    QMI_WLFW_PIPEDIR_IN_V01 = 1,
    QMI_WLFW_PIPEDIR_OUT_V01 = 2,
    QMI_WLFW_PIPEDIR_INOUT_V01 = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_ce_tgt_pipe_cfg_s_v01 {
    pub pipe_num: __le32,
    pub pipe_dir: __le32,
    pub nentries: __le32,
    pub nbytes_max: __le32,
    pub flags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_ce_svc_pipe_cfg_s_v01 {
    pub service_id: __le32,
    pub pipe_dir: __le32,
    pub pipe_num: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_shadow_reg_cfg_s_v01 {
    pub id: u16,
    pub offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_shadow_reg_v2_cfg_s_v01 {
    pub addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_memory_region_info_s_v01 {
    pub region_addr: u64,
    pub size: u32,
    pub secure_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_rf_chip_info_s_v01 {
    pub chip_id: u32,
    pub chip_family: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_rf_board_info_s_v01 {
    pub board_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_soc_info_s_v01 {
    pub soc_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_fw_version_info_s_v01 {
    pub fw_version: u32,
    pub 1]: char fw_build_timestamp[ATH11K_QMI_WLANFW_MAX_TIMESTAMP_LEN_V01 +,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qmi_wlanfw_cal_temp_id_enum_v01 {
    QMI_WLANFW_CAL_TEMP_IDX_0_V01 = 0,
    QMI_WLANFW_CAL_TEMP_IDX_1_V01 = 1,
    QMI_WLANFW_CAL_TEMP_IDX_2_V01 = 2,
    QMI_WLANFW_CAL_TEMP_IDX_3_V01 = 3,
    QMI_WLANFW_CAL_TEMP_IDX_4_V01 = 4,
    QMI_WLANFW_CAL_TEMP_ID_MAX_V01 = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_cap_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub chip_info_valid: u8,
    pub chip_info: qmi_wlanfw_rf_chip_info_s_v01,
    pub board_info_valid: u8,
    pub board_info: qmi_wlanfw_rf_board_info_s_v01,
    pub soc_info_valid: u8,
    pub soc_info: qmi_wlanfw_soc_info_s_v01,
    pub fw_version_info_valid: u8,
    pub fw_version_info: qmi_wlanfw_fw_version_info_s_v01,
    pub fw_build_id_valid: u8,
    pub 1]: char fw_build_id[ATH11K_QMI_WLANFW_MAX_BUILD_ID_LEN_V01 +,
    pub num_macs_valid: u8,
    pub num_macs: u8,
    pub voltage_mv_valid: u8,
    pub voltage_mv: u32,
    pub time_freq_hz_valid: u8,
    pub time_freq_hz: u32,
    pub otp_version_valid: u8,
    pub otp_version: u32,
    pub eeprom_read_timeout_valid: u8,
    pub eeprom_read_timeout: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_cap_req_msg_v01 {
    pub placeholder: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_device_info_req_msg_v01 {
    pub placeholder: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_device_info_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub bar_addr: u64,
    pub bar_size: u32,
    pub bar_addr_valid: u8,
    pub bar_size_valid: u8,
}

pub const QMI_WLANFW_BDF_DOWNLOAD_REQ_MSG_V01_MAX_LEN: c_int = 6182;
pub const QMI_WLANFW_BDF_DOWNLOAD_RESP_MSG_V01_MAX_LEN: c_int = 7;
pub const QMI_WLANFW_BDF_DOWNLOAD_RESP_V01: c_uint = 0x0025;
pub const QMI_WLANFW_BDF_DOWNLOAD_REQ_V01: c_uint = 0x0025;
// TODO: Need to check with MCL and FW team that data can be pointer and
// can be last element in structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_bdf_download_req_msg_v01 {
    pub valid: u8,
    pub file_id_valid: u8,
    pub file_id: qmi_wlanfw_cal_temp_id_enum_v01,
    pub total_size_valid: u8,
    pub total_size: u32,
    pub seg_id_valid: u8,
    pub seg_id: u32,
    pub data_valid: u8,
    pub data_len: u32,
    pub data: [u8; QMI_WLANFW_MAX_DATA_SIZE_V01],
    pub end_valid: u8,
    pub end: u8,
    pub bdf_type_valid: u8,
    pub bdf_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_bdf_download_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const QMI_WLANFW_M3_INFO_REQ_MSG_V01_MAX_MSG_LEN: c_int = 18;
pub const QMI_WLANFW_M3_INFO_RESP_MSG_V01_MAX_MSG_LEN: c_int = 7;
pub const QMI_WLANFW_M3_INFO_RESP_V01: c_uint = 0x003C;
pub const QMI_WLANFW_M3_INFO_REQ_V01: c_uint = 0x003C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_m3_info_req_msg_v01 {
    pub addr: u64,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_m3_info_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

pub const QMI_WLANFW_WLAN_MODE_REQ_MSG_V01_MAX_LEN: c_int = 11;
pub const QMI_WLANFW_WLAN_MODE_RESP_MSG_V01_MAX_LEN: c_int = 7;
pub const QMI_WLANFW_WLAN_CFG_REQ_MSG_V01_MAX_LEN: c_int = 803;
pub const QMI_WLANFW_WLAN_CFG_RESP_MSG_V01_MAX_LEN: c_int = 7;
pub const QMI_WLANFW_WLAN_INI_REQ_MSG_V01_MAX_LEN: c_int = 4;
pub const QMI_WLANFW_WLAN_MODE_REQ_V01: c_uint = 0x0022;
pub const QMI_WLANFW_WLAN_MODE_RESP_V01: c_uint = 0x0022;
pub const QMI_WLANFW_WLAN_CFG_REQ_V01: c_uint = 0x0023;
pub const QMI_WLANFW_WLAN_CFG_RESP_V01: c_uint = 0x0023;
pub const QMI_WLANFW_WLAN_INI_REQ_V01: c_uint = 0x002F;
pub const QMI_WLANFW_MAX_STR_LEN_V01: c_int = 16;
pub const QMI_WLANFW_MAX_NUM_CE_V01: c_int = 12;
pub const QMI_WLANFW_MAX_NUM_SVC_V01: c_int = 24;
pub const QMI_WLANFW_MAX_NUM_SHADOW_REG_V01: c_int = 24;
pub const QMI_WLANFW_MAX_NUM_SHADOW_REG_V2_V01: c_int = 36;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_wlan_mode_req_msg_v01 {
    pub mode: u32,
    pub hw_debug_valid: u8,
    pub hw_debug: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_wlan_mode_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_wlan_cfg_req_msg_v01 {
    pub host_version_valid: u8,
    pub 1]: char host_version[QMI_WLANFW_MAX_STR_LEN_V01 +,
    pub tgt_cfg_valid: u8,
    pub tgt_cfg_len: u32,
    pub svc_cfg_valid: u8,
    pub svc_cfg_len: u32,
    pub shadow_reg_valid: u8,
    pub shadow_reg_len: u32,
    pub shadow_reg_v2_valid: u8,
    pub shadow_reg_v2_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_wlan_cfg_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_wlan_ini_req_msg_v01 {
// Must be set to true if enablefwlog is being passed
    pub enablefwlog_valid: u8,
    pub enablefwlog: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_wlanfw_wlan_ini_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
}

extern "C" {
    pub fn ath11k_qmi_firmware_stop(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_qmi_deinit_service(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_qmi_init_service(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_qmi_free_resource(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_qmi_fwreset_from_cold_boot(ab: *mut ath11k_base) -> c_int;
}
