//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/inc/bounding_boxes/utm_qos_model_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utm_soc_operating_point {
    pub uclk_khz: u32,
    pub fclk_khz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utm_qos_model_socbb {
    pub fabric_datapath_to_dcn_data_return_bytes: u8,
    pub dram_channel_width_bytes: u8,
    pub dram_channel_count: u8,
    pub dram_transactions_per_clock: u8,
    pub fabric_derate_percent_nominal: u8,
    pub fabric_derate_percent_urgent: u8,
    pub dram_derate_percent_nominal: u8,
    pub dram_derate_percent_urgent: u8,
    pub lsdma_fabric_derate_percent: u8,
    pub lsdma_dram_derate_percent: u8,
    pub fabric_datapath_to_lsdma_data_return_bytes: u8,
}

pub const MAX_UTM_SOP_COUNT: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum utm_qos_model_version {
    utm_qos_model_version_v1,
    utm_qos_model_version_v2,
    utm_qos_model_version_v3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utm_qos_model {
    pub version: c_int,
    pub sops: [utm_soc_operating_point; MAX_UTM_SOP_COUNT],
    pub dchub_v1: *const utm_qos_model_dchub_v1,
    pub dchub_v2: *const utm_qos_model_dchub_v2,
    pub dchub_v3: *const utm_qos_model_dchub_v3,
}
