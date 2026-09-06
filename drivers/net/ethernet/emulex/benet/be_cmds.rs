//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/emulex/benet/be_cmds.h
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
// Copyright (C) 2005 - 2016 Broadcom
// All rights reserved.
//
// Contact Information:
// linux-drivers@emulex.com
//
// Emulex
// 3333 Susan Street
// Costa Mesa, CA 92626
//
// The driver sends configuration and managements command requests to the
// firmware in the BE. These requests are communicated to the processor
// using Work Request Blocks (WRBs) submitted to the MCC-WRB ring or via one
// WRB inside a MAILBOX.
// The commands are serviced by the ARM processor in the BladeEngine's MPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_sge {
    pub pa_lo: u32,
    pub pa_hi: u32,
    pub len: u32,
}

pub const MCC_WRB_SGE_CNT_MASK: c_uint = 0x1F	/* bits 3 - 7 of dword 0 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mcc_wrb {
    pub /: *mut *mut u32 embedded; / dword 0,
    pub /: *mut *mut u32 payload_length; / dword 1,
    pub /: *mut *mut u32 tag0; / dword 2,
    pub /: *mut *mut u32 tag1; / dword 3,
    pub /: *mut *mut u32 rsvd; / dword 4,
    pub /: *mut *mut u8 embedded_payload[236]; / used by embedded cmds,
    pub /: *mut *mut be_sge sgl[19]; / used by non-embedded cmds,
    pub payload: },
}

// Completion Status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcc_base_status {
    MCC_STATUS_SUCCESS = 0,
    MCC_STATUS_FAILED = 1,
    MCC_STATUS_ILLEGAL_REQUEST = 2,
    MCC_STATUS_ILLEGAL_FIELD = 3,
    MCC_STATUS_INSUFFICIENT_BUFFER = 4,
    MCC_STATUS_UNAUTHORIZED_REQUEST = 5,
    MCC_STATUS_NOT_SUPPORTED = 66,
    MCC_STATUS_FEATURE_NOT_SUPPORTED = 68,
    MCC_STATUS_INVALID_LENGTH = 116
}

// Additional status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcc_addl_status {
    MCC_ADDL_STATUS_INSUFFICIENT_RESOURCES = 0x16,
    MCC_ADDL_STATUS_FLASH_IMAGE_CRC_MISMATCH = 0x4d,
    MCC_ADDL_STATUS_TOO_MANY_INTERFACES = 0x4a,
    MCC_ADDL_STATUS_INSUFFICIENT_VLANS = 0xab,
    MCC_ADDL_STATUS_INVALID_SIGNATURE = 0x56,
    MCC_ADDL_STATUS_MISSING_SIGNATURE = 0x57,
    MCC_ADDL_STATUS_INSUFFICIENT_PRIVILEGES = 0x60
}

pub const CQE_BASE_STATUS_MASK: c_uint = 0xFFFF;

pub const CQE_ADDL_STATUS_MASK: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mcc_compl {
    pub /: *mut *mut u32 status; / dword 0,
    pub /: *mut *mut u32 tag0; / dword 1,
    pub /: *mut *mut u32 tag1; / dword 2,
    pub /: *mut *mut u32 flags; / dword 3,
}

// When the async bit of mcc_compl flags is set, flags
// is interpreted as follows:
//

pub const ASYNC_EVENT_CODE_MASK: c_uint = 0xFF;
pub const ASYNC_EVENT_TYPE_SHIFT: c_int = 16;
pub const ASYNC_EVENT_TYPE_MASK: c_uint = 0xFF;
pub const ASYNC_EVENT_CODE_LINK_STATE: c_uint = 0x1;
pub const ASYNC_EVENT_CODE_GRP_5: c_uint = 0x5;
pub const ASYNC_EVENT_QOS_SPEED: c_uint = 0x1;
pub const ASYNC_EVENT_COS_PRIORITY: c_uint = 0x2;
pub const ASYNC_EVENT_PVID_STATE: c_uint = 0x3;
pub const ASYNC_EVENT_CODE_QNQ: c_uint = 0x6;
pub const ASYNC_DEBUG_EVENT_TYPE_QNQ: c_int = 1;
pub const ASYNC_EVENT_CODE_SLIPORT: c_uint = 0x11;
pub const ASYNC_EVENT_PORT_MISCONFIG: c_uint = 0x9;
pub const ASYNC_EVENT_FW_CONTROL: c_uint = 0x5;
pub const LINK_STATUS_MASK: c_uint = 0x1;
pub const LOGICAL_LINK_STATUS_MASK: c_uint = 0x2;
// When the event code of compl->flags is link-state, the mcc_compl
// must be interpreted as follows
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_event_link_state {
    pub physical_port: u8,
    pub port_link_status: u8,
    pub port_duplex: u8,
    pub port_speed: u8,
    pub port_fault: u8,
    pub rsvd0: [u8; 7],
    pub flags: u32,
    pub __packed: },
// When the event code of compl->flags is GRP-5 and event_type is QOS_SPEED
// the mcc_compl must be interpreted as follows
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_event_grp5_qos_link_speed {
    pub physical_port: u8,
    pub rsvd: [u8; 5],
    pub qos_link_speed: u16,
    pub event_tag: u32,
    pub flags: u32,
    pub __packed: },
// When the event code of compl->flags is GRP5 and event type is
// CoS-Priority, the mcc_compl must be interpreted as follows
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_event_grp5_cos_priority {
    pub physical_port: u8,
    pub available_priority_bmap: u8,
    pub reco_default_priority: u8,
    pub valid: u8,
    pub rsvd0: u8,
    pub event_tag: u8,
    pub flags: u32,
    pub __packed: },
// When the event code of compl->flags is GRP5 and event type is
// PVID state, the mcc_compl must be interpreted as follows
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_event_grp5_pvid_state {
    pub enabled: u8,
    pub rsvd0: u8,
    pub tag: u16,
    pub event_tag: u32,
    pub rsvd1: u32,
    pub flags: u32,
    pub __packed: },
// async event indicating outer VLAN tag in QnQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_event_qnq {
    pub /: *mut *mut u8 valid; / Indicates if outer VLAN is valid,
    pub rsvd0: u8,
    pub vlan_tag: u16,
    pub event_tag: u32,
    pub rsvd1: [u8; 4],
    pub flags: u32,
    pub __packed: },
}

pub const PHY_STATE_MSG_SEVERITY: c_uint = 0x6;
pub const PHY_STATE_OPER: c_uint = 0x1;
pub const PHY_STATE_INFO_VALID: c_uint = 0x80;
pub const PHY_STATE_OPER_MSG_NONE: c_uint = 0x2;
pub const DEFAULT_MSG_SEVERITY: c_uint = 0x1;

// async event indicating misconfigured port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_event_misconfig_port {
// DATA_WORD1:
// phy state of port 0: bits 7 - 0
// phy state of port 1: bits 15 - 8
// phy state of port 2: bits 23 - 16
// phy state of port 3: bits 31 - 24
//
    pub event_data_word1: u32,
// DATA_WORD2:
// phy state info of port 0: bits 7 - 0
// phy state info of port 1: bits 15 - 8
// phy state info of port 2: bits 23 - 16
// phy state info of port 3: bits 31 - 24
//
// PHY STATE INFO:
// Link operability	 :bit 0
// Message severity	 :bit 2 - 1
// Rsvd			 :bits 6 - 3
// phy state info valid	 :bit 7
//
    pub event_data_word2: u32,
    pub rsvd0: u32,
    pub flags: u32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_fw_control {
    pub event_data_word1: u32,
    pub event_data_word2: u32,
    pub evt_tag: u32,
    pub event_data_word4: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mcc_mailbox {
    pub wrb: be_mcc_wrb,
    pub compl: be_mcc_compl,
}

pub const CMD_SUBSYSTEM_COMMON: c_uint = 0x1;
pub const CMD_SUBSYSTEM_ETH: c_uint = 0x3;
pub const CMD_SUBSYSTEM_LOWLEVEL: c_uint = 0xb;
pub const OPCODE_COMMON_NTWK_MAC_QUERY: c_int = 1;
pub const OPCODE_COMMON_NTWK_MAC_SET: c_int = 2;
pub const OPCODE_COMMON_NTWK_MULTICAST_SET: c_int = 3;
pub const OPCODE_COMMON_NTWK_VLAN_CONFIG: c_int = 4;
pub const OPCODE_COMMON_NTWK_LINK_STATUS_QUERY: c_int = 5;
pub const OPCODE_COMMON_READ_FLASHROM: c_int = 6;
pub const OPCODE_COMMON_WRITE_FLASHROM: c_int = 7;
pub const OPCODE_COMMON_CQ_CREATE: c_int = 12;
pub const OPCODE_COMMON_EQ_CREATE: c_int = 13;
pub const OPCODE_COMMON_MCC_CREATE: c_int = 21;
pub const OPCODE_COMMON_SET_QOS: c_int = 28;
pub const OPCODE_COMMON_MCC_CREATE_EXT: c_int = 90;
pub const OPCODE_COMMON_SEEPROM_READ: c_int = 30;
pub const OPCODE_COMMON_GET_CNTL_ATTRIBUTES: c_int = 32;
pub const OPCODE_COMMON_NTWK_RX_FILTER: c_int = 34;
pub const OPCODE_COMMON_GET_FW_VERSION: c_int = 35;
pub const OPCODE_COMMON_SET_FLOW_CONTROL: c_int = 36;
pub const OPCODE_COMMON_GET_FLOW_CONTROL: c_int = 37;
pub const OPCODE_COMMON_SET_FRAME_SIZE: c_int = 39;
pub const OPCODE_COMMON_MODIFY_EQ_DELAY: c_int = 41;
pub const OPCODE_COMMON_FIRMWARE_CONFIG: c_int = 42;
pub const OPCODE_COMMON_NTWK_INTERFACE_CREATE: c_int = 50;
pub const OPCODE_COMMON_NTWK_INTERFACE_DESTROY: c_int = 51;
pub const OPCODE_COMMON_MCC_DESTROY: c_int = 53;
pub const OPCODE_COMMON_CQ_DESTROY: c_int = 54;
pub const OPCODE_COMMON_EQ_DESTROY: c_int = 55;
pub const OPCODE_COMMON_QUERY_FIRMWARE_CONFIG: c_int = 58;
pub const OPCODE_COMMON_NTWK_PMAC_ADD: c_int = 59;
pub const OPCODE_COMMON_NTWK_PMAC_DEL: c_int = 60;
pub const OPCODE_COMMON_FUNCTION_RESET: c_int = 61;
pub const OPCODE_COMMON_MANAGE_FAT: c_int = 68;
pub const OPCODE_COMMON_ENABLE_DISABLE_BEACON: c_int = 69;
pub const OPCODE_COMMON_GET_BEACON_STATE: c_int = 70;
pub const OPCODE_COMMON_READ_TRANSRECV_DATA: c_int = 73;
pub const OPCODE_COMMON_GET_PORT_NAME: c_int = 77;
pub const OPCODE_COMMON_SET_LOGICAL_LINK_CONFIG: c_int = 80;
pub const OPCODE_COMMON_SET_INTERRUPT_ENABLE: c_int = 89;
pub const OPCODE_COMMON_SET_FN_PRIVILEGES: c_int = 100;
pub const OPCODE_COMMON_GET_PHY_DETAILS: c_int = 102;
pub const OPCODE_COMMON_SET_DRIVER_FUNCTION_CAP: c_int = 103;
pub const OPCODE_COMMON_GET_CNTL_ADDITIONAL_ATTRIBUTES: c_int = 121;
pub const OPCODE_COMMON_GET_EXT_FAT_CAPABILITIES: c_int = 125;
pub const OPCODE_COMMON_SET_EXT_FAT_CAPABILITIES: c_int = 126;
pub const OPCODE_COMMON_GET_MAC_LIST: c_int = 147;
pub const OPCODE_COMMON_SET_MAC_LIST: c_int = 148;
pub const OPCODE_COMMON_GET_HSW_CONFIG: c_int = 152;
pub const OPCODE_COMMON_GET_FUNC_CONFIG: c_int = 160;
pub const OPCODE_COMMON_GET_PROFILE_CONFIG: c_int = 164;
pub const OPCODE_COMMON_SET_PROFILE_CONFIG: c_int = 165;
pub const OPCODE_COMMON_GET_ACTIVE_PROFILE: c_int = 167;
pub const OPCODE_COMMON_SET_HSW_CONFIG: c_int = 153;
pub const OPCODE_COMMON_GET_FN_PRIVILEGES: c_int = 170;
pub const OPCODE_COMMON_READ_OBJECT: c_int = 171;
pub const OPCODE_COMMON_WRITE_OBJECT: c_int = 172;
pub const OPCODE_COMMON_DELETE_OBJECT: c_int = 174;
pub const OPCODE_COMMON_SET_FEATURES: c_int = 191;
pub const OPCODE_COMMON_MANAGE_IFACE_FILTERS: c_int = 193;
pub const OPCODE_COMMON_GET_IFACE_LIST: c_int = 194;
pub const OPCODE_COMMON_ENABLE_DISABLE_VF: c_int = 196;
pub const OPCODE_ETH_RSS_CONFIG: c_int = 1;
pub const OPCODE_ETH_ACPI_CONFIG: c_int = 2;
pub const OPCODE_ETH_PROMISCUOUS: c_int = 3;
pub const OPCODE_ETH_GET_STATISTICS: c_int = 4;
pub const OPCODE_ETH_TX_CREATE: c_int = 7;
pub const OPCODE_ETH_RX_CREATE: c_int = 8;
pub const OPCODE_ETH_TX_DESTROY: c_int = 9;
pub const OPCODE_ETH_RX_DESTROY: c_int = 10;
pub const OPCODE_ETH_ACPI_WOL_MAGIC_CONFIG: c_int = 12;
pub const OPCODE_ETH_GET_PPORT_STATS: c_int = 18;
pub const OPCODE_LOWLEVEL_HOST_DDR_DMA: c_int = 17;
pub const OPCODE_LOWLEVEL_LOOPBACK_TEST: c_int = 18;
pub const OPCODE_LOWLEVEL_SET_LOOPBACK_MODE: c_int = 19;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_hdr {
    pub /: *mut *mut u8 opcode; / dword 0,
    pub /: *mut *mut u8 subsystem; / dword 0,
    pub /: *mut *mut u8 port_number; / dword 0,
    pub /: *mut *mut u8 domain; / dword 0,
    pub /: *mut *mut u32 timeout; / dword 1,
    pub /: *mut *mut u32 request_length; / dword 2,
    pub /: *mut *mut u8 version; / dword 3,
    pub /: *mut *mut u8 rsvd[3]; / dword 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_hdr {
    pub /: *mut *mut u8 opcode; / dword 0,
    pub /: *mut *mut u8 subsystem; / dword 0,
    pub /: *mut *mut u8 rsvd[2]; / dword 0,
    pub /: *mut *mut u8 base_status; / dword 1,
    pub /: *mut *mut u8 addl_status; / dword 1,
    pub /: *mut *mut u8 rsvd1[2]; / dword 1,
    pub /: *mut *mut u32 response_length; / dword 2,
    pub /: *mut *mut u32 actual_resp_len; / dword 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phys_addr {
    pub lo: u32,
    pub hi: u32,
}

//
// BE Command definitions
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_eq_context {
    pub 0*/: *mut *mut u8 cidx[13]; / dword,
    pub 0*/: *mut *mut u8 rsvd0[3]; / dword,
    pub 0*/: *mut *mut u8 epidx[13]; / dword,
    pub 0*/: *mut *mut u8 valid; / dword,
    pub 0*/: *mut *mut u8 rsvd1; / dword,
    pub 0*/: *mut *mut u8 size; / dword,
    pub 1*/: *mut *mut u8 pidx[13]; / dword,
    pub 1*/: *mut *mut u8 rsvd2[3]; / dword,
    pub 1*/: *mut *mut u8 pd[10]; / dword,
    pub 1*/: *mut *mut u8 count[3]; / dword,
    pub 1*/: *mut *mut u8 solevent; / dword,
    pub 1*/: *mut *mut u8 stalled; / dword,
    pub 1*/: *mut *mut u8 armed; / dword,
    pub 2*/: *mut *mut u8 rsvd3[4]; / dword,
    pub 2*/: *mut *mut u8 func[8]; / dword,
    pub 2*/: *mut *mut u8 rsvd4; / dword,
    pub 2*/: *mut *mut u8 delaymult[10]; / dword,
    pub 2*/: *mut *mut u8 rsvd5[2]; / dword,
    pub 2*/: *mut *mut u8 phase[2]; / dword,
    pub 2*/: *mut *mut u8 nodelay; / dword,
    pub 2*/: *mut *mut u8 rsvd6[4]; / dword,
    pub 3*/: *mut *mut u8 rsvd7[32]; / dword,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_eq_create {
    pub hdr: be_cmd_req_hdr,
    pub /: *mut *mut u16 num_pages; / sword,
    pub /: *mut *mut u16 rsvd0; / sword,
    pub 8]: u8 context[sizeof(struct amap_eq_context) /,
    pub pages: [phys_addr; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_eq_create {
    pub resp_hdr: be_cmd_resp_hdr,
    pub /: *mut *mut u16 eq_id; / sword,
    pub /: *mut *mut u16 msix_idx; / available only in v2,
    pub __packed: },
// Mac query
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_addr {
    pub size_of_struct: u16,
    pub addr: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_mac_query {
    pub hdr: be_cmd_req_hdr,
    pub type: u8,
    pub permanent: u8,
    pub if_id: u16,
    pub pmac_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_mac_query {
    pub hdr: be_cmd_resp_hdr,
    pub mac: mac_addr,
}

// PMac Add
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_pmac_add {
    pub hdr: be_cmd_req_hdr,
    pub if_id: u32,
    pub mac_address: [u8; ETH_ALEN],
    pub rsvd0: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_pmac_add {
    pub hdr: be_cmd_resp_hdr,
    pub pmac_id: u32,
}

// PMac Del
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_pmac_del {
    pub hdr: be_cmd_req_hdr,
    pub if_id: u32,
    pub pmac_id: u32,
}

// Create CQ
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_cq_context_be {
    pub 0*/: *mut *mut u8 cidx[11]; / dword,
    pub 0*/: *mut *mut u8 rsvd0; / dword,
    pub 0*/: *mut *mut u8 coalescwm[2]; / dword,
    pub 0*/: *mut *mut u8 nodelay; / dword,
    pub 0*/: *mut *mut u8 epidx[11]; / dword,
    pub 0*/: *mut *mut u8 rsvd1; / dword,
    pub 0*/: *mut *mut u8 count[2]; / dword,
    pub 0*/: *mut *mut u8 valid; / dword,
    pub 0*/: *mut *mut u8 solevent; / dword,
    pub 0*/: *mut *mut u8 eventable; / dword,
    pub 1*/: *mut *mut u8 pidx[11]; / dword,
    pub 1*/: *mut *mut u8 rsvd2; / dword,
    pub 1*/: *mut *mut u8 pd[10]; / dword,
    pub 1*/: *mut *mut u8 eqid[8]; / dword,
    pub 1*/: *mut *mut u8 stalled; / dword,
    pub 1*/: *mut *mut u8 armed; / dword,
    pub 2*/: *mut *mut u8 rsvd3[4]; / dword,
    pub 2*/: *mut *mut u8 func[8]; / dword,
    pub 2*/: *mut *mut u8 rsvd4[20]; / dword,
    pub 3*/: *mut *mut u8 rsvd5[32]; / dword,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_cq_context_v2 {
    pub 0*/: *mut *mut u8 rsvd0[12]; / dword,
    pub 0*/: *mut *mut u8 coalescwm[2]; / dword,
    pub 0*/: *mut *mut u8 nodelay; / dword,
    pub 0*/: *mut *mut u8 rsvd1[12]; / dword,
    pub 0*/: *mut *mut u8 count[2]; / dword,
    pub 0*/: *mut *mut u8 valid; / dword,
    pub 0*/: *mut *mut u8 rsvd2; / dword,
    pub 0*/: *mut *mut u8 eventable; / dword,
    pub 1*/: *mut *mut u8 eqid[16]; / dword,
    pub 1*/: *mut *mut u8 rsvd3[15]; / dword,
    pub 1*/: *mut *mut u8 armed; / dword,
    pub 2*/: *mut *mut u8 rsvd4[32]; / dword,
    pub 3*/: *mut *mut u8 rsvd5[32]; / dword,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_cq_create {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u16,
    pub page_size: u8,
    pub rsvd0: u8,
    pub 8]: u8 context[sizeof(struct amap_cq_context_be) /,
    pub pages: [phys_addr; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_cq_create {
    pub hdr: be_cmd_resp_hdr,
    pub cq_id: u16,
    pub rsvd0: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_fat {
    pub hdr: be_cmd_req_hdr,
    pub fat_operation: u32,
    pub read_log_offset: u32,
    pub read_log_length: u32,
    pub data_buffer_size: u32,
    pub data_buffer: [u32; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_fat {
    pub hdr: be_cmd_resp_hdr,
    pub log_size: u32,
    pub read_log_length: u32,
    pub rsvd: [u32; 2],
    pub data_buffer: [u32; 1],
    pub __packed: },
// Create MCCQ
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_mcc_context_be {
    pub con_index: [u8; 14],
    pub rsvd0: [u8; 2],
    pub ring_size: [u8; 4],
    pub fetch_wrb: u8,
    pub fetch_r2t: u8,
    pub cq_id: [u8; 10],
    pub prod_index: [u8; 14],
    pub fid: [u8; 8],
    pub pdid: [u8; 9],
    pub valid: u8,
    pub rsvd1: [u8; 32],
    pub rsvd2: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_mcc_context_v1 {
    pub async_cq_id: [u8; 16],
    pub ring_size: [u8; 4],
    pub rsvd0: [u8; 12],
    pub rsvd1: [u8; 31],
    pub valid: u8,
    pub async_cq_valid: [u8; 1],
    pub rsvd2: [u8; 31],
    pub rsvd3: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_mcc_create {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u16,
    pub cq_id: u16,
    pub 8]: u8 context[sizeof(struct amap_mcc_context_be) /,
    pub pages: [phys_addr; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_mcc_ext_create {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u16,
    pub cq_id: u16,
    pub async_event_bitmap: [u32; 1],
    pub 8]: u8 context[sizeof(struct amap_mcc_context_v1) /,
    pub pages: [phys_addr; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_mcc_create {
    pub hdr: be_cmd_resp_hdr,
    pub id: u16,
    pub rsvd0: u16,
    pub __packed: },
// Create TxQ
pub const BE_ETH_TX_RING_TYPE_STANDARD: c_int = 2;
pub const BE_ULP1_NUM: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_eth_tx_create {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u8,
    pub ulp_num: u8,
    pub type: u16,
    pub if_id: u16,
    pub queue_size: u8,
    pub rsvd0: u8,
    pub rsvd1: u32,
    pub cq_id: u16,
    pub rsvd2: u16,
    pub rsvd3: [u32; 13],
    pub pages: [phys_addr; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_eth_tx_create {
    pub hdr: be_cmd_resp_hdr,
    pub cid: u16,
    pub rid: u16,
    pub db_offset: u32,
    pub rsvd0: [u32; 4],
    pub __packed: },
// Create RxQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_eth_rx_create {
    pub hdr: be_cmd_req_hdr,
    pub cq_id: u16,
    pub frag_size: u8,
    pub num_pages: u8,
    pub pages: [phys_addr; 2],
    pub interface_id: u32,
    pub max_frame_size: u16,
    pub rsvd0: u16,
    pub rss_queue: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_eth_rx_create {
    pub hdr: be_cmd_resp_hdr,
    pub id: u16,
    pub rss_id: u8,
    pub rsvd0: u8,
    pub __packed: },
// Q Destroy
// Type of Queue to be destroyed
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_q_destroy {
    pub hdr: be_cmd_req_hdr,
    pub id: u16,
    pub /: *mut *mut u16 bypass_flush; / valid only for rx q destroy,
    pub __packed: },
// I/f Create (it's actually I/f Config Create)
// Capability flags for the i/f
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum be_if_flags {
    BE_IF_FLAGS_RSS = 0x4,
    BE_IF_FLAGS_PROMISCUOUS = 0x8,
    BE_IF_FLAGS_BROADCAST = 0x10,
    BE_IF_FLAGS_UNTAGGED = 0x20,
    BE_IF_FLAGS_ULP = 0x40,
    BE_IF_FLAGS_VLAN_PROMISCUOUS = 0x80,
    BE_IF_FLAGS_VLAN = 0x100,
    BE_IF_FLAGS_MCAST_PROMISCUOUS = 0x200,
    BE_IF_FLAGS_PASS_L2_ERRORS = 0x400,
    BE_IF_FLAGS_PASS_L3L4_ERRORS = 0x800,
    BE_IF_FLAGS_MULTICAST = 0x1000,
    BE_IF_FLAGS_DEFQ_RSS = 0x1000000
}

// An RX interface is an object with one or more MAC addresses and
// filtering capabilities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_if_create {
    pub hdr: be_cmd_req_hdr,
    pub /: *mut *mut u32 version; / ignore currently,
    pub capability_flags: u32,
    pub enable_flags: u32,
    pub mac_addr: [u8; ETH_ALEN],
    pub rsvd0: u8,
    pub /: *mut *mut u8 pmac_invalid; / if set, don't attach the mac addr to the i/f,
    pub /: *mut *mut u32 vlan_tag; / not used currently,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_if_create {
    pub hdr: be_cmd_resp_hdr,
    pub interface_id: u32,
    pub pmac_id: u32,
}

// I/f Destroy(it's actually I/f Config Destroy )
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_if_destroy {
    pub hdr: be_cmd_req_hdr,
    pub interface_id: u32,
}

// HW Stats Get
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_port_rxf_stats_v0 {
    pub 0*/: *mut *mut u32 rx_bytes_lsd; / dword,
    pub 1*/: *mut *mut u32 rx_bytes_msd; / dword,
    pub 2*/: *mut *mut u32 rx_total_frames; / dword,
    pub 3*/: *mut *mut u32 rx_unicast_frames; / dword,
    pub 4*/: *mut *mut u32 rx_multicast_frames; / dword,
    pub 5*/: *mut *mut u32 rx_broadcast_frames; / dword,
    pub 6*/: *mut *mut u32 rx_crc_errors; / dword,
    pub 7*/: *mut *mut u32 rx_alignment_symbol_errors; / dword,
    pub 8*/: *mut *mut u32 rx_pause_frames; / dword,
    pub 9*/: *mut *mut u32 rx_control_frames; / dword,
    pub 10*/: *mut *mut u32 rx_in_range_errors; / dword,
    pub 11*/: *mut *mut u32 rx_out_range_errors; / dword,
    pub 12*/: *mut *mut u32 rx_frame_too_long; / dword,
    pub 13*/: *mut *mut u32 rx_address_filtered; / dword,
    pub 14*/: *mut *mut u32 rx_vlan_filtered; / dword,
    pub 15*/: *mut *mut u32 rx_dropped_too_small; / dword,
    pub 16*/: *mut *mut u32 rx_dropped_too_short; / dword,
    pub 17*/: *mut *mut u32 rx_dropped_header_too_small; / dword,
    pub 18*/: *mut *mut u32 rx_dropped_tcp_length; / dword,
    pub 19*/: *mut *mut u32 rx_dropped_runt; / dword,
    pub 20*/: *mut *mut u32 rx_64_byte_packets; / dword,
    pub 21*/: *mut *mut u32 rx_65_127_byte_packets; / dword,
    pub 22*/: *mut *mut u32 rx_128_256_byte_packets; / dword,
    pub 23*/: *mut *mut u32 rx_256_511_byte_packets; / dword,
    pub 24*/: *mut *mut u32 rx_512_1023_byte_packets; / dword,
    pub 25*/: *mut *mut u32 rx_1024_1518_byte_packets; / dword,
    pub 26*/: *mut *mut u32 rx_1519_2047_byte_packets; / dword,
    pub 27*/: *mut *mut u32 rx_2048_4095_byte_packets; / dword,
    pub 28*/: *mut *mut u32 rx_4096_8191_byte_packets; / dword,
    pub 29*/: *mut *mut u32 rx_8192_9216_byte_packets; / dword,
    pub 30*/: *mut *mut u32 rx_ip_checksum_errs; / dword,
    pub 31*/: *mut *mut u32 rx_tcp_checksum_errs; / dword,
    pub 32*/: *mut *mut u32 rx_udp_checksum_errs; / dword,
    pub 33*/: *mut *mut u32 rx_non_rss_packets; / dword,
    pub 34*/: *mut *mut u32 rx_ipv4_packets; / dword,
    pub 35*/: *mut *mut u32 rx_ipv6_packets; / dword,
    pub 36*/: *mut *mut u32 rx_ipv4_bytes_lsd; / dword,
    pub 37*/: *mut *mut u32 rx_ipv4_bytes_msd; / dword,
    pub 38*/: *mut *mut u32 rx_ipv6_bytes_lsd; / dword,
    pub 39*/: *mut *mut u32 rx_ipv6_bytes_msd; / dword,
    pub 40*/: *mut *mut u32 rx_chute1_packets; / dword,
    pub 41*/: *mut *mut u32 rx_chute2_packets; / dword,
    pub 42*/: *mut *mut u32 rx_chute3_packets; / dword,
    pub 43*/: *mut *mut u32 rx_management_packets; / dword,
    pub 44*/: *mut *mut u32 rx_switched_unicast_packets; / dword,
    pub 45*/: *mut *mut u32 rx_switched_multicast_packets; / dword,
    pub 46*/: *mut *mut u32 rx_switched_broadcast_packets; / dword,
    pub 47*/: *mut *mut u32 tx_bytes_lsd; / dword,
    pub 48*/: *mut *mut u32 tx_bytes_msd; / dword,
    pub 49*/: *mut *mut u32 tx_unicastframes; / dword,
    pub 50*/: *mut *mut u32 tx_multicastframes; / dword,
    pub 51*/: *mut *mut u32 tx_broadcastframes; / dword,
    pub 52*/: *mut *mut u32 tx_pauseframes; / dword,
    pub 53*/: *mut *mut u32 tx_controlframes; / dword,
    pub 54*/: *mut *mut u32 tx_64_byte_packets; / dword,
    pub 55*/: *mut *mut u32 tx_65_127_byte_packets; / dword,
    pub 56*/: *mut *mut u32 tx_128_256_byte_packets; / dword,
    pub 57*/: *mut *mut u32 tx_256_511_byte_packets; / dword,
    pub 58*/: *mut *mut u32 tx_512_1023_byte_packets; / dword,
    pub 59*/: *mut *mut u32 tx_1024_1518_byte_packets; / dword,
    pub 60*/: *mut *mut u32 tx_1519_2047_byte_packets; / dword,
    pub 61*/: *mut *mut u32 tx_2048_4095_byte_packets; / dword,
    pub 62*/: *mut *mut u32 tx_4096_8191_byte_packets; / dword,
    pub 63*/: *mut *mut u32 tx_8192_9216_byte_packets; / dword,
    pub 64*/: *mut *mut u32 rx_fifo_overflow; / dword,
    pub 65*/: *mut *mut u32 rx_input_fifo_overflow; / dword,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_rxf_stats_v0 {
    pub port: [be_port_rxf_stats_v0; 2],
    pub 132*/: *mut *mut u32 rx_drops_no_pbuf; / dword,
    pub 133*/: *mut *mut u32 rx_drops_no_txpb; / dword,
    pub 134*/: *mut *mut u32 rx_drops_no_erx_descr; / dword,
    pub 135*/: *mut *mut u32 rx_drops_no_tpre_descr; / dword,
    pub 136*/: *mut *mut u32 management_rx_port_packets; / dword,
    pub 137*/: *mut *mut u32 management_rx_port_bytes; / dword,
    pub 138*/: *mut *mut u32 management_rx_port_pause_frames; / dword,
    pub 139*/: *mut *mut u32 management_rx_port_errors; / dword,
    pub 140*/: *mut *mut u32 management_tx_port_packets; / dword,
    pub 141*/: *mut *mut u32 management_tx_port_bytes; / dword,
    pub 142*/: *mut *mut u32 management_tx_port_pause; / dword,
    pub 143*/: *mut *mut u32 management_rx_port_rxfifo_overflow; / dword,
    pub 144*/: *mut *mut u32 rx_drops_too_many_frags; / dword,
    pub 145*/: *mut *mut u32 rx_drops_invalid_ring; / dword,
    pub 146*/: *mut *mut u32 forwarded_packets; / dword,
    pub 147*/: *mut *mut u32 rx_drops_mtu; / dword,
    pub rsvd0: [u32; 7],
    pub port0_jabber_events: u32,
    pub port1_jabber_events: u32,
    pub rsvd1: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_erx_stats_v0 {
    pub 43*/: *mut *mut u32 rx_drops_no_fragments[44]; / dwordS 0 to,
    pub rsvd: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_pmem_stats {
    pub eth_red_drops: u32,
    pub rsvd: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_hw_stats_v0 {
    pub rxf: be_rxf_stats_v0,
    pub rsvd: [u32; 48],
    pub erx: be_erx_stats_v0,
    pub pmem: be_pmem_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_stats_v0 {
    pub hdr: be_cmd_req_hdr,
    pub be_hw_stats_v0)]: u8 rsvd[sizeof(struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_stats_v0 {
    pub hdr: be_cmd_resp_hdr,
    pub hw_stats: be_hw_stats_v0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lancer_pport_stats {
    pub tx_packets_lo: u32,
    pub tx_packets_hi: u32,
    pub tx_unicast_packets_lo: u32,
    pub tx_unicast_packets_hi: u32,
    pub tx_multicast_packets_lo: u32,
    pub tx_multicast_packets_hi: u32,
    pub tx_broadcast_packets_lo: u32,
    pub tx_broadcast_packets_hi: u32,
    pub tx_bytes_lo: u32,
    pub tx_bytes_hi: u32,
    pub tx_unicast_bytes_lo: u32,
    pub tx_unicast_bytes_hi: u32,
    pub tx_multicast_bytes_lo: u32,
    pub tx_multicast_bytes_hi: u32,
    pub tx_broadcast_bytes_lo: u32,
    pub tx_broadcast_bytes_hi: u32,
    pub tx_discards_lo: u32,
    pub tx_discards_hi: u32,
    pub tx_errors_lo: u32,
    pub tx_errors_hi: u32,
    pub tx_pause_frames_lo: u32,
    pub tx_pause_frames_hi: u32,
    pub tx_pause_on_frames_lo: u32,
    pub tx_pause_on_frames_hi: u32,
    pub tx_pause_off_frames_lo: u32,
    pub tx_pause_off_frames_hi: u32,
    pub tx_internal_mac_errors_lo: u32,
    pub tx_internal_mac_errors_hi: u32,
    pub tx_control_frames_lo: u32,
    pub tx_control_frames_hi: u32,
    pub tx_packets_64_bytes_lo: u32,
    pub tx_packets_64_bytes_hi: u32,
    pub tx_packets_65_to_127_bytes_lo: u32,
    pub tx_packets_65_to_127_bytes_hi: u32,
    pub tx_packets_128_to_255_bytes_lo: u32,
    pub tx_packets_128_to_255_bytes_hi: u32,
    pub tx_packets_256_to_511_bytes_lo: u32,
    pub tx_packets_256_to_511_bytes_hi: u32,
    pub tx_packets_512_to_1023_bytes_lo: u32,
    pub tx_packets_512_to_1023_bytes_hi: u32,
    pub tx_packets_1024_to_1518_bytes_lo: u32,
    pub tx_packets_1024_to_1518_bytes_hi: u32,
    pub tx_packets_1519_to_2047_bytes_lo: u32,
    pub tx_packets_1519_to_2047_bytes_hi: u32,
    pub tx_packets_2048_to_4095_bytes_lo: u32,
    pub tx_packets_2048_to_4095_bytes_hi: u32,
    pub tx_packets_4096_to_8191_bytes_lo: u32,
    pub tx_packets_4096_to_8191_bytes_hi: u32,
    pub tx_packets_8192_to_9216_bytes_lo: u32,
    pub tx_packets_8192_to_9216_bytes_hi: u32,
    pub tx_lso_packets_lo: u32,
    pub tx_lso_packets_hi: u32,
    pub rx_packets_lo: u32,
    pub rx_packets_hi: u32,
    pub rx_unicast_packets_lo: u32,
    pub rx_unicast_packets_hi: u32,
    pub rx_multicast_packets_lo: u32,
    pub rx_multicast_packets_hi: u32,
    pub rx_broadcast_packets_lo: u32,
    pub rx_broadcast_packets_hi: u32,
    pub rx_bytes_lo: u32,
    pub rx_bytes_hi: u32,
    pub rx_unicast_bytes_lo: u32,
    pub rx_unicast_bytes_hi: u32,
    pub rx_multicast_bytes_lo: u32,
    pub rx_multicast_bytes_hi: u32,
    pub rx_broadcast_bytes_lo: u32,
    pub rx_broadcast_bytes_hi: u32,
    pub rx_unknown_protos: u32,
    pub /: *mut *mut u32 rsvd_69; / Word 69 is reserved,
    pub rx_discards_lo: u32,
    pub rx_discards_hi: u32,
    pub rx_errors_lo: u32,
    pub rx_errors_hi: u32,
    pub rx_crc_errors_lo: u32,
    pub rx_crc_errors_hi: u32,
    pub rx_alignment_errors_lo: u32,
    pub rx_alignment_errors_hi: u32,
    pub rx_symbol_errors_lo: u32,
    pub rx_symbol_errors_hi: u32,
    pub rx_pause_frames_lo: u32,
    pub rx_pause_frames_hi: u32,
    pub rx_pause_on_frames_lo: u32,
    pub rx_pause_on_frames_hi: u32,
    pub rx_pause_off_frames_lo: u32,
    pub rx_pause_off_frames_hi: u32,
    pub rx_frames_too_long_lo: u32,
    pub rx_frames_too_long_hi: u32,
    pub rx_internal_mac_errors_lo: u32,
    pub rx_internal_mac_errors_hi: u32,
    pub rx_undersize_packets: u32,
    pub rx_oversize_packets: u32,
    pub rx_fragment_packets: u32,
    pub rx_jabbers: u32,
    pub rx_control_frames_lo: u32,
    pub rx_control_frames_hi: u32,
    pub rx_control_frames_unknown_opcode_lo: u32,
    pub rx_control_frames_unknown_opcode_hi: u32,
    pub rx_in_range_errors: u32,
    pub rx_out_of_range_errors: u32,
    pub rx_address_filtered: u32,
    pub rx_vlan_filtered: u32,
    pub rx_dropped_too_small: u32,
    pub rx_dropped_too_short: u32,
    pub rx_dropped_header_too_small: u32,
    pub rx_dropped_invalid_tcp_length: u32,
    pub rx_dropped_runt: u32,
    pub rx_ip_checksum_errors: u32,
    pub rx_tcp_checksum_errors: u32,
    pub rx_udp_checksum_errors: u32,
    pub rx_non_rss_packets: u32,
    pub rsvd_111: u32,
    pub rx_ipv4_packets_lo: u32,
    pub rx_ipv4_packets_hi: u32,
    pub rx_ipv6_packets_lo: u32,
    pub rx_ipv6_packets_hi: u32,
    pub rx_ipv4_bytes_lo: u32,
    pub rx_ipv4_bytes_hi: u32,
    pub rx_ipv6_bytes_lo: u32,
    pub rx_ipv6_bytes_hi: u32,
    pub rx_nic_packets_lo: u32,
    pub rx_nic_packets_hi: u32,
    pub rx_tcp_packets_lo: u32,
    pub rx_tcp_packets_hi: u32,
    pub rx_iscsi_packets_lo: u32,
    pub rx_iscsi_packets_hi: u32,
    pub rx_management_packets_lo: u32,
    pub rx_management_packets_hi: u32,
    pub rx_switched_unicast_packets_lo: u32,
    pub rx_switched_unicast_packets_hi: u32,
    pub rx_switched_multicast_packets_lo: u32,
    pub rx_switched_multicast_packets_hi: u32,
    pub rx_switched_broadcast_packets_lo: u32,
    pub rx_switched_broadcast_packets_hi: u32,
    pub num_forwards_lo: u32,
    pub num_forwards_hi: u32,
    pub rx_fifo_overflow: u32,
    pub rx_input_fifo_overflow: u32,
    pub rx_drops_too_many_frags_lo: u32,
    pub rx_drops_too_many_frags_hi: u32,
    pub rx_drops_invalid_queue: u32,
    pub rsvd_141: u32,
    pub rx_drops_mtu_lo: u32,
    pub rx_drops_mtu_hi: u32,
    pub rx_packets_64_bytes_lo: u32,
    pub rx_packets_64_bytes_hi: u32,
    pub rx_packets_65_to_127_bytes_lo: u32,
    pub rx_packets_65_to_127_bytes_hi: u32,
    pub rx_packets_128_to_255_bytes_lo: u32,
    pub rx_packets_128_to_255_bytes_hi: u32,
    pub rx_packets_256_to_511_bytes_lo: u32,
    pub rx_packets_256_to_511_bytes_hi: u32,
    pub rx_packets_512_to_1023_bytes_lo: u32,
    pub rx_packets_512_to_1023_bytes_hi: u32,
    pub rx_packets_1024_to_1518_bytes_lo: u32,
    pub rx_packets_1024_to_1518_bytes_hi: u32,
    pub rx_packets_1519_to_2047_bytes_lo: u32,
    pub rx_packets_1519_to_2047_bytes_hi: u32,
    pub rx_packets_2048_to_4095_bytes_lo: u32,
    pub rx_packets_2048_to_4095_bytes_hi: u32,
    pub rx_packets_4096_to_8191_bytes_lo: u32,
    pub rx_packets_4096_to_8191_bytes_hi: u32,
    pub rx_packets_8192_to_9216_bytes_lo: u32,
    pub rx_packets_8192_to_9216_bytes_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pport_stats_params {
    pub pport_num: u16,
    pub rsvd: u8,
    pub reset_stats: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lancer_cmd_req_pport_stats {
    pub hdr: be_cmd_req_hdr,
    pub params: pport_stats_params,
    pub lancer_pport_stats)]: u8 rsvd[sizeof(struct,
    pub cmd_params: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lancer_cmd_resp_pport_stats {
    pub hdr: be_cmd_resp_hdr,
    pub pport_stats: lancer_pport_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_cntl_addnl_attribs {
    pub hdr: be_cmd_req_hdr,
    pub rsvd: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_cntl_addnl_attribs {
    pub hdr: be_cmd_resp_hdr,
    pub ipl_file_number: u16,
    pub ipl_file_version: u8,
    pub rsvd0: u8,
    pub centigrade*/: *mut *mut u8 on_die_temperature; / in degrees,
    pub rsvd1: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_vlan_config {
    pub hdr: be_cmd_req_hdr,
    pub interface_id: u8,
    pub promiscuous: u8,
    pub untagged: u8,
    pub num_vlan: u8,
    pub normal_vlan: [u16; 64],
    pub __packed: },
// RX FILTER

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macaddr {
    pub byte: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_rx_filter {
    pub hdr: be_cmd_req_hdr,
    pub global_flags_mask: u32,
    pub global_flags: u32,
    pub if_flags_mask: u32,
    pub if_flags: u32,
    pub if_id: u32,
    pub mcast_num: u32,
    pub mcast_mac: [macaddr; BE_MAX_MC],
}

// Link Status Query
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_link_status {
    pub hdr: be_cmd_req_hdr,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_link_status {
    pub hdr: be_cmd_resp_hdr,
    pub physical_port: u8,
    pub mac_duplex: u8,
    pub mac_speed: u8,
    pub mac_fault: u8,
    pub mgmt_mac_duplex: u8,
    pub mgmt_mac_speed: u8,
    pub link_speed: u16,
    pub logical_link_status: u8,
    pub rsvd1: [u8; 3],
    pub __packed: },
// Port Identification
// Identifies the type of port attached to NIC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_port_type {
    pub hdr: be_cmd_req_hdr,
    pub page_num: __le32,
    pub port: __le32,
}

// From SFF-8436 QSFP+ spec
pub const QSFP_PLUS_CABLE_TYPE_OFFSET: c_uint = 0x83;
pub const QSFP_PLUS_CR4_CABLE: c_uint = 0x8;
pub const QSFP_PLUS_SR4_CABLE: c_uint = 0x4;
pub const QSFP_PLUS_LR4_CABLE: c_uint = 0x2;
// From SFF-8472 spec
pub const SFP_PLUS_SFF_8472_COMP: c_uint = 0x5E;
pub const SFP_PLUS_CABLE_TYPE_OFFSET: c_uint = 0x8;
pub const SFP_PLUS_COPPER_CABLE: c_uint = 0x4;
pub const SFP_VENDOR_NAME_OFFSET: c_uint = 0x14;
pub const SFP_VENDOR_PN_OFFSET: c_uint = 0x28;
pub const PAGE_DATA_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_port_type {
    pub hdr: be_cmd_resp_hdr,
    pub page_num: u32,
    pub port: u32,
    pub page_data: [u8; PAGE_DATA_LEN],
}

// Get FW Version
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_fw_version {
    pub hdr: be_cmd_req_hdr,
    pub rsvd0: [u8; FW_VER_LEN],
    pub rsvd1: [u8; FW_VER_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_fw_version {
    pub hdr: be_cmd_resp_hdr,
    pub firmware_version_string: [u8; FW_VER_LEN],
    pub fw_on_flash_version_string: [u8; FW_VER_LEN],
    pub __packed: },
// Set Flow Control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_flow_control {
    pub hdr: be_cmd_req_hdr,
    pub tx_flow_control: u16,
    pub rx_flow_control: u16,
    pub __packed: },
// Get Flow Control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_flow_control {
    pub hdr: be_cmd_req_hdr,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_flow_control {
    pub hdr: be_cmd_resp_hdr,
    pub tx_flow_control: u16,
    pub rx_flow_control: u16,
    pub __packed: },
// Modify EQ Delay
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_set_eqd {
    pub eq_id: u32,
    pub phase: u32,
    pub delay_multiplier: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_modify_eq_delay {
    pub hdr: be_cmd_req_hdr,
    pub num_eq: u32,
    pub set_eqd: [be_set_eqd; MAX_EVT_QS],
    pub __packed: },
// Get FW Config
// The HW can come up in either of the following multi-channel modes
// based on the skew/IPL.
//
pub const RDMA_ENABLED: c_uint = 0x4;
pub const QNQ_MODE: c_uint = 0x400;
pub const VNIC_MODE: c_uint = 0x20000;
pub const UMC_ENABLED: c_uint = 0x1000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_query_fw_cfg {
    pub hdr: be_cmd_req_hdr,
    pub rsvd: [u32; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_query_fw_cfg {
    pub hdr: be_cmd_resp_hdr,
    pub be_config_number: u32,
    pub asic_revision: u32,
    pub phys_port: u32,
    pub function_mode: u32,
    pub rsvd: [u32; 26],
    pub function_caps: u32,
}

// RSS Config
// RSS type		Input parameters used to compute RX hash
// RSS_ENABLE_IPV4	SRC IPv4, DST IPv4
// RSS_ENABLE_TCP_IPV4	SRC IPv4, DST IPv4, TCP SRC PORT, TCP DST PORT
// RSS_ENABLE_IPV6	SRC IPv6, DST IPv6
// RSS_ENABLE_TCP_IPV6	SRC IPv6, DST IPv6, TCP SRC PORT, TCP DST PORT
// RSS_ENABLE_UDP_IPV4	SRC IPv4, DST IPv4, UDP SRC PORT, UDP DST PORT
// RSS_ENABLE_UDP_IPV6	SRC IPv6, DST IPv6, UDP SRC PORT, UDP DST PORT
//
// When multiple RSS types are enabled, HW picks the best hash policy
// based on the type of the received packet.
//
pub const RSS_ENABLE_NONE: c_uint = 0x0;
pub const RSS_ENABLE_IPV4: c_uint = 0x1;
pub const RSS_ENABLE_TCP_IPV4: c_uint = 0x2;
pub const RSS_ENABLE_IPV6: c_uint = 0x4;
pub const RSS_ENABLE_TCP_IPV6: c_uint = 0x8;
pub const RSS_ENABLE_UDP_IPV4: c_uint = 0x10;
pub const RSS_ENABLE_UDP_IPV6: c_uint = 0x20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_rss_config {
    pub hdr: be_cmd_req_hdr,
    pub if_id: u32,
    pub enable_rss: u16,
    pub cpu_table_size_log2: u16,
    pub hash: [u32; 10],
    pub cpu_table: [u8; 128],
    pub flush: u8,
    pub rsvd0: [u8; 3],
}

// Port Beacon
pub const BEACON_STATE_ENABLED: c_uint = 0x1;
pub const BEACON_STATE_DISABLED: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_enable_disable_beacon {
    pub hdr: be_cmd_req_hdr,
    pub port_num: u8,
    pub beacon_state: u8,
    pub beacon_duration: u8,
    pub status_duration: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_beacon_state {
    pub hdr: be_cmd_req_hdr,
    pub port_num: u8,
    pub rsvd0: u8,
    pub rsvd1: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_beacon_state {
    pub resp_hdr: be_cmd_resp_hdr,
    pub beacon_state: u8,
    pub rsvd0: [u8; 3],
    pub __packed: },
// Flashrom related descriptors
pub const MAX_FLASH_COMP: c_int = 32;
// Optypes of each component in the UFI
}

// Maximum sizes of components in BE2 FW UFI
// Maximum sizes of components in BE3 FW UFI
// Offsets for components in BE2 FW UFI
// Offsets for components in BE3 FW UFI
// Component entry types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct controller_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_comp {
    pub offset: c_ulong,
    pub optype: c_int,
    pub size: c_int,
    pub img_type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_hdr {
    pub imageid: u32,
    pub imageoffset: u32,
    pub imagelength: u32,
    pub image_checksum: u32,
    pub image_version: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_file_hdr_g2 {
    pub sign: [u8; 32],
    pub cksum: u32,
    pub antidote: u32,
    pub cont_id: controller_id,
    pub file_len: u32,
    pub chunk_num: u32,
    pub total_chunks: u32,
    pub num_imgs: u32,
    pub build: [u8; 24],
}

// First letter of the build version of the image

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_file_hdr_g3 {
    pub sign: [u8; 52],
    pub ufi_version: [u8; 4],
    pub file_len: u32,
    pub cksum: u32,
    pub antidote: u32,
    pub num_imgs: u32,
    pub build: [u8; 24],
    pub asic_type_rev: u8,
    pub rsvd: [u8; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_section_hdr {
    pub format_rev: u32,
    pub cksum: u32,
    pub antidote: u32,
    pub num_images: u32,
    pub id_string: [u8; 128],
    pub rsvd: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_section_hdr_g2 {
    pub format_rev: u32,
    pub cksum: u32,
    pub antidote: u32,
    pub build_num: u32,
    pub id_string: [u8; 128],
    pub rsvd: [u32; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_section_entry {
    pub type: u32,
    pub offset: u32,
    pub pad_size: u32,
    pub image_size: u32,
    pub cksum: u32,
    pub entry_point: u32,
    pub optype: u16,
    pub rsvd0: u16,
    pub rsvd1: u32,
    pub ver_data: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_section_info {
    pub __nonstring: u8 cookie[32],
    pub fsec_hdr: flash_section_hdr,
    pub fsec_entry: [flash_section_entry; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_section_info_g2 {
    pub cookie: [u8; 32],
    pub fsec_hdr: flash_section_hdr_g2,
    pub fsec_entry: [flash_section_entry; 32],
    pub __packed: },
// Firmware Flash
pub const FLASHROM_OPER_FLASH: c_int = 1;
pub const FLASHROM_OPER_SAVE: c_int = 2;
pub const FLASHROM_OPER_REPORT: c_int = 4;
pub const FLASHROM_OPER_PHY_FLASH: c_int = 9;
pub const FLASHROM_OPER_PHY_SAVE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flashrom_params {
    pub op_code: u32,
    pub op_type: u32,
    pub data_buf_size: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_write_flashrom {
    pub hdr: be_cmd_req_hdr,
    pub params: flashrom_params,
    pub data_buf: [u8; 32768],
    pub rsvd: [u8; 4],
    pub __packed: },
// cmd to read flash crc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_read_flash_crc {
    pub hdr: be_cmd_req_hdr,
    pub params: flashrom_params,
    pub crc: [u8; 4],
    pub rsvd: [u8; 4],
    pub __packed: },
// Lancer Firmware Flash

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_lancer_write_obj_context {
    pub write_length: [u8; 24],
    pub reserved1: [u8; 7],
    pub eof: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lancer_cmd_req_write_object {
    pub hdr: be_cmd_req_hdr,
    pub 8]: u8 context[sizeof(struct amap_lancer_write_obj_context) /,
    pub write_offset: u32,
    pub object_name: [u8; 104],
    pub descriptor_count: u32,
    pub buf_len: u32,
    pub addr_low: u32,
    pub addr_high: u32,
}

pub const LANCER_NO_RESET_NEEDED: c_uint = 0x00;
pub const LANCER_FW_RESET_NEEDED: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lancer_cmd_resp_write_object {
    pub opcode: u8,
    pub subsystem: u8,
    pub rsvd1: [u8; 2],
    pub status: u8,
    pub additional_status: u8,
    pub rsvd2: [u8; 2],
    pub resp_len: u32,
    pub actual_resp_len: u32,
    pub actual_write_len: u32,
    pub change_status: u8,
    pub rsvd3: [u8; 3],
}

// Lancer Read FW info

pub const LANCER_READ_FILE_EOF_MASK: c_uint = 0x80000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lancer_cmd_req_read_object {
    pub hdr: be_cmd_req_hdr,
    pub desired_read_len: u32,
    pub read_offset: u32,
    pub object_name: [u8; 104],
    pub descriptor_count: u32,
    pub buf_len: u32,
    pub addr_low: u32,
    pub addr_high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lancer_cmd_resp_read_object {
    pub opcode: u8,
    pub subsystem: u8,
    pub rsvd1: [u8; 2],
    pub status: u8,
    pub additional_status: u8,
    pub rsvd2: [u8; 2],
    pub resp_len: u32,
    pub actual_resp_len: u32,
    pub actual_read_len: u32,
    pub eof: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lancer_cmd_req_delete_object {
    pub hdr: be_cmd_req_hdr,
    pub rsvd1: u32,
    pub rsvd2: u32,
    pub object_name: [u8; 104],
}

// WOL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_acpi_wol_magic_config {
    pub hdr: be_cmd_req_hdr,
    pub rsvd0: [u32; 145],
    pub magic_mac: [u8; 6],
    pub rsvd2: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_acpi_wol_magic_config_v1 {
    pub hdr: be_cmd_req_hdr,
    pub rsvd0: [u8; 2],
    pub query_options: u8,
    pub rsvd1: [u8; 5],
    pub rsvd2: [u32; 288],
    pub magic_mac: [u8; 6],
    pub rsvd3: [u8; 22],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_acpi_wol_magic_config_v1 {
    pub hdr: be_cmd_resp_hdr,
    pub rsvd0: [u8; 2],
    pub wol_settings: u8,
    pub rsvd1: [u8; 5],
    pub rsvd2: [u32; 288],
    pub magic_mac: [u8; 6],
    pub rsvd3: [u8; 22],
    pub __packed: },
pub const BE_GET_WOL_CAP: c_int = 2;
pub const BE_WOL_CAP: c_uint = 0x1;
pub const BE_PME_D0_CAP: c_uint = 0x8;
pub const BE_PME_D1_CAP: c_uint = 0x10;
pub const BE_PME_D2_CAP: c_uint = 0x20;
pub const BE_PME_D3HOT_CAP: c_uint = 0x40;
pub const BE_PME_D3COLD_CAP: c_uint = 0x80;
// LoopBack test
pub const SET_LB_MODE_TIMEOUT: c_int = 12000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_loopback_test {
    pub hdr: be_cmd_req_hdr,
    pub loopback_type: u32,
    pub num_pkts: u32,
    pub pattern: u64,
    pub src_port: u32,
    pub dest_port: u32,
    pub pkt_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_loopback_test {
    pub resp_hdr: be_cmd_resp_hdr,
    pub status: u32,
    pub num_txfer: u32,
    pub num_rx: u32,
    pub miscomp_off: u32,
    pub ticks_compl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_lmode {
    pub hdr: be_cmd_req_hdr,
    pub src_port: u8,
    pub dest_port: u8,
    pub loopback_type: u8,
    pub loopback_state: u8,
}

// DDR DMA test
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_ddrdma_test {
    pub hdr: be_cmd_req_hdr,
    pub pattern: u64,
    pub byte_count: u32,
    pub rsvd0: u32,
    pub snd_buff: [u8; 4096],
    pub rsvd1: [u8; 4096],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_ddrdma_test {
    pub hdr: be_cmd_resp_hdr,
    pub pattern: u64,
    pub byte_cnt: u32,
    pub snd_err: u32,
    pub rsvd0: [u8; 4096],
    pub rcv_buff: [u8; 4096],
}

// SEEPROM Read
pub const BE_READ_SEEPROM_LEN: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_seeprom_read {
    pub hdr: be_cmd_req_hdr,
    pub rsvd0: [u8; BE_READ_SEEPROM_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_seeprom_read {
    pub hdr: be_cmd_req_hdr,
    pub seeprom_data: [u8; BE_READ_SEEPROM_LEN],
}

pub const BE_SUPPORTED_SPEED_NONE: c_int = 0;
pub const BE_SUPPORTED_SPEED_10MBPS: c_int = 1;
pub const BE_SUPPORTED_SPEED_100MBPS: c_int = 2;
pub const BE_SUPPORTED_SPEED_1GBPS: c_int = 4;
pub const BE_SUPPORTED_SPEED_10GBPS: c_int = 8;
pub const BE_SUPPORTED_SPEED_20GBPS: c_uint = 0x10;
pub const BE_SUPPORTED_SPEED_40GBPS: c_uint = 0x20;
pub const BE_AN_EN: c_uint = 0x2;
pub const BE_PAUSE_SYM_EN: c_uint = 0x80;
// MAC speed valid values
pub const SPEED_DEFAULT: c_uint = 0x0;
pub const SPEED_FORCED_10GB: c_uint = 0x1;
pub const SPEED_FORCED_1GB: c_uint = 0x2;
pub const SPEED_AUTONEG_10GB: c_uint = 0x3;
pub const SPEED_AUTONEG_1GB: c_uint = 0x4;
pub const SPEED_AUTONEG_100MB: c_uint = 0x5;
pub const SPEED_AUTONEG_10GB_1GB: c_uint = 0x6;
pub const SPEED_AUTONEG_10GB_1GB_100MB: c_uint = 0x7;
pub const SPEED_AUTONEG_1GB_100MB: c_uint = 0x8;
pub const SPEED_AUTONEG_10MB: c_uint = 0x9;
pub const SPEED_AUTONEG_1GB_100MB_10MB: c_uint = 0xa;
pub const SPEED_AUTONEG_100MB_10MB: c_uint = 0xb;
pub const SPEED_FORCED_100MB: c_uint = 0xc;
pub const SPEED_FORCED_10MB: c_uint = 0xd;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_phy_info {
    pub hdr: be_cmd_req_hdr,
    pub rsvd0: [u8; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_phy_info {
    pub phy_type: u16,
    pub interface_type: u16,
    pub misc_params: u32,
    pub ext_phy_details: u16,
    pub rsvd: u16,
    pub auto_speeds_supported: u16,
    pub fixed_speeds_supported: u16,
    pub future_use: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_phy_info {
    pub hdr: be_cmd_req_hdr,
    pub phy_info: be_phy_info,
}

// Set QOS
pub const BE_QOS_BITS_NIC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_qos {
    pub hdr: be_cmd_req_hdr,
    pub valid_bits: u32,
    pub max_bps_nic: u32,
    pub rsvd: [u32; 7],
}

// Controller Attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_hba_attribs {
    pub rsvd0: [u32; 24],
    pub controller_model_number: [u8; 32],
    pub rsvd1: [u32; 16],
    pub controller_serial_number: [u32; 8],
    pub rsvd2: [u32; 55],
    pub rsvd3: [u8; 3],
    pub phy_port: u8,
    pub rsvd4: [u32; 15],
    pub rsvd5: [u8; 2],
    pub pci_funcnum: u8,
    pub rsvd6: u8,
    pub rsvd7: [u32; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_controller_attrib {
    pub hba_attribs: mgmt_hba_attribs,
    pub rsvd0: [u32; 10],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_cntl_attribs {
    pub hdr: be_cmd_req_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_cntl_attribs {
    pub hdr: be_cmd_resp_hdr,
    pub attribs: mgmt_controller_attrib,
}

// Set driver function
pub const CAPABILITY_SW_TIMESTAMPS: c_int = 2;
pub const CAPABILITY_BE3_NATIVE_ERX_API: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_func_cap {
    pub hdr: be_cmd_req_hdr,
    pub valid_cap_flags: u32,
    pub cap_flags: u32,
    pub rsvd: [u8; 212],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_set_func_cap {
    pub hdr: be_cmd_resp_hdr,
    pub valid_cap_flags: u32,
    pub cap_flags: u32,
    pub rsvd: [u8; 212],
}

// Function Privileges

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_priv_map {
    pub opcode: u8,
    pub subsystem: u8,
    pub priv_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_fn_privileges {
    pub hdr: be_cmd_req_hdr,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_fn_privileges {
    pub hdr: be_cmd_resp_hdr,
    pub privilege_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_fn_privileges {
    pub hdr: be_cmd_req_hdr,
    pub /: *mut *mut u32 privileges; / Used by BE3, SH-R,
    pub /: *mut *mut u32 privileges_lancer; / Used by Lancer,
}

// GET/SET_MACLIST
pub const BE_MAX_MAC: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_mac_list {
    pub hdr: be_cmd_req_hdr,
    pub mac_type: u8,
    pub perm_override: u8,
    pub iface_id: u16,
    pub mac_id: u32,
    pub rsvd: [u32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_list_macaddr {
    pub mac_addr_size: u16,
    pub macaddr: [u8; 6],
    pub rsvd: [u8; 2],
    pub mac_id: u32,
    pub s_mac_id: } __packed,
    pub mac_addr_id: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_mac_list {
    pub hdr: be_cmd_resp_hdr,
    pub /: *mut *mut get_list_macaddr fd_macaddr; / Factory default mac,
    pub /: *mut *mut get_list_macaddr macid_macaddr; / soft mac,
    pub true_mac_count: u8,
    pub pseudo_mac_count: u8,
    pub mac_list_size: u8,
    pub rsvd: u8,
// perm override mac
    pub macaddr_list: [get_list_macaddr; BE_MAX_MAC],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_mac_list {
    pub hdr: be_cmd_req_hdr,
    pub mac_count: u8,
    pub rsvd1: u8,
    pub rsvd2: u16,
    pub mac: [macaddr; BE_MAX_MAC],
    pub __packed: },
// HSW Config
pub const PORT_FWD_TYPE_VEPA: c_uint = 0x3;
pub const PORT_FWD_TYPE_VEB: c_uint = 0x2;
pub const PORT_FWD_TYPE_PASSTHRU: c_uint = 0x1;
pub const ENABLE_MAC_SPOOFCHK: c_uint = 0x2;
pub const DISABLE_MAC_SPOOFCHK: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_set_hsw_context {
    pub interface_id: [u8; 16],
    pub rsvd0: [u8; 8],
    pub mac_spoofchk: [u8; 2],
    pub rsvd1: [u8; 4],
    pub pvid_valid: u8,
    pub pport: u8,
    pub rsvd2: [u8; 6],
    pub port_fwd_type: [u8; 3],
    pub rsvd3: [u8; 5],
    pub vlan_spoofchk: [u8; 2],
    pub pvid: [u8; 16],
    pub rsvd4: [u8; 32],
    pub rsvd5: [u8; 32],
    pub rsvd6: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_hsw_config {
    pub hdr: be_cmd_req_hdr,
    pub 8]: u8 context[sizeof(struct amap_set_hsw_context) /,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_get_hsw_req_context {
    pub interface_id: [u8; 16],
    pub rsvd0: [u8; 14],
    pub pvid_valid: u8,
    pub pport: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_get_hsw_resp_context {
    pub rsvd0: [u8; 6],
    pub port_fwd_type: [u8; 3],
    pub rsvd1: [u8; 5],
    pub spoofchk: u8,
    pub rsvd2: u8,
    pub pvid: [u8; 16],
    pub rsvd3: [u8; 32],
    pub rsvd4: [u8; 32],
    pub rsvd5: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_hsw_config {
    pub hdr: be_cmd_req_hdr,
    pub 8]: u8 context[sizeof(struct amap_get_hsw_req_context) /,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_hsw_config {
    pub hdr: be_cmd_resp_hdr,
    pub 8]: u8 context[sizeof(struct amap_get_hsw_resp_context) /,
    pub rsvd: u32,
}

// get port names
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_port_name {
    pub hdr: be_cmd_req_hdr,
    pub rsvd0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_port_name {
    pub hdr: be_cmd_req_hdr,
    pub port_name: [u8; 4],
}

// HW Stats Get v1
pub const BE_TXP_SW_SZ: c_int = 48;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_port_rxf_stats_v1 {
    pub rsvd0: [u32; 12],
    pub rx_crc_errors: u32,
    pub rx_alignment_symbol_errors: u32,
    pub rx_pause_frames: u32,
    pub rx_priority_pause_frames: u32,
    pub rx_control_frames: u32,
    pub rx_in_range_errors: u32,
    pub rx_out_range_errors: u32,
    pub rx_frame_too_long: u32,
    pub rx_address_filtered: u32,
    pub rx_dropped_too_small: u32,
    pub rx_dropped_too_short: u32,
    pub rx_dropped_header_too_small: u32,
    pub rx_dropped_tcp_length: u32,
    pub rx_dropped_runt: u32,
    pub rsvd1: [u32; 10],
    pub rx_ip_checksum_errs: u32,
    pub rx_tcp_checksum_errs: u32,
    pub rx_udp_checksum_errs: u32,
    pub rsvd2: [u32; 7],
    pub rx_switched_unicast_packets: u32,
    pub rx_switched_multicast_packets: u32,
    pub rx_switched_broadcast_packets: u32,
    pub rsvd3: [u32; 3],
    pub tx_pauseframes: u32,
    pub tx_priority_pauseframes: u32,
    pub tx_controlframes: u32,
    pub rsvd4: [u32; 10],
    pub rxpp_fifo_overflow_drop: u32,
    pub rx_input_fifo_overflow_drop: u32,
    pub pmem_fifo_overflow_drop: u32,
    pub jabber_events: u32,
    pub rsvd5: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_rxf_stats_v1 {
    pub port: [be_port_rxf_stats_v1; 4],
    pub rsvd0: [u32; 2],
    pub rx_drops_no_pbuf: u32,
    pub rx_drops_no_txpb: u32,
    pub rx_drops_no_erx_descr: u32,
    pub rx_drops_no_tpre_descr: u32,
    pub rsvd1: [u32; 6],
    pub rx_drops_too_many_frags: u32,
    pub rx_drops_invalid_ring: u32,
    pub forwarded_packets: u32,
    pub rx_drops_mtu: u32,
    pub rsvd2: [u32; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_erx_stats_v1 {
    pub 67*/: *mut *mut u32 rx_drops_no_fragments[68]; / dwordS 0 to,
    pub rsvd: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_port_rxf_stats_v2 {
    pub rsvd0: [u32; 10],
    pub roce_bytes_received_lsd: u32,
    pub roce_bytes_received_msd: u32,
    pub rsvd1: [u32; 5],
    pub roce_frames_received: u32,
    pub rx_crc_errors: u32,
    pub rx_alignment_symbol_errors: u32,
    pub rx_pause_frames: u32,
    pub rx_priority_pause_frames: u32,
    pub rx_control_frames: u32,
    pub rx_in_range_errors: u32,
    pub rx_out_range_errors: u32,
    pub rx_frame_too_long: u32,
    pub rx_address_filtered: u32,
    pub rx_dropped_too_small: u32,
    pub rx_dropped_too_short: u32,
    pub rx_dropped_header_too_small: u32,
    pub rx_dropped_tcp_length: u32,
    pub rx_dropped_runt: u32,
    pub rsvd2: [u32; 10],
    pub rx_ip_checksum_errs: u32,
    pub rx_tcp_checksum_errs: u32,
    pub rx_udp_checksum_errs: u32,
    pub rsvd3: [u32; 7],
    pub rx_switched_unicast_packets: u32,
    pub rx_switched_multicast_packets: u32,
    pub rx_switched_broadcast_packets: u32,
    pub rsvd4: [u32; 3],
    pub tx_pauseframes: u32,
    pub tx_priority_pauseframes: u32,
    pub tx_controlframes: u32,
    pub rsvd5: [u32; 10],
    pub rxpp_fifo_overflow_drop: u32,
    pub rx_input_fifo_overflow_drop: u32,
    pub pmem_fifo_overflow_drop: u32,
    pub jabber_events: u32,
    pub rsvd6: [u32; 3],
    pub rx_drops_payload_size: u32,
    pub rx_drops_clipped_header: u32,
    pub rx_drops_crc: u32,
    pub roce_drops_payload_len: u32,
    pub roce_drops_crc: u32,
    pub rsvd7: [u32; 19],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_rxf_stats_v2 {
    pub port: [be_port_rxf_stats_v2; 4],
    pub rsvd0: [u32; 2],
    pub rx_drops_no_pbuf: u32,
    pub rx_drops_no_txpb: u32,
    pub rx_drops_no_erx_descr: u32,
    pub rx_drops_no_tpre_descr: u32,
    pub rsvd1: [u32; 6],
    pub rx_drops_too_many_frags: u32,
    pub rx_drops_invalid_ring: u32,
    pub forwarded_packets: u32,
    pub rx_drops_mtu: u32,
    pub rsvd2: [u32; 35],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_hw_stats_v1 {
    pub rxf: be_rxf_stats_v1,
    pub rsvd0: [u32; BE_TXP_SW_SZ],
    pub erx: be_erx_stats_v1,
    pub pmem: be_pmem_stats,
    pub rsvd1: [u32; 18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_stats_v1 {
    pub hdr: be_cmd_req_hdr,
    pub be_hw_stats_v1)]: u8 rsvd[sizeof(struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_stats_v1 {
    pub hdr: be_cmd_resp_hdr,
    pub hw_stats: be_hw_stats_v1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_erx_stats_v2 {
    pub 135*/: *mut *mut u32 rx_drops_no_fragments[136]; / dwordS 0 to,
    pub rsvd: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_hw_stats_v2 {
    pub rxf: be_rxf_stats_v2,
    pub rsvd0: [u32; BE_TXP_SW_SZ],
    pub erx: be_erx_stats_v2,
    pub pmem: be_pmem_stats,
    pub rsvd1: [u32; 18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_stats_v2 {
    pub hdr: be_cmd_req_hdr,
    pub be_hw_stats_v2)]: u8 rsvd[sizeof(struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_stats_v2 {
    pub hdr: be_cmd_resp_hdr,
    pub hw_stats: be_hw_stats_v2,
}

// get fat capabilities
pub const MAX_MODULES: c_int = 27;
pub const MAX_MODES: c_int = 4;
pub const MODE_UART: c_int = 0;
pub const FW_LOG_LEVEL_DEFAULT: c_int = 48;
pub const FW_LOG_LEVEL_FATAL: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_fat_mode {
    pub mode: u8,
    pub rsvd0: u8,
    pub port_mask: u16,
    pub dbg_lvl: u32,
    pub fun_mask: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_fat_modules {
    pub modules_str: [u8; 32],
    pub modules_id: u32,
    pub num_modes: u32,
    pub trace_lvl: [ext_fat_mode; MAX_MODES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_fat_conf_params {
    pub max_log_entries: u32,
    pub log_entry_size: u32,
    pub log_type: u8,
    pub max_log_funs: u8,
    pub max_log_ports: u8,
    pub rsvd0: u8,
    pub supp_modes: u32,
    pub num_modules: u32,
    pub module: [ext_fat_modules; MAX_MODULES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_ext_fat_caps {
    pub hdr: be_cmd_req_hdr,
    pub parameter_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_ext_fat_caps {
    pub hdr: be_cmd_resp_hdr,
    pub get_params: be_fat_conf_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_ext_fat_caps {
    pub hdr: be_cmd_req_hdr,
    pub set_params: be_fat_conf_params,
}

pub const RESOURCE_DESC_SIZE_V0: c_int = 72;
pub const RESOURCE_DESC_SIZE_V1: c_int = 88;
pub const PCIE_RESOURCE_DESC_TYPE_V0: c_uint = 0x40;
pub const NIC_RESOURCE_DESC_TYPE_V0: c_uint = 0x41;
pub const PCIE_RESOURCE_DESC_TYPE_V1: c_uint = 0x50;
pub const NIC_RESOURCE_DESC_TYPE_V1: c_uint = 0x51;
pub const PORT_RESOURCE_DESC_TYPE_V1: c_uint = 0x55;
pub const MAX_RESOURCE_DESC: c_int = 264;

pub const MISSION_NIC: c_int = 1;
pub const MISSION_RDMA: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_res_desc_hdr {
    pub desc_type: u8,
    pub desc_len: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_port_res_desc {
    pub hdr: be_res_desc_hdr,
    pub rsvd0: u8,
    pub flags: u8,
    pub link_num: u8,
    pub mc_type: u8,
    pub rsvd1: u16,
pub const NV_TYPE_MASK: c_uint = 0x3	/* bits 0-1 */;
pub const NV_TYPE_DISABLED: c_int = 1;
pub const NV_TYPE_VXLAN: c_int = 3;

pub const PF_NUM_IGNORE: c_int = 255;
    pub nv_flags: u8,
    pub rsvd2: u8,
    pub /: *mut *mut __le16 nv_port; / vxlan/gre port,
    pub rsvd3: [u32; 19],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_pcie_res_desc {
    pub hdr: be_res_desc_hdr,
    pub rsvd0: u8,
    pub flags: u8,
    pub rsvd1: u16,
    pub pf_num: u8,
    pub rsvd2: u8,
    pub rsvd3: u32,
    pub sriov_state: u8,
    pub pf_state: u8,
    pub pf_type: u8,
    pub rsvd4: u8,
    pub num_vfs: u16,
    pub rsvd5: u16,
    pub rsvd6: [u32; 17],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_nic_res_desc {
    pub hdr: be_res_desc_hdr,
    pub rsvd1: u8,

    pub flags: u8,
    pub vf_num: u8,
    pub rsvd2: u8,
    pub pf_num: u8,
    pub rsvd3: u8,
    pub unicast_mac_count: u16,
    pub rsvd4: [u8; 6],
    pub mcc_count: u16,
    pub vlan_count: u16,
    pub mcast_mac_count: u16,
    pub txq_count: u16,
    pub rq_count: u16,
    pub rssq_count: u16,
    pub lro_count: u16,
    pub cq_count: u16,
    pub toe_conn_count: u16,
    pub eq_count: u16,
    pub vlan_id: u16,
    pub iface_count: u16,
    pub cap_flags: u32,
    pub link_param: u8,
    pub rsvd6: u8,
    pub channel_id_param: u16,
    pub bw_min: u32,
    pub bw_max: u32,
    pub acpi_params: u8,
    pub wol_param: u8,
    pub rsvd7: u16,
    pub tunnel_iface_count: u16,
    pub direct_tenant_iface_count: u16,
    pub rsvd8: [u32; 6],
    pub __packed: },
// Multi-Channel type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mc_type {
    MC_NONE = 0x01,
    UMC = 0x02,
    FLEX10 = 0x03,
    vNIC1 = 0x04,
    nPAR = 0x05,
    UFP = 0x06,
    vNIC2 = 0x07
}

// Is BE in a multi-channel mode
    pub MC_NONE: return adapter->mc_type >,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_func_config {
    pub hdr: be_cmd_req_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_func_config {
    pub hdr: be_cmd_resp_hdr,
    pub desc_count: u32,
    pub RESOURCE_DESC_SIZE_V1]: *mut *mut u8 func_param[MAX_RESOURCE_DESC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_profile_config {
    pub hdr: be_cmd_req_hdr,
    pub rsvd: u8,
pub const ACTIVE_PROFILE_TYPE: c_uint = 0x2;
pub const SAVED_PROFILE_TYPE: c_uint = 0x0;

    pub type: u8,
    pub rsvd1: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_profile_config {
    pub hdr: be_cmd_resp_hdr,
    pub desc_count: __le16,
    pub rsvd: u16,
    pub RESOURCE_DESC_SIZE_V1]: *mut *mut u8 func_param[MAX_RESOURCE_DESC,
}

pub const FIELD_MODIFIABLE: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_profile_config {
    pub hdr: be_cmd_req_hdr,
    pub rsvd: u32,
    pub desc_count: u32,
    pub RESOURCE_DESC_SIZE_V1]: *mut *mut u8 desc[2,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_active_profile {
    pub hdr: be_cmd_req_hdr,
    pub rsvd: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_active_profile {
    pub hdr: be_cmd_resp_hdr,
    pub active_profile_id: u16,
    pub next_profile_id: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_enable_disable_vf {
    pub hdr: be_cmd_req_hdr,
    pub enable: u8,
    pub rsvd: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_intr_set {
    pub hdr: be_cmd_req_hdr,
    pub intr_enabled: u8,
    pub rsvd: [u8; 3],
}

// Get IFACE LIST
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_if_desc {
    pub if_id: u32,
    pub cap_flags: u32,
    pub en_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_get_iface_list {
    pub hdr: be_cmd_req_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_get_iface_list {
    pub hdr: be_cmd_req_hdr,
    pub if_cnt: u32,
    pub if_desc: be_if_desc,
}

// Set Features
pub const BE_FEATURE_UE_RECOVERY: c_uint = 0x10;
pub const BE_UE_RECOVERY_UER_MASK: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_req_ue_recovery {
    pub uer: u32,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_features {
    pub hdr: be_cmd_req_hdr,
    pub features: u32,
    pub parameter_len: u32,
    pub req: be_req_ue_recovery,
    pub rsvd: [u32; 2],
    pub parameter: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_resp_ue_recovery {
    pub uer: u32,
    pub ue2rp: u16,
    pub ue2sr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_set_features {
    pub hdr: be_cmd_resp_hdr,
    pub features: u32,
    pub parameter_len: u32,
    pub resp: be_resp_ue_recovery,
    pub rsvd: [u32; 2],
    pub parameter: },
}

// Set logical link

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_set_ll_link {
    pub hdr: be_cmd_req_hdr,
    pub /: *mut *mut u32 link_config; / Bit 0: UP_DOWN, Bit 9: PLINK,
}

// Manage IFACE Filters
pub const OP_CONVERT_NORMAL_TO_TUNNEL: c_int = 0;
pub const OP_CONVERT_TUNNEL_TO_NORMAL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_manage_iface_filters {
    pub hdr: be_cmd_req_hdr,
    pub op: u8,
    pub rsvd0: u8,
    pub flags: u8,
    pub rsvd1: u8,
    pub tunnel_iface_id: u32,
    pub target_iface_id: u32,
    pub mac: [u8; 6],
    pub vlan_tag: u16,
    pub tenant_id: u32,
    pub filter_id: u32,
    pub cap_flags: u32,
    pub cap_control_flags: u32,
    pub __packed: },
    pub adapter): *mut u16 be_POST_stage_get(struct be_adapter,
    pub adapter): *mut int be_fw_wait_ready(struct be_adapter,
    pub pmac_id): bool permanent, u32 if_handle, u32,
    pub domain): *mut *mut u32 pmac_id, u32,
    pub domain): u32,
    pub domain): *mut *mut u32 if_handle, u32,
    pub domain): *mut *mut int be_cmd_if_destroy(struct be_adapter adapter, int if_handle, u32,
    pub eqo): *mut *mut int be_cmd_eq_create(struct be_adapter adapter, struct be_eq_obj,
    pub num_cqe_dma_coalesce): c_int,
    pub cq): *mut be_queue_info,
    pub txo): *mut *mut int be_cmd_txq_create(struct be_adapter adapter, struct be_tx_obj,
    pub rss_id): *mut u16 cq_id, u16 frag_size, u32 if_id, u32 rss, u8,
    pub type): c_int,
    pub q): *mut *mut int be_cmd_rxq_destroy(struct be_adapter adapter, struct be_queue_info,
    pub dom): *mut *mut u8 link_status, u32,
    pub nonemb_cmd): *mut *mut int be_cmd_get_stats(struct be_adapter adapter, struct be_dma_mem,
    pub nonemb_cmd): *mut be_dma_mem,
    pub adapter): *mut int be_cmd_get_fw_ver(struct be_adapter,
    pub num): *mut *mut *mut int be_cmd_modify_eqd(struct be_adapter adapter, struct be_set_eqd , int,
    pub domain): u32 num, u32,
    pub status): *mut *mut int be_cmd_rx_filter(struct be_adapter adapter, u32 flags, u32,
    pub rx_fc): *mut *mut int be_cmd_set_flow_control(struct be_adapter adapter, u32 tx_fc, u32,
    pub rx_fc): *mut *mut *mut int be_cmd_get_flow_control(struct be_adapter adapter, u32 tx_fc, u32,
    pub adapter): *mut int be_cmd_query_fw_cfg(struct be_adapter,
    pub adapter): *mut int be_cmd_reset_function(struct be_adapter,
    pub rss_hkey): *const u32 rss_hash_opts, u16 table_size, u8,
    pub adapter): *mut int be_process_mcc(struct be_adapter,
    pub state): u8 status, u8,
    pub state): *mut u32,
    pub data): *mut u8 page_num, u32 off, u32 len, u8,
    pub adapter): *mut int be_cmd_query_cable_type(struct be_adapter,
    pub adapter): *mut int be_cmd_query_sfp_info(struct be_adapter,
    pub addn_status): *mut *mut *mut u32 data_read, u32 eof, u8,
    pub fw): *const *const int lancer_fw_download(struct be_adapter adapter, struct firmware,
    pub fw): *const *const int be_fw_download(struct be_adapter adapter, struct firmware,
    pub nonemb_cmd): *mut be_dma_mem,
    pub adapter): *mut int be_cmd_fw_init(struct be_adapter,
    pub adapter): *mut int be_cmd_fw_clean(struct be_adapter,
    pub adapter): *mut void be_async_mcc_enable(struct be_adapter,
    pub adapter): *mut void be_async_mcc_disable(struct be_adapter,
    pub pattern): u64,
    pub cmd): *mut be_dma_mem,
    pub nonemb_cmd): *mut be_dma_mem,
    pub enable): u8 loopback_type, u8,
    pub adapter): *mut int be_cmd_get_phy_info(struct be_adapter,
    pub domain): u16 link_speed, u8,
    pub adapter): *mut void be_detect_error(struct be_adapter,
    pub adapter): *mut int be_cmd_get_die_temperature(struct be_adapter,
    pub adapter): *mut int be_cmd_get_cntl_attributes(struct be_adapter,
    pub dump_size): *mut *mut int be_cmd_get_fat_dump_len(struct be_adapter adapter, u32,
    pub buf): *mut *mut int be_cmd_get_fat_dump(struct be_adapter adapter, u32 buf_len, void,
    pub adapter): *mut int be_cmd_req_native_mode(struct be_adapter,
    pub domain): u32,
    pub vf_num): u32,
    pub domain): u32 if_handle, u8,
    pub domain): u32 if_handle, bool active, u32,
    pub mac): *mut *mut int be_cmd_get_perm_mac(struct be_adapter adapter, u8,
    pub domain): u32,
    pub dom): *mut *mut *mut int be_cmd_set_mac(struct be_adapter adapter, u8 mac, int if_id, u32,
    pub spoofchk): u16 intf_id, u16 hsw_mode, u8,
    pub spoofchk): *mut *mut u16 intf_id, u8 mode, bool,
    pub adapter): *mut int be_cmd_get_acpi_wol_cap(struct be_adapter,
    pub level): *mut *mut int be_cmd_set_fw_log_level(struct be_adapter adapter, u32,
    pub adapter): *mut int be_cmd_get_fw_log_level(struct be_adapter,
    pub cmd): *mut be_dma_mem,
    pub cfgs): *mut be_fat_conf_params,
    pub mask): *mut *mut int lancer_physdev_ctrl(struct be_adapter adapter, u32,
    pub adapter): *mut int lancer_initiate_dump(struct be_adapter,
    pub adapter): *mut int lancer_delete_dump(struct be_adapter,
    pub adapter): *mut bool dump_present(struct be_adapter,
    pub adapter): *mut int be_cmd_query_port_name(struct be_adapter,
    pub res): *mut be_resources,
    pub domain): u8 profile_type, u8 query, u8,
    pub profile): *mut *mut int be_cmd_get_active_profile(struct be_adapter adapter, u16,
    pub vf_num): c_int,
    pub domain): *mut *mut int be_cmd_enable_vf(struct be_adapter adapter, u8,
    pub intr_enable): *mut *mut int be_cmd_intr_set(struct be_adapter adapter, bool,
    pub domain): int link_state, u8,
    pub port): *mut *mut int be_cmd_set_vxlan_port(struct be_adapter adapter, __be16,
    pub op): *mut *mut int be_cmd_manage_iface(struct be_adapter adapter, u32 iface, u8,
    pub vft_res): *mut be_resources,
    pub adapter): *mut int be_cmd_set_features(struct be_adapter,
