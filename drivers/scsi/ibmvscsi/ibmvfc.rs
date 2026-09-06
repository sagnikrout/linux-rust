//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/ibmvscsi/ibmvfc.h
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
// ibmvfc.h -- driver for IBM Power Virtual Fibre Channel Adapter
//
// Written By: Brian King <brking@linux.vnet.ibm.com>, IBM Corporation
//
// Copyright (C) IBM Corporation, 2008-2026
//

pub const IBMVFC_DEFAULT_TIMEOUT: c_int = 60;
pub const IBMVFC_ADISC_CANCEL_TIMEOUT: c_int = 45;
pub const IBMVFC_ADISC_TIMEOUT: c_int = 15;

pub const IBMVFC_INIT_TIMEOUT: c_int = 120;
pub const IBMVFC_ABORT_TIMEOUT: c_int = 8;
pub const IBMVFC_ABORT_WAIT_TIMEOUT: c_int = 40;
pub const IBMVFC_MAX_REQUESTS_DEFAULT: c_int = 100;
pub const IBMVFC_SCSI_QDEPTH: c_int = 128;
pub const IBMVFC_DEBUG: c_int = 0;
pub const IBMVFC_MAX_TARGETS: c_int = 1024;
pub const IBMVFC_MAX_LUN: c_uint = 0xffffffff;
pub const IBMVFC_MAX_SECTORS: c_int = 2048;
pub const IBMVFC_MAX_DISC_THREADS: c_int = 4;
pub const IBMVFC_TGT_MEMPOOL_SZ: c_int = 64;
pub const IBMVFC_MAX_CMDS_PER_LUN: c_int = 64;
pub const IBMVFC_MAX_HOST_INIT_RETRIES: c_int = 6;
pub const IBMVFC_MAX_TGT_INIT_RETRIES: c_int = 3;

pub const IBMVFC_DEFAULT_LOG_LEVEL: c_int = 2;
pub const IBMVFC_MAX_CDB_LEN: c_int = 16;
pub const IBMVFC_CLS3_ERROR: c_int = 0;
pub const IBMVFC_MQ: c_int = 1;
pub const IBMVFC_SCSI_CHANNELS: c_int = 8;
pub const IBMVFC_MAX_SCSI_QUEUES: c_int = 16;
pub const IBMVFC_SCSI_HW_QUEUES: c_int = 8;
pub const IBMVFC_MIG_NO_SUB_TO_CRQ: c_int = 0;
pub const IBMVFC_MIG_NO_N_TO_M: c_int = 0;
//
// Ensure we have resources for ERP and initialization:
// 1 for ERP
// 1 for initialization
// 1 for NPIV Logout
// 2 for BSG passthru
// 2 for each discovery thread
//

// Reserved suset of events for cancelling channelized IO commands
pub const IBMVFC_NUM_INTERNAL_SUBQ_REQ: c_int = 4;
pub const IBMVFC_MAD_SUCCESS: c_uint = 0x00;
pub const IBMVFC_MAD_NOT_SUPPORTED: c_uint = 0xF1;
pub const IBMVFC_MAD_VERSION_NOT_SUPP: c_uint = 0xF2;
pub const IBMVFC_MAD_FAILED: c_uint = 0xF7;
pub const IBMVFC_MAD_DRIVER_FAILED: c_uint = 0xEE;
pub const IBMVFC_MAD_CRQ_ERROR: c_uint = 0xEF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_crq_valid {
    IBMVFC_CRQ_CMD_RSP		= 0x80,
    IBMVFC_CRQ_INIT_RSP		= 0xC0,
    IBMVFC_CRQ_XPORT_EVENT		= 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_crq_init_msg {
    IBMVFC_CRQ_INIT			= 0x01,
    IBMVFC_CRQ_INIT_COMPLETE	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_crq_xport_evts {
    IBMVFC_PARTNER_FAILED		= 0x01,
    IBMVFC_PARTNER_DEREGISTER	= 0x02,
    IBMVFC_PARTITION_MIGRATED	= 0x06,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_cmd_status_flags {
    IBMVFC_FABRIC_MAPPED		= 0x0001,
    IBMVFC_VIOS_FAILURE		= 0x0002,
    IBMVFC_FC_FAILURE			= 0x0004,
    IBMVFC_FC_SCSI_ERROR		= 0x0008,
    IBMVFC_HW_EVENT_LOGGED		= 0x0010,
    IBMVFC_VIOS_LOGGED		= 0x0020,
    IBMVFC_FC_NVME_STATUS		= 0x0040,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_fabric_mapped_errors {
    IBMVFC_UNABLE_TO_ESTABLISH	= 0x0001,
    IBMVFC_XPORT_FAULT		= 0x0002,
    IBMVFC_CMD_TIMEOUT		= 0x0003,
    IBMVFC_ENETDOWN			= 0x0004,
    IBMVFC_HW_FAILURE			= 0x0005,
    IBMVFC_LINK_DOWN_ERR		= 0x0006,
    IBMVFC_LINK_DEAD_ERR		= 0x0007,
    IBMVFC_UNABLE_TO_REGISTER	= 0x0008,
    IBMVFC_XPORT_BUSY			= 0x000A,
    IBMVFC_XPORT_DEAD			= 0x000B,
    IBMVFC_CONFIG_ERROR		= 0x000C,
    IBMVFC_NAME_SERVER_FAIL		= 0x000D,
    IBMVFC_LINK_HALTED		= 0x000E,
    IBMVFC_XPORT_GENERAL		= 0x8000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_vios_errors {
    IBMVFC_CRQ_FAILURE			= 0x0001,
    IBMVFC_SW_FAILURE				= 0x0002,
    IBMVFC_INVALID_PARAMETER		= 0x0003,
    IBMVFC_MISSING_PARAMETER		= 0x0004,
    IBMVFC_HOST_IO_BUS			= 0x0005,
    IBMVFC_TRANS_CANCELLED			= 0x0006,
    IBMVFC_TRANS_CANCELLED_IMPLICIT	= 0x0007,
    IBMVFC_INSUFFICIENT_RESOURCE		= 0x0008,
    IBMVFC_PLOGI_REQUIRED			= 0x0010,
    IBMVFC_COMMAND_FAILED			= 0x8000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_fc_nvme_errors {
    IBMVFC_NVMS_VALID_ERSP		= 0x0001,
    IBMVFC_NVMS_VALID_NODMA_CQE	= 0x0002,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_mad_types {
    IBMVFC_NPIV_LOGIN		= 0x0001,
    IBMVFC_DISC_TARGETS		= 0x0002,
    IBMVFC_DISC_NVMF_TARGETS	= 0x0003,
    IBMVFC_PORT_LOGIN		= 0x0004,
    IBMVFC_NVMF_PORT_LOGIN		= 0x0005,
    IBMVFC_PROCESS_LOGIN		= 0x0008,
    IBMVFC_NVMF_PROCESS_LOGIN	= 0x0009,
    IBMVFC_QUERY_TARGET		= 0x0010,
    IBMVFC_NVMF_QUERY_TARGET	= 0x0011,
    IBMVFC_MOVE_LOGIN		= 0x0020,
    IBMVFC_NVMF_MOVE_LOGIN		= 0x0021,
    IBMVFC_IMPLICIT_LOGOUT		= 0x0040,
    IBMVFC_NVMF_IMPLICIT_LOGOUT	= 0x0041,
    IBMVFC_RNID			= 0x0080,
    IBMVFC_NVMF_RNID		= 0x0081,
    IBMVFC_TMF_MAD			= 0x0100,
    IBMVFC_NVMF_TMF_MAD		= 0x0101,
    IBMVFC_PASSTHRU			= 0x0200,
    IBMVFC_NVMF_PASSTHRU		= 0x0201,
    IBMVFC_NPIV_LOGOUT		= 0x0800,
    IBMVFC_CHANNEL_ENQUIRY		= 0x1000,
    IBMVFC_CHANNEL_SETUP		= 0x2000,
    IBMVFC_CONNECTION_INFO		= 0x4000,
    IBMVFC_FABRIC_LOGIN		= 0x8000,
    IBMVFC_NVMF_FABRIC_LOGIN	= 0x8001,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_mad_common {
    pub version: __be32,
    pub reserved: __be32,
    pub opcode: __be32,
    pub status: __be16,
    pub length: __be16,
    pub tag: __be64,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_npiv_login_mad {
    pub common: ibmvfc_mad_common,
    pub buffer: srp_direct_buf,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_npiv_logout_mad {
    pub common: ibmvfc_mad_common,
    pub __aligned(8): } __packed,
pub const IBMVFC_MAX_NAME: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_npiv_login {
    pub ostype: __be32,
pub const IBMVFC_OS_LINUX: c_uint = 0x02;
    pub pad: __be32,
    pub max_dma_len: __be64,
    pub max_payload: __be32,
    pub max_response: __be32,
    pub partition_num: __be32,
    pub vfc_frame_version: __be32,
    pub fcp_version: __be16,
    pub flags: __be16,
pub const IBMVFC_CLIENT_MIGRATED: c_uint = 0x01;
pub const IBMVFC_FLUSH_ON_HALT: c_uint = 0x02;
    pub max_cmds: __be32,
    pub capabilities: __be64,
pub const IBMVFC_CAN_MIGRATE: c_uint = 0x001;
pub const IBMVFC_CAN_USE_CHANNELS: c_uint = 0x002;
pub const IBMVFC_CAN_HANDLE_FPIN: c_uint = 0x004;
pub const IBMVFC_CAN_USE_MAD_VERSION: c_uint = 0x008;
pub const IBMVFC_CAN_SEND_VF_WWPN: c_uint = 0x010;
pub const IBMVFC_YES_NVMEOF: c_uint = 0x020;
pub const IBMVFC_YES_SCSI: c_uint = 0x040;
pub const IBMVFC_CAN_USE_WWPN_ALL: c_uint = 0x080;
pub const IBMVFC_USE_ASYNC_SUBQ: c_uint = 0x100;
pub const IBMVFC_CAN_USE_NOOP_CMD: c_uint = 0x200;
    pub node_name: __be64,
    pub async: srp_direct_buf,
    pub partition_name: [u8; IBMVFC_MAX_NAME],
    pub device_name: [u8; IBMVFC_MAX_NAME],
    pub drc_name: [u8; IBMVFC_MAX_NAME],
    pub reserved2: [__be64; 2],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_common_svc_parms {
    pub fcph_version: __be16,
    pub b2b_credit: __be16,
    pub features: __be16,
    pub /: *mut *mut __be16 bb_rcv_sz; / upper nibble is BB_SC_N,
    pub ratov: __be32,
    pub edtov: __be32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_service_parms {
    pub common: ibmvfc_common_svc_parms,
    pub port_name: [u8; 8],
    pub node_name: [u8; 8],
    pub class1_parms: [__be32; 4],
    pub class2_parms: [__be32; 4],
    pub class3_parms: [__be32; 4],
    pub obsolete: [__be32; 4],
    pub vendor_version: [__be32; 4],
    pub services_avail: [__be32; 2],
    pub ext_len: __be32,
    pub reserved: [__be32; 30],
    pub clk_sync_qos: [__be32; 2],
    pub reserved2: __be32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_npiv_login_resp {
    pub version: __be32,
    pub status: __be16,
    pub error: __be16,
    pub flags: __be32,
pub const IBMVFC_NATIVE_FC: c_uint = 0x01;
    pub possible_nports: __be32,
    pub capabilities: __be64,
pub const IBMVFC_CAN_FLUSH_ON_HALT: c_uint = 0x0008;
pub const IBMVFC_CAN_SUPPRESS_ABTS: c_uint = 0x0010;
pub const IBMVFC_MAD_VERSION_CAP: c_uint = 0x0020;
pub const IBMVFC_HANDLE_VF_WWPN: c_uint = 0x0040;
pub const IBMVFC_CAN_SUPPORT_CHANNELS: c_uint = 0x0080;
pub const IBMVFC_SUPPORT_NVMEOF: c_uint = 0x0100;
pub const IBMVFC_SUPPORT_SCSI: c_uint = 0x0200;
pub const IBMVFC_SUPPORT_WWPN_ALL: c_uint = 0x0400;
pub const IBMVFC_ASYNC_SUBQ: c_uint = 0x0800;
pub const IBMVFC_SUPPORT_NOOP_CMD: c_uint = 0x1000;
    pub max_cmds: __be32,
    pub scsi_id_sz: __be32,
    pub max_dma_len: __be64,
    pub scsi_id: __be64,
    pub port_name: __be64,
    pub node_name: __be64,
    pub link_speed: __be64,
    pub partition_name: [u8; IBMVFC_MAX_NAME],
    pub device_name: [u8; IBMVFC_MAX_NAME],
    pub port_loc_code: [u8; IBMVFC_MAX_NAME],
    pub drc_name: [u8; IBMVFC_MAX_NAME],
    pub service_parms: ibmvfc_service_parms,
    pub reserved: __be64,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibmvfc_npiv_login_data {
    pub login: ibmvfc_npiv_login,
    pub resp: ibmvfc_npiv_login_resp,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_fabric_login_mad {
    pub common: ibmvfc_mad_common,
    pub flags: __be64,
    pub capabilities: __be64,
    pub nport_id: __be64,
    pub status: __be16,
    pub error: __be16,
    pub reserved: __be32,
    pub reserved2: [__be64; 16],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_discover_targets_entry {
    pub scsi_id: __be32,
    pub pad: __be32,
    pub wwpn: __be64,
pub const IBMVFC_DISC_TGT_SCSI_ID_MASK: c_uint = 0x00ffffff;
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_discover_targets {
    pub common: ibmvfc_mad_common,
    pub buffer: srp_direct_buf,
    pub flags: __be32,
pub const IBMVFC_DISC_TGT_PORT_ID_WWPN_LIST: c_uint = 0x02;
    pub status: __be16,
    pub error: __be16,
    pub bufflen: __be32,
    pub num_avail: __be32,
    pub num_written: __be32,
    pub reserved: [__be64; 2],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_fc_reason {
    IBMVFC_INVALID_ELS_CMD_CODE	= 0x01,
    IBMVFC_INVALID_VERSION		= 0x02,
    IBMVFC_LOGICAL_ERROR		= 0x03,
    IBMVFC_INVALID_CT_IU_SIZE	= 0x04,
    IBMVFC_LOGICAL_BUSY		= 0x05,
    IBMVFC_PROTOCOL_ERROR		= 0x07,
    IBMVFC_UNABLE_TO_PERFORM_REQ	= 0x09,
    IBMVFC_CMD_NOT_SUPPORTED	= 0x0B,
    IBMVFC_SERVER_NOT_AVAIL		= 0x0D,
    IBMVFC_CMD_IN_PROGRESS		= 0x0E,
    IBMVFC_VENDOR_SPECIFIC		= 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_fc_type {
    IBMVFC_FABRIC_REJECT	= 0x01,
    IBMVFC_PORT_REJECT	= 0x02,
    IBMVFC_LS_REJECT		= 0x03,
    IBMVFC_FABRIC_BUSY	= 0x04,
    IBMVFC_PORT_BUSY		= 0x05,
    IBMVFC_BASIC_REJECT	= 0x06,
    IBMVFC_FC4_LS_REJECT	= 0x07,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_gs_explain {
    IBMVFC_PORT_NAME_NOT_REG	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_port_login {
    pub common: ibmvfc_mad_common,
    pub scsi_id: __be64,
    pub reserved: __be16,
    pub fc_service_class: __be16,
    pub blksz: __be32,
    pub hdr_per_blk: __be32,
    pub status: __be16,
    pub /: *mut *mut __be16 error; / also fc_reason,
    pub fc_explain: __be16,
    pub fc_type: __be16,
    pub reserved2: __be32,
    pub service_parms: ibmvfc_service_parms,
    pub service_parms_change: ibmvfc_service_parms,
    pub target_wwpn: __be64,
    pub reserved3: [__be64; 2],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_move_login {
    pub common: ibmvfc_mad_common,
    pub old_scsi_id: __be64,
    pub new_scsi_id: __be64,
    pub wwpn: __be64,
    pub node_name: __be64,
    pub flags: __be32,
pub const IBMVFC_MOVE_LOGIN_IMPLICIT_OLD_FAILED: c_uint = 0x01;
pub const IBMVFC_MOVE_LOGIN_IMPLICIT_NEW_FAILED: c_uint = 0x02;
pub const IBMVFC_MOVE_LOGIN_PORT_LOGIN_FAILED: c_uint = 0x04;
    pub reserved: __be32,
    pub service_parms: ibmvfc_service_parms,
    pub service_parms_change: ibmvfc_service_parms,
    pub reserved2: __be32,
    pub service_class: __be16,
    pub vios_flags: __be16,
pub const IBMVFC_MOVE_LOGIN_VF_NOT_SENT_ADAPTER: c_uint = 0x01;
    pub reserved3: __be64,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_prli_svc_parms {
    pub type: u8,
pub const IBMVFC_SCSI_FCP_TYPE: c_uint = 0x08;
pub const IBMVFC_NVME_FCP_TYPE: c_uint = 0x28;
    pub type_ext: u8,
    pub flags: __be16,
pub const IBMVFC_PRLI_ORIG_PA_VALID: c_uint = 0x8000;
pub const IBMVFC_PRLI_RESP_PA_VALID: c_uint = 0x4000;
pub const IBMVFC_PRLI_EST_IMG_PAIR: c_uint = 0x2000;
    pub orig_pa: __be32,
    pub resp_pa: __be32,
    pub service_parms: __be32,
pub const IBMVFC_PRLI_TASK_RETRY: c_uint = 0x00000200;
pub const IBMVFC_PRLI_RETRY: c_uint = 0x00000100;
pub const IBMVFC_PRLI_DATA_OVERLAY: c_uint = 0x00000040;
pub const IBMVFC_PRLI_INITIATOR_FUNC: c_uint = 0x00000020;
pub const IBMVFC_PRLI_TARGET_FUNC: c_uint = 0x00000010;
pub const IBMVFC_PRLI_READ_FCP_XFER_RDY_DISABLED: c_uint = 0x00000002;
pub const IBMVFC_PRLI_WR_FCP_XFER_RDY_DISABLED: c_uint = 0x00000001;
pub const IBMVFC_PRLI_NVME_PI_CTRL: c_uint = 0x00000200;
pub const IBMVFC_PRLI_NVME_SLER: c_uint = 0x00000100;
pub const IBMVFC_PRLI_NVME_INITIATOR: c_uint = 0x00000020;
pub const IBMVFC_PRLI_NVME_TARGET: c_uint = 0x00000010;
pub const IBMVFC_PRLI_NVME_DISCOVERY: c_uint = 0x00000008;
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_process_login {
    pub common: ibmvfc_mad_common,
    pub scsi_id: __be64,
    pub parms: ibmvfc_prli_svc_parms,
    pub reserved: [u8; 48],
    pub status: __be16,
    pub /: *mut *mut __be16 error; / also fc_reason,
    pub reserved2: __be32,
    pub target_wwpn: __be64,
    pub reserved3: [__be64; 2],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_query_tgt {
    pub common: ibmvfc_mad_common,
    pub wwpn: __be64,
    pub scsi_id: __be64,
    pub status: __be16,
    pub error: __be16,
    pub fc_explain: __be16,
    pub fc_type: __be16,
    pub reserved: [__be64; 2],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_implicit_logout {
    pub common: ibmvfc_mad_common,
    pub old_scsi_id: __be64,
    pub reserved: [__be64; 8],
    pub target_wwpn: __be64,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_tmf {
    pub common: ibmvfc_mad_common,
    pub scsi_id: __be64,
    pub lun: scsi_lun,
    pub assoc_id: __be64,
}

pub const IBMVFC_TMF_ABORT_TASK: c_uint = 0x002;
pub const IBMVFC_TMF_ABORT_TASK_SET: c_uint = 0x004;
pub const IBMVFC_TMF_LUN_RESET: c_uint = 0x010;
pub const IBMVFC_TMF_TGT_RESET: c_uint = 0x020;
pub const IBMVFC_TMF_LUA_VALID: c_uint = 0x040;
pub const IBMVFC_TMF_SUPPRESS_ABTS: c_uint = 0x080;
pub const IBMVFC_TMF_NVMF_ASSOC: c_uint = 0x100;
pub const IBMVFC_TMF_NVMF_TARGET: c_uint = 0x200;
pub const IBMVFC_TMF_BITMASK_VALID: c_uint = 0x400;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_fcp_rsp_info_codes {
    RSP_NO_FAILURE		= 0x00,
    RSP_TMF_REJECTED		= 0x04,
    RSP_TMF_FAILED		= 0x05,
    RSP_TMF_INVALID_LUN	= 0x09,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_fcp_rsp_info {
    pub reserved: [u8; 3],
    pub rsp_code: u8,
    pub reserved2: [u8; 4],
    pub __aligned(2): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_fcp_rsp_flags {
    FCP_BIDI_RSP			= 0x80,
    FCP_BIDI_READ_RESID_UNDER	= 0x40,
    FCP_BIDI_READ_RESID_OVER	= 0x20,
    FCP_CONF_REQ			= 0x10,
    FCP_RESID_UNDER			= 0x08,
    FCP_RESID_OVER			= 0x04,
    FCP_SNS_LEN_VALID			= 0x02,
    FCP_RSP_LEN_VALID			= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ibmvfc_fcp_rsp_data {
    pub info: ibmvfc_fcp_rsp_info,
    pub ibmvfc_fcp_rsp_info)]: u8 sense[SCSI_SENSE_BUFFERSIZE + sizeof(struct,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_fcp_rsp {
    pub reserved: __be64,
    pub retry_delay_timer: __be16,
    pub flags: u8,
    pub scsi_status: u8,
    pub fcp_resid: __be32,
    pub fcp_sense_len: __be32,
    pub fcp_rsp_len: __be32,
    pub data: ibmvfc_fcp_rsp_data,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_cmd_flags {
    IBMVFC_SCATTERLIST	= 0x0001,
    IBMVFC_NO_MEM_DESC	= 0x0002,
    IBMVFC_READ			= 0x0004,
    IBMVFC_WRITE		= 0x0008,
    IBMVFC_TMF			= 0x0080,
    IBMVFC_CLASS_3_ERR	= 0x0100,
    IBMVFC_NVMEOF_PROTOCOL	= 0x0200,
    IBMVFC_NVMF_SLER	= 0x0400,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_fc_task_attr {
    IBMVFC_SIMPLE_TASK	= 0x00,
    IBMVFC_HEAD_OF_QUEUE	= 0x01,
    IBMVFC_ORDERED_TASK	= 0x02,
    IBMVFC_ACA_TASK		= 0x04,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_fc_tmf_flags {
    IBMVFC_ABORT_TASK_SET	= 0x02,
    IBMVFC_LUN_RESET		= 0x10,
    IBMVFC_TARGET_RESET	= 0x20,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_fcp_cmd_iu {
    pub lun: scsi_lun,
    pub crn: u8,
    pub pri_task_attr: u8,
    pub tmf_flags: u8,
    pub add_cdb_len: u8,
pub const IBMVFC_RDDATA: c_uint = 0x02;
pub const IBMVFC_WRDATA: c_uint = 0x01;
    pub cdb: [u8; IBMVFC_MAX_CDB_LEN],
    pub xfer_len: __be32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_cmd {
    pub task_tag: __be64,
    pub frame_type: __be32,
    pub payload_len: __be32,
    pub resp_len: __be32,
    pub adapter_resid: __be32,
    pub status: __be16,
    pub error: __be16,
    pub flags: __be16,
    pub response_flags: __be16,
pub const IBMVFC_ADAPTER_RESID_VALID: c_uint = 0x01;
    pub cancel_key: __be32,
    pub exchange_id: __be32,
    pub ext_func: srp_direct_buf,
    pub ioba: srp_direct_buf,
    pub resp: srp_direct_buf,
    pub correlation: __be64,
    pub tgt_scsi_id: __be64,
    pub tag: __be64,
    pub target_wwpn: __be64,
    pub assoc_id: __be64,
    pub iu: ibmvfc_fcp_cmd_iu,
    pub rsp: ibmvfc_fcp_rsp,
    pub v1: },
    pub reserved4: __be64,
    pub iu: ibmvfc_fcp_cmd_iu,
    pub rsp: ibmvfc_fcp_rsp,
    pub v2: },
    pub reserved5: [__be64; 4],
    pub iu: ibmvfc_fcp_cmd_iu,
    pub rsp: ibmvfc_fcp_rsp,
    pub v3scsi: },
    pub reserved: [__be64; 4],
    pub iu: nvme_fc_cmd_iu,
    pub v3nvme: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_passthru_fc_iu {
    pub payload: [__be32; 7],
pub const IBMVFC_ADISC: c_uint = 0x52000000;
    pub response: [__be32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_passthru_iu {
    pub task_tag: __be64,
    pub cmd_len: __be32,
    pub rsp_len: __be32,
    pub status: __be16,
    pub error: __be16,
    pub flags: __be32,
pub const IBMVFC_FC_ELS: c_uint = 0x01;
pub const IBMVFC_FC_CT_IU: c_uint = 0x02;
pub const IBMVFC_PT_PRLI: c_uint = 0x04;
pub const IBMVFC_FC4_LS_OTH: c_uint = 0x08;
pub const IBMVFC_FC4_LS_DSC_CTRL: c_uint = 0x10;
    pub cancel_key: __be32,
pub const IBMVFC_PASSTHRU_CANCEL_KEY: c_uint = 0x80000000;
pub const IBMVFC_INTERNAL_CANCEL_KEY: c_uint = 0x80000001;
    pub reserved: __be32,
    pub cmd: srp_direct_buf,
    pub rsp: srp_direct_buf,
    pub correlation: __be64,
    pub scsi_id: __be64,
    pub tag: __be64,
    pub target_wwpn: __be64,
    pub reserved2: [__be64; 2],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_passthru_mad {
    pub common: ibmvfc_mad_common,
    pub cmd_ioba: srp_direct_buf,
    pub iu: ibmvfc_passthru_iu,
    pub fc_iu: ibmvfc_passthru_fc_iu,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_channel_enquiry {
    pub common: ibmvfc_mad_common,
    pub flags: __be32,
pub const IBMVFC_NO_CHANNELS_TO_CRQ_SUPPORT: c_uint = 0x01;
pub const IBMVFC_SUPPORT_VARIABLE_SUBQ_MSG: c_uint = 0x02;
pub const IBMVFC_NO_N_TO_M_CHANNELS_SUPPORT: c_uint = 0x04;
    pub num_scsi_subq_channels: __be32,
    pub num_nvme_subq_channels: __be32,
    pub num_scsi_vas_channels: __be32,
    pub num_nvme_vas_channels: __be32,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_channel_setup_mad {
    pub common: ibmvfc_mad_common,
    pub buffer: srp_direct_buf,
    pub __aligned(8): } __packed,
pub const IBMVFC_MAX_CHANNELS: c_int = 501;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_channel_setup {
    pub flags: __be32,
pub const IBMVFC_CANCEL_CHANNELS: c_uint = 0x01;
pub const IBMVFC_USE_BUFFER: c_uint = 0x02;
pub const IBMVFC_CHANNELS_CANCELED: c_uint = 0x04;
    pub reserved: __be32,
    pub num_scsi_subq_channels: __be32,
    pub num_nvme_subq_channels: __be32,
    pub num_scsi_vas_channels: __be32,
    pub num_nvme_vas_channels: __be32,
    pub buffer: srp_direct_buf,
    pub reserved2: [__be64; 5],
    pub channel_handles: [__be64; IBMVFC_MAX_CHANNELS],
    pub async_sub_crq_handle: __be64,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_connection_info {
    pub common: ibmvfc_mad_common,
    pub information_bits: __be64,
pub const IBMVFC_NO_FC_IO_CHANNEL: c_uint = 0x01;
pub const IBMVFC_NO_PHYP_VAS: c_uint = 0x02;
pub const IBMVFC_NO_PHYP_SUBQ: c_uint = 0x04;
pub const IBMVFC_PHYP_DEPRECATED_SUBQ: c_uint = 0x08;
pub const IBMVFC_PHYP_PRESERVED_SUBQ: c_uint = 0x10;
pub const IBMVFC_PHYP_FULL_SUBQ: c_uint = 0x20;
    pub reserved: [__be64; 16],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_trace_start_entry {
    pub xfer_len: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_trace_end_entry {
    pub status: u16,
    pub error: u16,
    pub fcp_rsp_flags: u8,
    pub rsp_code: u8,
    pub scsi_status: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_trace_entry {
    pub evt: *mut ibmvfc_event,
    pub time: u32,
    pub scsi_id: u32,
    pub lun: u32,
    pub fmt: u8,
    pub op_code: u8,
    pub tmf_flags: u8,
    pub type: u8,
pub const IBMVFC_TRC_START: c_uint = 0x00;
pub const IBMVFC_TRC_END: c_uint = 0xff;
    pub start: ibmvfc_trace_start_entry,
    pub end: ibmvfc_trace_end_entry,
    pub u: },
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_crq_formats {
    IBMVFC_CMD_FORMAT		= 0x01,
    IBMVFC_ASYNC_EVENT		= 0x02,
    IBMVFC_NOOP			= 0x03,
    IBMVFC_MAD_FORMAT		= 0x04,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_async_event {
    IBMVFC_AE_ELS_PLOGI		= 0x0001,
    IBMVFC_AE_ELS_LOGO		= 0x0002,
    IBMVFC_AE_ELS_PRLO		= 0x0004,
    IBMVFC_AE_SCN_NPORT		= 0x0008,
    IBMVFC_AE_SCN_GROUP		= 0x0010,
    IBMVFC_AE_SCN_DOMAIN		= 0x0020,
    IBMVFC_AE_SCN_FABRIC		= 0x0040,
    IBMVFC_AE_LINK_UP			= 0x0080,
    IBMVFC_AE_LINK_DOWN		= 0x0100,
    IBMVFC_AE_LINK_DEAD		= 0x0200,
    IBMVFC_AE_HALT			= 0x0400,
    IBMVFC_AE_RESUME			= 0x0800,
    IBMVFC_AE_ADAPTER_FAILED	= 0x1000,
    IBMVFC_AE_FPIN			= 0x2000,
    IBMVFC_NVME_DISCONNECT		= 0x4000,
    IBMVFC_NVMEOF_PROTO_AVAIL	= 0x40000000,
    IBMVFC_SCSI_PROTO_AVAIL		= 0x80000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_async_desc {
    pub desc: *const c_char,
    pub ae: ibmvfc_async_event,
    pub log_level: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_crq {
    pub valid: volatile u8,
    pub format: volatile u8,
    pub reserved: [u8; 6],
    pub ioba: volatile __be64,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_sub_crq {
    pub crq: ibmvfc_crq,
    pub reserved: [__be64; 2],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_ae_link_state {
    IBMVFC_AE_LS_LINK_UP		= 0x01,
    IBMVFC_AE_LS_LINK_BOUNCED	= 0x02,
    IBMVFC_AE_LS_LINK_DOWN		= 0x04,
    IBMVFC_AE_LS_LINK_DEAD		= 0x08,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_ae_fpin_status {
    IBMVFC_AE_FPIN_LINK_CONGESTED	= 0x1,
    IBMVFC_AE_FPIN_PORT_CONGESTED	= 0x2,
    IBMVFC_AE_FPIN_PORT_CLEARED	= 0x3,
    IBMVFC_AE_FPIN_PORT_DEGRADED	= 0x4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_async_crq {
    pub valid: volatile u8,
    pub link_state: u8,
    pub fpin_status: u8,
    pub pad: u8,
    pub pad2: __be32,
    pub event: volatile __be64,
    pub scsi_id: volatile __be64,
    pub wwpn: volatile __be64,
    pub node_name: volatile __be64,
    pub assoc_id: __be64,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_async_sub_crq {
    pub valid: volatile u8,
    pub flags: u8,
pub const IBMVFC_ASYNC_ID_IS_ASSOC_ID: c_uint = 0x01;
    pub link_state: u8,
    pub fpin_status: u8,
    pub event: __be16,
    pub pad: __be16,
    pub wwpn: __be64,
    pub nport_id: __be64,
    pub node_name: __be64,
    pub assoc_id: __be64,
    pub id: },
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibmvfc_iu {
    pub mad_common: ibmvfc_mad_common,
    pub npiv_login: ibmvfc_npiv_login_mad,
    pub npiv_logout: ibmvfc_npiv_logout_mad,
    pub fabric_login: ibmvfc_fabric_login_mad,
    pub discover_targets: ibmvfc_discover_targets,
    pub plogi: ibmvfc_port_login,
    pub prli: ibmvfc_process_login,
    pub move_login: ibmvfc_move_login,
    pub query_tgt: ibmvfc_query_tgt,
    pub implicit_logout: ibmvfc_implicit_logout,
    pub tmf: ibmvfc_tmf,
    pub cmd: ibmvfc_cmd,
    pub passthru: ibmvfc_passthru_mad,
    pub channel_enquiry: ibmvfc_channel_enquiry,
    pub channel_setup: ibmvfc_channel_setup_mad,
    pub connection_info: ibmvfc_connection_info,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_target_action {
    IBMVFC_TGT_ACTION_NONE = 0,
    IBMVFC_TGT_ACTION_INIT,
    IBMVFC_TGT_ACTION_INIT_WAIT,
    IBMVFC_TGT_ACTION_LOGOUT_RPORT,
    IBMVFC_TGT_ACTION_LOGOUT_RPORT_WAIT,
    IBMVFC_TGT_ACTION_DEL_RPORT,
    IBMVFC_TGT_ACTION_DELETED_RPORT,
    IBMVFC_TGT_ACTION_DEL_AND_LOGOUT_RPORT,
    IBMVFC_TGT_ACTION_LOGOUT_DELETED_RPORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_protocol {
    IBMVFC_PROTO_SCSI = 0,
    IBMVFC_PROTO_NVME = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_target {
    pub queue: list_head,
    pub vhost: *mut ibmvfc_host,
    pub protocol: ibmvfc_protocol,
    pub scsi_id: u64,
    pub wwpn: u64,
    pub new_scsi_id: u64,
    pub assoc_id: u64,
    pub rport: *mut fc_rport,
    pub target_id: c_int,
    pub action: ibmvfc_target_action,
    pub need_login: c_int,
    pub add_rport: c_int,
    pub init_retries: c_int,
    pub logo_rcvd: c_int,
    pub move_login: c_int,
    pub cancel_key: u32,
    pub service_parms: ibmvfc_service_parms,
    pub service_parms_change: ibmvfc_service_parms,
    pub ids: fc_rport_identifiers,
    pub ): *mut *mut void (job_step) (struct ibmvfc_target,
    pub timer: timer_list,
    pub kref: kref,
    pub nvme_delete_done: completion,
    pub nvme_remote_port: *mut nvme_fc_remote_port,
}

// a unit of work for the hosting partition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_event {
    pub queue_list: list_head,
    pub cancel: list_head,
    pub vhost: *mut ibmvfc_host,
    pub queue: *mut ibmvfc_queue,
    pub tgt: *mut ibmvfc_target,
    pub cmnd: *mut scsi_cmnd,
    pub ls_req: *mut nvmefc_ls_req,
    pub fcp_req: *mut nvmefc_fcp_req,
    pub free: core::sync::atomic::AtomicI32,
    pub active: core::sync::atomic::AtomicI32,
    pub xfer_iu: *mut ibmvfc_iu,
    pub evt): *mut *mut void (done)(struct ibmvfc_event,
    pub evt): *mut *mut void (_done)(struct ibmvfc_event,
    pub crq: ibmvfc_crq,
    pub iu: ibmvfc_iu,
    pub sync_iu: *mut ibmvfc_iu,
    pub ext_list: *mut srp_direct_buf,
    pub ext_list_token: dma_addr_t,
    pub comp: completion,
    pub eh_comp: *mut completion,
    pub timer: timer_list,
    pub hwq: u16,
    pub reserved: u8,
}

// a pool of event structs for use
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_event_pool {
    pub events: *mut ibmvfc_event,
    pub size: u32,
    pub iu_storage: *mut ibmvfc_iu,
    pub iu_token: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_msg_fmt {
    IBMVFC_CRQ_FMT = 0,
    IBMVFC_ASYNC_FMT,
    IBMVFC_SUB_CRQ_FMT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ibmvfc_msgs {
    pub handle: *mut c_void,
    pub crq: *mut ibmvfc_crq,
    pub async: *mut ibmvfc_async_crq,
    pub scrq: *mut ibmvfc_sub_crq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_queue {
    pub msgs: ibmvfc_msgs,
    pub msg_token: dma_addr_t,
    pub fmt: ibmvfc_msg_fmt,
    pub cur: int size,,
    pub _lock: spinlock_t,
    pub q_lock: *mut spinlock_t,
    pub vhost: *mut ibmvfc_host,
    pub evt_pool: ibmvfc_event_pool,
    pub sent: list_head,
    pub free: list_head,
    pub total_depth: u16,
    pub evt_depth: u16,
    pub reserved_depth: u16,
    pub evt_free: u16,
    pub reserved_free: u16,
    pub l_lock: spinlock_t,
    pub cancel_rsp: ibmvfc_iu,
// Sub-CRQ fields
    pub cookie: c_ulong,
    pub vios_cookie: c_ulong,
    pub hw_irq: c_ulong,
    pub irq: c_ulong,
    pub hwq_id: c_ulong,
    pub name: [c_char; 32],
    pub handler: irq_handler_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_channels {
    pub scrqs: *mut ibmvfc_queue,
    pub protocol: ibmvfc_protocol,
    pub active_queues: c_uint,
    pub desired_queues: c_uint,
    pub max_queues: c_uint,
    pub num_targets: c_int,
    pub targets: list_head,
    pub disc_buf_sz: c_int,
    pub disc_buf: *mut ibmvfc_discover_targets_entry,
    pub disc_buf_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_host_action {
    IBMVFC_HOST_ACTION_NONE = 0,
    IBMVFC_HOST_ACTION_RESET,
    IBMVFC_HOST_ACTION_REENABLE,
    IBMVFC_HOST_ACTION_LOGO,
    IBMVFC_HOST_ACTION_LOGO_WAIT,
    IBMVFC_HOST_ACTION_INIT,
    IBMVFC_HOST_ACTION_INIT_WAIT,
    IBMVFC_HOST_ACTION_QUERY,
    IBMVFC_HOST_ACTION_QUERY_TGTS,
    IBMVFC_HOST_ACTION_TGT_DEL,
    IBMVFC_HOST_ACTION_ALLOC_TGTS,
    IBMVFC_HOST_ACTION_TGT_INIT,
    IBMVFC_HOST_ACTION_TGT_DEL_FAILED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_host_state {
    IBMVFC_NO_CRQ = 0,
    IBMVFC_INITIALIZING,
    IBMVFC_ACTIVE,
    IBMVFC_HALTED,
    IBMVFC_LINK_DOWN,
    IBMVFC_LINK_DEAD,
    IBMVFC_HOST_OFFLINE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_host {
    pub name: [c_char; 8],
    pub queue: list_head,
    pub host: *mut Scsi_Host,
    pub state: ibmvfc_host_state,
    pub action: ibmvfc_host_action,
pub const IBMVFC_NUM_TRACE_INDEX_BITS: c_int = 8;

    pub trace: *mut ibmvfc_trace_entry,
    pub trace_index: core::sync::atomic::AtomicI32,
    pub purge: list_head,
    pub dev: *mut device,
    pub sg_pool: *mut dma_pool,
    pub tgt_pool: *mut mempool_t,
    pub crq: ibmvfc_queue,
    pub async_crq: ibmvfc_queue,
    pub scsi_scrqs: ibmvfc_channels,
    pub nvme_scrqs: ibmvfc_channels,
    pub login_info: ibmvfc_npiv_login,
    pub login_buf: *mut ibmvfc_npiv_login_data,
    pub login_buf_dma: dma_addr_t,
    pub channel_setup_buf: *mut ibmvfc_channel_setup,
    pub channel_setup_dma: dma_addr_t,
    pub log_level: c_int,
    pub passthru_mutex: mutex,
    pub max_vios_scsi_channels: c_uint,
    pub max_vios_nvme_channels: c_uint,
    pub task_set: c_int,
    pub init_retries: c_int,
    pub discovery_threads: c_int,
    pub abort_threads: c_int,
    pub client_migrated:1: c_uint,
    pub reinit:1: c_uint,
    pub delay_init:1: c_uint,
    pub logged_in:1: c_uint,
    pub mq_enabled:1: c_uint,
    pub using_channels:1: c_uint,
    pub do_enquiry:1: c_uint,
    pub nvme_enabled:1: c_uint,
    pub do_scsi_login:1: c_uint,
    pub do_nvme_login:1: c_uint,
    pub aborting_passthru:1: c_uint,
    pub scan_complete:1: c_uint,
    pub scan_timeout: c_int,
    pub events_to_log: c_int,
pub const IBMVFC_AE_LINKUP: c_uint = 0x0001;
pub const IBMVFC_AE_LINKDOWN: c_uint = 0x0002;
pub const IBMVFC_AE_RSCN: c_uint = 0x0004;
    pub partition_number: c_uint,
    pub partition_name: [c_char; 97],
    pub ): *mut *mut void (job_step) (struct ibmvfc_host,
    pub work_thread: *mut task_struct,
    pub tasklet: tasklet_struct,
    pub rport_add_work_q: work_struct,
    pub init_wait_q: wait_queue_head_t,
    pub work_wait_q: wait_queue_head_t,
    pub nvme_local_port: *mut nvme_fc_local_port,
    pub nvme_delete_done: completion,
}

extern "C" {
    pub fn ibmvfc_init_event(evt: *mut ibmvfc_event, ): *mut *mut void (done) (struct ibmvfc_event, format: u8);
}
extern "C" {
    pub fn ibmvfc_free_event(evt: *mut ibmvfc_event);
}
extern "C" {
    pub fn ibmvfc_release_tgt(kref: *mut kref);
}
extern "C" {
    pub fn ibmvfc_send_event(evt: *mut ibmvfc_event, vhost: *mut ibmvfc_host, timeout: c_ulong) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: channels, ibmvfc_host: struct, _arg: scsi_scrqs) -> return;
}
extern "C" {
    pub fn container_of(_arg: channels, ibmvfc_host: struct, _arg: nvme_scrqs) -> return;
}

