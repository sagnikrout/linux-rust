//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/fifo/cgrp.h
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
pub struct nvkm_vctx {
    pub ectx: *mut nvkm_ectx,
    pub vmm: *mut nvkm_vmm,
    pub refs: refcount_t,
    pub inst: *mut nvkm_gpuobj,
    pub vma: *mut nvkm_vma,
    pub head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_ectx {
    pub engn: *mut nvkm_engn,
    pub refs: refcount_t,
    pub uses: refcount_t,
    pub object: *mut nvkm_object,
    pub head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_cgrp {
    pub ): *mut *mut void (preempt)(struct nvkm_cgrp,
    pub func: *mut },
    pub name: [c_char; 64],
    pub runl: *mut nvkm_runl,
    pub vmm: *mut nvkm_vmm,
    pub hw: bool,
    pub id: c_int,
    pub kref: kref,
    pub chans: list_head,
    pub chan_nr: c_int,
    pub /: *mut *mut spinlock_t lock; / protects irq handler channel (group) lookup,
    pub ectxs: list_head,
    pub vctxs: list_head,
    pub mutex: mutex,
pub const NVKM_CGRP_RC_NONE: c_int = 0;
pub const NVKM_CGRP_RC_PENDING: c_int = 1;
pub const NVKM_CGRP_RC_RUNNING: c_int = 2;
    pub rc: core::sync::atomic::AtomicI32,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_cgrp_unref(: *mut nvkm_cgrp);
}
extern "C" {
    pub fn nvkm_cgrp_vctx_put(: *mut nvkm_cgrp, : *mut nvkm_vctx);
}
extern "C" {
    pub fn nvkm_cgrp_put(: *mut nvkm_cgrp, irqflags: c_ulong);
}

