//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_wqe.h
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

pub const HINIC_CMDQ_CTRL_PI_SHIFT: c_int = 0;
pub const HINIC_CMDQ_CTRL_CMD_SHIFT: c_int = 16;
pub const HINIC_CMDQ_CTRL_MOD_SHIFT: c_int = 24;
pub const HINIC_CMDQ_CTRL_ACK_TYPE_SHIFT: c_int = 29;
pub const HINIC_CMDQ_CTRL_HW_BUSY_BIT_SHIFT: c_int = 31;
pub const HINIC_CMDQ_CTRL_PI_MASK: c_uint = 0xFFFF;
pub const HINIC_CMDQ_CTRL_CMD_MASK: c_uint = 0xFF;
pub const HINIC_CMDQ_CTRL_MOD_MASK: c_uint = 0x1F;
pub const HINIC_CMDQ_CTRL_ACK_TYPE_MASK: c_uint = 0x3;
pub const HINIC_CMDQ_CTRL_HW_BUSY_BIT_MASK: c_uint = 0x1;

pub const HINIC_CMDQ_WQE_HEADER_BUFDESC_LEN_SHIFT: c_int = 0;
pub const HINIC_CMDQ_WQE_HEADER_COMPLETE_FMT_SHIFT: c_int = 15;
pub const HINIC_CMDQ_WQE_HEADER_DATA_FMT_SHIFT: c_int = 22;
pub const HINIC_CMDQ_WQE_HEADER_COMPLETE_REQ_SHIFT: c_int = 23;
pub const HINIC_CMDQ_WQE_HEADER_COMPLETE_SECT_LEN_SHIFT: c_int = 27;
pub const HINIC_CMDQ_WQE_HEADER_CTRL_LEN_SHIFT: c_int = 29;
pub const HINIC_CMDQ_WQE_HEADER_TOGGLED_WRAPPED_SHIFT: c_int = 31;
pub const HINIC_CMDQ_WQE_HEADER_BUFDESC_LEN_MASK: c_uint = 0xFF;
pub const HINIC_CMDQ_WQE_HEADER_COMPLETE_FMT_MASK: c_uint = 0x1;
pub const HINIC_CMDQ_WQE_HEADER_DATA_FMT_MASK: c_uint = 0x1;
pub const HINIC_CMDQ_WQE_HEADER_COMPLETE_REQ_MASK: c_uint = 0x1;
pub const HINIC_CMDQ_WQE_HEADER_COMPLETE_SECT_LEN_MASK: c_uint = 0x3;
pub const HINIC_CMDQ_WQE_HEADER_CTRL_LEN_MASK: c_uint = 0x3;
pub const HINIC_CMDQ_WQE_HEADER_TOGGLED_WRAPPED_MASK: c_uint = 0x1;

pub const HINIC_SQ_CTRL_BUFDESC_SECT_LEN_SHIFT: c_int = 0;
pub const HINIC_SQ_CTRL_TASKSECT_LEN_SHIFT: c_int = 16;
pub const HINIC_SQ_CTRL_DATA_FORMAT_SHIFT: c_int = 22;
pub const HINIC_SQ_CTRL_LEN_SHIFT: c_int = 29;
pub const HINIC_SQ_CTRL_BUFDESC_SECT_LEN_MASK: c_uint = 0xFF;
pub const HINIC_SQ_CTRL_TASKSECT_LEN_MASK: c_uint = 0x1F;
pub const HINIC_SQ_CTRL_DATA_FORMAT_MASK: c_uint = 0x1;
pub const HINIC_SQ_CTRL_LEN_MASK: c_uint = 0x3;
pub const HINIC_SQ_CTRL_QUEUE_INFO_PLDOFF_SHIFT: c_int = 2;
pub const HINIC_SQ_CTRL_QUEUE_INFO_UFO_SHIFT: c_int = 10;
pub const HINIC_SQ_CTRL_QUEUE_INFO_TSO_SHIFT: c_int = 11;
pub const HINIC_SQ_CTRL_QUEUE_INFO_TCPUDP_CS_SHIFT: c_int = 12;
pub const HINIC_SQ_CTRL_QUEUE_INFO_MSS_SHIFT: c_int = 13;
pub const HINIC_SQ_CTRL_QUEUE_INFO_SCTP_SHIFT: c_int = 27;
pub const HINIC_SQ_CTRL_QUEUE_INFO_UC_SHIFT: c_int = 28;
pub const HINIC_SQ_CTRL_QUEUE_INFO_PRI_SHIFT: c_int = 29;
pub const HINIC_SQ_CTRL_QUEUE_INFO_PLDOFF_MASK: c_uint = 0xFF;
pub const HINIC_SQ_CTRL_QUEUE_INFO_UFO_MASK: c_uint = 0x1;
pub const HINIC_SQ_CTRL_QUEUE_INFO_TSO_MASK: c_uint = 0x1;
pub const HINIC_SQ_CTRL_QUEUE_INFO_TCPUDP_CS_MASK: c_uint = 0x1;
pub const HINIC_SQ_CTRL_QUEUE_INFO_MSS_MASK: c_uint = 0x3FFF;
pub const HINIC_SQ_CTRL_QUEUE_INFO_SCTP_MASK: c_uint = 0x1;
pub const HINIC_SQ_CTRL_QUEUE_INFO_UC_MASK: c_uint = 0x1;
pub const HINIC_SQ_CTRL_QUEUE_INFO_PRI_MASK: c_uint = 0x7;

pub const HINIC_SQ_TASK_INFO0_L2HDR_LEN_SHIFT: c_int = 0;
pub const HINIC_SQ_TASK_INFO0_L4_OFFLOAD_SHIFT: c_int = 8;
pub const HINIC_SQ_TASK_INFO0_INNER_L3TYPE_SHIFT: c_int = 10;
pub const HINIC_SQ_TASK_INFO0_VLAN_OFFLOAD_SHIFT: c_int = 12;
pub const HINIC_SQ_TASK_INFO0_PARSE_FLAG_SHIFT: c_int = 13;
// 1 bit reserved
pub const HINIC_SQ_TASK_INFO0_TSO_FLAG_SHIFT: c_int = 15;
pub const HINIC_SQ_TASK_INFO0_VLAN_TAG_SHIFT: c_int = 16;
pub const HINIC_SQ_TASK_INFO0_L2HDR_LEN_MASK: c_uint = 0xFF;
pub const HINIC_SQ_TASK_INFO0_L4_OFFLOAD_MASK: c_uint = 0x3;
pub const HINIC_SQ_TASK_INFO0_INNER_L3TYPE_MASK: c_uint = 0x3;
pub const HINIC_SQ_TASK_INFO0_VLAN_OFFLOAD_MASK: c_uint = 0x1;
pub const HINIC_SQ_TASK_INFO0_PARSE_FLAG_MASK: c_uint = 0x1;
// 1 bit reserved
pub const HINIC_SQ_TASK_INFO0_TSO_FLAG_MASK: c_uint = 0x1;
pub const HINIC_SQ_TASK_INFO0_VLAN_TAG_MASK: c_uint = 0xFFFF;

// 8 bits reserved
pub const HINIC_SQ_TASK_INFO1_MEDIA_TYPE_SHIFT: c_int = 8;
pub const HINIC_SQ_TASK_INFO1_INNER_L4LEN_SHIFT: c_int = 16;
pub const HINIC_SQ_TASK_INFO1_INNER_L3LEN_SHIFT: c_int = 24;
// 8 bits reserved
pub const HINIC_SQ_TASK_INFO1_MEDIA_TYPE_MASK: c_uint = 0xFF;
pub const HINIC_SQ_TASK_INFO1_INNER_L4LEN_MASK: c_uint = 0xFF;
pub const HINIC_SQ_TASK_INFO1_INNER_L3LEN_MASK: c_uint = 0xFF;

pub const HINIC_SQ_TASK_INFO2_TUNNEL_L4LEN_SHIFT: c_int = 0;
pub const HINIC_SQ_TASK_INFO2_OUTER_L3LEN_SHIFT: c_int = 8;
pub const HINIC_SQ_TASK_INFO2_TUNNEL_L4TYPE_SHIFT: c_int = 16;
// 1 bit reserved
pub const HINIC_SQ_TASK_INFO2_OUTER_L3TYPE_SHIFT: c_int = 24;
// 8 bits reserved
pub const HINIC_SQ_TASK_INFO2_TUNNEL_L4LEN_MASK: c_uint = 0xFF;
pub const HINIC_SQ_TASK_INFO2_OUTER_L3LEN_MASK: c_uint = 0xFF;
pub const HINIC_SQ_TASK_INFO2_TUNNEL_L4TYPE_MASK: c_uint = 0x7;
// 1 bit reserved
pub const HINIC_SQ_TASK_INFO2_OUTER_L3TYPE_MASK: c_uint = 0x3;
// 8 bits reserved

// 31 bits reserved
pub const HINIC_SQ_TASK_INFO4_L2TYPE_SHIFT: c_int = 31;
// 31 bits reserved
pub const HINIC_SQ_TASK_INFO4_L2TYPE_MASK: c_uint = 0x1;

pub const HINIC_RQ_CQE_STATUS_RXDONE_SHIFT: c_int = 31;
pub const HINIC_RQ_CQE_STATUS_RXDONE_MASK: c_uint = 0x1;
pub const HINIC_RQ_CQE_STATUS_CSUM_ERR_SHIFT: c_int = 0;
pub const HINIC_RQ_CQE_STATUS_CSUM_ERR_MASK: c_uint = 0xFFFFU;

pub const HINIC_RQ_CQE_SGE_LEN_SHIFT: c_int = 16;
pub const HINIC_RQ_CQE_SGE_LEN_MASK: c_uint = 0xFFFF;

pub const HINIC_RQ_CTRL_BUFDESC_SECT_LEN_SHIFT: c_int = 0;
pub const HINIC_RQ_CTRL_COMPLETE_FORMAT_SHIFT: c_int = 15;
pub const HINIC_RQ_CTRL_COMPLETE_LEN_SHIFT: c_int = 27;
pub const HINIC_RQ_CTRL_LEN_SHIFT: c_int = 29;
pub const HINIC_RQ_CTRL_BUFDESC_SECT_LEN_MASK: c_uint = 0xFF;
pub const HINIC_RQ_CTRL_COMPLETE_FORMAT_MASK: c_uint = 0x1;
pub const HINIC_RQ_CTRL_COMPLETE_LEN_MASK: c_uint = 0x3;
pub const HINIC_RQ_CTRL_LEN_MASK: c_uint = 0x3;

pub const HINIC_SCMD_DATA_LEN: c_int = 16;
pub const HINIC_MAX_SQ_BUFDESCS: c_int = 17;
pub const HINIC_SQ_WQE_MAX_SIZE: c_int = 320;
pub const HINIC_RQ_WQE_SIZE: c_int = 32;
pub const HINIC_MSS_DEFAULT: c_uint = 0x3E00;
pub const HINIC_MSS_MIN: c_uint = 0x50;
pub const RQ_CQE_STATUS_NUM_LRO_SHIFT: c_int = 16;
pub const RQ_CQE_STATUS_NUM_LRO_MASK: c_uint = 0xFFU;

pub const RQ_CQE_OFFOLAD_TYPE_PKT_TYPE_SHIFT: c_int = 0;
pub const RQ_CQE_OFFOLAD_TYPE_PKT_TYPE_MASK: c_uint = 0xFFFU;
pub const RQ_CQE_OFFOLAD_TYPE_VLAN_EN_SHIFT: c_int = 21;
pub const RQ_CQE_OFFOLAD_TYPE_VLAN_EN_MASK: c_uint = 0x1U;

pub const RQ_CQE_SGE_VLAN_MASK: c_uint = 0xFFFFU;
pub const RQ_CQE_SGE_VLAN_SHIFT: c_int = 0;

pub const HINIC_RSS_TYPE_VALID_SHIFT: c_int = 23;
pub const HINIC_RSS_TYPE_TCP_IPV6_EXT_SHIFT: c_int = 24;
pub const HINIC_RSS_TYPE_IPV6_EXT_SHIFT: c_int = 25;
pub const HINIC_RSS_TYPE_TCP_IPV6_SHIFT: c_int = 26;
pub const HINIC_RSS_TYPE_IPV6_SHIFT: c_int = 27;
pub const HINIC_RSS_TYPE_TCP_IPV4_SHIFT: c_int = 28;
pub const HINIC_RSS_TYPE_IPV4_SHIFT: c_int = 29;
pub const HINIC_RSS_TYPE_UDP_IPV6_SHIFT: c_int = 30;
pub const HINIC_RSS_TYPE_UDP_IPV4_SHIFT: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_l3_offload_type {
    L3TYPE_UNKNOWN = 0,
    IPV6_PKT = 1,
    IPV4_PKT_NO_CHKSUM_OFFLOAD = 2,
    IPV4_PKT_WITH_CHKSUM_OFFLOAD = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_l4_offload_type {
    OFFLOAD_DISABLE     = 0,
    TCP_OFFLOAD_ENABLE  = 1,
    SCTP_OFFLOAD_ENABLE = 2,
    UDP_OFFLOAD_ENABLE  = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_l4_tunnel_type {
    NOT_TUNNEL,
    TUNNEL_UDP_NO_CSUM,
    TUNNEL_UDP_CSUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_outer_l3type {
    HINIC_OUTER_L3TYPE_UNKNOWN              = 0,
    HINIC_OUTER_L3TYPE_IPV6                 = 1,
    HINIC_OUTER_L3TYPE_IPV4_NO_CHKSUM       = 2,
    HINIC_OUTER_L3TYPE_IPV4_CHKSUM          = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_l2type {
    HINIC_L2TYPE_ETH = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_header {
    pub header_info: u32,
    pub saved_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_status {
    pub status_info: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_ctrl {
    pub ctrl_info: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sge_resp {
    pub sge: hinic_sge,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_completion {
// HW Format
    pub sge_resp: hinic_sge_resp,
    pub direct_resp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_scmd_bufdesc {
    pub buf_len: u32,
    pub rsvd: u32,
    pub data: [u8; HINIC_SCMD_DATA_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_lcmd_bufdesc {
    pub sge: hinic_sge,
    pub rsvd1: u32,
    pub rsvd2: u64,
    pub rsvd3: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_wqe_scmd {
    pub header: hinic_cmdq_header,
    pub rsvd: u64,
    pub status: hinic_status,
    pub ctrl: hinic_ctrl,
    pub completion: hinic_cmdq_completion,
    pub buf_desc: hinic_scmd_bufdesc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_wqe_lcmd {
    pub header: hinic_cmdq_header,
    pub status: hinic_status,
    pub ctrl: hinic_ctrl,
    pub completion: hinic_cmdq_completion,
    pub buf_desc: hinic_lcmd_bufdesc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_direct_wqe {
    pub wqe_scmd: hinic_cmdq_wqe_scmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_wqe {
// HW Format
    pub direct_wqe: hinic_cmdq_direct_wqe,
    pub wqe_lcmd: hinic_cmdq_wqe_lcmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sq_ctrl {
    pub ctrl_info: u32,
    pub queue_info: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sq_task {
    pub pkt_info0: u32,
    pub pkt_info1: u32,
    pub pkt_info2: u32,
    pub ufo_v6_identify: u32,
    pub pkt_info4: u32,
    pub zero_pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sq_bufdesc {
    pub sge: hinic_sge,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sq_wqe {
    pub ctrl: hinic_sq_ctrl,
    pub task: hinic_sq_task,
    pub buf_descs: [hinic_sq_bufdesc; HINIC_MAX_SQ_BUFDESCS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rq_cqe {
    pub status: u32,
    pub len: u32,
    pub offload_type: u32,
    pub rsvd3: u32,
    pub rsvd4: u32,
    pub rsvd5: u32,
    pub rsvd6: u32,
    pub rsvd7: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rq_ctrl {
    pub ctrl_info: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rq_cqe_sect {
    pub sge: hinic_sge,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rq_bufdesc {
    pub hi_addr: u32,
    pub lo_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rq_wqe {
    pub ctrl: hinic_rq_ctrl,
    pub rsvd: u32,
    pub cqe_sect: hinic_rq_cqe_sect,
    pub buf_desc: hinic_rq_bufdesc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_hw_wqe {
// HW Format
    pub cmdq_wqe: hinic_cmdq_wqe,
    pub sq_wqe: hinic_sq_wqe,
    pub rq_wqe: hinic_rq_wqe,
}
