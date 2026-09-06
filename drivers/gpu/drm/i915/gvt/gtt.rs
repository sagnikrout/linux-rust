//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/gtt.h
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


//
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Zhi Wang <zhi.a.wang@intel.com>
// Zhenyu Wang <zhenyuw@linux.intel.com>
// Xiao Zheng <xiao.zheng@intel.com>
//
// Contributors:
// Min He <min.he@intel.com>
// Bing Niu <bing.niu@intel.com>
//

pub const I915_GTT_PAGE_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_gtt_entry {
    pub val64: u64,
    pub type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_gtt_pte_ops {
    pub vgpu): *mut intel_vgpu,
    pub vgpu): *mut intel_vgpu,
    pub e): *mut *mut bool (test_present)(struct intel_gvt_gtt_entry,
    pub e): *mut *mut void (clear_present)(struct intel_gvt_gtt_entry,
    pub e): *mut *mut void (set_present)(struct intel_gvt_gtt_entry,
    pub e): *mut *mut bool (test_pse)(struct intel_gvt_gtt_entry,
    pub e): *mut *mut void (clear_pse)(struct intel_gvt_gtt_entry,
    pub e): *mut *mut bool (test_ips)(struct intel_gvt_gtt_entry,
    pub e): *mut *mut void (clear_ips)(struct intel_gvt_gtt_entry,
    pub e): *mut *mut bool (test_64k_splited)(struct intel_gvt_gtt_entry,
    pub e): *mut *mut void (clear_64k_splited)(struct intel_gvt_gtt_entry,
    pub e): *mut *mut void (set_64k_splited)(struct intel_gvt_gtt_entry,
    pub pfn): *mut *mut *mut void (set_pfn)(struct intel_gvt_gtt_entry e, unsigned long,
    pub e): *mut *mut unsigned long (get_pfn)(struct intel_gvt_gtt_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_gtt_gma_ops {
    pub gma): *mut *mut unsigned long (gma_to_ggtt_pte_index)(unsigned long,
    pub gma): *mut *mut unsigned long (gma_to_pte_index)(unsigned long,
    pub gma): *mut *mut unsigned long (gma_to_pde_index)(unsigned long,
    pub gma): *mut *mut unsigned long (gma_to_l3_pdp_index)(unsigned long,
    pub gma): *mut *mut unsigned long (gma_to_l4_pdp_index)(unsigned long,
    pub gma): *mut *mut unsigned long (gma_to_pml4_index)(unsigned long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_gtt {
    pub pte_ops: *const intel_gvt_gtt_pte_ops,
    pub gma_ops: *const intel_gvt_gtt_gma_ops,
    pub oos_page_use_list_head: list_head,
    pub oos_page_free_list_head: list_head,
    pub ppgtt_mm_lock: mutex,
    pub ppgtt_mm_lru_list_head: list_head,
    pub scratch_page: *mut page,
    pub scratch_mfn: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_gvt_gtt_type {
    GTT_TYPE_INVALID = 0,

    GTT_TYPE_GGTT_PTE,

    GTT_TYPE_PPGTT_PTE_4K_ENTRY,
    GTT_TYPE_PPGTT_PTE_64K_ENTRY,
    GTT_TYPE_PPGTT_PTE_2M_ENTRY,
    GTT_TYPE_PPGTT_PTE_1G_ENTRY,

    GTT_TYPE_PPGTT_PTE_ENTRY,

    GTT_TYPE_PPGTT_PDE_ENTRY,
    GTT_TYPE_PPGTT_PDP_ENTRY,
    GTT_TYPE_PPGTT_PML4_ENTRY,

    GTT_TYPE_PPGTT_ROOT_ENTRY,

    GTT_TYPE_PPGTT_ROOT_L3_ENTRY,
    GTT_TYPE_PPGTT_ROOT_L4_ENTRY,

    GTT_TYPE_PPGTT_ENTRY,

    GTT_TYPE_PPGTT_PTE_PT,
    GTT_TYPE_PPGTT_PDE_PT,
    GTT_TYPE_PPGTT_PDP_PT,
    GTT_TYPE_PPGTT_PML4_PT,

    GTT_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_gvt_mm_type {
    INTEL_GVT_MM_GGTT,
    INTEL_GVT_MM_PPGTT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_partial_pte {
    pub offset: c_ulong,
    pub data: u64,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_mm {
    pub type: intel_gvt_mm_type,
    pub vgpu: *mut intel_vgpu,
    pub ref: kref,
    pub pincount: core::sync::atomic::AtomicI32,
    pub root_entry_type: intel_gvt_gtt_type,
//
// The 4 PDPs in ring context. For 48bit addressing,
// only PDP0 is valid and point to PML4. For 32it
// addressing, all 4 are used as true PDPs.
//
    pub guest_pdps: [u64; GVT_RING_CTX_NR_PDPS],
    pub shadow_pdps: [u64; GVT_RING_CTX_NR_PDPS],
    pub shadowed: bool,
    pub list: list_head,
    pub lru_list: list_head,
    pub /: *mut *mut list_head link; / possible LRI shadow mm list,
    pub ppgtt_mm: },
    pub virtual_ggtt: *mut c_void,
// Save/restore for PM
    pub host_ggtt_aperture: *mut u64,
    pub host_ggtt_hidden: *mut u64,
    pub partial_pte_list: list_head,
    pub ggtt_mm: },
}

extern "C" {
    pub fn _intel_vgpu_mm_release(mm_ref: *mut kref);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_scratch_pt {
    pub page: *mut page,
    pub page_mfn: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_gtt {
    pub ggtt_mm: *mut intel_vgpu_mm,
    pub ppgtt_mm_list_head: list_head,
    pub spt_tree: radix_tree_root,
    pub oos_page_list_head: list_head,
    pub post_shadow_list_head: list_head,
    pub scratch_pt: [intel_vgpu_scratch_pt; GTT_TYPE_MAX],
}

extern "C" {
    pub fn intel_vgpu_init_gtt(vgpu: *mut intel_vgpu) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_clean_gtt(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_vgpu_reset_ggtt(vgpu: *mut intel_vgpu, invalidate_old: bool);
}
extern "C" {
    pub fn intel_vgpu_invalidate_ppgtt(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_init_gtt(gvt: *mut intel_gvt) -> c_int;
}
extern "C" {
    pub fn intel_gvt_clean_gtt(gvt: *mut intel_gvt);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_oos_page {
    pub spt: *mut intel_vgpu_ppgtt_spt,
    pub list: list_head,
    pub vm_list: list_head,
    pub id: c_int,
    pub mem: *mut c_void,
}

pub const GTT_ENTRY_NUM_IN_ONE_PAGE: c_int = 512;
// Represent a vgpu shadow page table.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_ppgtt_spt {
    pub refcount: core::sync::atomic::AtomicI32,
    pub vgpu: *mut intel_vgpu,
    pub type: intel_gvt_gtt_type,
    pub /: *mut *mut bool pde_ips; / for 64KB PTEs,
    pub vaddr: *mut c_void,
    pub page: *mut page,
    pub mfn: c_ulong,
    pub shadow_page: },
    pub type: intel_gvt_gtt_type,
    pub /: *mut *mut bool pde_ips; / for 64KB PTEs,
    pub gfn: c_ulong,
    pub write_cnt: c_ulong,
    pub oos_page: *mut intel_vgpu_oos_page,
    pub guest_page: },
    pub GTT_ENTRY_NUM_IN_ONE_PAGE): DECLARE_BITMAP(post_shadow_bitmap,,
    pub post_shadow_list: list_head,
}

extern "C" {
    pub fn intel_vgpu_sync_oos_pages(vgpu: *mut intel_vgpu) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_flush_post_shadow(vgpu: *mut intel_vgpu) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_pin_mm(mm: *mut intel_vgpu_mm) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_unpin_mm(mm: *mut intel_vgpu_mm);
}
extern "C" {
    pub fn intel_vgpu_put_ppgtt_mm(vgpu: *mut intel_vgpu, pdps[]: u64) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_destroy_all_ppgtt_mm(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_restore_ggtt(gvt: *mut intel_gvt);
}
