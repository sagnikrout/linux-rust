//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/stm32/clk-stm32-core.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) STMicroelectronics 2022 - All Rights Reserved
// Author: Gabriel Fernandez <gabriel.fernandez@foss.st.com> for STMicroelectronics.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_mux_cfg {
    pub offset: u16,
    pub shift: u8,
    pub width: u8,
    pub flags: u8,
    pub table: *mut u32,
    pub ready: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_gate_cfg {
    pub offset: u16,
    pub bit_idx: u8,
    pub set_clr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_div_cfg {
    pub offset: u16,
    pub shift: u8,
    pub width: u8,
    pub flags: u8,
    pub ready: u8,
    pub table: *const clk_div_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_composite_cfg {
    pub mux: c_int,
    pub gate: c_int,
    pub div: c_int,
}

pub const NO_ID: c_uint = 0xFFFFFFFF;
pub const NO_STM32_MUX: c_uint = 0xFFFF;
pub const NO_STM32_DIV: c_uint = 0xFFFF;
pub const NO_STM32_GATE: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_config {
    pub id: c_ulong,
    pub sec_id: c_int,
    pub clock_cfg: *mut c_void,
    pub cfg): *const clock_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_stm32_clock_data {
    pub gate_cpt: *mut u16,
    pub gates: *const stm32_gate_cfg,
    pub muxes: *const stm32_mux_cfg,
    pub dividers: *const stm32_div_cfg,
    pub hw): *mut *mut *mut clk_hw (is_multi_mux)(clk_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_rcc_match_data {
    pub hw_clks: *mut clk_hw_onecell_data,
    pub num_clocks: c_uint,
    pub tab_clocks: *const clock_config,
    pub maxbinding: c_uint,
    pub clock_data: *mut clk_stm32_clock_data,
    pub reset_data: *mut clk_stm32_reset_data,
    pub cfg): *const clock_config,
    pub cfg): *const *const *const int (multi_mux)(void __iomem base, struct clock_config,
}

// MUX define
pub const MUX_NO_RDY: c_uint = 0xFF;

// DIV define
pub const DIV_NO_RDY: c_uint = 0xFF;
// Definition of clock structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_stm32_mux {
    pub mux_id: u16,
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub clock_data: *mut clk_stm32_clock_data,
    pub /: *mut *mut *mut spinlock_t lock; / spin lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_stm32_gate {
    pub gate_id: u16,
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub clock_data: *mut clk_stm32_clock_data,
    pub /: *mut *mut *mut spinlock_t lock; / spin lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_stm32_div {
    pub div_id: u16,
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub clock_data: *mut clk_stm32_clock_data,
    pub /: *mut *mut *mut spinlock_t lock; / spin lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_stm32_composite {
    pub gate_id: u16,
    pub mux_id: u16,
    pub div_id: u16,
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub clock_data: *mut clk_stm32_clock_data,
    pub /: *mut *mut *mut spinlock_t lock; / spin lock,
}

// Clock operators
// Clock registering

