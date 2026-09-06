//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/event.h
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
pub struct nvkm_event {
    pub func: *const nvkm_event_func,
    pub subdev: *mut nvkm_subdev,
    pub types_nr: c_int,
    pub index_nr: c_int,
    pub refs_lock: spinlock_t,
    pub list_lock: rwlock_t,
    pub refs: *mut c_int,
    pub ntfy: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_event_func {
    pub index): *mut *mut *mut void (init)(struct nvkm_event , int type, int,
    pub index): *mut *mut *mut void (fini)(struct nvkm_event , int type, int,
}

// Each nvkm_event needs its own lockdep class due to inter-dependencies, to
// prevent lockdep false-positives.
//
// Inlining the spinlock initialisation ensures each is unique.
//
extern "C" {
    pub fn __nvkm_event_init(_arg: func, _arg: subdev, _arg: types_nr, _arg: index_nr, _arg: event) -> return;
}
extern "C" {
    pub fn nvkm_event_fini(: *mut nvkm_event);
}
pub const NVKM_EVENT_KEEP: c_int = 0;
pub const NVKM_EVENT_DROP: c_int = 1;
extern "C" {
    pub fn int(: *mut *mut nvkm_event_func)(struct nvkm_event_ntfy, bits: u32) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_event_ntfy {
    pub event: *mut nvkm_event,
    pub id: c_int,
    pub bits: u32,
    pub wait: bool,
    pub func: nvkm_event_func,
    pub allowed: core::sync::atomic::AtomicI32,
    pub running: bool,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_event_ntfy(: *mut nvkm_event, id: c_int, bits: u32);
}
extern "C" {
    pub fn nvkm_event_ntfy_valid(: *mut nvkm_event, id: c_int, bits: u32) -> bool;
}
extern "C" {
    pub fn nvkm_event_ntfy_del(: *mut nvkm_event_ntfy);
}
extern "C" {
    pub fn nvkm_event_ntfy_allow(: *mut nvkm_event_ntfy);
}
extern "C" {
    pub fn nvkm_event_ntfy_block(: *mut nvkm_event_ntfy);
}
extern "C" {
    pub fn int(: *mut *mut nvkm_uevent_func)(struct nvkm_object, token: u64, bits: u32) -> typedef;
}
extern "C" {
    pub fn nvkm_uevent_new(: *const nvkm_oclass, argv: *mut c_void, argc: u32, : *mut nvkm_object) -> c_int;
}
extern "C" {
    pub fn nvkm_uevent_add(: *mut nvkm_uevent, : *mut nvkm_event, id: c_int, bits: u32, _arg: nvkm_uevent_func) -> c_int;
}
