//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_common.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

pub const HINIC3_MIN_PAGE_SIZE: c_uint = 0x1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_dma_addr_align {
    pub real_size: u32,
    pub ori_vaddr: *mut c_void,
    pub ori_paddr: dma_addr_t,
    pub align_vaddr: *mut c_void,
    pub align_paddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_wait_return {
    HINIC3_WAIT_PROCESS_CPL     = 0,
    HINIC3_WAIT_PROCESS_WAITING = 1,
    HINIC3_WAIT_PROCESS_ERR     = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_sge {
    pub hi_addr: __le32,
    pub lo_addr: __le32,
    pub len: __le32,
    pub rsvd: __le32,
}

extern "C" {
    pub fn hinic3_wait_return(priv_data: *mut *mut wait_cpl_handler)(void) -> typedef enum;
}
extern "C" {
    pub fn hinic3_cmdq_buf_swab32(data: *mut c_void, len: c_int);
}
