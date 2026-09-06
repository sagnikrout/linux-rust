//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_scsi.h
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


//
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2024 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc and/or its subsidiaries.
// Copyright (C) 2004-2016 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//

pub const LPFC_FCP_CDB_LEN: c_int = 16;
pub const LPFC_FCP_CDB_LEN_32: c_int = 32;

// per-port data that is allocated in the FC transport for us
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rport_data {
    pub /: *mut *mut *mut lpfc_nodelist pnode; / Pointer to the node structure.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_device_id {
    pub vport_wwpn: lpfc_name,
    pub target_wwpn: lpfc_name,
    pub lun: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_device_data {
    pub listentry: list_head,
    pub rport_data: *mut lpfc_rport_data,
    pub device_id: lpfc_device_id,
    pub priority: u8,
    pub oas_enabled: bool,
    pub available: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_rsp {
    pub /: *mut *mut uint32_t rspRsvd1; / FC Word 0, byte 0:3,
    pub /: *mut *mut uint32_t rspRsvd2; / FC Word 1, byte 0:3,
    pub /: *mut *mut uint8_t rspStatus0; / FCP_STATUS byte 0 (reserved),
    pub /: *mut *mut uint8_t rspStatus1; / FCP_STATUS byte 1 (reserved),
    pub /: *mut *mut uint8_t rspStatus2; / FCP_STATUS byte 2 field validity,
pub const RSP_LEN_VALID: c_uint = 0x01	/* bit 0 */;
pub const SNS_LEN_VALID: c_uint = 0x02	/* bit 1 */;
pub const RESID_OVER: c_uint = 0x04	/* bit 2 */;
pub const RESID_UNDER: c_uint = 0x08	/* bit 3 */;
    pub /: *mut *mut uint8_t rspStatus3; / FCP_STATUS byte 3 SCSI status byte,
    pub in: *mut *mut uint32_t rspResId; / Residual xfer if residual count field set,
// Received in Big Endian format
    pub /: *mut *mut uint32_t rspSnsLen; / Length of sense data in fcpSnsInfo,
// Received in Big Endian format
    pub /: *mut *mut uint32_t rspRspLen; / Length of FCP response data in fcpRspInfo,
// Received in Big Endian format
    pub /: *mut *mut uint8_t rspInfo0; / FCP_RSP_INFO byte 0 (reserved),
    pub /: *mut *mut uint8_t rspInfo1; / FCP_RSP_INFO byte 1 (reserved),
    pub /: *mut *mut uint8_t rspInfo2; / FCP_RSP_INFO byte 2 (reserved),
    pub /: *mut *mut uint8_t rspInfo3; / FCP_RSP_INFO RSP_CODE byte 3,
pub const RSP_NO_FAILURE: c_uint = 0x00;
pub const RSP_DATA_BURST_ERR: c_uint = 0x01;
pub const RSP_CMD_FIELD_ERR: c_uint = 0x02;
pub const RSP_RO_MISMATCH_ERR: c_uint = 0x03;
pub const RSP_TM_NOT_SUPPORTED: c_uint = 0x04	/* Task mgmt function not supported */;
pub const RSP_TM_NOT_COMPLETED: c_uint = 0x05	/* Task mgmt function not performed */;
pub const RSP_TM_INVALID_LU: c_uint = 0x09	/* Task mgmt function to invalid LU */;
    pub /: *mut *mut uint32_t rspInfoRsvd; / FCP_RSP_INFO bytes 4-7 (reserved),
    pub rspSnsInfo: [u8; 128],
pub const SNS_ILLEGAL_REQ: c_uint = 0x05	/* sense key is byte 3 ([2]) */;
pub const SNSCOD_BADCMD: c_uint = 0x20	/* sense code is byte 13 ([12]) */;
}

pub const SIMPLE_Q: c_uint = 0x00;
pub const HEAD_OF_Q: c_uint = 0x01;
pub const ORDERED_Q: c_uint = 0x02;
pub const ACA_Q: c_uint = 0x04;
pub const UNTAGGED: c_uint = 0x05;
pub const FCP_ABORT_TASK_SET: c_uint = 0x02	/* Bit 1 */;
pub const FCP_CLEAR_TASK_SET: c_uint = 0x04	/* bit 2 */;
pub const FCP_BUS_RESET: c_uint = 0x08	/* bit 3 */;
pub const FCP_LUN_RESET: c_uint = 0x10	/* bit 4 */;
pub const FCP_TARGET_RESET: c_uint = 0x20	/* bit 5 */;
pub const FCP_CLEAR_ACA: c_uint = 0x40	/* bit 6 */;
pub const FCP_TERMINATE_TASK: c_uint = 0x80	/* bit 7 */;
pub const WRITE_DATA: c_uint = 0x01	/* Bit 0 */;
pub const READ_DATA: c_uint = 0x02	/* Bit 1 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_cmnd {
    pub fcp_lun: scsi_lun,
    pub /: *mut *mut uint8_t fcpCntl0; / FCP_CNTL byte 0 (reserved),
    pub /: *mut *mut uint8_t fcpCntl1; / FCP_CNTL byte 1 task codes,
    pub /: *mut *mut uint8_t fcpCntl2; / FCP_CTL byte 2 task management codes,
    pub fcpCntl3: u8,
    pub /: *mut *mut uint8_t fcpCdb[LPFC_FCP_CDB_LEN]; / SRB cdb field is copied here,
    pub /: *mut *mut __be32 fcpDl; / Total transfer length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_cmnd32 {
    pub fcp_lun: scsi_lun,
    pub /: *mut *mut uint8_t fcpCntl0; / FCP_CNTL byte 0 (reserved),
    pub /: *mut *mut uint8_t fcpCntl1; / FCP_CNTL byte 1 task codes,
    pub /: *mut *mut uint8_t fcpCntl2; / FCP_CTL byte 2 task management codes,
    pub fcpCntl3: u8,
    pub /: *mut *mut uint8_t fcpCdb[LPFC_FCP_CDB_LEN_32]; / SRB cdb field is copied here,
    pub /: *mut *mut __be32 fcpDl; / Total transfer length,
}

pub const LPFC_SCSI_DMA_EXT_SIZE: c_int = 264;
pub const LPFC_BPL_SIZE: c_int = 1024;
pub const MDAC_DIRECT_CMD: c_uint = 0x22;
pub const FIND_FIRST_OAS_LUN: c_int = 0;

pub const FC_PORTSPEED_128GBIT: c_uint = 0x2000;

pub const FC_PORTSPEED_256GBIT: c_uint = 0x4000;

pub const TXRDY_PAYLOAD_LEN: c_int = 12;
