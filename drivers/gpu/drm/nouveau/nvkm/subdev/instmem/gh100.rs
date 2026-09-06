//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/instmem/gh100.c
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

    static void
    gh100_instmem_set_bar0_window_addr(struct nvkm_device *device, u64 addr)
    {
    nvkm_wr32(device, NV_XAL_EP_BAR0_WINDOW, addr >> NV_XAL_EP_BAR0_WINDOW_BASE_SHIFT);
    }
    static const struct nvkm_instmem_func
    gh100_instmem = {
    .fini = nv50_instmem_fini,
    .memory_new = nv50_instobj_new,
    .memory_wrap = nv50_instobj_wrap,
    .set_bar0_window_addr = gh100_instmem_set_bar0_window_addr,
    };
    int
    gh100_instmem_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_instmem **pimem)
    {
    return r535_instmem_new(&gh100_instmem, device, type, inst, pimem);
    }
