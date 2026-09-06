//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/hisilicon/kunpeng_hccs.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2023 Hisilicon Limited.
//
// |---------------  Chip0  ---------------|----------------  ChipN  -------------|
// |--------Die0-------|--------DieN-------|--------Die0-------|-------DieN-------|
// | P0 | P1 | P2 | P3 | P0 | P1 | P2 | P3 | P0 | P1 | P2 | P3 |P0 | P1 | P2 | P3 |
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hccs_port_type {
    HCCS_V1 = 1,
    HCCS_V2,
}

pub const HCCS_IP_MAX: c_int = 255;
pub const HCCS_NAME_MAX_LEN: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_type_name_map {
    pub type: u8,
    pub 1]: char name[HCCS_NAME_MAX_LEN +,
}

//
// This value cannot be 255, otherwise the loop of the multi-BD communication
// case cannot end.
//
pub const HCCS_DIE_MAX_PORT_ID: c_int = 254;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_port_info {
    pub port_id: u8,
    pub port_type: u8,
    pub max_lane_num: u8,
    pub /: *mut *mut bool enable; / if the port is enabled,
    pub kobj: kobject,
    pub dir_created: bool,
    pub /: *mut *mut *mut hccs_die_info die; / point to the die the port is located,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_die_info {
    pub die_id: u8,
    pub port_num: u8,
    pub min_port_id: u8,
    pub max_port_id: u8,
    pub ports: *mut hccs_port_info,
    pub kobj: kobject,
    pub dir_created: bool,
    pub /: *mut *mut *mut hccs_chip_info chip; / point to the chip the die is located,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_chip_info {
    pub chip_id: u8,
    pub die_num: u8,
    pub dies: *mut hccs_die_info,
    pub kobj: kobject,
    pub hdev: *mut hccs_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_mbox_client_info {
    pub client: mbox_client,
    pub pcc_chan: *mut pcc_mbox_chan,
    pub deadline_us: u64,
    pub done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_verspecific_data {
    pub mssg): *mut *mut *mut void (rx_callback)(struct mbox_client cl, void,
    pub hdev): *mut *mut int (wait_cmd_complete)(struct hccs_dev,
    pub space_size): u16,
    pub shared_mem_size: u16,
    pub has_txdone_irq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_dev {
    pub dev: *mut device,
    pub acpi_dev: *mut acpi_device,
    pub verspec_data: *const hccs_verspecific_data,
// device capabilities from firmware, like HCCS_CAPS_xxx.
    pub caps: u64,
    pub chip_num: u8,
    pub chips: *mut hccs_chip_info,
    pub used_type_num: u16,
    pub type_name_maps: *mut hccs_type_name_map,
    pub chan_id: u8,
    pub lock: mutex,
    pub cl_info: hccs_mbox_client_info,
}

pub const HCCS_SERDES_MODULE_CODE: c_uint = 0x32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hccs_subcmd_type {
    HCCS_GET_CHIP_NUM = 0x1,
    HCCS_GET_DIE_NUM,
    HCCS_GET_DIE_INFO,
    HCCS_GET_DIE_PORT_INFO,
    HCCS_GET_DEV_CAP,
    HCCS_GET_PORT_LINK_STATUS,
    HCCS_GET_PORT_CRC_ERR_CNT,
    HCCS_GET_DIE_PORTS_LANE_STA,
    HCCS_GET_DIE_PORTS_LINK_STA,
    HCCS_GET_DIE_PORTS_CRC_ERR_CNT,
    HCCS_GET_PORT_IDLE_STATUS,
    HCCS_PM_DEC_LANE,
    HCCS_PM_INC_LANE,
    HCCS_SUB_CMD_MAX = 255,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_die_num_req_param {
    pub chip_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_die_info_req_param {
    pub chip_id: u8,
    pub die_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_die_info_rsp_data {
    pub die_id: u8,
    pub port_num: u8,
    pub min_port_id: u8,
    pub max_port_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_port_attr {
    pub port_id: u8,
    pub port_type: u8,
    pub max_lane_num: u8,
    pub /: *mut *mut u8 enable : 1; / if the port is enabled,
    pub rsv: [u16; 2],
}

//
// The common command request for getting the information of all HCCS port on
// specified DIE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_die_comm_req_param {
    pub chip_id: u8,
    pub /: *mut *mut u8 die_id; / id in hardware,
}

// The common command request for getting the information of a specific port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_port_comm_req_param {
    pub chip_id: u8,
    pub die_id: u8,
    pub port_id: u8,
}

pub const HCCS_PREPARE_INC_LANE: c_int = 1;
pub const HCCS_GET_ADAPT_RES: c_int = 2;
pub const HCCS_START_RETRAINING: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_inc_lane_req_param {
    pub port_type: u8,
    pub opt_type: u8,
}

pub const HCCS_PORT_RESET: c_int = 1;
pub const HCCS_PORT_SETUP: c_int = 2;
pub const HCCS_PORT_CONFIG: c_int = 3;
pub const HCCS_PORT_READY: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_link_status {
    pub /: *mut *mut u8 lane_mask; / indicate which lanes are used.,
    pub /: *mut *mut u8 link_fsm : 3; / link fsm, 1: reset 2: setup 3: config 4: link-up,
    pub /: *mut *mut u8 lane_num : 5; / current lane number,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_req_head {
    pub /: *mut *mut u8 module_code; / set to 0x32 for serdes,
    pub start_id: u8,
    pub rsv: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_rsp_head {
    pub data_len: u8,
    pub next_id: u8,
    pub rsv: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_fw_inner_head {
    pub /: *mut *mut u8 retStatus; / 0: success, other: failure,
    pub rsv: [u8; 7],
}

pub const HCCS_PCC_SHARE_MEM_BYTES: c_int = 64;
pub const HCCS_FW_INNER_HEAD_BYTES: c_int = 8;
pub const HCCS_RSP_HEAD_BYTES: c_int = 4;

//
// Note: Actual available size of data field also depands on the PCC header
// bytes of the specific type. Driver needs to copy the response data in the
// communication space based on the real length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_rsp_desc {
    pub /: *mut *mut hccs_fw_inner_head fw_inner_head; / 8 Bytes,
    pub /: *mut *mut hccs_rsp_head rsp_head; / 4 Bytes,
    pub data: [u32; HCCS_MAX_RSP_DATA_SIZE_MAX],
}

pub const HCCS_REQ_HEAD_BYTES: c_int = 4;

//
// Note: Actual available size of data field also depands on the PCC header
// bytes of the specific type. Driver needs to copy the request data to the
// communication space based on the real length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_req_desc {
    pub /: *mut *mut hccs_req_head req_head; / 4 Bytes,
    pub data: [u32; HCCS_MAX_REQ_DATA_SIZE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hccs_desc {
    pub req: hccs_req_desc,
    pub rsp: hccs_rsp_desc,
}
