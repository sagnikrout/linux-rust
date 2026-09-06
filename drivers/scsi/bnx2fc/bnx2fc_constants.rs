//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bnx2fc/bnx2fc_constants.h
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


// bnx2fc_constants.h: QLogic Linux FCoE offload driver.
// Handles operations such as session offload/upload etc, and manages
// session resources such as connection id and qp resources.
//
// Copyright (c) 2008-2013 Broadcom Corporation
// Copyright (c) 2014-2016 QLogic Corporation
// Copyright (c) 2016-2017 Cavium Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// This file defines HSI constants for the FCoE flows
//
// Current FCoE HSI version number composed of two fields (16 bit)
// Implies on a change broken previous HSI

// Implies on a change which does not broken previous HSI

// KWQ/KCQ FCoE layer code

// KWQ (kernel work queue) request op codes

// KCQ (kernel completion queue) response op codes

// KCQ (kernel completion queue) completion status

// CQE type
pub const FCOE_PENDING_CQE_TYPE: c_int = 0;
pub const FCOE_UNSOLIC_CQE_TYPE: c_int = 1;
// Unsolicited CQE type
pub const FCOE_UNSOLICITED_FRAME_CQE_TYPE: c_int = 0;
pub const FCOE_ERROR_DETECTION_CQE_TYPE: c_int = 1;
pub const FCOE_WARNING_DETECTION_CQE_TYPE: c_int = 2;
// E_D_TOV timer resolution in ms

// E_D_TOV timer resolution for SDM (4 micro)

// REC timer resolution in ms

// REC timer resolution for SDM (4 micro)

// E_D_TOV timer default wraparound value (2 sec) in 20 ms resolution

// REC_TOV timer default wraparound value (3 sec) in 20 ms resolution

// Task context constants
// Remove FCP_CMD write tce sleep
// In case timer services are required then shall be updated by Xstorm after
// start processing the task. In case no timer facilities are required then the
// driver would initialize the state to this value
//
pub const FCOE_TASK_TX_STATE_NORMAL: c_int = 0;
// After driver has initialize the task in case timer services required
pub const FCOE_TASK_TX_STATE_INIT: c_int = 1;
// Remove FCP_CMD write tce sleep
// After driver has initialize the task in case timer services required
pub const FCOE_TASK_TX_STATE_INIT: c_int = 0;
// In case timer services are required then shall be updated by Xstorm after
// start processing the task. In case no timer facilities are required then the
// driver would initialize the state to this value
//
pub const FCOE_TASK_TX_STATE_NORMAL: c_int = 1;
// Task is under abort procedure. Updated in order to stop processing of
// pending WQEs on this task
//
pub const FCOE_TASK_TX_STATE_ABORT: c_int = 2;
// For E_D_T_TOV timer expiration in Xstorm (Class 2 only)
pub const FCOE_TASK_TX_STATE_ERROR: c_int = 3;
// For REC_TOV timer expiration indication received from Xstorm
pub const FCOE_TASK_TX_STATE_WARNING: c_int = 4;
// For completed unsolicited task
pub const FCOE_TASK_TX_STATE_UNSOLICITED_COMPLETED: c_int = 5;
// For exchange cleanup request task
pub const FCOE_TASK_TX_STATE_EXCHANGE_CLEANUP: c_int = 6;
// For sequence cleanup request task
pub const FCOE_TASK_TX_STATE_SEQUENCE_CLEANUP: c_int = 7;
// For completion the ABTS task.
pub const FCOE_TASK_TX_STATE_ABTS_TX: c_int = 8;
pub const FCOE_TASK_RX_STATE_NORMAL: c_int = 0;
pub const FCOE_TASK_RX_STATE_COMPLETED: c_int = 1;
// Obsolete: Intermediate completion (middle path with local completion)
pub const FCOE_TASK_RX_STATE_INTER_COMP: c_int = 2;
// For REC_TOV timer expiration indication received from Xstorm
pub const FCOE_TASK_RX_STATE_WARNING: c_int = 3;
// For E_D_T_TOV timer expiration in Ustorm
pub const FCOE_TASK_RX_STATE_ERROR: c_int = 4;
// FW only: First visit at rx-path, part of the abts round trip
pub const FCOE_TASK_RX_STATE_ABTS_IN_PROCESS: c_int = 5;
// FW only: Second visit at rx-path, after ABTS frame transmitted
pub const FCOE_TASK_RX_STATE_ABTS_TRANSMITTED: c_int = 6;
// Special completion indication in case of task was aborted.
pub const FCOE_TASK_RX_STATE_ABTS_COMPLETED: c_int = 7;
// FW only: First visit at rx-path, part of the cleanup round trip
pub const FCOE_TASK_RX_STATE_EXCHANGE_CLEANUP_IN_PROCESS: c_int = 8;
// FW only: Special completion indication in case of task was cleaned.
pub const FCOE_TASK_RX_STATE_EXCHANGE_CLEANUP_COMPLETED: c_int = 9;
// Not in used: Special completion indication (in task requested the exchange
// cleanup) in case cleaned task is in non-valid.
//
pub const FCOE_TASK_RX_STATE_ABORT_CLEANUP_COMPLETED: c_int = 10;
// Special completion indication (in task requested the sequence cleanup) in
// case cleaned task was already returned to normal.
//
pub const FCOE_TASK_RX_STATE_IGNORED_SEQUENCE_CLEANUP: c_int = 11;
pub const FCOE_TASK_TYPE_WRITE: c_int = 0;
pub const FCOE_TASK_TYPE_READ: c_int = 1;
pub const FCOE_TASK_TYPE_MIDPATH: c_int = 2;
pub const FCOE_TASK_TYPE_UNSOLICITED: c_int = 3;
pub const FCOE_TASK_TYPE_ABTS: c_int = 4;
pub const FCOE_TASK_TYPE_EXCHANGE_CLEANUP: c_int = 5;
pub const FCOE_TASK_TYPE_SEQUENCE_CLEANUP: c_int = 6;
pub const FCOE_TASK_DEV_TYPE_DISK: c_int = 0;
pub const FCOE_TASK_DEV_TYPE_TAPE: c_int = 1;
pub const FCOE_TASK_CLASS_TYPE_3: c_int = 0;
pub const FCOE_TASK_CLASS_TYPE_2: c_int = 1;
// FCoE/FC packet fields
pub const FCOE_ETH_TYPE: c_uint = 0x8906;
// FCoE maximum elements in hash table
pub const FCOE_MAX_ELEMENTS_IN_HASH_TABLE_ROW: c_int = 8;
// FCoE half of the elements in hash table

// FcoE number of cached T2 entries

// FCoE maximum elements in hash table
pub const FCOE_HASH_TBL_CHUNK_SIZE: c_int = 16384;
// Everest FCoE connection type
pub const B577XX_FCOE_CONNECTION_TYPE: c_int = 4;
// FCoE number of rows (in log). This number derives
// from the maximum connections supported which is 2048.
// TBA: Need a different constant for E2
//
pub const FCOE_MAX_NUM_SESSIONS_LOG: c_int = 11;
pub const FC_ABTS_REPLY_MAX_PAYLOAD_LEN: c_int = 12;
// Error codes for Error Reporting in slow path flows
pub const FCOE_SLOW_PATH_ERROR_CODE_TOO_MANY_FUNCS: c_int = 0;
pub const FCOE_SLOW_PATH_ERROR_CODE_NO_LICENSE: c_int = 1;
// Error codes for Error Reporting in fast path flows
// XFER error codes
//
pub const FCOE_ERROR_CODE_XFER_OOO_RO: c_int = 0;
pub const FCOE_ERROR_CODE_XFER_RO_NOT_ALIGNED: c_int = 1;
pub const FCOE_ERROR_CODE_XFER_NULL_BURST_LEN: c_int = 2;
pub const FCOE_ERROR_CODE_XFER_RO_GREATER_THAN_DATA2TRNS: c_int = 3;
pub const FCOE_ERROR_CODE_XFER_INVALID_PAYLOAD_SIZE: c_int = 4;
pub const FCOE_ERROR_CODE_XFER_TASK_TYPE_NOT_WRITE: c_int = 5;
pub const FCOE_ERROR_CODE_XFER_PEND_XFER_SET: c_int = 6;
pub const FCOE_ERROR_CODE_XFER_OPENED_SEQ: c_int = 7;
pub const FCOE_ERROR_CODE_XFER_FCTL: c_int = 8;
// FCP RSP error codes
pub const FCOE_ERROR_CODE_FCP_RSP_BIDI_FLAGS_SET: c_int = 9;
pub const FCOE_ERROR_CODE_FCP_RSP_UNDERFLOW: c_int = 10;
pub const FCOE_ERROR_CODE_FCP_RSP_OVERFLOW: c_int = 11;
pub const FCOE_ERROR_CODE_FCP_RSP_INVALID_LENGTH_FIELD: c_int = 12;
pub const FCOE_ERROR_CODE_FCP_RSP_INVALID_SNS_FIELD: c_int = 13;
pub const FCOE_ERROR_CODE_FCP_RSP_INVALID_PAYLOAD_SIZE: c_int = 14;
pub const FCOE_ERROR_CODE_FCP_RSP_PEND_XFER_SET: c_int = 15;
pub const FCOE_ERROR_CODE_FCP_RSP_OPENED_SEQ: c_int = 16;
pub const FCOE_ERROR_CODE_FCP_RSP_FCTL: c_int = 17;
pub const FCOE_ERROR_CODE_FCP_RSP_LAST_SEQ_RESET: c_int = 18;
pub const FCOE_ERROR_CODE_FCP_RSP_CONF_REQ_NOT_SUPPORTED_YET: c_int = 19;
// FCP DATA error codes
pub const FCOE_ERROR_CODE_DATA_OOO_RO: c_int = 20;
pub const FCOE_ERROR_CODE_DATA_EXCEEDS_DEFINED_MAX_FRAME_SIZE: c_int = 21;
pub const FCOE_ERROR_CODE_DATA_EXCEEDS_DATA2TRNS: c_int = 22;
pub const FCOE_ERROR_CODE_DATA_SOFI3_SEQ_ACTIVE_SET: c_int = 23;
pub const FCOE_ERROR_CODE_DATA_SOFN_SEQ_ACTIVE_RESET: c_int = 24;
pub const FCOE_ERROR_CODE_DATA_EOFN_END_SEQ_SET: c_int = 25;
pub const FCOE_ERROR_CODE_DATA_EOFT_END_SEQ_RESET: c_int = 26;
pub const FCOE_ERROR_CODE_DATA_TASK_TYPE_NOT_READ: c_int = 27;
pub const FCOE_ERROR_CODE_DATA_FCTL: c_int = 28;
// Middle path error codes
pub const FCOE_ERROR_CODE_MIDPATH_INVALID_TYPE: c_int = 29;
pub const FCOE_ERROR_CODE_MIDPATH_SOFI3_SEQ_ACTIVE_SET: c_int = 30;
pub const FCOE_ERROR_CODE_MIDPATH_SOFN_SEQ_ACTIVE_RESET: c_int = 31;
pub const FCOE_ERROR_CODE_MIDPATH_EOFN_END_SEQ_SET: c_int = 32;
pub const FCOE_ERROR_CODE_MIDPATH_EOFT_END_SEQ_RESET: c_int = 33;
pub const FCOE_ERROR_CODE_MIDPATH_REPLY_FCTL: c_int = 34;
pub const FCOE_ERROR_CODE_MIDPATH_INVALID_REPLY: c_int = 35;
pub const FCOE_ERROR_CODE_MIDPATH_ELS_REPLY_RCTL: c_int = 36;
// ABTS error codes
pub const FCOE_ERROR_CODE_ABTS_REPLY_F_CTL: c_int = 37;
pub const FCOE_ERROR_CODE_ABTS_REPLY_DDF_RCTL_FIELD: c_int = 38;
pub const FCOE_ERROR_CODE_ABTS_REPLY_INVALID_BLS_RCTL: c_int = 39;
pub const FCOE_ERROR_CODE_ABTS_REPLY_INVALID_RCTL: c_int = 40;
pub const FCOE_ERROR_CODE_ABTS_REPLY_RCTL_GENERAL_MISMATCH: c_int = 41;
// Common error codes
pub const FCOE_ERROR_CODE_COMMON_MIDDLE_FRAME_WITH_PAD: c_int = 42;
pub const FCOE_ERROR_CODE_COMMON_SEQ_INIT_IN_TCE: c_int = 43;
pub const FCOE_ERROR_CODE_COMMON_FC_HDR_RX_ID_MISMATCH: c_int = 44;
pub const FCOE_ERROR_CODE_COMMON_INCORRECT_SEQ_CNT: c_int = 45;
pub const FCOE_ERROR_CODE_COMMON_DATA_FC_HDR_FCP_TYPE_MISMATCH: c_int = 46;
pub const FCOE_ERROR_CODE_COMMON_DATA_NO_MORE_SGES: c_int = 47;
pub const FCOE_ERROR_CODE_COMMON_OPTIONAL_FC_HDR: c_int = 48;
pub const FCOE_ERROR_CODE_COMMON_READ_TCE_OX_ID_TOO_BIG: c_int = 49;
pub const FCOE_ERROR_CODE_COMMON_DATA_WAS_NOT_TRANSMITTED: c_int = 50;
// Unsolicited Rx error codes
pub const FCOE_ERROR_CODE_UNSOLICITED_TYPE_NOT_ELS: c_int = 51;
pub const FCOE_ERROR_CODE_UNSOLICITED_TYPE_NOT_BLS: c_int = 52;
pub const FCOE_ERROR_CODE_UNSOLICITED_FCTL_ELS: c_int = 53;
pub const FCOE_ERROR_CODE_UNSOLICITED_FCTL_BLS: c_int = 54;
pub const FCOE_ERROR_CODE_UNSOLICITED_R_CTL: c_int = 55;
pub const FCOE_ERROR_CODE_RW_TASK_DDF_RCTL_INFO_FIELD: c_int = 56;
pub const FCOE_ERROR_CODE_RW_TASK_INVALID_RCTL: c_int = 57;
pub const FCOE_ERROR_CODE_RW_TASK_RCTL_GENERAL_MISMATCH: c_int = 58;
// Timer error codes
pub const FCOE_ERROR_CODE_E_D_TOV_TIMER_EXPIRATION: c_int = 60;
pub const FCOE_ERROR_CODE_REC_TOV_TIMER_EXPIRATION: c_int = 61;
