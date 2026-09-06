//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/object.h
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
pub struct nvkm_object {
    pub func: *const nvkm_object_func,
    pub client: *mut nvkm_client,
    pub engine: *mut nvkm_engine,
    pub oclass: i32,
    pub handle: u32,
    pub head: list_head,
    pub tree: list_head,
    pub object: u64,
    pub node: rb_node,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_object_map {
    NVKM_OBJECT_MAP_IO,
    NVKM_OBJECT_MAP_VA
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_object_func {
    pub ): *mut *mut *mut void (dtor)(struct nvkm_object,
    pub ): *mut *mut int (init)(struct nvkm_object,
    pub suspend): *mut *mut *mut int (fini)(struct nvkm_object , enum nvkm_suspend_state,
    pub size): *mut *mut *mut *mut int (mthd)(struct nvkm_object , u32 mthd, void data, u32,
    pub ): *mut *mut *mut int (ntfy)(struct nvkm_object , u32 mthd, struct nvkm_event,
    pub size): *mut *mut *mut nvkm_object_map , u64 addr, u64,
    pub ): *mut *mut int (unmap)(struct nvkm_object,
    pub ): *mut nvkm_gpuobj,
    pub ): *mut *mut *mut int (sclass)(struct nvkm_object , int index, struct nvkm_oclass,
    pub ): *mut *mut *mut *mut int (uevent)(struct nvkm_object , void argv, u32 argc, struct nvkm_uevent,
}

extern "C" {
    pub fn nvkm_object_del(: *mut nvkm_object);
}
extern "C" {
    pub fn nvkm_object_init(: *mut nvkm_object) -> c_int;
}
extern "C" {
    pub fn nvkm_object_fini(: *mut nvkm_object, nvkm_suspend_state: enum) -> c_int;
}
extern "C" {
    pub fn nvkm_object_mthd(: *mut nvkm_object, mthd: u32, data: *mut c_void, size: u32) -> c_int;
}
extern "C" {
    pub fn nvkm_object_ntfy(: *mut nvkm_object, mthd: u32, : *mut nvkm_event) -> c_int;
}
extern "C" {
    pub fn nvkm_object_unmap(: *mut nvkm_object) -> c_int;
}
extern "C" {
    pub fn nvkm_object_insert(: *mut nvkm_object) -> bool;
}
extern "C" {
    pub fn nvkm_object_remove(: *mut nvkm_object);
}
