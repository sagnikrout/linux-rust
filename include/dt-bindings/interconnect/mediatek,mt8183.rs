//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/mediatek,mt8183.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2021 MediaTek Inc.
// Copyright (c) 2024 Collabora Ltd.
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//
pub const SLAVE_DDR_EMI: c_int = 0;
pub const MASTER_MCUSYS: c_int = 1;
pub const MASTER_MFG: c_int = 2;
pub const MASTER_MMSYS: c_int = 3;
pub const MASTER_MM_VPU: c_int = 4;
pub const MASTER_MM_DISP: c_int = 5;
pub const MASTER_MM_VDEC: c_int = 6;
pub const MASTER_MM_VENC: c_int = 7;
pub const MASTER_MM_CAM: c_int = 8;
pub const MASTER_MM_IMG: c_int = 9;
pub const MASTER_MM_MDP: c_int = 10;
