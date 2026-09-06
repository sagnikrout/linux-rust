//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_chan.h
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
pub struct nouveau_channel {
    pub chan: nvif_chan,
    pub cli: *mut nouveau_cli,
    pub vmm: *mut nouveau_vmm,
    pub mem_userd: nvif_mem,
    pub userd: *mut nvif_object,
    pub runlist: c_int,
    pub chid: c_int,
    pub inst: u64,
    pub token: u32,
    pub vram: nvif_object,
    pub gart: nvif_object,
    pub nvsw: nvif_object,
    pub buffer: *mut nouveau_bo,
    pub vma: *mut nouveau_vma,
    pub ctxdma: nvif_object,
    pub addr: u64,
    pub push: },
    pub fence: *mut c_void,
    pub max: c_int,
    pub free: c_int,
    pub cur: c_int,
    pub put: c_int,
    pub dma: },
    pub user_get: u32,
    pub user_put: u32,
    pub bo: *mut nouveau_bo,
    pub vma: *mut nouveau_vma,
    pub sema: },
    pub user: nvif_object,
    pub blit: nvif_object,
    pub kill: nvif_event,
    pub killed: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn nouveau_channels_init(: *mut nouveau_drm) -> c_int;
}
extern "C" {
    pub fn nouveau_channels_fini(: *mut nouveau_drm);
}
extern "C" {
    pub fn nouveau_channel_del(: *mut nouveau_channel);
}
extern "C" {
    pub fn nouveau_channel_idle(: *mut nouveau_channel) -> c_int;
}
extern "C" {
    pub fn nouveau_channel_kill(: *mut nouveau_channel);
}
