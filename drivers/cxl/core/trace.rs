//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cxl/core/trace.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2022 Intel Corporation. All rights reserved.

//
// Embed headerlog data for user app retrieval and parsing,
// but no need to print in the trace buffer. Only
// CXL_HEADERLOG_SIZE_U32 (16) dwords are hardware data;
// the remaining entries preserve the 512-byte ABI layout
// rasdaemon depends on and are zero-filled by the caller.
//
extern "C" {
    pub fn sizeof(_arg: u32)) -> *mut CXL_HEADERLOG_TRACE_SIZE_U32;
}
//
// Embed headerlog data for user app retrieval and parsing,
// but no need to print in the trace buffer. Only
// CXL_HEADERLOG_SIZE_U32 (16) dwords are hardware data;
// the remaining entries preserve the 512-byte ABI layout
// rasdaemon depends on and are zero-filled by the caller.
//
extern "C" {
    pub fn sizeof(_arg: u32)) -> *mut CXL_HEADERLOG_TRACE_SIZE_U32;
}

//
// Common Event Record Format
// CXL 3.0 section 8.2.9.2.1; Table 8-42
//

//
// Define macros for the common header of each CXL event.
//
// Tracepoints using these macros must do 3 things:
//
// 1) Add CXL_EVT_TP_entry to TP_STRUCT__entry
// 2) Use CXL_EVT_TP_fast_assign within TP_fast_assign;
// pass the dev, log, and CXL event header
// NOTE: The uuid must be assigned by the specific trace event
// 3) Use CXL_EVT_TP_printk() instead of TP_printk()
//
// See the generic_event tracepoint as an example.
//

//
// Physical Address field masks
//
// General Media Event Record
// CXL rev 3.0 Section 8.2.9.2.1.1; Table 8-43
//
// DRAM Event Record
// CXL rev 3.0 section 8.2.9.2.1.2; Table 8-44
//

//
// Component ID Format
// CXL 3.1 section 8.2.9.2.1; Table 8-44
//

//
// General Media Event Record - GMER
// CXL rev 3.1 Section 8.2.9.2.1.1; Table 8-45
//

pub const CXL_GMER_MEM_EVT_TYPE_ECC_ERROR: c_uint = 0x00;
pub const CXL_GMER_MEM_EVT_TYPE_INV_ADDR: c_uint = 0x01;
pub const CXL_GMER_MEM_EVT_TYPE_DATA_PATH_ERROR: c_uint = 0x02;
pub const CXL_GMER_MEM_EVT_TYPE_TE_STATE_VIOLATION: c_uint = 0x03;
pub const CXL_GMER_MEM_EVT_TYPE_SCRUB_MEDIA_ECC_ERROR: c_uint = 0x04;
pub const CXL_GMER_MEM_EVT_TYPE_AP_CME_COUNTER_EXPIRE: c_uint = 0x05;
pub const CXL_GMER_MEM_EVT_TYPE_CKID_VIOLATION: c_uint = 0x06;

pub const CXL_GMER_TRANS_UNKNOWN: c_uint = 0x00;
pub const CXL_GMER_TRANS_HOST_READ: c_uint = 0x01;
pub const CXL_GMER_TRANS_HOST_WRITE: c_uint = 0x02;
pub const CXL_GMER_TRANS_HOST_SCAN_MEDIA: c_uint = 0x03;
pub const CXL_GMER_TRANS_HOST_INJECT_POISON: c_uint = 0x04;
pub const CXL_GMER_TRANS_INTERNAL_MEDIA_SCRUB: c_uint = 0x05;
pub const CXL_GMER_TRANS_INTERNAL_MEDIA_MANAGEMENT: c_uint = 0x06;
pub const CXL_GMER_TRANS_INTERNAL_MEDIA_ECS: c_uint = 0x07;
pub const CXL_GMER_TRANS_MEDIA_INITIALIZATION: c_uint = 0x08;

pub const CXL_GMER_MEM_EVT_SUB_TYPE_NOT_REPORTED: c_uint = 0x00;
pub const CXL_GMER_MEM_EVT_SUB_TYPE_INTERNAL_DATAPATH_ERROR: c_uint = 0x01;
pub const CXL_GMER_MEM_EVT_SUB_TYPE_MEDIA_LINK_COMMAND_TRAINING_ERROR: c_uint = 0x02;
pub const CXL_GMER_MEM_EVT_SUB_TYPE_MEDIA_LINK_CONTROL_TRAINING_ERROR: c_uint = 0x03;
pub const CXL_GMER_MEM_EVT_SUB_TYPE_MEDIA_LINK_DATA_TRAINING_ERROR: c_uint = 0x04;
pub const CXL_GMER_MEM_EVT_SUB_TYPE_MEDIA_LINK_CRC_ERROR: c_uint = 0x05;

// General Media
// Following are out of order to pack trace record
// General Media
// Mask after flags have been parsed
//
// DRAM Event Record - DER
//
// CXL rev 3.1 section 8.2.9.2.1.2; Table 8-46
//
// DRAM Event Record defines many fields the same as the General Media Event
// Record.  Reuse those definitions as appropriate.
//
pub const CXL_DER_MEM_EVT_TYPE_ECC_ERROR: c_uint = 0x00;
pub const CXL_DER_MEM_EVT_TYPE_SCRUB_MEDIA_ECC_ERROR: c_uint = 0x01;
pub const CXL_DER_MEM_EVT_TYPE_INV_ADDR: c_uint = 0x02;
pub const CXL_DER_MEM_EVT_TYPE_DATA_PATH_ERROR: c_uint = 0x03;
pub const CXL_DER_MEM_EVT_TYPE_TE_STATE_VIOLATION: c_uint = 0x04;
pub const CXL_DER_MEM_EVT_TYPE_AP_CME_COUNTER_EXPIRE: c_uint = 0x05;
pub const CXL_DER_MEM_EVT_TYPE_CKID_VIOLATION: c_uint = 0x06;

// DRAM
// Following are out of order to pack trace record
// DRAM
//
// Memory Module Event Record - MMER
//
// CXL res 3.1 section 8.2.9.2.1.3; Table 8-47
//
pub const CXL_MMER_HEALTH_STATUS_CHANGE: c_uint = 0x00;
pub const CXL_MMER_MEDIA_STATUS_CHANGE: c_uint = 0x01;
pub const CXL_MMER_LIFE_USED_CHANGE: c_uint = 0x02;
pub const CXL_MMER_TEMP_CHANGE: c_uint = 0x03;
pub const CXL_MMER_DATA_PATH_ERROR: c_uint = 0x04;
pub const CXL_MMER_LSA_ERROR: c_uint = 0x05;
pub const CXL_MMER_UNRECOV_SIDEBAND_BUS_ERROR: c_uint = 0x06;
pub const CXL_MMER_MEMORY_MEDIA_FRU_ERROR: c_uint = 0x07;
pub const CXL_MMER_POWER_MANAGEMENT_FAULT: c_uint = 0x08;

//
// Device Health Information - DHI
//
// CXL res 3.1 section 8.2.9.9.3.1; Table 8-133
//

pub const CXL_DHI_MS_NORMAL: c_uint = 0x00;
pub const CXL_DHI_MS_NOT_READY: c_uint = 0x01;
pub const CXL_DHI_MS_WRITE_PERSISTENCY_LOST: c_uint = 0x02;
pub const CXL_DHI_MS_ALL_DATA_LOST: c_uint = 0x03;
pub const CXL_DHI_MS_WRITE_PERSISTENCY_LOSS_EVENT_POWER_LOSS: c_uint = 0x04;
pub const CXL_DHI_MS_WRITE_PERSISTENCY_LOSS_EVENT_SHUTDOWN: c_uint = 0x05;
pub const CXL_DHI_MS_WRITE_PERSISTENCY_LOSS_IMMINENT: c_uint = 0x06;
pub const CXL_DHI_MS_WRITE_ALL_DATA_LOSS_EVENT_POWER_LOSS: c_uint = 0x07;
pub const CXL_DHI_MS_WRITE_ALL_DATA_LOSS_EVENT_SHUTDOWN: c_uint = 0x08;
pub const CXL_DHI_MS_WRITE_ALL_DATA_LOSS_IMMINENT: c_uint = 0x09;

pub const CXL_DHI_AS_NORMAL: c_uint = 0x0;
pub const CXL_DHI_AS_WARNING: c_uint = 0x1;
pub const CXL_DHI_AS_CRITICAL: c_uint = 0x2;

pub const CXL_MMER_DEV_EVT_SUB_TYPE_NOT_REPORTED: c_uint = 0x00;
pub const CXL_MMER_DEV_EVT_SUB_TYPE_INVALID_CONFIG_DATA: c_uint = 0x01;
pub const CXL_MMER_DEV_EVT_SUB_TYPE_UNSUPP_CONFIG_DATA: c_uint = 0x02;
pub const CXL_MMER_DEV_EVT_SUB_TYPE_UNSUPP_MEM_MEDIA_FRU: c_uint = 0x03;

// Memory Module Event
// Device Health Info
// Memory Module Event
// Device Health Info
//
// Memory Sparing Event Record - MSER
//
// CXL rev 3.2 section 8.2.10.2.1.4; Table 8-60
//

// Memory Sparing Event

