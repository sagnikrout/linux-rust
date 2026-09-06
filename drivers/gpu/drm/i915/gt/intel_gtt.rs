//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_gtt.h
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
//
// Copyright © 2020 Intel Corporation
//
// Please try to maintain the following order within this file unless it makes
// sense to do otherwise. From top to bottom:
// 1. typedefs
// 2. #defines, and macros
// 3. structure definitions
// 4. function prototypes
//
// Within each section, please try to order by generation in ascending order,
// from top to bottom (ie. gen6 on the top, gen8 on the bottom).
//

// Macro flag: #define GTT_TRACE(...)

pub const I915_MAX_NUM_FENCES: c_int = 32;
// 32 fences + sign bit for FENCE_REG_NONE
pub const I915_MAX_NUM_FENCE_BITS: c_int = 6;
pub type gen6_pte_t = u32;
pub type gen8_pte_t = u64;

pub const I915_PDES: c_int = 512;

// gen6-hsw has bit 11-4 for physical addr bit 39-32

pub const GEN6_PDE_SHIFT: c_int = 22;

//
// Cacheability Control is a 4-bit value. The low three bits are stored in bits
// 3:1 of the PTE, while the fourth bit is stored in bit 11 of the PTE.
//

//
// GEN8 32b style address is defined as a 3 level page table:
// 31:30 | 29:21 | 20:12 |  11:0
// PDPE  |  PDE  |  PTE  | offset
// The difference as compared to normal x86 3 level page table is the PDPEs are
// programmed via register.
//
// GEN8 48b style address is defined as a 4 level page table:
// 47:39 | 38:30 | 29:21 | 20:12 |  11:0
// PML4E | PDPE  |  PDE  |  PTE  | offset
//
pub const GEN8_3LVL_PDPES: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_page_table {
    pub base: *mut drm_i915_gem_object,
    pub used: core::sync::atomic::AtomicI32,
    pub stash: *mut i915_page_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_page_directory {
    pub pt: i915_page_table,
    pub lock: spinlock_t,
    pub entry: *mut c_void,
}

extern "C" {
    pub fn __px_dma(p: *mut drm_i915_gem_object) -> dma_addr_t;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_vm_pt_stash {
// preallocated chains of page tables/directories
    pub pt: [*mut i915_page_table; 2],
//
// Optionally override the alignment/size of the physical page that
// contains each PT. If not set defaults back to the usual
// I915_GTT_PAGE_SIZE_4K. This does not influence the other paging
// structures. MUST be a power-of-two. ONLY applicable on discrete
// platforms.
//
    pub pt_sz: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_vma_ops {
// Map an object into an address space with the given cache flags.
    pub flags): u32,
//
// Unmap an object from an address space. This usually consists of
// setting the valid PTE entries to a reserved scratch page.
//
    pub vma_res): *mut i915_vma_resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_address_space {
    pub ref: kref,
    pub release_work: work_struct,
    pub mm: drm_mm,
    pub obj: *mut drm_i915_gem_object,
    pub vma: *mut i915_vma,
    pub rsvd: },
    pub gt: *mut intel_gt,
    pub i915: *mut drm_i915_private,
    pub fpriv: *mut drm_i915_file_private,
    pub dma: *mut device,
    pub /: *mut *mut u64 total; / size addr space maps (ex. 2GB for ggtt),
    pub /: *mut *mut u64 reserved; / size addr space reserved,
    pub 1]: u64 min_alignment[INTEL_MEMORY_STOLEN_LOCAL +,
    pub bind_async_flags: c_uint,
    pub /: *mut *mut mutex mutex; / protects vma and our lists,
    pub /: *mut *mut kref resv_ref; / kref to keep the reservation lock alive.,
    pub /: *mut *mut dma_resv _resv; / reservation lock for all pd objects, and buffer pool,
pub const VM_CLASS_GGTT: c_int = 0;
pub const VM_CLASS_PPGTT: c_int = 1;
pub const VM_CLASS_DPT: c_int = 2;
    pub scratch: [*mut drm_i915_gem_object; 4],
//
// List of vma currently bound.
//
    pub bound_list: list_head,
//
// List of vmas not yet bound or evicted.
//
    pub unbound_list: list_head,
// Global GTT
    pub is_ggtt:1: bool,
// Display page table
    pub is_dpt:1: bool,
// Some systems support read-only mappings for GGTT and/or PPGTT
    pub has_read_only:1: bool,
// Skip pte rewrite on unbind for suspend. Protected by @mutex
    pub skip_pte_rewrite:1: bool,
    pub top: u8,
    pub pd_shift: u8,
    pub scratch_order: u8,
// Flags used when creating page-table objects for this vm
    pub lmem_pt_obj_flags: c_ulong,
// Interval tree for pending unbind vma resources
    pub pending_unbind: rb_root_cached,
    pub sz): *mut *mut *mut (alloc_pt_dma)(struct i915_address_space vm, int,
    pub sz): *mut *mut *mut (alloc_scratch_dma)(struct i915_address_space vm, int,
    pub /: *mut *mut u32 flags); / Create a valid PTE,
    pub is_local): *mut *mut *mut dma_addr_t (pte_decode)(u64 pte, bool is_present, bool,

    pub length): u64 start, u64,
    pub length): u64 start, u64,
    pub length): u64 start, u64,
    pub flags): u32,
    pub flags): u32,
    pub flags): u32,
    pub flags): u32,
    pub is_local): *mut *mut u64 offset, bool is_present, bool,
    pub vm): *mut *mut void (cleanup)(struct i915_address_space,
    pub data): *mut c_void,
    pub vma_ops: i915_vma_ops,
    pub fault_attr): I915_SELFTEST_DECLARE(struct fault_attr,
    pub scrub_64K): I915_SELFTEST_DECLARE(bool,
}

//
// The Graphics Translation Table is the way in which GEN hardware translates a
// Graphics Virtual Address into a Physical Address. In addition to the normal
// collateral associated with any va->pa translations GEN hardware also has a
// portion of the GTT which can be mapped by the CPU and remain both coherent
// and correct (in cases like swizzling). That region is referred to as GMADR in
// the spec.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_ggtt {
    pub vm: i915_address_space,
    pub /: *mut *mut io_mapping iomap; / Mapping to our CPU mappable region,
    pub /: *mut *mut resource gmadr; / GMADR resource,
    pub /: *mut *mut resource_size_t mappable_end; / End offset that we can CPU map,
// "Graphics Stolen Memory" holds the global PTEs
    pub gsm: *mut void __iomem,
    pub ggtt): *mut *mut void (invalidate)(struct i915_ggtt,
// PPGTT used for aliasing the PPGTT with the GTT
    pub alias: *mut i915_ppgtt,
    pub do_idle_maps: bool,
    pub mtrr: c_int,
// Bit 6 swizzling required for X tiling
    pub bit_6_swizzle_x: u32,
// Bit 6 swizzling required for Y tiling
    pub bit_6_swizzle_y: u32,
    pub pin_bias: u32,
    pub num_fences: c_uint,
    pub fence_regs: *mut i915_fence_reg,
    pub fence_list: list_head,
//
// List of all objects in gtt_space, currently mmaped by userspace.
// All objects within this list must also be on bound_list.
//
    pub userfault_list: list_head,
    pub error_mutex: mutex,
    pub error_capture: drm_mm_node,
    pub uc_fw: drm_mm_node,
// List of GTs mapping this GGTT
    pub gt_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_ppgtt {
    pub vm: i915_address_space,
    pub pd: *mut i915_page_directory,
}

extern "C" {
    pub fn intel_vm_no_concurrent_access_wa(i915: *mut drm_i915_private) -> bool;
}
// avoid INTEL_MEMORY_MOCK overflow
extern "C" {
    pub fn i915_vm_min_alignment(_arg: vm, _arg: type) -> return;
}
extern "C" {
    pub fn container_of(_arg: vm, i915_ggtt: struct, _arg: vm) -> return;
}
extern "C" {
    pub fn container_of(_arg: vm, i915_ppgtt: struct, _arg: vm) -> return;
}
//
// i915_vm_resv_get - Obtain a reference on the vm's reservation lock
// @vm: The vm whose reservation lock we want to share.
//
// Return: A pointer to the vm's reservation lock.
//
extern "C" {
    pub fn i915_vm_release(kref: *mut kref);
}
extern "C" {
    pub fn i915_vm_resv_release(kref: *mut kref);
}
//
// i915_vm_resv_put - Release a reference on the vm's reservation lock
// @vm: The vm whose reservation lock reference we want to release
//
extern "C" {
    pub fn i915_address_space_init(vm: *mut i915_address_space, subclass: c_int);
}
extern "C" {
    pub fn i915_address_space_fini(vm: *mut i915_address_space);
}
//
// Helper to counts the number of PTEs within the given length. This count
// does not cross a page table boundary, so the max value would be
// GEN6_PTES for GEN6, and GEN8_PTES for GEN8.
//
extern "C" {
    pub fn NUM_PTE(i915_pte_index(addr: pde_shift) -, _arg: pde_shift) -> return;
}
extern "C" {
    pub fn i915_pte_index(_arg: end, i915_pte_index(addr: pde_shift) -, _arg: pde_shift) -> return;
}
extern "C" {
    pub fn __px_dma(ppgtt->vm.scratch[ppgtt->vm.top]: pt ? px_base(pt) :) -> return;
}
extern "C" {
    pub fn i915_ggtt_probe_hw(i915: *mut drm_i915_private) -> c_int;
}
extern "C" {
    pub fn i915_ggtt_init_hw(i915: *mut drm_i915_private) -> c_int;
}
extern "C" {
    pub fn i915_ggtt_enable_hw(i915: *mut drm_i915_private) -> c_int;
}
extern "C" {
    pub fn i915_init_ggtt(i915: *mut drm_i915_private) -> c_int;
}
extern "C" {
    pub fn i915_ggtt_driver_release(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn i915_ggtt_driver_late_release(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn i915_ppgtt_init_hw(gt: *mut intel_gt) -> c_int;
}
extern "C" {
    pub fn i915_ggtt_suspend_vm(vm: *mut i915_address_space, evict_all: bool);
}
extern "C" {
    pub fn i915_ggtt_resume_vm(vm: *mut i915_address_space, all_evicted: bool) -> bool;
}
extern "C" {
    pub fn i915_ggtt_suspend(gtt: *mut i915_ggtt);
}
extern "C" {
    pub fn i915_ggtt_resume(ggtt: *mut i915_ggtt);
}

extern "C" {
    pub fn setup_scratch_page(vm: *mut i915_address_space) -> c_int;
}
extern "C" {
    pub fn free_scratch(vm: *mut i915_address_space);
}
extern "C" {
    pub fn map_pt_dma(vm: *mut i915_address_space, obj: *mut drm_i915_gem_object) -> c_int;
}
extern "C" {
    pub fn map_pt_dma_locked(vm: *mut i915_address_space, obj: *mut drm_i915_gem_object) -> c_int;
}

extern "C" {
    pub fn gen6_ggtt_invalidate(ggtt: *mut i915_ggtt);
}
extern "C" {
    pub fn gtt_write_workarounds(gt: *mut intel_gt);
}
extern "C" {
    pub fn setup_private_pat(gt: *mut intel_gt);
}
extern "C" {
    pub fn i915_ggtt_require_binder(i915: *mut drm_i915_private) -> bool;
}
