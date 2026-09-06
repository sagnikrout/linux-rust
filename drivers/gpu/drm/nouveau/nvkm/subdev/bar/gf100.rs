//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bar/gf100.h
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
pub struct gf100_barN {
    pub inst: *mut nvkm_memory,
    pub vmm: *mut nvkm_vmm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_bar {
    pub base: nvkm_bar,
    pub bar2_halve: bool,
    pub bar: [gf100_barN; 2],
}

extern "C" {
    pub fn gf100_bar_oneinit(: *mut nvkm_bar) -> c_int;
}
extern "C" {
    pub fn gf100_bar_bar1_init(: *mut nvkm_bar);
}
extern "C" {
    pub fn gf100_bar_bar1_wait(: *mut nvkm_bar);
}
extern "C" {
    pub fn gf100_bar_bar2_init(: *mut nvkm_bar);
}
