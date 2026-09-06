//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_socbb.h
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


//
// Copyright 2019 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_info_voltage_scaling_v1_0 {
    pub state: u32,
    pub dscclk_mhz: u32,
    pub dcfclk_mhz: u32,
    pub socclk_mhz: u32,
    pub dram_speed_mts: u32,
    pub fabricclk_mhz: u32,
    pub dispclk_mhz: u32,
    pub phyclk_mhz: u32,
    pub dppclk_mhz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_info_soc_bounding_box_v1_0 {
    pub sr_exit_time_us: u32,
    pub sr_enter_plus_exit_time_us: u32,
    pub urgent_latency_us: u32,
    pub urgent_latency_pixel_data_only_us: u32,
    pub urgent_latency_pixel_mixed_with_vm_data_us: u32,
    pub urgent_latency_vm_data_only_us: u32,
    pub writeback_latency_us: u32,
    pub ideal_dram_bw_after_urgent_percent: u32,
    pub PercentOfIdealDRAMFabricAndSDPPortBWReceivedAfterUrgLatencyPixelDataOnly: uint32_t pct_ideal_dram_sdp_bw_after_urgent_pixel_only; //,
    pub pct_ideal_dram_sdp_bw_after_urgent_pixel_and_vm: u32,
    pub pct_ideal_dram_sdp_bw_after_urgent_vm_only: u32,
    pub max_avg_sdp_bw_use_normal_percent: u32,
    pub max_avg_dram_bw_use_normal_percent: u32,
    pub max_request_size_bytes: u32,
    pub downspread_percent: u32,
    pub dram_page_open_time_ns: u32,
    pub dram_rw_turnaround_time_ns: u32,
    pub dram_return_buffer_per_channel_bytes: u32,
    pub dram_channel_width_bytes: u32,
    pub fabric_datapath_to_dcn_data_return_bytes: u32,
    pub dcn_downspread_percent: u32,
    pub dispclk_dppclk_vco_speed_mhz: u32,
    pub dfs_vco_period_ps: u32,
    pub urgent_out_of_order_return_per_channel_pixel_only_bytes: u32,
    pub urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: u32,
    pub urgent_out_of_order_return_per_channel_vm_only_bytes: u32,
    pub round_trip_ping_latency_dcfclk_cycles: u32,
    pub urgent_out_of_order_return_per_channel_bytes: u32,
    pub channel_interleave_bytes: u32,
    pub num_banks: u32,
    pub num_chans: u32,
    pub vmm_page_size_bytes: u32,
    pub dram_clock_change_latency_us: u32,
    pub writeback_dram_clock_change_latency_us: u32,
    pub return_bus_width_bytes: u32,
    pub voltage_override: u32,
    pub xfc_bus_transport_time_us: u32,
    pub xfc_xbuf_latency_tolerance_us: u32,
    pub use_urgent_burst_bw: u32,
    pub num_states: u32,
    pub clock_limits: [gpu_info_voltage_scaling_v1_0; 8],
}
