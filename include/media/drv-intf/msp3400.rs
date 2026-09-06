//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/msp3400.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// msp3400 routing
//
// Inputs to the DSP unit: two independent selections have to be made:
//
// SCART input to DSP selection

// Tuner input to demodulator and DSP selection

// The msp has up to 5 DSP outputs, each output can independently select

// Output SCART select: the SCART outputs can select which input

// Shortcut macros

// This equals the RESET position of the msp3400 ACB register

// Tuner inputs vs. msp version
// Chip      TUNER_1   TUNER_2
//
// SCART inputs vs. msp version
// Chip      SC1 SC2 SC3 SC4
//
// DSP inputs vs. msp version (tuner and SCART inputs are always available)
// Chip      I2S1 I2S2 I2S3 MAIN_AVC MAIN AUX
//
// DSP outputs vs. msp version
// Chip      MAIN AUX SCART1 SCART2 I2S
//
