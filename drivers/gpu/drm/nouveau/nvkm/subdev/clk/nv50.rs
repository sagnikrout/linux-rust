//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/clk/nv50.h
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
pub struct nv50_clk_hwsq {
    pub base: hwsq,
    pub r_fifo: hwsq_reg,
    pub r_spll: [hwsq_reg; 2],
    pub r_nvpll: [hwsq_reg; 2],
    pub r_divs: hwsq_reg,
    pub r_mast: hwsq_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_clk {
    pub base: nvkm_clk,
    pub hwsq: nv50_clk_hwsq,
}

extern "C" {
    pub fn nv50_clk_read(: *mut nvkm_clk, nv_clk_src: enum) -> c_int;
}
extern "C" {
    pub fn nv50_clk_calc(: *mut nvkm_clk, : *mut nvkm_cstate) -> c_int;
}
extern "C" {
    pub fn nv50_clk_prog(: *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn nv50_clk_tidy(: *mut nvkm_clk);
}
