//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xen/xen_drm_front_evtchnl.h
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
// All operations which are not connector oriented use this ctrl event channel,
// e.g. fb_attach/destroy which belong to a DRM device, not to a CRTC.
//
pub const GENERIC_OP_EVT_CHNL: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xen_drm_front_evtchnl_state {
    EVTCHNL_STATE_DISCONNECTED,
    EVTCHNL_STATE_CONNECTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xen_drm_front_evtchnl_type {
    EVTCHNL_TYPE_REQ,
    EVTCHNL_TYPE_EVT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_drm_front_evtchnl {
    pub front_info: *mut xen_drm_front_info,
    pub gref: c_int,
    pub port: c_int,
    pub irq: c_int,
    pub index: c_int,
    pub state: xen_drm_front_evtchnl_state,
    pub type: xen_drm_front_evtchnl_type,
// either response id or incoming event id
    pub evt_id: u16,
// next request id or next expected event id
    pub evt_next_id: u16,
    pub ring: xen_displif_front_ring,
    pub completion: completion,
// latest response status
    pub resp_status: c_int,
// serializer for backend IO: request/response
    pub req_io_lock: mutex,
    pub req: },
    pub page: *mut xendispl_event_page,
    pub evt: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_drm_front_evtchnl_pair {
    pub req: xen_drm_front_evtchnl,
    pub evt: xen_drm_front_evtchnl,
}

extern "C" {
    pub fn xen_drm_front_evtchnl_create_all(front_info: *mut xen_drm_front_info) -> c_int;
}
extern "C" {
    pub fn xen_drm_front_evtchnl_publish_all(front_info: *mut xen_drm_front_info) -> c_int;
}
extern "C" {
    pub fn xen_drm_front_evtchnl_flush(evtchnl: *mut xen_drm_front_evtchnl);
}
extern "C" {
    pub fn xen_drm_front_evtchnl_free_all(front_info: *mut xen_drm_front_info);
}
