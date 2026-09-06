//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/iscsi_common.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//
// ISCSI FW CONSTANTS
//
// iSCSI HSI constants

// KWQ (kernel work queue) layer codes

// iSCSI parameter defaults

// iSCSI parameter limits

pub const ISCSI_AHS_CNTL_SIZE: c_int = 4;

// iSCSI reserved params

pub const ISCSI_OPTION_1_OFF_CHIP_TCP: c_int = 1;
pub const ISCSI_OPTION_2_ON_CHIP_TCP: c_int = 2;
pub const ISCSI_INITIATOR_MODE: c_int = 0;
pub const ISCSI_TARGET_MODE: c_int = 1;
// iSCSI request op codes

// iSCSI response/messages op codes

// iSCSI stages

// iSCSI CQE errors

// Union of data bd_opaque/ tq_tid
#[repr(C)]
#[derive(Copy, Clone)]
pub union bd_opaque_tq_union {
    pub bd_opaque: __le16,
    pub tq_tid: __le16,
}

// ISCSI SGL entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqe_error_bitmap {
    pub cqe_error_status_bits: u8,
pub const CQE_ERROR_BITMAP_DIF_ERR_BITS_MASK: c_uint = 0x7;
pub const CQE_ERROR_BITMAP_DIF_ERR_BITS_SHIFT: c_int = 0;
pub const CQE_ERROR_BITMAP_DATA_DIGEST_ERR_MASK: c_uint = 0x1;
pub const CQE_ERROR_BITMAP_DATA_DIGEST_ERR_SHIFT: c_int = 3;
pub const CQE_ERROR_BITMAP_RCV_ON_INVALID_CONN_MASK: c_uint = 0x1;
pub const CQE_ERROR_BITMAP_RCV_ON_INVALID_CONN_SHIFT: c_int = 4;
pub const CQE_ERROR_BITMAP_DATA_TRUNCATED_ERR_MASK: c_uint = 0x1;
pub const CQE_ERROR_BITMAP_DATA_TRUNCATED_ERR_SHIFT: c_int = 5;
pub const CQE_ERROR_BITMAP_UNDER_RUN_ERR_MASK: c_uint = 0x1;
pub const CQE_ERROR_BITMAP_UNDER_RUN_ERR_SHIFT: c_int = 6;
pub const CQE_ERROR_BITMAP_RESERVED2_MASK: c_uint = 0x1;
pub const CQE_ERROR_BITMAP_RESERVED2_SHIFT: c_int = 7;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cqe_error_status {
    pub error_status: u8,
    pub error_bits: cqe_error_bitmap,
}

// iSCSI Login Response PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_hdr {
    pub data: [__le32; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lun_mapper_addr_reserved {
    pub lun_mapper_addr: regpair,
    pub reserved0: [u8; 8],
}

// rdif conetxt for dif on immediate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dif_on_immediate_params {
    pub initial_ref_tag: __le32,
    pub application_tag: __le16,
    pub application_tag_mask: __le16,
    pub flags1: __le16,
pub const DIF_ON_IMMEDIATE_PARAMS_VALIDATE_GUARD_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_VALIDATE_GUARD_SHIFT: c_int = 0;
pub const DIF_ON_IMMEDIATE_PARAMS_VALIDATE_APP_TAG_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_VALIDATE_APP_TAG_SHIFT: c_int = 1;
pub const DIF_ON_IMMEDIATE_PARAMS_VALIDATE_REF_TAG_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_VALIDATE_REF_TAG_SHIFT: c_int = 2;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_GUARD_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_GUARD_SHIFT: c_int = 3;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_APP_TAG_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_APP_TAG_SHIFT: c_int = 4;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_REF_TAG_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_REF_TAG_SHIFT: c_int = 5;
pub const DIF_ON_IMMEDIATE_PARAMS_INTERVAL_SIZE_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_INTERVAL_SIZE_SHIFT: c_int = 6;
pub const DIF_ON_IMMEDIATE_PARAMS_NETWORK_INTERFACE_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_NETWORK_INTERFACE_SHIFT: c_int = 7;
pub const DIF_ON_IMMEDIATE_PARAMS_HOST_INTERFACE_MASK: c_uint = 0x3;
pub const DIF_ON_IMMEDIATE_PARAMS_HOST_INTERFACE_SHIFT: c_int = 8;
pub const DIF_ON_IMMEDIATE_PARAMS_REF_TAG_MASK_MASK: c_uint = 0xF;
pub const DIF_ON_IMMEDIATE_PARAMS_REF_TAG_MASK_SHIFT: c_int = 10;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_APP_TAG_WITH_MASK_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_APP_TAG_WITH_MASK_SHIFT: c_int = 14;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_REF_TAG_WITH_MASK_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_FORWARD_REF_TAG_WITH_MASK_SHIFT: c_int = 15;
    pub flags0: u8,
pub const DIF_ON_IMMEDIATE_PARAMS_RESERVED_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_RESERVED_SHIFT: c_int = 0;
pub const DIF_ON_IMMEDIATE_PARAMS_IGNORE_APP_TAG_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_IGNORE_APP_TAG_SHIFT: c_int = 1;
pub const DIF_ON_IMMEDIATE_PARAMS_INITIAL_REF_TAG_IS_VALID_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_INITIAL_REF_TAG_IS_VALID_SHIFT: c_int = 2;
pub const DIF_ON_IMMEDIATE_PARAMS_HOST_GUARD_TYPE_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_HOST_GUARD_TYPE_SHIFT: c_int = 3;
pub const DIF_ON_IMMEDIATE_PARAMS_PROTECTION_TYPE_MASK: c_uint = 0x3;
pub const DIF_ON_IMMEDIATE_PARAMS_PROTECTION_TYPE_SHIFT: c_int = 4;
pub const DIF_ON_IMMEDIATE_PARAMS_CRC_SEED_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_CRC_SEED_SHIFT: c_int = 6;
pub const DIF_ON_IMMEDIATE_PARAMS_KEEP_REF_TAG_CONST_MASK: c_uint = 0x1;
pub const DIF_ON_IMMEDIATE_PARAMS_KEEP_REF_TAG_CONST_SHIFT: c_int = 7;
    pub reserved_zero: [u8; 5],
}

// iSCSI dif on immediate mode attributes union
#[repr(C)]
#[derive(Copy, Clone)]
pub union dif_configuration_params {
    pub lun_mapper_address: lun_mapper_addr_reserved,
    pub def_dif_conf: dif_on_immediate_params,
}

// Union of data/r2t sequence number
#[repr(C)]
#[derive(Copy, Clone)]
pub union iscsi_seq_num {
    pub data_sn: __le16,
    pub r2t_sn: __le16,
}

// iSCSI DIF flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_dif_flags {
    pub flags: u8,
pub const ISCSI_DIF_FLAGS_PROT_INTERVAL_SIZE_LOG_MASK: c_uint = 0xF;
pub const ISCSI_DIF_FLAGS_PROT_INTERVAL_SIZE_LOG_SHIFT: c_int = 0;
pub const ISCSI_DIF_FLAGS_DIF_TO_PEER_MASK: c_uint = 0x1;
pub const ISCSI_DIF_FLAGS_DIF_TO_PEER_SHIFT: c_int = 4;
pub const ISCSI_DIF_FLAGS_HOST_INTERFACE_MASK: c_uint = 0x7;
pub const ISCSI_DIF_FLAGS_HOST_INTERFACE_SHIFT: c_int = 5;
}

// The iscsi storm task context of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_iscsi_task_state {
    pub data_desc: scsi_cached_sges,
    pub sgl_params: scsi_sgl_params,
    pub exp_r2t_sn: __le32,
    pub buffer_offset: __le32,
    pub seq_num: iscsi_seq_num,
    pub dif_flags: iscsi_dif_flags,
    pub flags: u8,
pub const YSTORM_ISCSI_TASK_STATE_LOCAL_COMP_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_STATE_LOCAL_COMP_SHIFT: c_int = 0;
pub const YSTORM_ISCSI_TASK_STATE_SLOW_IO_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_STATE_SLOW_IO_SHIFT: c_int = 1;
pub const YSTORM_ISCSI_TASK_STATE_SET_DIF_OFFSET_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_STATE_SET_DIF_OFFSET_SHIFT: c_int = 2;
pub const YSTORM_ISCSI_TASK_STATE_RESERVED0_MASK: c_uint = 0x1F;
pub const YSTORM_ISCSI_TASK_STATE_RESERVED0_SHIFT: c_int = 3;
}

// The iscsi storm task context of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_iscsi_task_rxmit_opt {
    pub fast_rxmit_sge_offset: __le32,
    pub scan_start_buffer_offset: __le32,
    pub fast_rxmit_buffer_offset: __le32,
    pub scan_start_sgl_index: u8,
    pub fast_rxmit_sgl_index: u8,
    pub reserved: __le16,
}

// iSCSI Common PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_common_hdr {
    pub hdr_status: u8,
    pub hdr_response: u8,
    pub hdr_flags: u8,
    pub hdr_first_byte: u8,
pub const ISCSI_COMMON_HDR_OPCODE_MASK: c_uint = 0x3F;
pub const ISCSI_COMMON_HDR_OPCODE_SHIFT: c_int = 0;
pub const ISCSI_COMMON_HDR_IMM_MASK: c_uint = 0x1;
pub const ISCSI_COMMON_HDR_IMM_SHIFT: c_int = 6;
pub const ISCSI_COMMON_HDR_RSRV_MASK: c_uint = 0x1;
pub const ISCSI_COMMON_HDR_RSRV_SHIFT: c_int = 7;
    pub hdr_second_dword: __le32,
pub const ISCSI_COMMON_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_COMMON_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_COMMON_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_COMMON_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub lun_reserved: regpair,
    pub itt: __le32,
    pub ttt: __le32,
    pub cmdstat_sn: __le32,
    pub exp_statcmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub data: [__le32; 3],
}

// iSCSI Command PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cmd_hdr {
    pub reserved1: __le16,
    pub flags_attr: u8,
pub const ISCSI_CMD_HDR_ATTR_MASK: c_uint = 0x7;
pub const ISCSI_CMD_HDR_ATTR_SHIFT: c_int = 0;
pub const ISCSI_CMD_HDR_RSRV_MASK: c_uint = 0x3;
pub const ISCSI_CMD_HDR_RSRV_SHIFT: c_int = 3;
pub const ISCSI_CMD_HDR_WRITE_MASK: c_uint = 0x1;
pub const ISCSI_CMD_HDR_WRITE_SHIFT: c_int = 5;
pub const ISCSI_CMD_HDR_READ_MASK: c_uint = 0x1;
pub const ISCSI_CMD_HDR_READ_SHIFT: c_int = 6;
pub const ISCSI_CMD_HDR_FINAL_MASK: c_uint = 0x1;
pub const ISCSI_CMD_HDR_FINAL_SHIFT: c_int = 7;
    pub hdr_first_byte: u8,
pub const ISCSI_CMD_HDR_OPCODE_MASK: c_uint = 0x3F;
pub const ISCSI_CMD_HDR_OPCODE_SHIFT: c_int = 0;
pub const ISCSI_CMD_HDR_IMM_MASK: c_uint = 0x1;
pub const ISCSI_CMD_HDR_IMM_SHIFT: c_int = 6;
pub const ISCSI_CMD_HDR_RSRV1_MASK: c_uint = 0x1;
pub const ISCSI_CMD_HDR_RSRV1_SHIFT: c_int = 7;
    pub hdr_second_dword: __le32,
pub const ISCSI_CMD_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_CMD_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_CMD_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_CMD_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: __le32,
    pub expected_transfer_length: __le32,
    pub cmd_sn: __le32,
    pub exp_stat_sn: __le32,
    pub cdb: [__le32; 4],
}

// iSCSI Command PDU header with Extended CDB (Initiator Mode)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_ext_cdb_cmd_hdr {
    pub reserved1: __le16,
    pub flags_attr: u8,
pub const ISCSI_EXT_CDB_CMD_HDR_ATTR_MASK: c_uint = 0x7;
pub const ISCSI_EXT_CDB_CMD_HDR_ATTR_SHIFT: c_int = 0;
pub const ISCSI_EXT_CDB_CMD_HDR_RSRV_MASK: c_uint = 0x3;
pub const ISCSI_EXT_CDB_CMD_HDR_RSRV_SHIFT: c_int = 3;
pub const ISCSI_EXT_CDB_CMD_HDR_WRITE_MASK: c_uint = 0x1;
pub const ISCSI_EXT_CDB_CMD_HDR_WRITE_SHIFT: c_int = 5;
pub const ISCSI_EXT_CDB_CMD_HDR_READ_MASK: c_uint = 0x1;
pub const ISCSI_EXT_CDB_CMD_HDR_READ_SHIFT: c_int = 6;
pub const ISCSI_EXT_CDB_CMD_HDR_FINAL_MASK: c_uint = 0x1;
pub const ISCSI_EXT_CDB_CMD_HDR_FINAL_SHIFT: c_int = 7;
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_EXT_CDB_CMD_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_EXT_CDB_CMD_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_EXT_CDB_CMD_HDR_CDB_SIZE_MASK: c_uint = 0xFF;
pub const ISCSI_EXT_CDB_CMD_HDR_CDB_SIZE_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: __le32,
    pub expected_transfer_length: __le32,
    pub cmd_sn: __le32,
    pub exp_stat_sn: __le32,
    pub cdb_sge: scsi_sge,
}

// iSCSI login request PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_login_req_hdr {
    pub version_min: u8,
    pub version_max: u8,
    pub flags_attr: u8,
pub const ISCSI_LOGIN_REQ_HDR_NSG_MASK: c_uint = 0x3;
pub const ISCSI_LOGIN_REQ_HDR_NSG_SHIFT: c_int = 0;
pub const ISCSI_LOGIN_REQ_HDR_CSG_MASK: c_uint = 0x3;
pub const ISCSI_LOGIN_REQ_HDR_CSG_SHIFT: c_int = 2;
pub const ISCSI_LOGIN_REQ_HDR_RSRV_MASK: c_uint = 0x3;
pub const ISCSI_LOGIN_REQ_HDR_RSRV_SHIFT: c_int = 4;
pub const ISCSI_LOGIN_REQ_HDR_C_MASK: c_uint = 0x1;
pub const ISCSI_LOGIN_REQ_HDR_C_SHIFT: c_int = 6;
pub const ISCSI_LOGIN_REQ_HDR_T_MASK: c_uint = 0x1;
pub const ISCSI_LOGIN_REQ_HDR_T_SHIFT: c_int = 7;
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_LOGIN_REQ_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_LOGIN_REQ_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_LOGIN_REQ_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_LOGIN_REQ_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub isid_tabc: __le32,
    pub tsih: __le16,
    pub isid_d: __le16,
    pub itt: __le32,
    pub reserved1: __le16,
    pub cid: __le16,
    pub cmd_sn: __le32,
    pub exp_stat_sn: __le32,
    pub reserved2: [__le32; 4],
}

// iSCSI logout request PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_logout_req_hdr {
    pub reserved0: __le16,
    pub reason_code: u8,
    pub opcode: u8,
    pub reserved1: __le32,
    pub reserved2: [__le32; 2],
    pub itt: __le32,
    pub reserved3: __le16,
    pub cid: __le16,
    pub cmd_sn: __le32,
    pub exp_stat_sn: __le32,
    pub reserved4: [__le32; 4],
}

// iSCSI Data-out PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_data_out_hdr {
    pub reserved1: __le16,
    pub flags_attr: u8,
pub const ISCSI_DATA_OUT_HDR_RSRV_MASK: c_uint = 0x7F;
pub const ISCSI_DATA_OUT_HDR_RSRV_SHIFT: c_int = 0;
pub const ISCSI_DATA_OUT_HDR_FINAL_MASK: c_uint = 0x1;
pub const ISCSI_DATA_OUT_HDR_FINAL_SHIFT: c_int = 7;
    pub opcode: u8,
    pub reserved2: __le32,
    pub lun: regpair,
    pub itt: __le32,
    pub ttt: __le32,
    pub reserved3: __le32,
    pub exp_stat_sn: __le32,
    pub reserved4: __le32,
    pub data_sn: __le32,
    pub buffer_offset: __le32,
    pub reserved5: __le32,
}

// iSCSI Data-in PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_data_in_hdr {
    pub status_rsvd: u8,
    pub reserved1: u8,
    pub flags: u8,
pub const ISCSI_DATA_IN_HDR_STATUS_MASK: c_uint = 0x1;
pub const ISCSI_DATA_IN_HDR_STATUS_SHIFT: c_int = 0;
pub const ISCSI_DATA_IN_HDR_UNDERFLOW_MASK: c_uint = 0x1;
pub const ISCSI_DATA_IN_HDR_UNDERFLOW_SHIFT: c_int = 1;
pub const ISCSI_DATA_IN_HDR_OVERFLOW_MASK: c_uint = 0x1;
pub const ISCSI_DATA_IN_HDR_OVERFLOW_SHIFT: c_int = 2;
pub const ISCSI_DATA_IN_HDR_RSRV_MASK: c_uint = 0x7;
pub const ISCSI_DATA_IN_HDR_RSRV_SHIFT: c_int = 3;
pub const ISCSI_DATA_IN_HDR_ACK_MASK: c_uint = 0x1;
pub const ISCSI_DATA_IN_HDR_ACK_SHIFT: c_int = 6;
pub const ISCSI_DATA_IN_HDR_FINAL_MASK: c_uint = 0x1;
pub const ISCSI_DATA_IN_HDR_FINAL_SHIFT: c_int = 7;
    pub opcode: u8,
    pub reserved2: __le32,
    pub lun: regpair,
    pub itt: __le32,
    pub ttt: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub data_sn: __le32,
    pub buffer_offset: __le32,
    pub residual_count: __le32,
}

// iSCSI R2T PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_r2t_hdr {
    pub reserved0: [u8; 3],
    pub opcode: u8,
    pub reserved2: __le32,
    pub lun: regpair,
    pub itt: __le32,
    pub ttt: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub r2t_sn: __le32,
    pub buffer_offset: __le32,
    pub desired_data_trns_len: __le32,
}

// iSCSI NOP-out PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_nop_out_hdr {
    pub reserved1: __le16,
    pub flags_attr: u8,
pub const ISCSI_NOP_OUT_HDR_RSRV_MASK: c_uint = 0x7F;
pub const ISCSI_NOP_OUT_HDR_RSRV_SHIFT: c_int = 0;
pub const ISCSI_NOP_OUT_HDR_CONST1_MASK: c_uint = 0x1;
pub const ISCSI_NOP_OUT_HDR_CONST1_SHIFT: c_int = 7;
    pub opcode: u8,
    pub reserved2: __le32,
    pub lun: regpair,
    pub itt: __le32,
    pub ttt: __le32,
    pub cmd_sn: __le32,
    pub exp_stat_sn: __le32,
    pub reserved3: __le32,
    pub reserved4: __le32,
    pub reserved5: __le32,
    pub reserved6: __le32,
}

// iSCSI NOP-in PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_nop_in_hdr {
    pub reserved0: __le16,
    pub flags_attr: u8,
pub const ISCSI_NOP_IN_HDR_RSRV_MASK: c_uint = 0x7F;
pub const ISCSI_NOP_IN_HDR_RSRV_SHIFT: c_int = 0;
pub const ISCSI_NOP_IN_HDR_CONST1_MASK: c_uint = 0x1;
pub const ISCSI_NOP_IN_HDR_CONST1_SHIFT: c_int = 7;
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_NOP_IN_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_NOP_IN_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_NOP_IN_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_NOP_IN_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: __le32,
    pub ttt: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub reserved5: __le32,
    pub reserved6: __le32,
    pub reserved7: __le32,
}

// iSCSI Login Response PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_login_response_hdr {
    pub version_active: u8,
    pub version_max: u8,
    pub flags_attr: u8,
pub const ISCSI_LOGIN_RESPONSE_HDR_NSG_MASK: c_uint = 0x3;
pub const ISCSI_LOGIN_RESPONSE_HDR_NSG_SHIFT: c_int = 0;
pub const ISCSI_LOGIN_RESPONSE_HDR_CSG_MASK: c_uint = 0x3;
pub const ISCSI_LOGIN_RESPONSE_HDR_CSG_SHIFT: c_int = 2;
pub const ISCSI_LOGIN_RESPONSE_HDR_RSRV_MASK: c_uint = 0x3;
pub const ISCSI_LOGIN_RESPONSE_HDR_RSRV_SHIFT: c_int = 4;
pub const ISCSI_LOGIN_RESPONSE_HDR_C_MASK: c_uint = 0x1;
pub const ISCSI_LOGIN_RESPONSE_HDR_C_SHIFT: c_int = 6;
pub const ISCSI_LOGIN_RESPONSE_HDR_T_MASK: c_uint = 0x1;
pub const ISCSI_LOGIN_RESPONSE_HDR_T_SHIFT: c_int = 7;
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_LOGIN_RESPONSE_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_LOGIN_RESPONSE_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_LOGIN_RESPONSE_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_LOGIN_RESPONSE_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub isid_tabc: __le32,
    pub tsih: __le16,
    pub isid_d: __le16,
    pub itt: __le32,
    pub reserved1: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub reserved2: __le16,
    pub status_detail: u8,
    pub status_class: u8,
    pub reserved4: [__le32; 2],
}

// iSCSI Logout Response PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_logout_response_hdr {
    pub reserved1: u8,
    pub response: u8,
    pub flags: u8,
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_LOGOUT_RESPONSE_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_LOGOUT_RESPONSE_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_LOGOUT_RESPONSE_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_LOGOUT_RESPONSE_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub reserved2: [__le32; 2],
    pub itt: __le32,
    pub reserved3: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub reserved4: __le32,
    pub time_2_retain: __le16,
    pub time_2_wait: __le16,
    pub reserved5: [__le32; 1],
}

// iSCSI Text Request PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_text_request_hdr {
    pub reserved0: __le16,
    pub flags_attr: u8,
pub const ISCSI_TEXT_REQUEST_HDR_RSRV_MASK: c_uint = 0x3F;
pub const ISCSI_TEXT_REQUEST_HDR_RSRV_SHIFT: c_int = 0;
pub const ISCSI_TEXT_REQUEST_HDR_C_MASK: c_uint = 0x1;
pub const ISCSI_TEXT_REQUEST_HDR_C_SHIFT: c_int = 6;
pub const ISCSI_TEXT_REQUEST_HDR_F_MASK: c_uint = 0x1;
pub const ISCSI_TEXT_REQUEST_HDR_F_SHIFT: c_int = 7;
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_TEXT_REQUEST_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_TEXT_REQUEST_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_TEXT_REQUEST_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_TEXT_REQUEST_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: __le32,
    pub ttt: __le32,
    pub cmd_sn: __le32,
    pub exp_stat_sn: __le32,
    pub reserved4: [__le32; 4],
}

// iSCSI Text Response PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_text_response_hdr {
    pub reserved1: __le16,
    pub flags: u8,
pub const ISCSI_TEXT_RESPONSE_HDR_RSRV_MASK: c_uint = 0x3F;
pub const ISCSI_TEXT_RESPONSE_HDR_RSRV_SHIFT: c_int = 0;
pub const ISCSI_TEXT_RESPONSE_HDR_C_MASK: c_uint = 0x1;
pub const ISCSI_TEXT_RESPONSE_HDR_C_SHIFT: c_int = 6;
pub const ISCSI_TEXT_RESPONSE_HDR_F_MASK: c_uint = 0x1;
pub const ISCSI_TEXT_RESPONSE_HDR_F_SHIFT: c_int = 7;
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_TEXT_RESPONSE_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_TEXT_RESPONSE_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_TEXT_RESPONSE_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_TEXT_RESPONSE_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: __le32,
    pub ttt: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub reserved4: [__le32; 3],
}

// iSCSI TMF Request PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tmf_request_hdr {
    pub reserved0: __le16,
    pub function: u8,
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_TMF_REQUEST_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_TMF_REQUEST_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_TMF_REQUEST_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_TMF_REQUEST_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: __le32,
    pub rtt: __le32,
    pub cmd_sn: __le32,
    pub exp_stat_sn: __le32,
    pub ref_cmd_sn: __le32,
    pub exp_data_sn: __le32,
    pub reserved4: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tmf_response_hdr {
    pub reserved2: u8,
    pub hdr_response: u8,
    pub hdr_flags: u8,
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_TMF_RESPONSE_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_TMF_RESPONSE_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_TMF_RESPONSE_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_TMF_RESPONSE_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub reserved0: regpair,
    pub itt: __le32,
    pub reserved1: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub reserved4: [__le32; 3],
}

// iSCSI Response PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_response_hdr {
    pub hdr_status: u8,
    pub hdr_response: u8,
    pub hdr_flags: u8,
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_RESPONSE_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_RESPONSE_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_RESPONSE_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_RESPONSE_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: __le32,
    pub snack_tag: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub exp_data_sn: __le32,
    pub bi_residual_count: __le32,
    pub residual_count: __le32,
}

// iSCSI Reject PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_reject_hdr {
    pub reserved4: u8,
    pub hdr_reason: u8,
    pub hdr_flags: u8,
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_REJECT_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_REJECT_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_REJECT_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_REJECT_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub reserved0: regpair,
    pub all_ones: __le32,
    pub reserved2: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub data_sn: __le32,
    pub reserved3: [__le32; 2],
}

// iSCSI Asynchronous Message PDU header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_async_msg_hdr {
    pub reserved0: __le16,
    pub flags_attr: u8,
pub const ISCSI_ASYNC_MSG_HDR_RSRV_MASK: c_uint = 0x7F;
pub const ISCSI_ASYNC_MSG_HDR_RSRV_SHIFT: c_int = 0;
pub const ISCSI_ASYNC_MSG_HDR_CONST1_MASK: c_uint = 0x1;
pub const ISCSI_ASYNC_MSG_HDR_CONST1_SHIFT: c_int = 7;
    pub opcode: u8,
    pub hdr_second_dword: __le32,
pub const ISCSI_ASYNC_MSG_HDR_DATA_SEG_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_ASYNC_MSG_HDR_DATA_SEG_LEN_SHIFT: c_int = 0;
pub const ISCSI_ASYNC_MSG_HDR_TOTAL_AHS_LEN_MASK: c_uint = 0xFF;
pub const ISCSI_ASYNC_MSG_HDR_TOTAL_AHS_LEN_SHIFT: c_int = 24;
    pub lun: regpair,
    pub all_ones: __le32,
    pub reserved1: __le32,
    pub stat_sn: __le32,
    pub exp_cmd_sn: __le32,
    pub max_cmd_sn: __le32,
    pub param1_rsrv: __le16,
    pub async_vcode: u8,
    pub async_event: u8,
    pub param3_rsrv: __le16,
    pub param2_rsrv: __le16,
    pub reserved7: __le32,
}

// PDU header part of Ystorm task context
#[repr(C)]
#[derive(Copy, Clone)]
pub union iscsi_task_hdr {
    pub common: iscsi_common_hdr,
    pub data: data_hdr,
    pub cmd: iscsi_cmd_hdr,
    pub ext_cdb_cmd: iscsi_ext_cdb_cmd_hdr,
    pub login_req: iscsi_login_req_hdr,
    pub logout_req: iscsi_logout_req_hdr,
    pub data_out: iscsi_data_out_hdr,
    pub data_in: iscsi_data_in_hdr,
    pub r2t: iscsi_r2t_hdr,
    pub nop_out: iscsi_nop_out_hdr,
    pub nop_in: iscsi_nop_in_hdr,
    pub login_response: iscsi_login_response_hdr,
    pub logout_response: iscsi_logout_response_hdr,
    pub text_request: iscsi_text_request_hdr,
    pub text_response: iscsi_text_response_hdr,
    pub tmf_request: iscsi_tmf_request_hdr,
    pub tmf_response: iscsi_tmf_response_hdr,
    pub response: iscsi_response_hdr,
    pub reject: iscsi_reject_hdr,
    pub async_msg: iscsi_async_msg_hdr,
}

// The iscsi storm task context of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_iscsi_task_st_ctx {
    pub state: ystorm_iscsi_task_state,
    pub rxmit_opt: ystorm_iscsi_task_rxmit_opt,
    pub pdu_hdr: iscsi_task_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_iscsi_task_ag_ctx {
    pub reserved: u8,
    pub byte1: u8,
    pub word0: __le16,
    pub flags0: u8,
pub const YSTORM_ISCSI_TASK_AG_CTX_NIBBLE0_MASK: c_uint = 0xF;
pub const YSTORM_ISCSI_TASK_AG_CTX_NIBBLE0_SHIFT: c_int = 0;
pub const YSTORM_ISCSI_TASK_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_BIT0_SHIFT: c_int = 4;
pub const YSTORM_ISCSI_TASK_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_BIT1_SHIFT: c_int = 5;
pub const YSTORM_ISCSI_TASK_AG_CTX_VALID_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_VALID_SHIFT: c_int = 6;
pub const YSTORM_ISCSI_TASK_AG_CTX_TTT_VALID_MASK: c_uint = 0x1	/* bit3 */;
pub const YSTORM_ISCSI_TASK_AG_CTX_TTT_VALID_SHIFT: c_int = 7;
    pub flags1: u8,
pub const YSTORM_ISCSI_TASK_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_ISCSI_TASK_AG_CTX_CF0_SHIFT: c_int = 0;
pub const YSTORM_ISCSI_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_ISCSI_TASK_AG_CTX_CF1_SHIFT: c_int = 2;
pub const YSTORM_ISCSI_TASK_AG_CTX_CF2SPECIAL_MASK: c_uint = 0x3;
pub const YSTORM_ISCSI_TASK_AG_CTX_CF2SPECIAL_SHIFT: c_int = 4;
pub const YSTORM_ISCSI_TASK_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const YSTORM_ISCSI_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags2: u8,
pub const YSTORM_ISCSI_TASK_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_BIT4_SHIFT: c_int = 0;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 1;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 2;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 3;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 4;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 5;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 6;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_TASK_AG_CTX_RULE6EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub TTT: __le32,
    pub byte3: u8,
    pub byte4: u8,
    pub word1: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_iscsi_task_ag_ctx {
    pub cdu_validation: u8,
    pub byte1: u8,
    pub task_cid: __le16,
    pub flags0: u8,
pub const MSTORM_ISCSI_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const MSTORM_ISCSI_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const MSTORM_ISCSI_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const MSTORM_ISCSI_TASK_AG_CTX_CONN_CLEAR_SQ_FLAG_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_CONN_CLEAR_SQ_FLAG_SHIFT: c_int = 5;
pub const MSTORM_ISCSI_TASK_AG_CTX_VALID_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_VALID_SHIFT: c_int = 6;
pub const MSTORM_ISCSI_TASK_AG_CTX_TASK_CLEANUP_FLAG_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_TASK_CLEANUP_FLAG_SHIFT: c_int = 7;
    pub flags1: u8,
pub const MSTORM_ISCSI_TASK_AG_CTX_TASK_CLEANUP_CF_MASK: c_uint = 0x3;
pub const MSTORM_ISCSI_TASK_AG_CTX_TASK_CLEANUP_CF_SHIFT: c_int = 0;
pub const MSTORM_ISCSI_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_ISCSI_TASK_AG_CTX_CF1_SHIFT: c_int = 2;
pub const MSTORM_ISCSI_TASK_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_ISCSI_TASK_AG_CTX_CF2_SHIFT: c_int = 4;
pub const MSTORM_ISCSI_TASK_AG_CTX_TASK_CLEANUP_CF_EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_TASK_CLEANUP_CF_EN_SHIFT: c_int = 6;
pub const MSTORM_ISCSI_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags2: u8,
pub const MSTORM_ISCSI_TASK_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 1;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 2;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 3;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 4;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 5;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 6;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_TASK_AG_CTX_RULE6EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub reg0: __le32,
    pub byte3: u8,
    pub byte4: u8,
    pub word1: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_task_ag_ctx {
    pub reserved: u8,
    pub state: u8,
    pub icid: __le16,
    pub flags0: u8,
pub const USTORM_ISCSI_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const USTORM_ISCSI_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const USTORM_ISCSI_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const USTORM_ISCSI_TASK_AG_CTX_CONN_CLEAR_SQ_FLAG_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_CONN_CLEAR_SQ_FLAG_SHIFT: c_int = 5;
pub const USTORM_ISCSI_TASK_AG_CTX_HQ_SCANNED_CF_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_TASK_AG_CTX_HQ_SCANNED_CF_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_ISCSI_TASK_AG_CTX_RESERVED1_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_TASK_AG_CTX_RESERVED1_SHIFT: c_int = 0;
pub const USTORM_ISCSI_TASK_AG_CTX_R2T2RECV_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_TASK_AG_CTX_R2T2RECV_SHIFT: c_int = 2;
pub const USTORM_ISCSI_TASK_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_TASK_AG_CTX_CF3_SHIFT: c_int = 4;
pub const USTORM_ISCSI_TASK_AG_CTX_DIF_ERROR_CF_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_TASK_AG_CTX_DIF_ERROR_CF_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_ISCSI_TASK_AG_CTX_HQ_SCANNED_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_HQ_SCANNED_CF_EN_SHIFT: c_int = 0;
pub const USTORM_ISCSI_TASK_AG_CTX_DISABLE_DATA_ACKED_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_DISABLE_DATA_ACKED_SHIFT: c_int = 1;
pub const USTORM_ISCSI_TASK_AG_CTX_R2T2RECV_EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_R2T2RECV_EN_SHIFT: c_int = 2;
pub const USTORM_ISCSI_TASK_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_ISCSI_TASK_AG_CTX_DIF_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_DIF_ERROR_CF_EN_SHIFT: c_int = 4;
pub const USTORM_ISCSI_TASK_AG_CTX_CMP_DATA_TOTAL_EXP_EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_CMP_DATA_TOTAL_EXP_EN_SHIFT: c_int = 5;
pub const USTORM_ISCSI_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 6;
pub const USTORM_ISCSI_TASK_AG_CTX_CMP_CONT_RCV_EXP_EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_CMP_CONT_RCV_EXP_EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_ISCSI_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 0;
pub const USTORM_ISCSI_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 1;
pub const USTORM_ISCSI_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 2;
pub const USTORM_ISCSI_TASK_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_AG_CTX_RULE6EN_SHIFT: c_int = 3;
pub const USTORM_ISCSI_TASK_AG_CTX_DIF_ERROR_TYPE_MASK: c_uint = 0xF;
pub const USTORM_ISCSI_TASK_AG_CTX_DIF_ERROR_TYPE_SHIFT: c_int = 4;
    pub dif_err_intervals: __le32,
    pub dif_error_1st_interval: __le32,
    pub rcv_cont_len: __le32,
    pub exp_cont_len: __le32,
    pub total_data_acked: __le32,
    pub exp_data_acked: __le32,
    pub byte2: u8,
    pub byte3: u8,
    pub word1: __le16,
    pub next_tid: __le16,
    pub word3: __le16,
    pub hdr_residual_count: __le32,
    pub exp_r2t_sn: __le32,
}

// The iscsi storm task context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_iscsi_task_st_ctx {
    pub data_desc: scsi_cached_sges,
    pub sgl_params: scsi_sgl_params,
    pub rem_task_size: __le32,
    pub data_buffer_offset: __le32,
    pub task_type: u8,
    pub dif_flags: iscsi_dif_flags,
    pub dif_task_icid: __le16,
    pub sense_db: regpair,
    pub expected_itt: __le32,
    pub reserved1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_reg1 {
    pub reg1_map: __le32,
pub const ISCSI_REG1_NUM_SGES_MASK: c_uint = 0xF;
pub const ISCSI_REG1_NUM_SGES_SHIFT: c_int = 0;
pub const ISCSI_REG1_RESERVED1_MASK: c_uint = 0xFFFFFFF;
pub const ISCSI_REG1_RESERVED1_SHIFT: c_int = 4;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tqe_opaque {
    pub opaque: [__le16; 2],
}

// The iscsi storm task context of Ustorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_task_st_ctx {
    pub rem_rcv_len: __le32,
    pub exp_data_transfer_len: __le32,
    pub exp_data_sn: __le32,
    pub lun: regpair,
    pub reg1: iscsi_reg1,
    pub flags2: u8,
pub const USTORM_ISCSI_TASK_ST_CTX_AHS_EXIST_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_AHS_EXIST_SHIFT: c_int = 0;
pub const USTORM_ISCSI_TASK_ST_CTX_RESERVED1_MASK: c_uint = 0x7F;
pub const USTORM_ISCSI_TASK_ST_CTX_RESERVED1_SHIFT: c_int = 1;
    pub dif_flags: iscsi_dif_flags,
    pub reserved3: __le16,
    pub tqe_opaque_list: tqe_opaque,
    pub reserved5: __le32,
    pub reserved6: __le32,
    pub reserved7: __le32,
    pub task_type: u8,
    pub error_flags: u8,
pub const USTORM_ISCSI_TASK_ST_CTX_DATA_DIGEST_ERROR_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_DATA_DIGEST_ERROR_SHIFT: c_int = 0;
pub const USTORM_ISCSI_TASK_ST_CTX_DATA_TRUNCATED_ERROR_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_DATA_TRUNCATED_ERROR_SHIFT: c_int = 1;
pub const USTORM_ISCSI_TASK_ST_CTX_UNDER_RUN_ERROR_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_UNDER_RUN_ERROR_SHIFT: c_int = 2;
pub const USTORM_ISCSI_TASK_ST_CTX_RESERVED8_MASK: c_uint = 0x1F;
pub const USTORM_ISCSI_TASK_ST_CTX_RESERVED8_SHIFT: c_int = 3;
    pub flags: u8,
pub const USTORM_ISCSI_TASK_ST_CTX_CQE_WRITE_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_TASK_ST_CTX_CQE_WRITE_SHIFT: c_int = 0;
pub const USTORM_ISCSI_TASK_ST_CTX_LOCAL_COMP_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_LOCAL_COMP_SHIFT: c_int = 2;
pub const USTORM_ISCSI_TASK_ST_CTX_Q0_R2TQE_WRITE_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_Q0_R2TQE_WRITE_SHIFT: c_int = 3;
pub const USTORM_ISCSI_TASK_ST_CTX_TOTAL_DATA_ACKED_DONE_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_TOTAL_DATA_ACKED_DONE_SHIFT: c_int = 4;
pub const USTORM_ISCSI_TASK_ST_CTX_HQ_SCANNED_DONE_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_HQ_SCANNED_DONE_SHIFT: c_int = 5;
pub const USTORM_ISCSI_TASK_ST_CTX_R2T2RECV_DONE_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_R2T2RECV_DONE_SHIFT: c_int = 6;
pub const USTORM_ISCSI_TASK_ST_CTX_RESERVED0_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_TASK_ST_CTX_RESERVED0_SHIFT: c_int = 7;
    pub cq_rss_number: u8,
}

// iscsi task context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context {
    pub ystorm_st_context: ystorm_iscsi_task_st_ctx,
    pub ystorm_ag_context: ystorm_iscsi_task_ag_ctx,
    pub ystorm_ag_padding: [regpair; 2],
    pub tdif_context: tdif_task_context,
    pub mstorm_ag_context: mstorm_iscsi_task_ag_ctx,
    pub mstorm_ag_padding: [regpair; 2],
    pub ustorm_ag_context: ustorm_iscsi_task_ag_ctx,
    pub mstorm_st_context: mstorm_iscsi_task_st_ctx,
    pub ustorm_st_context: ustorm_iscsi_task_st_ctx,
    pub rdif_context: rdif_task_context,
}

// iSCSI connection offload params passed by driver to FW in ISCSI offload
// ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_conn_offload_params {
    pub sq_pbl_addr: regpair,
    pub r2tq_pbl_addr: regpair,
    pub xhq_pbl_addr: regpair,
    pub uhq_pbl_addr: regpair,
    pub physical_q0: __le16,
    pub physical_q1: __le16,
    pub flags: u8,
pub const ISCSI_CONN_OFFLOAD_PARAMS_TCP_ON_CHIP_1B_MASK: c_uint = 0x1;
pub const ISCSI_CONN_OFFLOAD_PARAMS_TCP_ON_CHIP_1B_SHIFT: c_int = 0;
pub const ISCSI_CONN_OFFLOAD_PARAMS_TARGET_MODE_MASK: c_uint = 0x1;
pub const ISCSI_CONN_OFFLOAD_PARAMS_TARGET_MODE_SHIFT: c_int = 1;
pub const ISCSI_CONN_OFFLOAD_PARAMS_RESTRICTED_MODE_MASK: c_uint = 0x1;
pub const ISCSI_CONN_OFFLOAD_PARAMS_RESTRICTED_MODE_SHIFT: c_int = 2;
pub const ISCSI_CONN_OFFLOAD_PARAMS_RESERVED1_MASK: c_uint = 0x1F;
pub const ISCSI_CONN_OFFLOAD_PARAMS_RESERVED1_SHIFT: c_int = 3;
    pub default_cq: u8,
    pub reserved0: __le16,
    pub stat_sn: __le32,
    pub initial_ack: __le32,
}

// iSCSI connection statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_conn_stats_params {
    pub iscsi_tcp_tx_packets_cnt: regpair,
    pub iscsi_tcp_tx_bytes_cnt: regpair,
    pub iscsi_tcp_tx_rxmit_cnt: regpair,
    pub iscsi_tcp_rx_packets_cnt: regpair,
    pub iscsi_tcp_rx_bytes_cnt: regpair,
    pub iscsi_tcp_rx_dup_ack_cnt: regpair,
    pub iscsi_tcp_rx_chksum_err_cnt: __le32,
    pub reserved: __le32,
}

// iSCSI connection update params passed by driver to FW in ISCSI update
// ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_conn_update_ramrod_params {
    pub reserved0: __le16,
    pub conn_id: __le16,
    pub reserved1: __le32,
    pub flags: u8,
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_HD_EN_MASK: c_uint = 0x1;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_HD_EN_SHIFT: c_int = 0;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_DD_EN_MASK: c_uint = 0x1;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_DD_EN_SHIFT: c_int = 1;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_INITIAL_R2T_MASK: c_uint = 0x1;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_INITIAL_R2T_SHIFT: c_int = 2;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_IMMEDIATE_DATA_MASK: c_uint = 0x1;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_IMMEDIATE_DATA_SHIFT: c_int = 3;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_DIF_BLOCK_SIZE_MASK: c_uint = 0x1;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_DIF_BLOCK_SIZE_SHIFT: c_int = 4;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_DIF_ON_HOST_EN_MASK: c_uint = 0x1;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_DIF_ON_HOST_EN_SHIFT: c_int = 5;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_DIF_ON_IMM_EN_MASK: c_uint = 0x1;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_DIF_ON_IMM_EN_SHIFT: c_int = 6;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_LUN_MAPPER_EN_MASK: c_uint = 0x1;
pub const ISCSI_CONN_UPDATE_RAMROD_PARAMS_LUN_MAPPER_EN_SHIFT: c_int = 7;
    pub reserved3: [u8; 3],
    pub max_seq_size: __le32,
    pub max_send_pdu_length: __le32,
    pub max_recv_pdu_length: __le32,
    pub first_seq_length: __le32,
    pub exp_stat_sn: __le32,
    pub dif_on_imme_params: dif_configuration_params,
}

// iSCSI CQ element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cqe_common {
    pub conn_id: __le16,
    pub cqe_type: u8,
    pub error_bitmap: cqe_error_status,
    pub reserved: [__le32; 3],
    pub iscsi_hdr: iscsi_task_hdr,
}

// iSCSI CQ element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cqe_solicited {
    pub conn_id: __le16,
    pub cqe_type: u8,
    pub error_bitmap: cqe_error_status,
    pub itid: __le16,
    pub task_type: u8,
    pub fw_dbg_field: u8,
    pub caused_conn_err: u8,
    pub reserved0: [u8; 3],
    pub data_truncated_bytes: __le32,
    pub iscsi_hdr: iscsi_task_hdr,
}

// iSCSI CQ element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cqe_unsolicited {
    pub conn_id: __le16,
    pub cqe_type: u8,
    pub error_bitmap: cqe_error_status,
    pub reserved0: __le16,
    pub reserved1: u8,
    pub unsol_cqe_type: u8,
    pub rqe_opaque: __le16,
    pub reserved2: [__le16; 3],
    pub iscsi_hdr: iscsi_task_hdr,
}

// iSCSI CQ element
#[repr(C)]
#[derive(Copy, Clone)]
pub union iscsi_cqe {
    pub cqe_common: iscsi_cqe_common,
    pub cqe_solicited: iscsi_cqe_solicited,
    pub cqe_unsolicited: iscsi_cqe_unsolicited,
}

// iSCSI CQE type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_cqes_type {
    ISCSI_CQE_TYPE_SOLICITED = 1,
    ISCSI_CQE_TYPE_UNSOLICITED,
    ISCSI_CQE_TYPE_SOLICITED_WITH_SENSE,
    ISCSI_CQE_TYPE_TASK_CLEANUP,
    ISCSI_CQE_TYPE_DUMMY,
    MAX_ISCSI_CQES_TYPE
}

// iSCSI CQE type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_cqe_unsolicited_type {
    ISCSI_CQE_UNSOLICITED_NONE,
    ISCSI_CQE_UNSOLICITED_SINGLE,
    ISCSI_CQE_UNSOLICITED_FIRST,
    ISCSI_CQE_UNSOLICITED_MIDDLE,
    ISCSI_CQE_UNSOLICITED_LAST,
    MAX_ISCSI_CQE_UNSOLICITED_TYPE
}

// iscsi debug modes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_debug_modes {
    pub flags: u8,
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RX_CONN_ERROR_MASK: c_uint = 0x1;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RX_CONN_ERROR_SHIFT: c_int = 0;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_RESET_MASK: c_uint = 0x1;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_RESET_SHIFT: c_int = 1;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_FIN_MASK: c_uint = 0x1;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_FIN_SHIFT: c_int = 2;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_CLEANUP_MASK: c_uint = 0x1;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_CLEANUP_SHIFT: c_int = 3;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_REJECT_OR_ASYNC_MASK: c_uint = 0x1;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_REJECT_OR_ASYNC_SHIFT: c_int = 4;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_NOP_MASK: c_uint = 0x1;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_RECV_NOP_SHIFT: c_int = 5;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_DIF_OR_DATA_DIGEST_ERROR_MASK: c_uint = 0x1;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_DIF_OR_DATA_DIGEST_ERROR_SHIFT: c_int = 6;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_HQ_CORRUPT_MASK: c_uint = 0x1;
pub const ISCSI_DEBUG_MODES_ASSERT_IF_HQ_CORRUPT_SHIFT: c_int = 7;
}

// iSCSI kernel completion queue IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_eqe_opcode {
    ISCSI_EVENT_TYPE_INIT_FUNC = 0,
    ISCSI_EVENT_TYPE_DESTROY_FUNC,
    ISCSI_EVENT_TYPE_OFFLOAD_CONN,
    ISCSI_EVENT_TYPE_UPDATE_CONN,
    ISCSI_EVENT_TYPE_CLEAR_SQ,
    ISCSI_EVENT_TYPE_TERMINATE_CONN,
    ISCSI_EVENT_TYPE_MAC_UPDATE_CONN,
    ISCSI_EVENT_TYPE_COLLECT_STATS_CONN,
    ISCSI_EVENT_TYPE_ASYN_CONNECT_COMPLETE,
    ISCSI_EVENT_TYPE_ASYN_TERMINATE_DONE,
    ISCSI_EVENT_TYPE_START_OF_ERROR_TYPES = 10,
    ISCSI_EVENT_TYPE_ASYN_ABORT_RCVD,
    ISCSI_EVENT_TYPE_ASYN_CLOSE_RCVD,
    ISCSI_EVENT_TYPE_ASYN_SYN_RCVD,
    ISCSI_EVENT_TYPE_ASYN_MAX_RT_TIME,
    ISCSI_EVENT_TYPE_ASYN_MAX_RT_CNT,
    ISCSI_EVENT_TYPE_ASYN_MAX_KA_PROBES_CNT,
    ISCSI_EVENT_TYPE_ASYN_FIN_WAIT2,
    ISCSI_EVENT_TYPE_ISCSI_CONN_ERROR,
    ISCSI_EVENT_TYPE_TCP_CONN_ERROR,
    MAX_ISCSI_EQE_OPCODE
}

// iSCSI EQE and CQE completion status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_error_types {
    ISCSI_STATUS_NONE = 0,
    ISCSI_CQE_ERROR_UNSOLICITED_RCV_ON_INVALID_CONN = 1,
    ISCSI_CONN_ERROR_TASK_CID_MISMATCH,
    ISCSI_CONN_ERROR_TASK_NOT_VALID,
    ISCSI_CONN_ERROR_RQ_RING_IS_FULL,
    ISCSI_CONN_ERROR_CMDQ_RING_IS_FULL,
    ISCSI_CONN_ERROR_HQE_CACHING_FAILED,
    ISCSI_CONN_ERROR_HEADER_DIGEST_ERROR,
    ISCSI_CONN_ERROR_LOCAL_COMPLETION_ERROR,
    ISCSI_CONN_ERROR_DATA_OVERRUN,
    ISCSI_CONN_ERROR_OUT_OF_SGES_ERROR,
    ISCSI_CONN_ERROR_IP_OPTIONS_ERROR,
    ISCSI_CONN_ERROR_PRS_ERRORS,
    ISCSI_CONN_ERROR_CONNECT_INVALID_TCP_OPTION,
    ISCSI_CONN_ERROR_TCP_IP_FRAGMENT_ERROR,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_AHS_LEN,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_AHS_TYPE,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_ITT_OUT_OF_RANGE,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_TTT_OUT_OF_RANGE,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DATA_SEG_LEN_EXCEEDS_PDU_SIZE,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_INVALID_OPCODE,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_INVALID_OPCODE_BEFORE_UPDATE,
    ISCSI_CONN_ERROR_UNVALID_NOPIN_DSL,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_R2T_CARRIES_NO_DATA,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DATA_SN,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DATA_IN_TTT,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DATA_OUT_ITT,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_R2T_TTT,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_R2T_BUFFER_OFFSET,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_BUFFER_OFFSET_OOO,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_R2T_SN,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DESIRED_DATA_TRNS_LEN_0,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DESIRED_DATA_TRNS_LEN_1,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DESIRED_DATA_TRNS_LEN_2,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_LUN,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_F_BIT_ZERO,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_F_BIT_ZERO_S_BIT_ONE,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_EXP_STAT_SN,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DSL_NOT_ZERO,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_INVALID_DSL,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DATA_SEG_LEN_TOO_BIG,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_OUTSTANDING_R2T_COUNT,
    ISCSI_CONN_ERROR_PROTOCOL_ERR_DIF_TX,
    ISCSI_CONN_ERROR_SENSE_DATA_LENGTH,
    ISCSI_CONN_ERROR_DATA_PLACEMENT_ERROR,
    ISCSI_CONN_ERROR_INVALID_ITT,
    ISCSI_ERROR_UNKNOWN,
    MAX_ISCSI_ERROR_TYPES
}

// iSCSI Ramrod Command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_ramrod_cmd_id {
    ISCSI_RAMROD_CMD_ID_UNUSED = 0,
    ISCSI_RAMROD_CMD_ID_INIT_FUNC = 1,
    ISCSI_RAMROD_CMD_ID_DESTROY_FUNC = 2,
    ISCSI_RAMROD_CMD_ID_OFFLOAD_CONN = 3,
    ISCSI_RAMROD_CMD_ID_UPDATE_CONN = 4,
    ISCSI_RAMROD_CMD_ID_TERMINATION_CONN = 5,
    ISCSI_RAMROD_CMD_ID_CLEAR_SQ = 6,
    ISCSI_RAMROD_CMD_ID_MAC_UPDATE = 7,
    ISCSI_RAMROD_CMD_ID_CONN_STATS = 8,
    MAX_ISCSI_RAMROD_CMD_ID
}

// iSCSI connection termination request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_spe_conn_mac_update {
    pub reserved0: __le16,
    pub conn_id: __le16,
    pub reserved1: __le32,
    pub remote_mac_addr_lo: __le16,
    pub remote_mac_addr_mid: __le16,
    pub remote_mac_addr_hi: __le16,
    pub reserved2: [u8; 2],
}

// iSCSI and TCP connection (Option 1) offload params passed by driver to FW in
// iSCSI offload ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_spe_conn_offload {
    pub reserved0: __le16,
    pub conn_id: __le16,
    pub reserved1: __le32,
    pub iscsi: iscsi_conn_offload_params,
    pub tcp: tcp_offload_params,
}

// iSCSI and TCP connection(Option 2) offload params passed by driver to FW in
// iSCSI offload ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_spe_conn_offload_option2 {
    pub reserved0: __le16,
    pub conn_id: __le16,
    pub reserved1: __le32,
    pub iscsi: iscsi_conn_offload_params,
    pub tcp: tcp_offload_params_opt2,
}

// iSCSI collect connection statistics request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_spe_conn_statistics {
    pub reserved0: __le16,
    pub conn_id: __le16,
    pub reserved1: __le32,
    pub reset_stats: u8,
    pub reserved2: [u8; 7],
    pub stats_cnts_addr: regpair,
}

// iSCSI connection termination request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_spe_conn_termination {
    pub reserved0: __le16,
    pub conn_id: __le16,
    pub reserved1: __le32,
    pub abortive: u8,
    pub reserved2: [u8; 7],
    pub queue_cnts_addr: regpair,
    pub query_params_addr: regpair,
}

// iSCSI firmware function init parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_spe_func_init {
    pub half_way_close_timeout: __le16,
    pub num_sq_pages_in_ring: u8,
    pub num_r2tq_pages_in_ring: u8,
    pub num_uhq_pages_in_ring: u8,
    pub ll2_rx_queue_id: u8,
    pub flags: u8,
pub const ISCSI_SPE_FUNC_INIT_COUNTERS_EN_MASK: c_uint = 0x1;
pub const ISCSI_SPE_FUNC_INIT_COUNTERS_EN_SHIFT: c_int = 0;
pub const ISCSI_SPE_FUNC_INIT_RESERVED0_MASK: c_uint = 0x7F;
pub const ISCSI_SPE_FUNC_INIT_RESERVED0_SHIFT: c_int = 1;
    pub debug_mode: iscsi_debug_modes,
    pub params: u8,
pub const ISCSI_SPE_FUNC_INIT_MAX_SYN_RT_MASK: c_uint = 0xF;
pub const ISCSI_SPE_FUNC_INIT_MAX_SYN_RT_SHIFT: c_int = 0;
pub const ISCSI_SPE_FUNC_INIT_RESERVED1_MASK: c_uint = 0xF;
pub const ISCSI_SPE_FUNC_INIT_RESERVED1_SHIFT: c_int = 4;
    pub reserved2: [u8; 7],
    pub func_params: scsi_init_func_params,
    pub q_params: scsi_init_func_queues,
}

// iSCSI task type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_task_type {
    ISCSI_TASK_TYPE_INITIATOR_WRITE,
    ISCSI_TASK_TYPE_INITIATOR_READ,
    ISCSI_TASK_TYPE_MIDPATH,
    ISCSI_TASK_TYPE_UNSOLIC,
    ISCSI_TASK_TYPE_EXCHCLEANUP,
    ISCSI_TASK_TYPE_IRRELEVANT,
    ISCSI_TASK_TYPE_TARGET_WRITE,
    ISCSI_TASK_TYPE_TARGET_READ,
    ISCSI_TASK_TYPE_TARGET_RESPONSE,
    ISCSI_TASK_TYPE_LOGIN_RESPONSE,
    ISCSI_TASK_TYPE_TARGET_IMM_W_DIF,
    MAX_ISCSI_TASK_TYPE
}

// iSCSI DesiredDataTransferLength/ttt union
#[repr(C)]
#[derive(Copy, Clone)]
pub union iscsi_ttt_txlen_union {
    pub desired_tx_len: __le32,
    pub ttt: __le32,
}

// iSCSI uHQ element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_uhqe {
    pub reg1: __le32,
pub const ISCSI_UHQE_PDU_PAYLOAD_LEN_MASK: c_uint = 0xFFFFF;
pub const ISCSI_UHQE_PDU_PAYLOAD_LEN_SHIFT: c_int = 0;
pub const ISCSI_UHQE_LOCAL_COMP_MASK: c_uint = 0x1;
pub const ISCSI_UHQE_LOCAL_COMP_SHIFT: c_int = 20;
pub const ISCSI_UHQE_TOGGLE_BIT_MASK: c_uint = 0x1;
pub const ISCSI_UHQE_TOGGLE_BIT_SHIFT: c_int = 21;
pub const ISCSI_UHQE_PURE_PAYLOAD_MASK: c_uint = 0x1;
pub const ISCSI_UHQE_PURE_PAYLOAD_SHIFT: c_int = 22;
pub const ISCSI_UHQE_LOGIN_RESPONSE_PDU_MASK: c_uint = 0x1;
pub const ISCSI_UHQE_LOGIN_RESPONSE_PDU_SHIFT: c_int = 23;
pub const ISCSI_UHQE_TASK_ID_HI_MASK: c_uint = 0xFF;
pub const ISCSI_UHQE_TASK_ID_HI_SHIFT: c_int = 24;
    pub reg2: __le32,
pub const ISCSI_UHQE_BUFFER_OFFSET_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_UHQE_BUFFER_OFFSET_SHIFT: c_int = 0;
pub const ISCSI_UHQE_TASK_ID_LO_MASK: c_uint = 0xFF;
pub const ISCSI_UHQE_TASK_ID_LO_SHIFT: c_int = 24;
}

// iSCSI WQ element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_wqe {
    pub task_id: __le16,
    pub flags: u8,
pub const ISCSI_WQE_WQE_TYPE_MASK: c_uint = 0x7;
pub const ISCSI_WQE_WQE_TYPE_SHIFT: c_int = 0;
pub const ISCSI_WQE_NUM_SGES_MASK: c_uint = 0xF;
pub const ISCSI_WQE_NUM_SGES_SHIFT: c_int = 3;
pub const ISCSI_WQE_RESPONSE_MASK: c_uint = 0x1;
pub const ISCSI_WQE_RESPONSE_SHIFT: c_int = 7;
    pub prot_flags: iscsi_dif_flags,
    pub contlen_cdbsize: __le32,
pub const ISCSI_WQE_CONT_LEN_MASK: c_uint = 0xFFFFFF;
pub const ISCSI_WQE_CONT_LEN_SHIFT: c_int = 0;
pub const ISCSI_WQE_CDB_SIZE_MASK: c_uint = 0xFF;
pub const ISCSI_WQE_CDB_SIZE_SHIFT: c_int = 24;
}

// iSCSI wqe type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_wqe_type {
    ISCSI_WQE_TYPE_NORMAL,
    ISCSI_WQE_TYPE_TASK_CLEANUP,
    ISCSI_WQE_TYPE_MIDDLE_PATH,
    ISCSI_WQE_TYPE_LOGIN,
    ISCSI_WQE_TYPE_FIRST_R2T_CONT,
    ISCSI_WQE_TYPE_NONFIRST_R2T_CONT,
    ISCSI_WQE_TYPE_RESPONSE,
    MAX_ISCSI_WQE_TYPE
}

// iSCSI xHQ element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_xhqe {
    pub ttt_or_txlen: iscsi_ttt_txlen_union,
    pub exp_stat_sn: __le32,
    pub prot_flags: iscsi_dif_flags,
    pub total_ahs_length: u8,
    pub opcode: u8,
    pub flags: u8,
pub const ISCSI_XHQE_FINAL_MASK: c_uint = 0x1;
pub const ISCSI_XHQE_FINAL_SHIFT: c_int = 0;
pub const ISCSI_XHQE_STATUS_BIT_MASK: c_uint = 0x1;
pub const ISCSI_XHQE_STATUS_BIT_SHIFT: c_int = 1;
pub const ISCSI_XHQE_NUM_SGES_MASK: c_uint = 0xF;
pub const ISCSI_XHQE_NUM_SGES_SHIFT: c_int = 2;
pub const ISCSI_XHQE_RESERVED0_MASK: c_uint = 0x3;
pub const ISCSI_XHQE_RESERVED0_SHIFT: c_int = 6;
    pub seq_num: iscsi_seq_num,
    pub reserved1: __le16,
}

// Per PF iSCSI receive path statistics - mStorm RAM structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_iscsi_stats_drv {
    pub iscsi_rx_dropped_pdus_task_not_valid: regpair,
    pub iscsi_rx_dup_ack_cnt: regpair,
}

// Per PF iSCSI transmit path statistics - pStorm RAM structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_iscsi_stats_drv {
    pub iscsi_tx_bytes_cnt: regpair,
    pub iscsi_tx_packet_cnt: regpair,
}

// Per PF iSCSI receive path statistics - tStorm RAM structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_iscsi_stats_drv {
    pub iscsi_rx_bytes_cnt: regpair,
    pub iscsi_rx_packet_cnt: regpair,
    pub iscsi_rx_new_ooo_isle_events_cnt: regpair,
    pub iscsi_rx_tcp_payload_bytes_cnt: regpair,
    pub iscsi_rx_tcp_pkt_cnt: regpair,
    pub iscsi_rx_pure_ack_cnt: regpair,
    pub iscsi_cmdq_threshold_cnt: __le32,
    pub iscsi_rq_threshold_cnt: __le32,
    pub iscsi_immq_threshold_cnt: __le32,
}

// Per PF iSCSI receive path statistics - uStorm RAM structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_stats_drv {
    pub iscsi_rx_data_pdu_cnt: regpair,
    pub iscsi_rx_r2t_pdu_cnt: regpair,
    pub iscsi_rx_total_pdu_cnt: regpair,
}

// Per PF iSCSI transmit path statistics - xStorm RAM structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_iscsi_stats_drv {
    pub iscsi_tx_go_to_slow_start_event_cnt: regpair,
    pub iscsi_tx_fast_retransmit_event_cnt: regpair,
    pub iscsi_tx_pure_ack_cnt: regpair,
    pub iscsi_tx_delayed_ack_cnt: regpair,
}

// Per PF iSCSI transmit path statistics - yStorm RAM structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_iscsi_stats_drv {
    pub iscsi_tx_data_pdu_cnt: regpair,
    pub iscsi_tx_r2t_pdu_cnt: regpair,
    pub iscsi_tx_total_pdu_cnt: regpair,
    pub iscsi_tx_tcp_payload_bytes_cnt: regpair,
    pub iscsi_tx_tcp_pkt_cnt: regpair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_iscsi_task_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub word0: __le16,
    pub flags0: u8,
pub const TSTORM_ISCSI_TASK_AG_CTX_NIBBLE0_MASK: c_uint = 0xF;
pub const TSTORM_ISCSI_TASK_AG_CTX_NIBBLE0_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT0_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT1_SHIFT: c_int = 5;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT2_SHIFT: c_int = 6;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT3_SHIFT: c_int = 7;
    pub flags1: u8,
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT4_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_BIT5_SHIFT: c_int = 1;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF0_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF1_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_ISCSI_TASK_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF3_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF4_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF5_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_ISCSI_TASK_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF7_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF0EN_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF1EN_SHIFT: c_int = 3;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF2EN_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF3EN_SHIFT: c_int = 5;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF4EN_SHIFT: c_int = 6;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF5EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_ISCSI_TASK_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF6EN_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_CF7EN_SHIFT: c_int = 1;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 3;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 5;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 6;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub word1: __le16,
    pub reg0: __le32,
    pub byte3: u8,
    pub byte4: u8,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg1: __le32,
    pub reg2: __le32,
}

// iSCSI doorbell data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_db_data {
    pub params: u8,
pub const ISCSI_DB_DATA_DEST_MASK: c_uint = 0x3;
pub const ISCSI_DB_DATA_DEST_SHIFT: c_int = 0;
pub const ISCSI_DB_DATA_AGG_CMD_MASK: c_uint = 0x3;
pub const ISCSI_DB_DATA_AGG_CMD_SHIFT: c_int = 2;
pub const ISCSI_DB_DATA_BYPASS_EN_MASK: c_uint = 0x1;
pub const ISCSI_DB_DATA_BYPASS_EN_SHIFT: c_int = 4;
pub const ISCSI_DB_DATA_RESERVED_MASK: c_uint = 0x1;
pub const ISCSI_DB_DATA_RESERVED_SHIFT: c_int = 5;
pub const ISCSI_DB_DATA_AGG_VAL_SEL_MASK: c_uint = 0x3;
pub const ISCSI_DB_DATA_AGG_VAL_SEL_SHIFT: c_int = 6;
    pub agg_flags: u8,
    pub sq_prod: __le16,
}
