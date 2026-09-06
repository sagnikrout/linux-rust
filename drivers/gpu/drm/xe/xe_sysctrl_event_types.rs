//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sysctrl_event_types.h
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
//
// Copyright © 2026 Intel Corporation
//

pub const XE_SYSCTRL_EVENT_DATA_LEN: c_int = 59;
//
// enum xe_sysctrl_event - Events reported by System Controller
//
// @XE_SYSCTRL_EVENT_THRESHOLD_CROSSED: Error counter threshold crossed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_sysctrl_event {
    XE_SYSCTRL_EVENT_THRESHOLD_CROSSED	= 0x01,
}

//
// struct xe_sysctrl_event_request - Request structure for pending event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sysctrl_event_request {
// @vector: MSI-X vector that was triggered
    pub vector: u32,
// @fn: Function index (0-7) of PCIe device
    pub fn:8: u32,
// @reserved: Reserved for future use
    pub reserved:24: u32,
// @reserved1: Reserved for future use
    pub reserved1: [u32; 2],
    pub __packed: },
//
// struct xe_sysctrl_event_response - Response structure for pending event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sysctrl_event_response {
// @count: Pending event count after this response
    pub count: u32,
// @event: Pending event type
    pub event: u32,
// @timestamp: Timestamp of most recent event
    pub timestamp: u64,
// @extended: Event has extended payload
    pub extended:1: u32,
// @reserved: Reserved for future use
    pub reserved:31: u32,
// @data: Generic event data
    pub data: [u32; XE_SYSCTRL_EVENT_DATA_LEN],
    pub __packed: },
