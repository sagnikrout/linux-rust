//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/fifo/chan.h
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
pub struct nvkm_cctx {
    pub vctx: *mut nvkm_vctx,
    pub refs: refcount_t,
    pub uses: refcount_t,
    pub head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_chan_func {
    pub size: u32,
    pub zero: bool,
    pub vmm: bool,
    pub inst: *mut },
    pub bar: nvkm_bar_id,
    pub base: u32,
    pub size: u32,
    pub ): *mut *mut void (clear)(struct nvkm_chan,
    pub userd: *mut },
    pub bits:6: unsigned,
    pub ctxs:5: unsigned,
    pub ctxp:8: unsigned,
    pub regs:5: unsigned,
    pub regp: unsigned,
    pub layout: *mut },
    pub priv): *mut *mut *mut int (write)(struct nvkm_chan , u64 offset, u64 length, u32 devm, bool,
    pub ): *mut *mut void (clear)(struct nvkm_chan,
    pub ctxdma: bool,
    pub devm: u32,
    pub priv: bool,
    pub ramfc: *mut },
    pub ): *mut *mut void (bind)(struct nvkm_chan,
    pub ): *mut *mut void (unbind)(struct nvkm_chan,
    pub ): *mut *mut void (start)(struct nvkm_chan,
    pub ): *mut *mut void (stop)(struct nvkm_chan,
    pub ): *mut *mut void (preempt)(struct nvkm_chan,
    pub ): *mut *mut u32 (doorbell_handle)(struct nvkm_chan,
}

extern "C" {
    pub fn nvkm_chan_del(: *mut nvkm_chan);
}
extern "C" {
    pub fn nvkm_chan_allow(: *mut nvkm_chan);
}
extern "C" {
    pub fn nvkm_chan_block(: *mut nvkm_chan);
}
extern "C" {
    pub fn nvkm_chan_error(: *mut nvkm_chan, preempt: bool);
}
extern "C" {
    pub fn nvkm_chan_insert(: *mut nvkm_chan);
}
extern "C" {
    pub fn nvkm_chan_remove(: *mut nvkm_chan, preempt: bool);
}
extern "C" {
    pub fn nvkm_chan_remove_locked(: *mut nvkm_chan);
}
extern "C" {
    pub fn nvkm_chan_preempt(: *mut nvkm_chan, wait: bool) -> c_int;
}
extern "C" {
    pub fn nvkm_chan_preempt_locked(: *mut nvkm_chan, wait: bool) -> c_int;
}
extern "C" {
    pub fn nvkm_chan_cctx_put(: *mut nvkm_chan, : *mut nvkm_cctx);
}
extern "C" {
    pub fn nvkm_chan_cctx_bind(: *mut nvkm_chan, : *mut nvkm_engn, : *mut nvkm_cctx);
}

