//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/wm8766.h
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

// Macro flag: #define WM8766_H_INCLUDED
pub const WM8766_LDA1: c_uint = 0x00;
pub const WM8766_RDA1: c_uint = 0x01;
pub const WM8766_DAC_CTRL: c_uint = 0x02;
pub const WM8766_INT_CTRL: c_uint = 0x03;
pub const WM8766_LDA2: c_uint = 0x04;
pub const WM8766_RDA2: c_uint = 0x05;
pub const WM8766_LDA3: c_uint = 0x06;
pub const WM8766_RDA3: c_uint = 0x07;
pub const WM8766_MASTDA: c_uint = 0x08;
pub const WM8766_DAC_CTRL2: c_uint = 0x09;
pub const WM8766_DAC_CTRL3: c_uint = 0x0a;
pub const WM8766_MUTE1: c_uint = 0x0c;
pub const WM8766_MUTE2: c_uint = 0x0f;
pub const WM8766_RESET: c_uint = 0x1f;
// LDAx/RDAx/MASTDA
pub const WM8766_ATT_MASK: c_uint = 0x0ff;
pub const WM8766_UPDATE: c_uint = 0x100;
// DAC_CTRL
pub const WM8766_MUTEALL: c_uint = 0x001;
pub const WM8766_DEEMPALL: c_uint = 0x002;
pub const WM8766_PWDN: c_uint = 0x004;
pub const WM8766_ATC: c_uint = 0x008;
pub const WM8766_IZD: c_uint = 0x010;
pub const WM8766_PL_LEFT_MASK: c_uint = 0x060;
pub const WM8766_PL_LEFT_MUTE: c_uint = 0x000;
pub const WM8766_PL_LEFT_LEFT: c_uint = 0x020;
pub const WM8766_PL_LEFT_RIGHT: c_uint = 0x040;
pub const WM8766_PL_LEFT_LRMIX: c_uint = 0x060;
pub const WM8766_PL_RIGHT_MASK: c_uint = 0x180;
pub const WM8766_PL_RIGHT_MUTE: c_uint = 0x000;
pub const WM8766_PL_RIGHT_LEFT: c_uint = 0x080;
pub const WM8766_PL_RIGHT_RIGHT: c_uint = 0x100;
pub const WM8766_PL_RIGHT_LRMIX: c_uint = 0x180;
// INT_CTRL
pub const WM8766_FMT_MASK: c_uint = 0x003;
pub const WM8766_FMT_RJUST: c_uint = 0x000;
pub const WM8766_FMT_LJUST: c_uint = 0x001;
pub const WM8766_FMT_I2S: c_uint = 0x002;
pub const WM8766_FMT_DSP: c_uint = 0x003;
pub const WM8766_LRP: c_uint = 0x004;
pub const WM8766_BCP: c_uint = 0x008;
pub const WM8766_IWL_MASK: c_uint = 0x030;
pub const WM8766_IWL_16: c_uint = 0x000;
pub const WM8766_IWL_20: c_uint = 0x010;
pub const WM8766_IWL_24: c_uint = 0x020;
pub const WM8766_IWL_32: c_uint = 0x030;
pub const WM8766_PHASE_MASK: c_uint = 0x1c0;
// DAC_CTRL2
pub const WM8766_ZCD: c_uint = 0x001;
pub const WM8766_DZFM_MASK: c_uint = 0x006;
pub const WM8766_DMUTE_MASK: c_uint = 0x038;
pub const WM8766_DEEMP_MASK: c_uint = 0x1c0;
// DAC_CTRL3
pub const WM8766_DACPD_MASK: c_uint = 0x00e;
pub const WM8766_PWRDNALL: c_uint = 0x010;
pub const WM8766_MS: c_uint = 0x020;
pub const WM8766_RATE_MASK: c_uint = 0x1c0;
pub const WM8766_RATE_128: c_uint = 0x000;
pub const WM8766_RATE_192: c_uint = 0x040;
pub const WM8766_RATE_256: c_uint = 0x080;
pub const WM8766_RATE_384: c_uint = 0x0c0;
pub const WM8766_RATE_512: c_uint = 0x100;
pub const WM8766_RATE_768: c_uint = 0x140;
// MUTE1
pub const WM8766_MPD1: c_uint = 0x040;
// MUTE2
pub const WM8766_MPD2: c_uint = 0x020;
