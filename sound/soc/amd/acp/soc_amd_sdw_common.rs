//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/acp/soc_amd_sdw_common.h
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
// Copyright (c) 2024 Advanced Micro Devices, Inc. All rights reserved
//
// soc_amd_sdw_common.h - prototypes for common helpers
//

pub const ACP63_SDW_MAX_CPU_DAIS: c_int = 8;
pub const ACP63_SDW_MAX_LINKS: c_int = 2;
pub const AMD_SDW_MAX_GROUPS: c_int = 9;
pub const ACP63_PCI_REV: c_uint = 0x63;
pub const ACP70_PCI_REV: c_uint = 0x70;
pub const ACP71_PCI_REV: c_uint = 0x71;
pub const ACP72_PCI_REV: c_uint = 0x72;

pub const AMD_SDW0: c_int = 0;
pub const AMD_SDW1: c_int = 1;
pub const ACP63_SW0_AUDIO0_TX: c_int = 0;
pub const ACP63_SW0_AUDIO1_TX: c_int = 1;
pub const ACP63_SW0_AUDIO2_TX: c_int = 2;
pub const ACP63_SW0_AUDIO0_RX: c_int = 3;
pub const ACP63_SW0_AUDIO1_RX: c_int = 4;
pub const ACP63_SW0_AUDIO2_RX: c_int = 5;
pub const ACP63_SW1_AUDIO0_TX: c_int = 0;
pub const ACP63_SW1_AUDIO0_RX: c_int = 1;
pub const ACP_DMIC_BE_ID: c_int = 4;
pub const ACP70_SW_AUDIO0_TX: c_int = 0;
pub const ACP70_SW_AUDIO1_TX: c_int = 1;
pub const ACP70_SW_AUDIO2_TX: c_int = 2;
pub const ACP70_SW_AUDIO0_RX: c_int = 3;
pub const ACP70_SW_AUDIO1_RX: c_int = 4;
pub const ACP70_SW_AUDIO2_RX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_mc_ctx {
    pub acp_rev: c_uint,
    pub max_sdw_links: c_uint,
}

extern "C" {
    pub fn get_acp63_cpu_pin_id(sdw_link_id: u32, be_id: c_int, cpu_pin_id: *mut c_int, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn get_acp70_cpu_pin_id(sdw_link_id: u32, be_id: c_int, cpu_pin_id: *mut c_int, dev: *mut device) -> c_int;
}
