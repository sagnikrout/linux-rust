//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv50/core.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_core {
    pub func: *const nv50_core_func,
    pub disp: *mut nv50_disp,
    pub chan: nv50_dmac,
    pub assign_windows: bool,
}

extern "C" {
    pub fn nv50_core_new(: *mut nouveau_drm, : *mut nv50_core) -> c_int;
}
extern "C" {
    pub fn nv50_core_del(: *mut nv50_core);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_core_func {
    pub ): *mut *mut int (init)(struct nv50_core,
    pub offset): *mut *mut *mut void (ntfy_init)(struct nouveau_bo , u32,
    pub ): *mut *mut *mut int (caps_init)(struct nouveau_drm , struct nv50_disp,
    pub caps_class: u32,
    pub ): *mut nvif_device,
    pub ntfy): *mut *mut *mut *mut int (update)(struct nv50_core , u32 interlock, bool,
    pub ): *mut *mut int (owner)(struct nv50_core,
    pub wndw: },
    pub head: *const nv50_head_func,

    pub crc: *const nv50_crc_func,

    pub ): *mut nv50_head_atom,
// XXX: Only used by SORs and PIORs for now
    pub or): *mut *mut nouveau_encoder , int,
    pub sor: *mut *mut *mut } dac, pior,,
}

extern "C" {
    pub fn core507d_new(: *mut nouveau_drm, _arg: i32, : *mut nv50_core) -> c_int;
}
extern "C" {
    pub fn core507d_init(: *mut nv50_core) -> c_int;
}
extern "C" {
    pub fn core507d_ntfy_init(: *mut nouveau_bo, _arg: u32);
}
extern "C" {
    pub fn core507d_read_caps(disp: *mut nv50_disp) -> c_int;
}
extern "C" {
    pub fn core507d_caps_init(: *mut nouveau_drm, : *mut nv50_disp) -> c_int;
}
extern "C" {
    pub fn core507d_ntfy_wait_done(: *mut nouveau_bo, _arg: u32, : *mut nvif_device) -> c_int;
}
extern "C" {
    pub fn core507d_update(: *mut nv50_core, : *mut u32, _arg: bool) -> c_int;
}
extern "C" {
    pub fn core827d_new(: *mut nouveau_drm, _arg: i32, : *mut nv50_core) -> c_int;
}
extern "C" {
    pub fn core907d_new(: *mut nouveau_drm, _arg: i32, : *mut nv50_core) -> c_int;
}
extern "C" {
    pub fn core907d_caps_init(drm: *mut nouveau_drm, disp: *mut nv50_disp) -> c_int;
}
extern "C" {
    pub fn core917d_new(: *mut nouveau_drm, _arg: i32, : *mut nv50_core) -> c_int;
}
extern "C" {
    pub fn corec37d_new(: *mut nouveau_drm, _arg: i32, : *mut nv50_core) -> c_int;
}
extern "C" {
    pub fn corec37d_caps_init(: *mut nouveau_drm, : *mut nv50_disp) -> c_int;
}
extern "C" {
    pub fn corec37d_ntfy_wait_done(: *mut nouveau_bo, _arg: u32, : *mut nvif_device) -> c_int;
}
extern "C" {
    pub fn corec37d_update(: *mut nv50_core, : *mut u32, _arg: bool) -> c_int;
}
extern "C" {
    pub fn corec37d_wndw_owner(: *mut nv50_core) -> c_int;
}
extern "C" {
    pub fn corec57d_new(: *mut nouveau_drm, _arg: i32, : *mut nv50_core) -> c_int;
}
extern "C" {
    pub fn coreca7d_new(: *mut nouveau_drm, _arg: i32, : *mut nv50_core) -> c_int;
}
