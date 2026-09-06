//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/mmu/vmm.h
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
pub union nvkm_pte_tracker {
    pub u: u32,
    pub sparse:1: u32,
    pub spte_valid:1: u32,
    pub lpte_valid:1: u32,
    pub lptes:13: u32,
    pub sptes:16: u32,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_vmm_pt {
// Some GPUs have a mapping level with a dual page tables to
// support large and small pages in the same address-range.
//
// We track the state of both page tables in one place, which
// is why there's multiple PT pointers/refcounts here.
//
    pub pt: [*mut nvkm_mmu_pt; 2],
    pub refs: [u32; 2],
// Page size handled by this PT.
//
// Tesla backend needs to know this when writinge PDEs,
// otherwise unnecessary.
//
    pub page: u8,
// Entire page table sparse.
//
// Used to propagate sparseness to child page tables.
//
    pub sparse:1: bool,
// Tracking for page directories.
//
// The array is indexed by PDE, and will either point to the
// child page table, or indicate the PDE is marked as sparse.
//

    pub pde: *mut nvkm_vmm_pt,
// Tracking for dual page tables.
//
// There's one entry for each LPTE, keeping track of whether
// there are valid SPTEs in the same address-range.
//
// This information is used to manage LPTE state transitions.
//
    pub pte: [nvkm_pte_tracker; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_vmm_desc_func {
    pub invalid: nvkm_vmm_pxe_func,
    pub unmap: nvkm_vmm_pxe_func,
    pub sparse: nvkm_vmm_pxe_func,
    pub pde: nvkm_vmm_pde_func,
    pub mem: nvkm_vmm_pte_func,
    pub dma: nvkm_vmm_pte_func,
    pub sgl: nvkm_vmm_pte_func,
    pub pfn: nvkm_vmm_pte_func,
    pub ptes): *mut *mut *mut *mut bool (pfn_clear)(struct nvkm_vmm , struct nvkm_mmu_pt , u32 ptei, u32,
    pub pfn_unmap: nvkm_vmm_pxe_func,
}

extern "C" {
    pub fn gf100_vmm_pgd_pde(: *mut nvkm_vmm, : *mut nvkm_vmm_pt, _arg: u32);
}
extern "C" {
    pub fn gf100_vmm_pgt_unmap(: *mut nvkm_vmm, : *mut nvkm_mmu_pt, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn gk104_vmm_lpt_invalid(: *mut nvkm_vmm, : *mut nvkm_mmu_pt, _arg: u32, _arg: u32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_vmm_desc {
    pub type: },
    pub /: *mut *mut u8 bits; / VMA bits covered by PT.,
    pub /: *mut *mut u8 size; / Bytes-per-PTE.,
    pub /: *mut *mut u32 align; / PT address alignment.,
    pub func: *const nvkm_vmm_desc_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_vmm_page {
    pub shift: u8,
    pub desc: *const nvkm_vmm_desc,
pub const NVKM_VMM_PAGE_SPARSE: c_uint = 0x01;
pub const NVKM_VMM_PAGE_VRAM: c_uint = 0x02;
pub const NVKM_VMM_PAGE_HOST: c_uint = 0x04;
pub const NVKM_VMM_PAGE_COMP: c_uint = 0x08;

    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_vmm_func {
    pub inst): *mut *mut *mut int (join)(struct nvkm_vmm , struct nvkm_memory,
    pub inst): *mut *mut *mut void (part)(struct nvkm_vmm , struct nvkm_memory,
    pub nvkm_memory_target): *mut *mut int (aper)(enum,
    pub ): *mut nvkm_vmm_map,
    pub ): *mut nvkm_vmm_map,
    pub depth): *mut *mut *mut void (flush)(struct nvkm_vmm , int,
    pub argc): *mut *mut u32 mthd, void argv, u32,
    pub addr): *mut *mut *mut void (invalidate_pdb)(struct nvkm_vmm , u64,
    pub page_block: u64,
    pub page: [nvkm_vmm_page; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_vmm_join {
    pub inst: *mut nvkm_memory,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_vmm_put_locked(: *mut nvkm_vmm, : *mut nvkm_vma);
}
extern "C" {
    pub fn nvkm_vmm_unmap_locked(: *mut nvkm_vmm, : *mut nvkm_vma, pfn: bool);
}
extern "C" {
    pub fn nvkm_vmm_unmap_region(: *mut nvkm_vmm, : *mut nvkm_vma);
}
extern "C" {
    pub fn nvkm_vmm_raw_get(vmm: *mut nvkm_vmm, addr: u64, size: u64, refd: u8) -> c_int;
}
extern "C" {
    pub fn nvkm_vmm_raw_put(vmm: *mut nvkm_vmm, addr: u64, size: u64, refd: u8);
}
extern "C" {
    pub fn nvkm_vmm_raw_sparse(: *mut nvkm_vmm, addr: u64, size: u64, ref: bool) -> c_int;
}
pub const NVKM_VMM_PFN_ADDR: c_uint = 0xfffffffffffff000ULL;
pub const NVKM_VMM_PFN_ADDR_SHIFT: c_int = 12;
pub const NVKM_VMM_PFN_APER: c_uint = 0x00000000000000f0ULL;
pub const NVKM_VMM_PFN_HOST: c_uint = 0x0000000000000000ULL;
pub const NVKM_VMM_PFN_VRAM: c_uint = 0x0000000000000010ULL;
pub const NVKM_VMM_PFN_A: c_uint = 0x0000000000000004ULL;
pub const NVKM_VMM_PFN_W: c_uint = 0x0000000000000002ULL;
pub const NVKM_VMM_PFN_V: c_uint = 0x0000000000000001ULL;
pub const NVKM_VMM_PFN_NONE: c_uint = 0x0000000000000000ULL;
extern "C" {
    pub fn nvkm_vmm_pfn_map(: *mut nvkm_vmm, page: u8, addr: u64, size: u64, pfn: *mut u64) -> c_int;
}
extern "C" {
    pub fn nvkm_vmm_pfn_unmap(: *mut nvkm_vmm, addr: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn nv04_vmm_valid(: *mut nvkm_vmm, : *mut c_void, _arg: u32, : *mut nvkm_vmm_map) -> c_int;
}
extern "C" {
    pub fn nv50_vmm_join(: *mut nvkm_vmm, : *mut nvkm_memory) -> c_int;
}
extern "C" {
    pub fn nv50_vmm_part(: *mut nvkm_vmm, : *mut nvkm_memory);
}
extern "C" {
    pub fn nv50_vmm_valid(: *mut nvkm_vmm, : *mut c_void, _arg: u32, : *mut nvkm_vmm_map) -> c_int;
}
extern "C" {
    pub fn nv50_vmm_flush(: *mut nvkm_vmm, _arg: c_int);
}
extern "C" {
    pub fn gf100_vmm_join_(: *mut nvkm_vmm, : *mut nvkm_memory, base: u64) -> c_int;
}
extern "C" {
    pub fn gf100_vmm_join(: *mut nvkm_vmm, : *mut nvkm_memory) -> c_int;
}
extern "C" {
    pub fn gf100_vmm_part(: *mut nvkm_vmm, : *mut nvkm_memory);
}
extern "C" {
    pub fn gf100_vmm_aper(nvkm_memory_target: enum) -> c_int;
}
extern "C" {
    pub fn gf100_vmm_valid(: *mut nvkm_vmm, : *mut c_void, _arg: u32, : *mut nvkm_vmm_map) -> c_int;
}
extern "C" {
    pub fn gf100_vmm_flush(: *mut nvkm_vmm, _arg: c_int);
}
extern "C" {
    pub fn gf100_vmm_invalidate(: *mut nvkm_vmm, type: u32);
}
extern "C" {
    pub fn gf100_vmm_invalidate_pdb(: *mut nvkm_vmm, addr: u64);
}
extern "C" {
    pub fn gk20a_vmm_aper(nvkm_memory_target: enum) -> c_int;
}
extern "C" {
    pub fn gm200_vmm_join_(: *mut nvkm_vmm, : *mut nvkm_memory, base: u64) -> c_int;
}
extern "C" {
    pub fn gm200_vmm_join(: *mut nvkm_vmm, : *mut nvkm_memory) -> c_int;
}
extern "C" {
    pub fn gp100_vmm_join(: *mut nvkm_vmm, : *mut nvkm_memory) -> c_int;
}
extern "C" {
    pub fn gp100_vmm_valid(: *mut nvkm_vmm, : *mut c_void, _arg: u32, : *mut nvkm_vmm_map) -> c_int;
}
extern "C" {
    pub fn gp100_vmm_flush(: *mut nvkm_vmm, _arg: c_int);
}
extern "C" {
    pub fn gp100_vmm_mthd(: *mut nvkm_vmm, : *mut nvkm_client, _arg: u32, : *mut c_void, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gp100_vmm_invalidate_pdb(: *mut nvkm_vmm, addr: u64);
}
extern "C" {
    pub fn gv100_vmm_join(: *mut nvkm_vmm, : *mut nvkm_memory) -> c_int;
}
extern "C" {
    pub fn tu102_vmm_flush(: *mut nvkm_vmm, depth: c_int);
}

// MAP->dma, PAGE_SIZE, MAP->dma++)

