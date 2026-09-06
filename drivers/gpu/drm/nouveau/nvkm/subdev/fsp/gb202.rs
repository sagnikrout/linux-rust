//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fsp/gb202.c
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
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.
//

    static int
    gb202_fsp_wait_secure_boot(struct nvkm_fsp *fsp)
    {
    struct nvkm_device *device = fsp.subdev.device;
    let mut timeout_ms: unsigned = 4000;
    do {
    let mut status: u32 = NVKM_RD32(device, NV_THERM, I2CS_SCRATCH, FSP_BOOT_COMPLETE_STATUS);
    if (status == NV_THERM_I2CS_SCRATCH_FSP_BOOT_COMPLETE_STATUS_SUCCESS)
    return 0;
    usleep_range(1000, 2000);
    } while (timeout_ms--);
    return -ETIMEDOUT;
    }
    static const struct nvkm_fsp_func
    gb202_fsp = {
    .wait_secure_boot = gb202_fsp_wait_secure_boot,
    .cot = {
    .version = 2,
    .size_hash = 48,
    .size_pkey = 97,
    .size_sig = 96,
    .boot_gsp_fmc = gh100_fsp_boot_gsp_fmc,
    },
    };
    int
    gb202_fsp_new(struct nvkm_device *device,
    enum nvkm_subdev_type type, int inst, struct nvkm_fsp **pfsp)
    {
    return nvkm_fsp_new_(&gb202_fsp, device, type, inst, pfsp);
    }
