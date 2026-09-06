//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/adreno/a3xx_catalog.c
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
// Copyright (C) 2013-2014 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//
// Copyright (c) 2014,2017 The Linux Foundation. All rights reserved.
//

    static const struct adreno_info a3xx_gpus[] = {
    {
    .chip_ids = ADRENO_CHIP_IDS(0x03000512),
    .family = ADRENO_3XX,
    .fw = {
    [ADRENO_FW_PM4] = "a330_pm4.fw",
    [ADRENO_FW_PFP] = "a330_pfp.fw",
    },
    .gmem  = SZ_128K,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a3xx_gpu_funcs,
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x03000520),
    .family = ADRENO_3XX,
    .revn  = 305,
    .fw = {
    [ADRENO_FW_PM4] = "a300_pm4.fw",
    [ADRENO_FW_PFP] = "a300_pfp.fw",
    },
    .gmem  = SZ_256K,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a3xx_gpu_funcs,
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x03000600),
    .family = ADRENO_3XX,
    .revn  = 307,        /* because a305c is revn==306 */
    .fw = {
    [ADRENO_FW_PM4] = "a300_pm4.fw",
    [ADRENO_FW_PFP] = "a300_pfp.fw",
    },
    .gmem  = SZ_128K,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a3xx_gpu_funcs,
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x03000620),
    .family = ADRENO_3XX,
    .revn = 308,
    .fw = {
    [ADRENO_FW_PM4] = "a300_pm4.fw",
    [ADRENO_FW_PFP] = "a300_pfp.fw",
    },
    .gmem = SZ_128K,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a3xx_gpu_funcs,
    }, {
    .chip_ids = ADRENO_CHIP_IDS(
    0x03020000,
    0x03020001,
    0x03020002
    ),
    .family = ADRENO_3XX,
    .revn  = 320,
    .fw = {
    [ADRENO_FW_PM4] = "a300_pm4.fw",
    [ADRENO_FW_PFP] = "a300_pfp.fw",
    },
    .gmem  = SZ_512K,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a3xx_gpu_funcs,
    }, {
    .chip_ids = ADRENO_CHIP_IDS(
    0x03030000,
    0x03030001,
    0x03030002
    ),
    .family = ADRENO_3XX,
    .revn  = 330,
    .fw = {
    [ADRENO_FW_PM4] = "a330_pm4.fw",
    [ADRENO_FW_PFP] = "a330_pfp.fw",
    },
    .gmem  = SZ_1M,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a3xx_gpu_funcs,
    }
    };
    DECLARE_ADRENO_GPULIST(a3xx);
