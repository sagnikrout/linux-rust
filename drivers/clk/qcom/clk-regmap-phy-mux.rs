//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/clk-regmap-phy-mux.h
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
// Copyright (c) 2022, Linaro Ltd.
//

//
// A clock implementation for PHY pipe and symbols clock muxes.
//
// If the clock is running off the from-PHY source, report it as enabled.
// Report it as disabled otherwise (if it uses reference source).
//
// This way the PHY will disable the pipe clock before turning off the GDSC,
// which in turn would lead to disabling corresponding pipe_clk_src (and thus
// it being parked to a safe, reference clock source). And vice versa, after
// enabling the GDSC the PHY will enable the pipe clock, which would cause
// pipe_clk_src to be switched from a safe source to the working one.
//
// For some platforms this should be used for the UFS symbol_clk_src clocks
// too.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_regmap_phy_mux {
    pub reg: u32,
    pub clkr: clk_regmap,
}
