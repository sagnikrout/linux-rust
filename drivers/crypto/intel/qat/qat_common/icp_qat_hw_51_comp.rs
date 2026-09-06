//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_hw_51_comp.h
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
pub struct icp_qat_hw_comp_51_config_csr_lower {
    pub abd: icp_qat_hw_comp_51_abd,
    pub lllbd: icp_qat_hw_comp_51_lllbd_ctrl,
    pub sd: icp_qat_hw_comp_51_search_depth,
    pub mmctrl: icp_qat_hw_comp_51_min_match_control,
    pub lbc: icp_qat_hw_comp_51_lz4_block_checksum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_comp_51_config_csr_upper {
    pub edmm: icp_qat_hw_comp_51_dmm_algorithm,
    pub bms: icp_qat_hw_comp_51_bms,
    pub scb_mode_reset: icp_qat_hw_comp_51_scb_mode_reset_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_decomp_51_config_csr_lower {
    pub lbc: icp_qat_hw_decomp_51_lz4_block_checksum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_decomp_51_config_csr_upper {
    pub bms: icp_qat_hw_decomp_51_bms,
}
