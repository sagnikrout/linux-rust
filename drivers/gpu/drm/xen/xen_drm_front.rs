//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xen/xen_drm_front.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Xen para-virtual DRM device
//
// Copyright (C) 2016-2018 EPAM Systems Inc.
//
// Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
//

//
// DOC: Driver modes of operation in terms of display buffers used
//
// Depending on the requirements for the para-virtualized environment, namely
// requirements dictated by the accompanying DRM/(v)GPU drivers running in both
// host and guest environments, display buffers can be allocated by either
// frontend driver or backend.
//
// DOC: Buffers allocated by the frontend driver
//
// In this mode of operation driver allocates buffers from system memory.
//
// Note! If used with accompanying DRM/(v)GPU drivers this mode of operation
// may require IOMMU support on the platform, so accompanying DRM/vGPU
// hardware can still reach display buffer memory while importing PRIME
// buffers from the frontend driver.
//
// DOC: Buffers allocated by the backend
//
// This mode of operation is run-time configured via guest domain configuration
// through XenStore entries.
//
// For systems which do not provide IOMMU support, but having specific
// requirements for display buffers it is possible to allocate such buffers
// at backend side and share those with the frontend.
// For example, if host domain is 1:1 mapped and has DRM/GPU hardware expecting
// physically contiguous memory, this allows implementing zero-copying
// use-cases.
//
// Note, while using this scenario the following should be considered:
//
// #. If guest domain dies then pages/grants received from the backend
// cannot be claimed back
//
// #. Misbehaving guest may send too many requests to the
// backend exhausting its grant references and memory
// (consider this from security POV)
//
// DOC: Driver limitations
//
// #. Only primary plane without additional properties is supported.
//
// #. Only one video mode per connector supported which is configured
// via XenStore.
//
// #. All CRTCs operate at fixed frequency of 60Hz.
//
// timeout in ms to wait for backend to respond
pub const XEN_DRM_FRONT_WAIT_BACK_MS: c_int = 3000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_drm_front_info {
    pub xb_dev: *mut xenbus_device,
    pub drm_info: *mut xen_drm_front_drm_info,
// to protect data between backend IO code and interrupt handler
    pub io_lock: spinlock_t,
    pub num_evt_pairs: c_int,
    pub evt_pairs: *mut xen_drm_front_evtchnl_pair,
    pub cfg: xen_drm_front_cfg,
// display buffers
    pub dbuf_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_drm_front_drm_pipeline {
    pub drm_info: *mut xen_drm_front_drm_info,
    pub index: c_int,
    pub pipe: drm_simple_display_pipe,
    pub conn: drm_connector,
// These are only for connector mode checking
    pub height: int width,,
    pub pending_event: *mut drm_pending_vblank_event,
    pub pflip_to_worker: delayed_work,
    pub conn_connected: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_drm_front_drm_info {
    pub front_info: *mut xen_drm_front_info,
    pub drm_dev: *mut drm_device,
    pub pipeline: [xen_drm_front_drm_pipeline; XEN_DRM_FRONT_MAX_CRTCS],
}

extern "C" {
    pub fn xen_drm_front_gem_object_free(obj: *mut drm_gem_object);
}
