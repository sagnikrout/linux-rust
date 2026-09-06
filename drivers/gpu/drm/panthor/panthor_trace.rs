//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_trace.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2025 Collabora ltd.

//
// gpu_power_status - called whenever parts of GPU hardware are turned on or off
// @dev: pointer to the &struct device, for printing the device name
// @shader_bitmap: bitmap where a high bit indicates the shader core at a given
// bit index is on, and a low bit indicates a shader core is
// either powered off or absent
// @tiler_bitmap: bitmap where a high bit indicates the tiler unit at a given
// bit index is on, and a low bit indicates a tiler unit is
// either powered off or absent
// @l2_bitmap: bitmap where a high bit indicates the L2 cache at a given bit
// index is on, and a low bit indicates the L2 cache is either
// powered off or absent
//
// gpu_job_irq - called after a job interrupt from firmware completes
// @dev: pointer to the &struct device, for printing the device name
// @events: bitmask of BIT(CSG id) | BIT(31) for a global event
// @duration_ns: Nanoseconds between job IRQ handler entry and exit
//
// The panthor_job_irq_handler() function instrumented by this tracepoint exits
// once it has queued the firmware interrupts for processing, not when the
// firmware interrupts are fully processed. This tracepoint allows for debugging
// issues with delays in the workqueue's processing of events.
//

