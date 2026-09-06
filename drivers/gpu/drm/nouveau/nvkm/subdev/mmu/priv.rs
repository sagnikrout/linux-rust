//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/mmu/priv.h
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
pub struct nvkm_mmu_func {
    pub ): *mut *mut void (dtor)(struct nvkm_mmu,
    pub ): *mut *mut void (init)(struct nvkm_mmu,
    pub dma_bits: u8,
    pub user: nvkm_sclass,
    pub mmu: },
    pub user: nvkm_sclass,
    pub ): *mut *mut void argv, u32 argc, struct nvkm_memory,
    pub ): *mut *mut *mut u32 argc, u64 addr, u64 size, struct nvkm_vma,
    pub mem: },
    pub user: nvkm_sclass,
    pub ): *const *const char name, struct nvkm_vmm,
    pub global: bool,
    pub pd_offset: u32,
    pub vmm: },
    pub invalid): *const *const *const *const *const u8 (kind)(struct nvkm_mmu , int count, u8,
    pub kind_sys: bool,
    pub ): *mut *mut int (promote_vmm)(struct nvkm_vmm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_mmu_pt {
    pub ptc: *mut nvkm_mmu_ptc,
    pub ptp: *mut nvkm_mmu_ptp,
}

extern "C" {
    pub fn nvkm_mmu_ptc_dump(: *mut nvkm_mmu);
}
extern "C" {
    pub fn nvkm_mmu_ptc_put(: *mut nvkm_mmu, force: bool, : *mut nvkm_mmu_pt);
}
