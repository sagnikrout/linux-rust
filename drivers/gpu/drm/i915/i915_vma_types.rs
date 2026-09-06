//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_vma_types.h
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
// Copyright © 2016 Intel Corporation
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
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

//
// DOC: Global GTT views
//
// Background and previous state
//
// Historically objects could exists (be bound) in global GTT space only as
// singular instances with a view representing all of the object's backing pages
// in a linear fashion. This view will be called a normal view.
//
// To support multiple views of the same object, where the number of mapped
// pages is not equal to the backing store, or where the layout of the pages
// is not linear, concept of a GGTT view was added.
//
// One example of an alternative view is a stereo display driven by a single
// image. In this case we would have a framebuffer looking like this
// (2x2 pages):
//
// 12
// 34
//
// Above would represent a normal GGTT view as normally mapped for GPU or CPU
// rendering. In contrast, fed to the display engine would be an alternative
// view which could look something like this:
//
// 1212
// 3434
//
// In this example both the size and layout of pages in the alternative view is
// different from the normal view.
//
// Implementation and usage
//
// GGTT views are implemented using VMAs and are distinguished via enum
// i915_gtt_view_type and struct i915_gtt_view.
//
// A new flavour of core GEM functions which work with GGTT bound objects were
// added with the _ggtt_ infix, and sometimes with _view postfix to avoid
// renaming  in large amounts of code. They take the struct i915_gtt_view
// parameter encapsulating all metadata required to implement a view.
//
// As a helper for callers which are only interested in the normal view,
// globally const i915_gtt_view_normal singleton instance exists. All old core
// GEM API functions, the ones not taking the view parameter, are operating on,
// or with the normal GGTT view.
//
// Code wanting to add or use a new GGTT view needs to:
//
// 1. Add a new enum with a suitable name.
// 2. Extend the metadata in the i915_gtt_view structure if required.
// 3. Add support to i915_get_vma_pages().
//
// New views are required to build a scatter-gather table from within the
// i915_get_vma_pages function. This table is stored in the vma.gtt_view and
// exists for the lifetime of an VMA.
//
// Core API is designed to have copy semantics which means that passed in
// struct i915_gtt_view does not need to be persistent (left around after
// calling the core API functions).
//
// Check that rotation/remapped shares offsets for simplicity
// As we encode the size of each branch inside the union into its type,
// we have to be careful that each branch has a unique size.
//
// gcc complains if these are identical cases
//
// DOC: Virtual Memory Address
//
// A VMA represents a GEM BO that is bound into an address space. Therefore, a
// VMA's presence cannot be guaranteed before binding, or after unbinding the
// object into/from the address space.
//
// To make things as simple as possible (ie. no refcounting), a VMA's lifetime
// will always be <= an objects lifetime. So object refcounting should cover us.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_vma {
    pub node: drm_mm_node,
    pub vm: *mut i915_address_space,
    pub ops: *const i915_vma_ops,
    pub obj: *mut drm_i915_gem_object,
    pub pages: *mut sg_table,
    pub iomap: *mut void __iomem,
    pub /: *mut *mut *mut void private; / owned by creator,
    pub fence: *mut i915_fence_reg,
    pub size: u64,
    pub page_sizes: i915_page_sizes,
// mmap-offset associated with fencing for this vma
    pub mmo: *mut i915_mmap_offset,
    pub /: *mut *mut u32 guard; / padding allocated around vma->pages within the node,
    pub fence_size: u32,
    pub fence_alignment: u32,
    pub display_alignment: u32,
//
// Count of the number of times this vma has been opened by different
// handles (but same file) for execbuf, i.e. the number of aliases
// that exist in the ctx->handle_vmas LUT for this vma.
//
    pub open_count: core::sync::atomic::AtomicI32,
    pub flags: core::sync::atomic::AtomicI32,
//
// How many users have pinned this object in GTT space.
//
// This is a tightly bound, fairly small number of users, so we
// stuff inside the flags field so that we can both check for overflow
// and detect a no-op i915_vma_pin() in a single check, while also
// pinning the vma.
//
// The worst case display setup would have the same vma pinned for
// use on each plane on each crtc, while also building the next atomic
// state and holding a pin for the length of the cleanup queue. In the
// future, the flip queue may be increased from 1.
// Estimated worst case: 3 [qlen] * 4 [max crtcs] * 7 [max planes] = 84
//
// For GEM, the number of concurrent users for pwrite/pread is
// unbounded. For execbuffer, it is currently one but will in future
// be extended to allow multiple clients to pin vma concurrently.
//
// We also use suballocated pages, with each suballocation claiming
// its own pin on the shared vma. At present, this is limited to
// exclusive cachelines of a single page, so a maximum of 64 possible
// users.
//
pub const I915_VMA_PIN_MASK: c_uint = 0x3ff;
pub const I915_VMA_OVERFLOW: c_uint = 0x200;
// Flags and address space this VMA is bound to
pub const I915_VMA_GLOBAL_BIND_BIT: c_int = 10;
pub const I915_VMA_LOCAL_BIND_BIT: c_int = 11;

pub const I915_VMA_ERROR_BIT: c_int = 12;

pub const I915_VMA_GGTT_BIT: c_int = 13;
pub const I915_VMA_CAN_FENCE_BIT: c_int = 14;
pub const I915_VMA_USERFAULT_BIT: c_int = 15;
pub const I915_VMA_GGTT_WRITE_BIT: c_int = 16;

pub const I915_VMA_SCANOUT_BIT: c_int = 17;

    pub active: i915_active,
pub const I915_VMA_PAGES_BIAS: c_int = 24;

    pub /: *mut *mut atomic_t pages_count; / number of active binds to the pages,
//
// Whether we hold a reference on the vm dma_resv lock to temporarily
// block vm freeing until the vma is destroyed.
// Protected by the vm mutex.
//
    pub vm_ddestroy: bool,
//
// Support different GGTT views into the same object.
// This means there can be multiple VMA mappings per object and per VM.
// i915_gtt_view_type is used to distinguish between those entries.
// The default one of zero (I915_GTT_VIEW_NORMAL) is default and also
// assumed in GEM functions which take no ggtt view parameter.
//
    pub gtt_view: i915_gtt_view,
// This object's place on the active/inactive lists
    pub vm_link: list_head,
    pub /: *mut *mut list_head obj_link; / Link in the object's VMA list,
    pub obj_node: rb_node,
// This vma's place in the eviction list
    pub evict_link: list_head,
    pub closed_link: list_head,
// The async vma resource. Protected by the vm_mutex
    pub resource: *mut i915_vma_resource,
}
