//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/bcm/brcmstb/pm/pm.h
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
// Definitions for Broadcom STB power management / Always ON (AON) block
//
// Copyright © 2016-2017 Broadcom
//
pub const AON_CTRL_RESET_CTRL: c_uint = 0x00;
pub const AON_CTRL_PM_CTRL: c_uint = 0x04;
pub const AON_CTRL_PM_STATUS: c_uint = 0x08;
pub const AON_CTRL_PM_CPU_WAIT_COUNT: c_uint = 0x10;
pub const AON_CTRL_PM_INITIATE: c_uint = 0x88;
pub const AON_CTRL_HOST_MISC_CMDS: c_uint = 0x8c;
pub const AON_CTRL_SYSTEM_DATA_RAM_OFS: c_uint = 0x200;
// MIPS PM constants
// MEMC0 offsets
pub const DDR40_PHY_CONTROL_REGS_0_PLL_STATUS: c_uint = 0x10;
pub const DDR40_PHY_CONTROL_REGS_0_STANDBY_CTRL: c_uint = 0xa4;
// TIMER offsets
pub const TIMER_TIMER1_CTRL: c_uint = 0x0c;
pub const TIMER_TIMER1_STAT: c_uint = 0x1c;
// TIMER defines
pub const RESET_TIMER: c_uint = 0x0;
pub const START_TIMER: c_uint = 0xbfffffff;
pub const TIMER_MASK: c_uint = 0x3fffffff;
// PM_CTRL bitfield (Method #0)

// PM_CTRL bitfield (Method #1)

// Method 0 bitmasks

// Method 1 bitmask

// s2 asm
extern "C" {
    pub fn brcm_pm_do_s2(s2_params: *mut u32) -> asmlinkage int;
}
// s3 asm

