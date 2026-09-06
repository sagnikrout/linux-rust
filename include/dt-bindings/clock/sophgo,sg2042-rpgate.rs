//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sophgo,sg2042-rpgate.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (C) 2023 Sophgo Technology Inc. All rights reserved.
//
pub const GATE_CLK_RXU0: c_int = 0;
pub const GATE_CLK_RXU1: c_int = 1;
pub const GATE_CLK_RXU2: c_int = 2;
pub const GATE_CLK_RXU3: c_int = 3;
pub const GATE_CLK_RXU4: c_int = 4;
pub const GATE_CLK_RXU5: c_int = 5;
pub const GATE_CLK_RXU6: c_int = 6;
pub const GATE_CLK_RXU7: c_int = 7;
pub const GATE_CLK_RXU8: c_int = 8;
pub const GATE_CLK_RXU9: c_int = 9;
pub const GATE_CLK_RXU10: c_int = 10;
pub const GATE_CLK_RXU11: c_int = 11;
pub const GATE_CLK_RXU12: c_int = 12;
pub const GATE_CLK_RXU13: c_int = 13;
pub const GATE_CLK_RXU14: c_int = 14;
pub const GATE_CLK_RXU15: c_int = 15;
pub const GATE_CLK_RXU16: c_int = 16;
pub const GATE_CLK_RXU17: c_int = 17;
pub const GATE_CLK_RXU18: c_int = 18;
pub const GATE_CLK_RXU19: c_int = 19;
pub const GATE_CLK_RXU20: c_int = 20;
pub const GATE_CLK_RXU21: c_int = 21;
pub const GATE_CLK_RXU22: c_int = 22;
pub const GATE_CLK_RXU23: c_int = 23;
pub const GATE_CLK_RXU24: c_int = 24;
pub const GATE_CLK_RXU25: c_int = 25;
pub const GATE_CLK_RXU26: c_int = 26;
pub const GATE_CLK_RXU27: c_int = 27;
pub const GATE_CLK_RXU28: c_int = 28;
pub const GATE_CLK_RXU29: c_int = 29;
pub const GATE_CLK_RXU30: c_int = 30;
pub const GATE_CLK_RXU31: c_int = 31;
pub const GATE_CLK_MP0: c_int = 32;
pub const GATE_CLK_MP1: c_int = 33;
pub const GATE_CLK_MP2: c_int = 34;
pub const GATE_CLK_MP3: c_int = 35;
pub const GATE_CLK_MP4: c_int = 36;
pub const GATE_CLK_MP5: c_int = 37;
pub const GATE_CLK_MP6: c_int = 38;
pub const GATE_CLK_MP7: c_int = 39;
pub const GATE_CLK_MP8: c_int = 40;
pub const GATE_CLK_MP9: c_int = 41;
pub const GATE_CLK_MP10: c_int = 42;
pub const GATE_CLK_MP11: c_int = 43;
pub const GATE_CLK_MP12: c_int = 44;
pub const GATE_CLK_MP13: c_int = 45;
pub const GATE_CLK_MP14: c_int = 46;
pub const GATE_CLK_MP15: c_int = 47;
