//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panfrost/panfrost_gem_shrinker.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Arm Ltd.
//
// Based on msm_gem_freedreno.c:
// Copyright (C) 2016 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

    static unsigned long
    panfrost_gem_shrinker_count(struct shrinker *shrinker, struct shrink_control *sc)
    {
    struct panfrost_device *pfdev = shrinker.private_data;
    struct drm_gem_shmem_object *shmem;
    let mut count: c_ulong = 0;
    if (!mutex_trylock(&pfdev.shrinker_lock))
    return 0;
    list_for_each_entry(shmem, &pfdev.shrinker_list, madv_list) {
    if (drm_gem_shmem_is_purgeable(shmem))
    count += shmem.base.size >> PAGE_SHIFT;
    }
    mutex_unlock(&pfdev.shrinker_lock);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn panfrost_gem_purge(obj: *mut drm_gem_object) -> bool {
    static bool panfrost_gem_purge(struct drm_gem_object *obj)
    {
    struct drm_gem_shmem_object *shmem = to_drm_gem_shmem_obj(obj);
    struct panfrost_gem_object *bo = to_panfrost_bo(obj);
    let mut ret: bool = false;
    if (atomic_read(&bo.gpu_usecount))
    return false;
    if (!mutex_trylock(&bo.mappings.lock))
    return false;
    if (!dma_resv_trylock(shmem.base.resv))
    goto unlock_mappings;
    panfrost_gem_teardown_mappings_locked(bo);
    drm_gem_shmem_purge_locked(&bo.base);
    ret = true;
    dma_resv_unlock(shmem.base.resv);
    unlock_mappings:
    mutex_unlock(&bo.mappings.lock);
    return ret;
    }
    static unsigned long
    panfrost_gem_shrinker_scan(struct shrinker *shrinker, struct shrink_control *sc)
    {
    struct panfrost_device *pfdev = shrinker.private_data;
    struct drm_gem_shmem_object *shmem, *tmp;
    let mut freed: c_ulong = 0;
    if (!mutex_trylock(&pfdev.shrinker_lock))
    return SHRINK_STOP;
    list_for_each_entry_safe(shmem, tmp, &pfdev.shrinker_list, madv_list) {
    if (freed >= sc.nr_to_scan)
    break;
    if (drm_gem_shmem_is_purgeable(shmem) &&
    panfrost_gem_purge(&shmem.base)) {
    freed += shmem.base.size >> PAGE_SHIFT;
    list_del_init(&shmem.madv_list);
    }
    }
    mutex_unlock(&pfdev.shrinker_lock);
    if (freed > 0)
    pr_info_ratelimited("Purging %lu bytes\n", freed << PAGE_SHIFT);
    return freed;
    }
//
// panfrost_gem_shrinker_init - Initialize panfrost shrinker
// @dev: DRM device
//
// This function registers and sets up the panfrost shrinker.
//
#[no_mangle]
pub unsafe extern "C" fn panfrost_gem_shrinker_init(dev: *mut drm_device) -> c_int {
    int panfrost_gem_shrinker_init(struct drm_device *dev)
    {
    struct panfrost_device *pfdev = to_panfrost_device(dev);
    pfdev.shrinker = shrinker_alloc(0, "drm-panfrost");
    if (!pfdev.shrinker)
    return -ENOMEM;
    pfdev.shrinker.count_objects = panfrost_gem_shrinker_count;
    pfdev.shrinker.scan_objects = panfrost_gem_shrinker_scan;
    pfdev.shrinker.private_data = pfdev;
    shrinker_register(pfdev.shrinker);
    return 0;
    }
//
// panfrost_gem_shrinker_cleanup - Clean up panfrost shrinker
// @dev: DRM device
//
// This function unregisters the panfrost shrinker.
//
#[no_mangle]
pub unsafe extern "C" fn panfrost_gem_shrinker_cleanup(dev: *mut drm_device) {
    void panfrost_gem_shrinker_cleanup(struct drm_device *dev)
    {
    struct panfrost_device *pfdev = to_panfrost_device(dev);
    if (pfdev.shrinker)
    shrinker_free(pfdev.shrinker);
    }
