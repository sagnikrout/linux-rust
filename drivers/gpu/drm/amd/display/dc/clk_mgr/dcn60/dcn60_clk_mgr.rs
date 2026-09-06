//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn60/dcn60_clk_mgr.h
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
pub union dcn60_clk_mgr_block_sequence_params {
// inputs
    pub ppclk: u32,
    pub freq_mhz: u16,
// outputs
    pub response: *mut c_int,
    pub update_hardmin_params: },
// inputs
    pub ppclk: u32,
    pub freq_khz: c_int,
// outputs
    pub response: *mut c_int,
    pub update_hardmin_optimized_params: },
// inputs
    pub freq_mhz: u16,
    pub update_deep_sleep_dcfclk_params: },
// inputs
    pub allow_fclk: bool,
    pub allow_uclk: bool,
    pub wait_resp: bool,
    pub drr_enable: bool,
    pub alt_ch_enable: bool,
    pub indicate_pstate_status_params: },
// inputs
    pub context: *mut dc_state,
    pub ref_dppclk_khz: *mut c_int,
    pub safe_to_lower: bool,
    pub update_dppclk_dto_params: },
// inputs
    pub context: *mut dc_state,
    pub ref_dtbclk_khz: *mut c_int,
    pub update_dtbclk_dto_params: },
// inputs
    pub context: *mut dc_state,
    pub update_dentist_params: },
// inputs
    pub dmcu: *mut dmcu,
    pub wait: c_uint,
    pub update_psr_wait_loop_params: },
// inputs
    pub base_efficiency: u8,
    pub low_power_efficiency: u8,
    pub update_stutter_efficiency_params: },
    pub utm_urgent_bandwidth_lb_KBps: c_uint,
    pub utm_nominal_bandwidth_lb_KBps: c_uint,
    pub utm_lsdma_bandwidth_lb_KBps: c_uint,
    pub utm_latency_ub_index: c_uint,
    pub update_utm_qos_request_params: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn60_clk_mgr_block_sequence_func {
    CLK_MGR60_READ_CLOCKS_FROM_DENTIST,
    CLK_MGR60_UPDATE_HARDMIN_PPCLK,
    CLK_MGR60_UPDATE_HARDMIN_PPCLK_OPTIMIZED,
    CLK_MGR60_UPDATE_DEEP_SLEEP_DCFCLK,
    CLK_MGR60_INDICATE_PSTATE_STATUS,
    CLK_MGR60_UPDATE_DPPCLK_DTO,
    CLK_MGR60_UPDATE_DTBCLK_DTO,
    CLK_MGR60_UPDATE_DENTIST,
    CLK_MGR60_UPDATE_PSR_WAIT_LOOP,
    CLK_MGR60_UPDATE_STUTTER_EFFICIENCY,
    CLK_MGR60_UPDATE_UTM_QOS_REQUEST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence {
    pub params: dcn60_clk_mgr_block_sequence_params,
    pub func: dcn60_clk_mgr_block_sequence_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_update_action {
// clk_mgr state needs updating
    pub update: bool,
// SMU message required
    pub send_message: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_enablement_action {
// feature transitioning to enabled
    pub enable: bool,
// feature transitioning to disabled
    pub disable: bool,
// SMU message required for this transition
    pub send_message: bool,
}

//
// struct dcn60_bandwidth_clocks_update_action - captures what clock and
// p-state changes are needed for a bandwidth update, separating the action
// logic from block sequence construction and state mutation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_bandwidth_clocks_update_action {
    pub dcfclk: dcn60_update_action,
    pub deep_sleep_dcfclk: dcn60_update_action,
    pub socclk: dcn60_update_action,
    pub stutter: dcn60_update_action,
    pub utm_qos: dcn60_update_action,
    pub uclk_pstate: dcn60_enablement_action,
    pub fclk_pstate: dcn60_enablement_action,
    pub fams: dcn60_enablement_action,
    pub alt_ch: dcn60_enablement_action,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr {
    pub base: clk_mgr_internal,
    pub block_sequence: [dcn60_clk_mgr_block_sequence; DCN401_CLK_MGR_MAX_SEQUENCE_SIZE],
    pub utm_qos_model: utm_qos_model,
    pub dchub_v3: utm_qos_model_dchub_v3,

    pub num_block_sequence_steps: c_uint,
}

extern "C" {
    pub fn dcn60_init_clocks(clk_mgr_base: *mut clk_mgr);
}
extern "C" {
    pub fn dcn60_clk_mgr_destroy(clk_mgr: *mut clk_mgr_internal);
}
