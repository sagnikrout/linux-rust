//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_intf.h
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
//
// Copyright (c) 2022-2023 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
//

// intf timing settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_intf_timing_params {
    pub /: *mut *mut u32 width; / active width,
    pub /: *mut *mut u32 height; / active height,
    pub /: *mut *mut u32 xres; / Display panel width,
    pub /: *mut *mut u32 yres; / Display panel height,
    pub h_back_porch: u32,
    pub h_front_porch: u32,
    pub v_back_porch: u32,
    pub v_front_porch: u32,
    pub hsync_pulse_width: u32,
    pub vsync_pulse_width: u32,
    pub hsync_polarity: u32,
    pub vsync_polarity: u32,
    pub border_clr: u32,
    pub underflow_clr: u32,
    pub hsync_skew: u32,
    pub wide_bus_en: bool,
    pub compression_en: bool,
    pub dce_bytes_per_line: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_intf_prog_fetch {
    pub enable: u8,
// vsync counter for the front porch pixel line
    pub fetch_start: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_intf_status {
    pub /: *mut *mut u8 is_en; / interface timing engine is enabled or not,
    pub /: *mut *mut u8 is_prog_fetch_en; / interface prog fetch counter is enabled or not,
    pub /: *mut *mut u32 frame_count; / frame count since timing engine enabled,
    pub /: *mut *mut u32 line_count; / current line count including blanking,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_intf_cmd_mode_cfg {
    pub /: *mut *mut u8 data_compress; / enable data compress between dpu and dsi,
    pub /: *mut *mut u8 wide_bus_en; / enable databus widen mode,
}

//
// struct dpu_hw_intf_ops : Interface to the interface Hw driver functions
// Assumption is these functions will be called after clocks are enabled
// @setup_timing_gen : programs the timing engine
// @setup_prg_fetch  : enables/disables the programmable fetch logic
// @enable_timing: enable/disable timing engine
// @get_status: returns if timing engine is enabled or not
// @get_line_count: reads current vertical line counter
// @bind_pingpong_blk: enable/disable the connection with pingpong which will
// feed pixels to this interface
// @setup_misr: enable/disable MISR
// @collect_misr: read MISR signature
// @enable_tearcheck:           Enables vsync generation and sets up init value of read
// pointer and programs the tear check configuration
// @disable_tearcheck:          Disables tearcheck block
// @connect_external_te:        Read, modify, write to either set or clear listening to external TE
// Returns 1 if TE was originally connected, 0 if not, or -ERROR
// @vsync_sel:                  Select vsync signal for tear-effect configuration
// @disable_autorefresh:        Disable autorefresh if enabled
// @program_intf_cmd_cfg:       Program the DPU to interface datapath for command mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_intf_ops {
    pub fmt): *const msm_format,
    pub fetch): *const dpu_hw_intf_prog_fetch,
    pub enable): u8,
    pub status): *mut dpu_hw_intf_status,
    pub intf): *mut *mut u32 (get_line_count)(struct dpu_hw_intf,
    pub pp): dpu_pingpong,
    pub intf): *mut *mut void (setup_misr)(struct dpu_hw_intf,
    pub misr_value): *mut *mut *mut int (collect_misr)(struct dpu_hw_intf intf, u32,
// Tearcheck on INTF since DPU 5.0.0
    pub cfg): *mut *mut *mut int (enable_tearcheck)(struct dpu_hw_intf intf, struct dpu_hw_tear_check,
    pub intf): *mut *mut int (disable_tearcheck)(struct dpu_hw_intf,
    pub enable_external_te): *mut *mut *mut int (connect_external_te)(struct dpu_hw_intf intf, bool,
    pub cfg): *mut *mut *mut void (vsync_sel)(struct dpu_hw_intf intf, struct dpu_vsync_source_cfg,
    pub vdisplay): *mut *mut *mut void (disable_autorefresh)(struct dpu_hw_intf intf, uint32_t encoder_id, u16,
    pub cmd_mode_cfg): *mut dpu_hw_intf_cmd_mode_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_intf {
    pub hw: dpu_hw_blk_reg_map,
// intf
    pub idx: dpu_intf,
    pub cap: *const dpu_intf_cfg,
    pub mdss_ver: *const dpu_mdss_version,
// ops
    pub ops: dpu_hw_intf_ops,
}
