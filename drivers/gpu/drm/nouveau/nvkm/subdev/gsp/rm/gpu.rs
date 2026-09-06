//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/gpu.h
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
pub struct nvkm_rm_gpu {
    pub root: u32,
    pub caps: u32,
    pub core: u32,
    pub wndw: u32,
    pub wimm: u32,
    pub curs: u32,
    pub class: },
    pub disp: },
    pub class: u32,
    pub usermode: },
    pub class: u32,
    pub ): *mut *mut u32 (doorbell_handle)(struct nvkm_chan,
    pub chan: },
    pub fifo: },
    pub class: u32,
    pub ): *mut *mut u32 (grce_mask)(struct nvkm_device,
    pub ce: },
    pub i2m: u32,
    pub twod: u32,
    pub threed: u32,
    pub compute: u32,
    pub class: },
    pub gr: },
    pub class: u32,
    pub nvdec: },
    pub class: u32,
    pub nvenc: },
    pub class: u32,
    pub nvjpg: },
    pub class: u32,
    pub ofa: },
}
