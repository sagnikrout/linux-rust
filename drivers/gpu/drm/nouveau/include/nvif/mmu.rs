//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/mmu.h
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
pub struct nvif_mmu {
    pub object: nvif_object,
    pub dmabits: u8,
    pub heap_nr: u8,
    pub type_nr: u8,
    pub kind_inv: u8,
    pub kind_nr: u16,
    pub mem: i32,
    pub size: u64,
    pub heap: *mut },
pub const NVIF_MEM_VRAM: c_uint = 0x01;
pub const NVIF_MEM_HOST: c_uint = 0x02;
pub const NVIF_MEM_COMP: c_uint = 0x04;
pub const NVIF_MEM_DISP: c_uint = 0x08;
pub const NVIF_MEM_KIND: c_uint = 0x10;
pub const NVIF_MEM_MAPPABLE: c_uint = 0x20;
pub const NVIF_MEM_COHERENT: c_uint = 0x40;
pub const NVIF_MEM_UNCACHED: c_uint = 0x80;
    pub type: u8,
    pub heap: u8,
    pub type: *mut },
    pub kind: *mut u8,
}

extern "C" {
    pub fn nvif_mmu_dtor(: *mut nvif_mmu);
}
