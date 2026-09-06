//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv50/ovly.h
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


extern "C" {
    pub fn ovly507e_new(: *mut nouveau_drm, _arg: c_int, _arg: i32, : *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn ovly507e_scale_set(: *mut nv50_wndw, : *mut nv50_wndw_atom) -> c_int;
}
extern "C" {
    pub fn ovly827e_ntfy_reset(: *mut nouveau_bo, _arg: u32);
}
extern "C" {
    pub fn ovly827e_ntfy_wait_begun(: *mut nouveau_bo, _arg: u32, : *mut nvif_device) -> c_int;
}
extern "C" {
    pub fn ovly827e_new(: *mut nouveau_drm, _arg: c_int, _arg: i32, : *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn ovly907e_new(: *mut nouveau_drm, _arg: c_int, _arg: i32, : *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn ovly917e_new(: *mut nouveau_drm, _arg: c_int, _arg: i32, : *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn nv50_ovly_new(: *mut nouveau_drm, head: c_int, : *mut nv50_wndw) -> c_int;
}
