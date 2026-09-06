//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/swap.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2015-2016 Qualcomm Atheros, Inc.
//

pub const ATH10K_SWAP_CODE_SEG_MAGIC_BYTES_SZ: c_int = 12;
pub const ATH10K_SWAP_CODE_SEG_NUM_MAX: c_int = 16;
// Currently only one swap segment is supported
pub const ATH10K_SWAP_CODE_SEG_NUM_SUPPORTED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_swap_code_seg_tlv {
    pub address: __le32,
    pub length: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_swap_code_seg_tail {
    pub magic_signature: [u8; ATH10K_SWAP_CODE_SEG_MAGIC_BYTES_SZ],
    pub bmi_write_addr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ath10k_swap_code_seg_item {
    pub tlv: ath10k_swap_code_seg_tlv,
    pub tail: ath10k_swap_code_seg_tail,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_swap_code_seg_hw_info {
// Swap binary image size
    pub swap_size: __le32,
    pub num_segs: __le32,
// Swap data size
    pub size: __le32,
    pub size_log2: __le32,
    pub bus_addr: [__le32; ATH10K_SWAP_CODE_SEG_NUM_MAX],
    pub reserved: [__le64; ATH10K_SWAP_CODE_SEG_NUM_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_swap_code_seg_info {
    pub seg_hw_info: ath10k_swap_code_seg_hw_info,
    pub virt_address: [*mut c_void; ATH10K_SWAP_CODE_SEG_NUM_SUPPORTED],
    pub target_addr: u32,
    pub paddr: [dma_addr_t; ATH10K_SWAP_CODE_SEG_NUM_SUPPORTED],
}
