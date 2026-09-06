//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_hw_btrs_mtl_reg.h
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
// Copyright (C) 2020-2023 Intel Corporation
//

pub const VPU_HW_BTRS_MTL_INTERRUPT_TYPE: c_uint = 0x00000000u;
pub const VPU_HW_BTRS_MTL_INTERRUPT_STAT: c_uint = 0x00000004u;

pub const VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD0: c_uint = 0x00000008u;

pub const VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD1: c_uint = 0x0000000cu;

pub const VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD2: c_uint = 0x00000010u;

pub const VPU_HW_BTRS_MTL_WP_REQ_CMD: c_uint = 0x00000014u;

pub const VPU_HW_BTRS_MTL_WP_DOWNLOAD: c_uint = 0x00000018u;

pub const VPU_HW_BTRS_MTL_CURRENT_PLL: c_uint = 0x0000001cu;

pub const VPU_HW_BTRS_MTL_PLL_ENABLE: c_uint = 0x00000020u;
pub const VPU_HW_BTRS_MTL_FMIN_FUSE: c_uint = 0x00000024u;

pub const VPU_HW_BTRS_MTL_FMAX_FUSE: c_uint = 0x00000028u;

pub const VPU_HW_BTRS_MTL_TILE_FUSE: c_uint = 0x0000002cu;

pub const VPU_HW_BTRS_MTL_LOCAL_INT_MASK: c_uint = 0x00000030u;
pub const VPU_HW_BTRS_MTL_GLOBAL_INT_MASK: c_uint = 0x00000034u;
pub const VPU_HW_BTRS_MTL_PLL_STATUS: c_uint = 0x00000040u;

pub const VPU_HW_BTRS_MTL_VPU_STATUS: c_uint = 0x00000044u;

pub const VPU_HW_BTRS_MTL_VPU_D0I3_CONTROL: c_uint = 0x00000060u;

pub const VPU_HW_BTRS_MTL_VPU_IP_RESET: c_uint = 0x00000050u;

pub const VPU_HW_BTRS_MTL_VPU_TELEMETRY_OFFSET: c_uint = 0x00000080u;
pub const VPU_HW_BTRS_MTL_VPU_TELEMETRY_SIZE: c_uint = 0x00000084u;
pub const VPU_HW_BTRS_MTL_VPU_TELEMETRY_ENABLE: c_uint = 0x00000088u;
pub const VPU_HW_BTRS_MTL_ATS_ERR_LOG_0: c_uint = 0x000000a0u;
pub const VPU_HW_BTRS_MTL_ATS_ERR_LOG_1: c_uint = 0x000000a4u;
pub const VPU_HW_BTRS_MTL_ATS_ERR_CLEAR: c_uint = 0x000000a8u;
pub const VPU_HW_BTRS_MTL_UFI_ERR_LOG: c_uint = 0x000000b0u;

pub const VPU_HW_BTRS_MTL_UFI_ERR_CLEAR: c_uint = 0x000000b4u;
