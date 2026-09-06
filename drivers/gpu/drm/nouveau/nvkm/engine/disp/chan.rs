//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/disp/chan.h
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
pub struct nvkm_disp_chan {
    pub func: *const nvkm_disp_chan_func,
    pub mthd: *const nvkm_disp_chan_mthd,
    pub disp: *mut nvkm_disp,
    pub ctrl: c_int,
    pub user: c_int,
    pub chid: },
    pub head: c_int,
    pub object: nvkm_object,
    pub memory: *mut nvkm_memory,
    pub push: u64,
    pub suspend_put: u32,
    pub object: nvkm_gsp_object,
    pub rm: },
}

extern "C" {
    pub fn nvkm_disp_core_new(: *const nvkm_oclass, : *mut c_void, _arg: u32, : *mut nvkm_object) -> c_int;
}
extern "C" {
    pub fn nvkm_disp_chan_new(: *const nvkm_oclass, : *mut c_void, _arg: u32, : *mut nvkm_object) -> c_int;
}
extern "C" {
    pub fn nvkm_disp_wndw_new(: *const nvkm_oclass, : *mut c_void, _arg: u32, : *mut nvkm_object) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_disp_chan_func {
    pub object): *mut *mut *mut int (push)(struct nvkm_disp_chan , u64,
    pub ): *mut *mut int (init)(struct nvkm_disp_chan,
    pub ): *mut *mut void (fini)(struct nvkm_disp_chan,
    pub en): *mut *mut *mut void (intr)(struct nvkm_disp_chan , bool,
    pub size): *mut *mut *mut u64 (user)(struct nvkm_disp_chan , u64,
    pub handle): *mut *mut *mut *mut int (bind)(struct nvkm_disp_chan , struct nvkm_object , u32,
}

extern "C" {
    pub fn nv50_disp_chan_intr(: *mut nvkm_disp_chan, _arg: bool);
}
extern "C" {
    pub fn nv50_disp_chan_user(: *mut nvkm_disp_chan, : *mut u64) -> u64;
}
extern "C" {
    pub fn nv50_disp_dmac_push(: *mut nvkm_disp_chan, _arg: u64) -> c_int;
}
extern "C" {
    pub fn nv50_disp_dmac_bind(: *mut nvkm_disp_chan, : *mut nvkm_object, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gf119_disp_chan_intr(: *mut nvkm_disp_chan, _arg: bool);
}
extern "C" {
    pub fn gf119_disp_dmac_fini(: *mut nvkm_disp_chan);
}
extern "C" {
    pub fn gf119_disp_dmac_bind(: *mut nvkm_disp_chan, : *mut nvkm_object, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gf119_disp_core_fini(: *mut nvkm_disp_chan);
}
extern "C" {
    pub fn gv100_disp_chan_user(: *mut nvkm_disp_chan, : *mut u64) -> u64;
}
extern "C" {
    pub fn gv100_disp_dmac_init(: *mut nvkm_disp_chan) -> c_int;
}
extern "C" {
    pub fn gv100_disp_dmac_fini(: *mut nvkm_disp_chan);
}
extern "C" {
    pub fn gv100_disp_dmac_bind(: *mut nvkm_disp_chan, : *mut nvkm_object, _arg: u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_disp_chan_user {
    pub func: *const nvkm_disp_chan_func,
    pub ctrl: c_int,
    pub user: c_int,
    pub mthd: *const nvkm_disp_chan_mthd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_disp_mthd_list {
    pub mthd: u32,
    pub addr: u32,
    pub mthd: u32,
    pub addr: u32,
    pub name: *const c_char,
    pub data: [}; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_disp_chan_mthd {
    pub name: *const c_char,
    pub addr: u32,
    pub prev: i32,
    pub name: *const c_char,
    pub nr: c_int,
    pub mthd: *const nvkm_disp_mthd_list,
    pub data: [}; ],
}

extern "C" {
    pub fn nv50_disp_chan_mthd(: *mut nvkm_disp_chan, debug: c_int);
}
