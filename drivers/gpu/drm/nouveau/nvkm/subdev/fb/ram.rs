//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fb/ram.h
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

extern "C" {
    pub fn nvkm_ram_del(: *mut nvkm_ram);
}
extern "C" {
    pub fn nvkm_ram_init(: *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gf100_ram_probe_fbpa_amount(: *mut nvkm_device, _arg: c_int) -> u32;
}
extern "C" {
    pub fn gf100_ram_init(: *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gf100_ram_calc(: *mut nvkm_ram, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gf100_ram_prog(: *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gf100_ram_tidy(: *mut nvkm_ram);
}
extern "C" {
    pub fn gk104_ram_init(: *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gk104_ram_calc(: *mut nvkm_ram, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gk104_ram_prog(: *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gk104_ram_tidy(: *mut nvkm_ram);
}
extern "C" {
    pub fn gp100_ram_init(: *mut nvkm_ram) -> c_int;
}
// RAM type-specific MR calculation routines
extern "C" {
    pub fn nvkm_sddr2_calc(: *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nvkm_sddr3_calc(: *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nvkm_gddr3_calc(: *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nvkm_gddr5_calc(: *mut nvkm_ram, nuts: bool) -> c_int;
}
extern "C" {
    pub fn nv04_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nv10_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nv1a_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nv20_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nv40_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nv41_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nv44_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nv49_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nv4e_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn nv50_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gt215_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn mcp77_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gf100_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gf108_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gk104_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gm107_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gm200_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gp100_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
extern "C" {
    pub fn gp102_ram_new(: *mut nvkm_fb, : *mut nvkm_ram) -> c_int;
}
