//! Automatically rewritten from C Header to Rust Module
//! Source: sound/xen/xen_snd_front_evtchnl.h
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
// Xen para-virtual sound device
//
// Copyright (C) 2016-2018 EPAM Systems Inc.
//
// Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
//

// Timeout in ms to wait for backend to respond.
pub const VSND_WAIT_BACK_MS: c_int = 3000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xen_snd_front_evtchnl_state {
    EVTCHNL_STATE_DISCONNECTED,
    EVTCHNL_STATE_CONNECTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xen_snd_front_evtchnl_type {
    EVTCHNL_TYPE_REQ,
    EVTCHNL_TYPE_EVT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_snd_front_evtchnl {
    pub front_info: *mut xen_snd_front_info,
    pub gref: c_int,
    pub port: c_int,
    pub irq: c_int,
    pub index: c_int,
// State of the event channel.
    pub state: xen_snd_front_evtchnl_state,
    pub type: xen_snd_front_evtchnl_type,
// Current response id or next expected incoming event id.
    pub evt_id: u16,
// Next request id.
    pub evt_next_id: u16,
// Shared ring access lock.
    pub ring_io_lock: mutex,
    pub ring: xen_sndif_front_ring,
    pub completion: completion,
// Serializer for backend IO: request/response.
    pub req_io_lock: mutex,
// Latest response status.
    pub resp_status: c_int,
    pub hw_param: xensnd_query_hw_param,
    pub resp: },
    pub req: },
    pub page: *mut xensnd_event_page,
// This is needed to handle XENSND_EVT_CUR_POS event.
    pub substream: *mut snd_pcm_substream,
    pub evt: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_snd_front_evtchnl_pair {
    pub req: xen_snd_front_evtchnl,
    pub evt: xen_snd_front_evtchnl,
}

extern "C" {
    pub fn xen_snd_front_evtchnl_free_all(front_info: *mut xen_snd_front_info);
}
extern "C" {
    pub fn xen_snd_front_evtchnl_publish_all(front_info: *mut xen_snd_front_info) -> c_int;
}
extern "C" {
    pub fn xen_snd_front_evtchnl_flush(evtchnl: *mut xen_snd_front_evtchnl);
}
extern "C" {
    pub fn xen_snd_front_evtchnl_pair_clear(evt_pair: *mut xen_snd_front_evtchnl_pair);
}
