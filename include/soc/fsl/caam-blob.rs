//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/caam-blob.h
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
// Copyright (C) 2020 Pengutronix, Ahmad Fatoum <kernel@pengutronix.de>
// Copyright 2024-2025 NXP
//

pub const CAAM_BLOB_KEYMOD_LENGTH: c_int = 16;

pub const CAAM_BLOB_MAX_LEN: c_int = 4096;
pub const CAAM_ENC_ALGO_CCM: c_uint = 0x1;
pub const CAAM_ENC_ALGO_ECB: c_uint = 0x2;
pub const CAAM_NONCE_SIZE: c_int = 6;
pub const CAAM_ICV_SIZE: c_int = 6;

//
// struct caam_pkey_info - information for CAAM protected key
// @is_pkey:		flag to identify, if the key is protected.
// @key_enc_algo:	identifies the algorithm, ccm or ecb
// @plain_key_sz:	size of plain key.
// @key_buf:		contains key data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_pkey_info {
    pub is_pkey: u8,
    pub key_enc_algo: u8,
    pub plain_key_sz: u16,
    pub key_buf: [u8; ],
    pub __packed: },
// sizeof struct caam_pkey_info
pub const CAAM_PKEY_HEADER: c_int = 4;
//
// struct caam_blob_info - information for CAAM blobbing
// @pkey_info:	 pointer to keep protected key information
// @input:       pointer to input buffer (must be DMAable)
// @input_len:   length of @input buffer in bytes.
// @output:      pointer to output buffer (must be DMAable)
// @output_len:  length of @output buffer in bytes.
// @key_mod:     key modifier
// @key_mod_len: length of @key_mod in bytes.
// May not exceed %CAAM_BLOB_KEYMOD_LENGTH
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_blob_info {
    pub pkey_info: caam_pkey_info,
    pub input: *mut c_void,
    pub input_len: usize,
    pub output: *mut c_void,
    pub output_len: usize,
    pub key_mod: *const c_void,
    pub key_mod_len: usize,
}

//
// caam_blob_gen_init - initialize blob generation
// Return: pointer to new &struct caam_blob_priv instance on success
// and ``ERR_PTR(-ENODEV)`` if CAAM has no hardware blobbing support
// or no job ring could be allocated.
//
// caam_blob_gen_exit - free blob generation resources
// @priv: instance returned by caam_blob_gen_init()
//
extern "C" {
    pub fn caam_blob_gen_exit(priv: *mut caam_blob_priv);
}
//
// caam_process_blob - encapsulate or decapsulate blob
// @priv:   instance returned by caam_blob_gen_init()
// @info:   pointer to blobbing info describing key, blob and
// key modifier buffers.
// @encap:  true for encapsulation, false for decapsulation
//
// Return: %0 and sets ``info->output_len`` on success and a negative
// error code otherwise.
//
// caam_encap_blob - encapsulate blob
// @priv:   instance returned by caam_blob_gen_init()
// @info:   pointer to blobbing info describing input key,
// output blob and key modifier buffers.
//
// Return: %0 and sets ``info->output_len`` on success and
// a negative error code otherwise.
//
extern "C" {
    pub fn caam_process_blob(_arg: priv, _arg: info, _arg: true) -> return;
}
//
// caam_decap_blob - decapsulate blob
// @priv:   instance returned by caam_blob_gen_init()
// @info:   pointer to blobbing info describing output key,
// input blob and key modifier buffers.
//
// Return: %0 and sets ``info->output_len`` on success and
// a negative error code otherwise.
//
extern "C" {
    pub fn caam_process_blob(_arg: priv, _arg: info, _arg: false) -> return;
}
