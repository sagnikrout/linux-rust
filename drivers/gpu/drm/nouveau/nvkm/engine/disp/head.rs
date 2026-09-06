//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/disp/head.h
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
pub struct nvkm_head {
    pub func: *const nvkm_head_func,
    pub disp: *mut nvkm_disp,
    pub id: c_int,
    pub head: list_head,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_head_state {
    pub htotal: u16,
    pub hsynce: u16,
    pub hblanke: u16,
    pub hblanks: u16,
    pub vtotal: u16,
    pub vsynce: u16,
    pub vblanke: u16,
    pub vblanks: u16,
    pub hz: u32,
// Prior to GF119, these are set by the OR.
    pub depth: u8,
    pub or: },
    pub asy: } arm,,
    pub object: nvkm_object,
}

extern "C" {
    pub fn nvkm_head_new_(: *const nvkm_head_func, : *mut nvkm_disp, id: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_head_del(: *mut nvkm_head);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_head_func {
    pub ): *mut *mut *mut void (state)(struct nvkm_head , struct nvkm_head_state,
    pub vline): *mut *mut *mut *mut void (rgpos)(struct nvkm_head , u16 hline, u16,
    pub div): *mut *mut *mut void (rgclk)(struct nvkm_head , int,
    pub ): *mut *mut void (vblank_get)(struct nvkm_head,
    pub ): *mut *mut void (vblank_put)(struct nvkm_head,
}

extern "C" {
    pub fn nv50_head_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn nv50_head_new(: *mut nvkm_disp, id: c_int) -> c_int;
}
extern "C" {
    pub fn nv50_head_rgpos(: *mut nvkm_head, : *mut u16, : *mut u16);
}
extern "C" {
    pub fn gf119_head_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn gf119_head_new(: *mut nvkm_disp, id: c_int) -> c_int;
}
extern "C" {
    pub fn gf119_head_rgclk(: *mut nvkm_head, _arg: c_int);
}
extern "C" {
    pub fn gv100_head_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn gv100_head_new(: *mut nvkm_disp, id: c_int) -> c_int;
}
extern "C" {
    pub fn gv100_head_state(head: *mut nvkm_head, state: *mut nvkm_head_state);
}
extern "C" {
    pub fn gv100_head_rgpos(head: *mut nvkm_head, hline: *mut u16, vline: *mut u16);
}

