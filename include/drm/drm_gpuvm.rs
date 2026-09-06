//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_gpuvm.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Copyright (c) 2022 Red Hat.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

//
// enum drm_gpuva_flags - flags for struct drm_gpuva
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_gpuva_flags {
//
// @DRM_GPUVA_INVALIDATED:
//
// Flag indicating that the &drm_gpuva's backing GEM is invalidated.
//
    DRM_GPUVA_INVALIDATED = (1 << 0),

//
// @DRM_GPUVA_SPARSE:
//
// Flag indicating that the &drm_gpuva is a sparse mapping.
//
    DRM_GPUVA_SPARSE = (1 << 1),

//
// @DRM_GPUVA_USERBITS: user defined bits
//
    DRM_GPUVA_USERBITS = (1 << 2),
}

//
// struct drm_gpuva - structure to track a GPU VA mapping
//
// This structure represents a GPU VA mapping and is associated with a
// &drm_gpuvm.
//
// Typically, this structure is embedded in bigger driver structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuva {
//
// @vm: the &drm_gpuvm this object is associated with
//
    pub vm: *mut drm_gpuvm,
//
// @vm_bo: the &drm_gpuvm_bo abstraction for the mapped
// &drm_gem_object
//
    pub vm_bo: *mut drm_gpuvm_bo,
//
// @flags: the &drm_gpuva_flags for this mapping
//
    pub flags: drm_gpuva_flags,
//
// @va: structure containing the address and range of the &drm_gpuva
//
// @va.addr: the start address
//
    pub addr: u64,
//
// @range: the range
//
    pub range: u64,
    pub va: },
//
// @gem: structure containing the &drm_gem_object and its offset
//
// @gem.offset: the offset within the &drm_gem_object
//
    pub offset: u64,
//
// @gem.obj: the mapped &drm_gem_object
//
    pub obj: *mut drm_gem_object,
//
// @gem.entry: the &list_head to attach this object to a &drm_gpuvm_bo
//
    pub entry: list_head,
    pub gem: },
//
// @rb: structure containing data to store &drm_gpuvas in a rb-tree
//
// @rb.node: the rb-tree node
//
    pub node: rb_node,
//
// @rb.entry: The &list_head to additionally connect &drm_gpuvas
// in the same order they appear in the interval tree. This is
// useful to keep iterating &drm_gpuvas from a start node found
// through the rb-tree while doing modifications on the rb-tree
// itself.
//
    pub entry: list_head,
//
// @rb.__subtree_last: needed by the interval tree, holding last-in-subtree
//
    pub __subtree_last: u64,
    pub rb: },
}

extern "C" {
    pub fn drm_gpuva_insert(gpuvm: *mut drm_gpuvm, va: *mut drm_gpuva) -> c_int;
}
extern "C" {
    pub fn drm_gpuva_remove(va: *mut drm_gpuva);
}
extern "C" {
    pub fn drm_gpuva_link(va: *mut drm_gpuva, vm_bo: *mut drm_gpuvm_bo);
}
extern "C" {
    pub fn drm_gpuva_unlink(va: *mut drm_gpuva);
}
extern "C" {
    pub fn drm_gpuva_unlink_defer(va: *mut drm_gpuva);
}
//
// drm_gpuva_invalidate() - sets whether the backing GEM of this &drm_gpuva is
// invalidated
// @va: the &drm_gpuva to set the invalidate flag for
// @invalidate: indicates whether the &drm_gpuva is invalidated
//
// drm_gpuva_invalidated() - indicates whether the backing BO of this &drm_gpuva
// is invalidated
// @va: the &drm_gpuva to check
//
// Returns: %true if the GPU VA is invalidated, %false otherwise
//
// enum drm_gpuvm_flags - flags for struct drm_gpuvm
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_gpuvm_flags {
//
// @DRM_GPUVM_RESV_PROTECTED: GPUVM is protected externally by the
// GPUVM's &dma_resv lock
//
    DRM_GPUVM_RESV_PROTECTED = BIT(0),

//
// @DRM_GPUVM_IMMEDIATE_MODE: use the locking scheme for GEMs designed
// for modifying the GPUVM during the fence signalling path
//
// When set, gpuva.lock is used to protect gpuva.list in all GEM
// objects associated with this GPUVM. Otherwise, the GEMs dma-resv is
// used.
//
    DRM_GPUVM_IMMEDIATE_MODE = BIT(1),

//
// @DRM_GPUVM_USERBITS: user defined bits
//
    DRM_GPUVM_USERBITS = BIT(2),
}

//
// struct drm_gpuvm - DRM GPU VA Manager
//
// The DRM GPU VA Manager keeps track of a GPU's virtual address space by using
// &maple_tree structures. Typically, this structure is embedded in bigger
// driver structures.
//
// Drivers can pass addresses and ranges in an arbitrary unit, e.g. bytes or
// pages.
//
// There should be one manager instance per GPU virtual address space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuvm {
//
// @name: the name of the DRM GPU VA space
//
    pub name: *const c_char,
//
// @flags: the &drm_gpuvm_flags of this GPUVM
//
    pub flags: drm_gpuvm_flags,
//
// @drm: the &drm_device this VM lives in
//
    pub drm: *mut drm_device,
//
// @mm_start: start of the VA space
//
    pub mm_start: u64,
//
// @mm_range: length of the VA space
//
    pub mm_range: u64,
//
// @rb: structures to track &drm_gpuva entries
//
// @rb.tree: the rb-tree to track GPU VA mappings
//
    pub tree: rb_root_cached,
//
// @rb.list: the &list_head to track GPU VA mappings
//
    pub list: list_head,
    pub rb: },
//
// @kref: reference count of this object
//
    pub kref: kref,
//
// @kernel_alloc_node:
//
// &drm_gpuva representing the address space cutout reserved for
// the kernel
//
    pub kernel_alloc_node: drm_gpuva,
//
// @ops: &drm_gpuvm_ops providing the split/merge steps to drivers
//
    pub ops: *const drm_gpuvm_ops,
//
// @r_obj: Resv GEM object; representing the GPUVM's common &dma_resv.
//
    pub r_obj: *mut drm_gem_object,
//
// @extobj: structure holding the extobj list
//
// @extobj.list: &list_head storing &drm_gpuvm_bos serving as
// external object
//
    pub list: list_head,
//
// @extobj.local_list: pointer to the local list temporarily
// storing entries from the external object list
//
    pub local_list: *mut list_head,
//
// @extobj.lock: spinlock to protect the extobj list
//
    pub lock: spinlock_t,
    pub extobj: },
//
// @evict: structure holding the evict list and evict list lock
//
// @evict.list: &list_head storing &drm_gpuvm_bos currently
// being evicted
//
    pub list: list_head,
//
// @evict.local_list: pointer to the local list temporarily
// storing entries from the evicted object list
//
    pub local_list: *mut list_head,
//
// @evict.lock: spinlock to protect the evict list
//
    pub lock: spinlock_t,
    pub evict: },
//
// @bo_defer: structure holding vm_bos that need to be destroyed
//
    pub bo_defer: llist_head,
}

//
// drm_gpuvm_get() - acquire a struct drm_gpuvm reference
// @gpuvm: the &drm_gpuvm to acquire the reference of
//
// This function acquires an additional reference to @gpuvm. It is illegal to
// call this without already holding a reference. No locks required.
//
// Returns: the &struct drm_gpuvm pointer
//
extern "C" {
    pub fn drm_gpuvm_put(gpuvm: *mut drm_gpuvm);
}
extern "C" {
    pub fn drm_gpuvm_range_valid(gpuvm: *mut drm_gpuvm, addr: u64, range: u64) -> bool;
}
extern "C" {
    pub fn drm_gpuvm_interval_empty(gpuvm: *mut drm_gpuvm, addr: u64, range: u64) -> bool;
}
//
// drm_gpuvm_resv_protected() - indicates whether &DRM_GPUVM_RESV_PROTECTED is
// set
// @gpuvm: the &drm_gpuvm
//
// Returns: true if &DRM_GPUVM_RESV_PROTECTED is set, false otherwise.
//
// drm_gpuvm_immediate_mode() - indicates whether &DRM_GPUVM_IMMEDIATE_MODE is
// set
// @gpuvm: the &drm_gpuvm
//
// Returns: true if &DRM_GPUVM_IMMEDIATE_MODE is set, false otherwise.
//
// drm_gpuvm_resv() - returns the &drm_gpuvm's &dma_resv
// @gpuvm__: the &drm_gpuvm
//
// Returns: a pointer to the &drm_gpuvm's shared &dma_resv
//

//
// drm_gpuvm_resv_obj() - returns the &drm_gem_object holding the &drm_gpuvm's
// &dma_resv
// @gpuvm__: the &drm_gpuvm
//
// Returns: a pointer to the &drm_gem_object holding the &drm_gpuvm's shared
// &dma_resv
//

//
// drm_gpuvm_is_extobj() - indicates whether the given &drm_gem_object is an
// external object
// @gpuvm: the &drm_gpuvm to check
// @obj: the &drm_gem_object to check
//
// Returns: true if the &drm_gem_object &dma_resv differs from the
// &drm_gpuvms &dma_resv, false otherwise
//
extern "C" {
    pub fn list_next_entry(_arg: va, _arg: rb.entry) -> return;
}
//
// drm_gpuvm_for_each_va_range() - iterate over a range of &drm_gpuvas
// @va__: &drm_gpuva structure to assign to in each iteration step
// @gpuvm__: &drm_gpuvm to walk over
// @start__: starting offset, the first gpuva will overlap this
// @end__: ending offset, the last gpuva will start before this (but may
// overlap)
//
// This iterator walks over all &drm_gpuvas in the &drm_gpuvm that lie
// between @start__ and @end__. It is implemented similarly to list_for_each(),
// but is using the &drm_gpuvm's internal interval tree to accelerate
// the search for the starting &drm_gpuva, and hence isn't safe against removal
// of elements. It assumes that @end__ is within (or is the upper limit of) the
// &drm_gpuvm. This iterator does not skip over the &drm_gpuvm's
// @kernel_alloc_node.
//

//
// drm_gpuvm_for_each_va_range_safe() - safely iterate over a range of
// &drm_gpuvas
// @va__: &drm_gpuva to assign to in each iteration step
// @next__: another &drm_gpuva to use as temporary storage
// @gpuvm__: &drm_gpuvm to walk over
// @start__: starting offset, the first gpuva will overlap this
// @end__: ending offset, the last gpuva will start before this (but may
// overlap)
//
// This iterator walks over all &drm_gpuvas in the &drm_gpuvm that lie
// between @start__ and @end__. It is implemented similarly to
// list_for_each_safe(), but is using the &drm_gpuvm's internal interval
// tree to accelerate the search for the starting &drm_gpuva, and hence is safe
// against removal of elements. It assumes that @end__ is within (or is the
// upper limit of) the &drm_gpuvm. This iterator does not skip over the
// &drm_gpuvm's @kernel_alloc_node.
//

//
// drm_gpuvm_for_each_va() - iterate over all &drm_gpuvas
// @va__: &drm_gpuva to assign to in each iteration step
// @gpuvm__: &drm_gpuvm to walk over
//
// This iterator walks over all &drm_gpuva structures associated with the given
// &drm_gpuvm.
//

//
// drm_gpuvm_for_each_va_safe() - safely iterate over all &drm_gpuvas
// @va__: &drm_gpuva to assign to in each iteration step
// @next__: another &drm_gpuva to use as temporary storage
// @gpuvm__: &drm_gpuvm to walk over
//
// This iterator walks over all &drm_gpuva structures associated with the given
// &drm_gpuvm. It is implemented with list_for_each_entry_safe(), and
// hence safe against the removal of elements.
//

//
// struct drm_gpuvm_exec - &drm_gpuvm abstraction of &drm_exec
//
// This structure should be created on the stack as &drm_exec should be.
//
// Optionally, @extra can be set in order to lock additional &drm_gem_objects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuvm_exec {
//
// @exec: the &drm_exec structure
//
    pub exec: drm_exec,
//
// @flags: the flags for the struct drm_exec
//
    pub flags: u32,
//
// @vm: the &drm_gpuvm to lock its DMA reservations
//
    pub vm: *mut drm_gpuvm,
//
// @num_fences: the number of fences to reserve for the &dma_resv of the
// locked &drm_gem_objects
//
    pub num_fences: c_uint,
//
// @extra: Callback and corresponding private data for the driver to
// lock arbitrary additional &drm_gem_objects.
//
// @extra.fn: The driver callback to lock additional
// &drm_gem_objects.
//
    pub vm_exec): *mut *mut int (fn)(struct drm_gpuvm_exec,
//
// @extra.priv: driver private data for the @fn callback
//
    pub priv: *mut c_void,
    pub extra: },
}

extern "C" {
    pub fn drm_gpuvm_exec_lock(vm_exec: *mut drm_gpuvm_exec) -> c_int;
}
//
// drm_gpuvm_exec_unlock() - lock all dma-resv of all assoiciated BOs
// @vm_exec: the &drm_gpuvm_exec wrapper
//
// Releases all dma-resv locks of all &drm_gem_objects previously acquired
// through drm_gpuvm_exec_lock() or its variants.
//
// Returns: 0 on success, negative error code on failure.
//
extern "C" {
    pub fn drm_gpuvm_validate(gpuvm: *mut drm_gpuvm, exec: *mut drm_exec) -> c_int;
}
//
// drm_gpuvm_exec_resv_add_fence() - add fence to private and all extobj
// @vm_exec: the &drm_gpuvm_exec wrapper
// @fence: fence to add
// @private_usage: private dma-resv usage
// @extobj_usage: extobj dma-resv usage
//
// See drm_gpuvm_resv_add_fence().
//
// drm_gpuvm_exec_validate() - validate all BOs marked as evicted
// @vm_exec: the &drm_gpuvm_exec wrapper
//
// See drm_gpuvm_validate().
//
// Returns: 0 on success, negative error code on failure.
//
extern "C" {
    pub fn drm_gpuvm_validate(_arg: vm_exec->vm, _arg: &vm_exec->exec) -> return;
}
//
// struct drm_gpuvm_bo - structure representing a &drm_gpuvm and
// &drm_gem_object combination
//
// This structure is an abstraction representing a &drm_gpuvm and
// &drm_gem_object combination. It serves as an indirection to accelerate
// iterating all &drm_gpuvas within a &drm_gpuvm backed by the same
// &drm_gem_object.
//
// Furthermore it is used cache evicted GEM objects for a certain GPU-VM to
// accelerate validation.
//
// Typically, drivers want to create an instance of a struct drm_gpuvm_bo once
// a GEM object is mapped first in a GPU-VM and release the instance once the
// last mapping of the GEM object in this GPU-VM is unmapped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuvm_bo {
//
// @vm: The &drm_gpuvm the @obj is mapped in. This is a reference
// counted pointer.
//
    pub vm: *mut drm_gpuvm,
//
// @obj: The &drm_gem_object being mapped in @vm. This is a reference
// counted pointer.
//
    pub obj: *mut drm_gem_object,
//
// @evicted: Indicates whether the &drm_gem_object is evicted; field
// protected by the &drm_gem_object's dma-resv lock.
//
    pub evicted: bool,
//
// @kref: The reference count for this &drm_gpuvm_bo.
//
    pub kref: kref,
//
// @list: Structure containing all &list_heads.
//
// @list.gpuva: The list of linked &drm_gpuvas.
//
// It is safe to access entries from this list as long as the
// GEM's gpuva lock is held. See also struct drm_gem_object.
//
    pub gpuva: list_head,
//
// @list.entry: Structure containing all &list_heads serving as
// entry.
//
// @list.entry.gem: List entry to attach to the
// &drm_gem_objects gpuva list.
//
    pub gem: list_head,
//
// @list.entry.evict: List entry to attach to the
// &drm_gpuvms extobj list.
//
    pub extobj: list_head,
//
// @list.entry.evict: List entry to attach to the
// &drm_gpuvms evict list.
//
    pub evict: list_head,
//
// @list.entry.bo_defer: List entry to attach to
// the &drm_gpuvms bo_defer list.
//
    pub bo_defer: llist_node,
    pub entry: },
    pub list: },
}

//
// drm_gpuvm_bo_get() - acquire a struct drm_gpuvm_bo reference
// @vm_bo: the &drm_gpuvm_bo to acquire the reference of
//
// This function acquires an additional reference to @vm_bo. It is illegal to
// call this without already holding a reference. No locks required.
//
// Returns: the &struct vm_bo pointer
//
extern "C" {
    pub fn drm_gpuvm_bo_put(vm_bo: *mut drm_gpuvm_bo) -> bool;
}
extern "C" {
    pub fn drm_gpuvm_bo_put_deferred(vm_bo: *mut drm_gpuvm_bo) -> bool;
}
extern "C" {
    pub fn drm_gpuvm_bo_deferred_cleanup(gpuvm: *mut drm_gpuvm);
}
extern "C" {
    pub fn drm_gpuvm_bo_evict(vm_bo: *mut drm_gpuvm_bo, evict: bool);
}
//
// drm_gpuvm_bo_gem_evict() - add/remove all &drm_gpuvm_bo's in the list
// to/from the &drm_gpuvms evicted list
// @obj: the &drm_gem_object
// @evict: indicates whether @obj is evicted
//
// See drm_gpuvm_bo_evict().
//
extern "C" {
    pub fn drm_gpuvm_bo_extobj_add(vm_bo: *mut drm_gpuvm_bo);
}
//
// drm_gpuvm_bo_for_each_va() - iterator to walk over a list of &drm_gpuva
// @va__: &drm_gpuva structure to assign to in each iteration step
// @vm_bo__: the &drm_gpuvm_bo the &drm_gpuva to walk are associated with
//
// This iterator walks over all &drm_gpuva structures associated with the
// &drm_gpuvm_bo.
//
// The caller must hold the GEM's gpuva lock.
//

//
// drm_gpuvm_bo_for_each_va_safe() - iterator to safely walk over a list of
// &drm_gpuva
// @va__: &drm_gpuva structure to assign to in each iteration step
// @next__: &next &drm_gpuva to store the next step
// @vm_bo__: the &drm_gpuvm_bo the &drm_gpuva to walk are associated with
//
// This iterator walks over all &drm_gpuva structures associated with the
// &drm_gpuvm_bo. It is implemented with list_for_each_entry_safe(), hence
// it is save against removal of elements.
//
// The caller must hold the GEM's gpuva lock.
//

//
// enum drm_gpuva_op_type - GPU VA operation type
//
// Operations to alter the GPU VA mappings tracked by the &drm_gpuvm.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_gpuva_op_type {
//
// @DRM_GPUVA_OP_MAP: the map op type
//
    DRM_GPUVA_OP_MAP,

//
// @DRM_GPUVA_OP_REMAP: the remap op type
//
    DRM_GPUVA_OP_REMAP,

//
// @DRM_GPUVA_OP_UNMAP: the unmap op type
//
    DRM_GPUVA_OP_UNMAP,

//
// @DRM_GPUVA_OP_PREFETCH: the prefetch op type
//
    DRM_GPUVA_OP_PREFETCH,

//
// @DRM_GPUVA_OP_DRIVER: the driver defined op type
//
    DRM_GPUVA_OP_DRIVER,
}

//
// struct drm_gpuva_op_map - GPU VA map operation
//
// This structure represents a single map operation generated by the
// DRM GPU VA manager.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuva_op_map {
//
// @va: structure containing address and range of a map
// operation
//
// @va.addr: the base address of the new mapping
//
    pub addr: u64,
//
// @va.range: the range of the new mapping
//
    pub range: u64,
    pub va: },
//
// @gem: structure containing the &drm_gem_object and its offset
//
// @gem.offset: the offset within the &drm_gem_object
//
    pub offset: u64,
//
// @gem.obj: the &drm_gem_object to map
//
    pub obj: *mut drm_gem_object,
    pub gem: },
}

//
// struct drm_gpuva_op_unmap - GPU VA unmap operation
//
// This structure represents a single unmap operation generated by the
// DRM GPU VA manager.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuva_op_unmap {
//
// @va: the &drm_gpuva to unmap
//
    pub va: *mut drm_gpuva,
//
// @keep:
//
// Indicates whether this &drm_gpuva is physically contiguous with the
// original mapping request.
//
// Optionally, if &keep is set, drivers may keep the actual page table
// mappings for this &drm_gpuva, adding the missing page table entries
// only and update the &drm_gpuvm accordingly.
//
    pub keep: bool,
}

//
// struct drm_gpuva_op_remap - GPU VA remap operation
//
// This represents a single remap operation generated by the DRM GPU VA manager.
//
// A remap operation is generated when an existing GPU VA mmapping is split up
// by inserting a new GPU VA mapping or by partially unmapping existent
// mapping(s), hence it consists of a maximum of two map and one unmap
// operation.
//
// The @unmap operation takes care of removing the original existing mapping.
// @prev is used to remap the preceding part, @next the subsequent part.
//
// If either a new mapping's start address is aligned with the start address
// of the old mapping or the new mapping's end address is aligned with the
// end address of the old mapping, either @prev or @next is NULL.
// This will also be the case when the requested mapping begins before the
// old mapping's start address or stretches beyond its end address.
//
// Note, the reason for a dedicated remap operation, rather than arbitrary
// unmap and map operations, is to give drivers the chance of extracting driver
// specific data for creating the new mappings from the unmap operations's
// &drm_gpuva structure which typically is embedded in larger driver specific
// structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuva_op_remap {
//
// @prev: the preceding part of a split mapping
//
    pub prev: *mut drm_gpuva_op_map,
//
// @next: the subsequent part of a split mapping
//
    pub next: *mut drm_gpuva_op_map,
//
// @unmap: the unmap operation for the original existing mapping
//
    pub unmap: *mut drm_gpuva_op_unmap,
}

//
// struct drm_gpuva_op_prefetch - GPU VA prefetch operation
//
// This structure represents a single prefetch operation generated by the
// DRM GPU VA manager.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuva_op_prefetch {
//
// @va: the &drm_gpuva to prefetch
//
    pub va: *mut drm_gpuva,
}

//
// struct drm_gpuva_op - GPU VA operation
//
// This structure represents a single generic operation.
//
// The particular type of the operation is defined by @op.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuva_op {
//
// @entry:
//
// The &list_head used to distribute instances of this struct within
// &drm_gpuva_ops.
//
    pub entry: list_head,
//
// @op: the type of the operation
//
    pub op: drm_gpuva_op_type,
//
// @map: the map operation
//
    pub map: drm_gpuva_op_map,
//
// @remap: the remap operation
//
    pub remap: drm_gpuva_op_remap,
//
// @unmap: the unmap operation
//
    pub unmap: drm_gpuva_op_unmap,
//
// @prefetch: the prefetch operation
//
    pub prefetch: drm_gpuva_op_prefetch,
}

//
// struct drm_gpuva_ops - wraps a list of &drm_gpuva_op
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuva_ops {
//
// @list: the &list_head
//
    pub list: list_head,
}

//
// drm_gpuva_for_each_op() - iterator to walk over &drm_gpuva_ops
// @op: &drm_gpuva_op to assign in each iteration step
// @ops: &drm_gpuva_ops to walk
//
// This iterator walks over all ops within a given list of operations.
//

//
// drm_gpuva_for_each_op_safe() - iterator to safely walk over &drm_gpuva_ops
// @op: &drm_gpuva_op to assign in each iteration step
// @next: &next &drm_gpuva_op to store the next step
// @ops: &drm_gpuva_ops to walk
//
// This iterator walks over all ops within a given list of operations. It is
// implemented with list_for_each_safe(), so save against removal of elements.
//

//
// drm_gpuva_for_each_op_from_reverse() - iterate backwards from the given point
// @op: &drm_gpuva_op to assign in each iteration step
// @ops: &drm_gpuva_ops to walk
//
// This iterator walks over all ops within a given list of operations beginning
// from the given operation in reverse order.
//

//
// drm_gpuva_for_each_op_reverse - iterator to walk over &drm_gpuva_ops in reverse
// @op: &drm_gpuva_op to assign in each iteration step
// @ops: &drm_gpuva_ops to walk
//
// This iterator walks over all ops within a given list of operations in reverse
//

//
// drm_gpuva_first_op() - returns the first &drm_gpuva_op from &drm_gpuva_ops
// @ops: the &drm_gpuva_ops to get the fist &drm_gpuva_op from
//

//
// drm_gpuva_last_op() - returns the last &drm_gpuva_op from &drm_gpuva_ops
// @ops: the &drm_gpuva_ops to get the last &drm_gpuva_op from
//

//
// drm_gpuva_prev_op() - previous &drm_gpuva_op in the list
// @op: the current &drm_gpuva_op
//

//
// drm_gpuva_next_op() - next &drm_gpuva_op in the list
// @op: the current &drm_gpuva_op
//

//
// struct drm_gpuvm_map_req - arguments passed to drm_gpuvm_sm_map[_ops_create]()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuvm_map_req {
//
// @map: struct drm_gpuva_op_map
//
    pub map: drm_gpuva_op_map,
}

//
// struct drm_gpuvm_ops - callbacks for split/merge steps
//
// This structure defines the callbacks used by &drm_gpuvm_sm_map and
// &drm_gpuvm_sm_unmap to provide the split/merge steps for map and unmap
// operations to drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpuvm_ops {
//
// @vm_free: called when the last reference of a struct drm_gpuvm is
// dropped
//
// This callback is mandatory.
//
    pub gpuvm): *mut *mut void (vm_free)(struct drm_gpuvm,
//
// @op_alloc: called when the &drm_gpuvm allocates
// a struct drm_gpuva_op
//
// Some drivers may want to embed struct drm_gpuva_op into driver
// specific structures. By implementing this callback drivers can
// allocate memory accordingly.
//
// This callback is optional.
//
    pub (*op_alloc)(void): *mut drm_gpuva_op,
//
// @op_free: called when the &drm_gpuvm frees a
// struct drm_gpuva_op
//
// Some drivers may want to embed struct drm_gpuva_op into driver
// specific structures. By implementing this callback drivers can
// free the previously allocated memory accordingly.
//
// This callback is optional.
//
    pub op): *mut *mut void (op_free)(struct drm_gpuva_op,
//
// @vm_bo_alloc: called when the &drm_gpuvm allocates
// a struct drm_gpuvm_bo
//
// Some drivers may want to embed struct drm_gpuvm_bo into driver
// specific structures. By implementing this callback drivers can
// allocate memory accordingly.
//
// This callback is optional.
//
    pub (*vm_bo_alloc)(void): *mut drm_gpuvm_bo,
//
// @vm_bo_free: called when the &drm_gpuvm frees a
// struct drm_gpuvm_bo
//
// Some drivers may want to embed struct drm_gpuvm_bo into driver
// specific structures. By implementing this callback drivers can
// free the previously allocated memory accordingly.
//
// This callback is optional.
//
    pub vm_bo): *mut *mut void (vm_bo_free)(struct drm_gpuvm_bo,
//
// @vm_bo_validate: called from drm_gpuvm_validate()
//
// Drivers receive this callback for every evicted &drm_gem_object being
// mapped in the corresponding &drm_gpuvm.
//
// Typically, drivers would call their driver specific variant of
// ttm_bo_validate() from within this callback.
//
    pub exec): *mut drm_exec,
//
// @sm_step_map: called from &drm_gpuvm_sm_map to finally insert the
// mapping once all previous steps were completed
//
// The &priv pointer matches the one the driver passed to
// &drm_gpuvm_sm_map or &drm_gpuvm_sm_unmap, respectively.
//
// Can be NULL if &drm_gpuvm_sm_map is used.
//
    pub priv): *mut *mut *mut int (sm_step_map)(struct drm_gpuva_op op, void,
//
// @sm_step_remap: called from &drm_gpuvm_sm_map and
// &drm_gpuvm_sm_unmap to split up an existent mapping
//
// This callback is called when existent mapping needs to be split up.
// This is the case when either a newly requested mapping overlaps or
// is enclosed by an existent mapping or a partial unmap of an existent
// mapping is requested.
//
// The &priv pointer matches the one the driver passed to
// &drm_gpuvm_sm_map or &drm_gpuvm_sm_unmap, respectively.
//
// Can be NULL if neither &drm_gpuvm_sm_map nor &drm_gpuvm_sm_unmap is
// used.
//
    pub priv): *mut *mut *mut int (sm_step_remap)(struct drm_gpuva_op op, void,
//
// @sm_step_unmap: called from &drm_gpuvm_sm_map and
// &drm_gpuvm_sm_unmap to unmap an existing mapping
//
// This callback is called when existing mapping needs to be unmapped.
// This is the case when either a newly requested mapping encloses an
// existing mapping or an unmap of an existing mapping is requested.
//
// The &priv pointer matches the one the driver passed to
// &drm_gpuvm_sm_map or &drm_gpuvm_sm_unmap, respectively.
//
// Can be NULL if neither &drm_gpuvm_sm_map nor &drm_gpuvm_sm_unmap is
// used.
//
    pub priv): *mut *mut *mut int (sm_step_unmap)(struct drm_gpuva_op op, void,
}

extern "C" {
    pub fn drm_gpuva_unmap(op: *const drm_gpuva_op_unmap);
}
//
// drm_gpuva_op_remap_to_unmap_range() - Helper to get the start and range of
// the unmap stage of a remap op.
// @op: Remap op.
// @start_addr: Output pointer for the start of the required unmap.
// @range: Output pointer for the length of the required unmap.
//
// The given start address and range will be set such that they represent the
// range of the address space that was previously covered by the mapping being
// re-mapped, but is now empty.
//
// start_addr = va_start;
// range = va_end - va_start;
