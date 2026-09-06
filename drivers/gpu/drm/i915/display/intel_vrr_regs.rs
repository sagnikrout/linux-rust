//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_vrr_regs.h
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
// Copyright © 2024 Intel Corporation
//

pub const _TRANS_VRR_DCB_ADJ_FLIPLINE_CFG_A: c_uint = 0x604d4;
pub const _TRANS_VRR_DCB_ADJ_FLIPLINE_CFG_B: c_uint = 0x614d4;

pub const _TRANS_VRR_DCB_ADJ_FLIPLINE_CFG_LIVE_A: c_uint = 0x90700;
pub const _TRANS_VRR_DCB_ADJ_FLIPLINE_CFG_LIVE_B: c_uint = 0x98700;

pub const _TRANS_VRR_DCB_ADJ_VMAX_CFG_A: c_uint = 0x604d8;
pub const _TRANS_VRR_DCB_ADJ_VMAX_CFG_B: c_uint = 0x614d8;

pub const _TRANS_VRR_DCB_ADJ_VMAX_CFG_LIVE_A: c_uint = 0x906f8;
pub const _TRANS_VRR_DCB_ADJ_VMAX_CFG_LIVE_B: c_uint = 0x986f8;

pub const _TRANS_VRR_DCB_FLIPLINE_A: c_uint = 0x60418;
pub const _TRANS_VRR_DCB_FLIPLINE_B: c_uint = 0x61418;

pub const _TRANS_VRR_DCB_FLIPLINE_LIVE_A: c_uint = 0x906fc;
pub const _TRANS_VRR_DCB_FLIPLINE_LIVE_B: c_uint = 0x986fc;

pub const _TRANS_VRR_DCB_VMAX_A: c_uint = 0x60414;
pub const _TRANS_VRR_DCB_VMAX_B: c_uint = 0x61414;

pub const _TRANS_VRR_DCB_VMAX_LIVE_A: c_uint = 0x906f4;
pub const _TRANS_VRR_DCB_VMAX_LIVE_B: c_uint = 0x986f4;

pub const _TRANS_ADAPTIVE_SYNC_DCB_CTL_A: c_uint = 0x604c0;
pub const _TRANS_ADAPTIVE_SYNC_DCB_CTL_B: c_uint = 0x614c0;

pub const _TRANS_VRR_CTL_A: c_uint = 0x60420;
pub const _TRANS_VRR_CTL_B: c_uint = 0x61420;
pub const _TRANS_VRR_CTL_C: c_uint = 0x62420;
pub const _TRANS_VRR_CTL_D: c_uint = 0x63420;

pub const _TRANS_VRR_VMAX_A: c_uint = 0x60424;
pub const _TRANS_VRR_VMAX_B: c_uint = 0x61424;
pub const _TRANS_VRR_VMAX_C: c_uint = 0x62424;
pub const _TRANS_VRR_VMAX_D: c_uint = 0x63424;

pub const _TRANS_VRR_VMIN_A: c_uint = 0x60434;
pub const _TRANS_VRR_VMIN_B: c_uint = 0x61434;
pub const _TRANS_VRR_VMIN_C: c_uint = 0x62434;
pub const _TRANS_VRR_VMIN_D: c_uint = 0x63434;

pub const _TRANS_VRR_VMAXSHIFT_A: c_uint = 0x60428;
pub const _TRANS_VRR_VMAXSHIFT_B: c_uint = 0x61428;
pub const _TRANS_VRR_VMAXSHIFT_C: c_uint = 0x62428;
pub const _TRANS_VRR_VMAXSHIFT_D: c_uint = 0x63428;

pub const _TRANS_VRR_STATUS_A: c_uint = 0x6042c;
pub const _TRANS_VRR_STATUS_B: c_uint = 0x6142c;
pub const _TRANS_VRR_STATUS_C: c_uint = 0x6242c;
pub const _TRANS_VRR_STATUS_D: c_uint = 0x6342c;

pub const _TRANS_VRR_VTOTAL_PREV_A: c_uint = 0x60480;
pub const _TRANS_VRR_VTOTAL_PREV_B: c_uint = 0x61480;
pub const _TRANS_VRR_VTOTAL_PREV_C: c_uint = 0x62480;
pub const _TRANS_VRR_VTOTAL_PREV_D: c_uint = 0x63480;

pub const _TRANS_VRR_FLIPLINE_A: c_uint = 0x60438;
pub const _TRANS_VRR_FLIPLINE_B: c_uint = 0x61438;
pub const _TRANS_VRR_FLIPLINE_C: c_uint = 0x62438;
pub const _TRANS_VRR_FLIPLINE_D: c_uint = 0x63438;

pub const _TRANS_VRR_STATUS2_A: c_uint = 0x6043c;
pub const _TRANS_VRR_STATUS2_B: c_uint = 0x6143c;
pub const _TRANS_VRR_STATUS2_C: c_uint = 0x6243c;
pub const _TRANS_VRR_STATUS2_D: c_uint = 0x6343c;

pub const _TRANS_PUSH_A: c_uint = 0x60a70;
pub const _TRANS_PUSH_B: c_uint = 0x61a70;
pub const _TRANS_PUSH_C: c_uint = 0x62a70;
pub const _TRANS_PUSH_D: c_uint = 0x63a70;

pub const _TRANS_VRR_VSYNC_A: c_uint = 0x60078;

// Common register for HDMI EMP and DP AS SDP
pub const _EMP_AS_SDP_TL_A: c_uint = 0x60204;

pub const _TRANS_CMRR_M_LO_A: c_uint = 0x604F0;

pub const _TRANS_CMRR_M_HI_A: c_uint = 0x604F4;

pub const _TRANS_CMRR_N_LO_A: c_uint = 0x604F8;

pub const _TRANS_CMRR_N_HI_A: c_uint = 0x604FC;

