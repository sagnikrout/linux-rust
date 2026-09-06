//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fb/gb202.c
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
    gb202_fb_sysmem_flush_page_init(struct nvkm_fb *fb)
    {
    struct nvkm_device *device = fb.subdev.device;
    let mut addr: u64 = fb.sysmem.flush_page_addr;
// Ensure that the address is within hardware limits
    WARN_ON(fb.sysmem.flush_page_addr > DMA_BIT_MASK(52));
    nvkm_wr32(device, NV_PFB_FBHUB0_PCIE_FLUSH_SYSMEM_ADDR_HI, upper_32_bits(addr));
    nvkm_wr32(device, NV_PFB_FBHUB0_PCIE_FLUSH_SYSMEM_ADDR_LO, lower_32_bits(addr));
    }
    static const struct nvkm_fb_func
    gb202_fb = {
    .sysmem.flush_page_init = gb202_fb_sysmem_flush_page_init,
    .vidmem.size = ga102_fb_vidmem_size,
    };
    int
    gb202_fb_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst, struct nvkm_fb **pfb)
    {
    return r535_fb_new(&gb202_fb, device, type, inst, pfb);
    }
