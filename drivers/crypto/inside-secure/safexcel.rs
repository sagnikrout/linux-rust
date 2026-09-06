//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/inside-secure/safexcel.h
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
// Copyright (C) 2017 Marvell
//
// Antoine Tenart <antoine.tenart@free-electrons.com>
//

pub const EIP197_HIA_VERSION_BE: c_uint = 0xca35;
pub const EIP197_HIA_VERSION_LE: c_uint = 0x35ca;
pub const EIP97_VERSION_LE: c_uint = 0x9e61;
pub const EIP196_VERSION_LE: c_uint = 0x3bc4;
pub const EIP197_VERSION_LE: c_uint = 0x3ac5;
pub const EIP96_VERSION_LE: c_uint = 0x9f60;
pub const EIP201_VERSION_LE: c_uint = 0x36c9;
pub const EIP206_VERSION_LE: c_uint = 0x31ce;
pub const EIP207_VERSION_LE: c_uint = 0x30cf;

// EIP197 HIA OPTIONS ENCODING

// EIP206 OPTIONS ENCODING

// EIP197 OPTIONS ENCODING

// Static configuration
pub const EIP197_DEFAULT_RING_SIZE: c_int = 400;

pub const EIP197_MAX_TOKENS: c_int = 16;
pub const EIP197_MAX_RINGS: c_int = 4;
pub const EIP197_FETCH_DEPTH: c_int = 2;
pub const EIP197_MAX_BATCH_SZ: c_int = 64;
pub const EIP197_MAX_RING_AIC: c_int = 14;

// Custom on-stack requests (for invalidation)

// Xilinx dev board base offsets
pub const EIP197_XLX_GPIO_BASE: c_uint = 0x200000;
pub const EIP197_XLX_IRQ_BLOCK_ID_ADDR: c_uint = 0x2000;
pub const EIP197_XLX_IRQ_BLOCK_ID_VALUE: c_uint = 0x1fc2;
pub const EIP197_XLX_USER_INT_ENB_MSK: c_uint = 0x2004;
pub const EIP197_XLX_USER_INT_ENB_SET: c_uint = 0x2008;
pub const EIP197_XLX_USER_INT_ENB_CLEAR: c_uint = 0x200c;
pub const EIP197_XLX_USER_INT_BLOCK: c_uint = 0x2040;
pub const EIP197_XLX_USER_INT_PEND: c_uint = 0x2048;
pub const EIP197_XLX_USER_VECT_LUT0_ADDR: c_uint = 0x2080;
pub const EIP197_XLX_USER_VECT_LUT0_IDENT: c_uint = 0x03020100;
pub const EIP197_XLX_USER_VECT_LUT1_ADDR: c_uint = 0x2084;
pub const EIP197_XLX_USER_VECT_LUT1_IDENT: c_uint = 0x07060504;
pub const EIP197_XLX_USER_VECT_LUT2_ADDR: c_uint = 0x2088;
pub const EIP197_XLX_USER_VECT_LUT2_IDENT: c_uint = 0x0b0a0908;
pub const EIP197_XLX_USER_VECT_LUT3_ADDR: c_uint = 0x208c;
pub const EIP197_XLX_USER_VECT_LUT3_IDENT: c_uint = 0x0f0e0d0c;
// Helper defines for probe function

// Register base offsets

// EIP197 base offsets
pub const EIP197_HIA_AIC_BASE: c_uint = 0x90000;
pub const EIP197_HIA_AIC_G_BASE: c_uint = 0x90000;
pub const EIP197_HIA_AIC_R_BASE: c_uint = 0x90800;
pub const EIP197_HIA_AIC_xDR_BASE: c_uint = 0x80000;
pub const EIP197_HIA_DFE_BASE: c_uint = 0x8c000;
pub const EIP197_HIA_DFE_THR_BASE: c_uint = 0x8c040;
pub const EIP197_HIA_DSE_BASE: c_uint = 0x8d000;
pub const EIP197_HIA_DSE_THR_BASE: c_uint = 0x8d040;
pub const EIP197_HIA_GEN_CFG_BASE: c_uint = 0xf0000;
pub const EIP197_PE_BASE: c_uint = 0xa0000;
pub const EIP197_GLOBAL_BASE: c_uint = 0xf0000;
// EIP97 base offsets
pub const EIP97_HIA_AIC_BASE: c_uint = 0x0;
pub const EIP97_HIA_AIC_G_BASE: c_uint = 0x0;
pub const EIP97_HIA_AIC_R_BASE: c_uint = 0x0;
pub const EIP97_HIA_AIC_xDR_BASE: c_uint = 0x0;
pub const EIP97_HIA_DFE_BASE: c_uint = 0xf000;
pub const EIP97_HIA_DFE_THR_BASE: c_uint = 0xf200;
pub const EIP97_HIA_DSE_BASE: c_uint = 0xf400;
pub const EIP97_HIA_DSE_THR_BASE: c_uint = 0xf600;
pub const EIP97_HIA_GEN_CFG_BASE: c_uint = 0x10000;
pub const EIP97_PE_BASE: c_uint = 0x10000;
pub const EIP97_GLOBAL_BASE: c_uint = 0x10000;
// CDR/RDR register offsets

pub const EIP197_HIA_xDR_RING_BASE_ADDR_LO: c_uint = 0x0000;
pub const EIP197_HIA_xDR_RING_BASE_ADDR_HI: c_uint = 0x0004;
pub const EIP197_HIA_xDR_RING_SIZE: c_uint = 0x0018;
pub const EIP197_HIA_xDR_DESC_SIZE: c_uint = 0x001c;
pub const EIP197_HIA_xDR_CFG: c_uint = 0x0020;
pub const EIP197_HIA_xDR_DMA_CFG: c_uint = 0x0024;
pub const EIP197_HIA_xDR_THRESH: c_uint = 0x0028;
pub const EIP197_HIA_xDR_PREP_COUNT: c_uint = 0x002c;
pub const EIP197_HIA_xDR_PROC_COUNT: c_uint = 0x0030;
pub const EIP197_HIA_xDR_PREP_PNTR: c_uint = 0x0034;
pub const EIP197_HIA_xDR_PROC_PNTR: c_uint = 0x0038;
pub const EIP197_HIA_xDR_STAT: c_uint = 0x003c;
// register offsets

pub const EIP197_HIA_RA_PE_STAT: c_uint = 0x0014;

pub const EIP197_HIA_AIC_G_ENABLE_CTRL: c_uint = 0xf808;
pub const EIP197_HIA_AIC_G_ENABLED_STAT: c_uint = 0xf810;
pub const EIP197_HIA_AIC_G_ACK: c_uint = 0xf810;
pub const EIP197_HIA_MST_CTRL: c_uint = 0xfff4;
pub const EIP197_HIA_OPTIONS: c_uint = 0xfff8;
pub const EIP197_HIA_VERSION: c_uint = 0xfffc;

pub const EIP197_MST_CTRL: c_uint = 0xfff4;
pub const EIP197_OPTIONS: c_uint = 0xfff8;
pub const EIP197_VERSION: c_uint = 0xfffc;
// EIP197-specific registers, no indirection
pub const EIP197_CLASSIFICATION_RAMS: c_uint = 0xe0000;
pub const EIP197_TRC_CTRL: c_uint = 0xf0800;
pub const EIP197_TRC_LASTRES: c_uint = 0xf0804;
pub const EIP197_TRC_REGINDEX: c_uint = 0xf0808;
pub const EIP197_TRC_PARAMS: c_uint = 0xf0820;
pub const EIP197_TRC_FREECHAIN: c_uint = 0xf0824;
pub const EIP197_TRC_PARAMS2: c_uint = 0xf0828;
pub const EIP197_TRC_ECCCTRL: c_uint = 0xf0830;
pub const EIP197_TRC_ECCSTAT: c_uint = 0xf0834;
pub const EIP197_TRC_ECCADMINSTAT: c_uint = 0xf0838;
pub const EIP197_TRC_ECCDATASTAT: c_uint = 0xf083c;
pub const EIP197_TRC_ECCDATA: c_uint = 0xf0840;
pub const EIP197_STRC_CONFIG: c_uint = 0xf43f0;

pub const EIP197_FLUE_OFFSETS: c_uint = 0xf6808;
pub const EIP197_FLUE_ARC4_OFFSET: c_uint = 0xf680c;

pub const EIP197_CS_RAM_CTRL: c_uint = 0xf7ff0;
// EIP197_HIA_xDR_DESC_SIZE

// EIP197_HIA_xDR_DMA_CFG

// EIP197_HIA_CDR_THRESH

// EIP197_HIA_RDR_THRESH

// EIP197_HIA_xDR_PREP_COUNT

// EIP197_HIA_xDR_PROC_COUNT
pub const EIP197_xDR_PROC_xD_PKT_OFFSET: c_int = 24;

// EIP197_HIA_xDR_STAT

// EIP197_HIA_OPTIONS
pub const EIP197_N_RINGS_OFFSET: c_int = 0;

pub const EIP197_N_PES_OFFSET: c_int = 4;

pub const EIP197_HWDATAW_OFFSET: c_int = 25;

pub const EIP197_CFSIZE_OFFSET: c_int = 9;
pub const EIP197_CFSIZE_ADJUST: c_int = 4;
pub const EIP97_CFSIZE_OFFSET: c_int = 8;

pub const EIP197_RFSIZE_OFFSET: c_int = 12;
pub const EIP197_RFSIZE_ADJUST: c_int = 4;
pub const EIP97_RFSIZE_OFFSET: c_int = 12;

// EIP197_HIA_AIC_R_ENABLE_CTRL

// EIP197_HIA_DFE/DSE_CFG

// EIP197_HIA_DFE/DSE_THR_CTRL

// EIP197_PE_ICE_PUE/FPP_CTRL

pub const EIP197_PE_ICE_UENG_INIT_ALIGN_MASK: c_uint = 0x7ff0;

// EIP197_HIA_AIC_G_ENABLED_STAT

// EIP197_HIA_MST_CTRL
pub const RD_CACHE_3BITS: c_uint = 0x5;
pub const WR_CACHE_3BITS: c_uint = 0x3;

// EIP197_PE_IN_DBUF/TBUF_THRES

// EIP197_PE_OUT_DBUF_THRES

// EIP197_PE_ICE_SCRATCH_CTRL

// EIP197_PE_ICE_SCRATCH_RAM
pub const EIP197_NUM_OF_SCRATCH_BLOCKS: c_int = 32;
// EIP197_PE_ICE_PUE/FPP_CTRL

// EIP197_PE_ICE_RAM_CTRL

// EIP197_PE_EIP96_TOKEN_CTRL

// EIP197_PE_EIP96_FUNCTION_EN
pub const EIP197_FUNCTION_ALL: c_uint = 0xffffffff;
// EIP197_PE_EIP96_CONTEXT_CTRL

// EIP197_PE_EIP96_TOKEN_CTRL2

// EIP197_PE_DEBUG

// EIP197_STRC_CONFIG

// EIP197_FLUE_CONFIG
pub const EIP197_FLUE_CONFIG_MAGIC: c_uint = 0xc7000004;
// Context Control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_context_record {
    pub control0: __le32,
    pub control1: __le32,
    pub data: [__le32; 40],
    pub __packed: },
// control0
pub const CONTEXT_CONTROL_TYPE_NULL_OUT: c_uint = 0x0;
pub const CONTEXT_CONTROL_TYPE_NULL_IN: c_uint = 0x1;
pub const CONTEXT_CONTROL_TYPE_HASH_OUT: c_uint = 0x2;
pub const CONTEXT_CONTROL_TYPE_HASH_IN: c_uint = 0x3;
pub const CONTEXT_CONTROL_TYPE_CRYPTO_OUT: c_uint = 0x4;
pub const CONTEXT_CONTROL_TYPE_CRYPTO_IN: c_uint = 0x5;
pub const CONTEXT_CONTROL_TYPE_ENCRYPT_HASH_OUT: c_uint = 0x6;
pub const CONTEXT_CONTROL_TYPE_DECRYPT_HASH_IN: c_uint = 0x7;
pub const CONTEXT_CONTROL_TYPE_HASH_ENCRYPT_OUT: c_uint = 0xe;
pub const CONTEXT_CONTROL_TYPE_HASH_DECRYPT_IN: c_uint = 0xf;

// control1

pub const EIP197_XCM_MODE_GCM: c_int = 1;
pub const EIP197_XCM_MODE_CCM: c_int = 2;
pub const EIP197_AEAD_TYPE_IPSEC_ESP: c_int = 2;
pub const EIP197_AEAD_TYPE_IPSEC_ESP_GMAC: c_int = 3;
pub const EIP197_AEAD_IPSEC_IV_SIZE: c_int = 8;
pub const EIP197_AEAD_IPSEC_NONCE_SIZE: c_int = 4;
pub const EIP197_AEAD_IPSEC_COUNTER_SIZE: c_int = 4;
pub const EIP197_AEAD_IPSEC_CCM_NONCE_SIZE: c_int = 3;
// The hash counter given to the engine in the context has a granularity of
// 64 bits.
//
pub const EIP197_COUNTER_BLOCK_SIZE: c_int = 64;
// EIP197_CS_RAM_CTRL

pub const EIP197_CS_BANKSEL_OFS: c_int = 12;
// EIP197_TRC_PARAMS

// EIP197_TRC_FREECHAIN

// EIP197_TRC_PARAMS2

// Cache helpers
pub const EIP197_MIN_DSIZE: c_int = 1024;
pub const EIP197_MIN_ASIZE: c_int = 8;
pub const EIP197_CS_TRC_REC_WC: c_int = 64;

pub const EIP197_RC_NULL: c_uint = 0x3ff;
// Result data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct result_data_desc {
    pub packet_length:17: u32,
    pub error_code:15: u32,
    pub bypass_length:4: u32,
    pub e15:1: u32,
    pub rsvd0:16: u32,
    pub hash_bytes:1: u32,
    pub hash_length:6: u32,
    pub generic_bytes:1: u32,
    pub checksum:1: u32,
    pub next_header:1: u32,
    pub length:1: u32,
    pub application_id: u16,
    pub rsvd1: u16,
    pub rsvd2: [u32; 5],
    pub __packed: },
// Basic Result Descriptor format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_result_desc {
    pub particle_size:17: u32,
    pub rsvd0:3: u8,
    pub descriptor_overflow:1: u8,
    pub buffer_overflow:1: u8,
    pub last_seg:1: u8,
    pub first_seg:1: u8,
    pub result_size:8: u16,
    pub rsvd1: u32,
    pub data_lo: u32,
    pub data_hi: u32,
    pub __packed: },
//
// The EIP(1)97 only needs to fetch the descriptor part of
// the result descriptor, not the result token part!
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_token {
    pub packet_length:17: u32,
    pub stat:2: u8,
    pub instructions:9: u16,
    pub opcode:4: u8,
    pub __packed: },

pub const EIP197_TOKEN_OPCODE_DIRECTION: c_uint = 0x0;
pub const EIP197_TOKEN_OPCODE_INSERT: c_uint = 0x2;

pub const EIP197_TOKEN_OPCODE_RETRIEVE: c_uint = 0x4;
pub const EIP197_TOKEN_OPCODE_INSERT_REMRES: c_uint = 0xa;
pub const EIP197_TOKEN_OPCODE_VERIFY: c_uint = 0xd;
pub const EIP197_TOKEN_OPCODE_CTX_ACCESS: c_uint = 0xe;

    pub EIP197_TOKEN_OPCODE_NOOP: token->opcode =,
    pub BIT(2): token->packet_length =,
    pub 0: token->stat =,
    pub 0: token->instructions =,
// Instructions
pub const EIP197_TOKEN_INS_INSERT_HASH_DIGEST: c_uint = 0x1c;
pub const EIP197_TOKEN_INS_ORIGIN_IV0: c_uint = 0x14;
pub const EIP197_TOKEN_INS_ORIGIN_TOKEN: c_uint = 0x1b;

// Processing Engine Control Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_control_data_desc {
    pub packet_length:17: u32,
    pub options:13: u16,
    pub type:2: u8,
    pub application_id: u16,
    pub rsvd: u16,
    pub context_lo: u32,
    pub context_hi: u32,
    pub control0: u32,
    pub control1: u32,
    pub token: [u32; EIP197_EMB_TOKENS],
    pub __packed: },

pub const EIP197_TYPE_BCLA: c_uint = 0x0;
pub const EIP197_TYPE_EXTENDED: c_uint = 0x3;
pub const EIP197_CONTEXT_SMALL: c_uint = 0x2;
pub const EIP197_CONTEXT_SIZE_MASK: c_uint = 0x3;
// Basic Command Descriptor format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_command_desc {
    pub particle_size:17: u32,
    pub rsvd0:5: u8,
    pub last_seg:1: u8,
    pub first_seg:1: u8,
    pub additional_cdata_size:8: u8,
    pub rsvd1: u32,
    pub data_lo: u32,
    pub data_hi: u32,
    pub atok_lo: u32,
    pub atok_hi: u32,
    pub control_data: safexcel_control_data_desc,
    pub __packed: },

//
// Internal structures & functions
//
pub const EIP197_FW_TERMINAL_NOPS: c_int = 2;
pub const EIP197_FW_START_POLLCNT: c_int = 16;
pub const EIP197_FW_PUE_READY: c_uint = 0x14;
pub const EIP197_FW_FPP_READY: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eip197_fw {
    FW_IFPP = 0,
    FW_IPUE,
    FW_NB
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_desc_ring {
    pub base: *mut c_void,
    pub shbase: *mut c_void,
    pub base_end: *mut c_void,
    pub shbase_end: *mut c_void,
    pub base_dma: dma_addr_t,
    pub shbase_dma: dma_addr_t,
// write and read pointers
    pub write: *mut c_void,
    pub shwrite: *mut c_void,
    pub read: *mut c_void,
// descriptor element offset
    pub offset: c_uint,
    pub shoffset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum safexcel_alg_type {
    SAFEXCEL_ALG_TYPE_SKCIPHER,
    SAFEXCEL_ALG_TYPE_AEAD,
    SAFEXCEL_ALG_TYPE_AHASH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_config {
    pub pes: u32,
    pub rings: u32,
    pub cd_size: u32,
    pub cd_offset: u32,
    pub cdsh_offset: u32,
    pub rd_size: u32,
    pub rd_offset: u32,
    pub res_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_work_data {
    pub work: work_struct,
    pub priv: *mut safexcel_crypto_priv,
    pub ring: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_ring {
    pub lock: spinlock_t,
    pub workqueue: *mut workqueue_struct,
    pub work_data: safexcel_work_data,
// command/result rings
    pub cdr: safexcel_desc_ring,
    pub rdr: safexcel_desc_ring,
// result ring crypto API request
    pub rdr_req: *mut crypto_async_request,
// queue
    pub queue: crypto_queue,
    pub queue_lock: spinlock_t,
// Number of requests in the engine.
    pub requests: c_int,
// The ring is currently handling at least one request
    pub busy: bool,
// Store for current requests when bailing out of the dequeueing
// function when no enough resources are available.
//
    pub req: *mut crypto_async_request,
    pub backlog: *mut crypto_async_request,
// irq of this ring
    pub irq: c_int,
}

// EIP integration context flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum safexcel_eip_version {
// Platform (EIP integration context) specifier
    EIP97IES_MRVL,
    EIP197B_MRVL,
    EIP197D_MRVL,
    EIP197_DEVBRD,
    EIP197C_MXL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_priv_data {
    pub version: safexcel_eip_version,
    pub fw_little_endian: bool,
}

// Priority we use for advertising our algorithms
pub const SAFEXCEL_CRA_PRIORITY: c_int = 300;
// SM3 digest result for zero length message

// EIP algorithm presence flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum safexcel_eip_algorithms {
    SAFEXCEL_ALG_BC0      = BIT(5),
    SAFEXCEL_ALG_SM4      = BIT(6),
    SAFEXCEL_ALG_SM3      = BIT(7),
    SAFEXCEL_ALG_CHACHA20 = BIT(8),
    SAFEXCEL_ALG_POLY1305 = BIT(9),
    SAFEXCEL_SEQMASK_256   = BIT(10),
    SAFEXCEL_SEQMASK_384   = BIT(11),
    SAFEXCEL_ALG_AES      = BIT(12),
    SAFEXCEL_ALG_AES_XFB  = BIT(13),
    SAFEXCEL_ALG_DES      = BIT(15),
    SAFEXCEL_ALG_DES_XFB  = BIT(16),
    SAFEXCEL_ALG_ARC4     = BIT(18),
    SAFEXCEL_ALG_AES_XTS  = BIT(20),
    SAFEXCEL_ALG_WIRELESS = BIT(21),
    SAFEXCEL_ALG_MD5      = BIT(22),
    SAFEXCEL_ALG_SHA1     = BIT(23),
    SAFEXCEL_ALG_SHA2_256 = BIT(25),
    SAFEXCEL_ALG_SHA2_512 = BIT(26),
    SAFEXCEL_ALG_XCBC_MAC = BIT(27),
    SAFEXCEL_ALG_CBC_MAC_ALL = BIT(29),
    SAFEXCEL_ALG_GHASH    = BIT(30),
    SAFEXCEL_ALG_SHA3     = BIT(31),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_register_offsets {
    pub hia_aic: u32,
    pub hia_aic_g: u32,
    pub hia_aic_r: u32,
    pub hia_aic_xdr: u32,
    pub hia_dfe: u32,
    pub hia_dfe_thr: u32,
    pub hia_dse: u32,
    pub hia_dse_thr: u32,
    pub hia_gen_cfg: u32,
    pub pe: u32,
    pub global: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum safexcel_flags {
    EIP197_TRC_CACHE	= BIT(0),
    SAFEXCEL_HW_EIP197	= BIT(1),
    EIP197_PE_ARB		= BIT(2),
    EIP197_ICE		= BIT(3),
    EIP197_SIMPLE_TRC	= BIT(4),
    EIP197_OCE		= BIT(5),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_hwconfig {
    pub algo_flags: safexcel_eip_algorithms,
    pub hwver: c_int,
    pub hiaver: c_int,
    pub ppver: c_int,
    pub icever: c_int,
    pub pever: c_int,
    pub ocever: c_int,
    pub psever: c_int,
    pub hwdataw: c_int,
    pub hwcfsize: c_int,
    pub hwrfsize: c_int,
    pub hwnumpes: c_int,
    pub hwnumrings: c_int,
    pub hwnumraic: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_crypto_priv {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub reg_clk: *mut clk,
    pub config: safexcel_config,
    pub data: *mut safexcel_priv_data,
    pub offsets: safexcel_register_offsets,
    pub hwconfig: safexcel_hwconfig,
    pub flags: u32,
// context DMA pool
    pub context_pool: *mut dma_pool,
    pub ring_used: core::sync::atomic::AtomicI32,
    pub ring: *mut safexcel_ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_context {
    pub results): *mut *mut int commands, int,
    pub ret): *mut c_int,
    pub ctxr: *mut safexcel_context_record,
    pub priv: *mut safexcel_crypto_priv,
    pub ctxr_dma: dma_addr_t,
    pub 4]: __le32 le[SHA3_512_BLOCK_SIZE /,
    pub 4]: __be32 be[SHA3_512_BLOCK_SIZE /,
    pub 4]: u32 word[SHA3_512_BLOCK_SIZE /,
    pub byte: [u8; SHA3_512_BLOCK_SIZE],
    pub opad: } ipad,,
    pub ring: c_int,
    pub needs_inv: bool,
    pub exit_inv: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_ahash_export_state {
    pub len: u64,
    pub processed: u64,
    pub digest: u32,
    pub sizeof(u32)]: u32 state[SHA512_DIGEST_SIZE /,
    pub cache: [u8; HASH_CACHE_SIZE],
}

//
// Template structure to describe the algorithms in order to register them.
// It also has the purpose to contain our private structure and is actually
// the only way I know in this framework to avoid having global pointers...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct safexcel_alg_template {
    pub priv: *mut safexcel_crypto_priv,
    pub type: safexcel_alg_type,
    pub algo_mask: safexcel_eip_algorithms,
    pub skcipher: skcipher_alg,
    pub aead: aead_alg,
    pub ahash: ahash_alg,
    pub alg: },
}

extern "C" {
    pub fn safexcel_dequeue(priv: *mut safexcel_crypto_priv, ring: c_int);
}
extern "C" {
    pub fn safexcel_complete(priv: *mut safexcel_crypto_priv, ring: c_int);
}
extern "C" {
    pub fn safexcel_select_ring(priv: *mut safexcel_crypto_priv) -> c_int;
}
// available algorithms
