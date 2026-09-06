//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/inc/bounding_boxes/utm_qos_model_dchub_v2.h
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
pub struct utm_qos_model_dchub_memory_path_latency_v2 {
    pub urgent_ramp_ps: u32,
    pub t_trip_ps: u32,
    pub meta_trip_to_mem_ps: u32,
    pub max_req_latency_urg_ps: u32,
    pub avg_req_latency_urg_ps: u32,
    pub max_req_latency_non_urg_ps: u32,
    pub avg_req_latency_non_urg_ps: u32,
    pub df_response_time_ps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utm_qos_model_dchub_memory_path_bandwidth_v2 {
    pub nominal_bandwidth_KBps: u32,
    pub urgent_bandwidth_KBps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utm_qos_model_dchub_memory_path_qos_v2 {
    pub latency_upper_bound: utm_qos_model_dchub_memory_path_latency_v2,
    pub bandwidth_lower_bound: utm_qos_model_dchub_memory_path_bandwidth_v2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utm_qos_model_dchub_v2 {
    pub latencies: [utm_qos_model_dchub_memory_path_latency_v2; MAX_UTM_SOP_COUNT],
    pub max_nominal_utm_budget_percent: u8,
    pub min_nominal_utm_budget_percent: u8,
    pub max_urgent_utm_budget_percent: u8,
    pub min_urgent_utm_budget_percent: u8,
}

// socbb->dram_channel_count
// socbb->dram_channel_width_bytes
// socbb->dram_transactions_per_clock
// socbb->dram_derate_percent_nominal / 100;
// socbb->dram_channel_count
// socbb->dram_channel_width_bytes
// socbb->dram_transactions_per_clock
// socbb->dram_derate_percent_urgent / 100;
// socbb->fabric_datapath_to_dcn_data_return_bytes
// socbb->fabric_derate_percent_nominal / 100;
// socbb->fabric_datapath_to_dcn_data_return_bytes
// socbb->fabric_derate_percent_urgent / 100;
