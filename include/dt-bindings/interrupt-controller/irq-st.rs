//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interrupt-controller/irq-st.h
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
// include/linux/irqchip/irq-st.h
//
// Copyright (C) 2014 STMicroelectronics – All Rights Reserved
//
// Author: Lee Jones <lee.jones@linaro.org>
//
pub const ST_IRQ_SYSCFG_EXT_0: c_int = 0;
pub const ST_IRQ_SYSCFG_EXT_1: c_int = 1;
pub const ST_IRQ_SYSCFG_EXT_2: c_int = 2;
pub const ST_IRQ_SYSCFG_CTI_0: c_int = 3;
pub const ST_IRQ_SYSCFG_CTI_1: c_int = 4;
pub const ST_IRQ_SYSCFG_PMU_0: c_int = 5;
pub const ST_IRQ_SYSCFG_PMU_1: c_int = 6;
pub const ST_IRQ_SYSCFG_pl310_L2: c_int = 7;
pub const ST_IRQ_SYSCFG_DISABLED: c_uint = 0xFFFFFFFF;
pub const ST_IRQ_SYSCFG_EXT_1_INV: c_uint = 0x1;
pub const ST_IRQ_SYSCFG_EXT_2_INV: c_uint = 0x2;
pub const ST_IRQ_SYSCFG_EXT_3_INV: c_uint = 0x4;
