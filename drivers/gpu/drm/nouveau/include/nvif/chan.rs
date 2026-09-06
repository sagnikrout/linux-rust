//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/chan.h
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
//
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_chan {
    pub ): *mut *mut u32 (read_get)(struct nvif_chan,
    pub push: },
    pub ): *mut *mut u32 (read_get)(struct nvif_chan,
    pub no_prefetch): bool,
    pub ): *mut *mut void (kick)(struct nvif_chan,
    pub pbptr): *mut *mut *mut int (post)(struct nvif_chan , u32 gpptr, u32,
    pub post_size: u32,
    pub gpfifo: },
    pub data): *mut *mut *mut int (release)(struct nvif_chan , u64 addr, u32,
    pub sem: },
    pub func: *mut },
    pub map: nvif_map,
    pub userd: },
    pub map: nvif_map,
    pub cur: u32,
    pub max: u32,
    pub free: c_int,
    pub gpfifo: },
    pub map: nvif_map,
    pub addr: u64,
    pub sema: },
    pub push: nvif_push,
    pub usermode: *mut nvif_user,
    pub doorbell_token: u32,
}

extern "C" {
    pub fn nvif_chan_dma_wait(: *mut nvif_chan, push_nr: u32) -> c_int;
}
extern "C" {
    pub fn nvif_chan_gpfifo_wait(: *mut nvif_chan, gpfifo_nr: u32, push_nr: u32) -> c_int;
}
extern "C" {
    pub fn nvif_chan_gpfifo_push(: *mut nvif_chan, addr: u64, size: u32, no_prefetch: bool);
}
extern "C" {
    pub fn nvif_chan_gpfifo_post(: *mut nvif_chan) -> c_int;
}
extern "C" {
    pub fn nvif_chan506f_gpfifo_push(: *mut nvif_chan, main: bool, addr: u64, size: u32, no_prefetch: bool);
}
extern "C" {
    pub fn nvif_chan506f_gpfifo_kick(: *mut nvif_chan);
}
extern "C" {
    pub fn nvif_chan906f_read_get(: *mut nvif_chan) -> u32;
}
extern "C" {
    pub fn nvif_chan906f_gpfifo_read_get(: *mut nvif_chan) -> u32;
}
extern "C" {
    pub fn nvif_chan906f_gpfifo_post(: *mut nvif_chan, gpptr: u32, pbptr: u32) -> c_int;
}
