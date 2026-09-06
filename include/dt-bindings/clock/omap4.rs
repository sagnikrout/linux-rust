//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/omap4.h
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
// Copyright 2017 Texas Instruments, Inc.
//
pub const OMAP4_CLKCTRL_OFFSET: c_uint = 0x20;

// mpuss clocks

// tesla clocks

// abe clocks

// l4_ao clocks

// l3_1 clocks

// l3_2 clocks

// ducati clocks

// l3_dma clocks

// l3_emif clocks

// d2d clocks

// l4_cfg clocks

// l3_instr clocks

// ivahd clocks

// iss clocks

// l3_dss clocks

// l3_gfx clocks

// l3_init clocks

// l4_per clocks

// l4_secure clocks
pub const OMAP4_L4_SECURE_CLKCTRL_OFFSET: c_uint = 0x1a0;

// l4_wkup clocks

// emu_sys clocks

