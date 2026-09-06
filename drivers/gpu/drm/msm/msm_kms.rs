//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/msm_kms.h
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

pub const MAX_PLANE: c_int = 4;
// As there are different display controller blocks depending on the
// snapdragon version, the kms support is split out and the appropriate
// implementation is loaded at runtime.  The kms module is responsible
// for constructing the appropriate planes/crtcs/encoders/connectors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_kms_funcs {
// hw initialization:
    pub kms): *mut *mut int (hw_init)(struct msm_kms,
// irq handling:
    pub kms): *mut *mut void (irq_preinstall)(struct msm_kms,
    pub kms): *mut *mut int (irq_postinstall)(struct msm_kms,
    pub kms): *mut *mut void (irq_uninstall)(struct msm_kms,
    pub kms): *mut *mut irqreturn_t (irq)(struct msm_kms,
    pub crtc): *mut *mut *mut int (enable_vblank)(struct msm_kms kms, struct drm_crtc,
    pub crtc): *mut *mut *mut void (disable_vblank)(struct msm_kms kms, struct drm_crtc,
//
// Atomic commit handling:
//
// Note that in the case of async commits, the funcs which take
// a crtc_mask (ie. ->flush_commit(), and ->complete_commit())
// might not be evenly balanced with ->prepare_commit(), however
// each crtc that effected by a ->prepare_commit() (potentially
// multiple times) will eventually (at end of vsync period) be
// flushed and completed.
//
// This has some implications about tracking of cleanup state,
// for example SMP blocks to release after commit completes.  Ie.
// cleanup state should be also duplicated in the various
// duplicate_state() methods, as the current cleanup state at
// ->complete_commit() time may have accumulated cleanup work
// from multiple commits.
//
// Enable/disable power/clks needed for hw access done in other
// commit related methods.
//
// If mdp4 is migrated to runpm, we could probably drop these
// and use runpm directly.
//
    pub kms): *mut *mut void (enable_commit)(struct msm_kms,
    pub kms): *mut *mut void (disable_commit)(struct msm_kms,
//
// @check_mode_changed:
//
// Verify if the commit requires a full modeset on one of CRTCs.
//
    pub state): *mut *mut *mut int (check_mode_changed)(struct msm_kms kms, struct drm_atomic_commit,
//
// Prepare for atomic commit.  This is called after any previous
// (async or otherwise) commit has completed.
//
    pub state): *mut *mut *mut void (prepare_commit)(struct msm_kms kms, struct drm_atomic_commit,
//
// Flush an atomic commit.  This is called after the hardware
// updates have already been pushed down to effected planes
// crtcs/encoders/connectors.
//
    pub crtc_mask): *mut *mut *mut void (flush_commit)(struct msm_kms kms, unsigned,
//
// Wait for any in-progress flush to complete on the specified
// crtcs.  This should not block if there is no in-progress
// commit (ie. don't just wait for a vblank), as it will also
// be called before ->prepare_commit() to ensure any potential
// "async" commit has completed.
//
    pub crtc_mask): *mut *mut *mut void (wait_flush)(struct msm_kms kms, unsigned,
//
// Clean up after commit is completed.  This is called after
// ->wait_flush(), to give the backend a chance to do any
// post-commit cleanup.
//
    pub crtc_mask): *mut *mut *mut void (complete_commit)(struct msm_kms kms, unsigned,
//
// Format handling:
//
// misc:
    pub encoder): *mut drm_encoder,
// cleanup:
    pub kms): *mut *mut void (destroy)(struct msm_kms,
// snapshot:
    pub kms): *mut *mut *mut void (snapshot)(struct msm_disp_state disp_state, struct msm_kms,

// debugfs:
    pub minor): *mut *mut *mut int (debugfs_init)(struct msm_kms kms, struct drm_minor,

}

//
// A per-crtc timer for pending async atomic flushes.  Scheduled to expire
// shortly before vblank to flush pending async updates.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_pending_timer {
    pub work: msm_hrtimer_work,
    pub worker: *mut kthread_worker,
    pub kms: *mut msm_kms,
    pub crtc_idx: unsigned,
}

// Commit/Event thread specific structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_drm_thread {
    pub dev: *mut drm_device,
    pub worker: *mut kthread_worker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_kms {
    pub funcs: *const msm_kms_funcs,
    pub dev: *mut drm_device,
    pub hdmi: *mut hdmi,
    pub dsi: [*mut msm_dsi; MSM_DSI_CONTROLLER_COUNT],
    pub dp: [*mut msm_dp; MSM_DP_CONTROLLER_COUNT],
// irq number to be passed on to msm_irq_install
    pub irq: c_int,
    pub irq_requested: bool,
// rate limit the snapshot capture to once per attach
    pub fault_snapshot_capture: core::sync::atomic::AtomicI32,
// mapper-id used to request GEM buffer mapped for scanout:
    pub vm: *mut drm_gpuvm,
// disp snapshot support
    pub dump_worker: *mut kthread_worker,
    pub dump_work: kthread_work,
    pub dump_mutex: mutex,
//
// For async commit, where ->flush_commit() and later happens
// from the crtc's pending_timer close to end of the frame:
//
    pub commit_lock: [mutex; MAX_CRTCS],
    pub pending_crtc_mask: unsigned,
    pub pending_timers: [msm_pending_timer; MAX_CRTCS],
    pub wq: *mut workqueue_struct,
    pub event_thread: [msm_drm_thread; MAX_CRTCS],
}

extern "C" {
    pub fn msm_drm_kms_init(dev: *mut device, drv: *const drm_driver) -> c_int;
}
extern "C" {
    pub fn msm_drm_kms_post_init(dev: *mut device);
}
extern "C" {
    pub fn msm_drm_kms_unregister(dev: *mut device);
}
extern "C" {
    pub fn msm_drm_kms_uninit(dev: *mut device);
}

