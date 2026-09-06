//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/nuvoton,npcm7xx-clock.h
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
// Nuvoton NPCM7xx Clock Generator binding
// clock binding number for all clocks supported by nuvoton,npcm7xx-clk
//
// Copyright (C) 2018 Nuvoton Technologies tali.perry@nuvoton.com
//
pub const NPCM7XX_CLK_CPU: c_int = 0;
pub const NPCM7XX_CLK_GFX_PIXEL: c_int = 1;
pub const NPCM7XX_CLK_MC: c_int = 2;
pub const NPCM7XX_CLK_ADC: c_int = 3;
pub const NPCM7XX_CLK_AHB: c_int = 4;
pub const NPCM7XX_CLK_TIMER: c_int = 5;
pub const NPCM7XX_CLK_UART: c_int = 6;
pub const NPCM7XX_CLK_MMC: c_int = 7;
pub const NPCM7XX_CLK_SPI3: c_int = 8;
pub const NPCM7XX_CLK_PCI: c_int = 9;
pub const NPCM7XX_CLK_AXI: c_int = 10;
pub const NPCM7XX_CLK_APB4: c_int = 11;
pub const NPCM7XX_CLK_APB3: c_int = 12;
pub const NPCM7XX_CLK_APB2: c_int = 13;
pub const NPCM7XX_CLK_APB1: c_int = 14;
pub const NPCM7XX_CLK_APB5: c_int = 15;
pub const NPCM7XX_CLK_CLKOUT: c_int = 16;
pub const NPCM7XX_CLK_GFX: c_int = 17;
pub const NPCM7XX_CLK_SU: c_int = 18;
pub const NPCM7XX_CLK_SU48: c_int = 19;
pub const NPCM7XX_CLK_SDHC: c_int = 20;
pub const NPCM7XX_CLK_SPI0: c_int = 21;
pub const NPCM7XX_CLK_SPIX: c_int = 22;
pub const NPCM7XX_CLK_REFCLK: c_int = 23;
pub const NPCM7XX_CLK_SYSBYPCK: c_int = 24;
pub const NPCM7XX_CLK_MCBYPCK: c_int = 25;

