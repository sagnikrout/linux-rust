//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/mmu.h
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
pub struct nvkm_vma {
    pub head: list_head,
    pub tree: rb_node,
    pub addr: u64,
    pub size: u64,
    pub /: *mut *mut bool mapref:1; / PTs (de)referenced on (un)map (vs pre-allocated).,
    pub /: *mut *mut bool sparse:1; / Unmapped PDEs/PTEs will not trigger MMU faults.,
pub const NVKM_VMA_PAGE_NONE: c_int = 7;
    pub /: *mut *mut u8 page:3; / Requested page type (index, or NONE for automatic).,
    pub /: *mut *mut u8 refd:3; / Current page type (index, or NONE for unreferenced).,
    pub /: *mut *mut bool used:1; / Region allocated.,
    pub /: *mut *mut bool part:1; / Region was split from an allocated region by map().,
    pub /: *mut *mut bool busy:1; / Region busy (for temporarily preventing user access).,
    pub /: *mut *mut bool mapped:1; / Region contains valid pages.,
    pub /: *mut *mut bool no_comp:1; / Force no memory compression.,
    pub /: *mut *mut *mut nvkm_memory memory; / Memory currently mapped into VMA.,
    pub /: *mut *mut *mut nvkm_tags tags; / Compression tag reference.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_vmm {
    pub func: *const nvkm_vmm_func,
    pub mmu: *mut nvkm_mmu,
    pub name: *const c_char,
    pub debug: u32,
    pub kref: kref,
    pub vmm: mutex,
    pub ref: mutex,
    pub map: mutex,
    pub mutex: },
    pub start: u64,
    pub limit: u64,
    pub addr: u64,
    pub size: u64,
    pub p: },
    pub addr: u64,
    pub size: u64,
    pub n: },
    pub raw: bool,
    pub managed: },
    pub pd: *mut nvkm_vmm_pt,
    pub join: list_head,
    pub list: list_head,
    pub free: rb_root,
    pub root: rb_root,
    pub bootstrapped: bool,
    pub engref: [core::sync::atomic::AtomicI32; NVKM_SUBDEV_NR],
    pub null: dma_addr_t,
    pub nullp: *mut c_void,
    pub replay: bool,
    pub bar2_pdb: u64,
    pub client: nvkm_gsp_client,
    pub device: nvkm_gsp_device,
    pub object: nvkm_gsp_object,
    pub rsvd: *mut nvkm_vma,
    pub external: bool,
    pub rm: },
}

extern "C" {
    pub fn nvkm_vmm_unref(: *mut nvkm_vmm);
}
extern "C" {
    pub fn nvkm_vmm_boot(: *mut nvkm_vmm) -> c_int;
}
extern "C" {
    pub fn nvkm_vmm_join(: *mut nvkm_vmm, inst: *mut nvkm_memory) -> c_int;
}
extern "C" {
    pub fn nvkm_vmm_part(: *mut nvkm_vmm, inst: *mut nvkm_memory);
}
extern "C" {
    pub fn nvkm_vmm_get(: *mut nvkm_vmm, page: u8, size: u64, : *mut nvkm_vma) -> c_int;
}
extern "C" {
    pub fn nvkm_vmm_put(: *mut nvkm_vmm, : *mut nvkm_vma);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_vmm_map {
    pub memory: *mut nvkm_memory,
    pub offset: u64,
    pub mem: *mut nvkm_mm_node,
    pub sgl: *mut scatterlist,
    pub dma: *mut dma_addr_t,
    pub pfn: *mut u64,
    pub off: u64,
    pub page: *const nvkm_vmm_page,
    pub no_comp: bool,
    pub tags: *mut nvkm_tags,
    pub next: u64,
    pub type: u64,
    pub ctag: u64,
}

extern "C" {
    pub fn nvkm_vmm_unmap(: *mut nvkm_vmm, : *mut nvkm_vma);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_mmu {
    pub func: *const nvkm_mmu_func,
    pub subdev: nvkm_subdev,
    pub dma_bits: u8,
    pub heap_nr: c_int,
pub const NVKM_MEM_VRAM: c_uint = 0x01;
pub const NVKM_MEM_HOST: c_uint = 0x02;
pub const NVKM_MEM_COMP: c_uint = 0x04;
pub const NVKM_MEM_DISP: c_uint = 0x08;
    pub type: u8,
    pub size: u64,
    pub heap: [}; 4],
    pub type_nr: c_int,
pub const NVKM_MEM_KIND: c_uint = 0x10;
pub const NVKM_MEM_MAPPABLE: c_uint = 0x20;
pub const NVKM_MEM_COHERENT: c_uint = 0x40;
pub const NVKM_MEM_UNCACHED: c_uint = 0x80;
    pub type: u8,
    pub heap: u8,
    pub type: [}; 16],
    pub vmm: *mut nvkm_vmm,
    pub mutex: mutex,
    pub list: list_head,
    pub ptp: } ptc,,
    pub /: *mut *mut mutex mutex; / serialises mmu invalidations,
    pub user: nvkm_device_oclass,
}

extern "C" {
    pub fn nv04_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn nv41_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn nv44_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn nv50_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn g84_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn mcp77_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn gf100_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn gk104_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn gk20a_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn gm200_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn gm20b_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn gp100_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn gp10b_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn gv100_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn tu102_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
extern "C" {
    pub fn gh100_mmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mmu) -> c_int;
}
