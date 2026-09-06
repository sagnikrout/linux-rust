//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedf/qedf_hsi.h
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
// QLogic FCoE Offload Driver
// Copyright (c) 2016-2018 Cavium Inc.
//
// Add include to common target
//

//
// Add include to common storage target
//

//
// Add include to common fcoe target for both eCore and protocol driver
//

//
// FCoE CQ element ABTS information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_abts_info {
    pub /: *mut *mut u8 r_ctl / R_CTL in the ABTS response frame,
    pub reserved0: u8,
    pub rx_id: __le16,
    pub reserved2: [__le32; 2],
    pub /: *mut *mut __le32 fc_payload[3] / ABTS FC payload response frame,
}

//
// FCoE class type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_class_type {
    FCOE_TASK_CLASS_TYPE_3,
    FCOE_TASK_CLASS_TYPE_2,
    MAX_FCOE_CLASS_TYPE
}

//
// FCoE CMDQ element control information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cmdqe_control {
    pub conn_id: __le16,
    pub num_additional_cmdqes: u8,
    pub cmdType: u8,
// true for ABTS request cmdqe. used in Target mode
pub const FCOE_CMDQE_CONTROL_ABTSREQCMD_MASK: c_uint = 0x1;
pub const FCOE_CMDQE_CONTROL_ABTSREQCMD_SHIFT: c_int = 0;
pub const FCOE_CMDQE_CONTROL_RESERVED1_MASK: c_uint = 0x7F;
pub const FCOE_CMDQE_CONTROL_RESERVED1_SHIFT: c_int = 1;
    pub reserved2: [u8; 4],
}

//
// FCoE control + payload CMDQ element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cmdqe {
    pub hdr: fcoe_cmdqe_control,
    pub fc_header: [u8; 24],
    pub fcp_cmd_payload: [__le32; 8],
}

//
// FCP RSP flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcp_rsp_flags {
    pub flags: u8,
pub const FCOE_FCP_RSP_FLAGS_FCP_RSP_LEN_VALID_MASK: c_uint = 0x1;
pub const FCOE_FCP_RSP_FLAGS_FCP_RSP_LEN_VALID_SHIFT: c_int = 0;
pub const FCOE_FCP_RSP_FLAGS_FCP_SNS_LEN_VALID_MASK: c_uint = 0x1;
pub const FCOE_FCP_RSP_FLAGS_FCP_SNS_LEN_VALID_SHIFT: c_int = 1;
pub const FCOE_FCP_RSP_FLAGS_FCP_RESID_OVER_MASK: c_uint = 0x1;
pub const FCOE_FCP_RSP_FLAGS_FCP_RESID_OVER_SHIFT: c_int = 2;
pub const FCOE_FCP_RSP_FLAGS_FCP_RESID_UNDER_MASK: c_uint = 0x1;
pub const FCOE_FCP_RSP_FLAGS_FCP_RESID_UNDER_SHIFT: c_int = 3;
pub const FCOE_FCP_RSP_FLAGS_FCP_CONF_REQ_MASK: c_uint = 0x1;
pub const FCOE_FCP_RSP_FLAGS_FCP_CONF_REQ_SHIFT: c_int = 4;
pub const FCOE_FCP_RSP_FLAGS_FCP_BIDI_FLAGS_MASK: c_uint = 0x7;
pub const FCOE_FCP_RSP_FLAGS_FCP_BIDI_FLAGS_SHIFT: c_int = 5;
}

//
// FCoE CQ element response information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cqe_rsp_info {
    pub rsp_flags: fcoe_fcp_rsp_flags,
    pub scsi_status_code: u8,
    pub retry_delay_timer: __le16,
    pub fcp_resid: __le32,
    pub fcp_sns_len: __le32,
    pub fcp_rsp_len: __le32,
    pub rx_id: __le16,
    pub fw_error_flags: u8,
pub const FCOE_CQE_RSP_INFO_FW_UNDERRUN_MASK: c_uint = 0x1 /* FW detected underrun */;
pub const FCOE_CQE_RSP_INFO_FW_UNDERRUN_SHIFT: c_int = 0;
pub const FCOE_CQE_RSP_INFO_RESREVED_MASK: c_uint = 0x7F;
pub const FCOE_CQE_RSP_INFO_RESREVED_SHIFT: c_int = 1;
    pub reserved: u8,
    pub /: *mut *mut __le32 fw_residual / Residual bytes calculated by FW,
}

//
// FCoE CQ element Target completion information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cqe_target_info {
    pub rx_id: __le16,
    pub reserved0: __le16,
    pub reserved1: [__le32; 5],
}

//
// FCoE error/warning reporting entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_err_report_entry {
    pub /: *mut *mut __le32 err_warn_bitmap_lo / Error bitmap lower 32 bits,
    pub /: *mut *mut __le32 err_warn_bitmap_hi / Error bitmap higher 32 bits,
// Buffer offset the beginning of the Sequence last transmitted
    pub tx_buf_off: __le32,
// Buffer offset from the beginning of the Sequence last received
    pub rx_buf_off: __le32,
    pub /: *mut *mut __le16 rx_id / RX_ID of the associated task,
    pub reserved1: __le16,
    pub reserved2: __le32,
}

//
// FCoE CQ element middle path information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cqe_midpath_info {
    pub data_placement_size: __le32,
    pub rx_id: __le16,
    pub reserved0: __le16,
    pub reserved1: [__le32; 4],
}

//
// FCoE CQ element unsolicited information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_unsolic_info {
// BD information: Physical address and opaque data
    pub bd_info: scsi_bd,
    pub /: *mut *mut __le16 conn_id / Connection ID the frame is associated to,
    pub /: *mut *mut __le16 pkt_len / Packet length,
    pub reserved1: [u8; 4],
}

//
// FCoE warning reporting entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_warning_report_entry {
// BD information: Physical address and opaque data
    pub bd_info: scsi_bd,
// Buffer offset the beginning of the Sequence last transmitted
    pub buf_off: __le32,
    pub /: *mut *mut __le16 rx_id / RX_ID of the associated task,
    pub reserved1: __le16,
}

//
// FCoE CQ element information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_cqe_info {
    pub /: *mut *mut fcoe_cqe_rsp_info rsp_info / Response completion information,
// Target completion information
    pub target_info: fcoe_cqe_target_info,
// Error completion information
    pub err_info: fcoe_err_report_entry,
    pub /: *mut *mut fcoe_abts_info abts_info / ABTS completion information,
// Middle path completion information
    pub midpath_info: fcoe_cqe_midpath_info,
// Unsolicited packet completion information
    pub unsolic_info: fcoe_unsolic_info,
// Warning completion information (Rec Tov expiration)
    pub warn_info: fcoe_warning_report_entry,
}

//
// FCoE CQ element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cqe {
    pub cqe_data: __le32,
// The task identifier (OX_ID) to be completed
pub const FCOE_CQE_TASK_ID_MASK: c_uint = 0xFFFF;
pub const FCOE_CQE_TASK_ID_SHIFT: c_int = 0;
//
// The CQE type: 0x0 Indicating on a pending work request completion.
// 0x1 - Indicating on an unsolicited event notification. use enum
// fcoe_cqe_type  (use enum fcoe_cqe_type)
//
pub const FCOE_CQE_CQE_TYPE_MASK: c_uint = 0xF;
pub const FCOE_CQE_CQE_TYPE_SHIFT: c_int = 16;
pub const FCOE_CQE_RESERVED0_MASK: c_uint = 0xFFF;
pub const FCOE_CQE_RESERVED0_SHIFT: c_int = 20;
    pub reserved1: __le16,
    pub fw_cq_prod: __le16,
    pub cqe_info: fcoe_cqe_info,
}

//
// FCoE CQE type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_cqe_type {
// solicited response on a R/W or middle-path SQE
    FCOE_GOOD_COMPLETION_CQE_TYPE,
    FCOE_UNSOLIC_CQE_TYPE /* unsolicited packet, RQ consumed */,
    FCOE_ERROR_DETECTION_CQE_TYPE /* timer expiration, validation error */,
    FCOE_WARNING_CQE_TYPE /* rec_tov or rr_tov timer expiration */,
    FCOE_EXCH_CLEANUP_CQE_TYPE /* task cleanup completed */,
    FCOE_ABTS_CQE_TYPE /* ABTS received and task cleaned */,
    FCOE_DUMMY_CQE_TYPE /* just increment SQ CONS */,
// Task was completed wight after sending a pkt to the target
    FCOE_LOCAL_COMP_CQE_TYPE,
    MAX_FCOE_CQE_TYPE
}

//
// FCoE fast path error codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_fp_error_warning_code {
    FCOE_ERROR_CODE_XFER_OOO_RO /* XFER error codes */,
    FCOE_ERROR_CODE_XFER_RO_NOT_ALIGNED,
    FCOE_ERROR_CODE_XFER_NULL_BURST_LEN,
    FCOE_ERROR_CODE_XFER_RO_GREATER_THAN_DATA2TRNS,
    FCOE_ERROR_CODE_XFER_INVALID_PAYLOAD_SIZE,
    FCOE_ERROR_CODE_XFER_TASK_TYPE_NOT_WRITE,
    FCOE_ERROR_CODE_XFER_PEND_XFER_SET,
    FCOE_ERROR_CODE_XFER_OPENED_SEQ,
    FCOE_ERROR_CODE_XFER_FCTL,
    FCOE_ERROR_CODE_FCP_RSP_BIDI_FLAGS_SET /* FCP RSP error codes */,
    FCOE_ERROR_CODE_FCP_RSP_INVALID_LENGTH_FIELD,
    FCOE_ERROR_CODE_FCP_RSP_INVALID_SNS_FIELD,
    FCOE_ERROR_CODE_FCP_RSP_INVALID_PAYLOAD_SIZE,
    FCOE_ERROR_CODE_FCP_RSP_PEND_XFER_SET,
    FCOE_ERROR_CODE_FCP_RSP_OPENED_SEQ,
    FCOE_ERROR_CODE_FCP_RSP_FCTL,
    FCOE_ERROR_CODE_FCP_RSP_LAST_SEQ_RESET,
    FCOE_ERROR_CODE_FCP_RSP_CONF_REQ_NOT_SUPPORTED_YET,
    FCOE_ERROR_CODE_DATA_OOO_RO /* FCP DATA error codes */,
    FCOE_ERROR_CODE_DATA_EXCEEDS_DEFINED_MAX_FRAME_SIZE,
    FCOE_ERROR_CODE_DATA_EXCEEDS_DATA2TRNS,
    FCOE_ERROR_CODE_DATA_SOFI3_SEQ_ACTIVE_SET,
    FCOE_ERROR_CODE_DATA_SOFN_SEQ_ACTIVE_RESET,
    FCOE_ERROR_CODE_DATA_EOFN_END_SEQ_SET,
    FCOE_ERROR_CODE_DATA_EOFT_END_SEQ_RESET,
    FCOE_ERROR_CODE_DATA_TASK_TYPE_NOT_READ,
    FCOE_ERROR_CODE_DATA_FCTL_INITIATIR,
    FCOE_ERROR_CODE_MIDPATH_INVALID_TYPE /* Middle path error codes */,
    FCOE_ERROR_CODE_MIDPATH_SOFI3_SEQ_ACTIVE_SET,
    FCOE_ERROR_CODE_MIDPATH_SOFN_SEQ_ACTIVE_RESET,
    FCOE_ERROR_CODE_MIDPATH_EOFN_END_SEQ_SET,
    FCOE_ERROR_CODE_MIDPATH_EOFT_END_SEQ_RESET,
    FCOE_ERROR_CODE_MIDPATH_REPLY_FCTL,
    FCOE_ERROR_CODE_MIDPATH_INVALID_REPLY,
    FCOE_ERROR_CODE_MIDPATH_ELS_REPLY_RCTL,
    FCOE_ERROR_CODE_COMMON_MIDDLE_FRAME_WITH_PAD /* Common error codes */,
    FCOE_ERROR_CODE_COMMON_SEQ_INIT_IN_TCE,
    FCOE_ERROR_CODE_COMMON_FC_HDR_RX_ID_MISMATCH,
    FCOE_ERROR_CODE_COMMON_INCORRECT_SEQ_CNT,
    FCOE_ERROR_CODE_COMMON_DATA_FC_HDR_FCP_TYPE_MISMATCH,
    FCOE_ERROR_CODE_COMMON_DATA_NO_MORE_SGES,
    FCOE_ERROR_CODE_COMMON_OPTIONAL_FC_HDR,
    FCOE_ERROR_CODE_COMMON_READ_TCE_OX_ID_TOO_BIG,
    FCOE_ERROR_CODE_COMMON_DATA_WAS_NOT_TRANSMITTED,
    FCOE_ERROR_CODE_COMMON_TASK_DDF_RCTL_INFO_FIELD,
    FCOE_ERROR_CODE_COMMON_TASK_INVALID_RCTL,
    FCOE_ERROR_CODE_COMMON_TASK_RCTL_GENERAL_MISMATCH,
    FCOE_ERROR_CODE_E_D_TOV_TIMER_EXPIRATION /* Timer error codes */,
    FCOE_WARNING_CODE_REC_TOV_TIMER_EXPIRATION /* Timer error codes */,
    FCOE_ERROR_CODE_RR_TOV_TIMER_EXPIRATION /* Timer error codes */,
// ABTSrsp pckt arrived unexpected
    FCOE_ERROR_CODE_ABTS_REPLY_UNEXPECTED,
    FCOE_ERROR_CODE_TARGET_MODE_FCP_RSP,
    FCOE_ERROR_CODE_TARGET_MODE_FCP_XFER,
    FCOE_ERROR_CODE_TARGET_MODE_DATA_TASK_TYPE_NOT_WRITE,
    FCOE_ERROR_CODE_DATA_FCTL_TARGET,
    FCOE_ERROR_CODE_TARGET_DATA_SIZE_NO_MATCH_XFER,
    FCOE_ERROR_CODE_TARGET_DIF_CRC_CHECKSUM_ERROR,
    FCOE_ERROR_CODE_TARGET_DIF_REF_TAG_ERROR,
    FCOE_ERROR_CODE_TARGET_DIF_APP_TAG_ERROR,
    MAX_FCOE_FP_ERROR_WARNING_CODE
}

//
// FCoE RESPQ element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_respqe {
    pub /: *mut *mut __le16 ox_id / OX_ID that is located in the FCP_RSP FC header,
    pub /: *mut *mut __le16 rx_id / RX_ID that is located in the FCP_RSP FC header,
    pub additional_info: __le32,
// PARAM that is located in the FCP_RSP FC header
pub const FCOE_RESPQE_PARAM_MASK: c_uint = 0xFFFFFF;
pub const FCOE_RESPQE_PARAM_SHIFT: c_int = 0;
// Indication whther its Target-auto-rsp mode or not
pub const FCOE_RESPQE_TARGET_AUTO_RSP_MASK: c_uint = 0xFF;
pub const FCOE_RESPQE_TARGET_AUTO_RSP_SHIFT: c_int = 24;
}

//
// FCoE slow path error codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_sp_error_code {
// Error codes for Error Reporting in slow path flows
    FCOE_ERROR_CODE_SLOW_PATH_TOO_MANY_FUNCS,
    FCOE_ERROR_SLOW_PATH_CODE_NO_LICENSE,
    MAX_FCOE_SP_ERROR_CODE
}

//
// FCoE task TX state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_task_tx_state {
// Initiate state after driver has initialized the task
    FCOE_TASK_TX_STATE_NORMAL,
// Updated by TX path after complete transmitting unsolicited packet
    FCOE_TASK_TX_STATE_UNSOLICITED_COMPLETED,
//
// Updated by TX path after start processing the task requesting the
// cleanup/abort operation
//
    FCOE_TASK_TX_STATE_CLEAN_REQ,
    FCOE_TASK_TX_STATE_ABTS /* Updated by TX path during abort procedure */,
// Updated by TX path during exchange cleanup procedure
    FCOE_TASK_TX_STATE_EXCLEANUP,
//
// Updated by TX path during exchange cleanup continuation task
// procedure
//
    FCOE_TASK_TX_STATE_EXCLEANUP_TARGET_WRITE_CONT,
// Updated by TX path during exchange cleanup first xfer procedure
    FCOE_TASK_TX_STATE_EXCLEANUP_TARGET_WRITE,
// Updated by TX path during exchange cleanup read task in Target
    FCOE_TASK_TX_STATE_EXCLEANUP_TARGET_READ_OR_RSP,
// Updated by TX path during target exchange cleanup procedure
    FCOE_TASK_TX_STATE_EXCLEANUP_TARGET_WRITE_LAST_CYCLE,
// Updated by TX path during sequence recovery procedure
    FCOE_TASK_TX_STATE_SEQRECOVERY,
    MAX_FCOE_TASK_TX_STATE
}
