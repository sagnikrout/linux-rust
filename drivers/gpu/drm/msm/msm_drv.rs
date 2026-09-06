//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/msm_drv.h
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
// Copyright (c) 2016-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

pub const MAX_CRTCS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msm_dp_controller {
    MSM_DP_CONTROLLER_0,
    MSM_DP_CONTROLLER_1,
    MSM_DP_CONTROLLER_2,
    MSM_DP_CONTROLLER_3,
    MSM_DP_CONTROLLER_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msm_dsi_controller {
    MSM_DSI_CONTROLLER_0,
    MSM_DSI_CONTROLLER_1,
    MSM_DSI_CONTROLLER_COUNT,
}

pub const MSM_GPU_MAX_RINGS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_drm_private {
    pub dev: *mut drm_device,
    pub kms: *mut msm_kms,
    pub dev): *mut *mut int (kms_init)(struct drm_device,
// subordinate devices, if present:
    pub gpu_pdev: *mut platform_device,
// when we have more than one 'msm_gpu' these need to be an array:
    pub gpu: *mut msm_gpu,
// gpu is only set on open(), but we need this info earlier
    pub is_a2xx: bool,
    pub has_cached_coherent: bool,
    pub /: *mut *mut *mut msm_rd_state rd; / debugfs to dump all submits,
    pub /: *mut *mut *mut msm_rd_state hangrd; / debugfs to dump hanging submits,
//
// total_mem: Total/global amount of memory backing GEM objects.
//
    pub total_mem: core::sync::atomic::AtomicI64,
//
// List of all GEM objects (mainly for debugfs, protected by obj_lock
// (acquire before per GEM object lock)
//
    pub objects: list_head,
    pub obj_lock: mutex,
//
// lru:
//
// The various LRU's that a GEM object is in at various stages of
// it's lifetime.  Objects start out in the unbacked LRU.  When
// pinned (for scannout or permanently mapped GPU buffers, like
// ringbuffer, memptr, fw, etc) it moves to the pinned LRU.  When
// unpinned, it moves into willneed or dontneed LRU depending on
// madvise state.  When backing pages are evicted (willneed) or
// purged (dontneed) it moves back into the unbacked LRU.
//
// The dontneed LRU is considered by the shrinker for objects
// that are candidate for purging, and the willneed LRU is
// considered for objects that could be evicted.
//
// unbacked:
//
// The LRU for GEM objects without backing pages allocated.
// This mostly exists so that objects are always is one
// LRU.
//
    pub unbacked: drm_gem_lru,
//
// pinned:
//
// The LRU for pinned GEM objects
//
    pub pinned: drm_gem_lru,
//
// willneed:
//
// The LRU for unpinned GEM objects which are in madvise
// WILLNEED state (ie. can be evicted)
//
    pub willneed: drm_gem_lru,
//
// dontneed:
//
// The LRU for unpinned GEM objects which are in madvise
// DONTNEED state (ie. can be purged)
//
    pub dontneed: drm_gem_lru,
    pub lru: },
    pub vmap_notifier: notifier_block,
    pub shrinker: *mut shrinker,
//
// hangcheck_period: For hang detection, in ms
//
// Note that in practice, a submit/job will get at least two hangcheck
// periods, due to checking for progress being implemented as simply
// "have the CP position registers changed since last time?"
//
    pub hangcheck_period: c_uint,
// gpu_devfreq_config: Devfreq tuning config for the GPU.
    pub gpu_devfreq_config: devfreq_simple_ondemand_data,
//
// gpu_clamp_to_idle: Enable clamping to idle freq when inactive
//
    pub gpu_clamp_to_idle: bool,
//
// disable_err_irq:
//
// Disable handling of GPU hw error interrupts, to force fallback to
// sw hangcheck timer.  Written (via debugfs) by igt tests to test
// the sw hangcheck mechanism.
//
    pub disable_err_irq: bool,
//
// @fault_stall_lock:
//
// Serialize changes to stall-on-fault state.
//
    pub fault_stall_lock: spinlock_t,
//
// @fault_stall_reenable_time:
//
// If stall_enabled is false, when to reenable stall-on-fault.
// Protected by @fault_stall_lock.
//
    pub stall_reenable_time: ktime_t,
//
// @stall_enabled:
//
// Whether stall-on-fault is currently enabled. Protected by
// @fault_stall_lock.
//
    pub stall_enabled: bool,
}

extern "C" {
    pub fn msm_atomic_destroy_pending_timer(timer: *mut msm_pending_timer);
}
extern "C" {
    pub fn msm_atomic_commit_tail(state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn msm_atomic_check(dev: *mut drm_device, state: *mut drm_atomic_commit) -> c_int;
}
extern "C" {
    pub fn msm_crtc_enable_vblank(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn msm_crtc_disable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn msm_register_mmu(dev: *mut drm_device, mmu: *mut msm_mmu) -> c_int;
}
extern "C" {
    pub fn msm_unregister_mmu(dev: *mut drm_device, mmu: *mut msm_mmu);
}
extern "C" {
    pub fn msm_use_mmu(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn msm_perfcntr_resume(gpu: *mut msm_gpu) -> c_int;
}
extern "C" {
    pub fn msm_perfcntr_suspend(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn msm_perfcntr_init(gpu: *mut msm_gpu) -> *mut msm_perfcntr_state;
}
extern "C" {
    pub fn msm_perfcntr_cleanup(gpu: *mut msm_gpu);
}

extern "C" {
    pub fn msm_gem_shrinker_shrink(dev: *mut drm_device, nr_to_scan: c_ulong) -> c_ulong;
}

extern "C" {
    pub fn msm_gem_shrinker_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn msm_gem_shrinker_cleanup(dev: *mut drm_device);
}
extern "C" {
    pub fn msm_gem_prime_vmap(obj: *mut drm_gem_object, map: *mut iosys_map) -> c_int;
}
extern "C" {
    pub fn msm_gem_prime_vunmap(obj: *mut drm_gem_object, map: *mut iosys_map);
}
extern "C" {
    pub fn msm_gem_prime_pin(obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn msm_gem_prime_unpin(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_framebuffer_prepare(fb: *mut drm_framebuffer, needs_dirtyfb: bool) -> c_int;
}
extern "C" {
    pub fn msm_framebuffer_cleanup(fb: *mut drm_framebuffer, needed_dirtyfb: bool);
}
extern "C" {
    pub fn msm_framebuffer_iova(fb: *mut drm_framebuffer, plane: c_int) -> u32;
}

extern "C" {
    pub fn msm_hdmi_register() -> void __init;
}
extern "C" {
    pub fn msm_hdmi_unregister() -> void __exit;
}

extern "C" {
    pub fn dsi_dev_attach(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn dsi_dev_detach(pdev: *mut platform_device);
}
extern "C" {
    pub fn msm_dsi_register() -> void __init;
}
extern "C" {
    pub fn msm_dsi_unregister() -> void __exit;
}
extern "C" {
    pub fn msm_dsi_snapshot(disp_state: *mut msm_disp_state, msm_dsi: *mut msm_dsi);
}
extern "C" {
    pub fn msm_dsi_is_cmd_mode(msm_dsi: *mut msm_dsi) -> bool;
}
extern "C" {
    pub fn msm_dsi_is_bonded_dsi(msm_dsi: *mut msm_dsi) -> bool;
}
extern "C" {
    pub fn msm_dsi_is_master_dsi(msm_dsi: *mut msm_dsi) -> bool;
}
extern "C" {
    pub fn msm_dsi_wide_bus_enabled(msm_dsi: *mut msm_dsi) -> bool;
}

extern "C" {
    pub fn msm_dp_register() -> int __init;
}
extern "C" {
    pub fn msm_dp_unregister() -> void __exit;
}
extern "C" {
    pub fn msm_dp_snapshot(disp_state: *mut msm_disp_state, dp_display: *mut msm_dp);
}
extern "C" {
    pub fn msm_dp_wide_bus_available(dp_display: *const msm_dp) -> bool;
}

extern "C" {
    pub fn msm_mdp4_register();
}
extern "C" {
    pub fn msm_mdp4_unregister();
}

extern "C" {
    pub fn msm_mdp_register();
}
extern "C" {
    pub fn msm_mdp_unregister();
}

extern "C" {
    pub fn msm_dpu_register();
}
extern "C" {
    pub fn msm_dpu_unregister();
}

extern "C" {
    pub fn msm_mdss_register();
}
extern "C" {
    pub fn msm_mdss_unregister();
}

extern "C" {
    pub fn msm_framebuffer_describe(fb: *mut drm_framebuffer, m: *mut seq_file);
}
extern "C" {
    pub fn msm_debugfs_late_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn msm_rd_debugfs_init(minor: *mut drm_minor) -> c_int;
}
extern "C" {
    pub fn msm_rd_debugfs_cleanup(priv: *mut msm_drm_private);
}

//
// struct msm_hrtimer_work - a helper to combine an hrtimer with kthread_work
//
// @timer: hrtimer to control when the kthread work is triggered
// @work:  the kthread work
// @worker: the kthread worker the work will be scheduled on
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_hrtimer_work {
    pub timer: hrtimer,
    pub work: kthread_work,
    pub worker: *mut kthread_worker,
}

// Helper for returning a UABI error with optional logging which can make
// it easier for userspace to understand what it is doing wrong.
//

// for the generated headers:

// for conditionally setting boolean flag(s):

extern "C" {
    pub fn clamp(_arg: remaining_jiffies, _arg: 1LL, _arg: (s64)INT_MAX) -> return;
}
// Driver helpers
extern "C" {
    pub fn msm_kms_pm_prepare(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn msm_kms_pm_complete(dev: *mut device);
}
extern "C" {
    pub fn msm_kms_shutdown(pdev: *mut platform_device);
}
extern "C" {
    pub fn msm_disp_drv_should_bind(dev: *mut device, dpu_driver: bool) -> bool;
}
extern "C" {
    pub fn msm_gpu_no_components() -> bool;
}
