//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_bank_state.h
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
// Copyright(c) 2025 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_config {
    pub base: u64,
    pub config: u32,
    pub head: u32,
    pub tail: u32,
    pub reserved0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_bank_state {
    pub ringstat0: u32,
    pub ringstat1: u32,
    pub ringuostat: u32,
    pub ringestat: u32,
    pub ringnestat: u32,
    pub ringnfstat: u32,
    pub ringfstat: u32,
    pub ringcstat0: u32,
    pub ringcstat1: u32,
    pub ringcstat2: u32,
    pub ringcstat3: u32,
    pub iaintflagen: u32,
    pub iaintflagreg: u32,
    pub iaintflagsrcsel0: u32,
    pub iaintflagsrcsel1: u32,
    pub iaintcolen: u32,
    pub iaintcolctl: u32,
    pub iaintflagandcolen: u32,
    pub ringexpstat: u32,
    pub ringexpintenable: u32,
    pub ringsrvarben: u32,
    pub reserved0: u32,
    pub rings: [ring_config; ADF_ETR_MAX_RINGS_PER_BANK],
}
