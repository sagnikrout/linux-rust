//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/xlnx-zynqmp-crypto.h
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
// Firmware layer for XilSECURE APIs.
//
// Copyright (C) 2014-2022 Xilinx, Inc.
// Copyright (C) 2022-2025 Advanced Micro Devices, Inc.
//
// struct xlnx_feature - Feature data
// @family:	Family code of platform
// @subfamily:	Subfamily code of platform
// @feature_id:	Feature id of module
// @data:	Collection of all supported platform data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlnx_feature {
    pub family: u32,
    pub feature_id: u32,
    pub data: *mut c_void,
}

// xilSecure API commands module id + api id
pub const XSECURE_API_AES_INIT: c_uint = 0x509;
pub const XSECURE_API_AES_OP_INIT: c_uint = 0x50a;
pub const XSECURE_API_AES_UPDATE_AAD: c_uint = 0x50b;
pub const XSECURE_API_AES_ENCRYPT_UPDATE: c_uint = 0x50c;
pub const XSECURE_API_AES_ENCRYPT_FINAL: c_uint = 0x50d;
pub const XSECURE_API_AES_DECRYPT_UPDATE: c_uint = 0x50e;
pub const XSECURE_API_AES_DECRYPT_FINAL: c_uint = 0x50f;
pub const XSECURE_API_AES_KEY_ZERO: c_uint = 0x510;
pub const XSECURE_API_AES_WRITE_KEY: c_uint = 0x511;

extern "C" {
    pub fn zynqmp_pm_aes_engine(address: u64, out: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_sha_hash(address: u64, size: u32, flags: u32) -> c_int;
}
extern "C" {
    pub fn versal_pm_aes_key_zero(keysrc: u32) -> c_int;
}
extern "C" {
    pub fn versal_pm_aes_op_init(hw_req: u64) -> c_int;
}
extern "C" {
    pub fn versal_pm_aes_update_aad(aad_addr: u64, aad_len: u32) -> c_int;
}
extern "C" {
    pub fn versal_pm_aes_enc_update(in_params: u64, in_addr: u64) -> c_int;
}
extern "C" {
    pub fn versal_pm_aes_dec_update(in_params: u64, in_addr: u64) -> c_int;
}
extern "C" {
    pub fn versal_pm_aes_dec_final(gcm_addr: u64) -> c_int;
}
extern "C" {
    pub fn versal_pm_aes_enc_final(gcm_addr: u64) -> c_int;
}
extern "C" {
    pub fn versal_pm_aes_init() -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

