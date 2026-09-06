//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/regs-fimc.h
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
// drivers/gpu/drm/exynos/regs-fimc.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Register definition file for Samsung Camera Interface (FIMC) driver
//
// Register part
//
// Input source format

// Window offset

// Global control

// Window offset 2

// Y 1st frame start address for output DMA

// Y 2nd frame start address for output DMA

// Y 3rd frame start address for output DMA

// Y 4th frame start address for output DMA

// Cb 1st frame start address for output DMA

// Cb 2nd frame start address for output DMA

// Cb 3rd frame start address for output DMA

// Cb 4th frame start address for output DMA

// Cr 1st frame start address for output DMA

// Cr 2nd frame start address for output DMA

// Cr 3rd frame start address for output DMA

// Cr 4th frame start address for output DMA

// Target image format

// Output DMA control

// Pre-scaler control 1

// Pre-scaler control 2

// Main scaler control

// Target area

// Status

// Status2

// Image capture enable command

// Capture sequence

// Image effects

// Y frame start address for input DMA

// Cb frame start address for input DMA

// Cr frame start address for input DMA

// Input DMA Y Line Skip

// Input DMA Cb Line Skip

// Input DMA Cr Line Skip

// Real input DMA image size

// Input DMA control

// Y frame start address for input DMA

// Cb frame start address for input DMA

// Cr frame start address for input DMA

// Output DMA Y offset

// Output DMA CB offset

// Output DMA CR offset

// Input DMA Y offset

// Input DMA CB offset

// Input DMA CR offset

// Input DMA original image size

// Output DMA original image size

// Real output DMA image size

// DMA parameter

// MIPI CSI image format

// FIMC Clock Source Select

// Add for FIMC v5.1
// Output Frame Buffer Sequence

// Y 5th frame start address for output DMA

// Y 6th frame start address for output DMA

// Y 7th frame start address for output DMA

// Y 8th frame start address for output DMA

// Y 9th frame start address for output DMA

// Y 10th frame start address for output DMA

// Y 11th frame start address for output DMA

// Y 12th frame start address for output DMA

// Y 13th frame start address for output DMA

// Y 14th frame start address for output DMA

// Y 15th frame start address for output DMA

// Y 16th frame start address for output DMA

// Y 17th frame start address for output DMA

// Y 18th frame start address for output DMA

// Y 19th frame start address for output DMA

// Y 20th frame start address for output DMA

// Y 21th frame start address for output DMA

// Y 22th frame start address for output DMA

// Y 23th frame start address for output DMA

// Y 24th frame start address for output DMA

// Y 25th frame start address for output DMA

// Y 26th frame start address for output DMA

// Y 27th frame start address for output DMA

// Y 28th frame start address for output DMA

// Y 29th frame start address for output DMA

// Y 30th frame start address for output DMA

// Y 31th frame start address for output DMA

// Y 32th frame start address for output DMA

// CB 5th frame start address for output DMA

// CB 6th frame start address for output DMA

// CB 7th frame start address for output DMA

// CB 8th frame start address for output DMA

// CB 9th frame start address for output DMA

// CB 10th frame start address for output DMA

// CB 11th frame start address for output DMA

// CB 12th frame start address for output DMA

// CB 13th frame start address for output DMA

// CB 14th frame start address for output DMA

// CB 15th frame start address for output DMA

// CB 16th frame start address for output DMA

// CB 17th frame start address for output DMA

// CB 18th frame start address for output DMA

// CB 19th frame start address for output DMA

// CB 20th frame start address for output DMA

// CB 21th frame start address for output DMA

// CB 22th frame start address for output DMA

// CB 23th frame start address for output DMA

// CB 24th frame start address for output DMA

// CB 25th frame start address for output DMA

// CB 26th frame start address for output DMA

// CB 27th frame start address for output DMA

// CB 28th frame start address for output DMA

// CB 29th frame start address for output DMA

// CB 30th frame start address for output DMA

// CB 31th frame start address for output DMA

// CB 32th frame start address for output DMA

// CR 5th frame start address for output DMA

// CR 6th frame start address for output DMA

// CR 7th frame start address for output DMA

// CR 8th frame start address for output DMA

// CR 9th frame start address for output DMA

// CR 10th frame start address for output DMA

// CR 11th frame start address for output DMA

// CR 12th frame start address for output DMA

// CR 13th frame start address for output DMA

// CR 14th frame start address for output DMA

// CR 15th frame start address for output DMA

// CR 16th frame start address for output DMA

// CR 17th frame start address for output DMA

// CR 18th frame start address for output DMA

// CR 19th frame start address for output DMA

// CR 20th frame start address for output DMA

// CR 21th frame start address for output DMA

// CR 22th frame start address for output DMA

// CR 23th frame start address for output DMA

// CR 24th frame start address for output DMA

// CR 25th frame start address for output DMA

// CR 26th frame start address for output DMA

// CR 27th frame start address for output DMA

// CR 28th frame start address for output DMA

// CR 29th frame start address for output DMA

// CR 30th frame start address for output DMA

// CR 31th frame start address for output DMA

// CR 32th frame start address for output DMA

//
// Macro part
//
// frame start address 1 ~ 4, 5 ~ 32
// Number of Default PingPong Memory
pub const DEF_PP: c_int = 4;

// Number of Default PingPong Memory
pub const DEF_IPP: c_int = 1;

//
// Bit definition part
//
// Source format register

// ITU601 16bit only

// ITU601 16bit only

// Window offset register

// Global control register

// Window offset2 register

// Target format register

// Output DMA control register

// Main scaler control register

// Status register

// Image capture enable register

// Image effects register

// Real input DMA size register

// Input DMA control register

// DMA parameter register

// Gathering Extension register

// FIMC Clock Source Select register

// SYSREG for FIMC writeback

pub const SYSREG_FIMD0WB_DEST_SHIFT: c_int = 23;
