//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_mgmt.h
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

pub const HINIC_MSG_HEADER_MSG_LEN_SHIFT: c_int = 0;
pub const HINIC_MSG_HEADER_MODULE_SHIFT: c_int = 11;
pub const HINIC_MSG_HEADER_SEG_LEN_SHIFT: c_int = 16;
pub const HINIC_MSG_HEADER_NO_ACK_SHIFT: c_int = 22;
pub const HINIC_MSG_HEADER_ASYNC_MGMT_TO_PF_SHIFT: c_int = 23;
pub const HINIC_MSG_HEADER_SEQID_SHIFT: c_int = 24;
pub const HINIC_MSG_HEADER_LAST_SHIFT: c_int = 30;
pub const HINIC_MSG_HEADER_DIRECTION_SHIFT: c_int = 31;
pub const HINIC_MSG_HEADER_CMD_SHIFT: c_int = 32;
pub const HINIC_MSG_HEADER_ZEROS_SHIFT: c_int = 40;
pub const HINIC_MSG_HEADER_PCI_INTF_SHIFT: c_int = 48;
pub const HINIC_MSG_HEADER_PF_IDX_SHIFT: c_int = 50;
pub const HINIC_MSG_HEADER_MSG_ID_SHIFT: c_int = 54;
pub const HINIC_MSG_HEADER_MSG_LEN_MASK: c_uint = 0x7FF;
pub const HINIC_MSG_HEADER_MODULE_MASK: c_uint = 0x1F;
pub const HINIC_MSG_HEADER_SEG_LEN_MASK: c_uint = 0x3F;
pub const HINIC_MSG_HEADER_NO_ACK_MASK: c_uint = 0x1;
pub const HINIC_MSG_HEADER_ASYNC_MGMT_TO_PF_MASK: c_uint = 0x1;
pub const HINIC_MSG_HEADER_SEQID_MASK: c_uint = 0x3F;
pub const HINIC_MSG_HEADER_LAST_MASK: c_uint = 0x1;
pub const HINIC_MSG_HEADER_DIRECTION_MASK: c_uint = 0x1;
pub const HINIC_MSG_HEADER_CMD_MASK: c_uint = 0xFF;
pub const HINIC_MSG_HEADER_ZEROS_MASK: c_uint = 0xFF;
pub const HINIC_MSG_HEADER_PCI_INTF_MASK: c_uint = 0x3;
pub const HINIC_MSG_HEADER_PF_IDX_MASK: c_uint = 0xF;
pub const HINIC_MSG_HEADER_MSG_ID_MASK: c_uint = 0x3FF;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_mgmt_msg_type {
    HINIC_MGMT_MSG_SYNC = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_cfg_cmd {
    HINIC_CFG_NIC_CAP = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_comm_cmd {
    HINIC_COMM_CMD_START_FLR          = 0x1,
    HINIC_COMM_CMD_IO_STATUS_GET    = 0x3,
    HINIC_COMM_CMD_DMA_ATTR_SET	    = 0x4,

    HINIC_COMM_CMD_CMDQ_CTXT_SET    = 0x10,
    HINIC_COMM_CMD_CMDQ_CTXT_GET    = 0x11,

    HINIC_COMM_CMD_HWCTXT_SET       = 0x12,
    HINIC_COMM_CMD_HWCTXT_GET       = 0x13,

    HINIC_COMM_CMD_SQ_HI_CI_SET     = 0x14,

    HINIC_COMM_CMD_RES_STATE_SET    = 0x24,

    HINIC_COMM_CMD_IO_RES_CLEAR     = 0x29,

    HINIC_COMM_CMD_CEQ_CTRL_REG_WR_BY_UP = 0x33,

    HINIC_COMM_CMD_MSI_CTRL_REG_WR_BY_UP,
    HINIC_COMM_CMD_MSI_CTRL_REG_RD_BY_UP,

    HINIC_COMM_CMD_FAULT_REPORT	= 0x37,

    HINIC_COMM_CMD_SET_LED_STATUS	= 0x4a,

    HINIC_COMM_CMD_L2NIC_RESET	= 0x4b,

    HINIC_COMM_CMD_PAGESIZE_SET	= 0x50,

    HINIC_COMM_CMD_GET_BOARD_INFO	= 0x52,

    HINIC_COMM_CMD_WATCHDOG_INFO	= 0x56,

    HINIC_MGMT_CMD_SET_VF_RANDOM_ID = 0x61,

    HINIC_COMM_CMD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_mgmt_cb_state {
    HINIC_MGMT_CB_ENABLED = BIT(0),
    HINIC_MGMT_CB_RUNNING = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_recv_msg {
    pub msg: *mut u8,
    pub buf_out: *mut u8,
    pub recv_done: completion,
    pub cmd: u16,
    pub mod: hinic_mod_type,
    pub async_mgmt_to_pf: c_int,
    pub msg_len: u16,
    pub msg_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_mgmt_cb {
    pub out_size): *mut *mut void buf_out, u16,
    pub handle: *mut c_void,
    pub state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_pf_to_mgmt {
    pub hwif: *mut hinic_hwif,
    pub hwdev: *mut hinic_hwdev,
    pub sync_msg_lock: semaphore,
    pub sync_msg_id: u16,
    pub sync_msg_buf: *mut u8,
    pub mgmt_ack_buf: *mut c_void,
    pub recv_resp_msg_from_mgmt: hinic_recv_msg,
    pub recv_msg_from_mgmt: hinic_recv_msg,
    pub cmd_chain: [*mut hinic_api_cmd_chain; HINIC_API_CMD_MAX],
    pub mgmt_cb: [hinic_mgmt_cb; HINIC_MOD_MAX],
    pub workq: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_mgmt_msg_handle_work {
    pub work: work_struct,
    pub pf_to_mgmt: *mut hinic_pf_to_mgmt,
    pub msg: *mut c_void,
    pub msg_len: u16,
    pub mod: hinic_mod_type,
    pub cmd: u8,
    pub msg_id: u16,
    pub async_mgmt_to_pf: c_int,
}

extern "C" {
    pub fn hinic_pf_to_mgmt_free(pf_to_mgmt: *mut hinic_pf_to_mgmt);
}
