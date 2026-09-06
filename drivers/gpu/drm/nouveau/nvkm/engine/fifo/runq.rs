//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/fifo/runq.h
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
pub struct nvkm_runq {
    pub ): *mut *mut void (init)(struct nvkm_runq,
    pub ): *mut *mut *mut bool (intr)(struct nvkm_runq , struct nvkm_runl,
    pub intr_0_names: *const nvkm_bitfield,
    pub chid): *mut *mut *mut bool (intr_1_ctxnotvalid)(struct nvkm_runq , int,
    pub ): *mut *mut bool (idle)(struct nvkm_runq,
    pub func: *mut },
    pub fifo: *mut nvkm_fifo,
    pub id: c_int,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_runq_del(: *mut nvkm_runq);
}

