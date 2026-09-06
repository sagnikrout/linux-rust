//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_hw.h
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
pub enum icp_qat_hw_ae_id {
    ICP_QAT_HW_AE_0 = 0,
    ICP_QAT_HW_AE_1 = 1,
    ICP_QAT_HW_AE_2 = 2,
    ICP_QAT_HW_AE_3 = 3,
    ICP_QAT_HW_AE_4 = 4,
    ICP_QAT_HW_AE_5 = 5,
    ICP_QAT_HW_AE_6 = 6,
    ICP_QAT_HW_AE_7 = 7,
    ICP_QAT_HW_AE_8 = 8,
    ICP_QAT_HW_AE_9 = 9,
    ICP_QAT_HW_AE_10 = 10,
    ICP_QAT_HW_AE_11 = 11,
    ICP_QAT_HW_AE_12 = 12,
    ICP_QAT_HW_AE_13 = 13,
    ICP_QAT_HW_AE_14 = 14,
    ICP_QAT_HW_AE_15 = 15,
    ICP_QAT_HW_AE_16 = 16,
    ICP_QAT_HW_AE_DELIMITER = 17
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_qat_id {
    ICP_QAT_HW_QAT_0 = 0,
    ICP_QAT_HW_QAT_1 = 1,
    ICP_QAT_HW_QAT_2 = 2,
    ICP_QAT_HW_QAT_3 = 3,
    ICP_QAT_HW_QAT_4 = 4,
    ICP_QAT_HW_QAT_5 = 5,
    ICP_QAT_HW_QAT_DELIMITER = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_auth_algo {
    ICP_QAT_HW_AUTH_ALGO_NULL = 0,
    ICP_QAT_HW_AUTH_ALGO_SHA1 = 1,
    ICP_QAT_HW_AUTH_ALGO_MD5 = 2,
    ICP_QAT_HW_AUTH_ALGO_SHA224 = 3,
    ICP_QAT_HW_AUTH_ALGO_SHA256 = 4,
    ICP_QAT_HW_AUTH_ALGO_SHA384 = 5,
    ICP_QAT_HW_AUTH_ALGO_SHA512 = 6,
    ICP_QAT_HW_AUTH_ALGO_AES_XCBC_MAC = 7,
    ICP_QAT_HW_AUTH_ALGO_AES_CBC_MAC = 8,
    ICP_QAT_HW_AUTH_ALGO_AES_F9 = 9,
    ICP_QAT_HW_AUTH_ALGO_GALOIS_128 = 10,
    ICP_QAT_HW_AUTH_ALGO_GALOIS_64 = 11,
    ICP_QAT_HW_AUTH_ALGO_KASUMI_F9 = 12,
    ICP_QAT_HW_AUTH_ALGO_SNOW_3G_UIA2 = 13,
    ICP_QAT_HW_AUTH_ALGO_ZUC_3G_128_EIA3 = 14,
    ICP_QAT_HW_AUTH_RESERVED_1 = 15,
    ICP_QAT_HW_AUTH_RESERVED_2 = 16,
    ICP_QAT_HW_AUTH_ALGO_SHA3_256 = 17,
    ICP_QAT_HW_AUTH_RESERVED_3 = 18,
    ICP_QAT_HW_AUTH_ALGO_SHA3_512 = 19,
    ICP_QAT_HW_AUTH_ALGO_DELIMITER = 20
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_auth_mode {
    ICP_QAT_HW_AUTH_MODE0 = 0,
    ICP_QAT_HW_AUTH_MODE1 = 1,
    ICP_QAT_HW_AUTH_MODE2 = 2,
    ICP_QAT_HW_AUTH_MODE_DELIMITER = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_auth_config {
    pub config: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_ucs_cipher_config {
    pub val: __u32,
    pub reserved: [__u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_slice_mask {
    ICP_ACCEL_MASK_CIPHER_SLICE = BIT(0),
    ICP_ACCEL_MASK_AUTH_SLICE = BIT(1),
    ICP_ACCEL_MASK_PKE_SLICE = BIT(2),
    ICP_ACCEL_MASK_COMPRESS_SLICE = BIT(3),
    ICP_ACCEL_MASK_LZS_SLICE = BIT(4),
    ICP_ACCEL_MASK_EIA3_SLICE = BIT(5),
    ICP_ACCEL_MASK_SHA3_SLICE = BIT(6),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_capabilities_mask {
    ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC = BIT(0),
    ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC = BIT(1),
    ICP_ACCEL_CAPABILITIES_CIPHER = BIT(2),
    ICP_ACCEL_CAPABILITIES_AUTHENTICATION = BIT(3),
    ICP_ACCEL_CAPABILITIES_RESERVED_1 = BIT(4),
    ICP_ACCEL_CAPABILITIES_COMPRESSION = BIT(5),
// Bit 6 is currently reserved
    ICP_ACCEL_CAPABILITIES_5G = BIT(7),
    ICP_ACCEL_CAPABILITIES_ZUC = BIT(8),
    ICP_ACCEL_CAPABILITIES_SHA3 = BIT(9),
// Bits 10-11 are currently reserved
    ICP_ACCEL_CAPABILITIES_HKDF = BIT(12),
    ICP_ACCEL_CAPABILITIES_ECEDMONT = BIT(13),
    ICP_ACCEL_CAPABILITIES_EXT_ALGCHAIN = BIT(14),
    ICP_ACCEL_CAPABILITIES_SHA3_EXT = BIT(15),
    ICP_ACCEL_CAPABILITIES_AESGCM_SPC = BIT(16),
    ICP_ACCEL_CAPABILITIES_CHACHA_POLY = BIT(17),
    ICP_ACCEL_CAPABILITIES_SM2 = BIT(18),
    ICP_ACCEL_CAPABILITIES_SM3 = BIT(19),
    ICP_ACCEL_CAPABILITIES_SM4 = BIT(20),
// Bit 21 is currently reserved
    ICP_ACCEL_CAPABILITIES_CNV_INTEGRITY = BIT(22),
    ICP_ACCEL_CAPABILITIES_CNV_INTEGRITY64 = BIT(23),
    ICP_ACCEL_CAPABILITIES_LZ4_COMPRESSION = BIT(24),
    ICP_ACCEL_CAPABILITIES_LZ4S_COMPRESSION = BIT(25),
    ICP_ACCEL_CAPABILITIES_AES_V2 = BIT(26),
    ICP_ACCEL_CAPABILITIES_KPT = BIT(27),
// Bit 28 is currently reserved
    ICP_ACCEL_CAPABILITIES_ZUC_256 = BIT(29),
    ICP_ACCEL_CAPABILITIES_WIRELESS_CRYPTO_EXT = BIT(30),
}

pub const QAT_AUTH_MODE_BITPOS: c_int = 4;
pub const QAT_AUTH_MODE_MASK: c_uint = 0xF;
pub const QAT_AUTH_ALGO_BITPOS: c_int = 0;
pub const QAT_AUTH_ALGO_MASK: c_uint = 0xF;
pub const QAT_AUTH_CMP_BITPOS: c_int = 8;
pub const QAT_AUTH_CMP_MASK: c_uint = 0x7F;
pub const QAT_AUTH_SHA3_PADDING_BITPOS: c_int = 16;
pub const QAT_AUTH_SHA3_PADDING_MASK: c_uint = 0x1;
pub const QAT_AUTH_ALGO_SHA3_BITPOS: c_int = 22;
pub const QAT_AUTH_ALGO_SHA3_MASK: c_uint = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_auth_counter {
    pub counter: __be32,
    pub reserved: __u32,
}

pub const QAT_AUTH_COUNT_MASK: c_uint = 0xFFFFFFFF;
pub const QAT_AUTH_COUNT_BITPOS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_auth_setup {
    pub auth_config: icp_qat_hw_auth_config,
    pub auth_counter: icp_qat_hw_auth_counter,
}

pub const QAT_HW_DEFAULT_ALIGNMENT: c_int = 8;

pub const ICP_QAT_HW_NULL_STATE1_SZ: c_int = 32;
pub const ICP_QAT_HW_MD5_STATE1_SZ: c_int = 16;
pub const ICP_QAT_HW_SHA1_STATE1_SZ: c_int = 20;
pub const ICP_QAT_HW_SHA224_STATE1_SZ: c_int = 32;
pub const ICP_QAT_HW_SHA256_STATE1_SZ: c_int = 32;
pub const ICP_QAT_HW_SHA3_256_STATE1_SZ: c_int = 32;
pub const ICP_QAT_HW_SHA384_STATE1_SZ: c_int = 64;
pub const ICP_QAT_HW_SHA512_STATE1_SZ: c_int = 64;
pub const ICP_QAT_HW_SHA3_512_STATE1_SZ: c_int = 64;
pub const ICP_QAT_HW_SHA3_224_STATE1_SZ: c_int = 28;
pub const ICP_QAT_HW_SHA3_384_STATE1_SZ: c_int = 48;
pub const ICP_QAT_HW_AES_XCBC_MAC_STATE1_SZ: c_int = 16;
pub const ICP_QAT_HW_AES_CBC_MAC_STATE1_SZ: c_int = 16;
pub const ICP_QAT_HW_AES_F9_STATE1_SZ: c_int = 32;
pub const ICP_QAT_HW_KASUMI_F9_STATE1_SZ: c_int = 16;
pub const ICP_QAT_HW_GALOIS_128_STATE1_SZ: c_int = 16;
pub const ICP_QAT_HW_SNOW_3G_UIA2_STATE1_SZ: c_int = 8;
pub const ICP_QAT_HW_ZUC_3G_EIA3_STATE1_SZ: c_int = 8;
pub const ICP_QAT_HW_NULL_STATE2_SZ: c_int = 32;
pub const ICP_QAT_HW_MD5_STATE2_SZ: c_int = 16;
pub const ICP_QAT_HW_SHA1_STATE2_SZ: c_int = 20;
pub const ICP_QAT_HW_SHA224_STATE2_SZ: c_int = 32;
pub const ICP_QAT_HW_SHA256_STATE2_SZ: c_int = 32;
pub const ICP_QAT_HW_SHA3_256_STATE2_SZ: c_int = 0;
pub const ICP_QAT_HW_SHA384_STATE2_SZ: c_int = 64;
pub const ICP_QAT_HW_SHA512_STATE2_SZ: c_int = 64;
pub const ICP_QAT_HW_SHA3_512_STATE2_SZ: c_int = 0;
pub const ICP_QAT_HW_SHA3_224_STATE2_SZ: c_int = 0;
pub const ICP_QAT_HW_SHA3_384_STATE2_SZ: c_int = 0;
pub const ICP_QAT_HW_AES_XCBC_MAC_KEY_SZ: c_int = 16;
pub const ICP_QAT_HW_AES_CBC_MAC_KEY_SZ: c_int = 16;
pub const ICP_QAT_HW_AES_CCM_CBC_E_CTR0_SZ: c_int = 16;
pub const ICP_QAT_HW_F9_IK_SZ: c_int = 16;
pub const ICP_QAT_HW_F9_FK_SZ: c_int = 16;

pub const ICP_QAT_HW_SNOW_3G_UIA2_STATE2_SZ: c_int = 24;
pub const ICP_QAT_HW_ZUC_3G_EIA3_STATE2_SZ: c_int = 32;
pub const ICP_QAT_HW_GALOIS_H_SZ: c_int = 16;
pub const ICP_QAT_HW_GALOIS_LEN_A_SZ: c_int = 8;
pub const ICP_QAT_HW_GALOIS_E_CTR0_SZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_auth_sha512 {
    pub inner_setup: icp_qat_hw_auth_setup,
    pub state1: [__u8; ICP_QAT_HW_SHA512_STATE1_SZ],
    pub outer_setup: icp_qat_hw_auth_setup,
    pub state2: [__u8; ICP_QAT_HW_SHA512_STATE2_SZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_auth_algo_blk {
    pub sha: icp_qat_hw_auth_sha512,
}

pub const ICP_QAT_HW_GALOIS_LEN_A_BITPOS: c_int = 0;
pub const ICP_QAT_HW_GALOIS_LEN_A_MASK: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_cipher_algo {
    ICP_QAT_HW_CIPHER_ALGO_NULL = 0,
    ICP_QAT_HW_CIPHER_ALGO_DES = 1,
    ICP_QAT_HW_CIPHER_ALGO_3DES = 2,
    ICP_QAT_HW_CIPHER_ALGO_AES128 = 3,
    ICP_QAT_HW_CIPHER_ALGO_AES192 = 4,
    ICP_QAT_HW_CIPHER_ALGO_AES256 = 5,
    ICP_QAT_HW_CIPHER_ALGO_ARC4 = 6,
    ICP_QAT_HW_CIPHER_ALGO_KASUMI = 7,
    ICP_QAT_HW_CIPHER_ALGO_SNOW_3G_UEA2 = 8,
    ICP_QAT_HW_CIPHER_ALGO_ZUC_3G_128_EEA3 = 9,
    ICP_QAT_HW_CIPHER_DELIMITER = 10
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_cipher_mode {
    ICP_QAT_HW_CIPHER_ECB_MODE = 0,
    ICP_QAT_HW_CIPHER_CBC_MODE = 1,
    ICP_QAT_HW_CIPHER_CTR_MODE = 2,
    ICP_QAT_HW_CIPHER_F8_MODE = 3,
    ICP_QAT_HW_CIPHER_XTS_MODE = 6,
    ICP_QAT_HW_CIPHER_MODE_DELIMITER = 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_cipher_config {
    pub val: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_cipher_dir {
    ICP_QAT_HW_CIPHER_ENCRYPT = 0,
    ICP_QAT_HW_CIPHER_DECRYPT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_cipher_convert {
    ICP_QAT_HW_CIPHER_NO_CONVERT = 0,
    ICP_QAT_HW_CIPHER_KEY_CONVERT = 1,
}

pub const QAT_CIPHER_MODE_BITPOS: c_int = 4;
pub const QAT_CIPHER_MODE_MASK: c_uint = 0xF;
pub const QAT_CIPHER_ALGO_BITPOS: c_int = 0;
pub const QAT_CIPHER_ALGO_MASK: c_uint = 0xF;
pub const QAT_CIPHER_CONVERT_BITPOS: c_int = 9;
pub const QAT_CIPHER_CONVERT_MASK: c_uint = 0x1;
pub const QAT_CIPHER_DIR_BITPOS: c_int = 8;
pub const QAT_CIPHER_DIR_MASK: c_uint = 0x1;
pub const QAT_CIPHER_MODE_F8_KEY_SZ_MULT: c_int = 2;
pub const QAT_CIPHER_MODE_XTS_KEY_SZ_MULT: c_int = 2;

pub const ICP_QAT_HW_DES_BLK_SZ: c_int = 8;
pub const ICP_QAT_HW_3DES_BLK_SZ: c_int = 8;
pub const ICP_QAT_HW_NULL_BLK_SZ: c_int = 8;
pub const ICP_QAT_HW_AES_BLK_SZ: c_int = 16;
pub const ICP_QAT_HW_KASUMI_BLK_SZ: c_int = 8;
pub const ICP_QAT_HW_SNOW_3G_BLK_SZ: c_int = 8;
pub const ICP_QAT_HW_ZUC_3G_BLK_SZ: c_int = 8;
pub const ICP_QAT_HW_NULL_KEY_SZ: c_int = 256;
pub const ICP_QAT_HW_DES_KEY_SZ: c_int = 8;
pub const ICP_QAT_HW_3DES_KEY_SZ: c_int = 24;
pub const ICP_QAT_HW_AES_128_KEY_SZ: c_int = 16;
pub const ICP_QAT_HW_AES_192_KEY_SZ: c_int = 24;
pub const ICP_QAT_HW_AES_256_KEY_SZ: c_int = 32;

pub const ICP_QAT_HW_KASUMI_KEY_SZ: c_int = 16;

pub const ICP_QAT_HW_ARC4_KEY_SZ: c_int = 256;
pub const ICP_QAT_HW_SNOW_3G_UEA2_KEY_SZ: c_int = 16;
pub const ICP_QAT_HW_SNOW_3G_UEA2_IV_SZ: c_int = 16;
pub const ICP_QAT_HW_ZUC_3G_EEA3_KEY_SZ: c_int = 16;
pub const ICP_QAT_HW_ZUC_3G_EEA3_IV_SZ: c_int = 16;
pub const ICP_QAT_HW_MODE_F8_NUM_REG_TO_CLEAR: c_int = 2;
pub const INIT_SHRAM_CONSTANTS_TABLE_SZ: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_cipher_aes256_f8 {
    pub cipher_config: icp_qat_hw_cipher_config,
    pub key: [__u8; ICP_QAT_HW_AES_256_F8_KEY_SZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_ucs_cipher_aes256_f8 {
    pub cipher_config: icp_qat_hw_ucs_cipher_config,
    pub key: [__u8; ICP_QAT_HW_AES_256_F8_KEY_SZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_cipher_algo_blk {
    pub aes: icp_qat_hw_cipher_aes256_f8,
    pub ucs_aes: icp_qat_hw_ucs_cipher_aes256_f8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_compression_direction {
    ICP_QAT_HW_COMPRESSION_DIR_COMPRESS = 0,
    ICP_QAT_HW_COMPRESSION_DIR_DECOMPRESS = 1,
    ICP_QAT_HW_COMPRESSION_DIR_DELIMITER = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_compression_delayed_match {
    ICP_QAT_HW_COMPRESSION_DELAYED_MATCH_DISABLED = 0,
    ICP_QAT_HW_COMPRESSION_DELAYED_MATCH_ENABLED = 1,
    ICP_QAT_HW_COMPRESSION_DELAYED_MATCH_DELIMITER = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_compression_algo {
    ICP_QAT_HW_COMPRESSION_ALGO_DEFLATE = 0,
    ICP_QAT_HW_COMPRESSION_ALGO_LZS = 1,
    ICP_QAT_HW_COMPRESSION_ALGO_ZSTD = 2,
    ICP_QAT_HW_COMPRESSION_ALGO_DELIMITER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_compression_depth {
    ICP_QAT_HW_COMPRESSION_DEPTH_1 = 0,
    ICP_QAT_HW_COMPRESSION_DEPTH_4 = 1,
    ICP_QAT_HW_COMPRESSION_DEPTH_8 = 2,
    ICP_QAT_HW_COMPRESSION_DEPTH_16 = 3,
    ICP_QAT_HW_COMPRESSION_DEPTH_128 = 4,
    ICP_QAT_HW_COMPRESSION_DEPTH_DELIMITER = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_hw_compression_file_type {
    ICP_QAT_HW_COMPRESSION_FILE_TYPE_0 = 0,
    ICP_QAT_HW_COMPRESSION_FILE_TYPE_1 = 1,
    ICP_QAT_HW_COMPRESSION_FILE_TYPE_2 = 2,
    ICP_QAT_HW_COMPRESSION_FILE_TYPE_3 = 3,
    ICP_QAT_HW_COMPRESSION_FILE_TYPE_4 = 4,
    ICP_QAT_HW_COMPRESSION_FILE_TYPE_DELIMITER = 5
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_compression_config {
    pub lower_val: __u32,
    pub upper_val: __u32,
}

pub const QAT_COMPRESSION_DIR_BITPOS: c_int = 4;
pub const QAT_COMPRESSION_DIR_MASK: c_uint = 0x7;
pub const QAT_COMPRESSION_DELAYED_MATCH_BITPOS: c_int = 16;
pub const QAT_COMPRESSION_DELAYED_MATCH_MASK: c_uint = 0x1;
pub const QAT_COMPRESSION_ALGO_BITPOS: c_int = 31;
pub const QAT_COMPRESSION_ALGO_MASK: c_uint = 0x1;
pub const QAT_COMPRESSION_DEPTH_BITPOS: c_int = 28;
pub const QAT_COMPRESSION_DEPTH_MASK: c_uint = 0x7;
pub const QAT_COMPRESSION_FILE_TYPE_BITPOS: c_int = 24;
pub const QAT_COMPRESSION_FILE_TYPE_MASK: c_uint = 0xF;

