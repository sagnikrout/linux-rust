//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_hw_20_comp.h
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
// Copyright(c) 2022 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_comp_20_config_csr_lower {
    pub edmm: icp_qat_hw_comp_20_extended_delay_match_mode,
    pub algo: icp_qat_hw_comp_20_hw_comp_format,
    pub sd: icp_qat_hw_comp_20_search_depth,
    pub hbs: icp_qat_hw_comp_20_hbs_control,
    pub abd: icp_qat_hw_comp_20_abd,
    pub lllbd: icp_qat_hw_comp_20_lllbd_ctrl,
    pub mmctrl: icp_qat_hw_comp_20_min_match_control,
    pub hash_col: icp_qat_hw_comp_20_skip_hash_collision,
    pub hash_update: icp_qat_hw_comp_20_skip_hash_update,
    pub skip_ctrl: icp_qat_hw_comp_20_byte_skip,
}

extern "C" {
    pub fn swab32(_arg: val32) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_comp_20_config_csr_upper {
    pub scb_ctrl: icp_qat_hw_comp_20_scb_control,
    pub rmb_ctrl: icp_qat_hw_comp_20_rmb_control,
    pub som_ctrl: icp_qat_hw_comp_20_som_control,
    pub skip_hash_ctrl: icp_qat_hw_comp_20_skip_hash_rd_control,
    pub scb_unload_ctrl: icp_qat_hw_comp_20_scb_unload_control,
    pub disable_token_fusion_ctrl: icp_qat_hw_comp_20_disable_token_fusion_control,
    pub lbms: icp_qat_hw_comp_20_lbms,
    pub scb_mode_reset: icp_qat_hw_comp_20_scb_mode_reset_mask,
    pub lazy: __u16,
    pub nice: __u16,
}

extern "C" {
    pub fn swab32(_arg: val32) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_decomp_20_config_csr_lower {
    pub hbs: icp_qat_hw_decomp_20_hbs_control,
    pub lbms: icp_qat_hw_decomp_20_lbms,
    pub algo: icp_qat_hw_decomp_20_hw_comp_format,
    pub mmctrl: icp_qat_hw_decomp_20_min_match_control,
    pub lbc: icp_qat_hw_decomp_20_lz4_block_checksum_present,
}

extern "C" {
    pub fn swab32(_arg: val32) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_hw_decomp_20_config_csr_upper {
    pub sdc: icp_qat_hw_decomp_20_speculative_decoder_control,
    pub mcc: icp_qat_hw_decomp_20_mini_cam_control,
}

extern "C" {
    pub fn swab32(_arg: val32) -> return;
}
