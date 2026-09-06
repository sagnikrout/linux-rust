//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs48l32.h
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
// Register definitions for Cirrus Logic CS48L32
//
// Copyright (C) 2017-2018, 2020, 2022, 2025 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// pll_id for snd_soc_component_set_pll()
pub const CS48L32_FLL1_REFCLK: c_int = 1;
// source for snd_soc_component_set_pll()

pub const CS48L32_FLL_SRC_MCLK1: c_int = 0;
pub const CS48L32_FLL_SRC_PDMCLK: c_int = 5;
pub const CS48L32_FLL_SRC_ASP1_BCLK: c_int = 8;
pub const CS48L32_FLL_SRC_ASP2_BCLK: c_int = 9;
pub const CS48L32_FLL_SRC_ASP1_FSYNC: c_int = 12;
pub const CS48L32_FLL_SRC_ASP2_FSYNC: c_int = 13;
// clk_id for snd_soc_component_set_sysclk() and snd_soc_dai_set_sysclk()
pub const CS48L32_CLK_SYSCLK_1: c_int = 1;
pub const CS48L32_CLK_SYSCLK_2: c_int = 2;
pub const CS48L32_CLK_SYSCLK_3: c_int = 3;
pub const CS48L32_CLK_SYSCLK_4: c_int = 4;
pub const CS48L32_CLK_DSPCLK: c_int = 7;
pub const CS48L32_CLK_PDM_FLLCLK: c_int = 13;
// source for snd_soc_component_set_sysclk()
pub const CS48L32_CLK_SRC_MCLK1: c_uint = 0x0;
pub const CS48L32_CLK_SRC_FLL1: c_uint = 0x4;
pub const CS48L32_CLK_SRC_ASP1_BCLK: c_uint = 0x8;
pub const CS48L32_CLK_SRC_ASP2_BCLK: c_uint = 0x9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs48l32 {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub reset_gpio: *mut gpio_desc,
    pub mclk1: *mut clk,
    pub core_supplies: [regulator_bulk_data; 2],
    pub vdd_d: *mut regulator,
    pub irq: c_int,
}
