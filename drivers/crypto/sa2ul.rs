//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/sa2ul.h
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
//
// K3 SA2UL crypto accelerator driver
//
// Copyright (C) 2018-2020 Texas Instruments Incorporated - http://www.ti.com
//
// Authors:	Keerthy
// Vitaly Andrianov
// Tero Kristo
//

pub const SA_ENGINE_STATUS: c_uint = 0x0008;
pub const SA_ENGINE_ENABLE_CONTROL: c_uint = 0x1000;
//
// SA_ENGINE_ENABLE_CONTROL register bits
//
pub const SA_EEC_ENCSS_EN: c_uint = 0x00000001;
pub const SA_EEC_AUTHSS_EN: c_uint = 0x00000002;
pub const SA_EEC_TRNG_EN: c_uint = 0x00000008;
pub const SA_EEC_PKA_EN: c_uint = 0x00000010;
pub const SA_EEC_CTXCACH_EN: c_uint = 0x00000080;
pub const SA_EEC_CPPI_PORT_IN_EN: c_uint = 0x00000200;
pub const SA_EEC_CPPI_PORT_OUT_EN: c_uint = 0x00000800;
//
// Encoding used to identify the typo of crypto operation
// performed on the packet when the packet is returned
// by SA
//
pub const SA_REQ_SUBTYPE_ENC: c_uint = 0x0001;
pub const SA_REQ_SUBTYPE_DEC: c_uint = 0x0002;
pub const SA_REQ_SUBTYPE_SHIFT: c_int = 16;
pub const SA_REQ_SUBTYPE_MASK: c_uint = 0xffff;
// Number of 32 bit words in EPIB
pub const SA_DMA_NUM_EPIB_WORDS: c_int = 4;
// Number of 32 bit words in PS data
pub const SA_DMA_NUM_PS_WORDS: c_int = 16;
pub const NKEY_SZ: c_int = 3;
pub const MCI_SZ: c_int = 27;
//
// Maximum number of simultaeneous security contexts
// supported by the driver
//
pub const SA_MAX_NUM_CTX: c_int = 512;
//
// Assumption: CTX size is multiple of 32
//

pub const SA_CTX_ENC_KEY_OFFSET: c_int = 32;
pub const SA_CTX_ENC_AUX1_OFFSET: c_int = 64;
pub const SA_CTX_ENC_AUX2_OFFSET: c_int = 96;
pub const SA_CTX_ENC_AUX3_OFFSET: c_int = 112;
pub const SA_CTX_ENC_AUX4_OFFSET: c_int = 128;
// Next Engine Select code in CP_ACE

//
// Command Label Definitions
//

// 16-bit Length of Data to be processed
pub const SA_CMDL_OFFSET_DATA_LEN: c_int = 2;

pub const SA_CMDL_OFFSET_OPTION_BYTE: c_int = 8;
pub const SA_CMDL_HEADER_SIZE_BYTES: c_int = 8;
pub const SA_CMDL_OPTION_BYTES_MAX_SIZE: c_int = 72;

// SWINFO word-0 flags
pub const SA_SW_INFO_FLAG_EVICT: c_uint = 0x0001;
pub const SA_SW_INFO_FLAG_TEAR: c_uint = 0x0002;
pub const SA_SW_INFO_FLAG_NOPD: c_uint = 0x0004;
//
// This type represents the various packet types to be processed
// by the PHP engine in SA.
// It is used to identify the corresponding PHP processing function.
//

// IPSec Encapsulating Security Payload
pub const SA_CTX_PE_PKT_TYPE_IPSEC_ESP: c_int = 3;
// Indicates that it is in data mode, It may not be used by PHP
pub const SA_CTX_PE_PKT_TYPE_NONE: c_int = 4;

// Size of security context for PHP engine
pub const SA_CTX_PHP_PE_CTX_SZ: c_int = 64;

//
// Encoding of F/E control in SCCTL
// Bit 0-1: Fetch PHP Bytes
// Bit 2-3: Fetch Encryption/Air Ciphering Bytes
// Bit 4-5: Fetch Authentication Bytes or Encr pass 2
// Bit 6-7: Evict PHP Bytes
//
// where   00 = 0 bytes
// 01 = 64 bytes
// 10 = 96 bytes
// 11 = 128 bytes
//
pub const SA_CTX_DMA_SIZE_0: c_int = 0;
pub const SA_CTX_DMA_SIZE_64: c_int = 1;
pub const SA_CTX_DMA_SIZE_96: c_int = 2;
pub const SA_CTX_DMA_SIZE_128: c_int = 3;
//
// Byte offset of the owner word in SCCTL
// in the security context
//
pub const SA_CTX_SCCTL_OWNER_OFFSET: c_int = 0;
pub const SA_CTX_ENC_KEY_OFFSET: c_int = 32;
pub const SA_CTX_ENC_AUX1_OFFSET: c_int = 64;
pub const SA_CTX_ENC_AUX2_OFFSET: c_int = 96;
pub const SA_CTX_ENC_AUX3_OFFSET: c_int = 112;
pub const SA_CTX_ENC_AUX4_OFFSET: c_int = 128;
pub const SA_SCCTL_FE_AUTH_ENC: c_uint = 0x65;
pub const SA_SCCTL_FE_ENC: c_uint = 0x8D;

pub const SA_AUTH_SW_CTRL_MD5: c_int = 1;
pub const SA_AUTH_SW_CTRL_SHA1: c_int = 2;
pub const SA_AUTH_SW_CTRL_SHA224: c_int = 3;
pub const SA_AUTH_SW_CTRL_SHA256: c_int = 4;
pub const SA_AUTH_SW_CTRL_SHA384: c_int = 5;
pub const SA_AUTH_SW_CTRL_SHA512: c_int = 6;
// SA2UL can only handle maximum data size of 64KB

//
// SA2UL can provide unpredictable results with packet sizes that fall
// the following range, so avoid using it.
//
pub const SA_UNSAFE_DATA_SZ_MIN: c_int = 240;
pub const SA_UNSAFE_DATA_SZ_MAX: c_int = 255;
//
// struct sa_crypto_data - Crypto driver instance data
// @base: Base address of the register space
// @soc_data: Pointer to SoC specific data
// @pdev: Platform device pointer
// @sc_pool: security context pool
// @dev: Device pointer
// @scid_lock: secure context ID lock
// @sc_id_start: starting index for SC ID
// @sc_id_end: Ending index for SC ID
// @sc_id: Security Context ID
// @ctx_bm: Bitmap to keep track of Security context ID's
// @ctx: SA tfm context pointer
// @dma_rx1: Pointer to DMA rx channel for sizes < 256 Bytes
// @dma_rx2: Pointer to DMA rx channel for sizes > 256 Bytes
// @dma_tx: Pointer to DMA TX channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_crypto_data {
    pub base: *mut void __iomem,
    pub match_data: *const sa_match_data,
    pub pdev: *mut platform_device,
    pub sc_pool: *mut dma_pool,
    pub dev: *mut device,
    pub /: *mut *mut spinlock_t scid_lock; / lock for SC-ID allocation,
// Security context data
    pub sc_id_start: u16,
    pub sc_id_end: u16,
    pub sc_id: u16,
    pub ctx: *mut sa_tfm_ctx,
    pub dma_rx1: *mut dma_chan,
    pub dma_rx2: *mut dma_chan,
    pub dma_tx: *mut dma_chan,
}

//
// struct sa_cmdl_param_info: Command label parameters info
// @index: Index of the parameter in the command label format
// @offset: the offset of the parameter
// @size: Size of the parameter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_cmdl_param_info {
    pub index: u16,
    pub offset: u16,
    pub size: u16,
}

// Maximum length of Auxiliary data in 32bit words
pub const SA_MAX_AUX_DATA_WORDS: c_int = 8;
//
// struct sa_cmdl_upd_info: Command label updation info
// @flags: flags in command label
// @submode: Encryption submodes
// @enc_size: Size of first pass encryption size
// @enc_size2: Size of second pass encryption size
// @enc_offset: Encryption payload offset in the packet
// @enc_iv: Encryption initialization vector for pass2
// @enc_iv2: Encryption initialization vector for pass2
// @aad: Associated data
// @payload: Payload info
// @auth_size: Authentication size for pass 1
// @auth_size2: Authentication size for pass 2
// @auth_offset: Authentication payload offset
// @auth_iv: Authentication initialization vector
// @aux_key_info: Authentication aux key information
// @aux_key: Aux key for authentication
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_cmdl_upd_info {
    pub flags: u16,
    pub submode: u16,
    pub enc_size: sa_cmdl_param_info,
    pub enc_size2: sa_cmdl_param_info,
    pub enc_offset: sa_cmdl_param_info,
    pub enc_iv: sa_cmdl_param_info,
    pub enc_iv2: sa_cmdl_param_info,
    pub aad: sa_cmdl_param_info,
    pub payload: sa_cmdl_param_info,
    pub auth_size: sa_cmdl_param_info,
    pub auth_size2: sa_cmdl_param_info,
    pub auth_offset: sa_cmdl_param_info,
    pub auth_iv: sa_cmdl_param_info,
    pub aux_key_info: sa_cmdl_param_info,
    pub aux_key: [u32; SA_MAX_AUX_DATA_WORDS],
}

//
// Number of 32bit words appended after the command label
// in PSDATA to identify the crypto request context.
// word-0: Request type
// word-1: pointer to request
//
pub const SA_PSDATA_CTX_WORDS: c_int = 4;
// Maximum size of Command label in 32 words

//
// struct sa_ctx_info: SA context information
// @sc: Pointer to security context
// @sc_phys: Security context physical address that is passed on to SA2UL
// @sc_id: Security context ID
// @cmdl_size: Command label size
// @cmdl: Command label for a particular iteration
// @cmdl_upd_info: structure holding command label updation info
// @epib: Extended protocol information block words
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_ctx_info {
    pub sc: *mut u8,
    pub sc_phys: dma_addr_t,
    pub sc_id: u16,
    pub cmdl_size: u16,
    pub cmdl: [u32; SA_MAX_CMDL_WORDS],
    pub cmdl_upd_info: sa_cmdl_upd_info,
// Store Auxiliary data such as K2/K3 subkeys in AES-XCBC
    pub epib: [u32; SA_DMA_NUM_EPIB_WORDS],
}

//
// struct sa_tfm_ctx: TFM context structure
// @dev_data: struct sa_crypto_data pointer
// @enc: struct sa_ctx_info for encryption
// @dec: struct sa_ctx_info for decryption
// @keylen: encrption/decryption keylength
// @iv_idx: Initialization vector index
// @key: encryption key
// @fallback: SW fallback algorithm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_tfm_ctx {
    pub dev_data: *mut sa_crypto_data,
    pub enc: sa_ctx_info,
    pub dec: sa_ctx_info,
    pub auth: sa_ctx_info,
    pub keylen: c_int,
    pub iv_idx: c_int,
    pub sizeof(u32)]: u32 key[AES_KEYSIZE_256 /,
    pub authkey: [u8; SHA512_BLOCK_SIZE],
    pub shash: *mut crypto_shash,
// for fallback
    pub skcipher: *mut crypto_skcipher,
    pub ahash: *mut crypto_ahash,
    pub aead: *mut crypto_aead,
    pub fallback: },
}

//
// struct sa_sha_req_ctx: Structure used for sha request
// @dev_data: struct sa_crypto_data pointer
// @cmdl: Complete command label with psdata and epib included
// @fallback_req: SW fallback request container
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_sha_req_ctx {
    pub dev_data: *mut sa_crypto_data,
    pub SA_PSDATA_CTX_WORDS]: u32 cmdl[SA_MAX_CMDL_WORDS +,
    pub fallback_req: ahash_request,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sa_submode {
    SA_MODE_GEN = 0,
    SA_MODE_CCM,
    SA_MODE_GCM,
    SA_MODE_GMAC
}

// Encryption algorithms
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sa_ealg_id {
    SA_EALG_ID_NONE = 0,        /* No encryption */
    SA_EALG_ID_NULL,            /* NULL encryption */
    SA_EALG_ID_AES_CTR,         /* AES Counter mode */
    SA_EALG_ID_AES_F8,          /* AES F8 mode */
    SA_EALG_ID_AES_CBC,         /* AES CBC mode */
    SA_EALG_ID_DES_CBC,         /* DES CBC mode */
    SA_EALG_ID_3DES_CBC,        /* 3DES CBC mode */
    SA_EALG_ID_CCM,             /* Counter with CBC-MAC mode */
    SA_EALG_ID_GCM,             /* Galois Counter mode */
    SA_EALG_ID_AES_ECB,
    SA_EALG_ID_LAST
}

// Authentication algorithms
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sa_aalg_id {
    SA_AALG_ID_NONE = 0,      /* No Authentication  */
    SA_AALG_ID_NULL = SA_EALG_ID_LAST, /* NULL Authentication  */
    SA_AALG_ID_MD5,           /* MD5 mode */
    SA_AALG_ID_SHA1,          /* SHA1 mode */
    SA_AALG_ID_SHA2_224,      /* 224-bit SHA2 mode */
    SA_AALG_ID_SHA2_256,      /* 256-bit SHA2 mode */
    SA_AALG_ID_SHA2_512,      /* 512-bit SHA2 mode */
    SA_AALG_ID_HMAC_MD5,      /* HMAC with MD5 mode */
    SA_AALG_ID_HMAC_SHA1,     /* HMAC with SHA1 mode */
    SA_AALG_ID_HMAC_SHA2_224, /* HMAC with 224-bit SHA2 mode */
    SA_AALG_ID_HMAC_SHA2_256, /* HMAC with 256-bit SHA2 mode */
    SA_AALG_ID_GMAC,          /* Galois Message Auth. Code mode */
    SA_AALG_ID_CMAC,          /* Cipher-based Mes. Auth. Code mode */
    SA_AALG_ID_CBC_MAC,       /* Cipher Block Chaining */
    SA_AALG_ID_AES_XCBC       /* AES Extended Cipher Block Chaining */
}

//
// Mode control engine algorithms used to index the
// mode control instruction tables
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sa_eng_algo_id {
    SA_ENG_ALGO_ECB = 0,
    SA_ENG_ALGO_CBC,
    SA_ENG_ALGO_CFB,
    SA_ENG_ALGO_OFB,
    SA_ENG_ALGO_CTR,
    SA_ENG_ALGO_F8,
    SA_ENG_ALGO_F8F9,
    SA_ENG_ALGO_GCM,
    SA_ENG_ALGO_GMAC,
    SA_ENG_ALGO_CCM,
    SA_ENG_ALGO_CMAC,
    SA_ENG_ALGO_CBCMAC,
    SA_NUM_ENG_ALGOS
}

//
// struct sa_eng_info: Security accelerator engine info
// @eng_id: Engine ID
// @sc_size: security context size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_eng_info {
    pub eng_id: u8,
    pub sc_size: u16,
}
