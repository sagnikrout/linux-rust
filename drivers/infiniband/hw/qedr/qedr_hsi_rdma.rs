//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/qedr/qedr_hsi_rdma.h
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


// QLogic qedr NIC Driver
// Copyright (c) 2015-2016  QLogic Corporation
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and /or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// rdma completion notification queue element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cnqe {
    pub cq_handle: regpair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cqe_responder {
    pub srq_wr_id: regpair,
    pub qp_handle: regpair,
    pub imm_data_or_inv_r_Key: __le32,
    pub length: __le32,
    pub imm_data_hi: __le32,
    pub rq_cons_or_srq_id: __le16,
    pub flags: u8,
pub const RDMA_CQE_RESPONDER_TOGGLE_BIT_MASK: c_uint = 0x1;
pub const RDMA_CQE_RESPONDER_TOGGLE_BIT_SHIFT: c_int = 0;
pub const RDMA_CQE_RESPONDER_TYPE_MASK: c_uint = 0x3;
pub const RDMA_CQE_RESPONDER_TYPE_SHIFT: c_int = 1;
pub const RDMA_CQE_RESPONDER_INV_FLG_MASK: c_uint = 0x1;
pub const RDMA_CQE_RESPONDER_INV_FLG_SHIFT: c_int = 3;
pub const RDMA_CQE_RESPONDER_IMM_FLG_MASK: c_uint = 0x1;
pub const RDMA_CQE_RESPONDER_IMM_FLG_SHIFT: c_int = 4;
pub const RDMA_CQE_RESPONDER_RDMA_FLG_MASK: c_uint = 0x1;
pub const RDMA_CQE_RESPONDER_RDMA_FLG_SHIFT: c_int = 5;
pub const RDMA_CQE_RESPONDER_RESERVED2_MASK: c_uint = 0x3;
pub const RDMA_CQE_RESPONDER_RESERVED2_SHIFT: c_int = 6;
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cqe_requester {
    pub sq_cons: __le16,
    pub reserved0: __le16,
    pub reserved1: __le32,
    pub qp_handle: regpair,
    pub reserved2: regpair,
    pub reserved3: __le32,
    pub reserved4: __le16,
    pub flags: u8,
pub const RDMA_CQE_REQUESTER_TOGGLE_BIT_MASK: c_uint = 0x1;
pub const RDMA_CQE_REQUESTER_TOGGLE_BIT_SHIFT: c_int = 0;
pub const RDMA_CQE_REQUESTER_TYPE_MASK: c_uint = 0x3;
pub const RDMA_CQE_REQUESTER_TYPE_SHIFT: c_int = 1;
pub const RDMA_CQE_REQUESTER_RESERVED5_MASK: c_uint = 0x1F;
pub const RDMA_CQE_REQUESTER_RESERVED5_SHIFT: c_int = 3;
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cqe_common {
    pub reserved0: regpair,
    pub qp_handle: regpair,
    pub reserved1: [__le16; 7],
    pub flags: u8,
pub const RDMA_CQE_COMMON_TOGGLE_BIT_MASK: c_uint = 0x1;
pub const RDMA_CQE_COMMON_TOGGLE_BIT_SHIFT: c_int = 0;
pub const RDMA_CQE_COMMON_TYPE_MASK: c_uint = 0x3;
pub const RDMA_CQE_COMMON_TYPE_SHIFT: c_int = 1;
pub const RDMA_CQE_COMMON_RESERVED2_MASK: c_uint = 0x1F;
pub const RDMA_CQE_COMMON_RESERVED2_SHIFT: c_int = 3;
    pub status: u8,
}

// rdma completion queue element
#[repr(C)]
#[derive(Copy, Clone)]
pub union rdma_cqe {
    pub resp: rdma_cqe_responder,
    pub req: rdma_cqe_requester,
    pub cmn: rdma_cqe_common,
}

// * CQE requester status enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_cqe_requester_status_enum {
    RDMA_CQE_REQ_STS_OK,
    RDMA_CQE_REQ_STS_BAD_RESPONSE_ERR,
    RDMA_CQE_REQ_STS_LOCAL_LENGTH_ERR,
    RDMA_CQE_REQ_STS_LOCAL_QP_OPERATION_ERR,
    RDMA_CQE_REQ_STS_LOCAL_PROTECTION_ERR,
    RDMA_CQE_REQ_STS_MEMORY_MGT_OPERATION_ERR,
    RDMA_CQE_REQ_STS_REMOTE_INVALID_REQUEST_ERR,
    RDMA_CQE_REQ_STS_REMOTE_ACCESS_ERR,
    RDMA_CQE_REQ_STS_REMOTE_OPERATION_ERR,
    RDMA_CQE_REQ_STS_RNR_NAK_RETRY_CNT_ERR,
    RDMA_CQE_REQ_STS_TRANSPORT_RETRY_CNT_ERR,
    RDMA_CQE_REQ_STS_WORK_REQUEST_FLUSHED_ERR,
    RDMA_CQE_REQ_STS_XRC_VOILATION_ERR,
    RDMA_CQE_REQ_STS_SIG_ERR,
    MAX_RDMA_CQE_REQUESTER_STATUS_ENUM
}

// CQE responder status enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_cqe_responder_status_enum {
    RDMA_CQE_RESP_STS_OK,
    RDMA_CQE_RESP_STS_LOCAL_ACCESS_ERR,
    RDMA_CQE_RESP_STS_LOCAL_LENGTH_ERR,
    RDMA_CQE_RESP_STS_LOCAL_QP_OPERATION_ERR,
    RDMA_CQE_RESP_STS_LOCAL_PROTECTION_ERR,
    RDMA_CQE_RESP_STS_MEMORY_MGT_OPERATION_ERR,
    RDMA_CQE_RESP_STS_REMOTE_INVALID_REQUEST_ERR,
    RDMA_CQE_RESP_STS_WORK_REQUEST_FLUSHED_ERR,
    MAX_RDMA_CQE_RESPONDER_STATUS_ENUM
}

// CQE type enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_cqe_type {
    RDMA_CQE_TYPE_REQUESTER,
    RDMA_CQE_TYPE_RESPONDER_RQ,
    RDMA_CQE_TYPE_RESPONDER_SRQ,
    RDMA_CQE_TYPE_RESPONDER_XRC_SRQ,
    RDMA_CQE_TYPE_INVALID,
    MAX_RDMA_CQE_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_sge {
    pub length: __le32,
    pub addr: regpair,
    pub l_key: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_rq_sge {
    pub addr: regpair,
    pub length: __le32,
    pub flags: __le32,
pub const RDMA_RQ_SGE_L_KEY_LO_MASK: c_uint = 0x3FFFFFF;
pub const RDMA_RQ_SGE_L_KEY_LO_SHIFT: c_int = 0;
pub const RDMA_RQ_SGE_NUM_SGES_MASK: c_uint = 0x7;
pub const RDMA_RQ_SGE_NUM_SGES_SHIFT: c_int = 26;
pub const RDMA_RQ_SGE_L_KEY_HI_MASK: c_uint = 0x7;
pub const RDMA_RQ_SGE_L_KEY_HI_SHIFT: c_int = 29;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_srq_wqe_header {
    pub wr_id: regpair,
    pub /: *mut *mut u8 num_sges / number of SGEs in WQE,
    pub reserved2: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_srq_sge {
    pub addr: regpair,
    pub length: __le32,
    pub l_key: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rdma_srq_elm {
    pub header: rdma_srq_wqe_header,
    pub sge: rdma_srq_sge,
}

// Rdma doorbell data for flags update
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_pwm_flags_data {
    pub /: *mut *mut __le16 icid; / internal CID,
    pub /: *mut *mut u8 agg_flags; / aggregative flags,
    pub reserved: u8,
}

// Rdma doorbell data for SQ and RQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_pwm_val16_data {
    pub icid: __le16,
    pub value: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rdma_pwm_val16_data_union {
    pub as_struct: rdma_pwm_val16_data,
    pub as_dword: __le32,
}

// Rdma doorbell data for CQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_pwm_val32_data {
    pub icid: __le16,
    pub agg_flags: u8,
    pub params: u8,
pub const RDMA_PWM_VAL32_DATA_AGG_CMD_MASK: c_uint = 0x3;
pub const RDMA_PWM_VAL32_DATA_AGG_CMD_SHIFT: c_int = 0;
pub const RDMA_PWM_VAL32_DATA_BYPASS_EN_MASK: c_uint = 0x1;
pub const RDMA_PWM_VAL32_DATA_BYPASS_EN_SHIFT: c_int = 2;
pub const RDMA_PWM_VAL32_DATA_CONN_TYPE_IS_IWARP_MASK: c_uint = 0x1;
pub const RDMA_PWM_VAL32_DATA_CONN_TYPE_IS_IWARP_SHIFT: c_int = 3;
pub const RDMA_PWM_VAL32_DATA_SET_16B_VAL_MASK: c_uint = 0x1;
pub const RDMA_PWM_VAL32_DATA_SET_16B_VAL_SHIFT: c_int = 4;
pub const RDMA_PWM_VAL32_DATA_RESERVED_MASK: c_uint = 0x7;
pub const RDMA_PWM_VAL32_DATA_RESERVED_SHIFT: c_int = 5;
    pub value: __le32,
}

// DIF Block size options
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_dif_block_size {
    RDMA_DIF_BLOCK_512 = 0,
    RDMA_DIF_BLOCK_4096 = 1,
    MAX_RDMA_DIF_BLOCK_SIZE
}

// DIF CRC initial value
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_dif_crc_seed {
    RDMA_DIF_CRC_SEED_0000 = 0,
    RDMA_DIF_CRC_SEED_FFFF = 1,
    MAX_RDMA_DIF_CRC_SEED
}

// RDMA DIF Error Result Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_dif_error_result {
    pub error_intervals: __le32,
    pub dif_error_1st_interval: __le32,
    pub flags: u8,
pub const RDMA_DIF_ERROR_RESULT_DIF_ERROR_TYPE_CRC_MASK: c_uint = 0x1;
pub const RDMA_DIF_ERROR_RESULT_DIF_ERROR_TYPE_CRC_SHIFT: c_int = 0;
pub const RDMA_DIF_ERROR_RESULT_DIF_ERROR_TYPE_APP_TAG_MASK: c_uint = 0x1;
pub const RDMA_DIF_ERROR_RESULT_DIF_ERROR_TYPE_APP_TAG_SHIFT: c_int = 1;
pub const RDMA_DIF_ERROR_RESULT_DIF_ERROR_TYPE_REF_TAG_MASK: c_uint = 0x1;
pub const RDMA_DIF_ERROR_RESULT_DIF_ERROR_TYPE_REF_TAG_SHIFT: c_int = 2;
pub const RDMA_DIF_ERROR_RESULT_RESERVED0_MASK: c_uint = 0xF;
pub const RDMA_DIF_ERROR_RESULT_RESERVED0_SHIFT: c_int = 3;
pub const RDMA_DIF_ERROR_RESULT_TOGGLE_BIT_MASK: c_uint = 0x1;
pub const RDMA_DIF_ERROR_RESULT_TOGGLE_BIT_SHIFT: c_int = 7;
    pub reserved1: [u8; 55],
}

// DIF IO direction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_dif_io_direction_flg {
    RDMA_DIF_DIR_RX = 0,
    RDMA_DIF_DIR_TX = 1,
    MAX_RDMA_DIF_IO_DIRECTION_FLG
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_dif_params {
    pub base_ref_tag: __le32,
    pub app_tag: __le16,
    pub app_tag_mask: __le16,
    pub runt_crc_value: __le16,
    pub flags: __le16,
pub const RDMA_DIF_PARAMS_IO_DIRECTION_FLG_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_IO_DIRECTION_FLG_SHIFT: c_int = 0;
pub const RDMA_DIF_PARAMS_BLOCK_SIZE_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_BLOCK_SIZE_SHIFT: c_int = 1;
pub const RDMA_DIF_PARAMS_RUNT_VALID_FLG_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_RUNT_VALID_FLG_SHIFT: c_int = 2;
pub const RDMA_DIF_PARAMS_VALIDATE_CRC_GUARD_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_VALIDATE_CRC_GUARD_SHIFT: c_int = 3;
pub const RDMA_DIF_PARAMS_VALIDATE_REF_TAG_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_VALIDATE_REF_TAG_SHIFT: c_int = 4;
pub const RDMA_DIF_PARAMS_VALIDATE_APP_TAG_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_VALIDATE_APP_TAG_SHIFT: c_int = 5;
pub const RDMA_DIF_PARAMS_CRC_SEED_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_CRC_SEED_SHIFT: c_int = 6;
pub const RDMA_DIF_PARAMS_RX_REF_TAG_CONST_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_RX_REF_TAG_CONST_SHIFT: c_int = 7;
pub const RDMA_DIF_PARAMS_BLOCK_GUARD_TYPE_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_BLOCK_GUARD_TYPE_SHIFT: c_int = 8;
pub const RDMA_DIF_PARAMS_APP_ESCAPE_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_APP_ESCAPE_SHIFT: c_int = 9;
pub const RDMA_DIF_PARAMS_REF_ESCAPE_MASK: c_uint = 0x1;
pub const RDMA_DIF_PARAMS_REF_ESCAPE_SHIFT: c_int = 10;
pub const RDMA_DIF_PARAMS_RESERVED4_MASK: c_uint = 0x1F;
pub const RDMA_DIF_PARAMS_RESERVED4_SHIFT: c_int = 11;
    pub reserved5: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_atomic_wqe {
    pub reserved1: __le32,
    pub length: __le32,
    pub xrc_srq: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_ATOMIC_WQE_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_ATOMIC_WQE_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_ATOMIC_WQE_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_ATOMIC_WQE_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_ATOMIC_WQE_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_ATOMIC_WQE_DIF_ON_HOST_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_DIF_ON_HOST_FLG_SHIFT: c_int = 5;
pub const RDMA_SQ_ATOMIC_WQE_RESERVED0_MASK: c_uint = 0x3;
pub const RDMA_SQ_ATOMIC_WQE_RESERVED0_SHIFT: c_int = 6;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
    pub remote_va: regpair,
    pub r_key: __le32,
    pub reserved2: __le32,
    pub cmp_data: regpair,
    pub swap_data: regpair,
}

// First element (16 bytes) of atomic wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_atomic_wqe_1st {
    pub reserved1: __le32,
    pub length: __le32,
    pub xrc_srq: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_ATOMIC_WQE_1ST_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_1ST_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_ATOMIC_WQE_1ST_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_1ST_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_ATOMIC_WQE_1ST_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_1ST_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_ATOMIC_WQE_1ST_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_1ST_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_ATOMIC_WQE_1ST_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_ATOMIC_WQE_1ST_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_ATOMIC_WQE_1ST_RESERVED0_MASK: c_uint = 0x7;
pub const RDMA_SQ_ATOMIC_WQE_1ST_RESERVED0_SHIFT: c_int = 5;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
}

// Second element (16 bytes) of atomic wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_atomic_wqe_2nd {
    pub remote_va: regpair,
    pub r_key: __le32,
    pub reserved2: __le32,
}

// Third element (16 bytes) of atomic wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_atomic_wqe_3rd {
    pub cmp_data: regpair,
    pub swap_data: regpair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_bind_wqe {
    pub addr: regpair,
    pub l_key: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_BIND_WQE_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_BIND_WQE_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_BIND_WQE_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_BIND_WQE_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_BIND_WQE_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_BIND_WQE_DIF_ON_HOST_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_DIF_ON_HOST_FLG_SHIFT: c_int = 5;
pub const RDMA_SQ_BIND_WQE_RESERVED0_MASK: c_uint = 0x3;
pub const RDMA_SQ_BIND_WQE_RESERVED0_SHIFT: c_int = 6;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
    pub bind_ctrl: u8,
pub const RDMA_SQ_BIND_WQE_ZERO_BASED_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_ZERO_BASED_SHIFT: c_int = 0;
pub const RDMA_SQ_BIND_WQE_RESERVED1_MASK: c_uint = 0x7F;
pub const RDMA_SQ_BIND_WQE_RESERVED1_SHIFT: c_int = 1;
    pub access_ctrl: u8,
pub const RDMA_SQ_BIND_WQE_REMOTE_READ_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_REMOTE_READ_SHIFT: c_int = 0;
pub const RDMA_SQ_BIND_WQE_REMOTE_WRITE_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_REMOTE_WRITE_SHIFT: c_int = 1;
pub const RDMA_SQ_BIND_WQE_ENABLE_ATOMIC_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_ENABLE_ATOMIC_SHIFT: c_int = 2;
pub const RDMA_SQ_BIND_WQE_LOCAL_READ_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_LOCAL_READ_SHIFT: c_int = 3;
pub const RDMA_SQ_BIND_WQE_LOCAL_WRITE_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_LOCAL_WRITE_SHIFT: c_int = 4;
pub const RDMA_SQ_BIND_WQE_RESERVED2_MASK: c_uint = 0x7;
pub const RDMA_SQ_BIND_WQE_RESERVED2_SHIFT: c_int = 5;
    pub reserved3: u8,
    pub length_hi: u8,
    pub length_lo: __le32,
    pub parent_l_key: __le32,
    pub reserved4: __le32,
    pub dif_params: rdma_dif_params,
}

// First element (16 bytes) of bind wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_bind_wqe_1st {
    pub addr: regpair,
    pub l_key: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_BIND_WQE_1ST_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_1ST_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_BIND_WQE_1ST_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_1ST_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_BIND_WQE_1ST_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_1ST_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_BIND_WQE_1ST_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_1ST_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_BIND_WQE_1ST_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_1ST_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_BIND_WQE_1ST_RESERVED0_MASK: c_uint = 0x7;
pub const RDMA_SQ_BIND_WQE_1ST_RESERVED0_SHIFT: c_int = 5;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
}

// Second element (16 bytes) of bind wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_bind_wqe_2nd {
    pub bind_ctrl: u8,
pub const RDMA_SQ_BIND_WQE_2ND_ZERO_BASED_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_2ND_ZERO_BASED_SHIFT: c_int = 0;
pub const RDMA_SQ_BIND_WQE_2ND_RESERVED1_MASK: c_uint = 0x7F;
pub const RDMA_SQ_BIND_WQE_2ND_RESERVED1_SHIFT: c_int = 1;
    pub access_ctrl: u8,
pub const RDMA_SQ_BIND_WQE_2ND_REMOTE_READ_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_2ND_REMOTE_READ_SHIFT: c_int = 0;
pub const RDMA_SQ_BIND_WQE_2ND_REMOTE_WRITE_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_2ND_REMOTE_WRITE_SHIFT: c_int = 1;
pub const RDMA_SQ_BIND_WQE_2ND_ENABLE_ATOMIC_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_2ND_ENABLE_ATOMIC_SHIFT: c_int = 2;
pub const RDMA_SQ_BIND_WQE_2ND_LOCAL_READ_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_2ND_LOCAL_READ_SHIFT: c_int = 3;
pub const RDMA_SQ_BIND_WQE_2ND_LOCAL_WRITE_MASK: c_uint = 0x1;
pub const RDMA_SQ_BIND_WQE_2ND_LOCAL_WRITE_SHIFT: c_int = 4;
pub const RDMA_SQ_BIND_WQE_2ND_RESERVED2_MASK: c_uint = 0x7;
pub const RDMA_SQ_BIND_WQE_2ND_RESERVED2_SHIFT: c_int = 5;
    pub reserved3: u8,
    pub length_hi: u8,
    pub length_lo: __le32,
    pub parent_l_key: __le32,
    pub reserved4: __le32,
}

// Third element (16 bytes) of bind wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_bind_wqe_3rd {
    pub dif_params: rdma_dif_params,
}

// Structure with only the SQ WQE common
// fields. Size is of one SQ element (16B)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_common_wqe {
    pub reserved1: [__le32; 3],
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_COMMON_WQE_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_COMMON_WQE_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_COMMON_WQE_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_COMMON_WQE_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_COMMON_WQE_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_COMMON_WQE_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_COMMON_WQE_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_COMMON_WQE_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_COMMON_WQE_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_COMMON_WQE_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_COMMON_WQE_RESERVED0_MASK: c_uint = 0x7;
pub const RDMA_SQ_COMMON_WQE_RESERVED0_SHIFT: c_int = 5;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_fmr_wqe {
    pub addr: regpair,
    pub l_key: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_FMR_WQE_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_FMR_WQE_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_FMR_WQE_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_FMR_WQE_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_FMR_WQE_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_FMR_WQE_DIF_ON_HOST_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_DIF_ON_HOST_FLG_SHIFT: c_int = 5;
pub const RDMA_SQ_FMR_WQE_RESERVED0_MASK: c_uint = 0x3;
pub const RDMA_SQ_FMR_WQE_RESERVED0_SHIFT: c_int = 6;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
    pub fmr_ctrl: u8,
pub const RDMA_SQ_FMR_WQE_PAGE_SIZE_LOG_MASK: c_uint = 0x1F;
pub const RDMA_SQ_FMR_WQE_PAGE_SIZE_LOG_SHIFT: c_int = 0;
pub const RDMA_SQ_FMR_WQE_ZERO_BASED_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_ZERO_BASED_SHIFT: c_int = 5;
pub const RDMA_SQ_FMR_WQE_BIND_EN_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_BIND_EN_SHIFT: c_int = 6;
pub const RDMA_SQ_FMR_WQE_RESERVED1_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_RESERVED1_SHIFT: c_int = 7;
    pub access_ctrl: u8,
pub const RDMA_SQ_FMR_WQE_REMOTE_READ_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_REMOTE_READ_SHIFT: c_int = 0;
pub const RDMA_SQ_FMR_WQE_REMOTE_WRITE_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_REMOTE_WRITE_SHIFT: c_int = 1;
pub const RDMA_SQ_FMR_WQE_ENABLE_ATOMIC_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_ENABLE_ATOMIC_SHIFT: c_int = 2;
pub const RDMA_SQ_FMR_WQE_LOCAL_READ_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_LOCAL_READ_SHIFT: c_int = 3;
pub const RDMA_SQ_FMR_WQE_LOCAL_WRITE_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_LOCAL_WRITE_SHIFT: c_int = 4;
pub const RDMA_SQ_FMR_WQE_RESERVED2_MASK: c_uint = 0x7;
pub const RDMA_SQ_FMR_WQE_RESERVED2_SHIFT: c_int = 5;
    pub reserved3: u8,
    pub length_hi: u8,
    pub length_lo: __le32,
    pub pbl_addr: regpair,
}

// First element (16 bytes) of fmr wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_fmr_wqe_1st {
    pub addr: regpair,
    pub l_key: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_FMR_WQE_1ST_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_1ST_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_FMR_WQE_1ST_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_1ST_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_FMR_WQE_1ST_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_1ST_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_FMR_WQE_1ST_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_1ST_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_FMR_WQE_1ST_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_1ST_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_FMR_WQE_1ST_DIF_ON_HOST_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_1ST_DIF_ON_HOST_FLG_SHIFT: c_int = 5;
pub const RDMA_SQ_FMR_WQE_1ST_RESERVED0_MASK: c_uint = 0x3;
pub const RDMA_SQ_FMR_WQE_1ST_RESERVED0_SHIFT: c_int = 6;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
}

// Second element (16 bytes) of fmr wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_fmr_wqe_2nd {
    pub fmr_ctrl: u8,
pub const RDMA_SQ_FMR_WQE_2ND_PAGE_SIZE_LOG_MASK: c_uint = 0x1F;
pub const RDMA_SQ_FMR_WQE_2ND_PAGE_SIZE_LOG_SHIFT: c_int = 0;
pub const RDMA_SQ_FMR_WQE_2ND_ZERO_BASED_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_2ND_ZERO_BASED_SHIFT: c_int = 5;
pub const RDMA_SQ_FMR_WQE_2ND_BIND_EN_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_2ND_BIND_EN_SHIFT: c_int = 6;
pub const RDMA_SQ_FMR_WQE_2ND_RESERVED1_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_2ND_RESERVED1_SHIFT: c_int = 7;
    pub access_ctrl: u8,
pub const RDMA_SQ_FMR_WQE_2ND_REMOTE_READ_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_2ND_REMOTE_READ_SHIFT: c_int = 0;
pub const RDMA_SQ_FMR_WQE_2ND_REMOTE_WRITE_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_2ND_REMOTE_WRITE_SHIFT: c_int = 1;
pub const RDMA_SQ_FMR_WQE_2ND_ENABLE_ATOMIC_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_2ND_ENABLE_ATOMIC_SHIFT: c_int = 2;
pub const RDMA_SQ_FMR_WQE_2ND_LOCAL_READ_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_2ND_LOCAL_READ_SHIFT: c_int = 3;
pub const RDMA_SQ_FMR_WQE_2ND_LOCAL_WRITE_MASK: c_uint = 0x1;
pub const RDMA_SQ_FMR_WQE_2ND_LOCAL_WRITE_SHIFT: c_int = 4;
pub const RDMA_SQ_FMR_WQE_2ND_RESERVED2_MASK: c_uint = 0x7;
pub const RDMA_SQ_FMR_WQE_2ND_RESERVED2_SHIFT: c_int = 5;
    pub reserved3: u8,
    pub length_hi: u8,
    pub length_lo: __le32,
    pub pbl_addr: regpair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_local_inv_wqe {
    pub reserved: regpair,
    pub inv_l_key: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_LOCAL_INV_WQE_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_LOCAL_INV_WQE_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_LOCAL_INV_WQE_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_LOCAL_INV_WQE_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_LOCAL_INV_WQE_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_LOCAL_INV_WQE_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_LOCAL_INV_WQE_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_LOCAL_INV_WQE_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_LOCAL_INV_WQE_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_LOCAL_INV_WQE_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_LOCAL_INV_WQE_DIF_ON_HOST_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_LOCAL_INV_WQE_DIF_ON_HOST_FLG_SHIFT: c_int = 5;
pub const RDMA_SQ_LOCAL_INV_WQE_RESERVED0_MASK: c_uint = 0x3;
pub const RDMA_SQ_LOCAL_INV_WQE_RESERVED0_SHIFT: c_int = 6;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_rdma_wqe {
    pub imm_data: __le32,
    pub length: __le32,
    pub xrc_srq: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_RDMA_WQE_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_RDMA_WQE_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_RDMA_WQE_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_RDMA_WQE_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_RDMA_WQE_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_RDMA_WQE_DIF_ON_HOST_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_DIF_ON_HOST_FLG_SHIFT: c_int = 5;
pub const RDMA_SQ_RDMA_WQE_READ_INV_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_READ_INV_FLG_SHIFT: c_int = 6;
pub const RDMA_SQ_RDMA_WQE_RESERVED1_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_RESERVED1_SHIFT: c_int = 7;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
    pub remote_va: regpair,
    pub r_key: __le32,
    pub dif_flags: u8,
pub const RDMA_SQ_RDMA_WQE_DIF_BLOCK_SIZE_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_DIF_BLOCK_SIZE_SHIFT: c_int = 0;
pub const RDMA_SQ_RDMA_WQE_RESERVED2_MASK: c_uint = 0x7F;
pub const RDMA_SQ_RDMA_WQE_RESERVED2_SHIFT: c_int = 1;
    pub reserved3: [u8; 3],
}

// First element (16 bytes) of rdma wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_rdma_wqe_1st {
    pub imm_data: __le32,
    pub length: __le32,
    pub xrc_srq: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_RDMA_WQE_1ST_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_1ST_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_RDMA_WQE_1ST_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_1ST_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_RDMA_WQE_1ST_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_1ST_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_RDMA_WQE_1ST_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_1ST_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_RDMA_WQE_1ST_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_1ST_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_RDMA_WQE_1ST_DIF_ON_HOST_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_1ST_DIF_ON_HOST_FLG_SHIFT: c_int = 5;
pub const RDMA_SQ_RDMA_WQE_1ST_READ_INV_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_1ST_READ_INV_FLG_SHIFT: c_int = 6;
pub const RDMA_SQ_RDMA_WQE_1ST_RESERVED0_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_1ST_RESERVED0_SHIFT: c_int = 7;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
}

// Second element (16 bytes) of rdma wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_rdma_wqe_2nd {
    pub remote_va: regpair,
    pub r_key: __le32,
    pub dif_flags: u8,
pub const RDMA_SQ_RDMA_WQE_2ND_DIF_BLOCK_SIZE_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_2ND_DIF_BLOCK_SIZE_SHIFT: c_int = 0;
pub const RDMA_SQ_RDMA_WQE_2ND_DIF_FIRST_SEGMENT_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_2ND_DIF_FIRST_SEGMENT_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_RDMA_WQE_2ND_DIF_LAST_SEGMENT_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_RDMA_WQE_2ND_DIF_LAST_SEGMENT_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_RDMA_WQE_2ND_RESERVED1_MASK: c_uint = 0x1F;
pub const RDMA_SQ_RDMA_WQE_2ND_RESERVED1_SHIFT: c_int = 3;
    pub reserved2: [u8; 3],
}

// SQ WQE req type enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_sq_req_type {
    RDMA_SQ_REQ_TYPE_SEND,
    RDMA_SQ_REQ_TYPE_SEND_WITH_IMM,
    RDMA_SQ_REQ_TYPE_SEND_WITH_INVALIDATE,
    RDMA_SQ_REQ_TYPE_RDMA_WR,
    RDMA_SQ_REQ_TYPE_RDMA_WR_WITH_IMM,
    RDMA_SQ_REQ_TYPE_RDMA_RD,
    RDMA_SQ_REQ_TYPE_ATOMIC_CMP_AND_SWAP,
    RDMA_SQ_REQ_TYPE_ATOMIC_ADD,
    RDMA_SQ_REQ_TYPE_LOCAL_INVALIDATE,
    RDMA_SQ_REQ_TYPE_FAST_MR,
    RDMA_SQ_REQ_TYPE_BIND,
    RDMA_SQ_REQ_TYPE_INVALID,
    MAX_RDMA_SQ_REQ_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_send_wqe {
    pub inv_key_or_imm_data: __le32,
    pub length: __le32,
    pub xrc_srq: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_SEND_WQE_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_SEND_WQE_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_SEND_WQE_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_SEND_WQE_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_SEND_WQE_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_SEND_WQE_DIF_ON_HOST_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_DIF_ON_HOST_FLG_SHIFT: c_int = 5;
pub const RDMA_SQ_SEND_WQE_RESERVED0_MASK: c_uint = 0x3;
pub const RDMA_SQ_SEND_WQE_RESERVED0_SHIFT: c_int = 6;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
    pub reserved1: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_send_wqe_1st {
    pub inv_key_or_imm_data: __le32,
    pub length: __le32,
    pub xrc_srq: __le32,
    pub req_type: u8,
    pub flags: u8,
pub const RDMA_SQ_SEND_WQE_1ST_COMP_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_1ST_COMP_FLG_SHIFT: c_int = 0;
pub const RDMA_SQ_SEND_WQE_1ST_RD_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_1ST_RD_FENCE_FLG_SHIFT: c_int = 1;
pub const RDMA_SQ_SEND_WQE_1ST_INV_FENCE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_1ST_INV_FENCE_FLG_SHIFT: c_int = 2;
pub const RDMA_SQ_SEND_WQE_1ST_SE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_1ST_SE_FLG_SHIFT: c_int = 3;
pub const RDMA_SQ_SEND_WQE_1ST_INLINE_FLG_MASK: c_uint = 0x1;
pub const RDMA_SQ_SEND_WQE_1ST_INLINE_FLG_SHIFT: c_int = 4;
pub const RDMA_SQ_SEND_WQE_1ST_RESERVED0_MASK: c_uint = 0x7;
pub const RDMA_SQ_SEND_WQE_1ST_RESERVED0_SHIFT: c_int = 5;
    pub wqe_size: u8,
    pub prev_wqe_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sq_send_wqe_2st {
    pub reserved1: [__le32; 4],
}
