//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_bo.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (c) 2023-2024 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_bo_domain {
    VMW_BO_DOMAIN_SYS           = BIT(0),
    VMW_BO_DOMAIN_WAITABLE_SYS  = BIT(1),
    VMW_BO_DOMAIN_VRAM          = BIT(2),
    VMW_BO_DOMAIN_GMR           = BIT(3),
    VMW_BO_DOMAIN_MOB           = BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_bo_params {
    pub domain: u32,
    pub busy_domain: u32,
    pub bo_type: ttm_bo_type,
    pub pin: bool,
    pub keep_resv: bool,
    pub size: usize,
    pub resv: *mut dma_resv,
    pub sg: *mut sg_table,
}

//
// struct vmw_bo - TTM buffer object with vmwgfx additions
// @tbo: The TTM buffer object
// @placement: The preferred placement for this buffer object
// @places: The chosen places for the preferred placement.
// @busy_places: Chosen busy places for the preferred placement
// @map: Kmap object for semi-persistent mappings
// @res_tree: RB tree of resources using this buffer object as a backing MOB
// @res_prios: Eviction priority counts for attached resources
// @map_count: The number of currently active maps. Will differ from the
// cpu_writers because it includes kernel maps.
// @cpu_writers: Number of synccpu write grabs. Protected by reservation when
// increased. May be decreased without reservation.
// @dx_query_ctx: DX context if this buffer object is used as a DX query MOB
// @dirty: structure for user-space dirty-tracking
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_bo {
    pub tbo: ttm_buffer_object,
    pub placement: ttm_placement,
    pub places: [ttm_place; 5],
// Protected by reservation
    pub map: ttm_bo_kmap_obj,
    pub res_tree: rb_root,
    pub res_prios: [u32; TTM_MAX_BO_PRIORITY],
    pub detached_resources: xarray,
    pub map_count: core::sync::atomic::AtomicI32,
    pub cpu_writers: core::sync::atomic::AtomicI32,
// Not ref-counted.  Protected by binding_mutex
    pub dx_query_ctx: *mut vmw_resource,
    pub dirty: *mut vmw_bo_dirty,
    pub is_dumb: bool,
    pub dumb_surface: *mut vmw_surface,
}

extern "C" {
    pub fn vmw_bo_placement_set(bo: *mut vmw_bo, domain: u32, busy_domain: u32);
}
extern "C" {
    pub fn vmw_bo_placement_set_default_accelerated(bo: *mut vmw_bo);
}
extern "C" {
    pub fn vmw_bo_pin_reserved(bo: *mut vmw_bo, pin: bool);
}
extern "C" {
    pub fn vmw_bo_unmap(vbo: *mut vmw_bo);
}
extern "C" {
    pub fn vmw_bo_swap_notify(bo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn vmw_bo_add_detached_resource(vbo: *mut vmw_bo, res: *mut vmw_resource) -> c_int;
}
extern "C" {
    pub fn vmw_bo_del_detached_resource(vbo: *mut vmw_bo, res: *mut vmw_resource);
}
//
// vmw_bo_adjust_prio - Adjust the buffer object eviction priority
// according to attached resources
// @vbo: The struct vmw_bo
//
// vmw_bo_prio_add - Notify a buffer object of a newly attached resource
// eviction priority
// @vbo: The struct vmw_bo
// @prio: The resource priority
//
// After being notified, the code assigns the highest resource eviction priority
// to the backing buffer object (mob).
//
// vmw_bo_used_prio_del - Notify a buffer object of a resource with a certain
// priority being removed
// @vbo: The struct vmw_bo
// @prio: The resource priority
//
// After being notified, the code assigns the highest resource eviction priority
// to the backing buffer object (mob).
//
// buf = NULL;
extern "C" {
    pub fn container_of(_arg: (gobj), vmw_bo: struct, _arg: tbo.base) -> return;
}
extern "C" {
    pub fn vmw_bo_mobid(vbo: *mut vmw_bo) -> i32;
}
