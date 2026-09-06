//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/sound/qcom,lpass.h
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
pub const MI2S_PRIMARY: c_int = 0;
pub const MI2S_SECONDARY: c_int = 1;
pub const MI2S_TERTIARY: c_int = 2;
pub const MI2S_QUATERNARY: c_int = 3;
pub const MI2S_QUINARY: c_int = 4;
pub const LPASS_DP_RX: c_int = 5;
pub const LPASS_CDC_DMA_RX0: c_int = 6;
pub const LPASS_CDC_DMA_RX1: c_int = 7;
pub const LPASS_CDC_DMA_RX2: c_int = 8;
pub const LPASS_CDC_DMA_RX3: c_int = 9;
pub const LPASS_CDC_DMA_RX4: c_int = 10;
pub const LPASS_CDC_DMA_RX5: c_int = 11;
pub const LPASS_CDC_DMA_RX6: c_int = 12;
pub const LPASS_CDC_DMA_RX7: c_int = 13;
pub const LPASS_CDC_DMA_RX8: c_int = 14;
pub const LPASS_CDC_DMA_RX9: c_int = 15;
pub const LPASS_CDC_DMA_TX0: c_int = 16;
pub const LPASS_CDC_DMA_TX1: c_int = 17;
pub const LPASS_CDC_DMA_TX2: c_int = 18;
pub const LPASS_CDC_DMA_TX3: c_int = 19;
pub const LPASS_CDC_DMA_TX4: c_int = 20;
pub const LPASS_CDC_DMA_TX5: c_int = 21;
pub const LPASS_CDC_DMA_TX6: c_int = 22;
pub const LPASS_CDC_DMA_TX7: c_int = 23;
pub const LPASS_CDC_DMA_TX8: c_int = 24;
pub const LPASS_CDC_DMA_VA_TX0: c_int = 25;
pub const LPASS_CDC_DMA_VA_TX1: c_int = 26;
pub const LPASS_CDC_DMA_VA_TX2: c_int = 27;
pub const LPASS_CDC_DMA_VA_TX3: c_int = 28;
pub const LPASS_CDC_DMA_VA_TX4: c_int = 29;
pub const LPASS_CDC_DMA_VA_TX5: c_int = 30;
pub const LPASS_CDC_DMA_VA_TX6: c_int = 31;
pub const LPASS_CDC_DMA_VA_TX7: c_int = 32;
pub const LPASS_CDC_DMA_VA_TX8: c_int = 33;
pub const LPASS_MCLK0: c_int = 0;
