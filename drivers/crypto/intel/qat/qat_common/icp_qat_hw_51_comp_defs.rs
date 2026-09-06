//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_hw_51_comp_defs.h
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
// Copyright(c) 2025 Intel Corporation

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_SOM_CONTROL_BITPOS: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_som_control {
    ICP_QAT_HW_COMP_51_SOM_CONTROL_NORMAL_MODE = 0x0,
    ICP_QAT_HW_COMP_51_SOM_CONTROL_DICTIONARY_MODE = 0x1,
    ICP_QAT_HW_COMP_51_SOM_CONTROL_INPUT_CRC = 0x2,
    ICP_QAT_HW_COMP_51_SOM_CONTROL_RESERVED_MODE = 0x3,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_SKIP_HASH_RD_CONTROL_BITPOS: c_int = 27;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_skip_hash_rd_control {
    ICP_QAT_HW_COMP_51_SKIP_HASH_RD_CONTROL_NO_SKIP = 0x0,
    ICP_QAT_HW_COMP_51_SKIP_HASH_RD_CONTROL_SKIP_HASH_READS = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_BYPASS_COMPRESSION_BITPOS: c_int = 25;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_bypass_compression {
    ICP_QAT_HW_COMP_51_BYPASS_COMPRESSION_DISABLED = 0x0,
    ICP_QAT_HW_COMP_51_BYPASS_COMPRESSION_ENABLED = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_DMM_ALGORITHM_BITPOS: c_int = 22;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_dmm_algorithm {
    ICP_QAT_HW_COMP_51_DMM_ALGORITHM_EDMM_ENABLED = 0x0,
    ICP_QAT_HW_COMP_51_DMM_ALGORITHM_ZSTD_DMM_LITE = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_TOKEN_FUSION_INTERNAL_ONLY_BITPOS: c_int = 21;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_token_fusion_internal_only {
    ICP_QAT_HW_COMP_51_TOKEN_FUSION_INTERNAL_ONLY_ENABLED = 0x0,
    ICP_QAT_HW_COMP_51_TOKEN_FUSION_INTERNAL_ONLY_DISABLED = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_BMS_BITPOS: c_int = 19;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_bms {
    ICP_QAT_HW_COMP_51_BMS_BMS_64KB = 0x0,
    ICP_QAT_HW_COMP_51_BMS_BMS_256KB = 0x1,
    ICP_QAT_HW_COMP_51_BMS_BMS_1MB = 0x2,
    ICP_QAT_HW_COMP_51_BMS_BMS_4MB = 0x3,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_SCB_MODE_RESET_MASK_BITPOS: c_int = 18;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_scb_mode_reset_mask {
    ICP_QAT_HW_COMP_51_SCB_MODE_RESET_MASK_DO_NOT_RESET_HB_HT = 0x0,
    ICP_QAT_HW_COMP_51_SCB_MODE_RESET_MASK_RESET_HB_HT = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_ZSTD_FRAME_GEN_DEC_EN_BITPOS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_zstd_frame_gen_dec_en {
    ICP_QAT_HW_COMP_51_ZSTD_FRAME_GEN_DEC_EN_ZSTD_FRAME_HDR_DISABLE = 0x0,
    ICP_QAT_HW_COMP_51_ZSTD_FRAME_GEN_DEC_EN_ZSTD_FRAME_HDR_ENABLE = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_CNV_DISABLE_BITPOS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_cnv_disable {
    ICP_QAT_HW_COMP_51_CNV_DISABLE_CNV_ENABLED = 0x0,
    ICP_QAT_HW_COMP_51_CNV_DISABLE_CNV_DISABLED = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_ASB_DISABLE_BITPOS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_asb_disable {
    ICP_QAT_HW_COMP_51_ASB_DISABLE_ASB_ENABLED = 0x0,
    ICP_QAT_HW_COMP_51_ASB_DISABLE_ASB_DISABLED = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_SPEC_DECODER_INTERNAL_ONLY_BITPOS: c_int = 21;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_spec_decoder_internal_only {
    ICP_QAT_HW_COMP_51_SPEC_DECODER_INTERNAL_ONLY_NORMAL = 0x0,
    ICP_QAT_HW_COMP_51_SPEC_DECODER_INTERNAL_ONLY_DISABLED = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_MINI_XCAM_INTERNAL_ONLY_BITPOS: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_mini_xcam_internal_only {
    ICP_QAT_HW_COMP_51_MINI_XCAM_INTERNAL_ONLY_NORMAL = 0x0,
    ICP_QAT_HW_COMP_51_MINI_XCAM_INTERNAL_ONLY_DISABLED = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_REP_OFF_ENC_INTERNAL_ONLY_BITPOS: c_int = 19;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_rep_off_enc_internal_only {
    ICP_QAT_HW_COMP_51_REP_OFF_ENC_INTERNAL_ONLY_ENABLED = 0x0,
    ICP_QAT_HW_COMP_51_REP_OFF_ENC_INTERNAL_ONLY_DISABLED = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_PROG_BLOCK_DROP_INTERNAL_ONLY_BITPOS: c_int = 18;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_prog_block_drop_internal_only {
    ICP_QAT_HW_COMP_51_PROG_BLOCK_DROP_INTERNAL_ONLY_DISABLE = 0x0,
    ICP_QAT_HW_COMP_51_PROG_BLOCK_DROP_INTERNAL_ONLY_ENABLE = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_SKIP_HASH_OVERRIDE_INTERNAL_ONLY_BITPOS: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_skip_hash_override_internal_only {
    ICP_QAT_HW_COMP_51_SKIP_HASH_OVERRIDE_INTERNAL_ONLY_DETERMINE_HASH_PARAMS = 0x0,
    ICP_QAT_HW_COMP_51_SKIP_HASH_OVERRIDE_INTERNAL_ONLY_OVERRIDE_HASH_PARAMS = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_HBS_BITPOS: c_int = 14;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_hbs {
    ICP_QAT_HW_COMP_51_HBS_32KB = 0x0,
    ICP_QAT_HW_COMP_51_HBS_64KB = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_ABD_BITPOS: c_int = 13;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_abd {
    ICP_QAT_HW_COMP_51_ABD_ABD_ENABLED = 0x0,
    ICP_QAT_HW_COMP_51_ABD_ABD_DISABLED = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_LLLBD_CTRL_BITPOS: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_lllbd_ctrl {
    ICP_QAT_HW_COMP_51_LLLBD_CTRL_LLLBD_ENABLED = 0x0,
    ICP_QAT_HW_COMP_51_LLLBD_CTRL_LLLBD_DISABLED = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_SEARCH_DEPTH_BITPOS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_search_depth {
    ICP_QAT_HW_COMP_51_SEARCH_DEPTH_LEVEL_1 = 0x1,
    ICP_QAT_HW_COMP_51_SEARCH_DEPTH_LEVEL_6 = 0x3,
    ICP_QAT_HW_COMP_51_SEARCH_DEPTH_LEVEL_9 = 0x4,
    ICP_QAT_HW_COMP_51_SEARCH_DEPTH_LEVEL_10 = 0x4,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_FORMAT_BITPOS: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_format {
    ICP_QAT_HW_COMP_51_FORMAT_ILZ77 = 0x1,
    ICP_QAT_HW_COMP_51_FORMAT_LZ4 = 0x2,
    ICP_QAT_HW_COMP_51_FORMAT_LZ4s = 0x3,
    ICP_QAT_HW_COMP_51_FORMAT_ZSTD = 0x4,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_MIN_MATCH_CONTROL_BITPOS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_min_match_control {
    ICP_QAT_HW_COMP_51_MIN_MATCH_CONTROL_MATCH_3B = 0x0,
    ICP_QAT_HW_COMP_51_MIN_MATCH_CONTROL_MATCH_4B = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_SKIP_HASH_COLLISION_BITPOS: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_skip_hash_collision {
    ICP_QAT_HW_COMP_51_SKIP_HASH_COLLISION_ALLOW = 0x0,
    ICP_QAT_HW_COMP_51_SKIP_HASH_COLLISION_DONT_ALLOW = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_SKIP_HASH_UPDATE_BITPOS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_skip_hash_update {
    ICP_QAT_HW_COMP_51_SKIP_HASH_UPDATE_ALLOW = 0x0,
    ICP_QAT_HW_COMP_51_SKIP_HASH_UPDATE_DONT_ALLOW = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_BYTE_SKIP_BITPOS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_byte_skip {
    ICP_QAT_HW_COMP_51_BYTE_SKIP_3BYTE_TOKEN = 0x0,
    ICP_QAT_HW_COMP_51_BYTE_SKIP_3BYTE_LITERAL = 0x1,
}

pub const ICP_QAT_HW_COMP_51_CONFIG_CSR_LZ4_BLOCK_CHECKSUM_BITPOS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_comp_51_lz4_block_checksum {
    ICP_QAT_HW_COMP_51_LZ4_BLOCK_CHECKSUM_ABSENT = 0x0,
    ICP_QAT_HW_COMP_51_LZ4_BLOCK_CHECKSUM_PRESENT = 0x1,
}

pub const ICP_QAT_HW_DECOMP_51_CONFIG_CSR_DISCARD_DATA_BITPOS: c_int = 26;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_decomp_51_discard_data {
    ICP_QAT_HW_DECOMP_51_DISCARD_DATA_DISABLED = 0x0,
    ICP_QAT_HW_DECOMP_51_DISCARD_DATA_ENABLED = 0x1,
}

pub const ICP_QAT_HW_DECOMP_51_CONFIG_CSR_BMS_BITPOS: c_int = 19;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_decomp_51_bms {
    ICP_QAT_HW_DECOMP_51_BMS_BMS_64KB = 0x0,
    ICP_QAT_HW_DECOMP_51_BMS_BMS_256KB = 0x1,
    ICP_QAT_HW_DECOMP_51_BMS_BMS_1MB = 0x2,
    ICP_QAT_HW_DECOMP_51_BMS_BMS_4MB = 0x3,
}

pub const ICP_QAT_HW_DECOMP_51_CONFIG_CSR_ZSTD_FRAME_GEN_DEC_EN_BITPOS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_decomp_51_zstd_frame_gen_dec_en {
    ICP_QAT_HW_DECOMP_51_ZSTD_FRAME_GEN_DEC_EN_ZSTD_FRAME_HDR_DISABLE = 0x0,
    ICP_QAT_HW_DECOMP_51_ZSTD_FRAME_GEN_DEC_EN_ZSTD_FRAME_HDR_ENABLE = 0x1,
}

pub const ICP_QAT_HW_DECOMP_51_CONFIG_CSR_SPEC_DECODER_INTERNAL_ONLY_BITPOS: c_int = 21;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_decomp_51_spec_decoder_internal_only {
    ICP_QAT_HW_DECOMP_51_SPEC_DECODER_INTERNAL_ONLY_NORMAL = 0x0,
    ICP_QAT_HW_DECOMP_51_SPEC_DECODER_INTERNAL_ONLY_DISABLED = 0x1,
}

pub const ICP_QAT_HW_DECOMP_51_CONFIG_CSR_MINI_XCAM_INTERNAL_ONLY_BITPOS: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_decomp_51_mini_xcam_internal_only {
    ICP_QAT_HW_DECOMP_51_MINI_XCAM_INTERNAL_ONLY_NORMAL = 0x0,
    ICP_QAT_HW_DECOMP_51_MINI_XCAM_INTERNAL_ONLY_DISABLED = 0x1,
}

pub const ICP_QAT_HW_DECOMP_51_CONFIG_CSR_HBS_BITPOS: c_int = 14;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_decomp_51_hbs {
    ICP_QAT_HW_DECOMP_51_HBS_32KB = 0x0,
    ICP_QAT_HW_DECOMP_51_HBS_64KB = 0x1,
}

pub const ICP_QAT_HW_DECOMP_51_CONFIG_CSR_FORMAT_BITPOS: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_decomp_51_format {
    ICP_QAT_HW_DECOMP_51_FORMAT_ILZ77 = 0x1,
    ICP_QAT_HW_DECOMP_51_FORMAT_LZ4 = 0x2,
    ICP_QAT_HW_DECOMP_51_FORMAT_RESERVED = 0x3,
    ICP_QAT_HW_DECOMP_51_FORMAT_ZSTD = 0x4,
}

pub const ICP_QAT_HW_DECOMP_51_CONFIG_CSR_LZ4_BLOCK_CHECKSUM_BITPOS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_decomp_51_lz4_block_checksum {
    ICP_QAT_HW_DECOMP_51_LZ4_BLOCK_CHECKSUM_ABSENT = 0x0,
    ICP_QAT_HW_DECOMP_51_LZ4_BLOCK_CHECKSUM_PRESENT = 0x1,
}

