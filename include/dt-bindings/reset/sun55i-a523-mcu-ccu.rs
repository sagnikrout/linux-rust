//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/sun55i-a523-mcu-ccu.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2025 Chen-Yu Tsai <wens@csie.org>
//
pub const RST_BUS_MCU_I2S0: c_int = 0;
pub const RST_BUS_MCU_I2S1: c_int = 1;
pub const RST_BUS_MCU_I2S2: c_int = 2;
pub const RST_BUS_MCU_I2S3: c_int = 3;
pub const RST_BUS_MCU_SPDIF: c_int = 4;
pub const RST_BUS_MCU_DMIC: c_int = 5;
pub const RST_BUS_MCU_AUDIO_CODEC: c_int = 6;
pub const RST_BUS_MCU_DSP_MSGBOX: c_int = 7;
pub const RST_BUS_MCU_DSP_CFG: c_int = 8;
pub const RST_BUS_MCU_NPU: c_int = 9;
pub const RST_BUS_MCU_TIMER: c_int = 10;
pub const RST_BUS_MCU_DSP_DEBUG: c_int = 11;
pub const RST_BUS_MCU_DSP: c_int = 12;
pub const RST_BUS_MCU_DMA: c_int = 13;
pub const RST_BUS_MCU_PUBSRAM: c_int = 14;
pub const RST_BUS_MCU_RISCV_CFG: c_int = 15;
pub const RST_BUS_MCU_RISCV_DEBUG: c_int = 16;
pub const RST_BUS_MCU_RISCV_CORE: c_int = 17;
pub const RST_BUS_MCU_RISCV_MSGBOX: c_int = 18;
pub const RST_BUS_MCU_PWM0: c_int = 19;
