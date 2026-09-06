//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/tpm/tpm_crb_ffa.h
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
// Copyright (C) 2024 Arm Ltd.
//
// This device driver implements the TPM CRB start method
// as defined in the TPM Service Command Response Buffer
// Interface Over FF-A (DEN0138).
//

extern "C" {
    pub fn tpm_crb_ffa_init() -> c_int;
}
extern "C" {
    pub fn tpm_crb_ffa_start(request_type: c_int, locality: c_int) -> c_int;
}

pub const CRB_FFA_START_TYPE_COMMAND: c_int = 0;
pub const CRB_FFA_START_TYPE_LOCALITY_REQUEST: c_int = 1;
