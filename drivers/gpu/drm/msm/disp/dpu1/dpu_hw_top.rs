//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_top.h
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
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
//

//
// struct traffic_shaper_cfg: traffic shaper configuration
// @en        : enable/disable traffic shaper
// @rd_client : true if read client; false if write client
// @client_id : client identifier
// @bpc_denom : denominator of byte per clk
// @bpc_numer : numerator of byte per clk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct traffic_shaper_cfg {
    pub en: bool,
    pub rd_client: bool,
    pub client_id: u32,
    pub bpc_denom: u32,
    pub bpc_numer: u64,
}

//
// struct split_pipe_cfg - pipe configuration for dual display panels
// @en        : Enable/disable dual pipe configuration
// @mode      : Panel interface mode
// @intf      : Interface id for main control path
// @split_flush_en: Allows both the paths to be flushed when master path is
// flushed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct split_pipe_cfg {
    pub en: bool,
    pub mode: dpu_intf_mode,
    pub intf: dpu_intf,
    pub split_flush_en: bool,
}

//
// struct dpu_danger_safe_status: danger and safe status signals
// @mdp: top level status
// @sspp: source pipe status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_danger_safe_status {
    pub mdp: u8,
    pub sspp: [u8; SSPP_MAX],
}

//
// struct dpu_vsync_source_cfg - configure vsync source and configure the
// watchdog timers if required.
// @pp_count: number of ping pongs active
// @frame_rate: Display frame rate
// @ppnumber: ping pong index array
// @vsync_source: vsync source selection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_vsync_source_cfg {
    pub pp_count: u32,
    pub frame_rate: u32,
    pub ppnumber: [u32; PINGPONG_MAX],
    pub vsync_source: dpu_vsync_source,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_dp_phy_sel {
    DPU_DP_PHY_NONE,
    DPU_DP_PHY_0,
    DPU_DP_PHY_1,
    DPU_DP_PHY_2,
}

//
// struct dpu_hw_mdp_ops - interface to the MDP TOP Hw driver functions
// Assumption is these functions will be called after clocks are enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_mdp_ops {
//
// @setup_split_pipe : Programs the pipe control registers.
// Registers are not double buffered, this
// function should be called before timing control enable
// @mdp  : mdp top context driver
// @cfg  : upper and lower part of pipe configuration
//
    pub p): *mut split_pipe_cfg,
//
// @setup_traffic_shaper : programs traffic shaper control.
// @mdp  : mdp top context driver
// @cfg  : traffic shaper configuration
//
    pub cfg): *mut traffic_shaper_cfg,
//
// @setup_clk_force_ctrl: set clock force control
// @mdp: mdp top context driver
// @clk_ctrl: clock to be controlled
// @enable: force on enable
// @return: if the clock is forced-on by this function
//
    pub enable): dpu_clk_ctrl_type clk_ctrl, bool,
//
// @get_danger_status: get danger status
// @mdp: mdp top context driver
// @status: Pointer to danger safe status
//
    pub status): *mut dpu_danger_safe_status,
//
// @setup_vsync_source: setup vsync source configuration details
// @mdp: mdp top context driver
// @cfg: vsync source selection configuration
//
    pub cfg): *mut dpu_vsync_source_cfg,
//
// @get_safe_status: get safe status
// @mdp: mdp top context driver
// @status: Pointer to danger safe status
//
    pub status): *mut dpu_danger_safe_status,
//
// @dp_phy_intf_sel: configure intf to phy mapping
// @mdp: mdp top context driver
// @phys: list of phys the DP interfaces should be connected to. 0 disables the INTF.
//
    pub phys[2]): *mut *mut *mut void (dp_phy_intf_sel)(struct dpu_hw_mdp mdp, enum dpu_dp_phy_sel,
//
// @intf_audio_select: select the external interface for audio
// @mdp: mdp top context driver
//
    pub mdp): *mut *mut void (intf_audio_select)(struct dpu_hw_mdp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_mdp {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
// top
    pub caps: *const dpu_mdp_cfg,
// ops
    pub ops: dpu_hw_mdp_ops,
}
