//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/event.h
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

pub const NVIF_EVENT_KEEP: c_int = 0;
pub const NVIF_EVENT_DROP: c_int = 1;
extern "C" {
    pub fn int(: *mut *mut nvif_event_func)(struct nvif_event, repv: *mut c_void, repc: u32) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_event {
    pub object: nvif_object,
    pub func: nvif_event_func,
}

extern "C" {
    pub fn nvif_object_constructed(_arg: &event->object) -> return;
}
extern "C" {
    pub fn nvif_event_ctor_(_arg: parent, _arg: name, _arg: handle, _arg: func, _arg: wait, _arg: args, _arg: argc, _arg: true, _arg: event) -> return;
}
extern "C" {
    pub fn nvif_event_dtor(: *mut nvif_event);
}
extern "C" {
    pub fn nvif_event_allow(: *mut nvif_event) -> c_int;
}
extern "C" {
    pub fn nvif_event_block(: *mut nvif_event) -> c_int;
}
