//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_fence.h
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
pub struct nouveau_fence {
    pub base: dma_fence,
    pub head: list_head,
    pub channel: *mut nouveau_channel __rcu,
    pub timeout: c_ulong,
}

extern "C" {
    pub fn container_of(_arg: fence, nouveau_fence: struct, _arg: base) -> return;
}
extern "C" {
    pub fn nouveau_fence_create(: *mut nouveau_fence, : *mut nouveau_channel) -> c_int;
}
extern "C" {
    pub fn nouveau_fence_new(: *mut nouveau_fence, : *mut nouveau_channel) -> c_int;
}
extern "C" {
    pub fn nouveau_fence_unref(: *mut nouveau_fence);
}
extern "C" {
    pub fn nouveau_fence_emit(: *mut nouveau_fence) -> c_int;
}
extern "C" {
    pub fn nouveau_fence_done(: *mut nouveau_fence) -> bool;
}
extern "C" {
    pub fn nouveau_fence_wait(: *mut nouveau_fence, lazy: bool, intr: bool) -> c_int;
}
extern "C" {
    pub fn nouveau_fence_sync(: *mut nouveau_bo, : *mut nouveau_channel, exclusive: bool, intr: bool) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_fence_chan {
    pub lock: spinlock_t,
    pub fence_ref: kref,
    pub pending: list_head,
    pub flip: list_head,
    pub ): *mut *mut int (emit)(struct nouveau_fence,
    pub ): *mut nouveau_channel,
    pub ): *mut *mut u32 (read)(struct nouveau_channel,
    pub u32): *mut *mut *mut int (emit32)(struct nouveau_channel , u64,,
    pub u32): *mut *mut *mut int (sync32)(struct nouveau_channel , u64,,
    pub sequence: u32,
    pub context: u32,
    pub name: [c_char; 32],
    pub uevent_work: work_struct,
    pub event: nvif_event,
    pub killed: int notify_ref, dead,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_fence_priv {
    pub ): *mut *mut void (dtor)(struct nouveau_drm,
    pub ): *mut *mut bool (suspend)(struct nouveau_drm,
    pub ): *mut *mut void (resume)(struct nouveau_drm,
    pub ): *mut *mut int (context_new)(struct nouveau_channel,
    pub ): *mut *mut void (context_del)(struct nouveau_channel,
    pub uevent: bool,
}

extern "C" {
    pub fn nouveau_fence_context_new(: *mut nouveau_channel, : *mut nouveau_fence_chan);
}
extern "C" {
    pub fn nouveau_fence_context_del(: *mut nouveau_fence_chan);
}
extern "C" {
    pub fn nouveau_fence_context_free(: *mut nouveau_fence_chan);
}
extern "C" {
    pub fn nouveau_fence_context_kill(: *mut nouveau_fence_chan, error: c_int);
}
extern "C" {
    pub fn nv04_fence_create(: *mut nouveau_drm) -> c_int;
}
extern "C" {
    pub fn nv04_fence_mthd(: *mut nouveau_channel, _arg: u32, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn nv10_fence_emit(: *mut nouveau_fence) -> c_int;
}
extern "C" {
    pub fn nv10_fence_read(: *mut nouveau_channel) -> u32;
}
extern "C" {
    pub fn nv10_fence_context_del(: *mut nouveau_channel);
}
extern "C" {
    pub fn nv10_fence_destroy(: *mut nouveau_drm);
}
extern "C" {
    pub fn nv10_fence_create(: *mut nouveau_drm) -> c_int;
}
extern "C" {
    pub fn nv17_fence_create(: *mut nouveau_drm) -> c_int;
}
extern "C" {
    pub fn nv17_fence_resume(drm: *mut nouveau_drm);
}
extern "C" {
    pub fn nv50_fence_create(: *mut nouveau_drm) -> c_int;
}
extern "C" {
    pub fn nv84_fence_create(: *mut nouveau_drm) -> c_int;
}
extern "C" {
    pub fn nvc0_fence_create(: *mut nouveau_drm) -> c_int;
}
extern "C" {
    pub fn gv100_fence_create(: *mut nouveau_drm) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv84_fence_chan {
    pub base: nouveau_fence_chan,
    pub vma: *mut nouveau_vma,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv84_fence_priv {
    pub base: nouveau_fence_priv,
    pub bo: *mut nouveau_bo,
    pub suspend: *mut u32,
    pub mutex: mutex,
}

extern "C" {
    pub fn nv84_fence_context_new(: *mut nouveau_channel) -> c_int;
}
