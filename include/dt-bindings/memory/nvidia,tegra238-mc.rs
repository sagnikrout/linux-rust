//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/nvidia,tegra238-mc.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Copyright (c) 2026, NVIDIA CORPORATION. All rights reserved.
// special clients
pub const TEGRA238_SID_INVALID: c_uint = 0x0;
pub const TEGRA238_SID_PASSTHROUGH: c_uint = 0x7f;
// ISO stream IDs
pub const TEGRA238_SID_ISO_NVDISPLAY: c_uint = 0x1;
pub const TEGRA238_SID_ISO_APE0: c_uint = 0x2;
pub const TEGRA238_SID_ISO_APE1: c_uint = 0x3;
// NISO stream IDs
pub const TEGRA238_SID_AON: c_uint = 0x1;
pub const TEGRA238_SID_BPMP: c_uint = 0x2;
pub const TEGRA238_SID_ETR: c_uint = 0x3;
pub const TEGRA238_SID_FDE: c_uint = 0x4;
pub const TEGRA238_SID_HC: c_uint = 0x5;
pub const TEGRA238_SID_HDA: c_uint = 0x6;
pub const TEGRA238_SID_NVDEC: c_uint = 0x7;
pub const TEGRA238_SID_NVDISPLAY: c_uint = 0x8;
pub const TEGRA238_SID_NVENC: c_uint = 0x9;
pub const TEGRA238_SID_OFA: c_uint = 0xa;
pub const TEGRA238_SID_PCIE0: c_uint = 0xb;
pub const TEGRA238_SID_PCIE1: c_uint = 0xc;
pub const TEGRA238_SID_PCIE2: c_uint = 0xd;
pub const TEGRA238_SID_PCIE3: c_uint = 0xe;
pub const TEGRA238_SID_HWMP_PMA: c_uint = 0xf;
pub const TEGRA238_SID_PSC: c_uint = 0x10;
pub const TEGRA238_SID_SDMMC1A: c_uint = 0x11;
pub const TEGRA238_SID_SDMMC4A: c_uint = 0x12;
pub const TEGRA238_SID_SES_SE0: c_uint = 0x13;
pub const TEGRA238_SID_SES_SE1: c_uint = 0x14;
pub const TEGRA238_SID_SES_SE2: c_uint = 0x15;
pub const TEGRA238_SID_SEU1_SE0: c_uint = 0x16;
pub const TEGRA238_SID_SEU1_SE1: c_uint = 0x17;
pub const TEGRA238_SID_SEU1_SE2: c_uint = 0x18;
pub const TEGRA238_SID_TSEC: c_uint = 0x19;
pub const TEGRA238_SID_UFSHC: c_uint = 0x1a;
pub const TEGRA238_SID_VIC: c_uint = 0x1b;
pub const TEGRA238_SID_XUSB_HOST: c_uint = 0x1c;
pub const TEGRA238_SID_XUSB_DEV: c_uint = 0x1d;
pub const TEGRA238_SID_GPCDMA_0: c_uint = 0x1e;
pub const TEGRA238_SID_SMMU_TEST: c_uint = 0x1f;
// Host1x virtualization clients.
pub const TEGRA238_SID_HOST1X_CTX0: c_uint = 0x20;
pub const TEGRA238_SID_HOST1X_CTX1: c_uint = 0x21;
pub const TEGRA238_SID_HOST1X_CTX2: c_uint = 0x22;
pub const TEGRA238_SID_HOST1X_CTX3: c_uint = 0x23;
pub const TEGRA238_SID_HOST1X_CTX4: c_uint = 0x24;
pub const TEGRA238_SID_HOST1X_CTX5: c_uint = 0x25;
pub const TEGRA238_SID_HOST1X_CTX6: c_uint = 0x26;
pub const TEGRA238_SID_HOST1X_CTX7: c_uint = 0x27;
pub const TEGRA238_SID_XUSB_VF0: c_uint = 0x28;
pub const TEGRA238_SID_XUSB_VF1: c_uint = 0x29;
pub const TEGRA238_SID_XUSB_VF2: c_uint = 0x2a;
pub const TEGRA238_SID_XUSB_VF3: c_uint = 0x2b;
// Host1x command buffers
pub const TEGRA238_SID_HC_VM0: c_uint = 0x2c;
pub const TEGRA238_SID_HC_VM1: c_uint = 0x2d;
pub const TEGRA238_SID_HC_VM2: c_uint = 0x2e;
pub const TEGRA238_SID_HC_VM3: c_uint = 0x2f;
pub const TEGRA238_SID_HC_VM4: c_uint = 0x30;
pub const TEGRA238_SID_HC_VM5: c_uint = 0x31;
pub const TEGRA238_SID_HC_VM6: c_uint = 0x32;
pub const TEGRA238_SID_HC_VM7: c_uint = 0x33;
