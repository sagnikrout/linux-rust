//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/arm_sdei.h
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
// Copyright (C) 2017 Arm Ltd.

// Arch code should override this to set the entry point from firmware...

//
// When an event occurs sdei_event_handler() will call a user-provided callback
// like this in NMI context on the CPU that received the event.
//
extern "C" {
    pub fn int(event: sdei_event_callback)(u32, regs: *mut pt_regs, arg: *mut c_void) -> typedef;
}
//
// Register your callback to claim an event. The event must be described
// by firmware.
//
extern "C" {
    pub fn sdei_event_register(event_num: u32, cb: *mut sdei_event_callback, arg: *mut c_void) -> c_int;
}
//
// Calls to sdei_event_unregister() may return EINPROGRESS. Keep calling
// it until it succeeds.
//
extern "C" {
    pub fn sdei_event_unregister(event_num: u32) -> c_int;
}
extern "C" {
    pub fn sdei_event_enable(event_num: u32) -> c_int;
}
extern "C" {
    pub fn sdei_event_disable(event_num: u32) -> c_int;
}
//
// Signal the software-signalled event (event 0) to another PE, NMI-like.
// @mpidr is the target's MPIDR affinity.
//
extern "C" {
    pub fn sdei_event_signal(event_num: u32, mpidr: u64) -> c_int;
}
// Was SDEI firmware probed and usable?
extern "C" {
    pub fn sdei_is_present() -> bool;
}
// GHES register/unregister helpers
extern "C" {
    pub fn sdei_unregister_ghes(ghes: *mut ghes) -> c_int;
}

// For use by arch code when CPU hotplug notifiers are not appropriate.
extern "C" {
    pub fn sdei_mask_local_cpu() -> c_int;
}
extern "C" {
    pub fn sdei_unmask_local_cpu() -> c_int;
}
extern "C" {
    pub fn acpi_sdei_init() -> void __init;
}
extern "C" {
    pub fn sdei_handler_abort();
}

//
// This struct represents an event that has been registered. The driver
// maintains a list of all events, and which ones are registered. (Private
// events have one entry in the list, but are registered on each CPU).
// A pointer to this struct is passed to firmware, and back to the event
// handler. The event handler can then use this to invoke the registered
// callback, without having to walk the list.
//
// For CPU private events, this structure is per-cpu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdei_registered_event {
// For use by arch code:
    pub interrupted_regs: pt_regs,
    pub callback: *mut sdei_event_callback,
    pub callback_arg: *mut c_void,
    pub event_num: u32,
    pub priority: u8,
}

// The arch code entry point should then call this when an event arrives.
// arch code may use this to retrieve the extra registers.
extern "C" {
    pub fn sdei_api_event_context(query: u32, result: *mut u64) -> c_int;
}
