//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/adreno/a5xx_catalog.c
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

    static const struct adreno_info a5xx_gpus[] = {
    {
    .chip_ids = ADRENO_CHIP_IDS(0x05000500),
    .family = ADRENO_5XX,
    .revn = 505,
    .fw = {
    [ADRENO_FW_PM4] = "a530_pm4.fw",
    [ADRENO_FW_PFP] = "a530_pfp.fw",
    },
    .gmem = (SZ_128K + SZ_8K),
    .inactive_period = DRM_MSM_INACTIVE_PERIOD,
    .quirks = ADRENO_QUIRK_TWO_PASS_USE_WFI |
    ADRENO_QUIRK_LMLOADKILL_DISABLE,
    .funcs = &a5xx_gpu_funcs,
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x05000600),
    .family = ADRENO_5XX,
    .revn = 506,
    .fw = {
    [ADRENO_FW_PM4] = "a530_pm4.fw",
    [ADRENO_FW_PFP] = "a530_pfp.fw",
    },
    .gmem = (SZ_128K + SZ_8K),
//
// Increase inactive period to 250 to avoid bouncing
// the GDSC which appears to make it grumpy
//
    .inactive_period = 250,
    .quirks = ADRENO_QUIRK_TWO_PASS_USE_WFI |
    ADRENO_QUIRK_LMLOADKILL_DISABLE,
    .funcs = &a5xx_gpu_funcs,
    .zapfw = "a506_zap.mdt",
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x05000800),
    .family = ADRENO_5XX,
    .revn = 508,
    .fw = {
    [ADRENO_FW_PM4] = "a530_pm4.fw",
    [ADRENO_FW_PFP] = "a530_pfp.fw",
    },
    .gmem = (SZ_128K + SZ_8K),
//
// Increase inactive period to 250 to avoid bouncing
// the GDSC which appears to make it grumpy
//
    .inactive_period = 250,
    .quirks = ADRENO_QUIRK_LMLOADKILL_DISABLE,
    .funcs = &a5xx_gpu_funcs,
    .zapfw = "a508_zap.mdt",
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x05000900),
    .family = ADRENO_5XX,
    .revn = 509,
    .fw = {
    [ADRENO_FW_PM4] = "a530_pm4.fw",
    [ADRENO_FW_PFP] = "a530_pfp.fw",
    },
    .gmem = (SZ_256K + SZ_16K),
//
// Increase inactive period to 250 to avoid bouncing
// the GDSC which appears to make it grumpy
//
    .inactive_period = 250,
    .quirks = ADRENO_QUIRK_LMLOADKILL_DISABLE,
    .funcs = &a5xx_gpu_funcs,
// Adreno 509 uses the same ZAP as 512
    .zapfw = "a512_zap.mdt",
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x05010000),
    .family = ADRENO_5XX,
    .revn = 510,
    .fw = {
    [ADRENO_FW_PM4] = "a530_pm4.fw",
    [ADRENO_FW_PFP] = "a530_pfp.fw",
    },
    .gmem = SZ_256K,
//
// Increase inactive period to 250 to avoid bouncing
// the GDSC which appears to make it grumpy
//
    .inactive_period = 250,
    .funcs = &a5xx_gpu_funcs,
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x05010200),
    .family = ADRENO_5XX,
    .revn = 512,
    .fw = {
    [ADRENO_FW_PM4] = "a530_pm4.fw",
    [ADRENO_FW_PFP] = "a530_pfp.fw",
    },
    .gmem = (SZ_256K + SZ_16K),
//
// Increase inactive period to 250 to avoid bouncing
// the GDSC which appears to make it grumpy
//
    .inactive_period = 250,
    .quirks = ADRENO_QUIRK_LMLOADKILL_DISABLE,
    .funcs = &a5xx_gpu_funcs,
    .zapfw = "a512_zap.mdt",
    }, {
    .chip_ids = ADRENO_CHIP_IDS(
    0x05030002,
    0x05030004
    ),
    .family = ADRENO_5XX,
    .revn = 530,
    .fw = {
    [ADRENO_FW_PM4] = "a530_pm4.fw",
    [ADRENO_FW_PFP] = "a530_pfp.fw",
    [ADRENO_FW_GPMU] = "a530v3_gpmu.fw2",
    },
    .gmem = SZ_1M,
//
// Increase inactive period to 250 to avoid bouncing
// the GDSC which appears to make it grumpy
//
    .inactive_period = 250,
    .quirks = ADRENO_QUIRK_TWO_PASS_USE_WFI |
    ADRENO_QUIRK_FAULT_DETECT_MASK,
    .funcs = &a5xx_gpu_funcs,
    .zapfw = "a530_zap.mdt",
    }, {
    .chip_ids = ADRENO_CHIP_IDS(0x05040001),
    .family = ADRENO_5XX,
    .revn = 540,
    .fw = {
    [ADRENO_FW_PM4] = "a530_pm4.fw",
    [ADRENO_FW_PFP] = "a530_pfp.fw",
    [ADRENO_FW_GPMU] = "a540_gpmu.fw2",
    },
    .gmem = SZ_1M,
//
// Increase inactive period to 250 to avoid bouncing
// the GDSC which appears to make it grumpy
//
    .inactive_period = 250,
    .quirks = ADRENO_QUIRK_LMLOADKILL_DISABLE,
    .funcs = &a5xx_gpu_funcs,
    .zapfw = "a540_zap.mdt",
    }
    };
    DECLARE_ADRENO_GPULIST(a5xx);
