//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/nv40.h
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
pub struct nv40_gr {
    pub base: nvkm_gr,
    pub size: u32,
    pub chan: list_head,
}

extern "C" {
    pub fn nv40_gr_init(: *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv40_gr_intr(: *mut nvkm_gr);
}
extern "C" {
    pub fn nv40_gr_units(: *mut nvkm_gr) -> u64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv40_gr_chan {
    pub object: nvkm_object,
    pub gr: *mut nv40_gr,
    pub fifo: *mut nvkm_chan,
    pub inst: u32,
    pub head: list_head,
}

// returns 1 if device is one of the nv4x using the 0x4497 object class,
// helpful to determine a number of other hardware features
//
extern "C" {
    pub fn nv40_grctx_init(: *mut nvkm_device, size: *mut u32) -> c_int;
}
extern "C" {
    pub fn nv40_grctx_fill(: *mut nvkm_device, : *mut nvkm_gpuobj);
}
