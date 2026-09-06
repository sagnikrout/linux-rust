//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/idt82p33_reg.h
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
// Register Map - Based on AN888_SMUforIEEE_SynchEther_82P33xxx_RevH.pdf
//
// Copyright (C) 2021 Integrated Device Technology, Inc., a Renesas Company.
//

// Macro flag: #define HAVE_IDT82P33_REG

// Register address
pub const DPLL1_TOD_CNFG: c_uint = 0x134;
pub const DPLL2_TOD_CNFG: c_uint = 0x1B4;
pub const DPLL1_TOD_STS: c_uint = 0x10B;
pub const DPLL2_TOD_STS: c_uint = 0x18B;
pub const DPLL1_TOD_TRIGGER: c_uint = 0x115;
pub const DPLL2_TOD_TRIGGER: c_uint = 0x195;
pub const DPLL1_OPERATING_MODE_CNFG: c_uint = 0x120;
pub const DPLL2_OPERATING_MODE_CNFG: c_uint = 0x1A0;
pub const DPLL1_HOLDOVER_FREQ_CNFG: c_uint = 0x12C;
pub const DPLL2_HOLDOVER_FREQ_CNFG: c_uint = 0x1AC;
pub const DPLL1_PHASE_OFFSET_CNFG: c_uint = 0x143;
pub const DPLL2_PHASE_OFFSET_CNFG: c_uint = 0x1C3;
pub const DPLL1_SYNC_EDGE_CNFG: c_uint = 0x140;
pub const DPLL2_SYNC_EDGE_CNFG: c_uint = 0x1C0;
pub const DPLL1_INPUT_MODE_CNFG: c_uint = 0x116;
pub const DPLL2_INPUT_MODE_CNFG: c_uint = 0x196;
pub const DPLL1_OPERATING_STS: c_uint = 0x102;
pub const DPLL2_OPERATING_STS: c_uint = 0x182;
pub const DPLL1_CURRENT_FREQ_STS: c_uint = 0x103;
pub const DPLL2_CURRENT_FREQ_STS: c_uint = 0x183;

// Register bit definitions

// Bit definitions for the DPLL_MODE register

// Bit definitions for DPLL_OPERATING_STS register

// Bit definitions for DPLL_TOD_TRIGGER register

// Bit definitions for REG_SOFT_RESET register

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pll_mode {
    PLL_MODE_MIN = 0,
    PLL_MODE_AUTOMATIC = PLL_MODE_MIN,
    PLL_MODE_FORCE_FREERUN = 1,
    PLL_MODE_FORCE_HOLDOVER = 2,
    PLL_MODE_FORCE_LOCKED = 4,
    PLL_MODE_FORCE_PRE_LOCKED2 = 5,
    PLL_MODE_FORCE_PRE_LOCKED = 6,
    PLL_MODE_FORCE_LOST_PHASE = 7,
    PLL_MODE_DCO = 10,
    PLL_MODE_WPH = 18,
    PLL_MODE_MAX = PLL_MODE_WPH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_tod_trig_sel {
    HW_TOD_TRIG_SEL_MIN = 0,
    HW_TOD_TRIG_SEL_NO_WRITE = HW_TOD_TRIG_SEL_MIN,
    HW_TOD_TRIG_SEL_NO_READ = HW_TOD_TRIG_SEL_MIN,
    HW_TOD_TRIG_SEL_SYNC_SEL = 1,
    HW_TOD_TRIG_SEL_IN12 = 2,
    HW_TOD_TRIG_SEL_IN13 = 3,
    HW_TOD_TRIG_SEL_IN14 = 4,
    HW_TOD_TRIG_SEL_TOD_PPS = 5,
    HW_TOD_TRIG_SEL_TIMER_INTERVAL = 6,
    HW_TOD_TRIG_SEL_MSB_PHASE_OFFSET_CNFG = 7,
    HW_TOD_TRIG_SEL_MSB_HOLDOVER_FREQ_CNFG = 8,
    HW_TOD_WR_TRIG_SEL_MSB_TOD_CNFG = 9,
    HW_TOD_RD_TRIG_SEL_LSB_TOD_STS = HW_TOD_WR_TRIG_SEL_MSB_TOD_CNFG,
    WR_TRIG_SEL_MAX = HW_TOD_WR_TRIG_SEL_MSB_TOD_CNFG,
}

// @brief Enumerated type listing DPLL operational modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpll_state {
    DPLL_STATE_FREERUN = 1,
    DPLL_STATE_HOLDOVER = 2,
    DPLL_STATE_LOCKED = 4,
    DPLL_STATE_PRELOCKED2 = 5,
    DPLL_STATE_PRELOCKED = 6,
    DPLL_STATE_LOSTPHASE = 7,
    DPLL_STATE_MAX
}
