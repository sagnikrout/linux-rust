//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/hisilicon/sec2/sec_crypto.h
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
// Copyright (c) 2019 HiSilicon Limited.
pub const SEC_AIV_SIZE: c_int = 12;
pub const SEC_IV_SIZE: c_int = 24;
pub const SEC_MAX_KEY_SIZE: c_int = 64;
pub const SEC_MAX_AKEY_SIZE: c_int = 128;
pub const SEC_COMM_SCENE: c_int = 0;
pub const SEC_MIN_BLOCK_SZ: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_calg {
    SEC_CALG_3DES = 0x1,
    SEC_CALG_AES  = 0x2,
    SEC_CALG_SM4  = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_hash_alg {
    SEC_A_HMAC_SHA1   = 0x10,
    SEC_A_HMAC_SHA256 = 0x11,
    SEC_A_HMAC_SHA512 = 0x15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_cmode {
    SEC_CMODE_ECB    = 0x0,
    SEC_CMODE_CBC    = 0x1,
    SEC_CMODE_CTR    = 0x4,
    SEC_CMODE_CCM    = 0x5,
    SEC_CMODE_GCM    = 0x6,
    SEC_CMODE_XTS    = 0x7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_ckey_type {
    SEC_CKEY_128BIT = 0x0,
    SEC_CKEY_192BIT = 0x1,
    SEC_CKEY_256BIT = 0x2,
    SEC_CKEY_3DES_3KEY = 0x1,
    SEC_CKEY_3DES_2KEY = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_bd_type {
    SEC_BD_TYPE1 = 0x1,
    SEC_BD_TYPE2 = 0x2,
    SEC_BD_TYPE3 = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_auth {
    SEC_NO_AUTH = 0x0,
    SEC_AUTH_TYPE1 = 0x1,
    SEC_AUTH_TYPE2 = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_cipher_dir {
    SEC_CIPHER_ENC = 0x1,
    SEC_CIPHER_DEC = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_addr_type {
    SEC_PBUF = 0x0,
    SEC_SGL  = 0x1,
    SEC_PRP  = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd_status {
    pub tag: u64,
    pub done: u8,
    pub err_type: u8,
    pub flag: u16,
    pub icv: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_sqe_type2 {
//
// mac_len: 0~4 bits
// a_key_len: 5~10 bits
// a_alg: 11~16 bits
//
    pub mac_key_alg: __le32,
//
// c_icv_len: 0~5 bits
// c_width: 6~8 bits
// c_key_len: 9~11 bits
// c_mode: 12~15 bits
//
    pub icvw_kmode: __le16,
// c_alg: 0~3 bits
    pub c_alg: __u8,
    pub rsvd4: __u8,
//
// a_len: 0~23 bits
// iv_offset_l: 24~31 bits
//
    pub alen_ivllen: __le32,
//
// c_len: 0~23 bits
// iv_offset_h: 24~31 bits
//
    pub clen_ivhlen: __le32,
    pub auth_src_offset: __le16,
    pub cipher_src_offset: __le16,
    pub cs_ip_header_offset: __le16,
    pub cs_udp_header_offset: __le16,
    pub pass_word_len: __le16,
    pub dk_len: __le16,
    pub salt3: __u8,
    pub salt2: __u8,
    pub salt1: __u8,
    pub salt0: __u8,
    pub tag: __le16,
    pub rsvd5: __le16,
//
// c_pad_type: 0~3 bits
// c_pad_len: 4~11 bits
// c_pad_data_type: 12~15 bits
//
    pub cph_pad: __le16,
// c_pad_len_field: 0~1 bits
    pub c_pad_len_field: __le16,
    pub long_a_data_len: __le64,
    pub a_ivin_addr: __le64,
    pub a_key_addr: __le64,
    pub mac_addr: __le64,
    pub c_ivin_addr: __le64,
    pub c_key_addr: __le64,
    pub data_src_addr: __le64,
    pub data_dst_addr: __le64,
//
// done: 0 bit
// icv: 1~3 bits
// csc: 4~6 bits
// flag: 7-10 bits
// dif_check: 11~13 bits
//
    pub done_flag: __le16,
    pub error_type: __u8,
    pub warning_type: __u8,
    pub mac_i3: __u8,
    pub mac_i2: __u8,
    pub mac_i1: __u8,
    pub mac_i0: __u8,
    pub check_sum_i: __le16,
    pub tls_pad_len_i: __u8,
    pub rsvd12: __u8,
    pub counter: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_sqe {
//
// type:	0~3 bits
// cipher:	4~5 bits
// auth:	6~7 bit s
//
    pub type_cipher_auth: __u8,
//
// seq:	0 bit
// de:	1~2 bits
// scene:	3~6 bits
// src_addr_type: ~7 bit, with sdm_addr_type 0-1 bits
//
    pub sds_sa_type: __u8,
//
// src_addr_type: 0~1 bits, not used now,
// if support PRP, set this field, or set zero.
// dst_addr_type: 2~4 bits
// mac_addr_type: 5~7 bits
//
    pub sdm_addr_type: __u8,
    pub rsvd0: __u8,
//
// nonce_len(type2): 0~3 bits
// huk(type2): 4 bit
// key_s(type2): 5 bit
// ci_gen: 6~7 bits
//
    pub huk_key_ci: __u8,
//
// ai_gen: 0~1 bits
// a_pad(type2): 2~3 bits
// c_s(type2): 4~5 bits
//
    pub ai_apd_cs: __u8,
//
// rhf(type2): 0 bit
// c_key_type: 1~2 bits
// a_key_type: 3~4 bits
// write_frame_len(type2): 5~7 bits
//
    pub rca_key_frm: __u8,
//
// cal_iv_addr_en(type2): 0 bit
// tls_up(type2): 1 bit
// inveld: 7 bit
//
    pub iv_tls_ld: __u8,
// Just using type2 BD now
    pub type2: sec_sqe_type2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd3_auth_ivin {
    pub a_ivin_addr: __le64,
    pub rsvd0: __le32,
    pub rsvd1: __le32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd3_skip_data {
    pub rsvd0: __le32,
//
// gran_num: 0~15 bits
// reserved: 16~31 bits
//
    pub gran_num: __le32,
//
// src_skip_data_len: 0~24 bits
// reserved: 25~31 bits
//
    pub src_skip_data_len: __le32,
//
// dst_skip_data_len: 0~24 bits
// reserved: 25~31 bits
//
    pub dst_skip_data_len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd3_stream_scene {
    pub c_ivin_addr: __le64,
    pub long_a_data_len: __le64,
//
// auth_pad: 0~1 bits
// stream_protocol: 2~4 bits
// reserved: 5~7 bits
//
    pub stream_auth_pad: __u8,
    pub plaintext_type: __u8,
    pub pad_len_1p3: __le16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd3_no_scene {
    pub c_ivin_addr: __le64,
    pub rsvd0: __le32,
    pub rsvd1: __le32,
    pub rsvd2: __le32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd3_check_sum {
    pub rsvd0: __u8,
    pub hac_sva_status: __u8,
    pub check_sum_i: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd3_tls_type_back {
    pub tls_1p3_type_back: __u8,
    pub hac_sva_status: __u8,
    pub pad_len_1p3_back: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_sqe3 {
//
// type: 0~3 bit
// bd_invalid: 4 bit
// scene: 5~8 bit
// de: 9~10 bit
// src_addr_type: 11~13 bit
// dst_addr_type: 14~16 bit
// mac_addr_type: 17~19 bit
// reserved: 20~31 bits
//
    pub bd_param: __le32,
//
// cipher: 0~1 bits
// ci_gen: 2~3 bit
// c_icv_len: 4~9 bit
// c_width: 10~12 bits
// c_key_len: 13~15 bits
//
    pub c_icv_key: __le16,
//
// c_mode : 0~3 bits
// c_alg : 4~7 bits
//
    pub c_mode_alg: __u8,
//
// nonce_len : 0~3 bits
// huk : 4 bits
// cal_iv_addr_en : 5 bits
// seq : 6 bits
// reserved : 7 bits
//
    pub huk_iv_seq: __u8,
    pub tag: __le64,
    pub data_src_addr: __le64,
    pub a_key_addr: __le64,
    pub auth_ivin: bd3_auth_ivin,
    pub skip_data: bd3_skip_data,
}

//
// auth: 0~1 bits
// ai_gen: 2~3 bits
// mac_len: 4~8 bits
// akey_len: 9~14 bits
// a_alg: 15~20 bits
// key_sel: 21~24 bits
// ctr_count_mode/sm4_xts: 25~26 bits
// sva_prefetch: 27 bits
// key_wrap_num: 28~30 bits
// update_key: 31 bits
//
// auth_len: 0~23 bit
// auth_key_offset: 24~31 bits
//
// cipher_len: 0~23 bit
// auth_ivin_offset: 24~31 bits
//
// done: 0 bit
// icv: 1~3 bit
// csc: 4~6 bit
// flag: 7~10 bit
// reserved: 11~15 bit
//
extern "C" {
    pub fn sec_register_to_crypto(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn sec_unregister_from_crypto(qm: *mut hisi_qm);
}
