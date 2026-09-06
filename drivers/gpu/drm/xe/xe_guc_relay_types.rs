//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_relay_types.h
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
// Copyright © 2023 Intel Corporation
//

//
// struct xe_guc_relay - Data used by the VF-PF Relay Communication over GuC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_relay {
// @lock: protects all internal data.
    pub lock: spinlock_t,
// @worker: dispatches incoming action messages.
    pub worker: work_struct,
// @pending_relays: list of sent requests that await a response.
    pub pending_relays: list_head,
// @incoming_actions: list of incoming relay action messages to process.
    pub incoming_actions: list_head,
// @pool: pool of the relay message buffers.
    pub pool: mempool_t,
// @last_rid: last Relay-ID used while sending a message.
    pub last_rid: u32,
// @diag_ratelimit: ratelimit state used to throttle diagnostics messages.
    pub diag_ratelimit: ratelimit_state,
}
