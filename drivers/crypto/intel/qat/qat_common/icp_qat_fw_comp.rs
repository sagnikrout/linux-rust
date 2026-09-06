//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_fw_comp.h
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
// Copyright(c) 2022 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_fw_comp_cmd_id {
    ICP_QAT_FW_COMP_CMD_STATIC = 0,
    ICP_QAT_FW_COMP_CMD_DYNAMIC = 1,
    ICP_QAT_FW_COMP_CMD_DECOMPRESS = 2,
    ICP_QAT_FW_COMP_CMD_ZSTD_COMPRESS = 10,
    ICP_QAT_FW_COMP_CMD_ZSTD_DECOMPRESS = 11,
    ICP_QAT_FW_COMP_CMD_DELIMITER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_fw_comp_20_cmd_id {
    ICP_QAT_FW_COMP_20_CMD_LZ4_COMPRESS = 3,
    ICP_QAT_FW_COMP_20_CMD_LZ4_DECOMPRESS = 4,
    ICP_QAT_FW_COMP_20_CMD_LZ4S_COMPRESS = 5,
    ICP_QAT_FW_COMP_20_CMD_LZ4S_DECOMPRESS = 6,
    ICP_QAT_FW_COMP_20_CMD_RESERVED_7 = 7,
    ICP_QAT_FW_COMP_20_CMD_RESERVED_8 = 8,
    ICP_QAT_FW_COMP_20_CMD_RESERVED_9 = 9,
    ICP_QAT_FW_COMP_23_CMD_ZSTD_COMPRESS = 10,
    ICP_QAT_FW_COMP_23_CMD_ZSTD_DECOMPRESS = 11,
    ICP_QAT_FW_COMP_20_CMD_DELIMITER
}

pub const ICP_QAT_FW_COMP_STATELESS_SESSION: c_int = 0;
pub const ICP_QAT_FW_COMP_STATEFUL_SESSION: c_int = 1;
pub const ICP_QAT_FW_COMP_NOT_AUTO_SELECT_BEST: c_int = 0;
pub const ICP_QAT_FW_COMP_AUTO_SELECT_BEST: c_int = 1;
pub const ICP_QAT_FW_COMP_NOT_ENH_AUTO_SELECT_BEST: c_int = 0;
pub const ICP_QAT_FW_COMP_ENH_AUTO_SELECT_BEST: c_int = 1;
pub const ICP_QAT_FW_COMP_NOT_DISABLE_TYPE0_ENH_AUTO_SELECT_BEST: c_int = 0;
pub const ICP_QAT_FW_COMP_DISABLE_TYPE0_ENH_AUTO_SELECT_BEST: c_int = 1;
pub const ICP_QAT_FW_COMP_DISABLE_SECURE_RAM_USED_AS_INTMD_BUF: c_int = 1;
pub const ICP_QAT_FW_COMP_ENABLE_SECURE_RAM_USED_AS_INTMD_BUF: c_int = 0;
pub const ICP_QAT_FW_COMP_SESSION_TYPE_BITPOS: c_int = 2;
pub const ICP_QAT_FW_COMP_SESSION_TYPE_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_AUTO_SELECT_BEST_BITPOS: c_int = 3;
pub const ICP_QAT_FW_COMP_AUTO_SELECT_BEST_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_ENHANCED_AUTO_SELECT_BEST_BITPOS: c_int = 4;
pub const ICP_QAT_FW_COMP_ENHANCED_AUTO_SELECT_BEST_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_RET_DISABLE_TYPE0_HEADER_DATA_BITPOS: c_int = 5;
pub const ICP_QAT_FW_COMP_RET_DISABLE_TYPE0_HEADER_DATA_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_DISABLE_SECURE_RAM_AS_INTMD_BUF_BITPOS: c_int = 7;
pub const ICP_QAT_FW_COMP_DISABLE_SECURE_RAM_AS_INTMD_BUF_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_AUTO_SELECT_BEST_MAX_VALUE: c_uint = 0xFFFFFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comp_req_hdr_cd_pars {
    pub content_desc_addr: __u64,
    pub content_desc_resrvd1: __u16,
    pub content_desc_params_sz: __u8,
    pub content_desc_hdr_resrvd2: __u8,
    pub content_desc_resrvd3: __u32,
    pub s: },
    pub comp_slice_cfg_word: [__u32; ICP_QAT_FW_NUM_LONGWORDS_2],
    pub content_desc_resrvd4: __u32,
    pub sl: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comp_req_params {
    pub comp_len: __u32,
    pub out_buffer_sz: __u32,
    pub initial_crc32: __u32,
    pub initial_adler: __u32,
    pub legacy: },
    pub crc_data_addr: __u64,
    pub crc: },
    pub req_par_flags: __u32,
    pub rsrvd: __u32,
}

pub const ICP_QAT_FW_COMP_NOT_SOP: c_int = 0;
pub const ICP_QAT_FW_COMP_SOP: c_int = 1;
pub const ICP_QAT_FW_COMP_NOT_EOP: c_int = 0;
pub const ICP_QAT_FW_COMP_EOP: c_int = 1;
pub const ICP_QAT_FW_COMP_NOT_BFINAL: c_int = 0;
pub const ICP_QAT_FW_COMP_BFINAL: c_int = 1;
pub const ICP_QAT_FW_COMP_NO_CNV: c_int = 0;
pub const ICP_QAT_FW_COMP_CNV: c_int = 1;
pub const ICP_QAT_FW_COMP_NO_CNV_RECOVERY: c_int = 0;
pub const ICP_QAT_FW_COMP_CNV_RECOVERY: c_int = 1;
pub const ICP_QAT_FW_COMP_NO_CNV_DFX: c_int = 0;
pub const ICP_QAT_FW_COMP_CNV_DFX: c_int = 1;
pub const ICP_QAT_FW_COMP_CRC_MODE_LEGACY: c_int = 0;
pub const ICP_QAT_FW_COMP_CRC_MODE_E2E: c_int = 1;
pub const ICP_QAT_FW_COMP_NO_XXHASH_ACC: c_int = 0;
pub const ICP_QAT_FW_COMP_XXHASH_ACC: c_int = 1;
pub const ICP_QAT_FW_COMP_APPEND_CRC: c_int = 1;
pub const ICP_QAT_FW_COMP_NO_APPEND_CRC: c_int = 0;
pub const ICP_QAT_FW_COMP_DROP_DATA: c_int = 1;
pub const ICP_QAT_FW_COMP_NO_DROP_DATA: c_int = 0;
pub const ICP_QAT_FW_COMP_PARTIAL_DECOMPRESS: c_int = 1;
pub const ICP_QAT_FW_COMP_NO_PARTIAL_DECOMPRESS: c_int = 0;
pub const ICP_QAT_FW_COMP_SOP_BITPOS: c_int = 0;
pub const ICP_QAT_FW_COMP_SOP_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_EOP_BITPOS: c_int = 1;
pub const ICP_QAT_FW_COMP_EOP_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_BFINAL_BITPOS: c_int = 6;
pub const ICP_QAT_FW_COMP_BFINAL_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_CNV_BITPOS: c_int = 16;
pub const ICP_QAT_FW_COMP_CNV_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_CNVNR_BITPOS: c_int = 17;
pub const ICP_QAT_FW_COMP_CNVNR_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_CNV_DFX_BITPOS: c_int = 18;
pub const ICP_QAT_FW_COMP_CNV_DFX_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_CRC_MODE_BITPOS: c_int = 19;
pub const ICP_QAT_FW_COMP_CRC_MODE_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_XXHASH_ACC_MODE_BITPOS: c_int = 20;
pub const ICP_QAT_FW_COMP_XXHASH_ACC_MODE_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_CNV_ERROR_BITPOS: c_int = 21;

pub const ICP_QAT_FW_COMP_APPEND_CRC_BITPOS: c_int = 24;
pub const ICP_QAT_FW_COMP_APPEND_CRC_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_DROP_DATA_BITPOS: c_int = 25;
pub const ICP_QAT_FW_COMP_DROP_DATA_MASK: c_uint = 0x1;
pub const ICP_QAT_FW_COMP_PARTIAL_DECOMP_BITPOS: c_int = 27;
pub const ICP_QAT_FW_COMP_PARTIAL_DECOMP_MASK: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_xlt_req_params {
    pub inter_buff_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comp_cd_hdr {
    pub ram_bank_flags: __u16,
    pub comp_cfg_offset: __u8,
    pub next_curr_id: __u8,
    pub resrvd: __u32,
    pub comp_state_addr: __u64,
    pub ram_banks_addr: __u64,
}

pub const COMP_CPR_INITIAL_CRC: c_int = 0;
pub const COMP_CPR_INITIAL_ADLER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_xlt_cd_hdr {
    pub resrvd1: __u16,
    pub resrvd2: __u8,
    pub next_curr_id: __u8,
    pub resrvd3: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comp_req {
    pub comn_hdr: icp_qat_fw_comn_req_hdr,
    pub cd_pars: icp_qat_fw_comp_req_hdr_cd_pars,
    pub comn_mid: icp_qat_fw_comn_req_mid,
    pub comp_pars: icp_qat_fw_comp_req_params,
    pub xlt_pars: icp_qat_fw_xlt_req_params,
    pub resrvd1: [__u32; ICP_QAT_FW_NUM_LONGWORDS_2],
    pub partial_decompress_length: __u32,
    pub partial_decompress_offset: __u32,
    pub partial_decompress: },
    pub u1: },
    pub resrvd2: [__u32; ICP_QAT_FW_NUM_LONGWORDS_2],
    pub asb_value: __u32,
    pub reserved: __u32,
    pub asb_threshold: },
    pub u3: },
    pub comp_cd_ctrl: icp_qat_fw_comp_cd_hdr,
    pub xlt_cd_ctrl: icp_qat_fw_xlt_cd_hdr,
    pub resrvd3: [__u32; ICP_QAT_FW_NUM_LONGWORDS_2],
    pub u2: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_resp_comp_pars {
    pub input_byte_counter: __u32,
    pub output_byte_counter: __u32,
    pub curr_crc32: __u32,
    pub curr_adler_32: __u32,
    pub legacy: },
    pub resrvd: [__u32; ICP_QAT_FW_NUM_LONGWORDS_2],
    pub crc: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comp_state {
    pub rd8_counter: __u32,
    pub status_flags: __u32,
    pub in_counter: __u32,
    pub out_counter: __u32,
    pub intermediate_state: __u64,
    pub lobc: __u32,
    pub replaybc: __u32,
    pub pcrc64_poly: __u64,
    pub crc32: __u32,
    pub adler_xxhash32: __u32,
    pub pcrc64_xorout: __u64,
    pub out_buf_size: __u32,
    pub in_buf_size: __u32,
    pub in_pcrc64: __u64,
    pub out_pcrc64: __u64,
    pub lobs: __u32,
    pub libc: __u32,
    pub reserved: __u64,
    pub xxhash_state: [__u32; 4],
    pub cleartext: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comp_resp {
    pub comn_resp: icp_qat_fw_comn_resp_hdr,
    pub opaque_data: __u64,
    pub comp_resp_pars: icp_qat_fw_resp_comp_pars,
}

pub const QAT_FW_COMP_BANK_FLAG_MASK: c_uint = 0x1;
pub const QAT_FW_COMP_BANK_I_BITPOS: c_int = 8;
pub const QAT_FW_COMP_BANK_H_BITPOS: c_int = 7;
pub const QAT_FW_COMP_BANK_G_BITPOS: c_int = 6;
pub const QAT_FW_COMP_BANK_F_BITPOS: c_int = 5;
pub const QAT_FW_COMP_BANK_E_BITPOS: c_int = 4;
pub const QAT_FW_COMP_BANK_D_BITPOS: c_int = 3;
pub const QAT_FW_COMP_BANK_C_BITPOS: c_int = 2;
pub const QAT_FW_COMP_BANK_B_BITPOS: c_int = 1;
pub const QAT_FW_COMP_BANK_A_BITPOS: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_fw_comp_bank_enabled {
    ICP_QAT_FW_COMP_BANK_DISABLED = 0,
    ICP_QAT_FW_COMP_BANK_ENABLED = 1,
    ICP_QAT_FW_COMP_BANK_DELIMITER = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comp_crc_data_struct {
    pub crc32: __u32,
    pub adler: __u32,
    pub xxhash: __u32,
    pub adler_xxhash_u: },
    pub cpr_in_crc_lo: __u32,
    pub cpr_in_crc_hi: __u32,
    pub cpr_out_crc_lo: __u32,
    pub cpr_out_crc_hi: __u32,
    pub xlt_in_crc_lo: __u32,
    pub xlt_in_crc_hi: __u32,
    pub xlt_out_crc_lo: __u32,
    pub xlt_out_crc_hi: __u32,
    pub prog_crc_poly_lo: __u32,
    pub prog_crc_poly_hi: __u32,
    pub xor_out_lo: __u32,
    pub xor_out_hi: __u32,
    pub append_crc_lo: __u32,
    pub append_crc_hi: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xxhash_acc_state_buff {
    pub in_counter: __u32,
    pub out_counter: __u32,
    pub xxhash_state: [__u32; 4],
    pub clear_txt: [__u32; 4],
}
