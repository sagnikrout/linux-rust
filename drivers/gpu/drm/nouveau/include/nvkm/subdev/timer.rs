//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/timer.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_alarm {
    pub head: list_head,
    pub exec: list_head,
    pub timestamp: u64,
    pub func: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_timer {
    pub func: *const nvkm_timer_func,
    pub subdev: nvkm_subdev,
    pub alarms: list_head,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn nvkm_timer_read(: *mut nvkm_timer) -> u64;
}
extern "C" {
    pub fn nvkm_timer_alarm(: *mut nvkm_timer, nsec: u32, : *mut nvkm_alarm);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_timer_wait {
    pub tmr: *mut nvkm_timer,
    pub limit: u64,
    pub time0: u64,
    pub time1: u64,
    pub reads: c_int,
}

extern "C" {
    pub fn nvkm_timer_wait_test(: *mut nvkm_timer_wait) -> i64;
}
// Delay based on GPU time (ie. PTIMER).
//
// Will return -ETIMEDOUT unless the loop was terminated with 'break',
// where it will return the number of nanoseconds taken instead.
//
// NVKM_DELAY can be passed for 'cond' to disable the timeout warning,
// which is useful for unconditional delay loops.
//

extern "C" {
    pub fn nv04_timer_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_timer) -> c_int;
}
extern "C" {
    pub fn nv40_timer_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_timer) -> c_int;
}
extern "C" {
    pub fn nv41_timer_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_timer) -> c_int;
}
extern "C" {
    pub fn gk20a_timer_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_timer) -> c_int;
}
