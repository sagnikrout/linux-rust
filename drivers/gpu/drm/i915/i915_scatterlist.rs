//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_scatterlist.h
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
// SPDX-License-Identifier: MIT
//
// Copyright © 2016 Intel Corporation
//

//
// Optimised SGL iterator for GEM objects
//
// __sg_next - return the next scatterlist entry in a list
// @sg:		The current sg entry
//
// Description:
// If the entry is the last, return NULL; otherwise, step to the next
// element in the array (@sg@+1). If that's a chain pointer, follow it;
// otherwise just return the pointer to the current element.
//
extern "C" {
    pub fn sg_is_last(____sg_next(sg: sg) ? NULL :) -> return;
}
//
// __for_each_sgt_daddr - iterate over the device addresses of the given sg_table
// @__dp:	Device address (output)
// @__iter:	'struct sgt_iter' (iterator state, internal)
// @__sgt:	sg_table to iterate over (input)
// @__step:	step size
//

//
// __for_each_daddr_next - iterates over the device addresses with pre-initialized iterator.
// @__dp:	Device address (output)
// @__iter:	'struct sgt_iter' (iterator state, external)
// @__step:	step size
//

//
// for_each_sgt_page - iterate over the pages of the given sg_table
// @__pp:	page pointer (output)
// @__iter:	'struct sgt_iter' (iterator state, internal)
// @__sgt:	sg_table to iterate over (input)
//

//
// i915_sg_dma_sizes - Record the dma segment sizes of a scatterlist
// @sg: The scatterlist
//
// Return: An unsigned int with segment sizes logically or'ed together.
// A caller can use this information to determine what hardware page table
// entry sizes can be used to map the memory represented by the scatterlist.
//
// For Xen PV guests pages aren't contiguous in DMA (machine) address
// space.  The DMA API takes care of that both in dma_alloc_* (by
// calling into the hypervisor to make the pages contiguous) and in
// dma_map_* (by bounce buffering).  But i915 abuses ignores the
// coherency aspects of the DMA API and thus can't cope with bounce
// buffering actually happening, so add a hack here to force small
// allocations and mappings when running in PV mode on Xen.
//
// Note this will still break if bounce buffering is required for other
// reasons, like confidential computing hypervisors or PCIe root ports
// with addressing limitations.
//
extern "C" {
    pub fn round_down(_arg: max, _arg: PAGE_SIZE) -> return;
}
extern "C" {
    pub fn i915_sg_trim(orig_st: *mut sg_table) -> bool;
}
//
// struct i915_refct_sgt_ops - Operations structure for struct i915_refct_sgt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_refct_sgt_ops {
//
// @release: Free the memory of the struct i915_refct_sgt
//
    pub ref): *mut *mut void (release)(struct kref,
}

//
// struct i915_refct_sgt - A refcounted scatter-gather table
// @kref: struct kref for refcounting
// @table: struct sg_table holding the scatter-gather table itself. Note that
// @table->sgl = NULL can be used to determine whether a scatter-gather table
// is present or not.
// @size: The size in bytes of the underlying memory buffer
// @ops: The operations structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_refct_sgt {
    pub kref: kref,
    pub table: sg_table,
    pub size: usize,
    pub ops: *const i915_refct_sgt_ops,
}

//
// i915_refct_sgt_put - Put a refcounted sg-table
// @rsgt: the struct i915_refct_sgt to put.
//
// i915_refct_sgt_get - Get a refcounted sg-table
// @rsgt: the struct i915_refct_sgt to get.
//
// __i915_refct_sgt_init - Initialize a refcounted sg-list with a custom
// operations structure
// @rsgt: The struct i915_refct_sgt to initialize.
// @size: Size in bytes of the underlying memory buffer.
// @ops: A customized operations structure in case the refcounted sg-list
// is embedded into another structure.
//
extern "C" {
    pub fn i915_refct_sgt_init(rsgt: *mut i915_refct_sgt, size: usize);
}
