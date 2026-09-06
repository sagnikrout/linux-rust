//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/zcrypt_cca_key.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright IBM Corp. 2001, 2006
// Author(s): Robert Burroughs
// Eric Rossman (edrossma@us.ibm.com)
//
// Hotplug & misc device support: Jochen Roehrig (roehrig@de.ibm.com)
// Major cleanup & driver split: Martin Schwidefsky <schwidefsky@de.ibm.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t6_keyblock_hdr {
    pub blen: c_ushort,
    pub ulen: c_ushort,
    pub flags: c_ushort,
}

//
// mapping for the cca private ME key token.
// Three parts of interest here: the header, the private section and
// the public section.
//
// mapping for the cca key token header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cca_token_hdr {
    pub token_identifier: c_uchar,
    pub version: c_uchar,
    pub token_length: c_ushort,
    pub reserved: [c_uchar; 4],
    pub __packed: },
pub const CCA_TKN_HDR_ID_EXT: c_uint = 0x1E;
pub const CCA_PVT_USAGE_ALL: c_uint = 0x80;
//
// mapping for the cca public section
// In a private key, the modulus doesn't appear in the public
// section. So, an arbitrary public exponent of 0x010001 will be
// used, for a section length of 0x0F always.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cca_public_sec {
    pub section_identifier: c_uchar,
    pub version: c_uchar,
    pub section_length: c_ushort,
    pub reserved: [c_uchar; 2],
    pub exponent_len: c_ushort,
    pub modulus_bit_len: c_ushort,
    pub /: *mut *mut unsigned short modulus_byte_len; / In a private key, this is 0,
    pub __packed: },
//
// mapping for the cca private CRT key 'token'
// The first three parts (the only parts considered in this release)
// are: the header, the private section and the public section.
// The header and public section are the same as for the
// struct cca_private_ext_ME
//
// Following the structure are the quantities p, q, dp, dq, u, pad,
// and modulus, in that order, where pad_len is the modulo 8
// complement of the residue modulo 8 of the sum of
// (p_len + q_len + dp_len + dq_len + u_len).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cca_pvt_ext_crt_sec {
    pub section_identifier: c_uchar,
    pub version: c_uchar,
    pub section_length: c_ushort,
    pub private_key_hash: [c_uchar; 20],
    pub reserved1: [c_uchar; 4],
    pub key_format: c_uchar,
    pub reserved2: c_uchar,
    pub key_name_hash: [c_uchar; 20],
    pub key_use_flags: [c_uchar; 4],
    pub p_len: c_ushort,
    pub q_len: c_ushort,
    pub dp_len: c_ushort,
    pub dq_len: c_ushort,
    pub u_len: c_ushort,
    pub mod_len: c_ushort,
    pub reserved3: [c_uchar; 4],
    pub pad_len: c_ushort,
    pub reserved4: [c_uchar; 52],
    pub confounder: [c_uchar; 8],
    pub __packed: },
pub const CCA_PVT_EXT_CRT_SEC_ID_PVT: c_uint = 0x08;
pub const CCA_PVT_EXT_CRT_SEC_FMT_CL: c_uint = 0x40;
//
// Set up private key fields of a type6 MEX message.
//
// @mex: pointer to user input data
// @p: pointer to memory area for the key
//
// Returns the size of the key area or negative errno value.
//
}

//
// The inputdatalength was a selection criteria in the dispatching
// function zcrypt_rsa_modexpo(). However, do a plausibility check
// here to make sure the following copy_from_user() can't be utilized
// to compromise the system.
//
// key parameter block
// modulus
//
// Set up private key fields of a type6 CRT message.
//
// @mex: pointer to user input data
// @p: pointer to memory area for the key
//
// Returns the size of the key area or -EFAULT
//
// The inputdatalength was a selection criteria in the dispatching
// function zcrypt_rsa_crt(). However, do a plausibility check
// here to make sure the following copy_from_user() can't be utilized
// to compromise the system.
//
// parameter block.key block
// key token header
// private section
// key parts
// pub = static_cca_pub_sec;
//
// In a private key, the modulus doesn't appear in the public
// section. So, an arbitrary public exponent of 0x010001 will be
// used.
//
