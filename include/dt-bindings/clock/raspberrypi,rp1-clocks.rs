//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/raspberrypi,rp1-clocks.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2021 Raspberry Pi Ltd.
//
pub const RP1_PLL_SYS_CORE: c_int = 0;
pub const RP1_PLL_AUDIO_CORE: c_int = 1;
pub const RP1_PLL_VIDEO_CORE: c_int = 2;
pub const RP1_PLL_SYS: c_int = 3;
pub const RP1_PLL_AUDIO: c_int = 4;
pub const RP1_PLL_VIDEO: c_int = 5;
pub const RP1_PLL_SYS_PRI_PH: c_int = 6;
pub const RP1_PLL_SYS_SEC_PH: c_int = 7;
pub const RP1_PLL_AUDIO_PRI_PH: c_int = 8;
pub const RP1_PLL_SYS_SEC: c_int = 9;
pub const RP1_PLL_AUDIO_SEC: c_int = 10;
pub const RP1_PLL_VIDEO_SEC: c_int = 11;
pub const RP1_CLK_SYS: c_int = 12;
pub const RP1_CLK_SLOW_SYS: c_int = 13;
pub const RP1_CLK_DMA: c_int = 14;
pub const RP1_CLK_UART: c_int = 15;
pub const RP1_CLK_ETH: c_int = 16;
pub const RP1_CLK_PWM0: c_int = 17;
pub const RP1_CLK_PWM1: c_int = 18;
pub const RP1_CLK_AUDIO_IN: c_int = 19;
pub const RP1_CLK_AUDIO_OUT: c_int = 20;
pub const RP1_CLK_I2S: c_int = 21;
pub const RP1_CLK_MIPI0_CFG: c_int = 22;
pub const RP1_CLK_MIPI1_CFG: c_int = 23;
pub const RP1_CLK_PCIE_AUX: c_int = 24;
pub const RP1_CLK_USBH0_MICROFRAME: c_int = 25;
pub const RP1_CLK_USBH1_MICROFRAME: c_int = 26;
pub const RP1_CLK_USBH0_SUSPEND: c_int = 27;
pub const RP1_CLK_USBH1_SUSPEND: c_int = 28;
pub const RP1_CLK_ETH_TSU: c_int = 29;
pub const RP1_CLK_ADC: c_int = 30;
pub const RP1_CLK_SDIO_TIMER: c_int = 31;
pub const RP1_CLK_SDIO_ALT_SRC: c_int = 32;
pub const RP1_CLK_GP0: c_int = 33;
pub const RP1_CLK_GP1: c_int = 34;
pub const RP1_CLK_GP2: c_int = 35;
pub const RP1_CLK_GP3: c_int = 36;
pub const RP1_CLK_GP4: c_int = 37;
pub const RP1_CLK_GP5: c_int = 38;
pub const RP1_CLK_VEC: c_int = 39;
pub const RP1_CLK_DPI: c_int = 40;
pub const RP1_CLK_MIPI0_DPI: c_int = 41;
pub const RP1_CLK_MIPI1_DPI: c_int = 42;
// Extra PLL output channels - RP1B0 only
pub const RP1_PLL_VIDEO_PRI_PH: c_int = 43;
pub const RP1_PLL_AUDIO_TERN: c_int = 44;
// MIPI clocks managed by the DSI driver
pub const RP1_CLK_MIPI0_DSI_BYTECLOCK: c_int = 45;
pub const RP1_CLK_MIPI1_DSI_BYTECLOCK: c_int = 46;
