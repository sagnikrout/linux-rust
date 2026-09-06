//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sun55i-a523-mcu-ccu.h
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
pub const CLK_MCU_PLL_AUDIO1: c_int = 0;
pub const CLK_MCU_PLL_AUDIO1_DIV2: c_int = 1;
pub const CLK_MCU_PLL_AUDIO1_DIV5: c_int = 2;
pub const CLK_MCU_AUDIO_OUT: c_int = 3;
pub const CLK_MCU_DSP: c_int = 4;
pub const CLK_MCU_I2S0: c_int = 5;
pub const CLK_MCU_I2S1: c_int = 6;
pub const CLK_MCU_I2S2: c_int = 7;
pub const CLK_MCU_I2S3: c_int = 8;
pub const CLK_MCU_I2S3_ASRC: c_int = 9;
pub const CLK_BUS_MCU_I2S0: c_int = 10;
pub const CLK_BUS_MCU_I2S1: c_int = 11;
pub const CLK_BUS_MCU_I2S2: c_int = 12;
pub const CLK_BUS_MCU_I2S3: c_int = 13;
pub const CLK_MCU_SPDIF_TX: c_int = 14;
pub const CLK_MCU_SPDIF_RX: c_int = 15;
pub const CLK_BUS_MCU_SPDIF: c_int = 16;
pub const CLK_MCU_DMIC: c_int = 17;
pub const CLK_BUS_MCU_DMIC: c_int = 18;
pub const CLK_MCU_AUDIO_CODEC_DAC: c_int = 19;
pub const CLK_MCU_AUDIO_CODEC_ADC: c_int = 20;
pub const CLK_BUS_MCU_AUDIO_CODEC: c_int = 21;
pub const CLK_BUS_MCU_DSP_MSGBOX: c_int = 22;
pub const CLK_BUS_MCU_DSP_CFG: c_int = 23;
pub const CLK_BUS_MCU_NPU_HCLK: c_int = 24;
pub const CLK_BUS_MCU_NPU_ACLK: c_int = 25;
pub const CLK_MCU_TIMER0: c_int = 26;
pub const CLK_MCU_TIMER1: c_int = 27;
pub const CLK_MCU_TIMER2: c_int = 28;
pub const CLK_MCU_TIMER3: c_int = 29;
pub const CLK_MCU_TIMER4: c_int = 30;
pub const CLK_MCU_TIMER5: c_int = 31;
pub const CLK_BUS_MCU_TIMER: c_int = 32;
pub const CLK_BUS_MCU_DMA: c_int = 33;
pub const CLK_MCU_TZMA0: c_int = 34;
pub const CLK_MCU_TZMA1: c_int = 35;
pub const CLK_BUS_MCU_PUBSRAM: c_int = 36;
pub const CLK_MCU_MBUS_DMA: c_int = 37;
pub const CLK_MCU_MBUS: c_int = 38;
pub const CLK_MCU_RISCV: c_int = 39;
pub const CLK_BUS_MCU_RISCV_CFG: c_int = 40;
pub const CLK_BUS_MCU_RISCV_MSGBOX: c_int = 41;
pub const CLK_MCU_PWM0: c_int = 42;
pub const CLK_BUS_MCU_PWM0: c_int = 43;
