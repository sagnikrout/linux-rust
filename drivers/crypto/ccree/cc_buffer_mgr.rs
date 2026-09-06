//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_buffer_mgr.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2012-2019 ARM Limited (or its affiliates).
// \file cc_buffer_mgr.h
// Buffer Manager
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_req_dma_buf_type {
    CC_DMA_BUF_NULL = 0,
    CC_DMA_BUF_DLLI,
    CC_DMA_BUF_MLLI
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_sg_cpy_direct {
    CC_SG_TO_BUF = 0,
    CC_SG_FROM_BUF = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_mlli {
    pub sram_addr: u32,
    pub mapped_nents: c_uint,
    pub nents: unsigned int nents; //sg,
    pub above: unsigned int mlli_nents; //mlli nents might be different than the,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlli_params {
    pub curr_pool: *mut dma_pool,
    pub mlli_virt_addr: *mut c_void,
    pub mlli_dma_addr: dma_addr_t,
    pub mlli_len: u32,
}

extern "C" {
    pub fn cc_buffer_mgr_init(drvdata: *mut cc_drvdata) -> c_int;
}
extern "C" {
    pub fn cc_buffer_mgr_fini(drvdata: *mut cc_drvdata) -> c_int;
}
extern "C" {
    pub fn cc_map_aead_request(drvdata: *mut cc_drvdata, req: *mut aead_request) -> c_int;
}
extern "C" {
    pub fn cc_unmap_aead_request(dev: *mut device, req: *mut aead_request);
}
