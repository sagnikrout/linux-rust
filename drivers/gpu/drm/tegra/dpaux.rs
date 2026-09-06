//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/dpaux.h
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
//
// Copyright (C) 2013 NVIDIA Corporation
//
pub const DPAUX_CTXSW: c_uint = 0x00;
pub const DPAUX_INTR_EN_AUX: c_uint = 0x01;
pub const DPAUX_INTR_AUX: c_uint = 0x05;

pub const DPAUX_DP_AUXADDR: c_uint = 0x29;
pub const DPAUX_DP_AUXCTL: c_uint = 0x2d;

pub const DPAUX_DP_AUXSTAT: c_uint = 0x31;

pub const DPAUX_DP_AUX_SINKSTAT_LO: c_uint = 0x35;
pub const DPAUX_DP_AUX_SINKSTAT_HI: c_uint = 0x39;
pub const DPAUX_HPD_CONFIG: c_uint = 0x3d;

pub const DPAUX_HPD_IRQ_CONFIG: c_uint = 0x41;

pub const DPAUX_DP_AUX_CONFIG: c_uint = 0x45;
pub const DPAUX_HYBRID_PADCTL: c_uint = 0x49;

pub const DPAUX_HYBRID_SPARE: c_uint = 0x4d;

pub const DPAUX_SCRATCH_REG0: c_uint = 0x51;
pub const DPAUX_SCRATCH_REG1: c_uint = 0x55;
pub const DPAUX_SCRATCH_REG2: c_uint = 0x59;
