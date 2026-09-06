//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ti/omap1-io.h
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
// NOTE: Please use ioremap + __raw_read/write where possible instead of these
//
extern "C" {
    pub fn omap_readb(pa: u32) -> u8;
}
extern "C" {
    pub fn omap_readw(pa: u32) -> u16;
}
extern "C" {
    pub fn omap_readl(pa: u32) -> u32;
}
extern "C" {
    pub fn omap_writeb(v: u8, pa: u32);
}
extern "C" {
    pub fn omap_writew(v: u16, pa: u32);
}
extern "C" {
    pub fn omap_writel(v: u32, pa: u32);
}

//
// ----------------------------------------------------------------------------
// System control registers
// ----------------------------------------------------------------------------
//
pub const MOD_CONF_CTRL_0: c_uint = 0xfffe1080;
pub const MOD_CONF_CTRL_1: c_uint = 0xfffe1110;
//
// ---------------------------------------------------------------------------
// UPLD
// ---------------------------------------------------------------------------
//

//
// ----------------------------------------------------------------------------
// Clocks
// ----------------------------------------------------------------------------
//

pub const CK_RATEF: c_int = 1;
pub const CK_IDLEF: c_int = 2;
pub const CK_ENABLEF: c_int = 4;
pub const CK_SELECTF: c_int = 8;
// Macro flag: #define SETARM_IDLE_SHIFT
// DPLL control registers

// DSP clock control. Must use __raw_readw() and __raw_writew() with these

//
// ----------------------------------------------------------------------------
// Pulse-Width Light
// ----------------------------------------------------------------------------
//
pub const OMAP_PWL_BASE: c_uint = 0xfffb5800;

//
// ----------------------------------------------------------------------------
// Pin multiplexing registers
// ----------------------------------------------------------------------------
//
pub const FUNC_MUX_CTRL_0: c_uint = 0xfffe1000;
pub const FUNC_MUX_CTRL_1: c_uint = 0xfffe1004;
pub const FUNC_MUX_CTRL_2: c_uint = 0xfffe1008;
pub const COMP_MODE_CTRL_0: c_uint = 0xfffe100c;
pub const FUNC_MUX_CTRL_3: c_uint = 0xfffe1010;
pub const FUNC_MUX_CTRL_4: c_uint = 0xfffe1014;
pub const FUNC_MUX_CTRL_5: c_uint = 0xfffe1018;
pub const FUNC_MUX_CTRL_6: c_uint = 0xfffe101C;
pub const FUNC_MUX_CTRL_7: c_uint = 0xfffe1020;
pub const FUNC_MUX_CTRL_8: c_uint = 0xfffe1024;
pub const FUNC_MUX_CTRL_9: c_uint = 0xfffe1028;
pub const FUNC_MUX_CTRL_A: c_uint = 0xfffe102C;
pub const FUNC_MUX_CTRL_B: c_uint = 0xfffe1030;
pub const FUNC_MUX_CTRL_C: c_uint = 0xfffe1034;
pub const FUNC_MUX_CTRL_D: c_uint = 0xfffe1038;
pub const PULL_DWN_CTRL_0: c_uint = 0xfffe1040;
pub const PULL_DWN_CTRL_1: c_uint = 0xfffe1044;
pub const PULL_DWN_CTRL_2: c_uint = 0xfffe1048;
pub const PULL_DWN_CTRL_3: c_uint = 0xfffe104c;
pub const PULL_DWN_CTRL_4: c_uint = 0xfffe10ac;
// OMAP-1610 specific multiplexing registers
pub const FUNC_MUX_CTRL_E: c_uint = 0xfffe1090;
pub const FUNC_MUX_CTRL_F: c_uint = 0xfffe1094;
pub const FUNC_MUX_CTRL_10: c_uint = 0xfffe1098;
pub const FUNC_MUX_CTRL_11: c_uint = 0xfffe109c;
pub const FUNC_MUX_CTRL_12: c_uint = 0xfffe10a0;
pub const PU_PD_SEL_0: c_uint = 0xfffe10b4;
pub const PU_PD_SEL_1: c_uint = 0xfffe10b8;
pub const PU_PD_SEL_2: c_uint = 0xfffe10bc;
pub const PU_PD_SEL_3: c_uint = 0xfffe10c0;
pub const PU_PD_SEL_4: c_uint = 0xfffe10c4;
