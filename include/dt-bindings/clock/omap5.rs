//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/omap5.h
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
pub const OMAP5_CLKCTRL_OFFSET: c_uint = 0x20;

// mpu clocks

// dsp clocks

// abe clocks

// l3main1 clocks

// l3main2 clocks

// ipu clocks

// dma clocks

// emif clocks

// l4cfg clocks

// l3instr clocks

// l4per clocks

// l4_secure clocks
pub const OMAP5_L4_SECURE_CLKCTRL_OFFSET: c_uint = 0x1a0;

// iva clocks

// dss clocks

// gpu clocks

// l3init clocks

// wkupaon clocks

