//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_dev.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const HINIC_MAX_QPS: c_int = 32;

pub const HINIC_PF_SET_VF_ALREADY: c_uint = 0x4;
pub const HINIC_MGMT_STATUS_EXIST: c_uint = 0x6;
pub const HINIC_MGMT_CMD_UNSUPPORTED: c_uint = 0xFF;
pub const HINIC_CMD_VER_FUNC_ID: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cap {
    pub max_qps: u16,
    pub num_qps: u16,
    pub max_vf: u8,
    pub max_vf_qps: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_ioctxt_set_cmdq_depth {
    HW_IOCTXT_SET_CMDQ_DEPTH_DEFAULT,
    HW_IOCTXT_SET_CMDQ_DEPTH_ENABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_port_cmd {
    HINIC_PORT_CMD_VF_REGISTER = 0x0,
    HINIC_PORT_CMD_VF_UNREGISTER = 0x1,

    HINIC_PORT_CMD_CHANGE_MTU = 0x2,

    HINIC_PORT_CMD_ADD_VLAN = 0x3,
    HINIC_PORT_CMD_DEL_VLAN = 0x4,

    HINIC_PORT_CMD_SET_ETS = 0x7,
    HINIC_PORT_CMD_GET_ETS = 0x8,

    HINIC_PORT_CMD_SET_PFC = 0x5,

    HINIC_PORT_CMD_SET_MAC = 0x9,
    HINIC_PORT_CMD_GET_MAC = 0xA,
    HINIC_PORT_CMD_DEL_MAC = 0xB,

    HINIC_PORT_CMD_SET_RX_MODE = 0xC,

    HINIC_PORT_CMD_SET_ANTI_ATTACK_RATE = 0xD,

    HINIC_PORT_CMD_GET_PAUSE_INFO = 0x14,
    HINIC_PORT_CMD_SET_PAUSE_INFO = 0x15,

    HINIC_PORT_CMD_GET_LINK_STATE = 0x18,

    HINIC_PORT_CMD_SET_LRO = 0x19,

    HINIC_PORT_CMD_SET_RX_CSUM = 0x1A,

    HINIC_PORT_CMD_SET_RX_VLAN_OFFLOAD = 0x1B,

    HINIC_PORT_CMD_GET_PORT_STATISTICS = 0x1C,

    HINIC_PORT_CMD_CLEAR_PORT_STATISTICS = 0x1D,

    HINIC_PORT_CMD_GET_VPORT_STAT = 0x1E,

    HINIC_PORT_CMD_CLEAN_VPORT_STAT	= 0x1F,

    HINIC_PORT_CMD_GET_RSS_TEMPLATE_INDIR_TBL = 0x25,

    HINIC_PORT_CMD_SET_PORT_STATE = 0x29,
    HINIC_PORT_CMD_GET_PORT_STATE = 0x30,

    HINIC_PORT_CMD_SET_RSS_TEMPLATE_TBL = 0x2B,

    HINIC_PORT_CMD_GET_RSS_TEMPLATE_TBL = 0x2C,

    HINIC_PORT_CMD_SET_RSS_HASH_ENGINE = 0x2D,

    HINIC_PORT_CMD_GET_RSS_HASH_ENGINE = 0x2E,

    HINIC_PORT_CMD_GET_RSS_CTX_TBL = 0x2F,

    HINIC_PORT_CMD_SET_RSS_CTX_TBL = 0x30,

    HINIC_PORT_CMD_RSS_TEMP_MGR	= 0x31,

    HINIC_PORT_CMD_RD_LINE_TBL = 0x39,

    HINIC_PORT_CMD_RSS_CFG = 0x42,

    HINIC_PORT_CMD_GET_PHY_TYPE = 0x44,

    HINIC_PORT_CMD_FWCTXT_INIT = 0x45,

    HINIC_PORT_CMD_GET_LOOPBACK_MODE = 0x48,
    HINIC_PORT_CMD_SET_LOOPBACK_MODE = 0x49,

    HINIC_PORT_CMD_GET_JUMBO_FRAME_SIZE = 0x4A,
    HINIC_PORT_CMD_SET_JUMBO_FRAME_SIZE = 0x4B,

    HINIC_PORT_CMD_ENABLE_SPOOFCHK = 0x4E,

    HINIC_PORT_CMD_GET_MGMT_VERSION = 0x58,

    HINIC_PORT_CMD_GET_PORT_TYPE = 0x5B,

    HINIC_PORT_CMD_SET_FUNC_STATE = 0x5D,

    HINIC_PORT_CMD_GET_PORT_ID_BY_FUNC_ID = 0x5E,

    HINIC_PORT_CMD_GET_DMA_CS = 0x64,
    HINIC_PORT_CMD_SET_DMA_CS = 0x65,

    HINIC_PORT_CMD_GET_GLOBAL_QPN = 0x66,

    HINIC_PORT_CMD_SET_VF_RATE = 0x69,

    HINIC_PORT_CMD_SET_VF_VLAN = 0x6A,

    HINIC_PORT_CMD_CLR_VF_VLAN = 0x6B,

    HINIC_PORT_CMD_SET_TSO = 0x70,

    HINIC_PORT_CMD_UPDATE_FW = 0x72,

    HINIC_PORT_CMD_SET_RQ_IQ_MAP = 0x73,

    HINIC_PORT_CMD_SET_PFC_THD = 0x75,

    HINIC_PORT_CMD_LINK_STATUS_REPORT = 0xA0,

    HINIC_PORT_CMD_SET_LOSSLESS_ETH	= 0xA3,

    HINIC_PORT_CMD_UPDATE_MAC = 0xA4,

    HINIC_PORT_CMD_GET_CAP = 0xAA,

    HINIC_PORT_CMD_UP_TC_ADD_FLOW = 0xAF,
    HINIC_PORT_CMD_UP_TC_DEL_FLOW = 0xB0,
    HINIC_PORT_CMD_UP_TC_GET_FLOW = 0xB1,

    HINIC_PORT_CMD_UP_TC_FLUSH_TCAM = 0xB2,

    HINIC_PORT_CMD_UP_TC_CTRL_TCAM_BLOCK = 0xB3,

    HINIC_PORT_CMD_UP_TC_ENABLE = 0xB4,

    HINIC_PORT_CMD_UP_TC_GET_TCAM_BLOCK = 0xB5,

    HINIC_PORT_CMD_SET_IPSU_MAC = 0xCB,
    HINIC_PORT_CMD_GET_IPSU_MAC = 0xCC,

    HINIC_PORT_CMD_SET_XSFP_STATUS = 0xD4,

    HINIC_PORT_CMD_GET_LINK_MODE = 0xD9,

    HINIC_PORT_CMD_SET_SPEED = 0xDA,

    HINIC_PORT_CMD_SET_AUTONEG = 0xDB,

    HINIC_PORT_CMD_CLEAR_QP_RES = 0xDD,

    HINIC_PORT_CMD_SET_SUPER_CQE = 0xDE,

    HINIC_PORT_CMD_SET_VF_COS = 0xDF,
    HINIC_PORT_CMD_GET_VF_COS = 0xE1,

    HINIC_PORT_CMD_CABLE_PLUG_EVENT	= 0xE5,

    HINIC_PORT_CMD_LINK_ERR_EVENT = 0xE6,

    HINIC_PORT_CMD_SET_COS_UP_MAP = 0xE8,

    HINIC_PORT_CMD_RESET_LINK_CFG = 0xEB,

    HINIC_PORT_CMD_GET_STD_SFP_INFO = 0xF0,

    HINIC_PORT_CMD_FORCE_PKT_DROP = 0xF3,

    HINIC_PORT_CMD_SET_LRO_TIMER = 0xF4,

    HINIC_PORT_CMD_SET_VHD_CFG = 0xF7,

    HINIC_PORT_CMD_SET_LINK_FOLLOW = 0xF8,

    HINIC_PORT_CMD_SET_VF_MAX_MIN_RATE = 0xF9,

    HINIC_PORT_CMD_GET_SFP_ABS = 0xFB,

    HINIC_PORT_CMD_Q_FILTER	= 0xFC,

    HINIC_PORT_CMD_TCAM_FILTER = 0xFE,

    HINIC_PORT_CMD_SET_VLAN_FILTER = 0xFF,
}

// cmd of mgmt CPU message for HILINK module
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_hilink_cmd {
    HINIC_HILINK_CMD_GET_LINK_INFO		= 0x3,
    HINIC_HILINK_CMD_SET_LINK_SETTINGS	= 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_ucode_cmd {
    HINIC_UCODE_CMD_MODIFY_QUEUE_CONTEXT    = 0,
    HINIC_UCODE_CMD_CLEAN_QUEUE_CONTEXT,
    HINIC_UCODE_CMD_ARM_SQ,
    HINIC_UCODE_CMD_ARM_RQ,
    HINIC_UCODE_CMD_SET_RSS_INDIR_TABLE,
    HINIC_UCODE_CMD_SET_RSS_CONTEXT_TABLE,
    HINIC_UCODE_CMD_GET_RSS_INDIR_TABLE,
    HINIC_UCODE_CMD_GET_RSS_CONTEXT_TABLE,
    HINIC_UCODE_CMD_SET_IQ_ENABLE,
    HINIC_UCODE_CMD_SET_RQ_FLUSH            = 10
}

pub const NIC_RSS_CMD_TEMP_ALLOC: c_uint = 0x01;
pub const NIC_RSS_CMD_TEMP_FREE: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_mgmt_msg_cmd {
    HINIC_MGMT_MSG_CMD_BASE         = 0xA0,

    HINIC_MGMT_MSG_CMD_LINK_STATUS  = 0xA0,

    HINIC_MGMT_MSG_CMD_CABLE_PLUG_EVENT	= 0xE5,
    HINIC_MGMT_MSG_CMD_LINK_ERR_EVENT	= 0xE6,

    HINIC_MGMT_MSG_CMD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_cb_state {
    HINIC_CB_ENABLED = BIT(0),
    HINIC_CB_RUNNING = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_res_state {
    HINIC_RES_CLEAN         = 0,
    HINIC_RES_ACTIVE        = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_fw_ctxt {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub rx_buf_sz: u16,
    pub rsvd1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_hw_ioctxt {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub rsvd1: u16,
    pub set_cmdq_depth: u8,
    pub cmdq_depth: u8,
    pub lro_en: u8,
    pub rsvd3: u8,
    pub ppf_idx: u8,
    pub rsvd4: u8,
    pub rq_depth: u16,
    pub rx_buf_sz_idx: u16,
    pub sq_depth: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_io_status {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub io_status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_clear_io_res {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub rsvd1: u8,
    pub rsvd2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_set_res_state {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub state: u8,
    pub rsvd1: u8,
    pub rsvd2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_ceq_ctrl_reg {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub q_id: u16,
    pub ctrl0: u32,
    pub ctrl1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_base_qpn {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub qpn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_hw_ci {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub dma_attr_off: u8,
    pub pending_limit: u8,
    pub coalesc_timer: u8,
    pub msix_en: u8,
    pub msix_entry_idx: u16,
    pub sq_id: u32,
    pub rsvd1: u32,
    pub ci_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_l2nic_reset {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub reset_flag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_msix_config {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub msix_index: u16,
    pub pending_cnt: u8,
    pub coalesce_timer_cnt: u8,
    pub lli_timer_cnt: u8,
    pub lli_credit_cnt: u8,
    pub resend_timer_cnt: u8,
    pub rsvd1: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_set_random_id {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub vf_in_pf: u8,
    pub rsvd1: u8,
    pub func_idx: u16,
    pub random_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_board_info {
    pub board_type: u32,
    pub port_num: u32,
    pub port_speed: u32,
    pub pcie_width: u32,
    pub host_num: u32,
    pub pf_num: u32,
    pub vf_total_num: u32,
    pub tile_num: u32,
    pub qcm_num: u32,
    pub core_num: u32,
    pub work_mode: u32,
    pub service_mode: u32,
    pub pcie_mode: u32,
    pub cfg_addr: u32,
    pub boot_sel: u32,
    pub board_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_comm_board_info {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub info: hinic_board_info,
    pub rsvd1: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_hwdev {
    pub hwif: *mut hinic_hwif,
    pub msix_entries: *mut msix_entry,
    pub aeqs: hinic_aeqs,
    pub func_to_io: hinic_func_to_io,
    pub func_to_func: *mut hinic_mbox_func_to_func,
    pub nic_cap: hinic_cap,
    pub port_id: u8,
    pub devlink_dev: *mut hinic_devlink_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_nic_cb {
    pub out_size): *mut u16,
    pub handle: *mut c_void,
    pub cb_state: c_ulong,
}

pub const HINIC_COMM_SELF_CMD_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_mgmt_self_msg_sub_info {
    pub cmd: u8,
    pub proc: comm_mgmt_self_msg_proc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_mgmt_self_msg_info {
    pub cmd_num: u8,
    pub info: [comm_mgmt_self_msg_sub_info; HINIC_COMM_SELF_CMD_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_pfhwdev {
    pub hwdev: hinic_hwdev,
    pub pf_to_mgmt: hinic_pf_to_mgmt,
    pub nic_cb: [hinic_nic_cb; HINIC_MGMT_NUM_MSG_CMD],
    pub proc: comm_mgmt_self_msg_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_dev_cap {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub rsvd1: [u8; 5],
    pub intr_type: u8,
    pub max_cos_id: u8,
    pub er_id: u8,
    pub port_id: u8,
    pub max_vf: u8,
    pub rsvd2: [u8; 62],
    pub max_sqs: u16,
    pub max_rqs: u16,
    pub max_vf_sqs: u16,
    pub max_vf_rqs: u16,
    pub rsvd3: [u8; 204],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hinic_fault_hw_mgmt {
    pub val: [u32; 4],
// valid only type == FAULT_TYPE_CHIP
    pub node_id: u8,
    pub err_level: u8,
    pub err_type: u16,
    pub err_csr_addr: u32,
    pub err_csr_value: u32,
// func_id valid only if err_level == FAULT_LEVEL_SERIOUS_FLR
    pub func_id: u16,
    pub rsvd2: u16,
    pub chip: },
// valid only if type == FAULT_TYPE_UCODE
    pub cause_id: u8,
    pub core_id: u8,
    pub c_id: u8,
    pub rsvd3: u8,
    pub epc: u32,
    pub rsvd4: u32,
    pub rsvd5: u32,
    pub ucode: },
// valid only if type == FAULT_TYPE_MEM_RD_TIMEOUT ||
// FAULT_TYPE_MEM_WR_TIMEOUT
//
    pub err_csr_ctrl: u32,
    pub err_csr_data: u32,
    pub ctrl_tab: u32,
    pub mem_index: u32,
    pub mem_timeout: },
// valid only if type == FAULT_TYPE_REG_RD_TIMEOUT ||
// FAULT_TYPE_REG_WR_TIMEOUT
//
    pub err_csr: u32,
    pub rsvd6: u32,
    pub rsvd7: u32,
    pub rsvd8: u32,
    pub reg_timeout: },
// 0: read; 1: write
    pub op_type: u8,
    pub port_id: u8,
    pub dev_ad: u8,
    pub rsvd9: u8,
    pub csr_addr: u32,
    pub op_data: u32,
    pub rsvd10: u32,
    pub phy_fault: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_fault_event {
    pub type: u8,
    pub fault_level: u8,
    pub rsvd0: [u8; 2],
    pub event: hinic_fault_hw_mgmt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_fault_event {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub event: hinic_fault_event,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_fault_type {
    FAULT_TYPE_CHIP,
    FAULT_TYPE_UCODE,
    FAULT_TYPE_MEM_RD_TIMEOUT,
    FAULT_TYPE_MEM_WR_TIMEOUT,
    FAULT_TYPE_REG_RD_TIMEOUT,
    FAULT_TYPE_REG_WR_TIMEOUT,
    FAULT_TYPE_PHY_FAULT,
    FAULT_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_fault_err_level {
    FAULT_LEVEL_FATAL,
    FAULT_LEVEL_SERIOUS_RESET,
    FAULT_LEVEL_SERIOUS_FLR,
    FAULT_LEVEL_GENERAL,
    FAULT_LEVEL_SUGGESTION,
    FAULT_LEVEL_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_mgmt_watchdog_info {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub curr_time_h: u32,
    pub curr_time_l: u32,
    pub task_id: u32,
    pub rsv: u32,
    pub reg: [u32; 13],
    pub pc: u32,
    pub lr: u32,
    pub cpsr: u32,
    pub stack_top: u32,
    pub stack_bottom: u32,
    pub sp: u32,
    pub curr_used: u32,
    pub peak_used: u32,
    pub is_overflow: u32,
    pub stack_actlen: u32,
    pub data: [u8; 1024],
}

extern "C" {
    pub fn hinic_hwdev_ifup(hwdev: *mut hinic_hwdev, sq_depth: u16, rq_depth: u16) -> c_int;
}
extern "C" {
    pub fn hinic_hwdev_ifdown(hwdev: *mut hinic_hwdev);
}
extern "C" {
    pub fn hinic_free_hwdev(hwdev: *mut hinic_hwdev);
}
extern "C" {
    pub fn hinic_hwdev_num_qps(hwdev: *mut hinic_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic_hwdev_msix_cnt_set(hwdev: *mut hinic_hwdev, msix_index: u16) -> c_int;
}
