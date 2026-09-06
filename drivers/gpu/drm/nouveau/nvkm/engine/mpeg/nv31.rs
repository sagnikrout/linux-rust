//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/mpeg/nv31.h
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
pub struct nv31_mpeg {
    pub func: *const nv31_mpeg_func,
    pub engine: nvkm_engine,
    pub chan: *mut nv31_mpeg_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv31_mpeg_func {
    pub data): *mut *mut *mut bool (mthd_dma)(struct nvkm_device , u32 mthd, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv31_mpeg_chan {
    pub object: nvkm_object,
    pub mpeg: *mut nv31_mpeg,
    pub fifo: *mut nvkm_chan,
}
