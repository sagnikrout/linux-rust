//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_crypto_ctx.h
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
// Copyright (C) 2012-2019 ARM Limited (or its affiliates).

pub const CC_DRV_DES_IV_SIZE: c_int = 8;
pub const CC_DRV_DES_BLOCK_SIZE: c_int = 8;
pub const CC_DRV_DES_ONE_KEY_SIZE: c_int = 8;
pub const CC_DRV_DES_DOUBLE_KEY_SIZE: c_int = 16;
pub const CC_DRV_DES_TRIPLE_KEY_SIZE: c_int = 24;

pub const CC_AES_IV_SIZE: c_int = 16;

pub const CC_AES_BLOCK_SIZE: c_int = 16;
pub const CC_AES_BLOCK_SIZE_WORDS: c_int = 4;
pub const CC_AES_128_BIT_KEY_SIZE: c_int = 16;

pub const CC_AES_192_BIT_KEY_SIZE: c_int = 24;

pub const CC_AES_256_BIT_KEY_SIZE: c_int = 32;

pub const CC_MD5_DIGEST_SIZE: c_int = 16;
pub const CC_SHA1_DIGEST_SIZE: c_int = 20;
pub const CC_SHA224_DIGEST_SIZE: c_int = 28;
pub const CC_SHA256_DIGEST_SIZE: c_int = 32;
pub const CC_SHA256_DIGEST_SIZE_IN_WORDS: c_int = 8;
pub const CC_SHA384_DIGEST_SIZE: c_int = 48;
pub const CC_SHA512_DIGEST_SIZE: c_int = 64;
pub const CC_SHA1_BLOCK_SIZE: c_int = 64;
pub const CC_SHA1_BLOCK_SIZE_IN_WORDS: c_int = 16;
pub const CC_MD5_BLOCK_SIZE: c_int = 64;
pub const CC_MD5_BLOCK_SIZE_IN_WORDS: c_int = 16;
pub const CC_SHA224_BLOCK_SIZE: c_int = 64;
pub const CC_SHA256_BLOCK_SIZE: c_int = 64;
pub const CC_SHA256_BLOCK_SIZE_IN_WORDS: c_int = 16;
pub const CC_SHA1_224_256_BLOCK_SIZE: c_int = 64;
pub const CC_SHA384_BLOCK_SIZE: c_int = 128;
pub const CC_SHA512_BLOCK_SIZE: c_int = 128;

pub const CC_CPP_NUM_SLOTS: c_int = 8;
pub const CC_CPP_NUM_ALGS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_cpp_alg {
    CC_CPP_SM4 = 1,
    CC_CPP_AES = 0
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drv_engine_type {
    DRV_ENGINE_NULL = 0,
    DRV_ENGINE_AES = 1,
    DRV_ENGINE_DES = 2,
    DRV_ENGINE_HASH = 3,
    DRV_ENGINE_RC4 = 4,
    DRV_ENGINE_DOUT = 5,
    DRV_ENGINE_RESERVE32B = S32_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drv_crypto_alg {
    DRV_CRYPTO_ALG_NULL = -1,
    DRV_CRYPTO_ALG_AES  = 0,
    DRV_CRYPTO_ALG_DES  = 1,
    DRV_CRYPTO_ALG_HASH = 2,
    DRV_CRYPTO_ALG_C2   = 3,
    DRV_CRYPTO_ALG_HMAC = 4,
    DRV_CRYPTO_ALG_AEAD = 5,
    DRV_CRYPTO_ALG_BYPASS = 6,
    DRV_CRYPTO_ALG_NUM = 7,
    DRV_CRYPTO_ALG_RESERVE32B = S32_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drv_crypto_direction {
    DRV_CRYPTO_DIRECTION_NULL = -1,
    DRV_CRYPTO_DIRECTION_ENCRYPT = 0,
    DRV_CRYPTO_DIRECTION_DECRYPT = 1,
    DRV_CRYPTO_DIRECTION_DECRYPT_ENCRYPT = 3,
    DRV_CRYPTO_DIRECTION_RESERVE32B = S32_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drv_cipher_mode {
    DRV_CIPHER_NULL_MODE = -1,
    DRV_CIPHER_ECB = 0,
    DRV_CIPHER_CBC = 1,
    DRV_CIPHER_CTR = 2,
    DRV_CIPHER_CBC_MAC = 3,
    DRV_CIPHER_XTS = 4,
    DRV_CIPHER_XCBC_MAC = 5,
    DRV_CIPHER_OFB = 6,
    DRV_CIPHER_CMAC = 7,
    DRV_CIPHER_CCM = 8,
    DRV_CIPHER_CBC_CTS = 11,
    DRV_CIPHER_GCTR = 12,
    DRV_CIPHER_ESSIV = 13,
    DRV_CIPHER_RESERVE32B = S32_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drv_hash_mode {
    DRV_HASH_NULL = -1,
    DRV_HASH_SHA1 = 0,
    DRV_HASH_SHA256 = 1,
    DRV_HASH_SHA224 = 2,
    DRV_HASH_SHA512 = 3,
    DRV_HASH_SHA384 = 4,
    DRV_HASH_MD5 = 5,
    DRV_HASH_CBC_MAC = 6,
    DRV_HASH_XCBC_MAC = 7,
    DRV_HASH_CMAC = 8,
    DRV_HASH_SM3 = 9,
    DRV_HASH_MODE_NUM = 10,
    DRV_HASH_RESERVE32B = S32_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drv_hash_hw_mode {
    DRV_HASH_HW_MD5 = 0,
    DRV_HASH_HW_SHA1 = 1,
    DRV_HASH_HW_SHA256 = 2,
    DRV_HASH_HW_SHA224 = 10,
    DRV_HASH_HW_SHA512 = 4,
    DRV_HASH_HW_SHA384 = 12,
    DRV_HASH_HW_GHASH = 6,
    DRV_HASH_HW_SM3 = 14,
    DRV_HASH_HW_RESERVE32B = S32_MAX
}
