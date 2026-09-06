//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/microchip/clk-pic32.h
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
// Purna Chandra Mandal,<purna.mandal@microchip.com>
// Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
//

// PIC32 clock data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_clk_common {
    pub dev: *mut device,
    pub iobase: *mut void __iomem,
    pub /: *mut *mut spinlock_t reg_lock; / clock lock,
}

// System PLL clock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_sys_pll_data {
    pub init_data: clk_init_data,
    pub ctrl_reg: u32,
    pub status_reg: u32,
    pub lock_mask: u32,
}

// System clock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_sys_clk_data {
    pub init_data: clk_init_data,
    pub mux_reg: u32,
    pub slew_reg: u32,
    pub parent_map: *const u32,
    pub slew_div: u32,
}

// Reference Oscillator clock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_ref_osc_data {
    pub init_data: clk_init_data,
    pub ctrl_reg: u32,
    pub parent_map: *const u32,
}

// Peripheral Bus clock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_periph_clk_data {
    pub init_data: clk_init_data,
    pub ctrl_reg: u32,
}

// External Secondary Oscillator clock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_sec_osc_data {
    pub init_data: clk_init_data,
    pub enable_reg: u32,
    pub status_reg: u32,
    pub enable_mask: u32,
    pub status_mask: u32,
    pub fixed_rate: c_ulong,
}
