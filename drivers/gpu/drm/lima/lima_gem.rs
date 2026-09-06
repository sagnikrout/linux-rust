//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/lima/lima_gem.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
// Copyright 2017-2019 Qiang Yu <yuq825@gmail.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_bo {
    pub base: drm_gem_shmem_object,
    pub lock: mutex,
    pub va: list_head,
    pub heap_size: usize,
}

extern "C" {
    pub fn container_of(_arg: to_drm_gem_shmem_obj(obj), lima_bo: struct, _arg: base) -> return;
}
extern "C" {
    pub fn lima_heap_alloc(bo: *mut lima_bo, vm: *mut lima_vm) -> c_int;
}
extern "C" {
    pub fn lima_gem_get_info(file: *mut drm_file, handle: u32, va: *mut u32, offset: *mut u64) -> c_int;
}
extern "C" {
    pub fn lima_gem_submit(file: *mut drm_file, submit: *mut lima_submit) -> c_int;
}
extern "C" {
    pub fn lima_gem_wait(file: *mut drm_file, handle: u32, op: u32, timeout_ns: i64) -> c_int;
}
extern "C" {
    pub fn lima_set_vma_flags(vma: *mut vm_area_struct);
}
