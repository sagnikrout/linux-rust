//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_vma_manager.h
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
// Copyright (c) 2013 David Herrmann <dh.herrmann@gmail.com>
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

// We make up offsets for buffer objects so we can recognize them at
// mmap time. pgoff in mmap is an unsigned long, so we need to make sure
// that the faked up offset will fit
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vma_offset_file {
    pub vm_rb: rb_node,
    pub vm_tag: *mut drm_file,
    pub vm_count: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vma_offset_node {
    pub vm_lock: rwlock_t,
    pub vm_node: drm_mm_node,
    pub vm_files: rb_root,
    pub driver_private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vma_offset_manager {
    pub vm_lock: rwlock_t,
    pub vm_addr_space_mm: drm_mm,
}

extern "C" {
    pub fn drm_vma_offset_manager_destroy(mgr: *mut drm_vma_offset_manager);
}
extern "C" {
    pub fn drm_vma_node_allow(node: *mut drm_vma_offset_node, tag: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn drm_vma_node_allow_once(node: *mut drm_vma_offset_node, tag: *mut drm_file) -> c_int;
}
//
// drm_vma_offset_exact_lookup_locked() - Look up node by exact address
// @mgr: Manager object
// @start: Start address (page-based, not byte-based)
// @pages: Size of object (page-based)
//
// Same as drm_vma_offset_lookup_locked() but does not allow any offset into the node.
// It only returns the exact object with the given start address.
//
// RETURNS:
// Node at exact start address @start.
//
// drm_vma_offset_lock_lookup() - Lock lookup for extended private use
// @mgr: Manager object
//
// Lock VMA manager for extended lookups. Only locked VMA function calls
// are allowed while holding this lock. All other contexts are blocked from VMA
// until the lock is released via drm_vma_offset_unlock_lookup().
//
// Use this if you need to take a reference to the objects returned by
// drm_vma_offset_lookup_locked() before releasing this lock again.
//
// This lock must not be used for anything else than extended lookups. You must
// not call any other VMA helpers while holding this lock.
//
// Note: You're in atomic-context while holding this lock!
//
// drm_vma_offset_unlock_lookup() - Unlock lookup for extended private use
// @mgr: Manager object
//
// Release lookup-lock. See drm_vma_offset_lock_lookup() for more information.
//
// drm_vma_node_reset() - Initialize or reset node object
// @node: Node to initialize or reset
//
// Reset a node to its initial state. This must be called before using it with
// any VMA offset manager.
//
// This must not be called on an already allocated node, or you will leak
// memory.
//
// drm_vma_node_start() - Return start address for page-based addressing
// @node: Node to inspect
//
// Return the start address of the given node. This can be used as offset into
// the linear VM space that is provided by the VMA offset manager. Note that
// this can only be used for page-based addressing. If you need a proper offset
// for user-space mappings, you must apply "<< PAGE_SHIFT" or use the
// drm_vma_node_offset_addr() helper instead.
//
// RETURNS:
// Start address of @node for page-based addressing. 0 if the node does not
// have an offset allocated.
//
// drm_vma_node_size() - Return size (page-based)
// @node: Node to inspect
//
// Return the size as number of pages for the given node. This is the same size
// that was passed to drm_vma_offset_add(). If no offset is allocated for the
// node, this is 0.
//
// RETURNS:
// Size of @node as number of pages. 0 if the node does not have an offset
// allocated.
//
// drm_vma_node_offset_addr() - Return sanitized offset for user-space mmaps
// @node: Linked offset node
//
// Same as drm_vma_node_start() but returns the address as a valid offset that
// can be used for user-space mappings during mmap().
// This must not be called on unlinked nodes.
//
// RETURNS:
// Offset of @node for byte-based addressing. 0 if the node does not have an
// object allocated.
//
// drm_vma_node_unmap() - Unmap offset node
// @node: Offset node
// @file_mapping: Address space to unmap @node from
//
// Unmap all userspace mappings for a given offset node. The mappings must be
// associated with the @file_mapping address-space. If no offset exists
// nothing is done.
//
// This call is unlocked. The caller must guarantee that drm_vma_offset_remove()
// is not called on this node concurrently.
//
// drm_vma_node_verify_access() - Access verification helper for TTM
// @node: Offset node
// @tag: Tag of file to check
//
// This checks whether @tag is granted access to @node. It is the same as
// drm_vma_node_is_allowed() but suitable as drop-in helper for TTM
// verify_access() callbacks.
//
// RETURNS:
// 0 if access is granted, -EACCES otherwise.
//
