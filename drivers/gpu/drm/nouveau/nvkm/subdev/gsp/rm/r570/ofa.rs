//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/r570/ofa.c
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
    r570_ofa_alloc(struct nvkm_gsp_object *parent, u32 handle, u32 oclass, int inst,
    struct nvkm_gsp_object *ofa)
    {
    NV_OFA_ALLOCATION_PARAMETERS *args;
    args = nvkm_gsp_rm_alloc_get(parent, handle, oclass, sizeof(*args), ofa);
    if (WARN_ON(IS_ERR(args)))
    return PTR_ERR(args);
    args.size = sizeof(*args);
    args.engineInstance = inst;
    return nvkm_gsp_rm_alloc_wr(ofa, args);
    }
    const struct nvkm_rm_api_engine
    r570_ofa = {
    .alloc = r570_ofa_alloc,
    };
