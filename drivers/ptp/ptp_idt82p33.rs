//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ptp/ptp_idt82p33.h
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
// PTP hardware clock driver for the IDT 82P33XXX family of clocks.
//
// Copyright (C) 2019 Integrated Device Technology, Inc., a Renesas Company.
//

pub const PLLMASK_ADDR_HI: c_uint = 0xFF;
pub const PLLMASK_ADDR_LO: c_uint = 0xA5;
pub const PLL0_OUTMASK_ADDR_HI: c_uint = 0xFF;
pub const PLL0_OUTMASK_ADDR_LO: c_uint = 0xB0;
pub const PLL1_OUTMASK_ADDR_HI: c_uint = 0xFF;
pub const PLL1_OUTMASK_ADDR_LO: c_uint = 0xB2;
pub const PLL2_OUTMASK_ADDR_HI: c_uint = 0xFF;
pub const PLL2_OUTMASK_ADDR_LO: c_uint = 0xB4;
pub const PLL3_OUTMASK_ADDR_HI: c_uint = 0xFF;
pub const PLL3_OUTMASK_ADDR_LO: c_uint = 0xB6;

//
// @brief Maximum absolute value for write phase offset in nanoseconds
//

// @brief Phase offset resolution
//
// DPLL phase offset = 10^15 fs / ( System Clock  * 2^13)
// = 10^15 fs / ( 1638400000 * 2^23)
// = 74.5058059692382 fs
//
pub const IDT_T0DPLL_PHASE_RESOL: c_int = 74506;
// PTP Hardware Clock interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt82p33_channel {
    pub caps: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
    pub idt82p33: *mut idt82p33,
    pub pll_mode: pll_mode,
// Workaround for TOD-to-output alignment issue
    pub adjtime_work: delayed_work,
    pub current_freq: i32,
// double dco mode
    pub ddco: bool,
    pub output_mask: u8,
// last input trigger for extts
    pub tod_trigger: u8,
    pub discard_next_extts: bool,
    pub plln: u8,
// remember last tod_sts for extts
    pub extts_tod_sts: [u8; TOD_BYTE_COUNT],
    pub dpll_tod_cnfg: u16,
    pub dpll_tod_trigger: u16,
    pub dpll_tod_sts: u16,
    pub dpll_mode_cnfg: u16,
    pub dpll_freq_cnfg: u16,
    pub dpll_phase_cnfg: u16,
    pub dpll_sync_cnfg: u16,
    pub dpll_input_mode_cnfg: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt82p33 {
    pub channel: [idt82p33_channel; MAX_PHC_PLL],
    pub dev: *mut device,
    pub pll_mask: u8,
// Polls for external time stamps
    pub extts_mask: u8,
    pub extts_single_shot: bool,
    pub extts_work: delayed_work,
// Remember the ptp channel to report extts
    pub event_channel: [*mut idt82p33_channel; MAX_PHC_PLL],
// Mutex to protect operations from being interrupted
    pub lock: *mut mutex,
    pub regmap: *mut regmap,
    pub mfd: *mut device,
// Overhead calculation for adjtime
    pub start_time: ktime_t,
    pub calculate_overhead_flag: c_int,
    pub tod_write_overhead_ns: i64,
}

// firmware interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt82p33_fwrc {
    pub hiaddr: u8,
    pub loaddr: u8,
    pub value: u8,
    pub reserved: u8,
    pub __packed: },
