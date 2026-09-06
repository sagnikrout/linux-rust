//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_vmm.h
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
pub struct nouveau_vma {
    pub vmm: *mut nouveau_vmm,
    pub refs: c_int,
    pub head: list_head,
    pub addr: u64,
    pub mem: *mut nouveau_mem,
    pub fence: *mut nouveau_fence,
}

extern "C" {
    pub fn nouveau_vma_del(: *mut nouveau_vma);
}
extern "C" {
    pub fn nouveau_vma_map(: *mut nouveau_vma, : *mut nouveau_mem) -> c_int;
}
extern "C" {
    pub fn nouveau_vma_unmap(: *mut nouveau_vma);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_vmm {
    pub cli: *mut nouveau_cli,
    pub vmm: nvif_vmm,
    pub svmm: *mut nouveau_svmm,
}

extern "C" {
    pub fn nouveau_vmm_init(: *mut nouveau_cli, oclass: i32, : *mut nouveau_vmm) -> c_int;
}
extern "C" {
    pub fn nouveau_vmm_fini(: *mut nouveau_vmm);
}
