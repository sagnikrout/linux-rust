//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_abi16.h
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
    pub fn nouveau_abi16_ioctl_getparam(_arg: ABI16_IOCTL_ARGS) -> c_int;
}
extern "C" {
    pub fn nouveau_abi16_ioctl_get_zcull_info(_arg: ABI16_IOCTL_ARGS) -> c_int;
}
extern "C" {
    pub fn nouveau_abi16_ioctl_channel_alloc(_arg: ABI16_IOCTL_ARGS) -> c_int;
}
extern "C" {
    pub fn nouveau_abi16_ioctl_channel_free(_arg: ABI16_IOCTL_ARGS) -> c_int;
}
extern "C" {
    pub fn nouveau_abi16_ioctl_grobj_alloc(_arg: ABI16_IOCTL_ARGS) -> c_int;
}
extern "C" {
    pub fn nouveau_abi16_ioctl_notifierobj_alloc(_arg: ABI16_IOCTL_ARGS) -> c_int;
}
extern "C" {
    pub fn nouveau_abi16_ioctl_gpuobj_free(_arg: ABI16_IOCTL_ARGS) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_abi16_ntfy {
    pub object: nvif_object,
    pub head: list_head,
    pub node: *mut nvkm_mm_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_abi16_chan {
    pub head: list_head,
    pub chan: *mut nouveau_channel,
    pub ce: nvif_object,
    pub notifiers: list_head,
    pub ntfy: *mut nouveau_bo,
    pub ntfy_vma: *mut nouveau_vma,
    pub heap: nvkm_mm,
    pub sched: *mut nouveau_sched,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_abi16 {
    pub cli: *mut nouveau_cli,
    pub channels: list_head,
    pub objects: list_head,
}

extern "C" {
    pub fn nouveau_abi16_put(: *mut nouveau_abi16, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nouveau_abi16_fini(: *mut nouveau_abi16);
}
extern "C" {
    pub fn nouveau_abi16_swclass(: *mut nouveau_drm) -> i32;
}
extern "C" {
    pub fn nouveau_abi16_ioctl(: *mut drm_file, user: *mut void __user, size: u32) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_grobj_alloc {
    pub channel: c_int,
    pub handle: u32,
    pub class: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_setparam {
    pub param: u64,
    pub value: u64,
}

