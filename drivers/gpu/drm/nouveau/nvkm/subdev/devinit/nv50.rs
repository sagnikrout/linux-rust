//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/devinit/nv50.h
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
pub struct nv50_devinit {
    pub base: nvkm_devinit,
    pub r001540: u32,
}

extern "C" {
    pub fn nv50_devinit_preinit(: *mut nvkm_devinit);
}
extern "C" {
    pub fn nv50_devinit_init(: *mut nvkm_devinit);
}
extern "C" {
    pub fn nv50_devinit_pll_set(: *mut nvkm_devinit, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gt215_devinit_pll_set(: *mut nvkm_devinit, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gf100_devinit_pll_set(: *mut nvkm_devinit, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gf100_devinit_preinit(: *mut nvkm_devinit);
}
extern "C" {
    pub fn gm107_devinit_disable(: *mut nvkm_devinit);
}
extern "C" {
    pub fn gm200_devinit_post(: *mut nvkm_devinit, _arg: bool) -> c_int;
}
extern "C" {
    pub fn gm200_devinit_preos(: *mut nv50_devinit, _arg: bool);
}
