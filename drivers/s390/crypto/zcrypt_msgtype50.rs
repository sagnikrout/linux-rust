//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/zcrypt_msgtype50.h
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
// Copyright IBM Corp. 2001, 2023
// Author(s): Robert Burroughs
// Eric Rossman (edrossma@us.ibm.com)
//
// Hotplug & misc device support: Jochen Roehrig (roehrig@de.ibm.com)
// Major cleanup & driver split: Martin Schwidefsky <schwidefsky@de.ibm.com>
// MSGTYPE restruct:		  Holger Dengler <hd@linux.vnet.ibm.com>
//

pub const MSGTYPE50_VARIANT_DEFAULT: c_int = 0;
pub const MSGTYPE50_CRB3_MAX_MSG_SIZE: c_uint = 0x710 /* sizeof(struct type50_crb3_msg) */;
pub const MSGTYPE_ADJUSTMENT: c_uint = 0x08  /* type04 extension (not needed in type50) */;
extern "C" {
    pub fn get_rsa_modex_fc(mex: *mut ica_rsa_modexpo, fc: *mut c_int) -> c_int;
}
extern "C" {
    pub fn get_rsa_crt_fc(crt: *mut ica_rsa_modexpo_crt, fc: *mut c_int) -> c_int;
}
extern "C" {
    pub fn zcrypt_msgtype50_init();
}
extern "C" {
    pub fn zcrypt_msgtype50_exit();
}
