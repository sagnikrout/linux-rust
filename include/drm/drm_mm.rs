//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_mm.h
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
// Copyright 2006-2008 Tungsten Graphics, Inc., Cedar Park, TX. USA.
// Copyright 2016 Intel Corporation
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Authors:
// Thomas Hellstrom <thomas-at-tungstengraphics-dot-com>
//
// Generic range manager structs
//

//
// enum drm_mm_insert_mode - control search and allocation behaviour
//
// The &struct drm_mm range manager supports finding a suitable modes using
// a number of search trees. These trees are oranised by size, by address and
// in most recent eviction order. This allows the user to find either the
// smallest hole to reuse, the lowest or highest address to reuse, or simply
// reuse the most recent eviction that fits. When allocating the &drm_mm_node
// from within the hole, the &drm_mm_insert_mode also dictate whether to
// allocate the lowest matching address or the highest.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_mm_insert_mode {
//
// @DRM_MM_INSERT_BEST:
//
// Search for the smallest hole (within the search range) that fits
// the desired node.
//
// Allocates the node from the bottom of the found hole.
//
    DRM_MM_INSERT_BEST = 0,

//
// @DRM_MM_INSERT_LOW:
//
// Search for the lowest hole (address closest to 0, within the search
// range) that fits the desired node.
//
// Allocates the node from the bottom of the found hole.
//
    DRM_MM_INSERT_LOW,

//
// @DRM_MM_INSERT_HIGH:
//
// Search for the highest hole (address closest to U64_MAX, within the
// search range) that fits the desired node.
//
// Allocates the node from the *top* of the found hole. The specified
// alignment for the node is applied to the base of the node
// (&drm_mm_node.start).
//
    DRM_MM_INSERT_HIGH,

//
// @DRM_MM_INSERT_EVICT:
//
// Search for the most recently evicted hole (within the search range)
// that fits the desired node. This is appropriate for use immediately
// after performing an eviction scan (see drm_mm_scan_init()) and
// removing the selected nodes to form a hole.
//
// Allocates the node from the bottom of the found hole.
//
    DRM_MM_INSERT_EVICT,

//
// @DRM_MM_INSERT_ONCE:
//
// Only check the first hole for suitablity and report -ENOSPC
// immediately otherwise, rather than check every hole until a
// suitable one is found. Can only be used in conjunction with another
// search method such as DRM_MM_INSERT_HIGH or DRM_MM_INSERT_LOW.
//
    DRM_MM_INSERT_ONCE = BIT(31),

//
// @DRM_MM_INSERT_HIGHEST:
//
// Only check the highest hole (the hole with the largest address) and
// insert the node at the top of the hole or report -ENOSPC if
// unsuitable.
//
// Does not search all holes.
//
    DRM_MM_INSERT_HIGHEST = DRM_MM_INSERT_HIGH | DRM_MM_INSERT_ONCE,

//
// @DRM_MM_INSERT_LOWEST:
//
// Only check the lowest hole (the hole with the smallest address) and
// insert the node at the bottom of the hole or report -ENOSPC if
// unsuitable.
//
// Does not search all holes.
//
    DRM_MM_INSERT_LOWEST  = DRM_MM_INSERT_LOW | DRM_MM_INSERT_ONCE,
}

//
// struct drm_mm_node - allocated block in the DRM allocator
//
// This represents an allocated block in a &drm_mm allocator. Except for
// pre-reserved nodes inserted using drm_mm_reserve_node() the structure is
// entirely opaque and should only be accessed through the provided funcions.
// Since allocation of these nodes is entirely handled by the driver they can be
// embedded.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mm_node {
// @color: Opaque driver-private tag.
    pub color: c_ulong,
// @start: Start address of the allocated block.
    pub start: u64,
// @size: Size of the allocated block.
    pub size: u64,
// private:
    pub mm: *mut drm_mm,
    pub node_list: list_head,
    pub hole_stack: list_head,
    pub rb: rb_node,
    pub rb_hole_size: rb_node,
    pub rb_hole_addr: rb_node,
    pub __subtree_last: u64,
    pub hole_size: u64,
    pub subtree_max_hole: u64,
    pub flags: c_ulong,
pub const DRM_MM_NODE_ALLOCATED_BIT: c_int = 0;
pub const DRM_MM_NODE_SCANNED_BIT: c_int = 1;

    pub stack: depot_stack_handle_t,

}

//
// struct drm_mm - DRM allocator
//
// DRM range allocator with a few special functions and features geared towards
// managing GPU memory. Except for the @color_adjust callback the structure is
// entirely opaque and should only be accessed through the provided functions
// and macros. This structure can be embedded into larger driver structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mm {
//
// @color_adjust:
//
// Optional driver callback to further apply restrictions on a hole. The
// node argument points at the node containing the hole from which the
// block would be allocated (see drm_mm_hole_follows() and friends). The
// other arguments are the size of the block to be allocated. The driver
// can adjust the start and end as needed to e.g. insert guard pages.
//
    pub end): *mut *mut u64 start, u64,
// private:
// List of all memory nodes that immediately precede a free hole.
    pub hole_stack: list_head,
// head_node.node_list is the list of all memory nodes, ordered
// according to the (increasing) start address of the memory node.
    pub head_node: drm_mm_node,
// Keep an interval_tree for fast lookup of drm_mm_nodes by address.
    pub interval_tree: rb_root_cached,
    pub holes_size: rb_root_cached,
    pub holes_addr: rb_root,
    pub scan_active: c_ulong,
}

//
// struct drm_mm_scan - DRM allocator eviction roaster data
//
// This structure tracks data needed for the eviction roaster set up using
// drm_mm_scan_init(), and used with drm_mm_scan_add_block() and
// drm_mm_scan_remove_block(). The structure is entirely opaque and should only
// be accessed through the provided functions and macros. It is meant to be
// allocated temporarily by the driver on the stack.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mm_scan {
// private:
    pub mm: *mut drm_mm,
    pub size: u64,
    pub alignment: u64,
    pub remainder_mask: u64,
    pub range_start: u64,
    pub range_end: u64,
    pub hit_start: u64,
    pub hit_end: u64,
    pub color: c_ulong,
    pub mode: drm_mm_insert_mode,
}

//
// drm_mm_node_allocated - checks whether a node is allocated
// @node: drm_mm_node to check
//
// Drivers are required to clear a node prior to using it with the
// drm_mm range manager.
//
// Drivers should use this helper for proper encapsulation of drm_mm
// internals.
//
// Returns:
// True if the @node is allocated.
//
extern "C" {
    pub fn test_bit(_arg: DRM_MM_NODE_ALLOCATED_BIT, _arg: &node->flags) -> return;
}
//
// drm_mm_initialized - checks whether an allocator is initialized
// @mm: drm_mm to check
//
// Drivers should clear the struct drm_mm prior to initialisation if they
// want to use this function.
//
// Drivers should use this helper for proper encapsulation of drm_mm
// internals.
//
// Returns:
// True if the @mm is initialized.
//
extern "C" {
    pub fn READ_ONCE(_arg: mm->hole_stack.next) -> return;
}
//
// drm_mm_hole_follows - checks whether a hole follows this node
// @node: drm_mm_node to check
//
// Holes are embedded into the drm_mm using the tail of a drm_mm_node.
// If you wish to know whether a hole follows this particular node,
// query this function. See also drm_mm_hole_node_start() and
// drm_mm_hole_node_end().
//
// Returns:
// True if a hole follows the @node.
//
// drm_mm_hole_node_start - computes the start of the hole following @node
// @hole_node: drm_mm_node which implicitly tracks the following hole
//
// This is useful for driver-specific debug dumpers. Otherwise drivers should
// not inspect holes themselves. Drivers must check first whether a hole indeed
// follows by looking at drm_mm_hole_follows()
//
// Returns:
// Start of the subsequent hole.
//
extern "C" {
    pub fn __drm_mm_hole_node_start(_arg: hole_node) -> return;
}
//
// drm_mm_hole_node_end - computes the end of the hole following @node
// @hole_node: drm_mm_node which implicitly tracks the following hole
//
// This is useful for driver-specific debug dumpers. Otherwise drivers should
// not inspect holes themselves. Drivers must check first whether a hole indeed
// follows by looking at drm_mm_hole_follows().
//
// Returns:
// End of the subsequent hole.
//
extern "C" {
    pub fn __drm_mm_hole_node_end(_arg: hole_node) -> return;
}
//
// drm_mm_nodes - list of nodes under the drm_mm range manager
// @mm: the struct drm_mm range manager
//
// As the drm_mm range manager hides its node_list deep with its
// structure, extracting it looks painful and repetitive. This is
// not expected to be used outside of the drm_mm_for_each_node()
// macros and similar internal functions.
//
// Returns:
// The node list, may be empty.
//

//
// drm_mm_for_each_node - iterator to walk over all allocated nodes
// @entry: &struct drm_mm_node to assign to in each iteration step
// @mm: &drm_mm allocator to walk
//
// This iterator walks over all nodes in the range allocator. It is implemented
// with list_for_each(), so not save against removal of elements.
//

//
// drm_mm_for_each_node_safe - iterator to walk over all allocated nodes
// @entry: &struct drm_mm_node to assign to in each iteration step
// @next: &struct drm_mm_node to store the next step
// @mm: &drm_mm allocator to walk
//
// This iterator walks over all nodes in the range allocator. It is implemented
// with list_for_each_safe(), so save against removal of elements.
//

//
// drm_mm_for_each_hole - iterator to walk over all holes
// @pos: &drm_mm_node used internally to track progress
// @mm: &drm_mm allocator to walk
// @hole_start: ulong variable to assign the hole start to on each iteration
// @hole_end: ulong variable to assign the hole end to on each iteration
//
// This iterator walks over all holes in the range allocator. It is implemented
// with list_for_each(), so not save against removal of elements. @entry is used
// internally and will not reflect a real drm_mm_node for the very first hole.
// Hence users of this iterator may not access it.
//
// Implementation Note:
// We need to inline list_for_each_entry in order to be able to set hole_start
// and hole_end on each iteration while keeping the macro sane.
//

//
// Basic range manager support (drm_mm.c)
//
extern "C" {
    pub fn drm_mm_reserve_node(mm: *mut drm_mm, node: *mut drm_mm_node) -> c_int;
}
//
// drm_mm_insert_node_generic - search for space and insert @node
// @mm: drm_mm to allocate from
// @node: preallocate node to insert
// @size: size of the allocation
// @alignment: alignment of the allocation
// @color: opaque tag value to use for this node
// @mode: fine-tune the allocation search and placement
//
// This is a simplified version of drm_mm_insert_node_in_range() with no
// range restrictions applied.
//
// The preallocated node must be cleared to 0.
//
// Returns:
// 0 on success, -ENOSPC if there's no suitable hole.
//
// drm_mm_insert_node - search for space and insert @node
// @mm: drm_mm to allocate from
// @node: preallocate node to insert
// @size: size of the allocation
//
// This is a simplified version of drm_mm_insert_node_generic() with @color set
// to 0.
//
// The preallocated node must be cleared to 0.
//
// Returns:
// 0 on success, -ENOSPC if there's no suitable hole.
//
extern "C" {
    pub fn drm_mm_insert_node_generic(_arg: mm, _arg: node, _arg: size, _arg: 0, _arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn drm_mm_remove_node(node: *mut drm_mm_node);
}
extern "C" {
    pub fn drm_mm_init(mm: *mut drm_mm, start: u64, size: u64);
}
extern "C" {
    pub fn drm_mm_takedown(mm: *mut drm_mm);
}
//
// drm_mm_clean - checks whether an allocator is clean
// @mm: drm_mm allocator to check
//
// Returns:
// True if the allocator is completely free, false if there's still a node
// allocated in it.
//
extern "C" {
    pub fn list_empty(_arg: drm_mm_nodes(mm)) -> return;
}
//
// drm_mm_for_each_node_in_range - iterator to walk over a range of
// allocated nodes
// @node__: drm_mm_node structure to assign to in each iteration step
// @mm__: drm_mm allocator to walk
// @start__: starting offset, the first node will overlap this
// @end__: ending offset, the last node will start before this (but may overlap)
//
// This iterator walks over all nodes in the range allocator that lie
// between @start and @end. It is implemented similarly to list_for_each(),
// but using the internal interval tree to accelerate the search for the
// starting node, and so not safe against removal of elements. It assumes
// that @end is within (or is the upper limit of) the drm_mm allocator.
// If [@start, @end] are beyond the range of the drm_mm, the iterator may walk
// over the special _unallocated_ &drm_mm.head_node, and may even continue
// indefinitely.
//

//
// drm_mm_scan_init - initialize lru scanning
// @scan: scan state
// @mm: drm_mm to scan
// @size: size of the allocation
// @alignment: alignment of the allocation
// @color: opaque tag value to use for the allocation
// @mode: fine-tune the allocation search and placement
//
// This is a simplified version of drm_mm_scan_init_with_range() with no range
// restrictions applied.
//
// This simply sets up the scanning routines with the parameters for the desired
// hole.
//
// Warning:
// As long as the scan list is non-empty, no other operations than
// adding/removing nodes to/from the scan list are allowed.
//
extern "C" {
    pub fn drm_mm_print(mm: *const drm_mm, p: *mut drm_printer);
}
