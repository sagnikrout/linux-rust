//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ttm/ttm_agp_backend.c
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
// Copyright (c) 2006-2009 VMware, Inc., Palo Alto, CA., USA
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
// Authors: Thomas Hellstrom <thellstrom-at-vmware-dot-com>
// Keith Packard.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_agp_backend {
    pub ttm: ttm_tt,
    pub mem: *mut agp_memory,
    pub bridge: *mut agp_bridge_data,
}

#[no_mangle]
pub unsafe extern "C" fn ttm_agp_bind(ttm: *mut ttm_tt, bo_mem: *mut ttm_resource) -> c_int {
    int ttm_agp_bind(struct ttm_tt *ttm, struct ttm_resource *bo_mem)
    {
    struct ttm_agp_backend *agp_be = container_of(ttm, struct ttm_agp_backend, ttm);
    struct page *dummy_read_page = ttm_glob.dummy_read_page;
    struct agp_memory *mem;
    int ret, cached = ttm.caching == ttm_cached;
    unsigned i;
    if (agp_be.mem)
    return 0;
    mem = agp_allocate_memory(agp_be.bridge, ttm.num_pages, AGP_USER_MEMORY);
    if (unlikely(mem == core::ptr::null_mut()))
    return -ENOMEM;
    mem.page_count = 0;
    for (i = 0; i < ttm.num_pages; i++) {
    struct page *page = ttm.pages[i];
    if (!page)
    page = dummy_read_page;
    mem.pages[mem.page_count++] = page;
    }
    agp_be.mem = mem;
    mem.is_flushed = 1;
    mem.type = (cached) ? AGP_USER_CACHED_MEMORY : AGP_USER_MEMORY;
    ret = agp_bind_memory(mem, bo_mem.start);
    if (ret)
    pr_err("AGP Bind memory failed\n");
    return ret;
    }
    EXPORT_SYMBOL(ttm_agp_bind);
#[no_mangle]
pub unsafe extern "C" fn ttm_agp_unbind(ttm: *mut ttm_tt) {
    void ttm_agp_unbind(struct ttm_tt *ttm)
    {
    struct ttm_agp_backend *agp_be = container_of(ttm, struct ttm_agp_backend, ttm);
    if (agp_be.mem) {
    if (agp_be.mem.is_bound) {
    agp_unbind_memory(agp_be.mem);
    return;
    }
    agp_free_memory(agp_be.mem);
    agp_be.mem = core::ptr::null_mut();
    }
    }
    EXPORT_SYMBOL(ttm_agp_unbind);
#[no_mangle]
pub unsafe extern "C" fn ttm_agp_is_bound(ttm: *mut ttm_tt) -> bool {
    bool ttm_agp_is_bound(struct ttm_tt *ttm)
    {
    struct ttm_agp_backend *agp_be = container_of(ttm, struct ttm_agp_backend, ttm);
    if (!ttm)
    return false;
    return (agp_be.mem != core::ptr::null_mut());
    }
    EXPORT_SYMBOL(ttm_agp_is_bound);
#[no_mangle]
pub unsafe extern "C" fn ttm_agp_destroy(ttm: *mut ttm_tt) {
    void ttm_agp_destroy(struct ttm_tt *ttm)
    {
    struct ttm_agp_backend *agp_be = container_of(ttm, struct ttm_agp_backend, ttm);
    if (agp_be.mem)
    ttm_agp_unbind(ttm);
    ttm_tt_fini(ttm);
    kfree(agp_be);
    }
    EXPORT_SYMBOL(ttm_agp_destroy);
    struct ttm_tt *ttm_agp_tt_create(struct ttm_buffer_object *bo,
    struct agp_bridge_data *bridge,
    uint32_t page_flags)
    {
    struct ttm_agp_backend *agp_be;
    agp_be = kmalloc_obj(*agp_be);
    if (!agp_be)
    return core::ptr::null_mut();
    agp_be.mem = core::ptr::null_mut();
    agp_be.bridge = bridge;
    if (ttm_tt_init(&agp_be.ttm, bo, page_flags, ttm_write_combined, 0)) {
    kfree(agp_be);
    return core::ptr::null_mut();
    }
    return &agp_be.ttm;
    }
    EXPORT_SYMBOL(ttm_agp_tt_create);
