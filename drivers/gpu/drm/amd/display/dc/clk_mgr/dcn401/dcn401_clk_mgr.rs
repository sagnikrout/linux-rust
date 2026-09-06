//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn401/dcn401_clk_mgr.h
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
// Copyright 2024 Advanced Micro Devices, Inc.
pub const DCN401_CLK_MGR_MAX_SEQUENCE_SIZE: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub union dcn401_clk_mgr_block_sequence_params {
// inputs
    pub num_displays: u32,
    pub update_num_displays_params: },
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
    pub uclk_mhz: u16,
    pub fclk_mhz: u16,
    pub update_idle_hardmin_params: },
// inputs
    pub freq_mhz: u16,
    pub update_deep_sleep_dcfclk_params: },
// inputs
    pub support: bool,
    pub update_pstate_support_params: },
// inputs
    pub num_ways: c_uint,
    pub update_cab_for_uclk_params: },
// inputs
    pub enable: bool,
    pub update_wait_for_dmub_ack_params: },
// inputs
    pub mod_drr_for_pstate: bool,
    pub indicate_drr_status_params: },
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
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn401_clk_mgr_block_sequence_func {
    CLK_MGR401_READ_CLOCKS_FROM_DENTIST,
    CLK_MGR401_UPDATE_NUM_DISPLAYS,
    CLK_MGR401_UPDATE_HARDMIN_PPCLK,
    CLK_MGR401_UPDATE_HARDMIN_PPCLK_OPTIMIZED,
    CLK_MGR401_UPDATE_ACTIVE_HARDMINS,
    CLK_MGR401_UPDATE_IDLE_HARDMINS,
    CLK_MGR401_UPDATE_DEEP_SLEEP_DCFCLK,
    CLK_MGR401_UPDATE_FCLK_PSTATE_SUPPORT,
    CLK_MGR401_UPDATE_UCLK_PSTATE_SUPPORT,
    CLK_MGR401_UPDATE_CAB_FOR_UCLK,
    CLK_MGR401_UPDATE_WAIT_FOR_DMUB_ACK,
    CLK_MGR401_INDICATE_DRR_STATUS,
    CLK_MGR401_UPDATE_DPPCLK_DTO,
    CLK_MGR401_UPDATE_DTBCLK_DTO,
    CLK_MGR401_UPDATE_DENTIST,
    CLK_MGR401_UPDATE_PSR_WAIT_LOOP,
    CLK_MGR401_UPDATE_SUBVP_HARDMINS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_clk_mgr_block_sequence {
    pub params: dcn401_clk_mgr_block_sequence_params,
    pub func: dcn401_clk_mgr_block_sequence_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_clk_mgr {
    pub base: clk_mgr_internal,
    pub block_sequence: [dcn401_clk_mgr_block_sequence; DCN401_CLK_MGR_MAX_SEQUENCE_SIZE],
    pub num_block_sequence_steps: c_uint,
}

extern "C" {
    pub fn dcn401_init_clocks(clk_mgr_base: *mut clk_mgr);
}
extern "C" {
    pub fn dcn401_is_dc_mode_present(clk_mgr_base: *mut clk_mgr) -> bool;
}
extern "C" {
    pub fn dcn401_clk_mgr_destroy(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn401_get_max_clock_khz(clk_mgr_base: *mut clk_mgr, clk_type: clk_type) -> c_uint;
}
