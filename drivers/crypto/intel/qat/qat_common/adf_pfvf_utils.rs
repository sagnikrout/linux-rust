//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_pfvf_utils.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2021 Intel Corporation

// How long to wait for far side to acknowledge receipt
pub const ADF_PFVF_MSG_ACK_DELAY_US: c_int = 4;

extern "C" {
    pub fn adf_pfvf_calc_blkmsg_crc(buf: *const u8, buf_len: u8) -> u8;
}
extern "C" {
    pub fn adf_pfvf_crc_init();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_field_format {
    pub offset: u8,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_csr_format {
    pub type: pfvf_field_format,
    pub data: pfvf_field_format,
}
