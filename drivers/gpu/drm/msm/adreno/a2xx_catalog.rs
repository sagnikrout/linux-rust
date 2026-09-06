//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/adreno/a2xx_catalog.c
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

    static const struct adreno_info a2xx_gpus[] = {
    {
    .chip_ids = ADRENO_CHIP_IDS(0x02000000),
    .family = ADRENO_2XX_GEN1,
    .revn  = 200,
    .fw = {
    [ADRENO_FW_PM4] = "yamato_pm4.fw",
    [ADRENO_FW_PFP] = "yamato_pfp.fw",
    },
    .gmem  = SZ_256K,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a2xx_gpu_funcs,
    }, { /* a200 on i.mx51 has only 128kib gmem */
    .chip_ids = ADRENO_CHIP_IDS(0x02000001),
    .family = ADRENO_2XX_GEN1,
    .revn  = 201,
    .fw = {
    [ADRENO_FW_PM4] = "yamato_pm4.fw",
    [ADRENO_FW_PFP] = "yamato_pfp.fw",
    },
    .gmem  = SZ_128K,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a2xx_gpu_funcs,
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x02020000),
    .family = ADRENO_2XX_GEN2,
    .revn  = 220,
    .fw = {
    [ADRENO_FW_PM4] = "leia_pm4_470.fw",
    [ADRENO_FW_PFP] = "leia_pfp_470.fw",
    },
    .gmem  = SZ_512K,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a2xx_gpu_funcs,
    }, {
// Only for msm8960v3, v2 required a special firmware
    .chip_ids = ADRENO_CHIP_IDS(0x02020506),
    .family = ADRENO_2XX_GEN2,
    .revn  = 225,
    .fw = {
    [ADRENO_FW_PM4] = "a225_pm4.fw",
    [ADRENO_FW_PFP] = "a225_pfp.fw",
    },
    .gmem  = SZ_512K,
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .funcs = &a2xx_gpu_funcs,
    }
    };
    DECLARE_ADRENO_GPULIST(a2xx);
