//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/xen/events/events_internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Xen Event Channels (internal header)
//
// Copyright (C) 2013 Citrix Systems R&D Ltd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_ops {
    pub (*max_channels)(void): *mut unsigned,
    pub (*nr_channels)(void): *mut unsigned,
    pub port): *mut *mut int (setup)(evtchn_port_t,
    pub cpu): *mut *mut void (remove)(evtchn_port_t port, unsigned int,
    pub old_cpu): c_uint,
    pub port): *mut *mut void (clear_pending)(evtchn_port_t,
    pub port): *mut *mut void (set_pending)(evtchn_port_t,
    pub port): *mut *mut bool (is_pending)(evtchn_port_t,
    pub port): *mut *mut void (mask)(evtchn_port_t,
    pub port): *mut *mut void (unmask)(evtchn_port_t,
    pub ctrl): *mut *mut void (handle_events)(unsigned cpu, struct evtchn_loop_ctrl,
    pub (*resume)(void): *mut c_void,
    pub cpu): *mut *mut int (percpu_init)(unsigned int,
    pub cpu): *mut *mut int (percpu_deinit)(unsigned int,
}

extern "C" {
    pub fn handle_irq_for_port(port: evtchn_port_t, ctrl: *mut evtchn_loop_ctrl);
}
extern "C" {
    pub fn cpu_from_evtchn(evtchn: evtchn_port_t) -> c_uint;
}
//
// Do any ABI specific setup for a bound event channel before it can
// be unmasked and used.
//
extern "C" {
    pub fn xen_evtchn_2l_init();
}
extern "C" {
    pub fn xen_evtchn_fifo_init() -> c_int;
}
