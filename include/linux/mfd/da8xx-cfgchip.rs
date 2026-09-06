//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da8xx-cfgchip.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// TI DaVinci DA8xx CHIPCFGx registers for syscon consumers.
//
// Copyright (C) 2016 David Lechner <david@lechnology.com>
//

// register offset (32-bit registers)

// CFGCHIP0 (PLL0/EDMA3_0) register bits

// CFGCHIP1 (eCAP/HPI/EDMA3_1/eHRPWM TBCLK/McASP0 AMUTEIN) register bits

// CFGCHIP2 (USB PHY) register bits

// CFGCHIP3 (EMAC/uPP/PLL1/ASYNC3/PRU/DIV4.5/EMIFA) register bits

// CFGCHIP4 (McASP0 AMUNTEIN) register bits

