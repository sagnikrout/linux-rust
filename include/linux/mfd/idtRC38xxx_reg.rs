//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/idtRC38xxx_reg.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Register Map - Based on PolarBear_CSRs.RevA.xlsx (2023-04-21)
//
// Copyright (C) 2023 Integrated Device Technology, Inc., a Renesas Company.
//

// Macro flag: #define MFD_IDTRC38XXX_REG
// GLOBAL

// FOD

// TDCAPLL

// TIME SYNC CHANNEL

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpf_mode {
    LPF_DISABLED = 0,
    LPF_WP       = 1,
    LPF_HOLDOVER = 2,
    LPF_WF       = 3,
    LPF_INVALID  = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdc_mux_sel {
    REF0 = 0,
    REF1 = 1,
    REF2 = 2,
    REF3 = 3,
    REF_CLK5 = 4,
    REF_CLK6 = 5,
    DPLL_FB_TO_TDC = 6,
    DPLL_FB_DIVIDED_TO_TDC = 7,
    TIME_CLK_DIVIDED = 8,
    TIME_SYNC = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdc_meas_mode {
    CONTINUOUS = 0,
    ONE_SHOT = 1,
    MEAS_MODE_INVALID = 2,
}

// DPLL

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpll_state {
    DPLL_STATE_MIN             = 0,
    DPLL_STATE_FREERUN         = DPLL_STATE_MIN,
    DPLL_STATE_LOCKED          = 1,
    DPLL_STATE_HOLDOVER        = 2,
    DPLL_STATE_WRITE_FREQUENCY = 3,
    DPLL_STATE_ACQUIRE         = 4,
    DPLL_STATE_HITLESS_SWITCH  = 5,
    DPLL_STATE_MAX             = DPLL_STATE_HITLESS_SWITCH
}

// REFMON

// Firmware interface

//
// Return register address and field mask based on passed in firmware version
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_version {
    V_DEFAULT = 0,
    VFC3W     = 1,
    VFC3A     = 2
}

// XTAL_FREQ_ADDR/TIME_CLK_FREQ_ADDR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idtfc3_hw_param {
    pub xtal_freq: u32,
    pub time_clk_freq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idtfc3_fwrc {
    pub hiaddr: u8,
    pub loaddr: u8,
    pub value: u8,
    pub reserved: u8,
    pub __packed: },
    pub 49152000: hw_param->xtal_freq =,
    pub 25000000: hw_param->time_clk_freq =,
    pub 49152000: hw_param->xtal_freq =,
    pub 50000000: hw_param->xtal_freq =,
    pub -EINVAL: return,
    pub 25000000: hw_param->time_clk_freq =,
    pub 50000000: hw_param->time_clk_freq =,
    pub 100000000: hw_param->time_clk_freq =,
    pub 125000000: hw_param->time_clk_freq =,
    pub 250000000: hw_param->time_clk_freq =,
    pub -EINVAL: return,
    pub -EFAULT: return,
    pub 0: return,
