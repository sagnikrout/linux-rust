//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/radeon_object.h
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
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2008 Red Hat Inc.
// Copyright 2009 Jerome Glisse.
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
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//

//
// radeon_mem_type_to_domain - return domain corresponding to mem_type
// @mem_type:	ttm memory type
//
// Returns corresponding domain of the ttm mem_type
//
// radeon_bo_reserve - reserve bo
// @bo:		bo structure
// @no_intr:	don't return -ERESTARTSYS on pending signal
//
// Returns:
// -ERESTARTSYS: A wait for the buffer to become unreserved was interrupted by
// a signal. Release all buffer reservations and return to user-space.
//
// radeon_bo_gpu_offset - return GPU offset of bo
// @bo:	radeon object for which we query the offset
//
// Returns current GPU offset of the object.
//
// Note: object should either be pinned or reserved when calling this
// function, it might be useful to add check for this for debugging.
//
// radeon_bo_mmap_offset - return mmap offset of bo
// @bo:	radeon object for which we query the offset
//
// Returns mmap offset of the object.
//
extern "C" {
    pub fn drm_vma_node_offset_addr(_arg: &bo->tbo.base.vma_node) -> return;
}
extern "C" {
    pub fn radeon_bo_kmap(bo: *mut radeon_bo, ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn radeon_bo_kunmap(bo: *mut radeon_bo);
}
extern "C" {
    pub fn radeon_bo_unref(bo: *mut radeon_bo);
}
extern "C" {
    pub fn radeon_bo_pin(bo: *mut radeon_bo, domain: u32, gpu_addr: *mut u64) -> c_int;
}
extern "C" {
    pub fn radeon_bo_unpin(bo: *mut radeon_bo);
}
extern "C" {
    pub fn radeon_bo_evict_vram(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_bo_force_delete(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_bo_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_bo_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_bo_move_notify(bo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn radeon_bo_fault_reserve_notify(bo: *mut ttm_buffer_object) -> vm_fault_t;
}
extern "C" {
    pub fn radeon_bo_get_surface_reg(bo: *mut radeon_bo) -> c_int;
}
//
// sub allocation
//
extern "C" {
    pub fn container_of(_arg: manager, radeon_sa_manager: struct, _arg: base) -> return;
}

