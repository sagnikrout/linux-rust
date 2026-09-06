//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/ecdh.h
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
// ECDH params to be used with kpp API
//
// Copyright (c) 2016, Intel Corporation
// Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
//
// DOC: ECDH Helper Functions
//
// To use ECDH with the KPP cipher API, the following data structure and
// functions should be used.
//
// The ECC curves known to the ECDH implementation are specified in this
// header file.
//
// To use ECDH with KPP, the following functions should be used to operate on
// an ECDH private key. The packet private key that can be set with
// the KPP API function call of crypto_kpp_set_secret.
//
// Curves IDs
pub const ECC_CURVE_NIST_P192: c_uint = 0x0001;
pub const ECC_CURVE_NIST_P256: c_uint = 0x0002;
pub const ECC_CURVE_NIST_P384: c_uint = 0x0003;
pub const ECC_CURVE_NIST_P521: c_uint = 0x0004;
//
// struct ecdh - define an ECDH private key
//
// @key:	Private ECDH key
// @key_size:	Size of the private ECDH key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecdh {
    pub key: *mut c_char,
    pub key_size: c_ushort,
}

//
// crypto_ecdh_key_len() - Obtain the size of the private ECDH key
// @params:	private ECDH key
//
// This function returns the packet ECDH key size. A caller can use that
// with the provided ECDH private key reference to obtain the required
// memory size to hold a packet key.
//
// Return: size of the key in bytes
//
extern "C" {
    pub fn crypto_ecdh_key_len(params: *const ecdh) -> c_uint;
}
//
// crypto_ecdh_encode_key() - encode the private key
// @buf:	Buffer allocated by the caller to hold the packet ECDH
// private key. The buffer should be at least crypto_ecdh_key_len
// bytes in size.
// @len:	Length of the packet private key buffer
// @p:		Buffer with the caller-specified private key
//
// The ECDH implementations operate on a packet representation of the private
// key.
//
// Return:	-EINVAL if buffer has insufficient size, 0 on success
//
extern "C" {
    pub fn crypto_ecdh_encode_key(buf: *mut c_char, len: c_uint, p: *const ecdh) -> c_int;
}
//
// crypto_ecdh_decode_key() - decode a private key
// @buf:	Buffer holding a packet key that should be decoded
// @len:	Length of the packet private key buffer
// @p:		Buffer allocated by the caller that is filled with the
// unpacked ECDH private key.
//
// The unpacking obtains the private key by pointing @p to the correct location
// in @buf. Thus, both pointers refer to the same memory.
//
// Return:	-EINVAL if buffer has insufficient size, 0 on success
//
extern "C" {
    pub fn crypto_ecdh_decode_key(buf: *const c_char, len: c_uint, p: *mut ecdh) -> c_int;
}
