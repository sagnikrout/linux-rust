//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/object.h
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
pub struct nvif_sclass {
    pub oclass: i32,
    pub minver: c_int,
    pub maxver: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_object {
    pub parent: *mut nvif_parent,
    pub client: *mut nvif_client,
    pub name: *const c_char,
    pub handle: u32,
    pub oclass: i32,
    pub /: *mut *mut *mut void priv; /XXX: hack,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_map {
    pub ptr: *mut void __iomem,
    pub size: u64,
    pub map: },
}

extern "C" {
    pub fn nvif_object_dtor(: *mut nvif_object);
}
extern "C" {
    pub fn nvif_object_ioctl(: *mut nvif_object, : *mut c_void, _arg: u32, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn nvif_object_sclass_get(: *mut nvif_object, : *mut nvif_sclass) -> c_int;
}
extern "C" {
    pub fn nvif_object_sclass_put(: *mut nvif_sclass);
}
extern "C" {
    pub fn nvif_object_mthd(: *mut nvif_object, _arg: u32, : *mut c_void, _arg: u32) -> c_int;
}
extern "C" {
    pub fn nvif_object_unmap_handle(: *mut nvif_object);
}
extern "C" {
    pub fn nvif_object_map(: *mut nvif_object, : *mut c_void, _arg: u32) -> c_int;
}
extern "C" {
    pub fn nvif_object_unmap(: *mut nvif_object);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_mclass {
    pub oclass: i32,
    pub version: c_int,
}

