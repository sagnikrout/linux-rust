//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/bcm/spum.h
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
// Copyright 2016 Broadcom
//
// This file contains SPU message definitions specific to SPU-M.
//
pub const SPU_CRYPTO_OPERATION_GENERIC: c_uint = 0x1;
// Length of STATUS field in tx and rx packets
pub const SPU_TX_STATUS_LEN: c_int = 4;
// SPU-M error codes
pub const SPU_STATUS_MASK: c_uint = 0x0000FF00;
pub const SPU_STATUS_SUCCESS: c_uint = 0x00000000;
pub const SPU_STATUS_INVALID_ICV: c_uint = 0x00000100;
pub const SPU_STATUS_ERROR_FLAG: c_uint = 0x00020000;
// Request message. MH + EMH + BDESC + BD header
pub const SPU_REQ_FIXED_LEN: c_int = 24;
//
// Max length of a SPU message header. Used to allocate a buffer where
// the SPU message header is constructed. Can be used for either a SPU-M
// header or a SPU2 header.
// For SPU-M, sum of the following:
// MH - 4 bytes
// EMH - 4
// SCTX - 3 +
// max auth key len - 64
// max cipher key len - 264 (RC4)
// max IV len - 16
// BDESC - 12
// BD header - 4
// Total:  371
//
// For SPU2, FMD_SIZE (32) plus lengths of hash and cipher keys,
// hash and cipher IVs. If SPU2 does not support RC4, then
//

//
// Response message header length. Normally MH, EMH, BD header, but when
// BD_SUPPRESS is used for hash requests, there is no BD header.
//
pub const SPU_RESP_HDR_LEN: c_int = 12;
pub const SPU_HASH_RESP_HDR_LEN: c_int = 8;
//
// Max value that can be represented in the Payload Length field of the BD
// header. This is a 16-bit field.
//

//
// NSP SPU is limited to ~9KB because of FA2 FIFO size limitations;
// Set MAX_PAYLOAD to 8k to allow for addition of header, digest, etc.
// and stay within limitation.
//
pub const SPUM_NSP_MAX_PAYLOAD: c_int = 8192;
// Buffer Descriptor Header [BDESC]. SPU in big-endian mode.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BDESC_HEADER {
    pub /: *mut *mut __be16 offset_mac; / word 0 [31-16],
    pub /: *mut *mut __be16 length_mac; / word 0 [15-0],
    pub /: *mut *mut __be16 offset_crypto; / word 1 [31-16],
    pub /: *mut *mut __be16 length_crypto; / word 1 [15-0],
    pub /: *mut *mut __be16 offset_icv; / word 2 [31-16],
    pub /: *mut *mut __be16 offset_iv; / word 2 [15-0],
}

// Buffer Data Header [BD]. SPU in big-endian mode.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BD_HEADER {
    pub size: __be16,
    pub prev_length: __be16,
}

// Command Context Header. SPU-M in big endian mode.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MHEADER {
    pub /: *mut *mut u8 flags; / [31:24],
    pub /: *mut *mut u8 op_code; / [23:16],
    pub /: *mut *mut u16 reserved; / [15:0],
}

// MH header flags bits

// SCTX word 0 bit offsets and fields masks
pub const SCTX_SIZE: c_uint = 0x000000FF;
// SCTX word 1 bit shifts and field masks
pub const UPDT_OFST: c_uint = 0x000000FF   /* offset of SCTX updateable fld */;
pub const HASH_TYPE: c_uint = 0x00000300   /* hash alg operation type */;
pub const HASH_TYPE_SHIFT: c_int = 8;
pub const HASH_MODE: c_uint = 0x00001C00   /* one of spu2_hash_mode */;
pub const HASH_MODE_SHIFT: c_int = 10;
pub const HASH_ALG: c_uint = 0x0000E000   /* hash algorithm */;
pub const HASH_ALG_SHIFT: c_int = 13;
pub const CIPHER_TYPE: c_uint = 0x00030000   /* encryption operation type */;
pub const CIPHER_TYPE_SHIFT: c_int = 16;
pub const CIPHER_MODE: c_uint = 0x001C0000   /* encryption mode */;
pub const CIPHER_MODE_SHIFT: c_int = 18;
pub const CIPHER_ALG: c_uint = 0x00E00000   /* encryption algo */;
pub const CIPHER_ALG_SHIFT: c_int = 21;

pub const ICV_IS_512_SHIFT: c_int = 27;

pub const CIPHER_ORDER_SHIFT: c_int = 30;

pub const CIPHER_INBOUND_SHIFT: c_int = 31;
// SCTX word 2 bit shifts and field masks
pub const EXP_IV_SIZE: c_uint = 0x7;

pub const IV_OFFSET_SHIFT: c_int = 3;

pub const GEN_IV_SHIFT: c_int = 5;

pub const EXPLICIT_IV_SHIFT: c_int = 6;

pub const SCTX_IV_SHIFT: c_int = 7;
pub const ICV_SIZE: c_uint = 0x0F00;
pub const ICV_SIZE_SHIFT: c_int = 8;

pub const CHECK_ICV_SHIFT: c_int = 12;

pub const INSERT_ICV_SHIFT: c_int = 13;

pub const BD_SUPPRESS_SHIFT: c_int = 19;
// Generic Mode Security Context Structure [SCTX]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SCTX {
// word 0: protocol flags
    pub proto_flags: __be32,
// word 1: cipher flags
    pub cipher_flags: __be32,
// word 2: Extended cipher flags
    pub ecf: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SPUHEADER {
    pub mh: MHEADER,
    pub emh: u32,
    pub sa: SCTX,
}
