//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/tdx_global_metadata.h
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
// Automatically generated TDX global metadata structures.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_sys_info_version {
    pub minor_version: u16,
    pub major_version: u16,
    pub update_version: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_sys_info_features {
    pub tdx_features0: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_sys_info_tdmr {
    pub max_tdmrs: u16,
    pub max_reserved_per_tdmr: u16,
    pub pamt_4k_entry_size: u16,
    pub pamt_2m_entry_size: u16,
    pub pamt_1g_entry_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_sys_info_td_ctrl {
    pub tdr_base_size: u16,
    pub tdcs_base_size: u16,
    pub tdvps_base_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_sys_info_td_conf {
    pub attributes_fixed0: u64,
    pub attributes_fixed1: u64,
    pub xfam_fixed0: u64,
    pub xfam_fixed1: u64,
    pub num_cpuid_config: u16,
    pub max_vcpus_per_td: u16,
    pub cpuid_config_leaves: [u64; 128],
    pub cpuid_config_values: [u64; 128][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_sys_info_handoff {
    pub module_hv: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_sys_info {
    pub version: tdx_sys_info_version,
    pub features: tdx_sys_info_features,
    pub tdmr: tdx_sys_info_tdmr,
    pub td_ctrl: tdx_sys_info_td_ctrl,
    pub td_conf: tdx_sys_info_td_conf,
}
