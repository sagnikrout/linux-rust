//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/timer/priv.h
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
pub struct nvkm_timer_func {
    pub ): *mut *mut void (init)(struct nvkm_timer,
    pub ): *mut *mut void (intr)(struct nvkm_timer,
    pub ): *mut *mut u64 (read)(struct nvkm_timer,
    pub time): *mut *mut *mut void (time)(struct nvkm_timer , u64,
    pub time): *mut *mut *mut void (alarm_init)(struct nvkm_timer , u32,
    pub ): *mut *mut void (alarm_fini)(struct nvkm_timer,
}

extern "C" {
    pub fn nvkm_timer_alarm_trigger(: *mut nvkm_timer);
}
extern "C" {
    pub fn nv04_timer_fini(: *mut nvkm_timer);
}
extern "C" {
    pub fn nv04_timer_intr(: *mut nvkm_timer);
}
extern "C" {
    pub fn nv04_timer_time(: *mut nvkm_timer, _arg: u64);
}
extern "C" {
    pub fn nv04_timer_read(: *mut nvkm_timer) -> u64;
}
extern "C" {
    pub fn nv04_timer_alarm_init(: *mut nvkm_timer, _arg: u32);
}
extern "C" {
    pub fn nv04_timer_alarm_fini(: *mut nvkm_timer);
}
