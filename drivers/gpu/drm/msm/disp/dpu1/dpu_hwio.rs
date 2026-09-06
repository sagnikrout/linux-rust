//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hwio.h
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
// MDP TOP block Register and bit fields and defines
//
pub const DISP_INTF_SEL: c_uint = 0x004;
pub const INTR_EN: c_uint = 0x010;
pub const INTR_STATUS: c_uint = 0x014;
pub const INTR_CLEAR: c_uint = 0x018;
pub const INTR2_EN: c_uint = 0x008;
pub const INTR2_STATUS: c_uint = 0x00c;
pub const SSPP_SPARE: c_uint = 0x028;
pub const INTR2_CLEAR: c_uint = 0x02c;
pub const HIST_INTR_EN: c_uint = 0x01c;
pub const HIST_INTR_STATUS: c_uint = 0x020;
pub const HIST_INTR_CLEAR: c_uint = 0x024;
pub const SPLIT_DISPLAY_EN: c_uint = 0x2F4;
pub const SPLIT_DISPLAY_UPPER_PIPE_CTRL: c_uint = 0x2F8;
pub const DSPP_IGC_COLOR0_RAM_LUTN: c_uint = 0x300;
pub const DSPP_IGC_COLOR1_RAM_LUTN: c_uint = 0x304;
pub const DSPP_IGC_COLOR2_RAM_LUTN: c_uint = 0x308;
pub const DANGER_STATUS: c_uint = 0x360;
pub const SAFE_STATUS: c_uint = 0x364;
pub const HW_EVENTS_CTL: c_uint = 0x37C;
pub const MDP_WD_TIMER_0_CTL: c_uint = 0x380;
pub const MDP_WD_TIMER_0_CTL2: c_uint = 0x384;
pub const MDP_WD_TIMER_0_LOAD_VALUE: c_uint = 0x388;
pub const MDP_WD_TIMER_1_CTL: c_uint = 0x390;
pub const MDP_WD_TIMER_1_CTL2: c_uint = 0x394;
pub const MDP_WD_TIMER_1_LOAD_VALUE: c_uint = 0x398;
pub const CLK_CTRL3: c_uint = 0x3A8;
pub const CLK_STATUS3: c_uint = 0x3AC;
pub const CLK_CTRL4: c_uint = 0x3B0;
pub const CLK_STATUS4: c_uint = 0x3B4;
pub const CLK_CTRL5: c_uint = 0x3B8;
pub const CLK_STATUS5: c_uint = 0x3BC;
pub const CLK_CTRL7: c_uint = 0x3D0;
pub const CLK_STATUS7: c_uint = 0x3D4;
pub const SPLIT_DISPLAY_LOWER_PIPE_CTRL: c_uint = 0x3F0;
pub const SPLIT_DISPLAY_TE_LINE_INTERVAL: c_uint = 0x3F4;
pub const INTF_SW_RESET_MASK: c_uint = 0x3FC;
pub const HDMI_DP_CORE_SELECT: c_uint = 0x408;
pub const MDP_OUT_CTL_0: c_uint = 0x410;
pub const MDP_VSYNC_SEL: c_uint = 0x414;
pub const MDP_WD_TIMER_2_CTL: c_uint = 0x420;
pub const MDP_WD_TIMER_2_CTL2: c_uint = 0x424;
pub const MDP_WD_TIMER_2_LOAD_VALUE: c_uint = 0x428;
pub const MDP_WD_TIMER_3_CTL: c_uint = 0x430;
pub const MDP_WD_TIMER_3_CTL2: c_uint = 0x434;
pub const MDP_WD_TIMER_3_LOAD_VALUE: c_uint = 0x438;
pub const MDP_WD_TIMER_4_CTL: c_uint = 0x440;
pub const MDP_WD_TIMER_4_CTL2: c_uint = 0x444;
pub const MDP_WD_TIMER_4_LOAD_VALUE: c_uint = 0x448;
pub const DCE_SEL: c_uint = 0x450;
pub const MDP_DP_PHY_INTF_SEL: c_uint = 0x460;

