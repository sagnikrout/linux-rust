//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/r570/nvrm/engine.h
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
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.

// Excerpt of RM headers from https://github.com/NVIDIA/open-gpu-kernel-modules/tree/570.144

pub const MC_ENGINE_IDX_TMR: c_int = 1;
pub const MC_ENGINE_IDX_DISP: c_int = 2;
pub const MC_ENGINE_IDX_FB: c_int = 3;
pub const MC_ENGINE_IDX_FIFO: c_int = 4;
pub const MC_ENGINE_IDX_VIDEO: c_int = 5;
pub const MC_ENGINE_IDX_MD: c_int = 6;
pub const MC_ENGINE_IDX_BUS: c_int = 7;
pub const MC_ENGINE_IDX_PMGR: c_int = 8;
pub const MC_ENGINE_IDX_VP2: c_int = 9;
pub const MC_ENGINE_IDX_CIPHER: c_int = 10;
pub const MC_ENGINE_IDX_BIF: c_int = 11;
pub const MC_ENGINE_IDX_PPP: c_int = 12;
pub const MC_ENGINE_IDX_PRIVRING: c_int = 13;
pub const MC_ENGINE_IDX_PMU: c_int = 14;
pub const MC_ENGINE_IDX_CE0: c_int = 15;
pub const MC_ENGINE_IDX_CE1: c_int = 16;
pub const MC_ENGINE_IDX_CE2: c_int = 17;
pub const MC_ENGINE_IDX_CE3: c_int = 18;
pub const MC_ENGINE_IDX_CE4: c_int = 19;
pub const MC_ENGINE_IDX_CE5: c_int = 20;
pub const MC_ENGINE_IDX_CE6: c_int = 21;
pub const MC_ENGINE_IDX_CE7: c_int = 22;
pub const MC_ENGINE_IDX_CE8: c_int = 23;
pub const MC_ENGINE_IDX_CE9: c_int = 24;
pub const MC_ENGINE_IDX_CE10: c_int = 25;
pub const MC_ENGINE_IDX_CE11: c_int = 26;
pub const MC_ENGINE_IDX_CE12: c_int = 27;
pub const MC_ENGINE_IDX_CE13: c_int = 28;
pub const MC_ENGINE_IDX_CE14: c_int = 29;
pub const MC_ENGINE_IDX_CE15: c_int = 30;
pub const MC_ENGINE_IDX_CE16: c_int = 31;
pub const MC_ENGINE_IDX_CE17: c_int = 32;
pub const MC_ENGINE_IDX_CE18: c_int = 33;
pub const MC_ENGINE_IDX_CE19: c_int = 34;

pub const MC_ENGINE_IDX_VIC: c_int = 35;
pub const MC_ENGINE_IDX_ISOHUB: c_int = 36;
pub const MC_ENGINE_IDX_VGPU: c_int = 37;
pub const MC_ENGINE_IDX_NVENC: c_int = 38;
pub const MC_ENGINE_IDX_NVENC1: c_int = 39;
pub const MC_ENGINE_IDX_NVENC2: c_int = 40;
pub const MC_ENGINE_IDX_NVENC3: c_int = 41;
pub const MC_ENGINE_IDX_C2C: c_int = 42;
pub const MC_ENGINE_IDX_LTC: c_int = 43;
pub const MC_ENGINE_IDX_FBHUB: c_int = 44;
pub const MC_ENGINE_IDX_HDACODEC: c_int = 45;
pub const MC_ENGINE_IDX_GMMU: c_int = 46;
pub const MC_ENGINE_IDX_SEC2: c_int = 47;
pub const MC_ENGINE_IDX_FSP: c_int = 48;
pub const MC_ENGINE_IDX_NVLINK: c_int = 49;
pub const MC_ENGINE_IDX_GSP: c_int = 50;
pub const MC_ENGINE_IDX_NVJPG: c_int = 51;

pub const MC_ENGINE_IDX_NVJPEG1: c_int = 52;
pub const MC_ENGINE_IDX_NVJPEG2: c_int = 53;
pub const MC_ENGINE_IDX_NVJPEG3: c_int = 54;
pub const MC_ENGINE_IDX_NVJPEG4: c_int = 55;
pub const MC_ENGINE_IDX_NVJPEG5: c_int = 56;
pub const MC_ENGINE_IDX_NVJPEG6: c_int = 57;
pub const MC_ENGINE_IDX_NVJPEG7: c_int = 58;
pub const MC_ENGINE_IDX_REPLAYABLE_FAULT: c_int = 59;
pub const MC_ENGINE_IDX_ACCESS_CNTR: c_int = 60;
pub const MC_ENGINE_IDX_NON_REPLAYABLE_FAULT: c_int = 61;
pub const MC_ENGINE_IDX_REPLAYABLE_FAULT_ERROR: c_int = 62;
pub const MC_ENGINE_IDX_NON_REPLAYABLE_FAULT_ERROR: c_int = 63;
pub const MC_ENGINE_IDX_INFO_FAULT: c_int = 64;
pub const MC_ENGINE_IDX_BSP: c_int = 65;

pub const MC_ENGINE_IDX_NVDEC1: c_int = 66;
pub const MC_ENGINE_IDX_NVDEC2: c_int = 67;
pub const MC_ENGINE_IDX_NVDEC3: c_int = 68;
pub const MC_ENGINE_IDX_NVDEC4: c_int = 69;
pub const MC_ENGINE_IDX_NVDEC5: c_int = 70;
pub const MC_ENGINE_IDX_NVDEC6: c_int = 71;
pub const MC_ENGINE_IDX_NVDEC7: c_int = 72;
pub const MC_ENGINE_IDX_CPU_DOORBELL: c_int = 73;
pub const MC_ENGINE_IDX_PRIV_DOORBELL: c_int = 74;
pub const MC_ENGINE_IDX_MMU_ECC_ERROR: c_int = 75;
pub const MC_ENGINE_IDX_BLG: c_int = 76;
pub const MC_ENGINE_IDX_PERFMON: c_int = 77;
pub const MC_ENGINE_IDX_BUF_RESET: c_int = 78;
pub const MC_ENGINE_IDX_XBAR: c_int = 79;
pub const MC_ENGINE_IDX_ZPW: c_int = 80;
pub const MC_ENGINE_IDX_OFA0: c_int = 81;
pub const MC_ENGINE_IDX_OFA1: c_int = 82;
pub const MC_ENGINE_IDX_TEGRA: c_int = 83;
pub const MC_ENGINE_IDX_GR: c_int = 84;

pub const MC_ENGINE_IDX_GR1: c_int = 85;
pub const MC_ENGINE_IDX_GR2: c_int = 86;
pub const MC_ENGINE_IDX_GR3: c_int = 87;
pub const MC_ENGINE_IDX_GR4: c_int = 88;
pub const MC_ENGINE_IDX_GR5: c_int = 89;
pub const MC_ENGINE_IDX_GR6: c_int = 90;
pub const MC_ENGINE_IDX_GR7: c_int = 91;
pub const MC_ENGINE_IDX_ESCHED: c_int = 92;
pub const MC_ENGINE_IDX_ESCHED__SIZE: c_int = 64;
pub const MC_ENGINE_IDX_GR_FECS_LOG: c_int = 156;

pub const MC_ENGINE_IDX_GR1_FECS_LOG: c_int = 157;
pub const MC_ENGINE_IDX_GR2_FECS_LOG: c_int = 158;
pub const MC_ENGINE_IDX_GR3_FECS_LOG: c_int = 159;
pub const MC_ENGINE_IDX_GR4_FECS_LOG: c_int = 160;
pub const MC_ENGINE_IDX_GR5_FECS_LOG: c_int = 161;
pub const MC_ENGINE_IDX_GR6_FECS_LOG: c_int = 162;
pub const MC_ENGINE_IDX_GR7_FECS_LOG: c_int = 163;
pub const MC_ENGINE_IDX_TMR_SWRL: c_int = 164;
pub const MC_ENGINE_IDX_DISP_GSP: c_int = 165;
pub const MC_ENGINE_IDX_REPLAYABLE_FAULT_CPU: c_int = 166;
pub const MC_ENGINE_IDX_NON_REPLAYABLE_FAULT_CPU: c_int = 167;
pub const MC_ENGINE_IDX_PXUC: c_int = 168;
pub const MC_ENGINE_IDX_SYSLTC: c_int = 169;
pub const MC_ENGINE_IDX_LRCC: c_int = 170;
pub const MC_ENGINE_IDX_GSPLITE: c_int = 171;

pub const MC_ENGINE_IDX_GSPLITE1: c_int = 172;
pub const MC_ENGINE_IDX_GSPLITE2: c_int = 173;
pub const MC_ENGINE_IDX_GSPLITE3: c_int = 174;

pub const MC_ENGINE_IDX_DPAUX: c_int = 175;
pub const MC_ENGINE_IDX_DISP_LOW: c_int = 176;
pub const MC_ENGINE_IDX_MAX: c_int = 177;
// Bug 4175886 - Use this new value for all chips once GB20X is released

