//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/spacemit/ccu_pll.h
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
// Copyright (c) 2024 SpacemiT Technology Co. Ltd
// Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
//

//
// struct ccu_pll_rate_tbl - Structure mapping between PLL rate and register
// configuration.
//
// @rate:	PLL rate
// @swcr1:	Value of register PLLx_SW1_CTRL.
// @swcr2:	Value of register PLLAx_SW2_CTRL.
// @swcr3:	value of register PLLx_SW3_CTRL.
//
// See below tables for the register used in PPL/PPLA clocks
//
// Regular PLL type
// | Enable | swcr3 | PLLx_SW3_CTRL - BIT[31]    |
// -----------------------------------------------
// | Config | swcr1 | PLLx_SW1_CTRL - BIT[31:0]  |
// |        | swcr2 | Not used                   |
// |        | swcr3 | PLLx_SW3_CTRL - BIT[30:0]  |
//
// Special PLL type A
// | Enable | swcr2 | PLLAx_SW2_CTRL - BIT[16]   |
// -----------------------------------------------
// | Config | swcr1 | PLLAx_SW1_CTRL - BIT[31:0] |
// |        | swcr2 | PLLAx_SW2_CTRL - BIT[15:8] |
// |        | swcr3 | PLLAx_SW3_CTRL - BIT[31:0] |
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_pll_rate_tbl {
    pub rate: c_ulong,
    pub swcr1: u32,
    pub swcr2: u32,
    pub swcr3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_pll_config {
    pub rate_tbl: *const ccu_pll_rate_tbl,
    pub tbl_num: u32,
    pub reg_lock: u32,
    pub mask_lock: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_pll {
    pub common: ccu_common,
    pub config: ccu_pll_config,
}

extern "C" {
    pub fn container_of(_arg: common, ccu_pll: struct, _arg: common) -> return;
}
