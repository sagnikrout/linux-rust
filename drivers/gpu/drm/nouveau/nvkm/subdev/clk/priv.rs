//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/clk/priv.h
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
pub struct nvkm_clk_func {
    pub ): *mut *mut int (init)(struct nvkm_clk,
    pub ): *mut *mut void (fini)(struct nvkm_clk,
    pub nv_clk_src): *mut *mut *mut int (read)(struct nvkm_clk , enum,
    pub ): *mut *mut *mut int (calc)(struct nvkm_clk , struct nvkm_cstate,
    pub ): *mut *mut int (prog)(struct nvkm_clk,
    pub ): *mut *mut void (tidy)(struct nvkm_clk,
    pub pstates: *mut nvkm_pstate,
    pub nr_pstates: c_int,
    pub domains: [nvkm_domain; ],
}

extern "C" {
    pub fn nv04_clk_pll_prog(: *mut nvkm_clk, reg1: u32, : *mut nvkm_pll_vals) -> c_int;
}
