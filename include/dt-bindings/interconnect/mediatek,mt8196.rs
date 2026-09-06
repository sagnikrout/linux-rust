//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/mediatek,mt8196.h
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
// Copyright (c) 2025 Collabora Ltd.
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//
pub const SLAVE_DDR_EMI: c_int = 0;
pub const MASTER_MCUSYS: c_int = 1;
pub const MASTER_MCU_0: c_int = 2;
pub const MASTER_MCU_1: c_int = 3;
pub const MASTER_MCU_2: c_int = 4;
pub const MASTER_MCU_3: c_int = 5;
pub const MASTER_MCU_4: c_int = 6;
pub const MASTER_GPUSYS: c_int = 7;
pub const MASTER_MMSYS: c_int = 8;
pub const MASTER_MM_VPU: c_int = 9;
pub const MASTER_MM_DISP: c_int = 10;
pub const MASTER_MM_VDEC: c_int = 11;
pub const MASTER_MM_VENC: c_int = 12;
pub const MASTER_MM_CAM: c_int = 13;
pub const MASTER_MM_IMG: c_int = 14;
pub const MASTER_MM_MDP: c_int = 15;
pub const MASTER_VPUSYS: c_int = 16;
pub const MASTER_VPU_0: c_int = 17;
pub const MASTER_VPU_1: c_int = 18;
pub const MASTER_MDLASYS: c_int = 19;
pub const MASTER_MDLA_0: c_int = 20;
pub const MASTER_UFS: c_int = 21;
pub const MASTER_PCIE: c_int = 22;
pub const MASTER_USB: c_int = 23;
pub const MASTER_WIFI: c_int = 24;
pub const MASTER_BT: c_int = 25;
pub const MASTER_NETSYS: c_int = 26;
pub const MASTER_DBGIF: c_int = 27;
pub const SLAVE_HRT_DDR_EMI: c_int = 28;
pub const MASTER_HRT_MMSYS: c_int = 29;
pub const MASTER_HRT_MM_DISP: c_int = 30;
pub const MASTER_HRT_MM_VDEC: c_int = 31;
pub const MASTER_HRT_MM_VENC: c_int = 32;
pub const MASTER_HRT_MM_CAM: c_int = 33;
pub const MASTER_HRT_MM_IMG: c_int = 34;
pub const MASTER_HRT_MM_MDP: c_int = 35;
pub const MASTER_HRT_ADSP: c_int = 36;
pub const MASTER_HRT_DBGIF: c_int = 37;
