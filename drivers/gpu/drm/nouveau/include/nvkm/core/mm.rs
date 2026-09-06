//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/mm.h
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
pub struct nvkm_mm_node {
    pub nl_entry: list_head,
    pub fl_entry: list_head,
    pub next: *mut nvkm_mm_node,
pub const NVKM_MM_HEAP_ANY: c_uint = 0x00;
    pub heap: u8,
pub const NVKM_MM_TYPE_NONE: c_uint = 0x00;
pub const NVKM_MM_TYPE_HOLE: c_uint = 0xff;
    pub type: u8,
    pub offset: u32,
    pub length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_mm {
    pub nodes: list_head,
    pub free: list_head,
    pub block_size: u32,
    pub heap_nodes: c_int,
}

extern "C" {
    pub fn nvkm_mm_init(: *mut nvkm_mm, heap: u8, offset: u32, length: u32, block: u32) -> c_int;
}
extern "C" {
    pub fn nvkm_mm_fini(: *mut nvkm_mm) -> c_int;
}
extern "C" {
    pub fn nvkm_mm_free(: *mut nvkm_mm, : *mut nvkm_mm_node);
}
extern "C" {
    pub fn nvkm_mm_dump(: *mut nvkm_mm, : *const c_char);
}
