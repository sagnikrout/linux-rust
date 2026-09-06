//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/r535/rm.c
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

    static const struct nvkm_rm_wpr
    r535_wpr_libos2 = {
    .os_carveout_size = GSP_FW_HEAP_PARAM_OS_SIZE_LIBOS2,
    .base_size = GSP_FW_HEAP_PARAM_BASE_RM_SIZE_TU10X,
    .heap_size_min = GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS2_MIN_MB,
    };
    static const struct nvkm_rm_wpr
    r535_wpr_libos3 = {
    .os_carveout_size = GSP_FW_HEAP_PARAM_OS_SIZE_LIBOS3,
    .base_size = GSP_FW_HEAP_PARAM_BASE_RM_SIZE_TU10X,
    .heap_size_min = GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS3_BAREMETAL_MIN_MB,
    };
    static const struct nvkm_rm_api
    r535_api = {
    .gsp = &r535_gsp,
    .rpc = &r535_rpc,
    .ctrl = &r535_ctrl,
    .alloc = &r535_alloc,
    .client = &r535_client,
    .device = &r535_device,
    .fbsr = &r535_fbsr,
    .disp = &r535_disp,
    .fifo = &r535_fifo,
    .ce = &r535_ce,
    .gr = &r535_gr,
    .nvdec = &r535_nvdec,
    .nvenc = &r535_nvenc,
    .nvjpg = &r535_nvjpg,
    .ofa = &r535_ofa,
    };
    const struct nvkm_rm_impl
    r535_rm_tu102 = {
    .wpr = &r535_wpr_libos2,
    .api = &r535_api,
    };
    const struct nvkm_rm_impl
    r535_rm_ga102 = {
    .wpr = &r535_wpr_libos3,
    .api = &r535_api,
    };
