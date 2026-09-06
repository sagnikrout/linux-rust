//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/qce/dma.h
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
// Copyright (c) 2011-2014, The Linux Foundation. All rights reserved.
//

// maximum data transfer block size between BAM and CE
pub const QCE_BAM_BURST_SIZE: c_int = 64;
pub const QCE_AUTHIV_REGS_CNT: c_int = 16;
pub const QCE_AUTH_BYTECOUNT_REGS_CNT: c_int = 4;
pub const QCE_CNTRIV_REGS_CNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_result_dump {
    pub auth_iv: [u32; QCE_AUTHIV_REGS_CNT],
    pub auth_byte_count: [u32; QCE_AUTH_BYTECOUNT_REGS_CNT],
    pub encr_cntr_iv: [u32; QCE_CNTRIV_REGS_CNT],
    pub status: u32,
    pub status2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_dma_data {
    pub txchan: *mut dma_chan,
    pub rxchan: *mut dma_chan,
    pub result_buf: *mut qce_result_dump,
    pub ignore_buf: *mut c_void,
}

extern "C" {
    pub fn devm_qce_dma_request(dev: *mut device, dma: *mut qce_dma_data) -> c_int;
}
extern "C" {
    pub fn qce_dma_issue_pending(dma: *mut qce_dma_data);
}
extern "C" {
    pub fn qce_dma_terminate_all(dma: *mut qce_dma_data) -> c_int;
}
