//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_fw_la.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_fw_la_cmd_id {
    ICP_QAT_FW_LA_CMD_CIPHER = 0,
    ICP_QAT_FW_LA_CMD_AUTH = 1,
    ICP_QAT_FW_LA_CMD_CIPHER_HASH = 2,
    ICP_QAT_FW_LA_CMD_HASH_CIPHER = 3,
    ICP_QAT_FW_LA_CMD_TRNG_GET_RANDOM = 4,
    ICP_QAT_FW_LA_CMD_TRNG_TEST = 5,
    ICP_QAT_FW_LA_CMD_SSL3_KEY_DERIVE = 6,
    ICP_QAT_FW_LA_CMD_TLS_V1_1_KEY_DERIVE = 7,
    ICP_QAT_FW_LA_CMD_TLS_V1_2_KEY_DERIVE = 8,
    ICP_QAT_FW_LA_CMD_MGF1 = 9,
    ICP_QAT_FW_LA_CMD_AUTH_PRE_COMP = 10,
    ICP_QAT_FW_LA_CMD_CIPHER_PRE_COMP = 11,
    ICP_QAT_FW_LA_CMD_DELIMITER = 12
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_la_bulk_req {
    pub comn_hdr: icp_qat_fw_comn_req_hdr,
    pub cd_pars: icp_qat_fw_comn_req_hdr_cd_pars,
    pub comn_mid: icp_qat_fw_comn_req_mid,
    pub serv_specif_rqpars: icp_qat_fw_comn_req_rqpars,
    pub cd_ctrl: icp_qat_fw_comn_req_cd_ctrl,
}

pub const ICP_QAT_FW_LA_USE_UCS_SLICE_TYPE: c_int = 1;
pub const QAT_LA_SLICE_TYPE_BITPOS: c_int = 14;
pub const QAT_LA_SLICE_TYPE_MASK: c_uint = 0x3;
pub const ICP_QAT_FW_LA_GCM_IV_LEN_12_OCTETS: c_int = 1;
pub const ICP_QAT_FW_LA_GCM_IV_LEN_NOT_12_OCTETS: c_int = 0;
pub const QAT_FW_LA_ZUC_3G_PROTO_FLAG_BITPOS: c_int = 12;
pub const ICP_QAT_FW_LA_ZUC_3G_PROTO: c_int = 1;
pub const QAT_FW_LA_ZUC_3G_PROTO_FLAG_MASK: c_uint = 0x1;
pub const QAT_LA_GCM_IV_LEN_FLAG_BITPOS: c_int = 11;
pub const QAT_LA_GCM_IV_LEN_FLAG_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_LA_DIGEST_IN_BUFFER: c_int = 1;
pub const ICP_QAT_FW_LA_NO_DIGEST_IN_BUFFER: c_int = 0;
pub const QAT_LA_DIGEST_IN_BUFFER_BITPOS: c_int = 10;
pub const QAT_LA_DIGEST_IN_BUFFER_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_LA_SNOW_3G_PROTO: c_int = 4;
pub const ICP_QAT_FW_LA_GCM_PROTO: c_int = 2;
pub const ICP_QAT_FW_LA_CCM_PROTO: c_int = 1;
pub const ICP_QAT_FW_LA_NO_PROTO: c_int = 0;
pub const QAT_LA_PROTO_BITPOS: c_int = 7;
pub const QAT_LA_PROTO_MASK: c_uint = 0x7;
pub const ICP_QAT_FW_LA_CMP_AUTH_RES: c_int = 1;
pub const ICP_QAT_FW_LA_NO_CMP_AUTH_RES: c_int = 0;
pub const QAT_LA_CMP_AUTH_RES_BITPOS: c_int = 6;
pub const QAT_LA_CMP_AUTH_RES_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_LA_RET_AUTH_RES: c_int = 1;
pub const ICP_QAT_FW_LA_NO_RET_AUTH_RES: c_int = 0;
pub const QAT_LA_RET_AUTH_RES_BITPOS: c_int = 5;
pub const QAT_LA_RET_AUTH_RES_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_LA_UPDATE_STATE: c_int = 1;
pub const ICP_QAT_FW_LA_NO_UPDATE_STATE: c_int = 0;
pub const QAT_LA_UPDATE_STATE_BITPOS: c_int = 4;
pub const QAT_LA_UPDATE_STATE_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_CIPH_AUTH_CFG_OFFSET_IN_CD_SETUP: c_int = 0;
pub const ICP_QAT_FW_CIPH_AUTH_CFG_OFFSET_IN_SHRAM_CP: c_int = 1;
pub const QAT_LA_CIPH_AUTH_CFG_OFFSET_BITPOS: c_int = 3;
pub const QAT_LA_CIPH_AUTH_CFG_OFFSET_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_CIPH_IV_64BIT_PTR: c_int = 0;
pub const ICP_QAT_FW_CIPH_IV_16BYTE_DATA: c_int = 1;
pub const QAT_LA_CIPH_IV_FLD_BITPOS: c_int = 2;
pub const QAT_LA_CIPH_IV_FLD_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_LA_PARTIAL_NONE: c_int = 0;
pub const ICP_QAT_FW_LA_PARTIAL_START: c_int = 1;
pub const ICP_QAT_FW_LA_PARTIAL_MID: c_int = 3;
pub const ICP_QAT_FW_LA_PARTIAL_END: c_int = 2;
pub const QAT_LA_PARTIAL_BITPOS: c_int = 0;
pub const QAT_LA_PARTIAL_MASK: c_uint = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_cipher_req_hdr_cd_pars {
    pub content_desc_addr: __u64,
    pub content_desc_resrvd1: __u16,
    pub content_desc_params_sz: __u8,
    pub content_desc_hdr_resrvd2: __u8,
    pub content_desc_resrvd3: __u32,
    pub s: },
    pub cipher_key_array: [__u32; ICP_QAT_FW_NUM_LONGWORDS_4],
    pub s1: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_cipher_auth_req_hdr_cd_pars {
    pub content_desc_addr: __u64,
    pub content_desc_resrvd1: __u16,
    pub content_desc_params_sz: __u8,
    pub content_desc_hdr_resrvd2: __u8,
    pub content_desc_resrvd3: __u32,
    pub s: },
    pub cipher_key_array: [__u32; ICP_QAT_FW_NUM_LONGWORDS_4],
    pub sl: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_cipher_cd_ctrl_hdr {
    pub cipher_state_sz: __u8,
    pub cipher_key_sz: __u8,
    pub cipher_cfg_offset: __u8,
    pub next_curr_id: __u8,
    pub cipher_padding_sz: __u8,
    pub resrvd1: __u8,
    pub resrvd2: __u16,
    pub resrvd3: [__u32; ICP_QAT_FW_NUM_LONGWORDS_3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_auth_cd_ctrl_hdr {
    pub resrvd1: __u32,
    pub resrvd2: __u8,
    pub hash_flags: __u8,
    pub hash_cfg_offset: __u8,
    pub next_curr_id: __u8,
    pub resrvd3: __u8,
    pub outer_prefix_sz: __u8,
    pub final_sz: __u8,
    pub inner_res_sz: __u8,
    pub resrvd4: __u8,
    pub inner_state1_sz: __u8,
    pub inner_state2_offset: __u8,
    pub inner_state2_sz: __u8,
    pub outer_config_offset: __u8,
    pub outer_state1_sz: __u8,
    pub outer_res_sz: __u8,
    pub outer_prefix_offset: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_cipher_auth_cd_ctrl_hdr {
    pub cipher_state_sz: __u8,
    pub cipher_key_sz: __u8,
    pub cipher_cfg_offset: __u8,
    pub next_curr_id_cipher: __u8,
    pub cipher_padding_sz: __u8,
    pub hash_flags: __u8,
    pub hash_cfg_offset: __u8,
    pub next_curr_id_auth: __u8,
    pub resrvd1: __u8,
    pub outer_prefix_sz: __u8,
    pub final_sz: __u8,
    pub inner_res_sz: __u8,
    pub resrvd2: __u8,
    pub inner_state1_sz: __u8,
    pub inner_state2_offset: __u8,
    pub inner_state2_sz: __u8,
    pub outer_config_offset: __u8,
    pub outer_state1_sz: __u8,
    pub outer_res_sz: __u8,
    pub outer_prefix_offset: __u8,
}

pub const ICP_QAT_FW_AUTH_HDR_FLAG_DO_NESTED: c_int = 1;
pub const ICP_QAT_FW_AUTH_HDR_FLAG_NO_NESTED: c_int = 0;
pub const ICP_QAT_FW_CCM_GCM_AAD_SZ_MAX: c_int = 240;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_la_cipher_req_params {
    pub cipher_offset: __u32,
    pub cipher_length: __u32,
    pub cipher_IV_array: [__u32; ICP_QAT_FW_NUM_LONGWORDS_4],
    pub cipher_IV_ptr: __u64,
    pub resrvd1: __u64,
    pub s: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_la_auth_req_params {
    pub auth_off: __u32,
    pub auth_len: __u32,
    pub auth_partial_st_prefix: __u64,
    pub aad_adr: __u64,
    pub u1: },
    pub auth_res_addr: __u64,
    pub inner_prefix_sz: __u8,
    pub aad_sz: __u8,
    pub u2: },
    pub resrvd1: __u8,
    pub hash_state_sz: __u8,
    pub auth_res_sz: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_la_auth_req_params_resrvd_flds {
    pub resrvd: [__u32; ICP_QAT_FW_NUM_LONGWORDS_6],
    pub inner_prefix_sz: __u8,
    pub aad_sz: __u8,
    pub u2: },
    pub resrvd1: __u8,
    pub resrvd2: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_la_resp {
    pub comn_resp: icp_qat_fw_comn_resp_hdr,
    pub opaque_data: __u64,
    pub resrvd: [__u32; ICP_QAT_FW_NUM_LONGWORDS_4],
}

