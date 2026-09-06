//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_mem.h
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
pub struct nouveau_mem {
    pub base: ttm_resource,
    pub drm: *mut nouveau_drm,
    pub kind: u8,
    pub comp: u8,
    pub mem: nvif_mem,
    pub vma: [nvif_vma; 2],
}

extern "C" {
    pub fn container_of(_arg: reg, nouveau_mem: struct, _arg: base) -> return;
}
extern "C" {
    pub fn nouveau_mem_vram(: *mut ttm_resource, contig: bool, page: u8) -> c_int;
}
extern "C" {
    pub fn nouveau_mem_host(: *mut ttm_resource, : *mut ttm_tt) -> c_int;
}
extern "C" {
    pub fn nouveau_mem_fini(: *mut nouveau_mem);
}
extern "C" {
    pub fn nouveau_mem_map(: *mut nouveau_mem, : *mut nvif_vmm, : *mut nvif_vma) -> c_int;
}
