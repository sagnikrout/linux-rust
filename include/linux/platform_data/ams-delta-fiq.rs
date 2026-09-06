//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/ams-delta-fiq.h
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
// include/linux/platform_data/ams-delta-fiq.h
//
// Taken from the original Amstrad modifications to fiq.h
//
// Copyright (c) 2004 Amstrad Plc
// Copyright (c) 2006 Matt Callow
// Copyright (c) 2010 Janusz Krzysztofik
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// These are the offsets from the beginning of the fiq_buffer. They are put here
// since the buffer and header need to be accessed by drivers servicing devices
// which generate GPIO interrupts - e.g. keyboard, modem, hook switch.
//
pub const FIQ_MASK: c_int = 0;
pub const FIQ_STATE: c_int = 1;
pub const FIQ_KEYS_CNT: c_int = 2;
pub const FIQ_TAIL_OFFSET: c_int = 3;
pub const FIQ_HEAD_OFFSET: c_int = 4;
pub const FIQ_BUF_LEN: c_int = 5;
pub const FIQ_KEY: c_int = 6;
pub const FIQ_MISSED_KEYS: c_int = 7;
pub const FIQ_BUFFER_START: c_int = 8;
pub const FIQ_GPIO_INT_MASK: c_int = 9;
pub const FIQ_KEYS_HICNT: c_int = 10;
pub const FIQ_IRQ_PEND: c_int = 11;
pub const FIQ_SIR_CODE_L1: c_int = 12;
pub const IRQ_SIR_CODE_L2: c_int = 13;
pub const FIQ_CNT_INT_00: c_int = 14;
pub const FIQ_CNT_INT_KEY: c_int = 15;
pub const FIQ_CNT_INT_MDM: c_int = 16;
pub const FIQ_CNT_INT_03: c_int = 17;
pub const FIQ_CNT_INT_HSW: c_int = 18;
pub const FIQ_CNT_INT_05: c_int = 19;
pub const FIQ_CNT_INT_06: c_int = 20;
pub const FIQ_CNT_INT_07: c_int = 21;
pub const FIQ_CNT_INT_08: c_int = 22;
pub const FIQ_CNT_INT_09: c_int = 23;
pub const FIQ_CNT_INT_10: c_int = 24;
pub const FIQ_CNT_INT_11: c_int = 25;
pub const FIQ_CNT_INT_12: c_int = 26;
pub const FIQ_CNT_INT_13: c_int = 27;
pub const FIQ_CNT_INT_14: c_int = 28;
pub const FIQ_CNT_INT_15: c_int = 29;

