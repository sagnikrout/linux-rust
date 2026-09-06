//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/amlogic,meson-g12a-audio-reset.h
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
// Copyright (c) 2019 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//
pub const AUD_RESET_PDM: c_int = 0;
pub const AUD_RESET_TDMIN_A: c_int = 1;
pub const AUD_RESET_TDMIN_B: c_int = 2;
pub const AUD_RESET_TDMIN_C: c_int = 3;
pub const AUD_RESET_TDMIN_LB: c_int = 4;
pub const AUD_RESET_LOOPBACK: c_int = 5;
pub const AUD_RESET_TODDR_A: c_int = 6;
pub const AUD_RESET_TODDR_B: c_int = 7;
pub const AUD_RESET_TODDR_C: c_int = 8;
pub const AUD_RESET_FRDDR_A: c_int = 9;
pub const AUD_RESET_FRDDR_B: c_int = 10;
pub const AUD_RESET_FRDDR_C: c_int = 11;
pub const AUD_RESET_TDMOUT_A: c_int = 12;
pub const AUD_RESET_TDMOUT_B: c_int = 13;
pub const AUD_RESET_TDMOUT_C: c_int = 14;
pub const AUD_RESET_SPDIFOUT: c_int = 15;
pub const AUD_RESET_SPDIFOUT_B: c_int = 16;
pub const AUD_RESET_SPDIFIN: c_int = 17;
pub const AUD_RESET_EQDRC: c_int = 18;
pub const AUD_RESET_RESAMPLE: c_int = 19;
pub const AUD_RESET_DDRARB: c_int = 20;
pub const AUD_RESET_POWDET: c_int = 21;
pub const AUD_RESET_TORAM: c_int = 22;
pub const AUD_RESET_TOACODEC: c_int = 23;
pub const AUD_RESET_TOHDMITX: c_int = 24;
pub const AUD_RESET_CLKTREE: c_int = 25;
// SM1 added resets
pub const AUD_RESET_RESAMPLE_B: c_int = 26;
pub const AUD_RESET_TOVAD: c_int = 27;
pub const AUD_RESET_LOCKER: c_int = 28;
pub const AUD_RESET_SPDIFIN_LB: c_int = 29;
pub const AUD_RESET_FRATV: c_int = 30;
pub const AUD_RESET_FRHDMIRX: c_int = 31;
pub const AUD_RESET_FRDDR_D: c_int = 32;
pub const AUD_RESET_TODDR_D: c_int = 33;
pub const AUD_RESET_LOOPBACK_B: c_int = 34;
pub const AUD_RESET_EARCTX: c_int = 35;
pub const AUD_RESET_EARCRX: c_int = 36;
pub const AUD_RESET_FRDDR_E: c_int = 37;
pub const AUD_RESET_TODDR_E: c_int = 38;
