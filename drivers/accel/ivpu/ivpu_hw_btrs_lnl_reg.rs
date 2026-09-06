//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_hw_btrs_lnl_reg.h
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
// Copyright (C) 2020-2024 Intel Corporation
//

pub const VPU_HW_BTRS_LNL_INTERRUPT_STAT: c_uint = 0x00000000u;

pub const VPU_HW_BTRS_LNL_LOCAL_INT_MASK: c_uint = 0x00000004u;
pub const VPU_HW_BTRS_LNL_GLOBAL_INT_MASK: c_uint = 0x00000008u;
pub const VPU_HW_BTRS_LNL_HM_ATS: c_uint = 0x0000000cu;
pub const VPU_HW_BTRS_LNL_ATS_ERR_LOG1: c_uint = 0x00000010u;
pub const VPU_HW_BTRS_LNL_ATS_ERR_LOG2: c_uint = 0x00000014u;
pub const VPU_HW_BTRS_LNL_ATS_ERR_CLEAR: c_uint = 0x00000018u;
pub const VPU_HW_BTRS_LNL_CFI0_ERR_LOG: c_uint = 0x0000001cu;
pub const VPU_HW_BTRS_LNL_CFI0_ERR_CLEAR: c_uint = 0x00000020u;
pub const VPU_HW_BTRS_LNL_PORT_ARBITRATION_WEIGHTS_ATS: c_uint = 0x00000024u;
pub const VPU_HW_BTRS_LNL_CFI1_ERR_LOG: c_uint = 0x00000040u;
pub const VPU_HW_BTRS_LNL_CFI1_ERR_CLEAR: c_uint = 0x00000044u;
pub const VPU_HW_BTRS_LNL_IMR_ERR_CFI0_LOW: c_uint = 0x00000048u;
pub const VPU_HW_BTRS_LNL_IMR_ERR_CFI0_HIGH: c_uint = 0x0000004cu;
pub const VPU_HW_BTRS_LNL_IMR_ERR_CFI0_CLEAR: c_uint = 0x00000050u;
pub const VPU_HW_BTRS_LNL_PORT_ARBITRATION_WEIGHTS: c_uint = 0x00000054u;
pub const VPU_HW_BTRS_LNL_IMR_ERR_CFI1_LOW: c_uint = 0x00000058u;
pub const VPU_HW_BTRS_LNL_IMR_ERR_CFI1_HIGH: c_uint = 0x0000005cu;
pub const VPU_HW_BTRS_LNL_IMR_ERR_CFI1_CLEAR: c_uint = 0x00000060u;
pub const VPU_HW_BTRS_LNL_PCODE_MAILBOX_STATUS: c_uint = 0x00000070u;

pub const VPU_HW_BTRS_LNL_PCODE_MAILBOX_SHADOW: c_uint = 0x00000074u;

pub const VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD0: c_uint = 0x00000130u;

pub const VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD1: c_uint = 0x00000134u;

pub const VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD2: c_uint = 0x00000138u;

pub const VPU_HW_BTRS_LNL_WP_REQ_CMD: c_uint = 0x0000013cu;

pub const VPU_HW_BTRS_LNL_PLL_FREQ: c_uint = 0x00000148u;

pub const VPU_HW_BTRS_LNL_CDYN: c_uint = 0x0000014cu;

pub const VPU_HW_BTRS_LNL_TILE_FUSE: c_uint = 0x00000150u;

pub const VPU_HW_BTRS_LNL_VPU_STATUS: c_uint = 0x00000154u;

pub const VPU_HW_BTRS_LNL_IP_RESET: c_uint = 0x00000160u;

pub const VPU_HW_BTRS_LNL_D0I3_CONTROL: c_uint = 0x00000164u;

pub const VPU_HW_BTRS_LNL_VPU_TELEMETRY_OFFSET: c_uint = 0x00000168u;
pub const VPU_HW_BTRS_LNL_VPU_TELEMETRY_SIZE: c_uint = 0x0000016cu;
pub const VPU_HW_BTRS_LNL_VPU_TELEMETRY_ENABLE: c_uint = 0x00000170u;
pub const VPU_HW_BTRS_LNL_FMIN_FUSE: c_uint = 0x00000174u;

pub const VPU_HW_BTRS_LNL_FMAX_FUSE: c_uint = 0x00000178u;

