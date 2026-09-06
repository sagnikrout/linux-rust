//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/m52790.h
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
// Input routing switch 1
pub const M52790_SW1_IN_MASK: c_uint = 0x0003;
pub const M52790_SW1_IN_TUNER: c_uint = 0x0000;
pub const M52790_SW1_IN_V2: c_uint = 0x0001;
pub const M52790_SW1_IN_V3: c_uint = 0x0002;
pub const M52790_SW1_IN_V4: c_uint = 0x0003;
// Selects component input instead of composite
pub const M52790_SW1_YCMIX: c_uint = 0x0004;
// Input routing switch 2
pub const M52790_SW2_IN_MASK: c_uint = 0x0300;
pub const M52790_SW2_IN_TUNER: c_uint = 0x0000;
pub const M52790_SW2_IN_V2: c_uint = 0x0100;
pub const M52790_SW2_IN_V3: c_uint = 0x0200;
pub const M52790_SW2_IN_V4: c_uint = 0x0300;
// Selects component input instead of composite
pub const M52790_SW2_YCMIX: c_uint = 0x0400;
// Output routing switch 1
// Enable 6dB amplifier for composite out
pub const M52790_SW1_V_AMP: c_uint = 0x0008;
// Enable 6dB amplifier for component out
pub const M52790_SW1_YC_AMP: c_uint = 0x0010;
// Audio output mode
pub const M52790_SW1_AUDIO_MASK: c_uint = 0x00c0;
pub const M52790_SW1_AUDIO_MUTE: c_uint = 0x0000;
pub const M52790_SW1_AUDIO_R: c_uint = 0x0040;
pub const M52790_SW1_AUDIO_L: c_uint = 0x0080;
pub const M52790_SW1_AUDIO_STEREO: c_uint = 0x00c0;
// Output routing switch 2
// Enable 6dB amplifier for composite out
pub const M52790_SW2_V_AMP: c_uint = 0x0800;
// Enable 6dB amplifier for component out
pub const M52790_SW2_YC_AMP: c_uint = 0x1000;
// Audio output mode
pub const M52790_SW2_AUDIO_MASK: c_uint = 0xc000;
pub const M52790_SW2_AUDIO_MUTE: c_uint = 0x0000;
pub const M52790_SW2_AUDIO_R: c_uint = 0x4000;
pub const M52790_SW2_AUDIO_L: c_uint = 0x8000;
pub const M52790_SW2_AUDIO_STEREO: c_uint = 0xc000;
// Common values

