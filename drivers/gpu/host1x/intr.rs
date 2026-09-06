//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/host1x/intr.h
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
// Tegra host1x Interrupt Management
//
// Copyright (c) 2010-2021, NVIDIA Corporation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_intr_irq_data {
    pub host: *mut host1x,
    pub offset: u32,
}

// Initialize host1x sync point interrupt
extern "C" {
    pub fn host1x_intr_init(host: *mut host1x) -> c_int;
}
// Deinitialize host1x sync point interrupt
extern "C" {
    pub fn host1x_intr_deinit(host: *mut host1x);
}
// Enable host1x sync point interrupt
extern "C" {
    pub fn host1x_intr_start(host: *mut host1x);
}
// Disable host1x sync point interrupt
extern "C" {
    pub fn host1x_intr_stop(host: *mut host1x);
}
extern "C" {
    pub fn host1x_intr_handle_interrupt(host: *mut host1x, id: c_uint);
}
extern "C" {
    pub fn host1x_intr_add_fence_locked(host: *mut host1x, fence: *mut host1x_syncpt_fence);
}
extern "C" {
    pub fn host1x_intr_remove_fence(host: *mut host1x, fence: *mut host1x_syncpt_fence) -> bool;
}
