//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/gb202.c
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

    static const struct nvkm_gsp_func
    gb202_gsp = {
    .flcn = &ga102_gsp_flcn,
    .sig_section = ".fwsignature_gb20x",
    .dtor = r535_gsp_dtor,
    .oneinit = gh100_gsp_oneinit,
    .init = gh100_gsp_init,
    .fini = gh100_gsp_fini,
    .rm.gpu = &gb20x_gpu,
    };
    static struct nvkm_gsp_fwif
    gb202_gsps[] = {
    { 0, gh100_gsp_load, &gb202_gsp, &r570_rm_gb20x, "570.144" },
    {}
    };
    int
    gb202_gsp_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_gsp **pgsp)
    {
    return nvkm_gsp_new_(gb202_gsps, device, type, inst, pgsp);
    }
    NVKM_GSP_FIRMWARE_FMC(gb202, 570.144);
    NVKM_GSP_FIRMWARE_FMC(gb203, 570.144);
    NVKM_GSP_FIRMWARE_FMC(gb205, 570.144);
    NVKM_GSP_FIRMWARE_FMC(gb206, 570.144);
    NVKM_GSP_FIRMWARE_FMC(gb207, 570.144);
