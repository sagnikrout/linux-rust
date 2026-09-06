//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hclge_mbx.h
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
// Copyright (c) 2016-2017 Hisilicon Limited.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_MBX_OPCODE {
    HCLGE_MBX_RESET = 0x01,		/* (VF -> PF) assert reset */
    HCLGE_MBX_ASSERTING_RESET,	/* (PF -> VF) PF is asserting reset */
    HCLGE_MBX_SET_UNICAST,		/* (VF -> PF) set UC addr */
    HCLGE_MBX_SET_MULTICAST,	/* (VF -> PF) set MC addr */
    HCLGE_MBX_SET_VLAN,		/* (VF -> PF) set VLAN */
    HCLGE_MBX_MAP_RING_TO_VECTOR,	/* (VF -> PF) map ring-to-vector */
    HCLGE_MBX_UNMAP_RING_TO_VECTOR,	/* (VF -> PF) unamp ring-to-vector */
    HCLGE_MBX_SET_PROMISC_MODE,	/* (VF -> PF) set promiscuous mode */
    HCLGE_MBX_SET_MACVLAN,		/* (VF -> PF) set unicast filter */
    HCLGE_MBX_API_NEGOTIATE,	/* (VF -> PF) negotiate API version */
    HCLGE_MBX_GET_QINFO,		/* (VF -> PF) get queue config */
    HCLGE_MBX_GET_QDEPTH,		/* (VF -> PF) get queue depth */
    HCLGE_MBX_GET_BASIC_INFO,	/* (VF -> PF) get basic info */
    HCLGE_MBX_GET_RETA,		/* (VF -> PF) get RETA */
    HCLGE_MBX_GET_RSS_KEY,		/* (VF -> PF) get RSS key */
    HCLGE_MBX_GET_MAC_ADDR,		/* (VF -> PF) get MAC addr */
    HCLGE_MBX_PF_VF_RESP,		/* (PF -> VF) generate response to VF */
    HCLGE_MBX_GET_BDNUM,		/* (VF -> PF) get BD num */
    HCLGE_MBX_GET_BUFSIZE,		/* (VF -> PF) get buffer size */
    HCLGE_MBX_GET_STREAMID,		/* (VF -> PF) get stream id */
    HCLGE_MBX_SET_AESTART,		/* (VF -> PF) start ae */
    HCLGE_MBX_SET_TSOSTATS,		/* (VF -> PF) get tso stats */
    HCLGE_MBX_LINK_STAT_CHANGE,	/* (PF -> VF) link status has changed */
    HCLGE_MBX_GET_BASE_CONFIG,	/* (VF -> PF) get config */
    HCLGE_MBX_BIND_FUNC_QUEUE,	/* (VF -> PF) bind function and queue */
    HCLGE_MBX_GET_LINK_STATUS,	/* (VF -> PF) get link status */
    HCLGE_MBX_QUEUE_RESET,		/* (VF -> PF) reset queue */
    HCLGE_MBX_KEEP_ALIVE,		/* (VF -> PF) send keep alive cmd */
    HCLGE_MBX_SET_ALIVE,		/* (VF -> PF) set alive state */
    HCLGE_MBX_SET_MTU,		/* (VF -> PF) set mtu */
    HCLGE_MBX_GET_QID_IN_PF,	/* (VF -> PF) get queue id in pf */
    HCLGE_MBX_LINK_STAT_MODE,	/* (PF -> VF) link mode has changed */
    HCLGE_MBX_GET_LINK_MODE,	/* (VF -> PF) get the link mode of pf */
    HCLGE_MBX_PUSH_VLAN_INFO,	/* (PF -> VF) push port base vlan */
    HCLGE_MBX_GET_MEDIA_TYPE,       /* (VF -> PF) get media type */
    HCLGE_MBX_PUSH_PROMISC_INFO,	/* (PF -> VF) push vf promisc info */
    HCLGE_MBX_VF_UNINIT,            /* (VF -> PF) vf is unintializing */
    HCLGE_MBX_HANDLE_VF_TBL,	/* (VF -> PF) store/clear hw table */
    HCLGE_MBX_GET_RING_VECTOR_MAP,	/* (VF -> PF) get ring-to-vector map */

    HCLGE_MBX_GET_VF_FLR_STATUS = 200, /* (M7 -> PF) get vf flr status */
    HCLGE_MBX_PUSH_LINK_STATUS,	/* (M7 -> PF) get port link status */
    HCLGE_MBX_NCSI_ERROR,		/* (M7 -> PF) receive a NCSI error */
}

// below are per-VF mac-vlan subcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_mbx_mac_vlan_subcode {
    HCLGE_MBX_MAC_VLAN_UC_MODIFY = 0,	/* modify UC mac addr */
    HCLGE_MBX_MAC_VLAN_UC_ADD,		/* add a new UC mac addr */
    HCLGE_MBX_MAC_VLAN_UC_REMOVE,		/* remove a new UC mac addr */
    HCLGE_MBX_MAC_VLAN_MC_MODIFY,		/* modify MC mac addr */
    HCLGE_MBX_MAC_VLAN_MC_ADD,		/* add new MC mac addr */
    HCLGE_MBX_MAC_VLAN_MC_REMOVE,		/* remove MC mac addr */
}

// below are per-VF vlan cfg subcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_mbx_vlan_cfg_subcode {
    HCLGE_MBX_VLAN_FILTER = 0,	/* set vlan filter */
    HCLGE_MBX_VLAN_TX_OFF_CFG,	/* set tx side vlan offload */
    HCLGE_MBX_VLAN_RX_OFF_CFG,	/* set rx side vlan offload */
    HCLGE_MBX_PORT_BASE_VLAN_CFG,	/* set port based vlan configuration */
    HCLGE_MBX_GET_PORT_BASE_VLAN_STATE,	/* get port based vlan state */
    HCLGE_MBX_ENABLE_VLAN_FILTER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_mbx_tbl_cfg_subcode {
    HCLGE_MBX_VPORT_LIST_CLEAR,
}

pub const HCLGE_MBX_MAX_MSG_SIZE: c_int = 14;

pub const HCLGE_MBX_MAX_RING_CHAIN_PARAM_NUM: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_ring_chain_param {
    pub ring_type: u8,
    pub tqp_index: u8,
    pub int_gl_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_basic_info {
    pub hw_tc_map: u8,
    pub rsv: u8,
    pub mbx_api_version: __le16,
    pub pf_caps: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_mbx_resp_status {
    pub /: *mut *mut mutex mbx_mutex; / protects against contending sync cmd resp,
    pub origin_mbx_msg: u32,
    pub received_resp: bool,
    pub resp_status: c_int,
    pub match_id: u16,
    pub additional_info: [u8; HCLGE_MBX_MAX_RESP_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_respond_to_vf_msg {
    pub status: c_int,
    pub data: [u8; HCLGE_MBX_MAX_RESP_DATA_SIZE],
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vf_to_pf_msg {
    pub code: u8,
    pub subcode: u8,
    pub data: [u8; HCLGE_MBX_MAX_MSG_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pf_to_vf_msg {
    pub code: __le16,
// used for mbx response
    pub vf_mbx_msg_code: __le16,
    pub vf_mbx_msg_subcode: __le16,
    pub resp_status: __le16,
    pub resp_data: [u8; HCLGE_MBX_MAX_RESP_DATA_SIZE],
}

// used for general mbx
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_vf_to_pf_cmd {
    pub rsv: u8,
    pub /: *mut *mut u8 mbx_src_vfid; / Auto filled by IMP,
    pub mbx_need_resp: u8,
    pub rsv1: [u8; 1],
    pub msg_len: u8,
    pub rsv2: u8,
    pub match_id: __le16,
    pub msg: hclge_vf_to_pf_msg,
}

pub const HCLGE_MBX_NEED_RESP_B: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_pf_to_vf_cmd {
    pub dest_vfid: u8,
    pub rsv: [u8; 3],
    pub msg_len: u8,
    pub rsv1: u8,
    pub match_id: __le16,
    pub msg: hclge_pf_to_vf_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vf_rst_cmd {
    pub dest_vfid: u8,
    pub vf_rst: u8,
    pub rsv: [u8; 22],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_link_status {
    pub link_status: __le16,
    pub speed: __le32,
    pub duplex: __le16,
    pub flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_link_mode {
    pub idx: __le16,
    pub link_mode: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_port_base_vlan {
    pub state: __le16,
    pub vlan_proto: __le16,
    pub qos: __le16,
    pub vlan_tag: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_vf_queue_info {
    pub num_tqps: __le16,
    pub rss_size: __le16,
    pub rx_buf_len: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_vf_queue_depth {
    pub num_tx_desc: __le16,
    pub num_rx_desc: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_vlan_filter {
    pub is_kill: u8,
    pub vlan_id: __le16,
    pub proto: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_mtu_info {
    pub mtu: __le32,
}

// used by VF to store the received Async responses from PF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_mbx_arq_ring {
pub const HCLGE_MBX_MAX_ARQ_MSG_SIZE: c_int = 8;
pub const HCLGE_MBX_MAX_ARQ_MSG_NUM: c_int = 1024;
    pub hdev: *mut hclgevf_dev,
    pub head: u32,
    pub tail: u32,
    pub count: core::sync::atomic::AtomicI32,
    pub msg_q: [__le16; HCLGE_MBX_MAX_ARQ_MSG_NUM][HCLGE_MBX_MAX_ARQ_MSG_SIZE],
}

pub const HCLGE_MBX_OPCODE_MAX: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mbx_ops_param {
    pub vport: *mut hclge_vport,
    pub req: *mut hclge_mbx_vf_to_pf_cmd,
    pub resp_msg: *mut hclge_respond_to_vf_msg,
}

extern "C" {
    pub fn int(param: *mut *mut hclge_mbx_ops_fn)(struct hclge_mbx_ops_param) -> typedef;
}

// PF immediately push link status to VFs when link status changed

