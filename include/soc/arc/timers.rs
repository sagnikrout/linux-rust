//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/arc/timers.h
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
// Copyright (C) 2016-17 Synopsys, Inc. (www.synopsys.com)
//

// Timer related Aux registers
pub const ARC_REG_TIMER0_LIMIT: c_uint = 0x23	/* timer 0 limit */;
pub const ARC_REG_TIMER0_CTRL: c_uint = 0x22	/* timer 0 control */;
pub const ARC_REG_TIMER0_CNT: c_uint = 0x21	/* timer 0 count */;
pub const ARC_REG_TIMER1_LIMIT: c_uint = 0x102	/* timer 1 limit */;
pub const ARC_REG_TIMER1_CTRL: c_uint = 0x101	/* timer 1 control */;
pub const ARC_REG_TIMER1_CNT: c_uint = 0x100	/* timer 1 count */;
// CTRL reg bits

pub const ARC_TIMERN_MAX: c_uint = 0xFFFFFFFF;
pub const ARC_REG_TIMERS_BCR: c_uint = 0x75;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcr_timer {

    pub ver:8: unsigned int pad2:15, rtsc:1, pad1:5, rtc:1, t1:1, t0:1,,

    pub pad2:15: unsigned int ver:8, t0:1, t1:1, rtc:1, pad1:5, rtsc:1,,

}
