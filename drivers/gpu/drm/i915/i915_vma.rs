//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_vma.h
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

extern "C" {
    pub fn i915_vma_unpin_and_release(p_vma: *mut i915_vma, flags: c_uint);
}

// do not reserve memory to prevent deadlocks

extern "C" {
    pub fn _i915_vma_move_to_active(_arg: vma, _arg: rq, _arg: &rq->fence, _arg: flags) -> return;
}

extern "C" {
    pub fn test_bit(_arg: I915_VMA_GGTT_BIT, _arg: __i915_vma_flags(vma)) -> return;
}
extern "C" {
    pub fn i915_is_dpt(_arg: vma->vm) -> return;
}
extern "C" {
    pub fn test_bit(_arg: I915_VMA_GGTT_WRITE_BIT, _arg: __i915_vma_flags(vma)) -> return;
}
extern "C" {
    pub fn i915_vma_flush_writes(vma: *mut i915_vma);
}
extern "C" {
    pub fn test_bit(_arg: I915_VMA_CAN_FENCE_BIT, _arg: __i915_vma_flags(vma)) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: I915_VMA_USERFAULT_BIT, _arg: __i915_vma_flags(vma)) -> return;
}
extern "C" {
    pub fn clear_bit(_arg: I915_VMA_USERFAULT_BIT, _arg: __i915_vma_flags(vma)) -> return;
}
extern "C" {
    pub fn test_bit(_arg: I915_VMA_USERFAULT_BIT, _arg: __i915_vma_flags(vma)) -> return;
}
// Internal use only.
//
// i915_vma_size - Obtain the va range size of the vma
// @vma: The vma
//
// GPU virtual address space may be allocated with padding. This
// function returns the effective virtual address range size
// with padding subtracted.
//
// Return: The effective virtual address range size.
//
extern "C" {
    pub fn __i915_vma_size(_arg: vma) -> return;
}
// Internal use only.
// The actual start of the vma->pages is after the guard pages.
//
// i915_vma_offset - Obtain the va offset of the vma
// @vma: The vma
//
// GPU virtual address space may be allocated with padding. This
// function returns the effective virtual address offset the gpu
// should use to access the bound data.
//
// Return: The effective virtual address offset.
//
extern "C" {
    pub fn __i915_vma_offset(_arg: vma) -> return;
}
extern "C" {
    pub fn lower_32_bits(_arg: i915_vma_offset(vma)) -> return;
}
// gtt_view.type also encodes its size so that we both distinguish
// different views using it as a "type" and also use a compact (no
// accessing of uninitialised padding bytes) memcmp without storing
// an extra parameter or adding more code.
//
// To ensure that the memcmp is valid for all branches of the union,
// even though the code looks like it is just comparing one branch,
// we assert above that all branches have the same address, and that
// each branch has a unique type/size.
//
extern "C" {
    pub fn memcmp(_arg: &vma->gtt_view.partial, _arg: &view->partial, _arg: view->type) -> return;
}
extern "C" {
    pub fn i915_gem_valid_gtt_space(vma: *mut i915_vma, color: c_ulong) -> bool;
}
extern "C" {
    pub fn __i915_vma_set_map_and_fenceable(vma: *mut i915_vma);
}
extern "C" {
    pub fn i915_vma_revoke_mmap(vma: *mut i915_vma);
}
extern "C" {
    pub fn vma_invalidate_tlb(vm: *mut i915_address_space, tlb: *mut u32);
}
extern "C" {
    pub fn __i915_vma_unbind(vma: *mut i915_vma) -> c_int;
}
extern "C" {
    pub fn i915_vma_unbind(vma: *mut i915_vma) -> int __must_check;
}
extern "C" {
    pub fn i915_vma_unbind_async(vma: *mut i915_vma, trylock_vm: bool) -> int __must_check;
}
extern "C" {
    pub fn i915_vma_unbind_unlocked(vma: *mut i915_vma) -> int __must_check;
}
extern "C" {
    pub fn i915_vma_unlink_ctx(vma: *mut i915_vma);
}
extern "C" {
    pub fn i915_vma_close(vma: *mut i915_vma);
}
extern "C" {
    pub fn i915_vma_reopen(vma: *mut i915_vma);
}
extern "C" {
    pub fn i915_vma_destroy_locked(vma: *mut i915_vma);
}
extern "C" {
    pub fn i915_vma_destroy(vma: *mut i915_vma);
}

extern "C" {
    pub fn i915_vma_pin_count(_arg: vma) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: vma->iomap) -> return;
}
//
// i915_vma_pin_iomap - calls ioremap_wc to map the GGTT VMA via the aperture
// @vma: VMA to iomap
//
// The passed in VMA has to be pinned in the global GTT mappable region.
// An extra pinning of the VMA is acquired for the return iomapping,
// the caller must call i915_vma_unpin_iomap to relinquish the pinning
// after the iomapping is no longer required.
//
// Returns a valid iomapped pointer or ERR_PTR.
//
// i915_vma_unpin_iomap - unpins the mapping returned from i915_vma_iomap
// @vma: VMA to unpin
//
// Unpins the previously iomapped VMA from i915_vma_pin_iomap().
//
// This function is only valid to be called on a VMA previously
// iomapped by the caller with i915_vma_pin_iomap().
//
extern "C" {
    pub fn i915_vma_unpin_iomap(vma: *mut i915_vma);
}
//
// i915_vma_pin_fence - pin fencing state
// @vma: vma to pin fencing for
//
// This pins the fencing state (whether tiled or untiled) to make sure the
// vma (and its object) is ready to be used as a scanout target. Fencing
// status must be synchronize first by calling i915_vma_get_fence():
//
// The resulting fence pin reference must be released again with
// i915_vma_unpin_fence().
//
// Returns:
// True if the vma has a fence, false otherwise.
//
extern "C" {
    pub fn i915_vma_pin_fence(vma: *mut i915_vma) -> int __must_check;
}
extern "C" {
    pub fn i915_vma_revoke_fence(vma: *mut i915_vma);
}
extern "C" {
    pub fn __i915_vma_pin_fence(vma: *mut i915_vma) -> c_int;
}
//
// i915_vma_unpin_fence - unpin fencing state
// @vma: vma to unpin fencing for
//
// This releases the fence pin reference acquired through
// i915_vma_pin_fence. It will handle both objects with and without an
// attached fence correctly, callers do not need to distinguish this.
//
extern "C" {
    pub fn i915_vma_parked(gt: *mut intel_gt);
}
extern "C" {
    pub fn test_bit(_arg: I915_VMA_SCANOUT_BIT, _arg: __i915_vma_flags(vma)) -> return;
}
extern "C" {
    pub fn i915_ggtt_clear_scanout(obj: *mut drm_i915_gem_object);
}

//
// for_each_ggtt_vma - Iterate over the GGTT VMA belonging to an object.
// @V: the #i915_vma iterator
// @OBJ: the #drm_i915_gem_object
//
// GGTT VMA are placed at the being of the object's vma_list, see
// vma_create(), so we can stop our walk as soon as we see a ppgtt VMA,
// or the list is empty ofc.
//

extern "C" {
    pub fn i915_vma_make_shrinkable(vma: *mut i915_vma);
}
extern "C" {
    pub fn i915_vma_make_purgeable(vma: *mut i915_vma);
}
extern "C" {
    pub fn i915_vma_wait_for_bind(vma: *mut i915_vma) -> c_int;
}
// Wait for the asynchronous bindings and pending GPU reads
extern "C" {
    pub fn i915_active_wait(_arg: &vma->active) -> return;
}
//
// i915_vma_get_current_resource - Get the current resource of the vma
// @vma: The vma to get the current resource from.
//
// It's illegal to call this function if the vma is not bound.
//
// Return: A refcounted pointer to the current vma resource
// of the vma, assuming the vma is bound.
//
extern "C" {
    pub fn i915_vma_resource_get(_arg: vma->resource) -> return;
}

extern "C" {
    pub fn i915_vma_module_exit();
}
extern "C" {
    pub fn i915_vma_module_init() -> c_int;
}
