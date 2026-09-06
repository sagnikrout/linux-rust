//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/mfd/dbx500-prcmu.h
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
// This header provides constants for the PRCMU bindings.
//
// Clock identifiers.
//
pub const ARMCLK: c_int = 0;
pub const PRCMU_ACLK: c_int = 1;
pub const PRCMU_SVAMMCSPCLK: c_int = 2;

pub const PRCMU_SIACLK: c_int = 3;

pub const PRCMU_SGACLK: c_int = 4;
pub const PRCMU_UARTCLK: c_int = 5;
pub const PRCMU_MSP02CLK: c_int = 6;
pub const PRCMU_MSP1CLK: c_int = 7;
pub const PRCMU_I2CCLK: c_int = 8;
pub const PRCMU_SDMMCCLK: c_int = 9;
pub const PRCMU_SLIMCLK: c_int = 10;

pub const PRCMU_PER1CLK: c_int = 11;
pub const PRCMU_PER2CLK: c_int = 12;
pub const PRCMU_PER3CLK: c_int = 13;
pub const PRCMU_PER5CLK: c_int = 14;
pub const PRCMU_PER6CLK: c_int = 15;
pub const PRCMU_PER7CLK: c_int = 16;
pub const PRCMU_LCDCLK: c_int = 17;
pub const PRCMU_BMLCLK: c_int = 18;
pub const PRCMU_HSITXCLK: c_int = 19;
pub const PRCMU_HSIRXCLK: c_int = 20;
pub const PRCMU_HDMICLK: c_int = 21;
pub const PRCMU_APEATCLK: c_int = 22;
pub const PRCMU_APETRACECLK: c_int = 23;
pub const PRCMU_MCDECLK: c_int = 24;
pub const PRCMU_IPI2CCLK: c_int = 25;
pub const PRCMU_DSIALTCLK: c_int = 26;
pub const PRCMU_DMACLK: c_int = 27;
pub const PRCMU_B2R2CLK: c_int = 28;
pub const PRCMU_TVCLK: c_int = 29;
pub const SPARE_UNIPROCLK: c_int = 30;
pub const PRCMU_SSPCLK: c_int = 31;
pub const PRCMU_RNGCLK: c_int = 32;
pub const PRCMU_UICCCLK: c_int = 33;

pub const PRCMU_SPARE1CLK: c_int = 36;
pub const PRCMU_SPARE2CLK: c_int = 37;
pub const PRCMU_NUM_REG_CLOCKS: c_int = 38;

pub const PRCMU_SYSCLK: c_int = 39;
pub const PRCMU_CDCLK: c_int = 40;
pub const PRCMU_TIMCLK: c_int = 41;
pub const PRCMU_PLLSOC0: c_int = 42;
pub const PRCMU_PLLSOC1: c_int = 43;
pub const PRCMU_ARMSS: c_int = 44;
pub const PRCMU_PLLDDR: c_int = 45;
// DSI Clocks
pub const PRCMU_PLLDSI: c_int = 46;
pub const PRCMU_DSI0CLK: c_int = 47;
pub const PRCMU_DSI1CLK: c_int = 48;
pub const PRCMU_DSI0ESCCLK: c_int = 49;
pub const PRCMU_DSI1ESCCLK: c_int = 50;
pub const PRCMU_DSI2ESCCLK: c_int = 51;
// LCD DSI PLL - Ux540 only
pub const PRCMU_PLLDSI_LCD: c_int = 52;
pub const PRCMU_DSI0CLK_LCD: c_int = 53;
pub const PRCMU_DSI1CLK_LCD: c_int = 54;
pub const PRCMU_DSI0ESCCLK_LCD: c_int = 55;
pub const PRCMU_DSI1ESCCLK_LCD: c_int = 56;
pub const PRCMU_DSI2ESCCLK_LCD: c_int = 57;
pub const PRCMU_NUM_CLKS: c_int = 58;
