//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_uvmm.h
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
pub struct nouveau_uvmm {
    pub base: drm_gpuvm,
    pub vmm: nouveau_vmm,
    pub region_mt: maple_tree,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_uvma_region {
    pub uvmm: *mut nouveau_uvmm,
    pub addr: u64,
    pub range: u64,
    pub va: },
    pub kref: kref,
    pub complete: completion,
    pub dirty: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_uvma {
    pub va: drm_gpuva,
    pub region: *mut nouveau_uvma_region,
    pub kind: u8,
    pub page_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_uvmm_bind_job {
    pub base: nouveau_job,
    pub kref: kref,
    pub complete: completion,
// struct bind_job_op
    pub ops: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_uvmm_bind_job_args {
    pub file_priv: *mut drm_file,
    pub sched: *mut nouveau_sched,
    pub flags: c_uint,
    pub s: *mut drm_nouveau_sync,
    pub count: u32,
    pub in_sync: },
    pub s: *mut drm_nouveau_sync,
    pub count: u32,
    pub out_sync: },
    pub s: *mut drm_nouveau_vm_bind_op,
    pub count: u32,
    pub op: },
}

extern "C" {
    pub fn nouveau_uvmm_fini(uvmm: *mut nouveau_uvmm);
}
extern "C" {
    pub fn nouveau_uvmm_bo_map_all(nvbov: *mut nouveau_bo, mem: *mut nouveau_mem);
}
extern "C" {
    pub fn nouveau_uvmm_bo_unmap_all(nvbo: *mut nouveau_bo);
}
