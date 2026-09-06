//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ptp/ptp_clockmatrix.h
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
// PTP hardware clock driver for the IDT ClockMatrix(TM) family of timing and
// synchronization devices.
//
// Copyright (C) 2019 Integrated Device Technology, Inc., a Renesas Company.
//

//
// Return register address based on passed in firmware version
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_version {
    V_DEFAULT = 0,
    V487 = 1,
    V520 = 2,
}

// PTP PLL Mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ptp_pll_mode {
    PTP_PLL_MODE_MIN = 0,
    PTP_PLL_MODE_WRITE_FREQUENCY = PTP_PLL_MODE_MIN,
    PTP_PLL_MODE_WRITE_PHASE,
    PTP_PLL_MODE_UNSUPPORTED,
    PTP_PLL_MODE_MAX = PTP_PLL_MODE_UNSUPPORTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idtcm_channel {
    pub caps: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
    pub idtcm: *mut idtcm,
    pub dpll_phase: u16,
    pub dpll_freq: u16,
    pub dpll_n: u16,
    pub dpll_ctrl_n: u16,
    pub dpll_phase_pull_in: u16,
    pub tod_read_primary: u16,
    pub tod_read_secondary: u16,
    pub tod_write: u16,
    pub tod_n: u16,
    pub hw_dpll_n: u16,
    pub sync_src: u8,
    pub mode: ptp_pll_mode,
    pub channel): *mut *mut int (configure_write_frequency)(struct idtcm_channel,
    pub channel): *mut *mut int (configure_write_phase)(struct idtcm_channel,
    pub max_ffo_ppb): s32 offset_ns, u32,
    pub current_freq_scaled_ppm: i32,
    pub phase_pull_in: bool,
    pub dco_delay: u32,
// last input trigger for extts
    pub refn: u8,
    pub pll: u8,
    pub tod: u8,
    pub output_mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idtcm {
    pub channel: [idtcm_channel; MAX_TOD],
    pub dev: *mut device,
    pub tod_mask: u8,
    pub version: [c_char; 16],
    pub fw_ver: fw_version,
// Polls for external time stamps
    pub extts_mask: u8,
    pub extts_single_shot: bool,
    pub extts_work: delayed_work,
// Remember the ptp channel to report extts
    pub event_channel: [*mut idtcm_channel; MAX_TOD],
// Mutex to protect operations from being interrupted
    pub lock: *mut mutex,
    pub mfd: *mut device,
    pub regmap: *mut regmap,
// Overhead calculation for adjtime
    pub calculate_overhead_flag: u8,
    pub tod_write_overhead_ns: i64,
    pub start_time: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idtcm_fwrc {
    pub hiaddr: u8,
    pub loaddr: u8,
    pub value: u8,
    pub reserved: u8,
    pub __packed: },
