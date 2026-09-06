//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/fc/fc_fcp.h
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
// Copyright(c) 2007 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

//
// Fibre Channel Protocol for SCSI.
// From T10 FCP-3, T10 project 1560-D Rev 4, Sept. 13, 2005.
//
// fc/fs.h defines FC_TYPE_FCP.
//
// Service parameter page parameters (word 3 bits) for Process Login.
//
pub const FCP_SPPF_TASK_RETRY_ID: c_uint = 0x0200	/* task retry ID requested */;
pub const FCP_SPPF_RETRY: c_uint = 0x0100	/* retry supported */;
pub const FCP_SPPF_CONF_COMPL: c_uint = 0x0080	/* confirmed completion allowed */;
pub const FCP_SPPF_OVLY_ALLOW: c_uint = 0x0040	/* data overlay allowed */;
pub const FCP_SPPF_INIT_FCN: c_uint = 0x0020	/* initiator function */;
pub const FCP_SPPF_TARG_FCN: c_uint = 0x0010	/* target function */;
pub const FCP_SPPF_RD_XRDY_DIS: c_uint = 0x0002	/* disable XFER_RDY for reads */;
pub const FCP_SPPF_WR_XRDY_DIS: c_uint = 0x0001	/* disable XFER_RDY for writes */;
//
// FCP_CMND IU Payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_cmnd {
    pub /: *mut *mut scsi_lun fc_lun; / logical unit number,
    pub /: *mut *mut __u8 fc_cmdref; / command reference number,
    pub /: *mut *mut __u8 fc_pri_ta; / priority and task attribute,
    pub /: *mut *mut __u8 fc_tm_flags; / task management flags,
    pub /: *mut *mut __u8 fc_flags; / additional len & flags,
    pub /: *mut *mut __u8 fc_cdb[16]; / base CDB,
    pub /: *mut *mut __be32 fc_dl; / data length (must follow fc_cdb),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_cmnd32 {
    pub /: *mut *mut scsi_lun fc_lun; / logical unit number,
    pub /: *mut *mut __u8 fc_cmdref; / command reference number,
    pub /: *mut *mut __u8 fc_pri_ta; / priority and task attribute,
    pub /: *mut *mut __u8 fc_tm_flags; / task management flags,
    pub /: *mut *mut __u8 fc_flags; / additional len & flags,
    pub /: *mut *mut __u8 fc_cdb[32]; / base CDB,
    pub /: *mut *mut __be32 fc_dl; / data length (must follow fc_cdb),
}

//
// fc_pri_ta.
//

pub const FCP_PRI_RESVD_MASK: c_uint = 0x80	/* reserved bits in priority field */;
//
// fc_tm_flags - task management flags field.
//
pub const FCP_TMF_CLR_ACA: c_uint = 0x40	/* clear ACA condition */;
pub const FCP_TMF_TGT_RESET: c_uint = 0x20	/* target reset task management,;
pub const FCP_TMF_LUN_RESET: c_uint = 0x10	/* logical unit reset task management */;
pub const FCP_TMF_CLR_TASK_SET: c_uint = 0x04	/* clear task set */;
pub const FCP_TMF_ABT_TASK_SET: c_uint = 0x02	/* abort task set */;
//
// fc_flags.
// Bits 7:2 are the additional FCP_CDB length / 4.
//
pub const FCP_CFL_LEN_MASK: c_uint = 0xfc	/* mask for additional length */;

pub const FCP_CFL_RDDATA: c_uint = 0x02	/* read data */;
pub const FCP_CFL_WRDATA: c_uint = 0x01	/* write data */;
//
// FCP_TXRDY IU - transfer ready payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_txrdy {
    pub /: *mut *mut __be32 ft_data_ro; / data relative offset,
    pub /: *mut *mut __be32 ft_burst_len; / burst length,
    pub /: *mut *mut __u8 _ft_resvd[4]; / reserved,
}

//
// FCP_RESP IU - response payload.
//
// The response payload comes in three parts: the flags/status, the
// sense/response lengths and the sense data/response info section.
//
// From FCP3r04, note 6 of section 9.5.13:
//
// Some early implementations presented the FCP_RSP IU without the FCP_RESID,
// FCP_SNS_LEN, and FCP_RSP_LEN fields if the FCP_RESID_UNDER, FCP_RESID_OVER,
// FCP_SNS_LEN_VALID, and FCP_RSP_LEN_VALID bits were all set to zero. This
// non-standard behavior should be tolerated.
//
// All response frames will always contain the fcp_resp template.  Some
// will also include the fcp_resp_len template.
//
// From Table 23, the FCP_RSP_INFO can either be 4 bytes or 8 bytes, both
// are valid length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_resp {
    pub /: *mut *mut __u8 _fr_resvd[8]; / reserved,
    pub /: *mut *mut __be16 fr_retry_delay; / retry delay timer,
    pub /: *mut *mut __u8 fr_flags; / flags,
    pub /: *mut *mut __u8 fr_status; / SCSI status code,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_resp_ext {
    pub /: *mut *mut __be32 fr_resid; / Residual value,
    pub /: *mut *mut __be32 fr_sns_len; / SCSI Sense length,
    pub /: *mut *mut __be32 fr_rsp_len; / Response Info length,
//
// Optionally followed by RSP info and/or SNS info and/or
// bidirectional read residual length, if any.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_resp_rsp_info {
    pub /: *mut *mut __u8 _fr_resvd[3]; / reserved,
    pub /: *mut *mut __u8 rsp_code; / Response Info Code,
    pub /: *mut *mut __u8 _fr_resvd2[4]; / reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_resp_with_ext {
    pub resp: fcp_resp,
    pub ext: fcp_resp_ext,
}

//
// fr_flags.
//
pub const FCP_BIDI_RSP: c_uint = 0x80	/* bidirectional read response */;
pub const FCP_BIDI_READ_UNDER: c_uint = 0x40	/* bidir. read less than requested */;
pub const FCP_BIDI_READ_OVER: c_uint = 0x20	/* DL insufficient for full transfer */;
pub const FCP_CONF_REQ: c_uint = 0x10	/* confirmation requested */;
pub const FCP_RESID_UNDER: c_uint = 0x08	/* transfer shorter than expected */;
pub const FCP_RESID_OVER: c_uint = 0x04	/* DL insufficient for full transfer */;
pub const FCP_SNS_LEN_VAL: c_uint = 0x02	/* SNS_LEN field is valid */;
pub const FCP_RSP_LEN_VAL: c_uint = 0x01	/* RSP_LEN field is valid */;
//
// rsp_codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcp_resp_rsp_codes {
    FCP_TMF_CMPL = 0,
    FCP_DATA_LEN_INVALID = 1,
    FCP_CMND_FIELDS_INVALID = 2,
    FCP_DATA_PARAM_MISMATCH = 3,
    FCP_TMF_REJECTED = 4,
    FCP_TMF_FAILED = 5,
    FCP_TMF_INVALID_LUN = 9,
}

//
// FCP SRR Link Service request - Sequence Retransmission Request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_srr {
    pub /: *mut *mut __u8 srr_op; / opcode ELS_SRR,
    pub /: *mut *mut __u8 srr_resvd[3]; / opcode / reserved - must be zero,
    pub /: *mut *mut __be16 srr_ox_id; / OX_ID of failed command,
    pub /: *mut *mut __be16 srr_rx_id; / RX_ID of failed command,
    pub /: *mut *mut __be32 srr_rel_off; / relative offset,
    pub /: *mut *mut __u8 srr_r_ctl; / r_ctl for the information unit,
    pub /: *mut *mut __u8 srr_resvd2[3]; / reserved,
}

//
// Feature bits in name server FC-4 Features object.
//

