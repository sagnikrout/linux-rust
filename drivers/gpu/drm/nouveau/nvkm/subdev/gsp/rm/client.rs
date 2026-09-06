//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/client.c
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

    void
    nvkm_gsp_client_dtor(struct nvkm_gsp_client *client)
    {
    let mut id: c_uint = client.object.handle - NVKM_RM_CLIENT(0);
    struct nvkm_gsp *gsp = client.gsp;
    if (!gsp)
    return;
    if (client.object.client)
    nvkm_gsp_rm_free(&client.object);
    mutex_lock(&gsp.client_id.mutex);
    idr_remove(&gsp.client_id.idr, id);
    mutex_unlock(&gsp.client_id.mutex);
    client.gsp = core::ptr::null_mut();
    }
    int
    nvkm_gsp_client_ctor(struct nvkm_gsp *gsp, struct nvkm_gsp_client *client)
    {
    int id, ret;
    if (WARN_ON(!gsp.rm))
    return -ENOSYS;
    mutex_lock(&gsp.client_id.mutex);
    id = idr_alloc(&gsp.client_id.idr, client, 0, NVKM_RM_CLIENT_MASK + 1, GFP_KERNEL);
    mutex_unlock(&gsp.client_id.mutex);
    if (id < 0)
    return id;
    client.gsp = gsp;
    client.object.client = client;
    INIT_LIST_HEAD(&client.events);
    ret = gsp.rm.api.client.ctor(client, NVKM_RM_CLIENT(id));
    if (ret)
    nvkm_gsp_client_dtor(client);
    return ret;
    }
