//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/nvdec.c
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

    static void *
    nvkm_rm_nvdec_dtor(struct nvkm_engine *engine)
    {
    return container_of(engine, struct nvkm_nvdec, engine);
    }
    int
    nvkm_rm_nvdec_new(struct nvkm_rm *rm, int inst)
    {
    struct nvkm_nvdec *nvdec;
    int ret;
    nvdec = kzalloc_obj(*nvdec);
    if (!nvdec)
    return -ENOMEM;
    ret = nvkm_rm_engine_ctor(nvkm_rm_nvdec_dtor, rm, NVKM_ENGINE_NVDEC, inst,
    &rm.gpu.nvdec.class, 1, &nvdec.engine);
    if (ret) {
    kfree(nvdec);
    return ret;
    }
    rm.device.nvdec[inst] = nvdec;
    return 0;
    }
