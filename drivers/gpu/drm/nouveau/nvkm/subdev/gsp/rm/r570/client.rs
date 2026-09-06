//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/r570/client.c
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
    r570_gsp_client_ctor(struct nvkm_gsp_client *client, u32 handle)
    {
    NV0000_ALLOC_PARAMETERS *args;
    args = nvkm_gsp_rm_alloc_get(&client.object, handle, NV01_ROOT, sizeof(*args),
    &client.object);
    if (IS_ERR(args))
    return PTR_ERR(args);
    args.hClient = client.object.handle;
    args.processID = ~0;
    return nvkm_gsp_rm_alloc_wr(&client.object, args);
    }
    const struct nvkm_rm_api_client
    r570_client = {
    .ctor = r570_gsp_client_ctor,
    };
