//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_mstate_mgr.h
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
// Copyright(c) 2024 Intel Corporation
pub const ADF_MSTATE_ID_LEN: c_int = 8;

pub const ADF_MSTATE_SECTION_NUM: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_mstate_mgr {
    pub buf: *mut u8,
    pub state: *mut u8,
    pub size: u32,
    pub n_sects: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_mstate_preh {
    pub magic: u32,
    pub version: u32,
    pub preh_len: u16,
    pub n_sects: u16,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_mstate_vreginfo {
    pub addr: *mut c_void,
    pub size: u32,
}

extern "C" {
    pub fn int(preamble: *mut *mut adf_mstate_preamble_checker)(struct adf_mstate_preh, opa: *mut c_void) -> typedef;
}
extern "C" {
    pub fn adf_mstate_mgr_destroy(mgr: *mut adf_mstate_mgr);
}
extern "C" {
    pub fn adf_mstate_mgr_init(mgr: *mut adf_mstate_mgr, buf: *mut u8, size: u32);
}
extern "C" {
    pub fn adf_mstate_preamble_update(mgr: *mut adf_mstate_mgr) -> c_int;
}
extern "C" {
    pub fn adf_mstate_state_size(mgr: *mut adf_mstate_mgr) -> u32;
}
extern "C" {
    pub fn adf_mstate_state_size_from_remote(mgr: *mut adf_mstate_mgr) -> u32;
}
