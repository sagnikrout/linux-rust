//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/instmem/priv.h
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
pub struct nvkm_instmem_func {
    pub ): *mut *mut *mut void (dtor)(struct nvkm_instmem,
    pub ): *mut *mut int (oneinit)(struct nvkm_instmem,
    pub ): *mut *mut int (suspend)(struct nvkm_instmem,
    pub ): *mut *mut void (resume)(struct nvkm_instmem,
    pub ): *mut *mut void (fini)(struct nvkm_instmem,
    pub addr): *mut *mut *mut u32 (rd32)(struct nvkm_instmem , u32,
    pub data): *mut *mut *mut void (wr32)(struct nvkm_instmem , u32 addr, u32,
    pub ): *mut bool zero, struct nvkm_memory,
    pub ): *mut *mut *mut *mut int (memory_wrap)(struct nvkm_instmem , struct nvkm_memory , struct nvkm_memory,
    pub zero: bool,
    pub addr): *mut *mut *mut void (set_bar0_window_addr)(struct nvkm_device , u64,
}

extern "C" {
    pub fn nv50_instmem_fini(: *mut nvkm_instmem);
}
extern "C" {
    pub fn nvkm_instmem_boot(: *mut nvkm_instmem);
}
extern "C" {
    pub fn nv04_instmem_suspend(: *mut nvkm_instmem) -> c_int;
}
extern "C" {
    pub fn nv04_instmem_resume(: *mut nvkm_instmem);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_instobj {
    pub memory: nvkm_memory,
    pub head: list_head,
    pub preserve: bool,
    pub suspend: *mut u32,
}

extern "C" {
    pub fn nvkm_instobj_dtor(: *mut nvkm_instmem, : *mut nvkm_instobj);
}
extern "C" {
    pub fn nvkm_instobj_save(: *mut nvkm_instobj) -> c_int;
}
extern "C" {
    pub fn nvkm_instobj_load(: *mut nvkm_instobj);
}
