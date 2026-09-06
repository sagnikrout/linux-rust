//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/inc/bounding_boxes/utm_qos_model_dchub_v3.h
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
// Must match DALSMC_MAX_UTM_SOP_COUNT in dalsmc.h without including it
pub const UTM_QOS_MODEL_V3_MAX_LOAD_LEVEL_COUNT: c_int = 3;
pub const UTM_QOS_MODEL_V3_MAX_SOP_COUNT: c_int = 5;
pub const UTM_QOS_MODEL_V3_LOAD_LEVEL_IDLE: c_int = 0;
pub const UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE_ALTERNATE_PSTATE: c_int = 1;
pub const UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE: c_int = 2;
//
// utm_qos_model_dchub_v3_sop_entry - Per-SOP QoS parameters for one load level.
//
// All latency fields are in picoseconds. All bandwidth fields are in KBps.
// Budget percentage and derate are pre-applied — callers use values
// directly without further scaling.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utm_qos_model_dchub_v3_sop_entry {
// latencies
    pub urgent_ramp_ps: u32,
    pub t_trip_ps: u32,
    pub meta_trip_to_mem_ps: u32,
    pub max_req_latency_urg_ps: u32,
    pub avg_req_latency_urg_ps: u32,
    pub max_req_latency_non_urg_ps: u32,
    pub avg_req_latency_non_urg_ps: u32,
    pub df_response_time_ps: u32,
// bandwidths (budget allocation and derate pre-applied)
    pub urgent_bandwidth_KBps: u32,
    pub nominal_bandwidth_KBps: u32,
    pub lsdma_bandwidth_KBps: u32,
}

//
// utm_qos_model_dchub_v3 - DCN6 flat UTM QoS table.
//
// Indexed as sops[load_level][sop_index]. Load level constants:
// UTM_QOS_MODEL_V3_LOAD_LEVEL_IDLE                    (max budget %)
// UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE_ALTERNATE_PSTATE (min budget %)
// UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE                  (same as alt pstate, lsdma=0)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utm_qos_model_dchub_v3 {
    pub load_level_count: u8,
    pub sop_count: u8,
}
