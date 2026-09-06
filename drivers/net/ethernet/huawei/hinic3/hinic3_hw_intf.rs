//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_hw_intf.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

pub const MGMT_MSG_CMD_OP_SET: c_int = 1;
pub const MGMT_MSG_CMD_OP_GET: c_int = 0;
pub const MGMT_STATUS_PF_SET_VF_ALREADY: c_uint = 0x4;
pub const MGMT_STATUS_EXIST: c_uint = 0x6;
pub const MGMT_STATUS_CMD_UNSUPPORTED: c_uint = 0xFF;
pub const MGMT_MSG_POLLING_TIMEOUT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_msg_head {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_msg_params {
    pub buf_in: *const c_void,
    pub in_size: u32,
    pub buf_out: *mut c_void,
    pub expected_out_size: u32,
    pub timeout_ms: u32,
}

// CMDQ MODULE_TYPE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mgmt_mod_type {
// HW communication module
    MGMT_MOD_COMM   = 0,
// L2NIC module
    MGMT_MOD_L2NIC  = 1,
// Configuration module
    MGMT_MOD_CFGM   = 7,
    MGMT_MOD_HILINK = 14,
// hardware max module id
    MGMT_MOD_HW_MAX = 20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg_cmd {
    CFG_CMD_GET_DEV_CAP = 0,
}

// Device capabilities, defined by hw
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg_cmd_dev_cap {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub rsvd1: u16,
// Public resources
    pub host_id: u8,
    pub ep_id: u8,
    pub er_id: u8,
    pub port_id: u8,
    pub host_total_func: u16,
    pub host_pf_num: u8,
    pub pf_id_start: u8,
    pub host_vf_num: u16,
    pub vf_id_start: u16,
    pub host_oq_id_mask_val: u8,
    pub timer_en: u8,
    pub host_valid_bitmap: u8,
    pub rsvd_host: u8,
    pub svc_cap_en: u16,
    pub max_vf: u16,
    pub flexq_en: u8,
    pub valid_cos_bitmap: u8,
    pub port_cos_valid_bitmap: u8,
    pub rsvd2: [u8; 45],
// l2nic
    pub nic_max_sq_id: u16,
    pub nic_max_rq_id: u16,
    pub nic_default_num_queues: u16,
    pub rsvd3: [u8; 250],
}

// COMM Commands between Driver to fw
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum comm_cmd {
// Commands for clearing FLR and resources
    COMM_CMD_FUNC_RESET              = 0,
    COMM_CMD_FEATURE_NEGO            = 1,
    COMM_CMD_FLUSH_DOORBELL          = 2,
    COMM_CMD_START_FLUSH             = 3,
    COMM_CMD_GET_GLOBAL_ATTR         = 5,
    COMM_CMD_SET_FUNC_SVC_USED_STATE = 7,

// Driver Configuration Commands
    COMM_CMD_SET_CMDQ_CTXT           = 20,
    COMM_CMD_SET_VAT                 = 21,
    COMM_CMD_CFG_PAGESIZE            = 22,
    COMM_CMD_CFG_MSIX_CTRL_REG       = 23,
    COMM_CMD_SET_CEQ_CTRL_REG        = 24,
    COMM_CMD_SET_DMA_ATTR            = 25,

// Commands for obtaining information
    COMM_CMD_GET_FW_VERSION          = 60,
    COMM_CMD_SYNC_TIME               = 62,
    COMM_CMD_SEND_BDF_INFO           = 64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_cfg_msix_ctrl_reg {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub opcode: u8,
    pub rsvd1: u8,
    pub msix_index: u16,
    pub pending_cnt: u8,
    pub coalesce_timer_cnt: u8,
    pub resend_timer_cnt: u8,
    pub lli_timer_cnt: u8,
    pub lli_credit_cnt: u8,
    pub rsvd2: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum comm_func_reset_bits {
    COMM_FUNC_RESET_BIT_FLUSH        = BIT(0),
    COMM_FUNC_RESET_BIT_MQM          = BIT(1),
    COMM_FUNC_RESET_BIT_SMF          = BIT(2),
    COMM_FUNC_RESET_BIT_PF_BW_CFG    = BIT(3),

    COMM_FUNC_RESET_BIT_COMM         = BIT(10),
// clear mbox and aeq, The COMM_FUNC_RESET_BIT_COMM bit must be set
    COMM_FUNC_RESET_BIT_COMM_MGMT_CH = BIT(11),
// clear cmdq and ceq, The COMM_FUNC_RESET_BIT_COMM bit must be set
    COMM_FUNC_RESET_BIT_COMM_CMD_CH  = BIT(12),
    COMM_FUNC_RESET_BIT_NIC          = BIT(13),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_func_reset {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub rsvd1: [u16; 3],
    pub reset_flag: u64,
}

pub const COMM_MAX_FEATURE_QWORD: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_feature_nego {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub opcode: u8,
    pub rsvd: u8,
    pub s_feature: [u64; COMM_MAX_FEATURE_QWORD],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_global_attr {
    pub max_host_num: u8,
    pub max_pf_num: u8,
    pub vf_id_start: u16,
// for api cmd to mgmt cpu
    pub mgmt_host_node_id: u8,
    pub cmdq_num: u8,
    pub rsvd1: [u8; 34],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_get_glb_attr {
    pub head: mgmt_msg_head,
    pub attr: comm_global_attr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum comm_func_svc_type {
    COMM_FUNC_SVC_T_COMM = 0,
    COMM_FUNC_SVC_T_NIC  = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_set_func_svc_used_state {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub svc_type: u16,
    pub used_state: u8,
    pub rsvd: [u8; 35],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_set_dma_attr {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub entry_idx: u8,
    pub st: u8,
    pub at: u8,
    pub ph: u8,
    pub no_snooping: u8,
    pub tph_en: u8,
    pub resv1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_set_ceq_ctrl_reg {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub q_id: u16,
    pub ctrl0: u32,
    pub ctrl1: u32,
    pub rsvd1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_cfg_wq_page_size {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub opcode: u8,
// real_size=4KB*2^page_size, range(0~20) must be checked by driver
    pub page_size: u8,
    pub rsvd1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_set_root_ctxt {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub set_cmdq_depth: u8,
    pub cmdq_depth: u8,
    pub rx_buf_sz: u16,
    pub lro_en: u8,
    pub rsvd1: u8,
    pub sq_depth: u16,
    pub rq_depth: u16,
    pub rsvd2: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmdq_ctxt_info {
    pub curr_wqe_page_pfn: __le64,
    pub wq_block_pfn: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_set_cmdq_ctxt {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub cmdq_id: u8,
    pub rsvd1: [u8; 5],
    pub ctxt: comm_cmdq_ctxt_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_clear_resource {
    pub head: mgmt_msg_head,
    pub func_id: u16,
    pub rsvd1: [u16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_sync_time {
    pub head: mgmt_msg_head,
    pub mstime: u64,
    pub rsvd1: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_bdf_info {
    pub head: mgmt_msg_head,
    pub function_idx: u16,
    pub rsvd1: [u8; 2],
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub rsvd2: [u8; 5],
}

pub const COMM_FW_VERSION_LEN: c_int = 16;
pub const COMM_FW_COMPILE_TIME_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_cmd_get_fw_version {
    pub head: mgmt_msg_head,
    pub fw_type: u16,
    pub rsvd1: u16,
    pub ver: [u8; COMM_FW_VERSION_LEN],
    pub time: [u8; COMM_FW_COMPILE_TIME_LEN],
}

// Services supported by HW. HW uses these values when delivering events.
// HW supports multiple services that are not yet supported by driver
// (e.g. RoCE).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_service_type {
    HINIC3_SERVICE_T_NIC = 0,
// MAX is only used by SW for array sizes.
    HINIC3_SERVICE_T_MAX = 1,
}
