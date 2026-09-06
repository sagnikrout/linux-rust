//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/cec/platform/tegra/tegra_cec.h
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
// Tegra CEC register definitions
//
// The original 3.10 CEC driver using a custom API:
//
// Copyright (c) 2012-2015, NVIDIA CORPORATION.  All rights reserved.
//
// Conversion to the CEC framework and to the mainline kernel:
//
// Copyright 2016-2017 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
// CEC registers
pub const TEGRA_CEC_SW_CONTROL: c_uint = 0x000;
pub const TEGRA_CEC_HW_CONTROL: c_uint = 0x004;
pub const TEGRA_CEC_INPUT_FILTER: c_uint = 0x008;
pub const TEGRA_CEC_TX_REGISTER: c_uint = 0x010;
pub const TEGRA_CEC_RX_REGISTER: c_uint = 0x014;
pub const TEGRA_CEC_RX_TIMING_0: c_uint = 0x018;
pub const TEGRA_CEC_RX_TIMING_1: c_uint = 0x01c;
pub const TEGRA_CEC_RX_TIMING_2: c_uint = 0x020;
pub const TEGRA_CEC_TX_TIMING_0: c_uint = 0x024;
pub const TEGRA_CEC_TX_TIMING_1: c_uint = 0x028;
pub const TEGRA_CEC_TX_TIMING_2: c_uint = 0x02c;
pub const TEGRA_CEC_INT_STAT: c_uint = 0x030;
pub const TEGRA_CEC_INT_MASK: c_uint = 0x034;
pub const TEGRA_CEC_HW_DEBUG_RX: c_uint = 0x038;
pub const TEGRA_CEC_HW_DEBUG_TX: c_uint = 0x03c;
pub const TEGRA_CEC_HWCTRL_RX_LADDR_MASK: c_uint = 0x7fff;

pub const TEGRA_CEC_INPUT_FILTER_FIFO_LENGTH_SHIFT: c_int = 0;
pub const TEGRA_CEC_TX_REG_DATA_SHIFT: c_int = 0;

pub const TEGRA_CEC_RX_REGISTER_SHIFT: c_int = 0;

pub const TEGRA_CEC_RX_TIM0_START_BIT_MAX_LO_TIME_SHIFT: c_int = 0;
pub const TEGRA_CEC_RX_TIM0_START_BIT_MIN_LO_TIME_SHIFT: c_int = 8;
pub const TEGRA_CEC_RX_TIM0_START_BIT_MAX_DURATION_SHIFT: c_int = 16;
pub const TEGRA_CEC_RX_TIM0_START_BIT_MIN_DURATION_SHIFT: c_int = 24;
pub const TEGRA_CEC_RX_TIM1_DATA_BIT_MAX_LO_TIME_SHIFT: c_int = 0;
pub const TEGRA_CEC_RX_TIM1_DATA_BIT_SAMPLE_TIME_SHIFT: c_int = 8;
pub const TEGRA_CEC_RX_TIM1_DATA_BIT_MAX_DURATION_SHIFT: c_int = 16;
pub const TEGRA_CEC_RX_TIM1_DATA_BIT_MIN_DURATION_SHIFT: c_int = 24;
pub const TEGRA_CEC_RX_TIM2_END_OF_BLOCK_TIME_SHIFT: c_int = 0;
pub const TEGRA_CEC_TX_TIM0_START_BIT_LO_TIME_SHIFT: c_int = 0;
pub const TEGRA_CEC_TX_TIM0_START_BIT_DURATION_SHIFT: c_int = 8;
pub const TEGRA_CEC_TX_TIM0_BUS_XITION_TIME_SHIFT: c_int = 16;
pub const TEGRA_CEC_TX_TIM0_BUS_ERROR_LO_TIME_SHIFT: c_int = 24;
pub const TEGRA_CEC_TX_TIM1_LO_DATA_BIT_LO_TIME_SHIFT: c_int = 0;
pub const TEGRA_CEC_TX_TIM1_HI_DATA_BIT_LO_TIME_SHIFT: c_int = 8;
pub const TEGRA_CEC_TX_TIM1_DATA_BIT_DURATION_SHIFT: c_int = 16;
pub const TEGRA_CEC_TX_TIM1_ACK_NAK_BIT_SAMPLE_TIME_SHIFT: c_int = 24;
pub const TEGRA_CEC_TX_TIM2_BUS_IDLE_TIME_ADDITIONAL_FRAME_SHIFT: c_int = 0;
pub const TEGRA_CEC_TX_TIM2_BUS_IDLE_TIME_NEW_FRAME_SHIFT: c_int = 4;
pub const TEGRA_CEC_TX_TIM2_BUS_IDLE_TIME_RETRY_FRAME_SHIFT: c_int = 8;

pub const TEGRA_CEC_HW_DEBUG_TX_DURATION_COUNT_SHIFT: c_int = 0;
pub const TEGRA_CEC_HW_DEBUG_TX_TXBIT_COUNT_SHIFT: c_int = 17;
pub const TEGRA_CEC_HW_DEBUG_TX_STATE_SHIFT: c_int = 21;

