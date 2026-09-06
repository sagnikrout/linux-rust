//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_fdi_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

pub const FDI_PLL_FB_CLOCK_MASK: c_uint = 0xff;

pub const FDI_PLL_FREQ_LOCK_LIMIT_MASK: c_uint = 0xfff00;
pub const FDI_PLL_FREQ_DISABLE_COUNT_LIMIT_MASK: c_uint = 0xff;
pub const _FDI_RXA_CHICKEN: c_uint = 0xc200c;
pub const _FDI_RXB_CHICKEN: c_uint = 0xc2010;

// CPU: FDI_TX
pub const _FDI_TXA_CTL: c_uint = 0x60100;
pub const _FDI_TXB_CTL: c_uint = 0x61100;

// ILK always use 400mV 0dB for voltage swing and pre-emphasis level.
// SNB A-stepping

// SNB B-stepping

pub const FDI_DP_PORT_WIDTH_SHIFT: c_int = 19;

// Ironlake: hardwired to 1

// Ivybridge has different bits for lolz

// both Tx and Rx

// FDI_RX, FDI_X is hard-wired to Transcoder_X
pub const _FDI_RXA_CTL: c_uint = 0xf000c;
pub const _FDI_RXB_CTL: c_uint = 0xf100c;

// train, dp width same as FDI_TX

// CPT

pub const _FDI_RXA_MISC: c_uint = 0xf0010;
pub const _FDI_RXB_MISC: c_uint = 0xf1010;

pub const _FDI_RXA_TUSIZE1: c_uint = 0xf0030;
pub const _FDI_RXA_TUSIZE2: c_uint = 0xf0038;
pub const _FDI_RXB_TUSIZE1: c_uint = 0xf1030;
pub const _FDI_RXB_TUSIZE2: c_uint = 0xf1038;

// FDI_RX interrupt register format

pub const _FDI_RXA_IIR: c_uint = 0xf0014;
pub const _FDI_RXA_IMR: c_uint = 0xf0018;
pub const _FDI_RXB_IIR: c_uint = 0xf1014;
pub const _FDI_RXB_IMR: c_uint = 0xf1018;

