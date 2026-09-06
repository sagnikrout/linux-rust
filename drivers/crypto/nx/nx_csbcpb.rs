//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/nx/nx_csbcpb.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_aes_ecb {
    pub key: [u8; 32],
    pub __rsvd: [u8; 80],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_aes_cbc {
    pub iv: [u8; 16],
    pub key: [u8; 32],
    pub cv: [u8; 16],
    pub spbc: u32,
    pub __rsvd: [u8; 44],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_aes_gca {
    pub in_pat: [u8; 16],
    pub key: [u8; 32],
    pub out_pat: [u8; 16],
    pub spbc: u32,
    pub __rsvd: [u8; 44],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_aes_gcm {
    pub in_pat_or_aad: [u8; 16],
    pub iv_or_cnt: [u8; 16],
    pub bit_length_aad: u64,
    pub bit_length_data: u64,
    pub in_s0: [u8; 16],
    pub key: [u8; 32],
    pub __rsvd1: [u8; 16],
    pub out_pat_or_mac: [u8; 16],
    pub out_s0: [u8; 16],
    pub out_cnt: [u8; 16],
    pub spbc: u32,
    pub __rsvd2: [u8; 12],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_aes_ctr {
    pub iv: [u8; 16],
    pub key: [u8; 32],
    pub cv: [u8; 16],
    pub spbc: u32,
    pub __rsvd2: [u8; 44],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_aes_cca {
    pub b0: [u8; 16],
    pub b1: [u8; 16],
    pub key: [u8; 16],
    pub out_pat_or_b0: [u8; 16],
    pub spbc: u32,
    pub __rsvd: [u8; 44],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_aes_ccm {
    pub in_pat_or_b0: [u8; 16],
    pub iv_or_ctr: [u8; 16],
    pub in_s0: [u8; 16],
    pub key: [u8; 16],
    pub __rsvd1: [u8; 48],
    pub out_pat_or_mac: [u8; 16],
    pub out_s0: [u8; 16],
    pub out_ctr: [u8; 16],
    pub spbc: u32,
    pub __rsvd2: [u8; 12],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_aes_xcbc {
    pub cv: [u8; 16],
    pub key: [u8; 16],
    pub __rsvd1: [u8; 16],
    pub out_cv_mac: [u8; 16],
    pub spbc: u32,
    pub __rsvd2: [u8; 44],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_sha256 {
    pub message_bit_length: u64,
    pub __rsvd1: u64,
    pub input_partial_digest: [u8; 32],
    pub message_digest: [u8; 32],
    pub spbc: u32,
    pub __rsvd2: [u8; 44],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_sha512 {
    pub message_bit_length_hi: u64,
    pub message_bit_length_lo: u64,
    pub input_partial_digest: [u8; 64],
    pub __rsvd1: [u8; 32],
    pub message_digest: [u8; 64],
    pub spbc: u32,
    pub __rsvd2: [u8; 76],
    pub __packed: },
pub const NX_FDM_INTERMEDIATE: c_uint = 0x01;
pub const NX_FDM_CONTINUATION: c_uint = 0x02;
pub const NX_FDM_ENDE_ENCRYPT: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_symcpb_header {
    pub mode: u8,
    pub fdm: u8,
    pub ks_ds: u8,
    pub pad_byte: u8,
    pub __rsvd: [u8; 12],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_parameter_block {
    pub hdr: cop_symcpb_header,
    pub aes_ecb: cop_symcpb_aes_ecb,
    pub aes_cbc: cop_symcpb_aes_cbc,
    pub aes_gca: cop_symcpb_aes_gca,
    pub aes_gcm: cop_symcpb_aes_gcm,
    pub aes_cca: cop_symcpb_aes_cca,
    pub aes_ccm: cop_symcpb_aes_ccm,
    pub aes_ctr: cop_symcpb_aes_ctr,
    pub aes_xcbc: cop_symcpb_aes_xcbc,
    pub sha256: cop_symcpb_sha256,
    pub sha512: cop_symcpb_sha512,
}

pub const NX_CSB_VALID_BIT: c_uint = 0x80;
// co-processor status block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cop_status_block {
    pub valid: u8,
    pub crb_seq_number: u8,
    pub completion_code: u8,
    pub completion_extension: u8,
    pub processed_byte_count: __be32,
    pub address: __be64,
    pub __packed: },
// Nest accelerator workbook section 4.4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_csbcpb {
    pub __rsvd: [c_uchar; 112],
    pub csb: cop_status_block,
    pub cpb: cop_parameter_block,
    pub __packed: },
// nx_csbcpb related definitions
pub const NX_MODE_AES_ECB: c_int = 0;
pub const NX_MODE_AES_CBC: c_int = 1;
pub const NX_MODE_AES_GMAC: c_int = 2;
pub const NX_MODE_AES_GCA: c_int = 3;
pub const NX_MODE_AES_GCM: c_int = 4;
pub const NX_MODE_AES_CCA: c_int = 5;
pub const NX_MODE_AES_CCM: c_int = 6;
pub const NX_MODE_AES_CTR: c_int = 7;
pub const NX_MODE_AES_XCBC_MAC: c_int = 20;
pub const NX_MODE_SHA: c_int = 0;
pub const NX_MODE_SHA_HMAC: c_int = 1;
pub const NX_MODE_AES_CBC_HMAC_ETA: c_int = 8;
pub const NX_MODE_AES_CBC_HMAC_ATE: c_int = 9;
pub const NX_MODE_AES_CBC_HMAC_EAA: c_int = 10;
pub const NX_MODE_AES_CTR_HMAC_ETA: c_int = 12;
pub const NX_MODE_AES_CTR_HMAC_ATE: c_int = 13;
pub const NX_MODE_AES_CTR_HMAC_EAA: c_int = 14;
pub const NX_FDM_CI_FULL: c_int = 0;
pub const NX_FDM_CI_FIRST: c_int = 1;
pub const NX_FDM_CI_LAST: c_int = 2;
pub const NX_FDM_CI_MIDDLE: c_int = 3;
pub const NX_FDM_PR_NONE: c_int = 0;
pub const NX_FDM_PR_PAD: c_int = 1;
pub const NX_KS_AES_128: c_int = 1;
pub const NX_KS_AES_192: c_int = 2;
pub const NX_KS_AES_256: c_int = 3;
pub const NX_DS_SHA256: c_int = 2;
pub const NX_DS_SHA512: c_int = 3;
pub const NX_FC_AES: c_int = 0;
pub const NX_FC_SHA: c_int = 2;
pub const NX_FC_AES_HMAC: c_int = 6;

// indices into the array of algorithm properties
pub const NX_PROPS_AES_128: c_int = 0;
pub const NX_PROPS_AES_192: c_int = 1;
pub const NX_PROPS_AES_256: c_int = 2;
pub const NX_PROPS_SHA256: c_int = 1;
pub const NX_PROPS_SHA512: c_int = 2;
