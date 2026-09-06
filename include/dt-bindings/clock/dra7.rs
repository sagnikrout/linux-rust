//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/dra7.h
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
pub const DRA7_CLKCTRL_OFFSET: c_uint = 0x20;

// mpu clocks

// dsp1 clocks

// ipu1 clocks

// ipu clocks
pub const DRA7_IPU_CLKCTRL_OFFSET: c_uint = 0x50;

// dsp2 clocks

// rtc clocks

// vip clocks

// vpe clocks
pub const DRA7_VPE_CLKCTRL_OFFSET: c_uint = 0x60;

// coreaon clocks

// l3main1 clocks

// ipu2 clocks

// dma clocks

// emif clocks

// atl clocks
pub const DRA7_ATL_CLKCTRL_OFFSET: c_uint = 0x0;

// l4cfg clocks

// l3instr clocks

// iva clocks

// dss clocks

// gpu clocks

// l3init clocks

// pcie clocks
pub const DRA7_PCIE_CLKCTRL_OFFSET: c_uint = 0xb0;

// gmac clocks
pub const DRA7_GMAC_CLKCTRL_OFFSET: c_uint = 0xd0;

// l4per clocks
pub const DRA7_L4PER_CLKCTRL_OFFSET: c_uint = 0x28;

// l4sec clocks
pub const DRA7_L4SEC_CLKCTRL_OFFSET: c_uint = 0x1a0;

// l4per2 clocks
pub const DRA7_L4PER2_CLKCTRL_OFFSET: c_uint = 0xc;

// l4per3 clocks
pub const DRA7_L4PER3_CLKCTRL_OFFSET: c_uint = 0x14;

// wkupaon clocks

