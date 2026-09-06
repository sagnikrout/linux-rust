//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/gr.h
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

pub const R515_GR_MAX_CTXBUFS: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r535_gr_chan {
    pub object: nvkm_object,
    pub gr: *mut r535_gr,
    pub vmm: *mut nvkm_vmm,
    pub chan: *mut nvkm_chan,
    pub mem: [*mut nvkm_memory; R515_GR_MAX_CTXBUFS],
    pub vma: [*mut nvkm_vma; R515_GR_MAX_CTXBUFS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r535_gr {
    pub base: nvkm_gr,
    pub bufferId: u16,
    pub size: u32,
    pub page: u8,
    pub align: u8,
    pub global: bool,
    pub init: bool,
    pub ro: bool,
    pub ctxbuf: [}; R515_GR_MAX_CTXBUFS],
    pub ctxbuf_nr: c_int,
    pub ctxbuf_mem: [*mut nvkm_memory; R515_GR_MAX_CTXBUFS],
    pub chid: c_int,
    pub inst: *mut nvkm_memory,
    pub vmm: *mut nvkm_vmm,
    pub chan: nvkm_gsp_object,
    pub threed: nvkm_gsp_object,
    pub mem: [*mut nvkm_memory; R515_GR_MAX_CTXBUFS],
    pub vma: [*mut nvkm_vma; R515_GR_MAX_CTXBUFS],
    pub ctxbuf: },
    pub enabled: bool,
    pub scrubber: },
}
