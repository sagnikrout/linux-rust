//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/dh.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Diffie-Hellman secret to be used with kpp API along with helper functions
//
// Copyright (c) 2016, Intel Corporation
// Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
//
// DOC: DH Helper Functions
//
// To use DH with the KPP cipher API, the following data structure and
// functions should be used.
//
// To use DH with KPP, the following functions should be used to operate on
// a DH private key. The packet private key that can be set with
// the KPP API function call of crypto_kpp_set_secret.
//
// struct dh - define a DH private key
//
// @key:	Private DH key
// @p:		Diffie-Hellman parameter P
// @g:		Diffie-Hellman generator G
// @key_size:	Size of the private DH key
// @p_size:	Size of DH parameter P
// @g_size:	Size of DH generator G
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dh {
    pub key: *const c_void,
    pub p: *const c_void,
    pub g: *const c_void,
    pub key_size: c_uint,
    pub p_size: c_uint,
    pub g_size: c_uint,
}

//
// crypto_dh_key_len() - Obtain the size of the private DH key
// @params:	private DH key
//
// This function returns the packet DH key size. A caller can use that
// with the provided DH private key reference to obtain the required
// memory size to hold a packet key.
//
// Return: size of the key in bytes
//
extern "C" {
    pub fn crypto_dh_key_len(params: *const dh) -> c_uint;
}
//
// crypto_dh_encode_key() - encode the private key
// @buf:	Buffer allocated by the caller to hold the packet DH
// private key. The buffer should be at least crypto_dh_key_len
// bytes in size.
// @len:	Length of the packet private key buffer
// @params:	Buffer with the caller-specified private key
//
// The DH implementations operate on a packet representation of the private
// key.
//
// Return:	-EINVAL if buffer has insufficient size, 0 on success
//
extern "C" {
    pub fn crypto_dh_encode_key(buf: *mut c_char, len: c_uint, params: *const dh) -> c_int;
}
//
// crypto_dh_decode_key() - decode a private key
// @buf:	Buffer holding a packet key that should be decoded
// @len:	Length of the packet private key buffer
// @params:	Buffer allocated by the caller that is filled with the
// unpacked DH private key.
//
// The unpacking obtains the private key by pointing @p to the correct location
// in @buf. Thus, both pointers refer to the same memory.
//
// Return:	-EINVAL if buffer has insufficient size, 0 on success
//
extern "C" {
    pub fn crypto_dh_decode_key(buf: *const c_char, len: c_uint, params: *mut dh) -> c_int;
}
//
// __crypto_dh_decode_key() - decode a private key without parameter checks
// @buf:	Buffer holding a packet key that should be decoded
// @len:	Length of the packet private key buffer
// @params:	Buffer allocated by the caller that is filled with the
// unpacked DH private key.
//
// Internal function providing the same services as the exported
// crypto_dh_decode_key(), but without any of those basic parameter
// checks conducted by the latter.
//
// Return:	-EINVAL if buffer has insufficient size, 0 on success
//
