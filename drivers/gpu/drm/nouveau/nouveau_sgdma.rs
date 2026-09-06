//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nouveau_sgdma.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_sgdma_be {
// this has to be the first field so populate/unpopulated in
// nouve_bo.c works properly, otherwise have to move them here
//
    pub ttm: ttm_tt,
    pub mem: *mut nouveau_mem,
}

    void
    nouveau_sgdma_destroy(struct ttm_device *bdev, struct ttm_tt *ttm)
    {
    struct nouveau_sgdma_be *nvbe = (struct nouveau_sgdma_be *)ttm;
    if (ttm) {
    ttm_tt_fini(&nvbe.ttm);
    kfree(nvbe);
    }
    }
    int
    nouveau_sgdma_bind(struct ttm_device *bdev, struct ttm_tt *ttm, struct ttm_resource *reg)
    {
    struct nouveau_sgdma_be *nvbe = (struct nouveau_sgdma_be *)ttm;
    struct nouveau_drm *drm = nouveau_bdev(bdev);
    struct nouveau_mem *mem = nouveau_mem(reg);
    int ret;
    if (nvbe.mem)
    return 0;
    ret = nouveau_mem_host(reg, &nvbe.ttm);
    if (ret)
    return ret;
    if (drm.client.device.info.family < NV_DEVICE_INFO_V0_TESLA) {
    ret = nouveau_mem_map(mem, &drm.client.vmm.vmm, &mem.vma[0]);
    if (ret) {
    nouveau_mem_fini(mem);
    return ret;
    }
    }
    nvbe.mem = mem;
    return 0;
    }
    void
    nouveau_sgdma_unbind(struct ttm_device *bdev, struct ttm_tt *ttm)
    {
    struct nouveau_sgdma_be *nvbe = (struct nouveau_sgdma_be *)ttm;
    if (nvbe.mem) {
    nouveau_mem_fini(nvbe.mem);
    nvbe.mem = core::ptr::null_mut();
    }
    }
    struct ttm_tt *
    nouveau_sgdma_create_ttm(struct ttm_buffer_object *bo, uint32_t page_flags)
    {
    struct nouveau_drm *drm = nouveau_bdev(bo.bdev);
    struct nouveau_bo *nvbo = nouveau_bo(bo);
    struct nouveau_sgdma_be *nvbe;
    enum ttm_caching caching;
    if (nvbo.force_coherent || drm.agp.bridge)
    caching = ttm_write_combined;
    else
    caching = ttm_cached;
    nvbe = kzalloc_obj(*nvbe);
    if (!nvbe)
    return core::ptr::null_mut();
    if (ttm_sg_tt_init(&nvbe.ttm, bo, page_flags, caching)) {
    kfree(nvbe);
    return core::ptr::null_mut();
    }
    return &nvbe.ttm;
    }
