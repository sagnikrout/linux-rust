//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_fw.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

pub const ICP_QAT_FW_REQ_DEFAULT_SZ: c_int = 128;
pub const ICP_QAT_FW_RESP_DEFAULT_SZ: c_int = 32;
pub const ICP_QAT_FW_COMN_ONE_BYTE_SHIFT: c_int = 8;
pub const ICP_QAT_FW_COMN_SINGLE_BYTE_MASK: c_uint = 0xFF;
pub const ICP_QAT_FW_NUM_LONGWORDS_1: c_int = 1;
pub const ICP_QAT_FW_NUM_LONGWORDS_2: c_int = 2;
pub const ICP_QAT_FW_NUM_LONGWORDS_3: c_int = 3;
pub const ICP_QAT_FW_NUM_LONGWORDS_4: c_int = 4;
pub const ICP_QAT_FW_NUM_LONGWORDS_5: c_int = 5;
pub const ICP_QAT_FW_NUM_LONGWORDS_6: c_int = 6;
pub const ICP_QAT_FW_NUM_LONGWORDS_7: c_int = 7;
pub const ICP_QAT_FW_NUM_LONGWORDS_10: c_int = 10;
pub const ICP_QAT_FW_NUM_LONGWORDS_13: c_int = 13;
pub const ICP_QAT_FW_NULL_REQ_SERV_ID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_fw_comn_resp_serv_id {
    ICP_QAT_FW_COMN_RESP_SERV_NULL,
    ICP_QAT_FW_COMN_RESP_SERV_CPM_FW,
    ICP_QAT_FW_COMN_RESP_SERV_DELIMITER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_fw_comn_request_id {
    ICP_QAT_FW_COMN_REQ_NULL = 0,
    ICP_QAT_FW_COMN_REQ_CPM_FW_PKE = 3,
    ICP_QAT_FW_COMN_REQ_CPM_FW_LA = 4,
    ICP_QAT_FW_COMN_REQ_CPM_FW_DMA = 7,
    ICP_QAT_FW_COMN_REQ_CPM_FW_COMP = 9,
    ICP_QAT_FW_COMN_REQ_DELIMITER
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_req_hdr_cd_pars {
    pub content_desc_addr: __u64,
    pub content_desc_resrvd1: __u16,
    pub content_desc_params_sz: __u8,
    pub content_desc_hdr_resrvd2: __u8,
    pub content_desc_resrvd3: __u32,
    pub s: },
    pub serv_specif_fields: [__u32; 4],
    pub s1: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_req_mid {
    pub opaque_data: __u64,
    pub src_data_addr: __u64,
    pub dest_data_addr: __u64,
    pub src_length: __u32,
    pub dst_length: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_req_cd_ctrl {
    pub content_desc_ctrl_lw: [__u32; ICP_QAT_FW_NUM_LONGWORDS_5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_req_hdr {
    pub resrvd1: __u8,
    pub service_cmd_id: __u8,
    pub service_type: __u8,
    pub hdr_flags: __u8,
    pub serv_specif_flags: __u16,
    pub comn_req_flags: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_req_rqpars {
    pub serv_specif_rqpars_lw: [__u32; ICP_QAT_FW_NUM_LONGWORDS_13],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_req {
    pub comn_hdr: icp_qat_fw_comn_req_hdr,
    pub cd_pars: icp_qat_fw_comn_req_hdr_cd_pars,
    pub comn_mid: icp_qat_fw_comn_req_mid,
    pub serv_specif_rqpars: icp_qat_fw_comn_req_rqpars,
    pub cd_ctrl: icp_qat_fw_comn_req_cd_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_error {
    pub xlat_err_code: __u8,
    pub cmp_err_code: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_resp_hdr {
    pub resrvd1: __u8,
    pub service_id: __u8,
    pub response_type: __u8,
    pub hdr_flags: __u8,
    pub comn_error: icp_qat_fw_comn_error,
    pub comn_status: __u8,
    pub cmd_id: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_resp {
    pub comn_hdr: icp_qat_fw_comn_resp_hdr,
    pub opaque_data: __u64,
    pub resrvd: [__u32; ICP_QAT_FW_NUM_LONGWORDS_4],
}

pub const ICP_QAT_FW_COMN_REQ_FLAG_SET: c_int = 1;
pub const ICP_QAT_FW_COMN_REQ_FLAG_CLR: c_int = 0;
pub const ICP_QAT_FW_COMN_VALID_FLAG_BITPOS: c_int = 7;
pub const ICP_QAT_FW_COMN_VALID_FLAG_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMN_HDR_RESRVD_FLD_MASK: c_uint = 0x7F;
pub const ICP_QAT_FW_COMN_CNV_FLAG_BITPOS: c_int = 6;
pub const ICP_QAT_FW_COMN_CNV_FLAG_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMN_CNVNR_FLAG_BITPOS: c_int = 5;
pub const ICP_QAT_FW_COMN_CNVNR_FLAG_MASK: c_uint = 0x1;

pub const ICP_QAT_FW_COMN_ST_BLK_FLAG_BITPOS: c_int = 4;
pub const ICP_QAT_FW_COMN_ST_BLK_FLAG_MASK: c_uint = 0x1;

pub const QAT_COMN_PTR_TYPE_BITPOS: c_int = 0;
pub const QAT_COMN_PTR_TYPE_MASK: c_uint = 0x1;
pub const QAT_COMN_CD_FLD_TYPE_BITPOS: c_int = 1;
pub const QAT_COMN_CD_FLD_TYPE_MASK: c_uint = 0x1;
pub const QAT_COMN_PTR_TYPE_FLAT: c_uint = 0x0;
pub const QAT_COMN_PTR_TYPE_SGL: c_uint = 0x1;
pub const QAT_COMN_CD_FLD_TYPE_64BIT_ADR: c_uint = 0x0;
pub const QAT_COMN_CD_FLD_TYPE_16BYTE_DATA: c_uint = 0x1;

pub const ICP_QAT_FW_COMN_NEXT_ID_BITPOS: c_int = 4;
pub const ICP_QAT_FW_COMN_NEXT_ID_MASK: c_uint = 0xF0;
pub const ICP_QAT_FW_COMN_CURR_ID_BITPOS: c_int = 0;
pub const ICP_QAT_FW_COMN_CURR_ID_MASK: c_uint = 0x0F;

pub const QAT_COMN_RESP_CRYPTO_STATUS_BITPOS: c_int = 7;
pub const QAT_COMN_RESP_CRYPTO_STATUS_MASK: c_uint = 0x1;
pub const QAT_COMN_RESP_PKE_STATUS_BITPOS: c_int = 6;
pub const QAT_COMN_RESP_PKE_STATUS_MASK: c_uint = 0x1;
pub const QAT_COMN_RESP_CMP_STATUS_BITPOS: c_int = 5;
pub const QAT_COMN_RESP_CMP_STATUS_MASK: c_uint = 0x1;
pub const QAT_COMN_RESP_XLAT_STATUS_BITPOS: c_int = 4;
pub const QAT_COMN_RESP_XLAT_STATUS_MASK: c_uint = 0x1;
pub const QAT_COMN_RESP_CMP_END_OF_LAST_BLK_BITPOS: c_int = 3;
pub const QAT_COMN_RESP_CMP_END_OF_LAST_BLK_MASK: c_uint = 0x1;

pub const ICP_QAT_FW_COMN_STATUS_FLAG_OK: c_int = 0;
pub const ICP_QAT_FW_COMN_STATUS_FLAG_ERROR: c_int = 1;
pub const ICP_QAT_FW_COMN_STATUS_CMP_END_OF_LAST_BLK_FLAG_CLR: c_int = 0;
pub const ICP_QAT_FW_COMN_STATUS_CMP_END_OF_LAST_BLK_FLAG_SET: c_int = 1;
pub const ERR_CODE_NO_ERROR: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_fw_slice {
    ICP_QAT_FW_SLICE_NULL = 0,
    ICP_QAT_FW_SLICE_CIPHER = 1,
    ICP_QAT_FW_SLICE_AUTH = 2,
    ICP_QAT_FW_SLICE_DRAM_RD = 3,
    ICP_QAT_FW_SLICE_DRAM_WR = 4,
    ICP_QAT_FW_SLICE_COMP = 5,
    ICP_QAT_FW_SLICE_XLAT = 6,
    ICP_QAT_FW_SLICE_DELIMITER
}
