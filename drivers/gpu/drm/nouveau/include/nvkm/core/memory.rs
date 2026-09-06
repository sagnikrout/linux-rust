//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/memory.h
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
pub struct nvkm_tags {
    pub mn: *mut nvkm_mm_node,
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_memory_target {
    NVKM_MEM_TARGET_INST_SR_LOST, /* instance memory - not preserved across suspend */
    NVKM_MEM_TARGET_INST, /* instance memory */
    NVKM_MEM_TARGET_VRAM, /* video memory */
    NVKM_MEM_TARGET_HOST, /* coherent system memory */
    NVKM_MEM_TARGET_NCOH, /* non-coherent system memory */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_memory {
    pub func: *const nvkm_memory_func,
    pub ptrs: *const nvkm_memory_ptrs,
    pub kref: kref,
    pub tags: *mut nvkm_tags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_memory_func {
    pub ): *mut *mut *mut void (dtor)(struct nvkm_memory,
    pub ): *mut *mut nvkm_memory_target (target)(struct nvkm_memory,
    pub ): *mut *mut u8 (page)(struct nvkm_memory,
    pub ): *mut *mut u64 (bar2)(struct nvkm_memory,
    pub ): *mut *mut u64 (addr)(struct nvkm_memory,
    pub ): *mut *mut u64 (size)(struct nvkm_memory,
    pub ): *mut *mut *mut void (boot)(struct nvkm_memory , struct nvkm_vmm,
    pub ): *mut *mut *mut void __iomem (acquire)(struct nvkm_memory,
    pub ): *mut *mut void (release)(struct nvkm_memory,
    pub argc): *mut *mut *mut nvkm_vma , void argv, u32,
    pub ): *mut *mut *mut int (kmap)(struct nvkm_memory , struct nvkm_memory,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_memory_ptrs {
    pub offset): *mut *mut *mut u32 (rd32)(struct nvkm_memory , u64,
    pub data): *mut *mut *mut void (wr32)(struct nvkm_memory , u64 offset, u32,
}

extern "C" {
    pub fn nvkm_memory_ctor(: *const nvkm_memory_func, : *mut nvkm_memory);
}
extern "C" {
    pub fn nvkm_memory_unref(: *mut nvkm_memory);
}

// accessor macros - kmap()/done() must bracket use of the other accessor
// macros to guarantee correct behaviour across all chipsets
//

// (_data++) = nvkm_ro32((o), _addr);                            \

