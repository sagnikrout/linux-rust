//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/lima/lima_vm.h
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

pub const LIMA_PAGE_SIZE: c_int = 4096;

pub const LIMA_VM_NUM_PT_PER_BT_SHIFT: c_int = 3;

pub const LIMA_VA_RESERVE_START: c_uint = 0x0FFF00000ULL;

pub const LIMA_VA_RESERVE_END: c_uint = 0x100000000ULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_vm_page {
    pub cpu: *mut u32,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_vm {
    pub lock: mutex,
    pub refcount: kref,
    pub mm: drm_mm,
    pub dev: *mut lima_device,
    pub pd: lima_vm_page,
    pub bts: [lima_vm_page; LIMA_VM_NUM_BT],
}

extern "C" {
    pub fn lima_vm_bo_add(vm: *mut lima_vm, bo: *mut lima_bo, create: bool) -> c_int;
}
extern "C" {
    pub fn lima_vm_bo_del(vm: *mut lima_vm, bo: *mut lima_bo);
}
extern "C" {
    pub fn lima_vm_get_va(vm: *mut lima_vm, bo: *mut lima_bo) -> u32;
}
extern "C" {
    pub fn lima_vm_release(kref: *mut kref);
}
extern "C" {
    pub fn lima_vm_print(vm: *mut lima_vm);
}
extern "C" {
    pub fn lima_vm_map_bo(vm: *mut lima_vm, bo: *mut lima_bo, pageoff: c_int) -> c_int;
}
