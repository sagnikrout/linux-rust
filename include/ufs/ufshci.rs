//! Automatically rewritten from C Header to Rust Module
//! Source: include/ufs/ufshci.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Universal Flash Storage Host controller driver
// Copyright (C) 2011-2013 Samsung India Software Operations
//
// Authors:
// Santosh Yaraganavi <santosh.sy@samsung.com>
// Vinayak Holikatti <h.vinayak@samsung.com>
//

// Larger response area, only for the devman UCD
// UFSHCI Registers
// Controller capability masks
// Submission Queue (SQ) Configuration Registers
// Completion Queue (CQ) Configuration Registers
// Operation and Runtime Registers - Submission Queues and Completion Queues

// UFS Version 08h

pub const UFSHCD_NUM_RESERVED: c_int = 1;
//
// Controller UFSHCI version
// - 2.x and newer use the following scheme:
// major << 8 + minor << 4
// - 1.x has been converted to match this in
// ufshcd_get_ufs_version()
//
// HCDDID - Host Controller Identification Descriptor
// - Device ID and Device Class 10h
//

//
// HCPMID - Host Controller Identification Descriptor
// - Product/Manufacturer ID  14h
//

// AHIT - Auto-Hibernate Idle Timer

pub const UFSHCI_AHIBERN8_SCALE_FACTOR: c_int = 10;

//
// IS - Interrupt Status - 20h
//
pub const UTP_TRANSFER_REQ_COMPL: c_uint = 0x1;
pub const UIC_DME_END_PT_RESET: c_uint = 0x2;
pub const UIC_ERROR: c_uint = 0x4;
pub const UIC_TEST_MODE: c_uint = 0x8;
pub const UIC_POWER_MODE: c_uint = 0x10;
pub const UIC_HIBERNATE_EXIT: c_uint = 0x20;
pub const UIC_HIBERNATE_ENTER: c_uint = 0x40;
pub const UIC_LINK_LOST: c_uint = 0x80;
pub const UIC_LINK_STARTUP: c_uint = 0x100;
pub const UTP_TASK_REQ_COMPL: c_uint = 0x200;
pub const UIC_COMMAND_COMPL: c_uint = 0x400;
pub const DEVICE_FATAL_ERROR: c_uint = 0x800;
pub const UTP_ERROR: c_uint = 0x1000;
pub const CONTROLLER_FATAL_ERROR: c_uint = 0x10000;
pub const SYSTEM_BUS_FATAL_ERROR: c_uint = 0x20000;
pub const CRYPTO_ENGINE_FATAL_ERROR: c_uint = 0x40000;
pub const MCQ_CQ_EVENT_STATUS: c_uint = 0x100000;
pub const MCQ_IAG_EVENT_STATUS: c_uint = 0x200000;

// HCS - Host Controller Status 30h
pub const DEVICE_PRESENT: c_uint = 0x1;
pub const UTP_TRANSFER_REQ_LIST_READY: c_uint = 0x2;
pub const UTP_TASK_REQ_LIST_READY: c_uint = 0x4;
pub const UIC_COMMAND_READY: c_uint = 0x8;
pub const HOST_ERROR_INDICATOR: c_uint = 0x10;
pub const DEVICE_ERROR_INDICATOR: c_uint = 0x20;

// HCE - Host Controller Enable 34h
pub const CONTROLLER_ENABLE: c_uint = 0x1;
pub const CONTROLLER_DISABLE: c_uint = 0x0;
pub const CRYPTO_GENERAL_ENABLE: c_uint = 0x2;
// UECPA - Host UIC Error Code PHY Adapter Layer 38h
pub const UIC_PHY_ADAPTER_LAYER_ERROR: c_uint = 0x80000000;
pub const UIC_PHY_ADAPTER_LAYER_ERROR_CODE_MASK: c_uint = 0x1F;
pub const UIC_PHY_ADAPTER_LAYER_LANE_ERR_MASK: c_uint = 0xF;
pub const UIC_PHY_ADAPTER_LAYER_GENERIC_ERROR: c_uint = 0x10;
// UECDL - Host UIC Error Code Data Link Layer 3Ch
pub const UIC_DATA_LINK_LAYER_ERROR: c_uint = 0x80000000;
pub const UIC_DATA_LINK_LAYER_ERROR_CODE_MASK: c_uint = 0xFFFF;
pub const UIC_DATA_LINK_LAYER_ERROR_TCX_REP_TIMER_EXP: c_uint = 0x2;
pub const UIC_DATA_LINK_LAYER_ERROR_AFCX_REQ_TIMER_EXP: c_uint = 0x4;
pub const UIC_DATA_LINK_LAYER_ERROR_FCX_PRO_TIMER_EXP: c_uint = 0x8;
pub const UIC_DATA_LINK_LAYER_ERROR_RX_BUF_OF: c_uint = 0x20;
pub const UIC_DATA_LINK_LAYER_ERROR_PA_INIT: c_uint = 0x2000;
pub const UIC_DATA_LINK_LAYER_ERROR_NAC_RECEIVED: c_uint = 0x0001;
pub const UIC_DATA_LINK_LAYER_ERROR_TCx_REPLAY_TIMEOUT: c_uint = 0x0002;
// UECN - Host UIC Error Code Network Layer 40h
pub const UIC_NETWORK_LAYER_ERROR: c_uint = 0x80000000;
pub const UIC_NETWORK_LAYER_ERROR_CODE_MASK: c_uint = 0x7;
pub const UIC_NETWORK_UNSUPPORTED_HEADER_TYPE: c_uint = 0x1;
pub const UIC_NETWORK_BAD_DEVICEID_ENC: c_uint = 0x2;
pub const UIC_NETWORK_LHDR_TRAP_PACKET_DROPPING: c_uint = 0x4;
// UECT - Host UIC Error Code Transport Layer 44h
pub const UIC_TRANSPORT_LAYER_ERROR: c_uint = 0x80000000;
pub const UIC_TRANSPORT_LAYER_ERROR_CODE_MASK: c_uint = 0x7F;
pub const UIC_TRANSPORT_UNSUPPORTED_HEADER_TYPE: c_uint = 0x1;
pub const UIC_TRANSPORT_UNKNOWN_CPORTID: c_uint = 0x2;
pub const UIC_TRANSPORT_NO_CONNECTION_RX: c_uint = 0x4;
pub const UIC_TRANSPORT_CONTROLLED_SEGMENT_DROPPING: c_uint = 0x8;
pub const UIC_TRANSPORT_BAD_TC: c_uint = 0x10;
pub const UIC_TRANSPORT_E2E_CREDIT_OVERFOW: c_uint = 0x20;
pub const UIC_TRANSPORT_SAFETY_VALUE_DROPPING: c_uint = 0x40;
// UECDME - Host UIC Error Code DME 48h
pub const UIC_DME_ERROR: c_uint = 0x80000000;
pub const UIC_DME_ERROR_CODE_MASK: c_uint = 0x1;
pub const UIC_DME_QOS_MASK: c_uint = 0xE;
// UTRIACR - Interrupt Aggregation control register - 0x4Ch
pub const INT_AGGR_TIMEOUT_VAL_MASK: c_uint = 0xFF;

pub const INT_AGGR_COUNTER_AND_TIMER_RESET: c_uint = 0x10000;
pub const INT_AGGR_STATUS_BIT: c_uint = 0x100000;
pub const INT_AGGR_PARAM_WRITE: c_uint = 0x1000000;
pub const INT_AGGR_ENABLE: c_uint = 0x80000000;
// UTRLRSR - UTP Transfer Request Run-Stop Register 60h
pub const UTP_TRANSFER_REQ_LIST_RUN_STOP_BIT: c_uint = 0x1;
// UTMRLRSR - UTP Task Management Request Run-Stop Register 80h
pub const UTP_TASK_REQ_LIST_RUN_STOP_BIT: c_uint = 0x1;
// REG_UFS_MEM_CFG - Global Config Registers 300h

// CQISy - CQ y Interrupt Status Register
pub const UFSHCD_MCQ_CQIS_TAIL_ENT_PUSH_STS: c_uint = 0x1;
// UICCMD - UIC Command
pub const COMMAND_OPCODE_MASK: c_uint = 0xFF;
pub const GEN_SELECTOR_INDEX_MASK: c_uint = 0xFFFF;

pub const RESET_LEVEL: c_uint = 0xFF;

pub const CONFIG_RESULT_CODE_MASK: c_uint = 0xFF;
pub const GENERIC_ERROR_CODE_MASK: c_uint = 0xFF;
// GenSelectorIndex calculation macros for M-PHY attributes

// Link Status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_status {
    UFSHCD_LINK_IS_DOWN	= 1,
    UFSHCD_LINK_IS_UP	= 2,
}

// UIC Commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uic_cmd_dme {
    UIC_CMD_DME_GET			= 0x01,
    UIC_CMD_DME_SET			= 0x02,
    UIC_CMD_DME_PEER_GET		= 0x03,
    UIC_CMD_DME_PEER_SET		= 0x04,
    UIC_CMD_DME_POWERON		= 0x10,
    UIC_CMD_DME_POWEROFF		= 0x11,
    UIC_CMD_DME_ENABLE		= 0x12,
    UIC_CMD_DME_RESET		= 0x14,
    UIC_CMD_DME_END_PT_RST		= 0x15,
    UIC_CMD_DME_LINK_STARTUP	= 0x16,
    UIC_CMD_DME_HIBER_ENTER		= 0x17,
    UIC_CMD_DME_HIBER_EXIT		= 0x18,
    UIC_CMD_DME_TEST_MODE		= 0x1A,
}

// UIC Config result code / Generic error code
pub const MASK_UIC_COMMAND_RESULT: c_uint = 0xFF;

// Interrupt disable masks
// Interrupt disable mask for UFSHCI v1.1
// Interrupt disable mask for UFSHCI v2.1
// CCAP - Crypto Capability 100h
#[repr(C)]
#[derive(Copy, Clone)]
pub union ufs_crypto_capabilities {
    pub reg_val: __le32,
    pub num_crypto_cap: u8,
    pub config_count: u8,
    pub reserved: u8,
    pub config_array_ptr: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_crypto_key_size {
    UFS_CRYPTO_KEY_SIZE_INVALID	= 0x0,
    UFS_CRYPTO_KEY_SIZE_128		= 0x1,
    UFS_CRYPTO_KEY_SIZE_192		= 0x2,
    UFS_CRYPTO_KEY_SIZE_256		= 0x3,
    UFS_CRYPTO_KEY_SIZE_512		= 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_crypto_alg {
    UFS_CRYPTO_ALG_AES_XTS			= 0x0,
    UFS_CRYPTO_ALG_BITLOCKER_AES_CBC	= 0x1,
    UFS_CRYPTO_ALG_AES_ECB			= 0x2,
    UFS_CRYPTO_ALG_ESSIV_AES_CBC		= 0x3,
}

// x-CRYPTOCAP - Crypto Capability X
#[repr(C)]
#[derive(Copy, Clone)]
pub union ufs_crypto_cap_entry {
    pub reg_val: __le32,
    pub algorithm_id: u8,
    pub /: *mut *mut u8 sdus_mask; / Supported data unit size mask,
    pub key_size: u8,
    pub reserved: u8,
}

pub const UFS_CRYPTO_KEY_MAX_SIZE: c_int = 64;
// x-CRYPTOCFG - Crypto Configuration X
#[repr(C)]
#[derive(Copy, Clone)]
pub union ufs_crypto_cfg_entry {
    pub reg_val: [__le32; 32],
    pub crypto_key: [u8; UFS_CRYPTO_KEY_MAX_SIZE],
    pub data_unit_size: u8,
    pub crypto_cap_idx: u8,
    pub reserved_1: u8,
    pub config_enable: u8,
    pub reserved_multi_host: u8,
    pub reserved_2: u8,
    pub vsb: [u8; 2],
    pub reserved_3: [u8; 56],
}

//
// Request Descriptor Definitions
//
// To accommodate UFS2.0 required Command type
// UTP Transfer Request Data Direction (DD)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum utp_data_direction {
    UTP_NO_DATA_TRANSFER	= 0,
    UTP_HOST_TO_DEVICE	= 1,
    UTP_DEVICE_TO_HOST	= 2,
}

// Overall command status values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum utp_ocs {
    OCS_SUCCESS			= 0x0,
    OCS_INVALID_CMD_TABLE_ATTR	= 0x1,
    OCS_INVALID_PRDT_ATTR		= 0x2,
    OCS_MISMATCH_DATA_BUF_SIZE	= 0x3,
    OCS_MISMATCH_RESP_UPIU_SIZE	= 0x4,
    OCS_PEER_COMM_FAILURE		= 0x5,
    OCS_ABORTED			= 0x6,
    OCS_FATAL_ERROR			= 0x7,
    OCS_DEVICE_FATAL_ERROR		= 0x8,
    OCS_INVALID_CRYPTO_CONFIG	= 0x9,
    OCS_GENERAL_CRYPTO_ERROR	= 0xA,
    OCS_INVALID_COMMAND_STATUS	= 0x0F,
}

// The maximum length of the data byte count field in the PRDT is 256KB

// The granularity of the data byte count field in the PRDT is 32-bit
pub const PRDT_DATA_BYTE_COUNT_PAD: c_int = 4;
//
// struct ufshcd_sg_entry - UFSHCI PRD Entry
// @addr: Physical address; DW-0 and DW-1.
// @reserved: Reserved for future use DW-2
// @size: size of physical segment DW-3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufshcd_sg_entry {
    pub addr: __le64,
    pub reserved: __le32,
    pub size: __le32,
//
// followed by variant-specific fields if
// CONFIG_SCSI_UFS_VARIABLE_SG_ENTRY_SIZE has been defined.
//
}

//
// struct utp_transfer_cmd_desc - UTP Command Descriptor (UCD)
// @command_upiu: Command UPIU Frame address
// @response_upiu: Response UPIU Frame address
// @prd_table: Physical Region Descriptor: an array of SG_ALL struct
// ufshcd_sg_entry's.  Variant-specific fields may be present after each.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utp_transfer_cmd_desc {
    pub command_upiu: [u8; ALIGNED_UPIU_SIZE],
    pub response_upiu: [u8; ALIGNED_UPIU_SIZE],
    pub prd_table: [u8; ],
}

// Dedicated UCD for the devman/reserved slot
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utp_devman_cmd_desc {
    pub command_upiu: [u8; ALIGNED_UPIU_SIZE],
    pub response_upiu: [u8; ALIGNED_DEVMAN_RSP_SIZE],
    pub prd_table: [u8; ],
}

//
// struct request_desc_header - Descriptor Header common to both UTRD and UTMRD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_desc_header {
    pub cci: u8,
    pub ehs_length: u8,

    pub enable_crypto:1: u8,
    pub reserved2:7: u8,
    pub command_type:4: u8,
    pub reserved1:1: u8,
    pub data_direction:2: u8,
    pub interrupt:1: u8,

    pub reserved2:7: u8,
    pub enable_crypto:1: u8,
    pub interrupt:1: u8,
    pub data_direction:2: u8,
    pub reserved1:1: u8,
    pub command_type:4: u8,

    pub dunl: __le32,
    pub ocs: u8,
    pub cds: u8,
    pub ldbc: __le16,
    pub dunu: __le32,
}

//
// struct utp_transfer_req_desc - UTP Transfer Request Descriptor (UTRD)
// @header: UTRD header DW-0 to DW-3
// @command_desc_base_addr: UCD base address DW 4-5
// @response_upiu_length: response UPIU length DW-6
// @response_upiu_offset: response UPIU offset DW-6
// @prd_table_length: Physical region descriptor length DW-7
// @prd_table_offset: Physical region descriptor offset DW-7
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utp_transfer_req_desc {
// DW 0-3
    pub header: request_desc_header,
// DW 4-5
    pub command_desc_base_addr: __le64,
// DW 6
    pub response_upiu_length: __le16,
    pub response_upiu_offset: __le16,
// DW 7
    pub prd_table_length: __le16,
    pub prd_table_offset: __le16,
}

// MCQ Completion Queue Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_entry {
// DW 0-1
    pub command_desc_base_addr: __le64,
// DW 2
    pub response_upiu_length: __le16,
    pub response_upiu_offset: __le16,
// DW 3
    pub prd_table_length: __le16,
    pub prd_table_offset: __le16,
// DW 4
    pub overall_status: u8,
    pub extended_error_code: u8,
    pub reserved_1: __le16,
// DW 5
    pub task_tag: u8,
    pub lun: u8,

    pub ext_iid:4: u8,
    pub iid:4: u8,

    pub iid:4: u8,
    pub ext_iid:4: u8,

    pub reserved_2: u8,
// DW 6-7
    pub reserved_3: [__le32; 2],
}

//
// UTMRD structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utp_task_req_desc {
// DW 0-3
    pub header: request_desc_header,
// DW 4-11 - Task request UPIU structure
    pub req_header: utp_upiu_header,
    pub input_param1: __be32,
    pub input_param2: __be32,
    pub input_param3: __be32,
    pub __reserved1: [__be32; 2],
    pub upiu_req: },
// DW 12-19 - Task Management Response UPIU structure
    pub rsp_header: utp_upiu_header,
    pub output_param1: __be32,
    pub output_param2: __be32,
    pub __reserved2: [__be32; 3],
    pub upiu_rsp: },
}
