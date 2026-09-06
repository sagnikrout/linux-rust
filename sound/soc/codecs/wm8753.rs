//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8753.h
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
// wm8753.h  --  audio driver for WM8753
//
// Copyright 2003 Wolfson Microelectronics PLC.
// Author: Liam Girdwood <lrg@slimlogic.co.uk>
//
// WM8753 register space
pub const WM8753_DAC: c_uint = 0x01;
pub const WM8753_ADC: c_uint = 0x02;
pub const WM8753_PCM: c_uint = 0x03;
pub const WM8753_HIFI: c_uint = 0x04;
pub const WM8753_IOCTL: c_uint = 0x05;
pub const WM8753_SRATE1: c_uint = 0x06;
pub const WM8753_SRATE2: c_uint = 0x07;
pub const WM8753_LDAC: c_uint = 0x08;
pub const WM8753_RDAC: c_uint = 0x09;
pub const WM8753_BASS: c_uint = 0x0a;
pub const WM8753_TREBLE: c_uint = 0x0b;
pub const WM8753_ALC1: c_uint = 0x0c;
pub const WM8753_ALC2: c_uint = 0x0d;
pub const WM8753_ALC3: c_uint = 0x0e;
pub const WM8753_NGATE: c_uint = 0x0f;
pub const WM8753_LADC: c_uint = 0x10;
pub const WM8753_RADC: c_uint = 0x11;
pub const WM8753_ADCTL1: c_uint = 0x12;
pub const WM8753_3D: c_uint = 0x13;
pub const WM8753_PWR1: c_uint = 0x14;
pub const WM8753_PWR2: c_uint = 0x15;
pub const WM8753_PWR3: c_uint = 0x16;
pub const WM8753_PWR4: c_uint = 0x17;
pub const WM8753_ID: c_uint = 0x18;
pub const WM8753_INTPOL: c_uint = 0x19;
pub const WM8753_INTEN: c_uint = 0x1a;
pub const WM8753_GPIO1: c_uint = 0x1b;
pub const WM8753_GPIO2: c_uint = 0x1c;
pub const WM8753_RESET: c_uint = 0x1f;
pub const WM8753_RECMIX1: c_uint = 0x20;
pub const WM8753_RECMIX2: c_uint = 0x21;
pub const WM8753_LOUTM1: c_uint = 0x22;
pub const WM8753_LOUTM2: c_uint = 0x23;
pub const WM8753_ROUTM1: c_uint = 0x24;
pub const WM8753_ROUTM2: c_uint = 0x25;
pub const WM8753_MOUTM1: c_uint = 0x26;
pub const WM8753_MOUTM2: c_uint = 0x27;
pub const WM8753_LOUT1V: c_uint = 0x28;
pub const WM8753_ROUT1V: c_uint = 0x29;
pub const WM8753_LOUT2V: c_uint = 0x2a;
pub const WM8753_ROUT2V: c_uint = 0x2b;
pub const WM8753_MOUTV: c_uint = 0x2c;
pub const WM8753_OUTCTL: c_uint = 0x2d;
pub const WM8753_ADCIN: c_uint = 0x2e;
pub const WM8753_INCTL1: c_uint = 0x2f;
pub const WM8753_INCTL2: c_uint = 0x30;
pub const WM8753_LINVOL: c_uint = 0x31;
pub const WM8753_RINVOL: c_uint = 0x32;
pub const WM8753_MICBIAS: c_uint = 0x33;
pub const WM8753_CLOCK: c_uint = 0x34;
pub const WM8753_PLL1CTL1: c_uint = 0x35;
pub const WM8753_PLL1CTL2: c_uint = 0x36;
pub const WM8753_PLL1CTL3: c_uint = 0x37;
pub const WM8753_PLL1CTL4: c_uint = 0x38;
pub const WM8753_PLL2CTL1: c_uint = 0x39;
pub const WM8753_PLL2CTL2: c_uint = 0x3a;
pub const WM8753_PLL2CTL3: c_uint = 0x3b;
pub const WM8753_PLL2CTL4: c_uint = 0x3c;
pub const WM8753_BIASCTL: c_uint = 0x3d;
pub const WM8753_ADCTL2: c_uint = 0x3f;
pub const WM8753_PLL1: c_int = 0;
pub const WM8753_PLL2: c_int = 1;
// clock inputs
pub const WM8753_MCLK: c_int = 0;
pub const WM8753_PCMCLK: c_int = 1;
// clock divider id's
pub const WM8753_PCMDIV: c_int = 0;
pub const WM8753_BCLKDIV: c_int = 1;
pub const WM8753_VXCLKDIV: c_int = 2;
// PCM clock dividers

// BCLK clock dividers

// VXCLK clock dividers

