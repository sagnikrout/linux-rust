//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/msm_gem_shrinker.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2016 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

// Default disabled for now until it has some more testing on the different
// iommu combinations that can be paired with the driver:
//
    let mut enable_eviction: static bool = true;
    MODULE_PARM_DESC(enable_eviction, "Enable swappable GEM buffers");
    module_param(enable_eviction, bool, 0600);
#[no_mangle]
unsafe extern "C" fn can_swap() -> bool {
    static bool can_swap(void)
    {
    return enable_eviction && get_nr_swap_pages() > 0;
    }
#[no_mangle]
unsafe extern "C" fn can_block(sc: *mut shrink_control) -> bool {
    static bool can_block(struct shrink_control *sc)
    {
    return (sc.gfp_mask & __GFP_DIRECT_RECLAIM) ||
    (current_is_kswapd() && (sc.gfp_mask & __GFP_KSWAPD_RECLAIM));
    }
    static unsigned long
    msm_gem_shrinker_count(struct shrinker *shrinker, struct shrink_control *sc)
    {
    struct msm_drm_private *priv = shrinker.private_data;
    let mut count: unsigned = priv.lru.dontneed.count;
    if (can_swap())
    count += priv.lru.willneed.count;
    return count;
    }
    static bool
    with_vm_locks(void (*fn)(struct drm_gem_object *obj),
    struct drm_gem_object *obj)
    {
//
// Track last locked entry for for unwinding locks in error and
// success paths
//
    struct drm_gpuvm_bo *vm_bo, *last_locked = core::ptr::null_mut();
    let mut locked: bool = true;
    drm_gem_for_each_gpuvm_bo (vm_bo, obj) {
    struct dma_resv *resv = drm_gpuvm_resv(vm_bo.vm);
    if (resv == obj.resv)
    continue;
//
// dma_resv_lock can't be used due to acquiring 'ticket' before the
// fs_reclaim lock, which is held in shrinker context
//
    if (!dma_resv_trylock(resv)) {
    locked = false;
    goto out_unlock;
    }
//
// Hold a ref to prevent the vm_bo from being freed
// and removed from the obj's gpuva list, as that would
// would result in missing the unlock below
//
    drm_gpuvm_bo_get(vm_bo);
    last_locked = vm_bo;
    }
    fn(obj);
    out_unlock:
    if (last_locked) {
    drm_gem_for_each_gpuvm_bo (vm_bo, obj) {
    struct dma_resv *resv = drm_gpuvm_resv(vm_bo.vm);
    if (resv == obj.resv)
    continue;
    dma_resv_unlock(resv);
// Drop the ref taken while locking:
    drm_gpuvm_bo_put(vm_bo);
    if (last_locked == vm_bo)
    break;
    }
    }
    return locked;
    }
    static bool
    purge(struct drm_gem_object *obj, struct ww_acquire_ctx *unused)
    {
    if (!is_purgeable(to_msm_bo(obj)))
    return false;
    if (msm_gem_active(obj))
    return false;
    return with_vm_locks(msm_gem_purge, obj);
    }
    static bool
    evict(struct drm_gem_object *obj, struct ww_acquire_ctx *unused)
    {
    if (is_unevictable(to_msm_bo(obj)))
    return false;
    if (msm_gem_active(obj))
    return false;
    return with_vm_locks(msm_gem_evict, obj);
    }
    static bool
    wait_for_idle(struct drm_gem_object *obj)
    {
    let mut usage: enum dma_resv_usage = DMA_RESV_USAGE_BOOKKEEP;
    return dma_resv_wait_timeout(obj.resv, usage, false, 10) > 0;
    }
    static bool
    active_purge(struct drm_gem_object *obj, struct ww_acquire_ctx *ticket)
    {
    if (!wait_for_idle(obj))
    return false;
    return purge(obj, ticket);
    }
    static bool
    active_evict(struct drm_gem_object *obj, struct ww_acquire_ctx *ticket)
    {
    if (!wait_for_idle(obj))
    return false;
    return evict(obj, ticket);
    }
    static unsigned long
    msm_gem_shrinker_scan(struct shrinker *shrinker, struct shrink_control *sc)
    {
    struct msm_drm_private *priv = shrinker.private_data;
    struct {
    struct drm_gem_lru *lru;
    bool (*shrink)(struct drm_gem_object *obj, struct ww_acquire_ctx *ticket);
    bool cond;
    unsigned long freed;
    unsigned long remaining;
    } stages[] = {
// Stages of progressively more aggressive/expensive reclaim:
    { &priv.lru.dontneed, purge,        true },
    { &priv.lru.willneed, evict,        can_swap() },
    { &priv.lru.dontneed, active_purge, can_block(sc) },
    { &priv.lru.willneed, active_evict, can_swap() && can_block(sc) },
    };
    let mut nr: c_long = sc.nr_to_scan;
    let mut freed: c_ulong = 0;
    let mut remaining: c_ulong = 0;
    for (unsigned i = 0; (nr > 0) && (i < ARRAY_SIZE(stages)); i++) {
    if (!stages[i].cond)
    continue;
//
// 'ticket' not needed on trylock paths
//
    stages[i].freed =
    drm_gem_lru_scan(priv.dev, stages[i].lru, nr,
    &stages[i].remaining,
    stages[i].shrink,
    core::ptr::null_mut());
    nr -= stages[i].freed;
    freed += stages[i].freed;
    remaining += stages[i].remaining;
    }
    if (freed) {
    trace_msm_gem_shrink(sc.nr_to_scan, stages[0].freed,
    stages[1].freed, stages[2].freed,
    stages[3].freed);
    }
    return (freed > 0 && remaining > 0) ? freed : SHRINK_STOP;
    }

    unsigned long
    msm_gem_shrinker_shrink(struct drm_device *dev, unsigned long nr_to_scan)
    {
    struct msm_drm_private *priv = dev.dev_private;
    struct shrink_control sc = {
    .nr_to_scan = nr_to_scan,
    };
    let mut ret: c_ulong = SHRINK_STOP;
    fs_reclaim_acquire(GFP_KERNEL);
    if (priv.shrinker)
    ret = msm_gem_shrinker_scan(priv.shrinker, &sc);
    fs_reclaim_release(GFP_KERNEL);
    return ret;
    }

// since we don't know any better, lets bail after a few
// and if necessary the shrinker will be invoked again.
// Seems better than unmapping *everything
//
    let mut vmap_shrink_limit: static int = 15;
    static bool
    vmap_shrink(struct drm_gem_object *obj, struct ww_acquire_ctx *ticket)
    {
    if (!is_vunmapable(to_msm_bo(obj)))
    return false;
    msm_gem_vunmap(obj);
    return true;
    }
    static int
    msm_gem_shrinker_vmap(struct notifier_block *nb, unsigned long event, void *ptr)
    {
    struct msm_drm_private *priv =
    container_of(nb, struct msm_drm_private, vmap_notifier);
    struct drm_gem_lru *lrus[] = {
    &priv.lru.dontneed,
    &priv.lru.willneed,
    &priv.lru.pinned,
    core::ptr::null_mut(),
    };
    unsigned idx, unmapped = 0;
    let mut remaining: c_ulong = 0;
    for (idx = 0; lrus[idx] && unmapped < vmap_shrink_limit; idx++) {
    unmapped += drm_gem_lru_scan(priv.dev, lrus[idx],
    vmap_shrink_limit - unmapped,
    &remaining,
    vmap_shrink,
    core::ptr::null_mut());
    }
// (unsigned long *)ptr += unmapped;
    if (unmapped > 0)
    trace_msm_gem_purge_vmaps(unmapped);
    return NOTIFY_DONE;
    }
//
// msm_gem_shrinker_init - Initialize msm shrinker
// @dev: drm device
//
// This function registers and sets up the msm shrinker.
//
#[no_mangle]
pub unsafe extern "C" fn msm_gem_shrinker_init(dev: *mut drm_device) -> c_int {
    int msm_gem_shrinker_init(struct drm_device *dev)
    {
    struct msm_drm_private *priv = dev.dev_private;
    priv.shrinker = shrinker_alloc(0, "drm-msm_gem");
    if (!priv.shrinker)
    return -ENOMEM;
    priv.shrinker.count_objects = msm_gem_shrinker_count;
    priv.shrinker.scan_objects = msm_gem_shrinker_scan;
    priv.shrinker.private_data = priv;
    shrinker_register(priv.shrinker);
    priv.vmap_notifier.notifier_call = msm_gem_shrinker_vmap;
    WARN_ON(register_vmap_purge_notifier(&priv.vmap_notifier));
    return 0;
    }
//
// msm_gem_shrinker_cleanup - Clean up msm shrinker
// @dev: drm device
//
// This function unregisters the msm shrinker.
//
#[no_mangle]
pub unsafe extern "C" fn msm_gem_shrinker_cleanup(dev: *mut drm_device) {
    void msm_gem_shrinker_cleanup(struct drm_device *dev)
    {
    struct msm_drm_private *priv = dev.dev_private;
    if (priv.shrinker) {
    WARN_ON(unregister_vmap_purge_notifier(&priv.vmap_notifier));
    shrinker_free(priv.shrinker);
    }
    }
