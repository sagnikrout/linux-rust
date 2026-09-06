//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_vma_resource.h
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
// Copyright © 2021 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_page_sizes {
//
// The sg mask of the pages sg_table. i.e the mask of
// the lengths for each sg entry.
//
    pub phys: c_uint,
//
// The gtt page sizes we are allowed to use given the
// sg mask and the supported page sizes. This will
// express the smallest unit we can use for the whole
// object, as well as the larger sizes we may be able
// to use opportunistically.
//
    pub sg: c_uint,
}

//
// struct i915_vma_bindinfo - Information needed for async bind
// only but that can be dropped after the bind has taken place.
// Consider making this a separate argument to the bind_vma
// op, coalescing with other arguments like vm, stash, cache_level
// and flags
// @pages: The pages sg-table.
// @page_sizes: Page sizes of the pages.
// @pages_rsgt: Refcounted sg-table when delayed object destruction
// is supported. May be NULL.
// @readonly: Whether the vma should be bound read-only.
// @lmem: Whether the vma points to lmem.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_vma_bindinfo {
    pub pages: *mut sg_table,
    pub page_sizes: i915_page_sizes,
    pub pages_rsgt: *mut i915_refct_sgt,
    pub readonly:1: bool,
    pub lmem:1: bool,
}

//
// struct i915_vma_resource - Snapshotted unbind information.
// @unbind_fence: Fence to mark unbinding complete. Note that this fence
// is not considered published until unbind is scheduled, and as such it
// is illegal to access this fence before scheduled unbind other than
// for refcounting.
// @lock: The @unbind_fence lock.
// @hold_count: Number of holders blocking the fence from finishing.
// The vma itself is keeping a hold, which is released when unbind
// is scheduled.
// @work: Work struct for deferred unbind work.
// @chain: Pointer to struct i915_sw_fence used to await dependencies.
// @rb: Rb node for the vm's pending unbind interval tree.
// @__subtree_last: Interval tree private member.
// @wakeref: wakeref.
// @vm: non-refcounted pointer to the vm. This is for internal use only and
// this member is cleared after vm_resource unbind.
// @mr: The memory region of the object pointed to by the vma.
// @ops: Pointer to the backend i915_vma_ops.
// @private: Bind backend private info.
// @start: Offset into the address space of bind range start. Note that
// this is after any padding that might have been allocated.
// @node_size: Size of the allocated range manager node with padding
// subtracted.
// @vma_size: Bind size.
// @guard: The size of guard area preceding and trailing the bind.
// @page_sizes_gtt: Resulting page sizes from the bind operation.
// @bound_flags: Flags indicating binding status.
// @allocated: Backend private data. TODO: Should move into @private.
// @immediate_unbind: Unbind can be done immediately and doesn't need to be
// deferred to a work item awaiting unsignaled fences. This is a hack.
// (dma_fence_work uses a fence flag for this, but this seems slightly
// cleaner).
// @needs_wakeref: Whether a wakeref is needed during unbind. Since we can't
// take a wakeref in the dma-fence signalling critical path, it needs to be
// taken when the unbind is scheduled.
// @skip_pte_rewrite: During ggtt suspend and vm takedown pte rewriting
// needs to be skipped for unbind.
// @tlb: pointer for obj->mm.tlb, if async unbind. Otherwise, NULL
//
// The lifetime of a struct i915_vma_resource is from a binding request to
// the actual possible asynchronous unbind has completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_vma_resource {
    pub unbind_fence: dma_fence,
// See above for description of the lock.
    pub lock: spinlock_t,
    pub hold_count: refcount_t,
    pub work: work_struct,
    pub chain: i915_sw_fence,
    pub rb: rb_node,
    pub __subtree_last: u64,
    pub vm: *mut i915_address_space,
    pub wakeref: intel_wakeref_t,
//
// @bi: Information needed for async bind only but that can be dropped
// after the bind has taken place.
//
// Consider making this a separate argument to the bind_vma op,
// coalescing with other arguments like vm, stash, cache_level and flags
//
    pub bi: i915_vma_bindinfo,

    pub mr: *mut intel_memory_region,

    pub ops: *const i915_vma_ops,
    pub private: *mut c_void,
    pub start: u64,
    pub node_size: u64,
    pub vma_size: u64,
    pub guard: u32,
    pub page_sizes_gtt: u32,
    pub bound_flags: u32,
    pub allocated:1: bool,
    pub immediate_unbind:1: bool,
    pub needs_wakeref:1: bool,
    pub skip_pte_rewrite:1: bool,
    pub tlb: *mut u32,
}

extern "C" {
    pub fn i915_vma_resource_free(vma_res: *mut i915_vma_resource);
}
extern "C" {
    pub fn __i915_vma_resource_init(vma_res: *mut i915_vma_resource);
}
//
// i915_vma_resource_get - Take a reference on a vma resource
// @vma_res: The vma resource on which to take a reference.
//
// Return: The @vma_res pointer
//
// i915_vma_resource_get(struct i915_vma_resource *vma_res)
//
// i915_vma_resource_put - Release a reference to a struct i915_vma_resource
// @vma_res: The resource
//
// i915_vma_resource_init - Initialize a vma resource.
// @vma_res: The vma resource to initialize
// @vm: Pointer to the vm.
// @pages: The pages sg-table.
// @page_sizes: Page sizes of the pages.
// @pages_rsgt: Pointer to a struct i915_refct_sgt of an object with
// delayed destruction.
// @readonly: Whether the vma should be bound read-only.
// @lmem: Whether the vma points to lmem.
// @mr: The memory region of the object the vma points to.
// @ops: The backend ops.
// @private: Bind backend private info.
// @start: Offset into the address space of bind range start after padding.
// @node_size: Size of the allocated range manager node minus padding.
// @size: Bind size.
// @guard: The size of the guard area preceding and trailing the bind.
//
// Initializes a vma resource allocated using i915_vma_resource_alloc().
// The reason for having separate allocate and initialize function is that
// initialization may need to be performed from under a lock where
// allocation is not allowed.
//

extern "C" {
    pub fn i915_vma_resource_bind_dep_sync_all(vm: *mut i915_address_space);
}
extern "C" {
    pub fn i915_vma_resource_module_exit();
}
extern "C" {
    pub fn i915_vma_resource_module_init() -> c_int;
}
