//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/ti/omap-mcbsp.h
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
// omap-mcbsp.h
//
// Copyright (C) 2008 Nokia Corporation
//
// Contact: Jarkko Nikula <jarkko.nikula@bitmer.com>
// Peter Ujfalusi <peter.ujfalusi@ti.com>
//

// Source clocks for McBSP sample rate generator
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_mcbsp_clksrg_clk {
    OMAP_MCBSP_SYSCLK_CLKS_FCLK,	/* Internal FCLK */
    OMAP_MCBSP_SYSCLK_CLKS_EXT,	/* External CLKS pin */
    OMAP_MCBSP_SYSCLK_CLK,		/* Internal ICLK */
    OMAP_MCBSP_SYSCLK_CLKX_EXT,	/* External CLKX pin */
    OMAP_MCBSP_SYSCLK_CLKR_EXT,	/* External CLKR pin */
}

// McBSP dividers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_mcbsp_div {
    OMAP_MCBSP_CLKGDV,		/* Sample rate generator divider */
}

extern "C" {
    pub fn omap_mcbsp_st_add_controls(rtd: *mut snd_soc_pcm_runtime, port_id: c_int) -> c_int;
}
