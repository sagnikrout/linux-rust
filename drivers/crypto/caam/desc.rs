//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/desc.h
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
// CAAM descriptor composition header
// Definitions to support CAAM descriptor instruction generation
//
// Copyright 2008-2011 Freescale Semiconductor, Inc.
// Copyright 2018, 2025 NXP
//
// 16-byte hardware scatter/gather table
// An 8-byte table exists in the hardware spec, but has never been
// implemented to date. The 8/16 option is selected at RTL-compile-time.
// and this selection is visible in the Compile Time Parameters Register
//
pub const SEC4_SG_LEN_EXT: c_uint = 0x80000000	/* Entry points to table */;
pub const SEC4_SG_LEN_FIN: c_uint = 0x40000000	/* Last entry in table */;
pub const SEC4_SG_BPID_MASK: c_uint = 0x000000ff;
pub const SEC4_SG_BPID_SHIFT: c_int = 16;
pub const SEC4_SG_LEN_MASK: c_uint = 0x3fffffff	/* Excludes EXT and FINAL */;
pub const SEC4_SG_OFFSET_MASK: c_uint = 0x00001fff;
// Max size of any CAAM descriptor in 32-bit words, inclusive of header
pub const MAX_CAAM_DESCSIZE: c_int = 64;
// Block size of any entity covered/uncovered with a KEK/TKEK
pub const KEK_BLOCKSIZE: c_int = 16;
//
// Supported descriptor command types as they show up
// inside a descriptor command word.
//
pub const CMD_SHIFT: c_int = 27;
pub const CMD_MASK: c_uint = 0xf8000000;

// General-purpose class selector for all commands
pub const CLASS_SHIFT: c_int = 25;

//
// Descriptor header command constructs
// Covers shared, job, and trusted descriptor headers
//
// Do Not Run - marks a descriptor inexecutable if there was
// a preceding error somewhere
//
pub const HDR_DNR: c_uint = 0x01000000;
//
// ONE - should always be set. Combination of ONE (always
// set) and ZRO (always clear) forms an endianness sanity check
//
pub const HDR_ONE: c_uint = 0x00800000;
pub const HDR_ZRO: c_uint = 0x00008000;
// Start Index or SharedDesc Length
pub const HDR_START_IDX_SHIFT: c_int = 16;

// If shared descriptor header, 6-bit length
pub const HDR_DESCLEN_SHR_MASK: c_uint = 0x3f;
// If non-shared header, 7-bit length
pub const HDR_DESCLEN_MASK: c_uint = 0x7f;
// This is a TrustedDesc (if not SharedDesc)
pub const HDR_TRUSTED: c_uint = 0x00004000;
// Make into TrustedDesc (if not SharedDesc)
pub const HDR_MAKE_TRUSTED: c_uint = 0x00002000;
// Save context if self-shared (if SharedDesc)
pub const HDR_SAVECTX: c_uint = 0x00001000;
// Next item points to SharedDesc
pub const HDR_SHARED: c_uint = 0x00001000;
//
// Reverse Execution Order - execute JobDesc first, then
// execute SharedDesc (normally SharedDesc goes first).
//
pub const HDR_REVERSE: c_uint = 0x00000800;
// Propagate DNR property to SharedDesc
pub const HDR_PROP_DNR: c_uint = 0x00000800;
// JobDesc/SharedDesc share property
pub const HDR_SD_SHARE_SHIFT: c_int = 8;

pub const HDR_JD_SHARE_SHIFT: c_int = 8;

// JobDesc/SharedDesc descriptor length
pub const HDR_JD_LENGTH_MASK: c_uint = 0x7f;
pub const HDR_SD_LENGTH_MASK: c_uint = 0x3f;
//
// KEY/SEQ_KEY Command Constructs
//
// Key Destination Class: 01 = Class 1, 02 - Class 2

// Scatter-Gather Table/Variable Length Field
pub const KEY_SGF: c_uint = 0x01000000;
pub const KEY_VLF: c_uint = 0x01000000;
// Immediate - Key follows command in the descriptor
pub const KEY_IMM: c_uint = 0x00800000;
//
// Encrypted - Key is encrypted either with the KEK, or
// with the TDKEK if TK is set
//
pub const KEY_ENC: c_uint = 0x00400000;
//
// No Write Back - Do not allow key to be FIFO STOREd
//
pub const KEY_NWB: c_uint = 0x00200000;
//
// Enhanced Encryption of Key
//
pub const KEY_EKT: c_uint = 0x00100000;
pub const KEY_EKT_OFFSET: c_int = 20;
//
// Encrypted with Trusted Key
//
pub const KEY_TK: c_uint = 0x00008000;
//
// KDEST - Key Destination: 0 - class key register,
// 1 - PKHA 'e', 2 - AFHA Sbox, 3 - MDHA split-key
//
pub const KEY_DEST_SHIFT: c_int = 16;

// Length in bytes
pub const KEY_LENGTH_MASK: c_uint = 0x000003ff;
//
// LOAD/SEQ_LOAD/STORE/SEQ_STORE Command Constructs
//
// Load/Store Destination: 0 = class independent CCB,
// 1 = class 1 CCB, 2 = class 2 CCB, 3 = DECO
//
pub const LDST_CLASS_SHIFT: c_int = 25;

// Scatter-Gather Table/Variable Length Field
pub const LDST_SGF: c_uint = 0x01000000;

// Immediate - Key follows this command in descriptor
pub const LDST_IMM_MASK: c_int = 1;
pub const LDST_IMM_SHIFT: c_int = 23;

// SRC/DST - Destination for LOAD, Source for STORE
pub const LDST_SRCDST_SHIFT: c_int = 16;

// Offset in source/destination
pub const LDST_OFFSET_SHIFT: c_int = 8;

// LDOFF definitions used when DST = LDST_SRCDST_WORD_DECOCTRL
// These could also be shifted by LDST_OFFSET_SHIFT - this reads better
pub const LDOFF_CHG_SHARE_SHIFT: c_int = 0;

pub const LDOFF_CHG_NONSEQLIODN_SHIFT: c_int = 4;

pub const LDOFF_CHG_SEQLIODN_SHIFT: c_int = 6;

// Data length in bytes
pub const LDST_LEN_SHIFT: c_int = 0;

// Special Length definitions when dst=deco-ctrl

pub const LDLEN_SET_OFIFO_OFFSET_SHIFT: c_int = 0;

// Special Length definitions when dst=sm, nfifo-{sm,m}
pub const LDLEN_MATH0: c_int = 0;
pub const LDLEN_MATH1: c_int = 1;
pub const LDLEN_MATH2: c_int = 2;
pub const LDLEN_MATH3: c_int = 3;
//
// FIFO_LOAD/FIFO_STORE/SEQ_FIFO_LOAD/SEQ_FIFO_STORE
// Command Constructs
//
// Load Destination: 0 = skip (SEQ_FIFO_LOAD only),
// 1 = Load for Class1, 2 = Load for Class2, 3 = Load both
// Store Source: 0 = normal, 1 = Class1key, 2 = Class2key
//
pub const FIFOLD_CLASS_SHIFT: c_int = 25;

pub const FIFOST_CLASS_SHIFT: c_int = 25;

//
// Scatter-Gather Table/Variable Length Field
// If set for FIFO_LOAD, refers to a SG table. Within
// SEQ_FIFO_LOAD, is variable input sequence
//
pub const FIFOLDST_SGF_SHIFT: c_int = 24;

// Immediate - Data follows command in descriptor
pub const FIFOLD_IMM_SHIFT: c_int = 23;

// Continue - Not the last FIFO store to come
pub const FIFOST_CONT_SHIFT: c_int = 23;

//
// Extended Length - use 32-bit extended length that
// follows the pointer field. Illegal with IMM set
//
pub const FIFOLDST_EXT_SHIFT: c_int = 22;

// Input data type.
pub const FIFOLD_TYPE_SHIFT: c_int = 16;

// PK types

// Other types. Need to OR in last/flush bits as desired

// Last/Flush bits for use with "other" types above

pub const FIFOLDST_LEN_MASK: c_uint = 0xffff;
pub const FIFOLDST_EXT_LEN_MASK: c_uint = 0xffffffff;
// Output data types
pub const FIFOST_TYPE_SHIFT: c_int = 16;

//
// OPERATION Command Constructs
//
// Operation type selectors - OP TYPE
pub const OP_TYPE_SHIFT: c_int = 24;

// ProtocolID selectors - PROTID
pub const OP_PCLID_SHIFT: c_int = 16;

// Assuming OP_TYPE = OP_TYPE_UNI_PROTOCOL

// Assuming OP_TYPE = OP_TYPE_DECAP_PROTOCOL/ENCAP_PROTOCOL

//
// ProtocolInfo selectors
//
pub const OP_PCLINFO_MASK: c_uint = 0xffff;
// for OP_PCLID_IPSEC
pub const OP_PCL_IPSEC_CIPHER_MASK: c_uint = 0xff00;
pub const OP_PCL_IPSEC_AUTH_MASK: c_uint = 0x00ff;
pub const OP_PCL_IPSEC_DES_IV64: c_uint = 0x0100;
pub const OP_PCL_IPSEC_DES: c_uint = 0x0200;
pub const OP_PCL_IPSEC_3DES: c_uint = 0x0300;
pub const OP_PCL_IPSEC_AES_CBC: c_uint = 0x0c00;
pub const OP_PCL_IPSEC_AES_CTR: c_uint = 0x0d00;
pub const OP_PCL_IPSEC_AES_XTS: c_uint = 0x1600;
pub const OP_PCL_IPSEC_AES_CCM8: c_uint = 0x0e00;
pub const OP_PCL_IPSEC_AES_CCM12: c_uint = 0x0f00;
pub const OP_PCL_IPSEC_AES_CCM16: c_uint = 0x1000;
pub const OP_PCL_IPSEC_AES_GCM8: c_uint = 0x1200;
pub const OP_PCL_IPSEC_AES_GCM12: c_uint = 0x1300;
pub const OP_PCL_IPSEC_AES_GCM16: c_uint = 0x1400;
pub const OP_PCL_IPSEC_HMAC_NULL: c_uint = 0x0000;
pub const OP_PCL_IPSEC_HMAC_MD5_96: c_uint = 0x0001;
pub const OP_PCL_IPSEC_HMAC_SHA1_96: c_uint = 0x0002;
pub const OP_PCL_IPSEC_AES_XCBC_MAC_96: c_uint = 0x0005;
pub const OP_PCL_IPSEC_HMAC_MD5_128: c_uint = 0x0006;
pub const OP_PCL_IPSEC_HMAC_SHA1_160: c_uint = 0x0007;
pub const OP_PCL_IPSEC_HMAC_SHA2_256_128: c_uint = 0x000c;
pub const OP_PCL_IPSEC_HMAC_SHA2_384_192: c_uint = 0x000d;
pub const OP_PCL_IPSEC_HMAC_SHA2_512_256: c_uint = 0x000e;
// For SRTP - OP_PCLID_SRTP
pub const OP_PCL_SRTP_CIPHER_MASK: c_uint = 0xff00;
pub const OP_PCL_SRTP_AUTH_MASK: c_uint = 0x00ff;
pub const OP_PCL_SRTP_AES_CTR: c_uint = 0x0d00;
pub const OP_PCL_SRTP_HMAC_SHA1_160: c_uint = 0x0007;
// For SSL 3.0 - OP_PCLID_SSL30
pub const OP_PCL_SSL30_AES_128_CBC_SHA: c_uint = 0x002f;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_2: c_uint = 0x0030;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_3: c_uint = 0x0031;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_4: c_uint = 0x0032;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_5: c_uint = 0x0033;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_6: c_uint = 0x0034;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_7: c_uint = 0x008c;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_8: c_uint = 0x0090;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_9: c_uint = 0x0094;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_10: c_uint = 0xc004;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_11: c_uint = 0xc009;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_12: c_uint = 0xc00e;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_13: c_uint = 0xc013;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_14: c_uint = 0xc018;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_15: c_uint = 0xc01d;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_16: c_uint = 0xc01e;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_17: c_uint = 0xc01f;
pub const OP_PCL_SSL30_AES_256_CBC_SHA: c_uint = 0x0035;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_2: c_uint = 0x0036;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_3: c_uint = 0x0037;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_4: c_uint = 0x0038;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_5: c_uint = 0x0039;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_6: c_uint = 0x003a;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_7: c_uint = 0x008d;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_8: c_uint = 0x0091;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_9: c_uint = 0x0095;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_10: c_uint = 0xc005;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_11: c_uint = 0xc00a;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_12: c_uint = 0xc00f;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_13: c_uint = 0xc014;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_14: c_uint = 0xc019;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_15: c_uint = 0xc020;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_16: c_uint = 0xc021;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_17: c_uint = 0xc022;
pub const OP_PCL_SSL30_3DES_EDE_CBC_MD5: c_uint = 0x0023;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA: c_uint = 0x001f;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_2: c_uint = 0x008b;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_3: c_uint = 0x008f;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_4: c_uint = 0x0093;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_5: c_uint = 0x000a;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_6: c_uint = 0x000d;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_7: c_uint = 0x0010;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_8: c_uint = 0x0013;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_9: c_uint = 0x0016;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_10: c_uint = 0x001b;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_11: c_uint = 0xc003;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_12: c_uint = 0xc008;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_13: c_uint = 0xc00d;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_14: c_uint = 0xc012;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_15: c_uint = 0xc017;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_16: c_uint = 0xc01a;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_17: c_uint = 0xc01b;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_18: c_uint = 0xc01c;
pub const OP_PCL_SSL30_DES40_CBC_MD5: c_uint = 0x0029;
pub const OP_PCL_SSL30_DES_CBC_MD5: c_uint = 0x0022;
pub const OP_PCL_SSL30_DES40_CBC_SHA: c_uint = 0x0008;
pub const OP_PCL_SSL30_DES40_CBC_SHA_2: c_uint = 0x000b;
pub const OP_PCL_SSL30_DES40_CBC_SHA_3: c_uint = 0x000e;
pub const OP_PCL_SSL30_DES40_CBC_SHA_4: c_uint = 0x0011;
pub const OP_PCL_SSL30_DES40_CBC_SHA_5: c_uint = 0x0014;
pub const OP_PCL_SSL30_DES40_CBC_SHA_6: c_uint = 0x0019;
pub const OP_PCL_SSL30_DES40_CBC_SHA_7: c_uint = 0x0026;
pub const OP_PCL_SSL30_DES_CBC_SHA: c_uint = 0x001e;
pub const OP_PCL_SSL30_DES_CBC_SHA_2: c_uint = 0x0009;
pub const OP_PCL_SSL30_DES_CBC_SHA_3: c_uint = 0x000c;
pub const OP_PCL_SSL30_DES_CBC_SHA_4: c_uint = 0x000f;
pub const OP_PCL_SSL30_DES_CBC_SHA_5: c_uint = 0x0012;
pub const OP_PCL_SSL30_DES_CBC_SHA_6: c_uint = 0x0015;
pub const OP_PCL_SSL30_DES_CBC_SHA_7: c_uint = 0x001a;
pub const OP_PCL_SSL30_RC4_128_MD5: c_uint = 0x0024;
pub const OP_PCL_SSL30_RC4_128_MD5_2: c_uint = 0x0004;
pub const OP_PCL_SSL30_RC4_128_MD5_3: c_uint = 0x0018;
pub const OP_PCL_SSL30_RC4_40_MD5: c_uint = 0x002b;
pub const OP_PCL_SSL30_RC4_40_MD5_2: c_uint = 0x0003;
pub const OP_PCL_SSL30_RC4_40_MD5_3: c_uint = 0x0017;
pub const OP_PCL_SSL30_RC4_128_SHA: c_uint = 0x0020;
pub const OP_PCL_SSL30_RC4_128_SHA_2: c_uint = 0x008a;
pub const OP_PCL_SSL30_RC4_128_SHA_3: c_uint = 0x008e;
pub const OP_PCL_SSL30_RC4_128_SHA_4: c_uint = 0x0092;
pub const OP_PCL_SSL30_RC4_128_SHA_5: c_uint = 0x0005;
pub const OP_PCL_SSL30_RC4_128_SHA_6: c_uint = 0xc002;
pub const OP_PCL_SSL30_RC4_128_SHA_7: c_uint = 0xc007;
pub const OP_PCL_SSL30_RC4_128_SHA_8: c_uint = 0xc00c;
pub const OP_PCL_SSL30_RC4_128_SHA_9: c_uint = 0xc011;
pub const OP_PCL_SSL30_RC4_128_SHA_10: c_uint = 0xc016;
pub const OP_PCL_SSL30_RC4_40_SHA: c_uint = 0x0028;
// For TLS 1.0 - OP_PCLID_TLS10
pub const OP_PCL_TLS10_AES_128_CBC_SHA: c_uint = 0x002f;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_2: c_uint = 0x0030;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_3: c_uint = 0x0031;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_4: c_uint = 0x0032;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_5: c_uint = 0x0033;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_6: c_uint = 0x0034;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_7: c_uint = 0x008c;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_8: c_uint = 0x0090;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_9: c_uint = 0x0094;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_10: c_uint = 0xc004;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_11: c_uint = 0xc009;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_12: c_uint = 0xc00e;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_13: c_uint = 0xc013;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_14: c_uint = 0xc018;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_15: c_uint = 0xc01d;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_16: c_uint = 0xc01e;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_17: c_uint = 0xc01f;
pub const OP_PCL_TLS10_AES_256_CBC_SHA: c_uint = 0x0035;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_2: c_uint = 0x0036;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_3: c_uint = 0x0037;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_4: c_uint = 0x0038;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_5: c_uint = 0x0039;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_6: c_uint = 0x003a;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_7: c_uint = 0x008d;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_8: c_uint = 0x0091;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_9: c_uint = 0x0095;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_10: c_uint = 0xc005;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_11: c_uint = 0xc00a;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_12: c_uint = 0xc00f;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_13: c_uint = 0xc014;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_14: c_uint = 0xc019;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_15: c_uint = 0xc020;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_16: c_uint = 0xc021;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_17: c_uint = 0xc022;
// #define OP_PCL_TLS10_3DES_EDE_CBC_MD5	0x0023
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA: c_uint = 0x001f;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_2: c_uint = 0x008b;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_3: c_uint = 0x008f;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_4: c_uint = 0x0093;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_5: c_uint = 0x000a;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_6: c_uint = 0x000d;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_7: c_uint = 0x0010;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_8: c_uint = 0x0013;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_9: c_uint = 0x0016;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_10: c_uint = 0x001b;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_11: c_uint = 0xc003;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_12: c_uint = 0xc008;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_13: c_uint = 0xc00d;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_14: c_uint = 0xc012;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_15: c_uint = 0xc017;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_16: c_uint = 0xc01a;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_17: c_uint = 0xc01b;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_18: c_uint = 0xc01c;
pub const OP_PCL_TLS10_DES40_CBC_MD5: c_uint = 0x0029;
pub const OP_PCL_TLS10_DES_CBC_MD5: c_uint = 0x0022;
pub const OP_PCL_TLS10_DES40_CBC_SHA: c_uint = 0x0008;
pub const OP_PCL_TLS10_DES40_CBC_SHA_2: c_uint = 0x000b;
pub const OP_PCL_TLS10_DES40_CBC_SHA_3: c_uint = 0x000e;
pub const OP_PCL_TLS10_DES40_CBC_SHA_4: c_uint = 0x0011;
pub const OP_PCL_TLS10_DES40_CBC_SHA_5: c_uint = 0x0014;
pub const OP_PCL_TLS10_DES40_CBC_SHA_6: c_uint = 0x0019;
pub const OP_PCL_TLS10_DES40_CBC_SHA_7: c_uint = 0x0026;
pub const OP_PCL_TLS10_DES_CBC_SHA: c_uint = 0x001e;
pub const OP_PCL_TLS10_DES_CBC_SHA_2: c_uint = 0x0009;
pub const OP_PCL_TLS10_DES_CBC_SHA_3: c_uint = 0x000c;
pub const OP_PCL_TLS10_DES_CBC_SHA_4: c_uint = 0x000f;
pub const OP_PCL_TLS10_DES_CBC_SHA_5: c_uint = 0x0012;
pub const OP_PCL_TLS10_DES_CBC_SHA_6: c_uint = 0x0015;
pub const OP_PCL_TLS10_DES_CBC_SHA_7: c_uint = 0x001a;
pub const OP_PCL_TLS10_RC4_128_MD5: c_uint = 0x0024;
pub const OP_PCL_TLS10_RC4_128_MD5_2: c_uint = 0x0004;
pub const OP_PCL_TLS10_RC4_128_MD5_3: c_uint = 0x0018;
pub const OP_PCL_TLS10_RC4_40_MD5: c_uint = 0x002b;
pub const OP_PCL_TLS10_RC4_40_MD5_2: c_uint = 0x0003;
pub const OP_PCL_TLS10_RC4_40_MD5_3: c_uint = 0x0017;
pub const OP_PCL_TLS10_RC4_128_SHA: c_uint = 0x0020;
pub const OP_PCL_TLS10_RC4_128_SHA_2: c_uint = 0x008a;
pub const OP_PCL_TLS10_RC4_128_SHA_3: c_uint = 0x008e;
pub const OP_PCL_TLS10_RC4_128_SHA_4: c_uint = 0x0092;
pub const OP_PCL_TLS10_RC4_128_SHA_5: c_uint = 0x0005;
pub const OP_PCL_TLS10_RC4_128_SHA_6: c_uint = 0xc002;
pub const OP_PCL_TLS10_RC4_128_SHA_7: c_uint = 0xc007;
pub const OP_PCL_TLS10_RC4_128_SHA_8: c_uint = 0xc00c;
pub const OP_PCL_TLS10_RC4_128_SHA_9: c_uint = 0xc011;
pub const OP_PCL_TLS10_RC4_128_SHA_10: c_uint = 0xc016;
pub const OP_PCL_TLS10_RC4_40_SHA: c_uint = 0x0028;
pub const OP_PCL_TLS10_3DES_EDE_CBC_MD5: c_uint = 0xff23;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA160: c_uint = 0xff30;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA224: c_uint = 0xff34;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA256: c_uint = 0xff36;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA384: c_uint = 0xff33;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA512: c_uint = 0xff35;
pub const OP_PCL_TLS10_AES_128_CBC_SHA160: c_uint = 0xff80;
pub const OP_PCL_TLS10_AES_128_CBC_SHA224: c_uint = 0xff84;
pub const OP_PCL_TLS10_AES_128_CBC_SHA256: c_uint = 0xff86;
pub const OP_PCL_TLS10_AES_128_CBC_SHA384: c_uint = 0xff83;
pub const OP_PCL_TLS10_AES_128_CBC_SHA512: c_uint = 0xff85;
pub const OP_PCL_TLS10_AES_192_CBC_SHA160: c_uint = 0xff20;
pub const OP_PCL_TLS10_AES_192_CBC_SHA224: c_uint = 0xff24;
pub const OP_PCL_TLS10_AES_192_CBC_SHA256: c_uint = 0xff26;
pub const OP_PCL_TLS10_AES_192_CBC_SHA384: c_uint = 0xff23;
pub const OP_PCL_TLS10_AES_192_CBC_SHA512: c_uint = 0xff25;
pub const OP_PCL_TLS10_AES_256_CBC_SHA160: c_uint = 0xff60;
pub const OP_PCL_TLS10_AES_256_CBC_SHA224: c_uint = 0xff64;
pub const OP_PCL_TLS10_AES_256_CBC_SHA256: c_uint = 0xff66;
pub const OP_PCL_TLS10_AES_256_CBC_SHA384: c_uint = 0xff63;
pub const OP_PCL_TLS10_AES_256_CBC_SHA512: c_uint = 0xff65;
// For TLS 1.1 - OP_PCLID_TLS11
pub const OP_PCL_TLS11_AES_128_CBC_SHA: c_uint = 0x002f;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_2: c_uint = 0x0030;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_3: c_uint = 0x0031;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_4: c_uint = 0x0032;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_5: c_uint = 0x0033;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_6: c_uint = 0x0034;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_7: c_uint = 0x008c;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_8: c_uint = 0x0090;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_9: c_uint = 0x0094;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_10: c_uint = 0xc004;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_11: c_uint = 0xc009;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_12: c_uint = 0xc00e;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_13: c_uint = 0xc013;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_14: c_uint = 0xc018;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_15: c_uint = 0xc01d;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_16: c_uint = 0xc01e;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_17: c_uint = 0xc01f;
pub const OP_PCL_TLS11_AES_256_CBC_SHA: c_uint = 0x0035;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_2: c_uint = 0x0036;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_3: c_uint = 0x0037;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_4: c_uint = 0x0038;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_5: c_uint = 0x0039;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_6: c_uint = 0x003a;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_7: c_uint = 0x008d;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_8: c_uint = 0x0091;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_9: c_uint = 0x0095;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_10: c_uint = 0xc005;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_11: c_uint = 0xc00a;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_12: c_uint = 0xc00f;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_13: c_uint = 0xc014;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_14: c_uint = 0xc019;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_15: c_uint = 0xc020;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_16: c_uint = 0xc021;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_17: c_uint = 0xc022;
// #define OP_PCL_TLS11_3DES_EDE_CBC_MD5	0x0023
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA: c_uint = 0x001f;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_2: c_uint = 0x008b;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_3: c_uint = 0x008f;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_4: c_uint = 0x0093;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_5: c_uint = 0x000a;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_6: c_uint = 0x000d;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_7: c_uint = 0x0010;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_8: c_uint = 0x0013;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_9: c_uint = 0x0016;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_10: c_uint = 0x001b;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_11: c_uint = 0xc003;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_12: c_uint = 0xc008;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_13: c_uint = 0xc00d;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_14: c_uint = 0xc012;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_15: c_uint = 0xc017;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_16: c_uint = 0xc01a;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_17: c_uint = 0xc01b;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_18: c_uint = 0xc01c;
pub const OP_PCL_TLS11_DES40_CBC_MD5: c_uint = 0x0029;
pub const OP_PCL_TLS11_DES_CBC_MD5: c_uint = 0x0022;
pub const OP_PCL_TLS11_DES40_CBC_SHA: c_uint = 0x0008;
pub const OP_PCL_TLS11_DES40_CBC_SHA_2: c_uint = 0x000b;
pub const OP_PCL_TLS11_DES40_CBC_SHA_3: c_uint = 0x000e;
pub const OP_PCL_TLS11_DES40_CBC_SHA_4: c_uint = 0x0011;
pub const OP_PCL_TLS11_DES40_CBC_SHA_5: c_uint = 0x0014;
pub const OP_PCL_TLS11_DES40_CBC_SHA_6: c_uint = 0x0019;
pub const OP_PCL_TLS11_DES40_CBC_SHA_7: c_uint = 0x0026;
pub const OP_PCL_TLS11_DES_CBC_SHA: c_uint = 0x001e;
pub const OP_PCL_TLS11_DES_CBC_SHA_2: c_uint = 0x0009;
pub const OP_PCL_TLS11_DES_CBC_SHA_3: c_uint = 0x000c;
pub const OP_PCL_TLS11_DES_CBC_SHA_4: c_uint = 0x000f;
pub const OP_PCL_TLS11_DES_CBC_SHA_5: c_uint = 0x0012;
pub const OP_PCL_TLS11_DES_CBC_SHA_6: c_uint = 0x0015;
pub const OP_PCL_TLS11_DES_CBC_SHA_7: c_uint = 0x001a;
pub const OP_PCL_TLS11_RC4_128_MD5: c_uint = 0x0024;
pub const OP_PCL_TLS11_RC4_128_MD5_2: c_uint = 0x0004;
pub const OP_PCL_TLS11_RC4_128_MD5_3: c_uint = 0x0018;
pub const OP_PCL_TLS11_RC4_40_MD5: c_uint = 0x002b;
pub const OP_PCL_TLS11_RC4_40_MD5_2: c_uint = 0x0003;
pub const OP_PCL_TLS11_RC4_40_MD5_3: c_uint = 0x0017;
pub const OP_PCL_TLS11_RC4_128_SHA: c_uint = 0x0020;
pub const OP_PCL_TLS11_RC4_128_SHA_2: c_uint = 0x008a;
pub const OP_PCL_TLS11_RC4_128_SHA_3: c_uint = 0x008e;
pub const OP_PCL_TLS11_RC4_128_SHA_4: c_uint = 0x0092;
pub const OP_PCL_TLS11_RC4_128_SHA_5: c_uint = 0x0005;
pub const OP_PCL_TLS11_RC4_128_SHA_6: c_uint = 0xc002;
pub const OP_PCL_TLS11_RC4_128_SHA_7: c_uint = 0xc007;
pub const OP_PCL_TLS11_RC4_128_SHA_8: c_uint = 0xc00c;
pub const OP_PCL_TLS11_RC4_128_SHA_9: c_uint = 0xc011;
pub const OP_PCL_TLS11_RC4_128_SHA_10: c_uint = 0xc016;
pub const OP_PCL_TLS11_RC4_40_SHA: c_uint = 0x0028;
pub const OP_PCL_TLS11_3DES_EDE_CBC_MD5: c_uint = 0xff23;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA160: c_uint = 0xff30;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA224: c_uint = 0xff34;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA256: c_uint = 0xff36;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA384: c_uint = 0xff33;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA512: c_uint = 0xff35;
pub const OP_PCL_TLS11_AES_128_CBC_SHA160: c_uint = 0xff80;
pub const OP_PCL_TLS11_AES_128_CBC_SHA224: c_uint = 0xff84;
pub const OP_PCL_TLS11_AES_128_CBC_SHA256: c_uint = 0xff86;
pub const OP_PCL_TLS11_AES_128_CBC_SHA384: c_uint = 0xff83;
pub const OP_PCL_TLS11_AES_128_CBC_SHA512: c_uint = 0xff85;
pub const OP_PCL_TLS11_AES_192_CBC_SHA160: c_uint = 0xff20;
pub const OP_PCL_TLS11_AES_192_CBC_SHA224: c_uint = 0xff24;
pub const OP_PCL_TLS11_AES_192_CBC_SHA256: c_uint = 0xff26;
pub const OP_PCL_TLS11_AES_192_CBC_SHA384: c_uint = 0xff23;
pub const OP_PCL_TLS11_AES_192_CBC_SHA512: c_uint = 0xff25;
pub const OP_PCL_TLS11_AES_256_CBC_SHA160: c_uint = 0xff60;
pub const OP_PCL_TLS11_AES_256_CBC_SHA224: c_uint = 0xff64;
pub const OP_PCL_TLS11_AES_256_CBC_SHA256: c_uint = 0xff66;
pub const OP_PCL_TLS11_AES_256_CBC_SHA384: c_uint = 0xff63;
pub const OP_PCL_TLS11_AES_256_CBC_SHA512: c_uint = 0xff65;
// For TLS 1.2 - OP_PCLID_TLS12
pub const OP_PCL_TLS12_AES_128_CBC_SHA: c_uint = 0x002f;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_2: c_uint = 0x0030;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_3: c_uint = 0x0031;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_4: c_uint = 0x0032;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_5: c_uint = 0x0033;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_6: c_uint = 0x0034;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_7: c_uint = 0x008c;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_8: c_uint = 0x0090;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_9: c_uint = 0x0094;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_10: c_uint = 0xc004;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_11: c_uint = 0xc009;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_12: c_uint = 0xc00e;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_13: c_uint = 0xc013;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_14: c_uint = 0xc018;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_15: c_uint = 0xc01d;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_16: c_uint = 0xc01e;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_17: c_uint = 0xc01f;
pub const OP_PCL_TLS12_AES_256_CBC_SHA: c_uint = 0x0035;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_2: c_uint = 0x0036;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_3: c_uint = 0x0037;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_4: c_uint = 0x0038;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_5: c_uint = 0x0039;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_6: c_uint = 0x003a;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_7: c_uint = 0x008d;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_8: c_uint = 0x0091;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_9: c_uint = 0x0095;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_10: c_uint = 0xc005;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_11: c_uint = 0xc00a;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_12: c_uint = 0xc00f;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_13: c_uint = 0xc014;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_14: c_uint = 0xc019;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_15: c_uint = 0xc020;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_16: c_uint = 0xc021;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_17: c_uint = 0xc022;
// #define OP_PCL_TLS12_3DES_EDE_CBC_MD5	0x0023
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA: c_uint = 0x001f;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_2: c_uint = 0x008b;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_3: c_uint = 0x008f;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_4: c_uint = 0x0093;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_5: c_uint = 0x000a;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_6: c_uint = 0x000d;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_7: c_uint = 0x0010;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_8: c_uint = 0x0013;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_9: c_uint = 0x0016;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_10: c_uint = 0x001b;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_11: c_uint = 0xc003;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_12: c_uint = 0xc008;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_13: c_uint = 0xc00d;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_14: c_uint = 0xc012;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_15: c_uint = 0xc017;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_16: c_uint = 0xc01a;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_17: c_uint = 0xc01b;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_18: c_uint = 0xc01c;
pub const OP_PCL_TLS12_DES40_CBC_MD5: c_uint = 0x0029;
pub const OP_PCL_TLS12_DES_CBC_MD5: c_uint = 0x0022;
pub const OP_PCL_TLS12_DES40_CBC_SHA: c_uint = 0x0008;
pub const OP_PCL_TLS12_DES40_CBC_SHA_2: c_uint = 0x000b;
pub const OP_PCL_TLS12_DES40_CBC_SHA_3: c_uint = 0x000e;
pub const OP_PCL_TLS12_DES40_CBC_SHA_4: c_uint = 0x0011;
pub const OP_PCL_TLS12_DES40_CBC_SHA_5: c_uint = 0x0014;
pub const OP_PCL_TLS12_DES40_CBC_SHA_6: c_uint = 0x0019;
pub const OP_PCL_TLS12_DES40_CBC_SHA_7: c_uint = 0x0026;
pub const OP_PCL_TLS12_DES_CBC_SHA: c_uint = 0x001e;
pub const OP_PCL_TLS12_DES_CBC_SHA_2: c_uint = 0x0009;
pub const OP_PCL_TLS12_DES_CBC_SHA_3: c_uint = 0x000c;
pub const OP_PCL_TLS12_DES_CBC_SHA_4: c_uint = 0x000f;
pub const OP_PCL_TLS12_DES_CBC_SHA_5: c_uint = 0x0012;
pub const OP_PCL_TLS12_DES_CBC_SHA_6: c_uint = 0x0015;
pub const OP_PCL_TLS12_DES_CBC_SHA_7: c_uint = 0x001a;
pub const OP_PCL_TLS12_RC4_128_MD5: c_uint = 0x0024;
pub const OP_PCL_TLS12_RC4_128_MD5_2: c_uint = 0x0004;
pub const OP_PCL_TLS12_RC4_128_MD5_3: c_uint = 0x0018;
pub const OP_PCL_TLS12_RC4_40_MD5: c_uint = 0x002b;
pub const OP_PCL_TLS12_RC4_40_MD5_2: c_uint = 0x0003;
pub const OP_PCL_TLS12_RC4_40_MD5_3: c_uint = 0x0017;
pub const OP_PCL_TLS12_RC4_128_SHA: c_uint = 0x0020;
pub const OP_PCL_TLS12_RC4_128_SHA_2: c_uint = 0x008a;
pub const OP_PCL_TLS12_RC4_128_SHA_3: c_uint = 0x008e;
pub const OP_PCL_TLS12_RC4_128_SHA_4: c_uint = 0x0092;
pub const OP_PCL_TLS12_RC4_128_SHA_5: c_uint = 0x0005;
pub const OP_PCL_TLS12_RC4_128_SHA_6: c_uint = 0xc002;
pub const OP_PCL_TLS12_RC4_128_SHA_7: c_uint = 0xc007;
pub const OP_PCL_TLS12_RC4_128_SHA_8: c_uint = 0xc00c;
pub const OP_PCL_TLS12_RC4_128_SHA_9: c_uint = 0xc011;
pub const OP_PCL_TLS12_RC4_128_SHA_10: c_uint = 0xc016;
pub const OP_PCL_TLS12_RC4_40_SHA: c_uint = 0x0028;
// #define OP_PCL_TLS12_AES_128_CBC_SHA256	0x003c
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_2: c_uint = 0x003e;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_3: c_uint = 0x003f;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_4: c_uint = 0x0040;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_5: c_uint = 0x0067;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_6: c_uint = 0x006c;
// #define OP_PCL_TLS12_AES_256_CBC_SHA256	0x003d
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_2: c_uint = 0x0068;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_3: c_uint = 0x0069;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_4: c_uint = 0x006a;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_5: c_uint = 0x006b;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_6: c_uint = 0x006d;
// AEAD_AES_xxx_CCM/GCM remain to be defined...
pub const OP_PCL_TLS12_3DES_EDE_CBC_MD5: c_uint = 0xff23;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA160: c_uint = 0xff30;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA224: c_uint = 0xff34;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA256: c_uint = 0xff36;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA384: c_uint = 0xff33;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA512: c_uint = 0xff35;
pub const OP_PCL_TLS12_AES_128_CBC_SHA160: c_uint = 0xff80;
pub const OP_PCL_TLS12_AES_128_CBC_SHA224: c_uint = 0xff84;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256: c_uint = 0xff86;
pub const OP_PCL_TLS12_AES_128_CBC_SHA384: c_uint = 0xff83;
pub const OP_PCL_TLS12_AES_128_CBC_SHA512: c_uint = 0xff85;
pub const OP_PCL_TLS12_AES_192_CBC_SHA160: c_uint = 0xff20;
pub const OP_PCL_TLS12_AES_192_CBC_SHA224: c_uint = 0xff24;
pub const OP_PCL_TLS12_AES_192_CBC_SHA256: c_uint = 0xff26;
pub const OP_PCL_TLS12_AES_192_CBC_SHA384: c_uint = 0xff23;
pub const OP_PCL_TLS12_AES_192_CBC_SHA512: c_uint = 0xff25;
pub const OP_PCL_TLS12_AES_256_CBC_SHA160: c_uint = 0xff60;
pub const OP_PCL_TLS12_AES_256_CBC_SHA224: c_uint = 0xff64;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256: c_uint = 0xff66;
pub const OP_PCL_TLS12_AES_256_CBC_SHA384: c_uint = 0xff63;
pub const OP_PCL_TLS12_AES_256_CBC_SHA512: c_uint = 0xff65;
// Blob protocol protinfo bits
pub const OP_PCL_BLOB_BLACK: c_uint = 0x0004;
pub const OP_PCL_BLOB_EKT: c_uint = 0x0100;
// For DTLS - OP_PCLID_DTLS
pub const OP_PCL_DTLS_AES_128_CBC_SHA: c_uint = 0x002f;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_2: c_uint = 0x0030;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_3: c_uint = 0x0031;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_4: c_uint = 0x0032;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_5: c_uint = 0x0033;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_6: c_uint = 0x0034;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_7: c_uint = 0x008c;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_8: c_uint = 0x0090;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_9: c_uint = 0x0094;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_10: c_uint = 0xc004;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_11: c_uint = 0xc009;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_12: c_uint = 0xc00e;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_13: c_uint = 0xc013;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_14: c_uint = 0xc018;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_15: c_uint = 0xc01d;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_16: c_uint = 0xc01e;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_17: c_uint = 0xc01f;
pub const OP_PCL_DTLS_AES_256_CBC_SHA: c_uint = 0x0035;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_2: c_uint = 0x0036;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_3: c_uint = 0x0037;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_4: c_uint = 0x0038;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_5: c_uint = 0x0039;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_6: c_uint = 0x003a;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_7: c_uint = 0x008d;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_8: c_uint = 0x0091;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_9: c_uint = 0x0095;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_10: c_uint = 0xc005;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_11: c_uint = 0xc00a;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_12: c_uint = 0xc00f;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_13: c_uint = 0xc014;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_14: c_uint = 0xc019;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_15: c_uint = 0xc020;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_16: c_uint = 0xc021;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_17: c_uint = 0xc022;
// #define OP_PCL_DTLS_3DES_EDE_CBC_MD5		0x0023
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA: c_uint = 0x001f;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_2: c_uint = 0x008b;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_3: c_uint = 0x008f;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_4: c_uint = 0x0093;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_5: c_uint = 0x000a;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_6: c_uint = 0x000d;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_7: c_uint = 0x0010;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_8: c_uint = 0x0013;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_9: c_uint = 0x0016;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_10: c_uint = 0x001b;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_11: c_uint = 0xc003;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_12: c_uint = 0xc008;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_13: c_uint = 0xc00d;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_14: c_uint = 0xc012;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_15: c_uint = 0xc017;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_16: c_uint = 0xc01a;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_17: c_uint = 0xc01b;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_18: c_uint = 0xc01c;
pub const OP_PCL_DTLS_DES40_CBC_MD5: c_uint = 0x0029;
pub const OP_PCL_DTLS_DES_CBC_MD5: c_uint = 0x0022;
pub const OP_PCL_DTLS_DES40_CBC_SHA: c_uint = 0x0008;
pub const OP_PCL_DTLS_DES40_CBC_SHA_2: c_uint = 0x000b;
pub const OP_PCL_DTLS_DES40_CBC_SHA_3: c_uint = 0x000e;
pub const OP_PCL_DTLS_DES40_CBC_SHA_4: c_uint = 0x0011;
pub const OP_PCL_DTLS_DES40_CBC_SHA_5: c_uint = 0x0014;
pub const OP_PCL_DTLS_DES40_CBC_SHA_6: c_uint = 0x0019;
pub const OP_PCL_DTLS_DES40_CBC_SHA_7: c_uint = 0x0026;
pub const OP_PCL_DTLS_DES_CBC_SHA: c_uint = 0x001e;
pub const OP_PCL_DTLS_DES_CBC_SHA_2: c_uint = 0x0009;
pub const OP_PCL_DTLS_DES_CBC_SHA_3: c_uint = 0x000c;
pub const OP_PCL_DTLS_DES_CBC_SHA_4: c_uint = 0x000f;
pub const OP_PCL_DTLS_DES_CBC_SHA_5: c_uint = 0x0012;
pub const OP_PCL_DTLS_DES_CBC_SHA_6: c_uint = 0x0015;
pub const OP_PCL_DTLS_DES_CBC_SHA_7: c_uint = 0x001a;
pub const OP_PCL_DTLS_3DES_EDE_CBC_MD5: c_uint = 0xff23;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA160: c_uint = 0xff30;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA224: c_uint = 0xff34;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA256: c_uint = 0xff36;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA384: c_uint = 0xff33;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA512: c_uint = 0xff35;
pub const OP_PCL_DTLS_AES_128_CBC_SHA160: c_uint = 0xff80;
pub const OP_PCL_DTLS_AES_128_CBC_SHA224: c_uint = 0xff84;
pub const OP_PCL_DTLS_AES_128_CBC_SHA256: c_uint = 0xff86;
pub const OP_PCL_DTLS_AES_128_CBC_SHA384: c_uint = 0xff83;
pub const OP_PCL_DTLS_AES_128_CBC_SHA512: c_uint = 0xff85;
pub const OP_PCL_DTLS_AES_192_CBC_SHA160: c_uint = 0xff20;
pub const OP_PCL_DTLS_AES_192_CBC_SHA224: c_uint = 0xff24;
pub const OP_PCL_DTLS_AES_192_CBC_SHA256: c_uint = 0xff26;
pub const OP_PCL_DTLS_AES_192_CBC_SHA384: c_uint = 0xff23;
pub const OP_PCL_DTLS_AES_192_CBC_SHA512: c_uint = 0xff25;
pub const OP_PCL_DTLS_AES_256_CBC_SHA160: c_uint = 0xff60;
pub const OP_PCL_DTLS_AES_256_CBC_SHA224: c_uint = 0xff64;
pub const OP_PCL_DTLS_AES_256_CBC_SHA256: c_uint = 0xff66;
pub const OP_PCL_DTLS_AES_256_CBC_SHA384: c_uint = 0xff63;
pub const OP_PCL_DTLS_AES_256_CBC_SHA512: c_uint = 0xff65;
// 802.16 WiMAX protinfos
pub const OP_PCL_WIMAX_OFDM: c_uint = 0x0201;
pub const OP_PCL_WIMAX_OFDMA: c_uint = 0x0231;
// 802.11 WiFi protinfos
pub const OP_PCL_WIFI: c_uint = 0xac04;
// MacSec protinfos
pub const OP_PCL_MACSEC: c_uint = 0x0001;
// Derived Key Protocol (DKP) Protinfo
pub const OP_PCL_DKP_SRC_SHIFT: c_int = 14;

pub const OP_PCL_DKP_DST_SHIFT: c_int = 12;

pub const OP_PCL_DKP_KEY_SHIFT: c_int = 0;

// PKI unidirectional protocol protinfo bits
pub const OP_PCL_PKPROT_TEST: c_uint = 0x0008;
pub const OP_PCL_PKPROT_DECRYPT: c_uint = 0x0004;
pub const OP_PCL_PKPROT_ECC: c_uint = 0x0002;
pub const OP_PCL_PKPROT_F2M: c_uint = 0x0001;
// For non-protocol/alg-only op commands
pub const OP_ALG_TYPE_SHIFT: c_int = 24;

// version register fields
pub const OP_VER_CCHA_NUM: c_uint = 0x000000ff /* Number CCHAs instantiated */;
pub const OP_VER_CCHA_MISC: c_uint = 0x0000ff00 /* CCHA Miscellaneous Information */;
pub const OP_VER_CCHA_REV: c_uint = 0x00ff0000 /* CCHA Revision Number */;
pub const OP_VER_CCHA_VID: c_uint = 0xff000000 /* CCHA Version ID */;
pub const OP_ALG_ALGSEL_SHIFT: c_int = 16;

pub const OP_ALG_AAI_SHIFT: c_int = 4;

// blockcipher AAI set

// randomizer AAI set

// RNG4 AAI set

// Chacha20 AAI set

// hmac/smac AAI set

// CRC AAI set

// Kasumi/SNOW AAI set

pub const OP_ALG_AS_SHIFT: c_int = 2;

pub const OP_ALG_ICV_SHIFT: c_int = 1;

pub const OP_ALG_DIR_SHIFT: c_int = 0;
pub const OP_ALG_DIR_MASK: c_int = 1;
pub const OP_ALG_DECRYPT: c_int = 0;
pub const OP_ALG_ENCRYPT: c_int = 1;
// PKHA algorithm type set
pub const OP_ALG_PK: c_uint = 0x00800000;
pub const OP_ALG_PK_FUN_MASK: c_uint = 0x3f /* clrmem, modmath, or cpymem */;
// PKHA mode clear memory functions
pub const OP_ALG_PKMODE_A_RAM: c_uint = 0x80000;
pub const OP_ALG_PKMODE_B_RAM: c_uint = 0x40000;
pub const OP_ALG_PKMODE_E_RAM: c_uint = 0x20000;
pub const OP_ALG_PKMODE_N_RAM: c_uint = 0x10000;
pub const OP_ALG_PKMODE_CLEARMEM: c_uint = 0x00001;
// PKHA mode modular-arithmetic functions
pub const OP_ALG_PKMODE_MOD_IN_MONTY: c_uint = 0x80000;
pub const OP_ALG_PKMODE_MOD_OUT_MONTY: c_uint = 0x40000;
pub const OP_ALG_PKMODE_MOD_F2M: c_uint = 0x20000;
pub const OP_ALG_PKMODE_MOD_R2_IN: c_uint = 0x10000;
pub const OP_ALG_PKMODE_PRJECTV: c_uint = 0x00800;
pub const OP_ALG_PKMODE_TIME_EQ: c_uint = 0x400;
pub const OP_ALG_PKMODE_OUT_B: c_uint = 0x000;
pub const OP_ALG_PKMODE_OUT_A: c_uint = 0x100;
pub const OP_ALG_PKMODE_MOD_ADD: c_uint = 0x002;
pub const OP_ALG_PKMODE_MOD_SUB_AB: c_uint = 0x003;
pub const OP_ALG_PKMODE_MOD_SUB_BA: c_uint = 0x004;
pub const OP_ALG_PKMODE_MOD_MULT: c_uint = 0x005;
pub const OP_ALG_PKMODE_MOD_EXPO: c_uint = 0x006;
pub const OP_ALG_PKMODE_MOD_REDUCT: c_uint = 0x007;
pub const OP_ALG_PKMODE_MOD_INV: c_uint = 0x008;
pub const OP_ALG_PKMODE_MOD_ECC_ADD: c_uint = 0x009;
pub const OP_ALG_PKMODE_MOD_ECC_DBL: c_uint = 0x00a;
pub const OP_ALG_PKMODE_MOD_ECC_MULT: c_uint = 0x00b;
pub const OP_ALG_PKMODE_MOD_MONT_CNST: c_uint = 0x00c;
pub const OP_ALG_PKMODE_MOD_CRT_CNST: c_uint = 0x00d;
pub const OP_ALG_PKMODE_MOD_GCD: c_uint = 0x00e;
pub const OP_ALG_PKMODE_MOD_PRIMALITY: c_uint = 0x00f;
// PKHA mode copy-memory functions
pub const OP_ALG_PKMODE_SRC_REG_SHIFT: c_int = 17;

pub const OP_ALG_PKMODE_DST_REG_SHIFT: c_int = 10;

pub const OP_ALG_PKMODE_SRC_SEG_SHIFT: c_int = 8;

pub const OP_ALG_PKMODE_DST_SEG_SHIFT: c_int = 6;

pub const OP_ALG_PKMODE_CPYMEM_N_SZ: c_uint = 0x80;
pub const OP_ALG_PKMODE_CPYMEM_SRC_SZ: c_uint = 0x81;
//
// SEQ_IN_PTR Command Constructs
//
// Release Buffers
pub const SQIN_RBS: c_uint = 0x04000000;
// Sequence pointer is really a descriptor
pub const SQIN_INL: c_uint = 0x02000000;
// Sequence pointer is a scatter-gather table
pub const SQIN_SGF: c_uint = 0x01000000;
// Appends to a previous pointer
pub const SQIN_PRE: c_uint = 0x00800000;
// Use extended length following pointer
pub const SQIN_EXT: c_uint = 0x00400000;
// Restore sequence with pointer/length
pub const SQIN_RTO: c_uint = 0x00200000;
// Replace job descriptor
pub const SQIN_RJD: c_uint = 0x00100000;
pub const SQIN_LEN_SHIFT: c_int = 0;

//
// SEQ_OUT_PTR Command Constructs
//
// Sequence pointer is a scatter-gather table
pub const SQOUT_SGF: c_uint = 0x01000000;
// Appends to a previous pointer

// Restore sequence with pointer/length

// Use extended length following pointer
pub const SQOUT_EXT: c_uint = 0x00400000;
pub const SQOUT_LEN_SHIFT: c_int = 0;

//
// SIGNATURE Command Constructs
//
// TYPE field is all that's relevant
pub const SIGN_TYPE_SHIFT: c_int = 16;

//
// MOVE Command Constructs
//
pub const MOVE_AUX_SHIFT: c_int = 25;

pub const MOVE_WAITCOMP_SHIFT: c_int = 24;

pub const MOVE_SRC_SHIFT: c_int = 20;

pub const MOVE_DEST_SHIFT: c_int = 16;

pub const MOVE_OFFSET_SHIFT: c_int = 8;

pub const MOVE_LEN_SHIFT: c_int = 0;

pub const MOVELEN_MRSEL_SHIFT: c_int = 0;

//
// MATH Command Constructs
//
pub const MATH_IFB_SHIFT: c_int = 26;

pub const MATH_NFU_SHIFT: c_int = 25;

pub const MATH_STL_SHIFT: c_int = 24;

// Function selectors
pub const MATH_FUN_SHIFT: c_int = 20;

// Source 0 selectors
pub const MATH_SRC0_SHIFT: c_int = 16;

// Source 1 selectors
pub const MATH_SRC1_SHIFT: c_int = 12;

// Destination selectors
pub const MATH_DEST_SHIFT: c_int = 8;

// Length selectors
pub const MATH_LEN_SHIFT: c_int = 0;

pub const MATH_LEN_1BYTE: c_uint = 0x01;
pub const MATH_LEN_2BYTE: c_uint = 0x02;
pub const MATH_LEN_4BYTE: c_uint = 0x04;
pub const MATH_LEN_8BYTE: c_uint = 0x08;
//
// JUMP Command Constructs
//
pub const JUMP_CLASS_SHIFT: c_int = 25;

pub const JUMP_CLASS_NONE: c_int = 0;

pub const JUMP_JSL_SHIFT: c_int = 24;

pub const JUMP_TYPE_SHIFT: c_int = 22;

pub const JUMP_TEST_SHIFT: c_int = 16;

// Condition codes. JSL bit is factored in
pub const JUMP_COND_SHIFT: c_int = 8;

pub const JUMP_OFFSET_SHIFT: c_int = 0;

//
// NFIFO ENTRY
// Data Constructs
//
pub const NFIFOENTRY_DEST_SHIFT: c_int = 30;

pub const NFIFOENTRY_LC2_SHIFT: c_int = 29;

pub const NFIFOENTRY_LC1_SHIFT: c_int = 28;

pub const NFIFOENTRY_FC2_SHIFT: c_int = 27;

pub const NFIFOENTRY_FC1_SHIFT: c_int = 26;

pub const NFIFOENTRY_STYPE_SHIFT: c_int = 24;

pub const NFIFOENTRY_DTYPE_SHIFT: c_int = 20;

pub const NFIFOENTRY_BND_SHIFT: c_int = 19;

pub const NFIFOENTRY_PTYPE_SHIFT: c_int = 16;

pub const NFIFOENTRY_OC_SHIFT: c_int = 15;

pub const NFIFOENTRY_AST_SHIFT: c_int = 14;

pub const NFIFOENTRY_BM_SHIFT: c_int = 11;

pub const NFIFOENTRY_PS_SHIFT: c_int = 10;

pub const NFIFOENTRY_DLEN_SHIFT: c_int = 0;

pub const NFIFOENTRY_PLEN_SHIFT: c_int = 0;

// Append Load Immediate Command
pub const FD_CMD_APPEND_LOAD_IMMEDIATE: c_uint = 0x80000000;
// Set SEQ LIODN equal to the Non-SEQ LIODN for the job
pub const FD_CMD_SET_SEQ_LIODN_EQUAL_NONSEQ_LIODN: c_uint = 0x40000000;
// Frame Descriptor Command for Replacement Job Descriptor
pub const FD_CMD_REPLACE_JOB_DESC: c_uint = 0x20000000;
